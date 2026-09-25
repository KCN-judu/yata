//! Readings from a file: a recording (the frame stream, ADR-0006) or an export (a `ProbeExport`
//! in proto3 JSON, ADR-0008). Both arrive as the same wire `Reading`s, so everything downstream is
//! indifferent to how a reading travelled.
//!
//! The header around the readings — who read them, over which channel, from which process — is
//! parsed here, once, into [`Provenance`]: an unspecified channel, an unstated pointer width, or
//! an empty engine name is refused, not carried on as the wire's default.

use std::path::{Path, PathBuf};

use yata_core::fact::Channel;
use yata_protocol::export::{self, ExportError};
use yata_protocol::probe::{
    self, PointerWidth, ProbeExport, ProtocolVersion, Reading, TargetProcess,
};

use super::session::{self, Failed, SessionError};

/// The largest recording read: the export file's bound.
pub const MAX_RECORDING_BYTES: u64 = export::MAX_EXPORT_BYTES as u64;

/// A process's pointer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    Bits32,
    Bits64,
}

/// The process a reading was read from, as the reader states it. Each fact the system did not
/// give the reader is `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadTarget {
    pub pid: u32,
    pub image_name: String,
    pub parent_pid: Option<u32>,
    pub session_id: Option<u32>,
    pub created_unix_ms: Option<u64>,
    pub width: Option<Width>,
}

/// Who produced a reading, as the reading itself states it.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    pub protocol_version: ProtocolVersion,
    pub probe_build_id: String,
    pub engine: String,
    pub channel: Channel,
    pub target: Option<ReadTarget>,
}

/// How the readings travelled, and what only that way of travelling carries.
#[derive(Debug, Clone, PartialEq)]
pub enum Carrier {
    /// A session read now, whether or not it was also recorded.
    Live { failures: Vec<Failed> },
    /// A frame stream read back. It states no capture time, and it holds the reader's failures.
    Recording { failures: Vec<Failed> },
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

/// A header field the reader must state and did not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unstated {
    Version,
    Channel,
    Engine,
    ProbeBuildId,
    PointerWidth,
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
    /// A `HandshakeAck` or an export header that leaves a field unstated.
    Unstated(Unstated),
}

/// Why a reading cannot be written as an export.
#[derive(Debug, Clone, PartialEq)]
pub enum ToExportError {
    /// The recording states no provenance to put in the file.
    NoProvenance,
}

/// The header fields both an acknowledgement and an export carry.
struct Header<'a> {
    version: Option<ProtocolVersion>,
    probe_build_id: &'a str,
    engine: &'a str,
    channel: i32,
    target: Option<&'a TargetProcess>,
}

fn nonempty(s: &str, u: Unstated) -> Result<String, InputError> {
    if s.is_empty() {
        Err(InputError::Unstated(u))
    } else {
        Ok(s.to_owned())
    }
}

impl Provenance {
    fn parse(h: Header<'_>) -> Result<Provenance, InputError> {
        let channel = match probe::Channel::try_from(h.channel) {
            Ok(probe::Channel::DesktopMemory) => Channel::DesktopMemory,
            Ok(probe::Channel::MumuAdb) => Channel::MumuAdb,
            Ok(probe::Channel::Unspecified) | Err(_) => {
                return Err(InputError::Unstated(Unstated::Channel));
            }
        };
        Ok(Provenance {
            protocol_version: h.version.ok_or(InputError::Unstated(Unstated::Version))?,
            probe_build_id: nonempty(h.probe_build_id, Unstated::ProbeBuildId)?,
            engine: nonempty(h.engine, Unstated::Engine)?,
            channel,
            target: h.target.map(ReadTarget::parse).transpose()?,
        })
    }

    fn channel_wire(&self) -> probe::Channel {
        match self.channel {
            Channel::DesktopMemory => probe::Channel::DesktopMemory,
            Channel::MumuAdb => probe::Channel::MumuAdb,
        }
    }
}

impl ReadTarget {
    fn parse(t: &TargetProcess) -> Result<ReadTarget, InputError> {
        let width = match t.pointer_width.map(PointerWidth::try_from) {
            None => None,
            Some(Ok(PointerWidth::PointerWidth32)) => Some(Width::Bits32),
            Some(Ok(PointerWidth::PointerWidth64)) => Some(Width::Bits64),
            Some(Ok(PointerWidth::Unspecified) | Err(_)) => {
                return Err(InputError::Unstated(Unstated::PointerWidth));
            }
        };
        Ok(ReadTarget {
            pid: t.pid,
            image_name: t.image_name.clone(),
            parent_pid: t.parent_pid,
            session_id: t.session_id,
            created_unix_ms: t.created_unix_ms,
            width,
        })
    }

    fn wire(&self) -> TargetProcess {
        TargetProcess {
            pid: self.pid,
            image_name: self.image_name.clone(),
            parent_pid: self.parent_pid,
            session_id: self.session_id,
            created_unix_ms: self.created_unix_ms,
            pointer_width: self.width.map(|w| {
                match w {
                    Width::Bits32 => PointerWidth::PointerWidth32,
                    Width::Bits64 => PointerWidth::PointerWidth64,
                }
                .into()
            }),
        }
    }
}

/// The provenance a live or recorded acknowledgement states.
pub fn provenance_of(a: &probe::HandshakeAck) -> Result<Provenance, InputError> {
    Provenance::parse(Header {
        version: a.version,
        probe_build_id: &a.probe_build_id,
        engine: &a.engine,
        channel: a.channel,
        target: a.target.as_ref(),
    })
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
        provenance: Some(Provenance::parse(Header {
            version: e.protocol_version,
            probe_build_id: &e.probe_build_id,
            engine: &e.engine,
            channel: e.channel,
            target: e.target.as_ref(),
        })?),
        carrier: Carrier::Export {
            captured_at: e.captured_at,
        },
        readings: e.readings,
    })
}

fn from_recording(bytes: &[u8]) -> Result<Loaded, InputError> {
    let replay = session::replay(bytes).map_err(InputError::Recording)?;
    Ok(Loaded {
        provenance: replay.ack().map(provenance_of).transpose()?,
        carrier: Carrier::Recording {
            failures: replay.failures().collect(),
        },
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
        channel: p.channel_wire().into(),
        readings: loaded.readings.clone(),
        captured_at: match &loaded.carrier {
            Carrier::Export { captured_at } => captured_at.clone(),
            Carrier::Live { .. } | Carrier::Recording { .. } => None,
        },
        target: p.target.as_ref().map(ReadTarget::wire),
    })
}
