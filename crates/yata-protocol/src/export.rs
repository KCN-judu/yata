//! The export file (ADR-0008): one `ProbeExport` in the proto3 JSON mapping, UTF-8.
//!
//! The version is checked before the body is parsed, so a file from a future major version is
//! refused as unsupported rather than reported as malformed. A file of the same major and a
//! newer minor is accepted: its unknown fields are skipped and an unknown enum name reads as the
//! enum's zero value (`protocol-versions.md`, "Compatibility rule"). Nothing here depends on
//! the platform the file was written on.

use crate::probe::{self, ProbeExport, ProtocolVersion};

/// The largest export file read: room for a large inventory, and a bound on what a malformed
/// file can make the parser allocate.
pub const MAX_EXPORT_BYTES: usize = 64 * 1024 * 1024;

/// Why a file is not an export this build accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportError {
    /// Larger than [`MAX_EXPORT_BYTES`].
    TooLarge { bytes: u64 },
    /// Not UTF-8 JSON, or not a JSON object.
    NotJson { reason: String },
    /// No `protocolVersion`, or one that is not a pair of unsigned integers.
    NoVersion,
    /// A protocol major version this build does not read.
    UnsupportedVersion { found: ProtocolVersion },
    /// JSON of the right version that is not a `ProbeExport`.
    Malformed { reason: String },
}

/// The file's text for an export: pretty-printed, ending in a newline.
pub fn to_json(export: &ProbeExport) -> String {
    // The generated mapping writes every value it holds, including non-finite floats as the
    // strings proto3 JSON names them, so serializing to a string cannot fail.
    let mut text = serde_json::to_string_pretty(export).unwrap_or_default();
    text.push('\n');
    text
}

/// The export a file holds, if this build accepts it.
pub fn from_json(bytes: &[u8]) -> Result<ProbeExport, ExportError> {
    if bytes.len() > MAX_EXPORT_BYTES {
        return Err(ExportError::TooLarge {
            bytes: bytes.len() as u64,
        });
    }
    // A byte-order mark is what some Windows editors put in front of UTF-8; it is not content.
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| ExportError::NotJson {
            reason: e.to_string(),
        })?;
    if !value.is_object() {
        return Err(ExportError::NotJson {
            reason: "the top level is not an object".to_owned(),
        });
    }
    let found = version_of(&value).ok_or(ExportError::NoVersion)?;
    if !probe::accepts(found) {
        return Err(ExportError::UnsupportedVersion { found });
    }
    serde_json::from_value(value).map_err(|e| ExportError::Malformed {
        reason: e.to_string(),
    })
}

