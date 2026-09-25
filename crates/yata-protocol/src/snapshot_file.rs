//! The yata-snapshot file (ADR-0031, `snapshot-ir.md`): one `YataSnapshot` in the proto3 JSON
//! mapping, UTF-8.
//!
//! The version is read before the body, so a file of a future major version is refused as
//! unsupported rather than reported as malformed. A file of the same major and a newer minor is
//! read: an unknown field or section is skipped, and an unknown enum name reads as the enum's
//! zero value, which the conversion into the IR refuses rather than reinterprets.

use crate::snapshot::{self, SchemaVersion, YataSnapshot};

/// The largest file read: the IR's file limit (`snapshot-ir.md`, "Limits").
pub const MAX_FILE_BYTES: usize = 64 * 1024 * 1024;

/// The top-level key that holds the schema version, and recognises the file.
pub const HEADER_KEY: &str = "yataSnapshot";

/// Why a file is not a yata-snapshot this build reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotFileError {
    TooLarge {
        bytes: u64,
    },
    /// Not UTF-8 JSON, or not a JSON object.
    NotJson {
        reason: String,
    },
    /// No `yataSnapshot`, or one that is not a pair of unsigned integers.
    NoVersion,
    UnsupportedVersion {
        found: SchemaVersion,
    },
    /// JSON of the right version that is not a `YataSnapshot`.
    Malformed {
        reason: String,
    },
}

/// Why a snapshot could not be written as JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unwritable {
    pub reason: String,
}

/// The file's text: pretty-printed, ending in a newline.
pub fn to_json(s: &YataSnapshot) -> Result<String, Unwritable> {
    let mut text = serde_json::to_string_pretty(s).map_err(|e| Unwritable {
        reason: e.to_string(),
    })?;
    text.push('\n');
    Ok(text)
}

/// The snapshot a file holds, if this build reads it.
pub fn from_json(bytes: &[u8]) -> Result<YataSnapshot, SnapshotFileError> {
    if bytes.len() > MAX_FILE_BYTES {
        return Err(SnapshotFileError::TooLarge {
            bytes: bytes.len() as u64,
        });
    }
    // A byte-order mark is what some Windows editors put in front of UTF-8; it is not content.
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| SnapshotFileError::NotJson {
            reason: e.to_string(),
        })?;
    if !value.is_object() {
        return Err(SnapshotFileError::NotJson {
            reason: "the top level is not an object".to_owned(),
        });
    }
    let found = version_of(&value).ok_or(SnapshotFileError::NoVersion)?;
    if !snapshot::accepts(found) {
        return Err(SnapshotFileError::UnsupportedVersion { found });
    }
    serde_json::from_value(value).map_err(|e| SnapshotFileError::Malformed {
        reason: e.to_string(),
    })
}

/// The header as proto3 JSON writes it: numbers, or decimal strings, and zero omitted.
pub fn version_of(value: &serde_json::Value) -> Option<SchemaVersion> {
    let version = value.get(HEADER_KEY)?.as_object()?;
    let part = |name: &str| match version.get(name) {
        None => Some(0),
        Some(serde_json::Value::Number(n)) => n.as_u64().and_then(|n| u32::try_from(n).ok()),
        Some(serde_json::Value::String(s)) => s.parse().ok(),
        Some(_) => None,
    };
    Some(SchemaVersion {
        major: part("major")?,
        minor: part("minor")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::{Completeness, GuildSection, Provenance, provenance};

    fn sample() -> YataSnapshot {
        YataSnapshot {
            yata_snapshot: Some(snapshot::VERSION),
            provenance: Some(Provenance {
                format: Some(provenance::Format::YataSnapshot(snapshot::VERSION)),
                original: vec![7; 32],
            }),
            captured_at: None,
            guild: Some(GuildSection {
                completeness: Completeness::Complete.into(),
                level: 3,
                member_count: 0,
            }),
            ..YataSnapshot::default()
        }
    }

    #[test]
    fn a_file_survives_its_json_form() {
        let text = to_json(&sample()).expect("writable");
        assert!(text.contains("\"yataSnapshot\""));
        assert_eq!(from_json(text.as_bytes()), Ok(sample()));
    }

    #[test]
    fn the_version_is_read_first() {
        let newer_major = br#"{"yataSnapshot": {"major": 2}, "souls": "not even a section"}"#;
        assert_eq!(
            from_json(newer_major),
            Err(SnapshotFileError::UnsupportedVersion {
                found: SchemaVersion { major: 2, minor: 0 }
            })
        );
        assert_eq!(
            from_json(br#"{"guild": {}}"#),
            Err(SnapshotFileError::NoVersion)
        );
        assert!(matches!(
            from_json(b"[1]"),
            Err(SnapshotFileError::NotJson { .. })
        ));
    }

    #[test]
    fn a_newer_minor_is_read_and_its_unknown_parts_skipped() {
        let text = br#"{"yataSnapshot": {"major": 1, "minor": 4},
                        "futureSection": {"x": 1},
                        "guild": {"completeness": "COMPLETENESS_SOMEDAY", "level": 3}}"#;
        let s = from_json(text).expect("read");
        let guild = s.guild.expect("present");
        assert_eq!(guild.level, 3);
        assert_eq!(
            guild.completeness,
            i32::from(Completeness::Unspecified),
            "an unknown enum name reads as the zero value, which the IR refuses"
        );
    }
}
