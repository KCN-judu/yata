//! Replay of a recording the reader made: its session over synthetic memory, captured frame for
//! frame (`yata-reader`, `tests/fixtures.rs`, pinned copy). The replay goes through the live
//! decoder and rules into typed observations, and must decode the same way every time.

use std::path::PathBuf;

use yata_core::import::evidence::{self, Attestation, BitStatus, GroupKey, Source, Unjoined};
use yata_core::import::observation::{Coverage, Evidence, RawValue, SoulField, SoulReading};
use yata_daemon::probe::convert::soul_reading;
use yata_daemon::probe::input::{Carrier, load};
use yata_protocol::probe::Channel;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn reading() -> SoulReading {
    let loaded = load(&fixture("synthetic-souls.frames")).expect("a whole capture");
    assert_eq!(loaded.carrier, Carrier::Recording);
    assert_eq!(loaded.results.len(), 1);
    soul_reading(&loaded.results[0]).expect("a soul reading")
}

#[test]
fn the_recording_carries_its_provenance() {
    let loaded = load(&fixture("synthetic-souls.frames")).expect("a whole capture");
    let p = &loaded.provenance;
    assert!(!p.engine.is_empty());
    assert_eq!(p.probe_build_id, "yata-reader synthetic-fixture");
    assert_eq!(p.channel, Channel::DesktopMemory);
    assert_eq!(
        p.target.as_ref().map(|t| t.image_name.as_str()),
        Some("synthetic.exe")
    );
    assert!(loaded.failures.is_empty());
}

#[test]
fn the_souls_decode_to_observations_with_nothing_typed() {
    let r = reading();
    assert_eq!(r.coverage, Coverage::Partial);
    assert_eq!(r.recognition, Some(Evidence::Inherited));
    for f in SoulField::ALL {
        assert_eq!(r.evidence_of(f), None, "{f:?} is not mapped by this reader");
    }
    assert_eq!(r.souls.len(), 4);
    assert!(
        r.souls
            .iter()
            .all(|s| s.soul_id.is_none() && s.suit_code.is_none())
    );
    let keys: Vec<GroupKey> = r
        .souls
        .iter()
        .map(|s| GroupKey::of(Source::Container.value(s).as_ref()))
        .collect();
    assert_eq!(
        keys,
        vec![
            GroupKey::Other("\"000000000000000000000001\"".into()),
            GroupKey::Other("\"000000000000000000000002\"".into()),
            GroupKey::Other("\"000000000000000000000003\"".into()),
            GroupKey::Absent,
        ]
    );
}

#[test]
fn a_survey_of_the_recording_is_deterministic() {
    let r = reading();
    let s = evidence::survey(&r.souls);
    let keys: Vec<(&str, u64)> = s.keys.iter().map(|k| (k.key.as_str(), k.records)).collect();
    assert_eq!(
        keys,
        vec![
            ("\"base_r\"", 4),
            ("\"base_rindex\"", 4),
            ("\"others\"", 4),
            ("\"rattr\"", 4),
            ("\"sattr\"", 4),
            ("\"single_attr\"", 4),
        ]
    );
    assert_eq!(evidence::survey(&r.souls), s);
}

#[test]
fn presence_and_absence_of_a_value_are_told_apart() {
    // What an innate-attribute question needs: per group, how many souls hold null, a value, or
    // nothing under a key.
    let r = reading();
    let t = evidence::crosstab(
        &r.souls,
        &Source::Entry("base_rindex".into()),
        &Source::Entry("single_attr".into()),
    );
    let null = GroupKey::Other("null".into());
    assert_eq!(t.get(&(GroupKey::Integer(1), null.clone())), Some(&1));
    assert_eq!(
        t.get(&(GroupKey::Integer(2), GroupKey::Integer(7))),
        Some(&1)
    );
    assert_eq!(
        t.get(&(GroupKey::Integer(4), GroupKey::Integer(9))),
        Some(&1)
    );
    let absent = evidence::crosstab(
        &r.souls,
        &Source::Entry("base_rindex".into()),
        &Source::Entry("no_such_key".into()),
    );
    assert!(absent.keys().all(|(_, c)| *c == GroupKey::Absent));
}

#[test]
fn attestations_against_synthetic_values_leave_the_inheritance_standing() {
    // The synthetic souls carry no suit code. Joined to an integer entry, an attestation is
    // compared with the inherited code as it is: bit 1's inherited code is 3, and the attested
    // soul holds 2, so the bit is contradicted. One agreeing bit proves little — the same soul
    // attested for bit 0, whose inherited code is 2, would agree by coincidence — which is why
    // only all mapped bits agreeing, one code to one bit, retires the inheritance.
    let r = reading();
    let ledger = evidence::suit_ledger(
        &r.souls,
        &Source::Container,
        &Source::Entry("base_rindex".into()),
        0,
        &[
            Attestation {
                identity: "000000000000000000000002".into(),
                bit: 1,
            },
            Attestation {
                identity: "nowhere".into(),
                bit: 2,
            },
        ],
    );
    assert_eq!(ledger.rows[1].status, BitStatus::Contradicted);
    assert_eq!(
        ledger.rows[1].observed.iter().copied().collect::<Vec<_>>(),
        vec![2]
    );
    assert_eq!(ledger.rows[0].status, BitStatus::Unattested);
    assert_eq!(ledger.rows[2].status, BitStatus::Unattested);
    assert_eq!(
        ledger.unjoined,
        vec![Unjoined::NoSoul {
            identity: "nowhere".into()
        }]
    );
    assert!(!ledger.retires_inheritance());
}

#[test]
fn nested_values_arrive_as_the_reader_saw_them() {
    let r = reading();
    let rattr = r.souls[2]
        .observed
        .as_ref()
        .and_then(|o| o.get("rattr"))
        .cloned();
    let Some(RawValue::Sequence { items, length, .. }) = rattr else {
        panic!("a list")
    };
    assert_eq!(length, 3);
    assert!(matches!(items[1], RawValue::Sequence { .. }));
}
