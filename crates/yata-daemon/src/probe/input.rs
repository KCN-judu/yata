//! Readings from a file: a recording (the frame stream, ADR-0006) or an export (a `ProbeExport`
//! in proto3 JSON, ADR-0008). Both arrive as the same wire `Reading`s, so everything downstream is
//! indifferent to how a reading travelled.

use std::path::{Path, PathBuf};

use yata_protocol::discipline::RequestId;
use yata_protocol::export::{self, ExportError};
use yata_protocol::probe::{Channel, ProbeExport, ProtocolVersion, Reading, TargetProcess};

use super::session::{self, ProbeFailure, SessionError};

/// The largest recording read: the export file's bound.
pub const MAX_RECORDING_BYTES: u64 = export::MAX_EXPORT_BYTES as u64;

/// Who produced a reading, as the reading itself states it.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    pub protocol_version: ProtocolVersion,
    pub probe_build_id: String,
    pub engine: String,
    pub channel: Channel,
    pub target: Option<TargetProcess>,
}

/// How the readings travelled, and what only that way of travelling carries.
#[derive(Debug, Clone, PartialEq)]
pub enum Carrier {
    /// A frame stream. It states no capture time, and it holds the reader's failures.
    Recording {
        failures: Vec<(Option<RequestId>, ProbeFailure)>,
    },
    /// An export file, with its capture time when it states one.
    Export { captured_at: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loaded {
    pub carrier: Carrier,
    /// `None` for a recording with no `HandshakeAck`: a reader that failed before attaching.
    pub provenance: Option<Provenance>,
    pub readings: Vec<Reading>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    Io {
        path: PathBuf,
        reason: String,
    },
    TooLarge {
        path: PathBuf,
        bytes: u64,
    },
    Recording(SessionError),
    Export(ExportError),
    /// A `HandshakeAck` or an export without a protocol version.
    NoVersion,
}

/// Why a reading cannot be written as an export.
#[derive(Debug, Clone, PartialEq)]
pub enum ToExportError {
    /// The recording states no provenance to put in the file.
    NoProvenance,
}

/// Read a recording or an export from a file, by its content.
pub fn load(path: &Path) -> Result<Loaded, InputError> {
    let io = |e: std::io::Error| InputError::Io {
        path: path.to_owned(),
        reason: e.to_string(),
    };
    let bytes = std::fs::metadata(path).map_err(io)?.len();
    if bytes > MAX_RECORDING_BYTES {
        return Err(InputError::TooLarge {
            path: path.to_owned(),
            bytes,
        });
    }
    load_bytes(&std::fs::read(path).map_err(io)?)
}

/// An export is a JSON object, so its first byte after a byte-order mark and white space is `{`.
/// A recording starts with a frame's length prefix, whose first byte is 0 or 1 for any legal
/// frame, so the two cannot be mistaken for each other.
pub fn load_bytes(bytes: &[u8]) -> Result<Loaded, InputError> {
    let text = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    let first = text.iter().find(|b| !b.is_ascii_whitespace());
    if first == Some(&b'{') {
        from_export(export::from_json(bytes).map_err(InputError::Export)?)
    } else {
        from_recording(bytes)
    }
}

fn from_export(e: ProbeExport) -> Result<Loaded, InputError> {
    let channel = e.channel();
    Ok(Loaded {
        provenance: Some(Provenance {
            protocol_version: e.protocol_version.ok_or(InputError::NoVersion)?,
            probe_build_id: e.probe_build_id,
            engine: e.engine,
            channel,
            target: e.target,
        }),
        carrier: Carrier::Export {
            captured_at: e.captured_at,
        },
        readings: e.readings,
    })
}

fn from_recording(bytes: &[u8]) -> Result<Loaded, InputError> {
    let replay = session::replay(bytes).map_err(InputError::Recording)?;
    let provenance = replay
        .ack()
        .map(|a| {
            Ok(Provenance {
                protocol_version: a.version.ok_or(InputError::NoVersion)?,
                probe_build_id: a.probe_build_id.clone(),
                engine: a.engine.clone(),
                channel: a.channel(),
                target: a.target.clone(),
            })
        })
        .transpose()?;
    Ok(Loaded {
        carrier: Carrier::Recording {
            failures: replay.failures().map(|(id, f)| (id, f.clone())).collect(),
        },
        provenance,
        readings: replay.readings().cloned().collect(),
    })
}

/// Readings as an export file's message: what `probe to-export` writes. The protocol version is
/// the one the readings were captured under, and the capture time is the carrier's, if any.
pub fn to_export(loaded: &Loaded) -> Result<ProbeExport, ToExportError> {
    let p = loaded
        .provenance
        .as_ref()
        .ok_or(ToExportError::NoProvenance)?;
    Ok(ProbeExport {
        protocol_version: Some(p.protocol_version),
        probe_build_id: p.probe_build_id.clone(),
        engine: p.engine.clone(),
        channel: p.channel.into(),
        readings: loaded.readings.clone(),
        captured_at: match &loaded.carrier {
            Carrier::Export { captured_at } => captured_at.clone(),
            Carrier::Recording { .. } => None,
        },
        target: p.target.clone(),
    })
}