/// `protocolVersion` as proto3 JSON writes it: numbers, or decimal strings, and zero omitted.
fn version_of(value: &serde_json::Value) -> Option<ProtocolVersion> {
    let version = value.get("protocolVersion")?.as_object()?;
    let part = |name: &str| match version.get(name) {
        None => Some(0),
        Some(serde_json::Value::Number(n)) => n.as_u64().and_then(|n| u32::try_from(n).ok()),
        Some(serde_json::Value::String(s)) => s.parse().ok(),
        Some(_) => None,
    };
    Some(ProtocolVersion {
        major: part("major")?,
        minor: part("minor")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::{
        Channel, Coverage, Evidence, FieldEvidence, ObservedRecord, RawEntry, RawNull, RawSequence,
        RawValue, ReadResult, Scope, SequenceKind, SoulRecord, SoulRecords, TargetProcess,
        raw_value::Kind, read_result::Records,
    };

    fn text(s: &str) -> RawValue {
        RawValue {
            kind: Some(Kind::Text(s.to_owned())),
        }
    }

    fn integer(n: i64) -> RawValue {
        RawValue {
            kind: Some(Kind::Integer(n)),
        }
    }

    fn sample() -> ProbeExport {
        let observed = ObservedRecord {
            type_name: "record".into(),
            container_key: Some(text("0123456789abcdef01234567")),
            entries: vec![
                RawEntry {
                    key: Some(text("a")),
                    value: Some(integer(i64::MIN)),
                },
                RawEntry {
                    key: Some(text("b")),
                    value: Some(RawValue {
                        kind: Some(Kind::Null(RawNull {})),
                    }),
                },
                RawEntry {
                    key: Some(integer(7)),
                    value: Some(RawValue {
                        kind: Some(Kind::Sequence(RawSequence {
                            kind: SequenceKind::Tuple.into(),
                            items: vec![integer(1), text("二")],
                            truncated: false,
                            length: 2,
                        })),
                    }),
                },
            ],
        };
        ProbeExport {
            protocol_version: Some(probe::VERSION),
            probe_build_id: "test".into(),
            engine: "synthetic".into(),
            channel: Channel::DesktopMemory.into(),
            captured_at: "2026-09-25T00:00:00Z".into(),
            target: Some(TargetProcess {
                pid: 42,
                image_name: "game.exe".into(),
                pointer_bits: 64,
                created_unix_ms: u64::MAX,
                ..TargetProcess::default()
            }),
            results: vec![ReadResult {
                request_id: 1,
                scope: Scope::Souls.into(),
                coverage: Coverage::Partial.into(),
                records: Some(Records::Souls(SoulRecords {
                    souls: vec![SoulRecord {
                        soul_id: Some("0123456789abcdef01234567".into()),
                        observed: Some(observed),
                        ..SoulRecord::default()
                    }],
                })),
                field_evidence: vec![FieldEvidence {
                    field: "SoulRecord.soul_id".into(),
                    evidence: Evidence::Inherited.into(),
                    basis: String::new(),
                }],
                ..ReadResult::default()
            }],
        }
    }

    #[test]
    fn an_export_survives_its_json_text() {
        let export = sample();
        assert_eq!(from_json(to_json(&export).as_bytes()), Ok(export));
    }

    #[test]
    fn the_text_follows_the_proto3_json_mapping() {
        let json = to_json(&sample());
        for expected in [
            "\"protocolVersion\"",
            "\"CHANNEL_DESKTOP_MEMORY\"",
            "\"COVERAGE_PARTIAL\"",
            "\"EVIDENCE_INHERITED\"",
            "\"SEQUENCE_KIND_TUPLE\"",
            // 64-bit integers are strings.
            "\"-9223372036854775808\"",
            "\"18446744073709551615\"",
            "\"null\": {}",
        ] {
            assert!(json.contains(expected), "{expected} missing from\n{json}");
        }
        assert!(json.ends_with("}\n"));
    }

    #[test]
    fn an_unset_typed_field_stays_unset() {
        let json = to_json(&sample());
        assert!(!json.contains("\"star\""));
        assert!(!json.contains("\"locked\""));
        let back = from_json(json.as_bytes()).expect("accepted");
        let Some(Records::Souls(souls)) = &back.results[0].records else {
            panic!("souls")
        };
        assert_eq!(souls.souls[0].star, None);
        assert_eq!(souls.souls[0].locked, None);
    }

    #[test]
    fn a_byte_order_mark_and_windows_line_ends_are_not_content() {
        let export = sample();
        let mut bytes = b"\xEF\xBB\xBF".to_vec();
        bytes.extend(to_json(&export).replace('\n', "\r\n").into_bytes());
        assert_eq!(from_json(&bytes), Ok(export));
    }

    #[test]
    fn a_future_major_version_is_unsupported_not_malformed() {
        let json = r#"{"protocolVersion": {"major": 2}, "results": [{"newThing": 1}], "x": 3}"#;
        assert_eq!(
            from_json(json.as_bytes()),
            Err(ExportError::UnsupportedVersion {
                found: ProtocolVersion { major: 2, minor: 0 }
            })
        );
    }

    #[test]
    fn a_missing_version_is_refused() {
        assert_eq!(from_json(b"{}"), Err(ExportError::NoVersion));
        assert_eq!(
            from_json(br#"{"protocolVersion": {"major": -1}}"#),
            Err(ExportError::NoVersion)
        );
    }

    #[test]
    fn a_newer_minor_version_is_read_without_its_new_fields() {
        let json = r#"{
            "protocolVersion": {"major": 1, "minor": 7},
            "engine": "e",
            "channel": "CHANNEL_FROM_THE_FUTURE",
            "fieldFromTheFuture": [1, 2, 3]
        }"#;
        let export = from_json(json.as_bytes()).expect("same major");
        assert_eq!(export.engine, "e");
        assert_eq!(export.channel(), Channel::Unspecified);
    }

    #[test]
    fn a_wrong_value_kind_is_malformed() {
        let json = r#"{"protocolVersion": {"major": 1}, "results": "none"}"#;
        assert!(matches!(
            from_json(json.as_bytes()),
            Err(ExportError::Malformed { .. })
        ));
    }

    #[test]
    fn non_json_and_non_objects_are_not_exports() {
        assert!(matches!(
            from_json(b"\x00\x01"),
            Err(ExportError::NotJson { .. })
        ));
        assert!(matches!(
            from_json(b"[1]"),
            Err(ExportError::NotJson { .. })
        ));
    }

    #[test]
    fn an_oversized_file_is_refused_before_parsing() {
        let big = vec![b' '; MAX_EXPORT_BYTES + 1];
        assert_eq!(
            from_json(&big),
            Err(ExportError::TooLarge {
                bytes: MAX_EXPORT_BYTES as u64 + 1
            })
        );
    }
}
