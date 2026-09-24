//! Readings from a file: a recording (the frame stream, ADR-0006) or an export (a `ProbeExport`
//! in proto3 JSON, ADR-0008). Both arrive as the same `ReadResult`s with the same provenance, so
//! everything downstream is indifferent to how a reading travelled.

use std::path::{Path, PathBuf};

use yata_protocol::export::{self, ExportError};
use yata_protocol::probe::{
    self, Channel, Failed, ProbeExport, ProtocolVersion, ReadResult, TargetProcess,
};

use super::session::{self, SessionError};

/// The largest recording read: the export file's bound.
pub const MAX_RECORDING_BYTES: u64 = export::MAX_EXPORT_BYTES as u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Carrier {
    Recording,
    Export,
}

/// Who produced a reading, as the reading itself states it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Provenance {
    pub protocol_version: Option<ProtocolVersion>,
    pub probe_build_id: String,
    pub engine: String,
    pub channel: Channel,
    pub target: Option<TargetProcess>,
    /// Only an export states when it was taken.
    pub captured_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loaded {
    pub carrier: Carrier,
    pub provenance: Provenance,
    pub results: Vec<ReadResult>,
    /// A recording's `Failed` answers; an export holds none.
    pub failures: Vec<Failed>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    Io { path: PathBuf, reason: String },
    TooLarge { path: PathBuf, bytes: u64 },
    Recording(SessionError),
    Export(ExportError),
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
    Ok(Loaded {
        carrier: Carrier::Export,
        provenance: Provenance {
            protocol_version: e.protocol_version,
            probe_build_id: e.probe_build_id.clone(),
            engine: e.engine.clone(),
            channel: e.channel(),
            target: e.target,
            captured_at: Some(e.captured_at).filter(|c| !c.is_empty()),
        },
        results: e.results,
        failures: Vec::new(),
    })
}

fn from_recording(bytes: &[u8]) -> Result<Loaded, InputError> {
    let replay = session::replay(bytes).map_err(InputError::Recording)?;
    let provenance = replay
        .ack()
        .map(|a| Provenance {
            protocol_version: a.version,
            probe_build_id: a.probe_build_id.clone(),
            engine: a.engine.clone(),
            channel: a.channel(),
            target: a.target.clone(),
            captured_at: None,
        })
        .unwrap_or_default();
    Ok(Loaded {
        carrier: Carrier::Recording,
        provenance,
        results: replay.results().cloned().collect(),
        failures: replay.failures().cloned().collect(),
    })
}

/// A reading as an export file's message: what `probe to-export` writes, and what the reader's
/// export mode writes from a live read.
pub fn to_export(loaded: &Loaded, captured_at: &str) -> ProbeExport {
    let p = &loaded.provenance;
    ProbeExport {
        protocol_version: Some(p.protocol_version.unwrap_or(probe::VERSION)),
        probe_build_id: p.probe_build_id.clone(),
        engine: p.engine.clone(),
        channel: p.channel.into(),
        results: loaded.results.clone(),
        captured_at: captured_at.to_owned(),
        target: p.target.clone(),
    }
}
