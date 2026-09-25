use crate::fact::GameSoulId;
use crate::soul::{InnateAttribute, SoulAttribute, SoulKind, SoulSet, SoulSlot};

use super::super::fixture::*;
use super::super::ir::*;
use super::*;

fn souls(a: &AdmittedSnapshot) -> &Admitted<(GameSoulId, Soul), SoulAdmissionError> {
    a.souls.present().expect("present").1
}

#[test]
fn a_record_becomes_a_domain_soul() {
    let (sid, s) = admit_soul(&soul("a", 2)).expect("admitted");
    assert_eq!(sid.as_str(), "a");
    assert_eq!(s.set, SoulSet::from_suit_code(30));
    assert_eq!(s.slot, SoulSlot::Slot2);
    assert_eq!(s.kind, SoulKind::Ordinary);
    let boss = SoulRecord {
        innate: Some(SoulAttribute::EffectRes),
        ..soul("b", 1)
    };
    assert_eq!(
        admit_soul(&boss).map(|(_, s)| s.kind),
        Ok(SoulKind::Boss(
            InnateAttribute::new(SoulAttribute::EffectRes).expect("innate")
        ))
    );
}

#[test]
fn each_soul_defect_is_named() {
    use SoulAdmissionError::*;
    let with = |f: fn(&mut SoulRecord)| {
        let mut r = soul("a", 1);
        f(&mut r);
        admit_soul(&r).map(|_| ())
    };
    assert_eq!(
        with(|r| r.set = SetName::new("破势x").expect("name")),
        Err(UnknownSet {
            name: "破势x".into()
        })
    );
    assert_eq!(with(|r| r.slot = 7), Err(NotASlot { n: 7 }));
    assert_eq!(with(|r| r.slot = 0), Err(NotASlot { n: 0 }));
    assert_eq!(with(|r| r.star = 7), Err(NotAStar { n: 7 }));
    assert_eq!(with(|r| r.level = 16), Err(NotALevel { n: 16 }));
    assert!(matches!(
        with(|r| r.main.value = -1.0),
        Err(NotAValue {
            at: ValueAt::Main,
            ..
        })
    ));
    assert_eq!(
        with(|r| r.rolled[1].rolls = None),
        Err(MissingRolls { sub: 1 })
    );
    assert_eq!(
        with(|r| r.rolled[0].rolls = Some(-1)),
        Err(NotARollCount { sub: 0, n: -1 })
    );
    assert_eq!(
        with(|r| r.innate = Some(SoulAttribute::Spd)),
        Err(NotInnate {
            attribute: SoulAttribute::Spd
        })
    );
}

#[test]
fn a_bad_record_is_left_out_and_the_rest_kept() {
    let admitted = admit(&with_souls(vec![soul("a", 1), soul("b", 9)])).expect("admitted");
    let s = souls(&admitted);
    assert_eq!(s.values.len(), 1);
    assert_eq!(
        s.rejected,
        vec![Rejected {
            index: 1,
            id: Some(id("b")),
            reason: SoulAdmissionError::NotASlot { n: 9 }
        }]
    );
}

#[test]
fn absent_sections_stay_absent() {
    let admitted = admit(&with_souls(vec![])).expect("admitted");
    assert_eq!(admitted.shikigami, Section::Absent);
    assert_eq!(admitted.presets, Section::Absent);
    assert_eq!(admitted.guild, Section::Absent);
    assert!(souls(&admitted).values.is_empty());
}

#[test]
fn a_shikigami_is_admitted_in_range() {
    let s = admit_shikigami(&shikigami("h", 40)).expect("admitted");
    assert_eq!(s.level.get(), 40);
    assert_eq!(
        admit_shikigami(&shikigami("h", 41)),
        Err(ShikigamiAdmissionError::NotALevel { n: 41 })
    );
}

#[test]
fn a_preset_keeps_each_soul_in_its_position() {
    let snap = YataSnapshot {
        presets: complete(GamePresets {
            presets: vec![
                preset([Some("a"), Some("b"), None, None, None, None]),
                preset([Some("b"), None, None, None, None, None]),
            ],
        }),
        ..with_souls(vec![soul("a", 1), soul("b", 2)])
    };
    let admitted = admit(&snap).expect("admitted");
    let p = admitted.presets.present().expect("present").1;
    assert_eq!(p.values.len(), 1);
    assert_eq!(
        p.values[0].souls[1].as_ref().map(GameSoulId::as_str),
        Some("b")
    );
    assert_eq!(
        p.rejected[0].reason,
        PresetAdmissionError::WrongPosition {
            position: 0,
            soul: id("b"),
            slot: 2
        }
    );
}

#[test]
fn a_snapshot_that_breaks_the_ir_is_refused_whole() {
    assert!(matches!(
        admit(&with_souls(vec![soul("a", 1), soul("a", 1)])),
        Err(IrError::DuplicateId { .. })
    ));
}
