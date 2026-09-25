//! `yata-snapshot` (`spec/snapshot-ir.md`, "The `yata-snapshot` file"): Yata's own format, the IR
//! in the proto3 JSON mapping of `snapshot.proto`. Recognised by its header; the version is read
//! before the body.

use serde_json::{Map, Value};
use yata_core::fact::Digest;
use yata_core::import::ir::{Provenance, SchemaVersion, SourceFormat};
use yata_protocol::snapshot::{self as pb, accepts};
use yata_protocol::snapshot_file::{HEADER_KEY, version_of};

use super::codec::from_proto;
use super::{ImportError, Kind, Normalized, Problem};

/// Recognised when the top level holds the header key.
pub(super) fn recognises(header: &Map<String, Value>) -> bool {
    header.contains_key(HEADER_KEY)
}

/// A yata-snapshot file, read into the IR. The snapshot this build holds is in its own schema
/// version, and its provenance is the file just imported: the file's own provenance was that of
/// an earlier import, and is replaced.
pub(super) fn normalize(
    file: Map<String, Value>,
    original: Digest,
) -> Result<Normalized, ImportError> {
    let file = Value::Object(file);
    let found = version_of(&file).ok_or(ImportError::Shape {
        field: HEADER_KEY,
        problem: Problem::WrongKind {
            expected: Kind::Object,
        },
    })?;
    if !accepts(found) {
        return Err(ImportError::UnsupportedVersion {
            found: SchemaVersion {
                major: found.major,
                minor: found.minor,
            },
        });
    }
    let proto: pb::YataSnapshot =
        serde_json::from_value(file).map_err(|e| ImportError::MalformedSource {
            reason: e.to_string(),
        })?;
    let (mut snapshot, left_out) = from_proto(&proto).map_err(ImportError::Decode)?;
    snapshot.schema = SchemaVersion::CURRENT;
    snapshot.provenance = Provenance {
        format: SourceFormat::YataSnapshot(SchemaVersion {
            major: found.major,
            minor: found.minor,
        }),
        original,
    };
    Ok(Normalized { snapshot, left_out })
}
