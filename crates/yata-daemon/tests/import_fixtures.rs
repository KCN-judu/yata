//! Import over committed fixtures (ADR-0031): a synthetic `mumu-snapshot-v1` file normalizes to a
//! committed yata-snapshot file, so the adapter's behaviour is reviewable apart from admission;
//! that yata-snapshot imports to the same sections; and the IR's stored form round-trips
//! byte for byte. `YATA_BLESS=1` rewrites the expected file.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    reason = "a failure here is the test failing"
)]

use std::path::{Path, PathBuf};

use yata_core::import::admit::admit;
use yata_core::import::ir::{
    Completeness, FormatTag, Section, SectionKind, SourceFormat, YataSnapshot,
};
use yata_core::soul::SoulKind;
use yata_daemon::import::codec::{SnapshotDecodeError, decode_snapshot, encode_snapshot, to_proto};
use yata_daemon::import::{ImportError, Normalized, export_json, read};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/import")
        .join(name)
}

fn mumu() -> Normalized {
    let bytes = std::fs::read(fixture("mumu-snapshot-v1.json")).expect("fixture");
    read(&bytes).expect("normalized")
}

/// Compare `text` with the committed fixture, or rewrite it when `YATA_BLESS` is set.
fn blessed(name: &str, text: &str) {
    let path = fixture(name);
    if std::env::var_os("YATA_BLESS").is_some() {
        std::fs::write(&path, text).expect("written");
        return;
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "{}: {e}; run with YATA_BLESS=1 to record it",
            path.display()
        )
    });
    assert_eq!(
        committed.replace("\r\n", "\n"),
        text,
        "{} differs from what the adapter writes; rerun with YATA_BLESS=1 if the change is intended",
        path.display()
    );
}

/// Every section but the provenance: what two imports of one snapshot must agree on.
fn sections(s: &YataSnapshot) -> YataSnapshot {
    YataSnapshot {
        provenance: yata_core::import::ir::Provenance {
            format: SourceFormat::Community(FormatTag::MumuSnapshotV1),
            original: yata_core::fact::Digest([0; 32]),
        },
        ..s.clone()
    }
}

#[test]
fn the_community_fixture_normalizes_to_the_committed_ir() {
    let n = mumu();
    assert!(n.left_out.is_empty());
    assert_eq!(
        n.snapshot.sections(),
        SectionKind::ALL
            .into_iter()
            .map(|k| (k, Completeness::Complete))
            .collect::<Vec<_>>()
    );
    blessed(
        "mumu-snapshot-v1.expected.yata-snapshot.json",
        &export_json(&n).expect("writable"),
    );
}

#[test]
fn the_committed_ir_imports_to_the_same_sections() {
    let bytes =
        std::fs::read(fixture("mumu-snapshot-v1.expected.yata-snapshot.json")).expect("fixture");
    let again = read(&bytes).expect("normalized");
    assert!(again.left_out.is_empty());
    assert!(matches!(
        again.snapshot.provenance.format,
        SourceFormat::YataSnapshot(_)
    ));
    assert_eq!(sections(&again.snapshot), sections(&mumu().snapshot));
}

#[test]
fn the_committed_ir_admits_to_domain_values() {
    let admitted = admit(&mumu().snapshot).expect("admitted");
    let souls = admitted.souls.present().expect("present").1;
    assert_eq!(souls.values.len(), 3);
    assert!(souls.rejected.is_empty());
    let kinds: Vec<bool> = souls
        .values
        .iter()
        .map(|(_, s)| matches!(s.kind, SoulKind::Boss(_)))
        .collect();
    assert_eq!(kinds, [false, true, false]);
    let presets = admitted.presets.present().expect("present").1;
    assert_eq!((presets.values.len(), presets.rejected.len()), (1, 0));
    let shikigami = admitted.shikigami.present().expect("present").1;
    assert_eq!(shikigami.values.len(), 2);
}

#[test]
fn the_stored_form_round_trips_byte_for_byte() {
    let s = mumu().snapshot;
    let bytes = encode_snapshot(&s);
    assert_eq!(decode_snapshot(&bytes), Ok(s.clone()));
    assert_eq!(encode_snapshot(&s), bytes, "one snapshot, one encoding");
    let again = decode_snapshot(&bytes).expect("decoded");
    assert_eq!(encode_snapshot(&again), bytes);
    let absent = YataSnapshot {
        guild: Section::Absent,
        ..s
    };
    assert_ne!(
        encode_snapshot(&absent),
        bytes,
        "an absent section changes the bytes"
    );
    assert_eq!(decode_snapshot(&encode_snapshot(&absent)), Ok(absent));
}

#[test]
fn a_stored_form_that_is_not_a_snapshot_is_refused() {
    assert!(matches!(
        decode_snapshot(b"\xff\xff\xff"),
        Err(SnapshotDecodeError::Protobuf { .. })
    ));
    let mut p = to_proto(&mumu().snapshot);
    if let Some(souls) = p.souls.as_mut() {
        souls.souls[0].innate = Some(99);
    }
    use prost::Message;
    assert!(matches!(
        decode_snapshot(&p.encode_to_vec()),
        Err(SnapshotDecodeError::Record(_))
    ));
    let mut no_header = to_proto(&mumu().snapshot);
    no_header.yata_snapshot = None;
    assert_eq!(
        decode_snapshot(&no_header.encode_to_vec()),
        Err(SnapshotDecodeError::Missing {
            field: "yataSnapshot"
        })
    );
}

#[test]
fn a_yata_snapshot_of_a_newer_major_is_refused() {
    let text = br#"{"yataSnapshot": {"major": 2}, "provenance": {}}"#;
    assert!(matches!(
        read(text),
        Err(ImportError::UnsupportedVersion { found }) if found.major == 2
    ));
}

#[test]
fn a_yata_record_this_build_cannot_read_is_left_out_and_its_section_partial() {
    let bytes =
        std::fs::read(fixture("mumu-snapshot-v1.expected.yata-snapshot.json")).expect("fixture");
    let text = String::from_utf8(bytes).expect("utf-8").replacen(
        "CURRENCY_JADE",
        "CURRENCY_FROM_THE_FUTURE",
        1,
    );
    let n = read(text.as_bytes()).expect("normalized");
    assert_eq!(n.left_out.len(), 1);
    assert_eq!(
        n.snapshot.assets.present().map(|(c, _)| c),
        Some(Completeness::Partial)
    );
    assert_eq!(
        n.snapshot.souls.present().map(|(c, _)| c),
        Some(Completeness::Complete),
        "the other sections keep their completeness"
    );
}

#[test]
fn a_yata_soul_left_out_that_a_preset_names_refuses_the_file() {
    let bytes =
        std::fs::read(fixture("mumu-snapshot-v1.expected.yata-snapshot.json")).expect("fixture");
    let text = String::from_utf8(bytes).expect("utf-8").replacen(
        "ATTRIBUTE_ATK_FLAT",
        "ATTRIBUTE_FROM_THE_FUTURE",
        1,
    );
    assert!(matches!(
        read(text.as_bytes()),
        Err(ImportError::Ir(
            yata_core::import::ir::IrError::InconsistentReference { .. }
        ))
    ));
}
