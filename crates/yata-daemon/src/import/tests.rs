//! Import over synthetic files in the shape of `mumu-snapshot-v1`. No real account data is used.

use serde_json::{Value, json};
use yata_core::import::snapshot::{FormatTag, SoulDefect};
use yata_core::soul::{InnateAttribute, SoulAttribute, SoulKind, SoulSet, SoulSlot};

use super::*;

fn soul(id: &str) -> Value {
    json!({
        "id": id,
        "setId": "破势",
        "slot": 6,
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

fn file(souls: Vec<Value>) -> Value {
    json!({
        "format": "mumu-snapshot-v1",
        "completeness": "complete",
        "capturedAt": "2026-09-25T09:10:32Z",
        "hero_equips": souls,
        "currency": {},
    })
}

fn read_value(v: &Value) -> Result<Imported, ImportError> {
    read(&serde_json::to_vec(v).expect("serialised"))
}

#[test]
fn a_file_becomes_domain_souls_in_display_units() {
    let imported = read_value(&file(vec![soul("a")])).expect("imported");
    assert_eq!(imported.format, FormatTag::MumuSnapshotV1);
    assert_eq!(imported.captured_at, "2026-09-25T09:10:32Z");
    assert!(imported.defects.is_empty());
    let (id, s) = &imported.souls[0];
    assert_eq!(id.as_str(), "a");
    assert_eq!(s.set, SoulSet::from_suit_code(30));
    assert_eq!(s.slot, SoulSlot::Slot6);
    assert_eq!(s.main, SoulAttribute::CritDmg);
    assert!(
        (s.main_value.get() - 89.0).abs() < 1e-9,
        "a rate becomes percentage points"
    );
    assert!((s.subs[0].value.get() - 3.0).abs() < 1e-9);
    assert!(
        (s.subs[1].value.get() - 11.35).abs() < 1e-9,
        "speed is kept as shown"
    );
    assert_eq!(s.kind, SoulKind::Ordinary);
}

#[test]
fn the_innate_entry_becomes_the_boss_kind() {
    let mut boss = soul("b");
    boss["setId"] = json!("荒骷髅");
    boss["subAttributes"]
        .as_array_mut()
        .expect("array")
        .push(json!({"type": "effect_resist", "value": 0.08, "enhancementCount": null, "fixedAttribute": true}));
    let imported = read_value(&file(vec![boss])).expect("imported");
    let (_, s) = &imported.souls[0];
    assert_eq!(
        s.subs.len(),
        2,
        "the innate attribute is not a sub-attribute"
    );
    assert_eq!(
        s.kind,
        SoulKind::Boss(InnateAttribute::new(SoulAttribute::EffectRes).expect("innate"))
    );
}

#[test]
fn only_a_known_header_is_read() {
    let mut other = file(vec![]);
    other["format"] = json!("some-other-v2");
    assert_eq!(
        read_value(&other),
        Err(ImportError::Unrecognised {
            stated: Some("some-other-v2".into())
        })
    );
    assert_eq!(
        read_value(&json!({"hero_equips": []})),
        Err(ImportError::Unrecognised { stated: None })
    );
    assert_eq!(read_value(&json!([])), Err(ImportError::NotAnObject));
    assert!(matches!(read(b"{"), Err(ImportError::NotJson { .. })));
}

#[test]
fn a_file_that_is_not_complete_is_refused() {
    let mut partial = file(vec![soul("a")]);
    partial["completeness"] = json!("partial");
    assert_eq!(
        read_value(&partial),
        Err(ImportError::NotComplete {
            stated: "partial".into()
        })
    );
    let mut none = file(vec![]);
    none.as_object_mut().expect("object").remove("completeness");
    assert_eq!(
        read_value(&none),
        Err(ImportError::Shape {
            field: "completeness",
            problem: Problem::Missing
        })
    );
}

#[test]
fn two_records_of_one_soul_refuse_the_file() {
    assert_eq!(
        read_value(&file(vec![soul("a"), soul("a")])),
        Err(ImportError::DuplicateSoul { id: "a".into() })
    );
}

#[test]
fn a_bad_record_is_left_out_and_named() {
    let mut no_rolls = soul("r");
    no_rolls["subAttributes"][1]["enhancementCount"] = Value::Null;
    let mut unknown = soul("u");
    unknown["mainAttrType"] = json!("luck");
    let mut stringly = soul("s");
    stringly["slot"] = json!("6");
    let mut marked = soul("m");
    marked["subAttributes"][0]["fixedAttribute"] = json!(false);
    let mut unset = soul("n");
    unset["subAttributes"][0]
        .as_object_mut()
        .expect("object")
        .remove("enhancementCount");
    let imported = read_value(&file(vec![
        soul("ok"),
        no_rolls,
        unknown,
        stringly,
        marked,
        unset,
        json!(7),
    ]))
    .expect("imported");
    assert_eq!(imported.souls.len(), 1);
    let reasons: Vec<(usize, Option<&str>, &RecordReason)> = imported
        .defects
        .iter()
        .map(|d| (d.index, d.id.as_deref(), &d.reason))
        .collect();
    assert_eq!(
        reasons,
        vec![
            (
                1,
                Some("r"),
                &RecordReason::Soul(SoulDefect::MissingRolls { sub: 1 })
            ),
            (
                2,
                Some("u"),
                &RecordReason::UnknownAttribute {
                    field: "mainAttrType".into(),
                    name: "luck".into()
                }
            ),
            (
                3,
                Some("s"),
                &RecordReason::Shape {
                    field: "slot".into(),
                    problem: Problem::WrongKind {
                        expected: Kind::Integer
                    }
                }
            ),
            (
                4,
                Some("m"),
                &RecordReason::UnknownSubAttribute { index: 0 }
            ),
            (
                5,
                Some("n"),
                &RecordReason::Shape {
                    field: "subAttributes[0].enhancementCount".into(),
                    problem: Problem::Missing
                }
            ),
            (
                6,
                None,
                &RecordReason::Shape {
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
fn an_empty_inventory_is_an_inventory() {
    let imported = read_value(&file(vec![])).expect("imported");
    assert!(imported.souls.is_empty() && imported.defects.is_empty());
}
