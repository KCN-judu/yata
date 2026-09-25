//! `mumu-snapshot-v1` into the IR, over synthetic files. No real account data is used.

use serde_json::{Value, json};
use yata_core::import::admit::{SoulAdmissionError, admit};
use yata_core::import::ir::{
    Completeness, Currency, FormatTag, IrError, Section, SectionKind, SourceFormat, SourceId,
};
use yata_core::soul::{SoulAttribute, SoulKind};

use super::*;

fn soul(id: &str, slot: i64) -> Value {
    json!({
        "id": id,
        "setId": "破势",
        "slot": slot,
        "quality": 6,
        "level": 15,
        "mainAttrType": "crit_damage",
        "mainAttrValue": 0.89,
        "subAttributes": [
            {"type": "crit_rate", "value": 0.03, "enhancementCount": 1},
            {"type": "speed", "value": 11.35, "enhancementCount": 3},
        ],
        "initialSubstatCount": null,
        "equippedState": null,
    })
}

fn scope(souls: bool, heroes: bool, assets: bool, guild: bool) -> Value {
    json!({"souls": souls, "heroes": heroes, "items": assets, "realmCards": assets,
           "guild": guild, "taskRecords": false})
}

fn file(souls: Vec<Value>) -> Value {
    json!({
        "format": "mumu-snapshot-v1",
        "source": "unread",
        "completeness": "complete",
        "capturedAt": "2026-09-25T09:10:32Z",
        "scope": scope(true, false, false, false),
        "hero_equips": souls,
        "equipPresets": [],
        "somethingNew": {"ignored": true},
    })
}

fn full() -> Value {
    let mut f = file(vec![soul("a", 6), soul("b", 2)]);
    f["scope"] = scope(true, true, true, true);
    f["equipPresets"] = json!([["主力", [null, "b", null, null, null, "a"]]]);
    f["heroes"] = json!({"h1": {"heroId": "301", "level": 40, "star": 6, "awake": 1,
                                "lock": false, "skinfo": [], "attrs": []}});
    f["currency"] = json!({"jade": 1200, "coin": 7});
    f["realmCards"] = json!([["c1", 200001, 4, [800, 0]]]);
    f["guild"] = json!({"level": 3, "activeMemberCount": 98, "members": [["x"]]});
    f
}

fn normalize(v: &Value) -> Result<Normalized, ImportError> {
    read(&serde_json::to_vec(v).expect("serialised"))
}

fn completeness<T>(s: &Section<T>) -> Option<Completeness> {
    s.present().map(|(c, _)| c)
}

#[test]
fn a_file_normalizes_into_the_ir_in_display_units() {
    let n = normalize(&file(vec![soul("a", 6)])).expect("normalized");
    let s = &n.snapshot;
    assert_eq!(
        s.provenance.format,
        SourceFormat::Community(FormatTag::MumuSnapshotV1)
    );
    assert_eq!(s.captured_at.as_deref(), Some("2026-09-25T09:10:32Z"));
    let (c, souls) = s.souls.present().expect("present");
    assert_eq!(c, Completeness::Complete);
    let r = &souls.souls[0];
    assert_eq!(r.id, SourceId::new("a").expect("id"));
    assert_eq!(r.set.as_str(), "破势");
    assert_eq!((r.slot, r.star, r.level), (6, 6, 15));
    assert_eq!(r.main.attribute, SoulAttribute::CritDmg);
    assert!(
        (r.main.value - 89.0).abs() < 1e-9,
        "a rate becomes percentage points"
    );
    assert!((r.rolled[0].valued.value - 3.0).abs() < 1e-9);
    assert!(
        (r.rolled[1].valued.value - 11.35).abs() < 1e-9,
        "speed as shown"
    );
    assert_eq!(r.rolled[1].rolls, Some(3));
    assert_eq!(r.innate, None);
    assert!(n.left_out.is_empty());
}

#[test]
fn the_scope_decides_which_sections_are_present() {
    let n = normalize(&file(vec![])).expect("normalized");
    assert_eq!(n.snapshot.shikigami, Section::Absent);
    assert_eq!(n.snapshot.assets, Section::Absent);
    assert_eq!(n.snapshot.guild, Section::Absent);
    assert_eq!(
        completeness(&n.snapshot.souls),
        Some(Completeness::Complete),
        "present and empty, not absent"
    );
    let all = normalize(&full()).expect("normalized");
    assert_eq!(
        all.snapshot.sections().len(),
        SectionKind::ALL.len(),
        "every section"
    );
}

#[test]
fn every_section_maps_into_yata_names() {
    let s = normalize(&full()).expect("normalized").snapshot;
    let heroes = s.shikigami.present().expect("present").1;
    assert_eq!(heroes.instances[0].species.0, 301);
    assert!(heroes.instances[0].evolved);
    let presets = s.presets.present().expect("present").1;
    assert_eq!(
        presets.presets[0].souls[5],
        Some(SourceId::new("a").expect("id"))
    );
    let assets = s.assets.present().expect("present").1;
    assert!(assets.currencies.contains(&(Currency::Jade, 1200)));
    assert_eq!(assets.realm_cards[0].kind.0, 200001);
    let guild = s.guild.present().expect("present").1;
    assert_eq!((guild.level, guild.member_count), (3, 98));
}

#[test]
fn the_innate_entry_becomes_the_innate_field() {
    let mut boss = soul("b", 1);
    boss["setId"] = json!("荒骷髅");
    boss["subAttributes"].as_array_mut().expect("array").push(
        json!({"type": "effect_resist", "value": 0.08, "enhancementCount": null, "fixedAttribute": true}),
    );
    let s = normalize(&file(vec![boss])).expect("normalized").snapshot;
    let r = &s.souls.present().expect("present").1.souls[0];
    assert_eq!(
        r.rolled.len(),
        2,
        "the innate attribute is not a rolled one"
    );
    assert_eq!(r.innate, Some(SoulAttribute::EffectRes));
    let admitted = admit(&s).expect("admitted");
    let (_, domain) = &admitted.souls.present().expect("present").1.values[0];
    assert!(matches!(domain.kind, SoulKind::Boss(_)));
}

#[test]
fn the_domain_refuses_at_admission_not_at_parsing() {
    let mut unknown = soul("u", 1);
    unknown["setId"] = json!("不存在");
    let mut seven = soul("s", 7);
    seven["setId"] = json!("破势");
    let mut no_rolls = soul("r", 1);
    no_rolls["subAttributes"][1]["enhancementCount"] = Value::Null;
    let n = normalize(&file(vec![unknown, seven, no_rolls])).expect("normalized");
    assert!(n.left_out.is_empty(), "each is valid source syntax");
    let admitted = admit(&n.snapshot).expect("admitted");
    let reasons: Vec<SoulAdmissionError> = admitted
        .souls
        .present()
        .expect("present")
        .1
        .rejected
        .iter()
        .map(|r| r.reason.clone())
        .collect();
    assert_eq!(
        reasons,
        vec![
            SoulAdmissionError::UnknownSet {
                name: "不存在".into()
            },
            SoulAdmissionError::NotASlot { n: 7 },
            SoulAdmissionError::MissingRolls { sub: 1 },
        ]
    );
}

#[test]
fn a_record_left_out_makes_its_section_partial() {
    let mut unknown = soul("u", 1);
    unknown["mainAttrType"] = json!("luck");
    let mut stringly = soul("s", 1);
    stringly["slot"] = json!("1");
    let mut marked = soul("m", 1);
    marked["subAttributes"][0]["fixedAttribute"] = json!(false);
    let n = normalize(&file(vec![
        soul("ok", 1),
        unknown,
        stringly,
        marked,
        json!(7),
    ]))
    .expect("normalized");
    assert_eq!(completeness(&n.snapshot.souls), Some(Completeness::Partial));
    let reasons: Vec<(usize, &SourceReason)> =
        n.left_out.iter().map(|d| (d.index, &d.reason)).collect();
    assert_eq!(
        reasons,
        vec![
            (
                1,
                &SourceReason::UnsupportedValue {
                    field: "mainAttrType".into(),
                    value: "luck".into()
                }
            ),
            (
                2,
                &SourceReason::Malformed {
                    field: "slot".into(),
                    problem: Problem::WrongKind {
                        expected: Kind::Integer
                    }
                }
            ),
            (3, &SourceReason::UnknownSubAttribute { index: 0 }),
            (
                4,
                &SourceReason::Malformed {
                    field: String::new(),
                    problem: Problem::WrongKind {
                        expected: Kind::Object
                    }
                }
            ),
        ]
    );
}

#[test]
fn only_a_known_header_and_a_complete_file_are_read() {
    let mut other = file(vec![]);
    other["format"] = json!("some-other-v2");
    assert_eq!(
        normalize(&other),
        Err(ImportError::UnknownFormat {
            stated: Some("some-other-v2".into())
        })
    );
    assert_eq!(
        normalize(&json!({"hero_equips": []})),
        Err(ImportError::UnknownFormat { stated: None })
    );
    assert!(matches!(
        normalize(&json!([])),
        Err(ImportError::MalformedSource { .. })
    ));
    assert!(matches!(
        read(b"{"),
        Err(ImportError::MalformedSource { .. })
    ));
    let mut partial = file(vec![]);
    partial["completeness"] = json!("partial");
    assert_eq!(
        normalize(&partial),
        Err(ImportError::UnsupportedSourceValue {
            field: "completeness",
            value: "partial".into()
        })
    );
    let mut no_scope = file(vec![]);
    no_scope.as_object_mut().expect("object").remove("scope");
    assert_eq!(
        normalize(&no_scope),
        Err(ImportError::Shape {
            field: "scope",
            problem: Problem::Missing
        })
    );
}

#[test]
fn a_preset_naming_a_missing_soul_refuses_the_file() {
    let mut f = file(vec![soul("a", 1)]);
    f["equipPresets"] = json!([["主力", ["zz", null, null, null, null, null]]]);
    assert!(matches!(
        normalize(&f),
        Err(ImportError::Ir(IrError::InconsistentReference { .. }))
    ));
}

#[test]
fn two_souls_with_one_id_refuse_the_file() {
    assert!(matches!(
        normalize(&file(vec![soul("a", 1), soul("a", 2)])),
        Err(ImportError::Ir(IrError::DuplicateId { .. }))
    ));
}
