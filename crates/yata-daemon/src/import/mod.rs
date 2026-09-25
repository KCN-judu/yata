//! External files at the import boundary (ADR-0031, `spec/import-format.md`): recognise the
//! format from the header, parse the source, and normalize it into the snapshot IR. Every format
//! detail ends in its module here; what leaves is a [`YataSnapshot`].
//!
//! A file is refused whole ([`ImportError`]) when it is not one snapshot: not JSON, an unknown,
//! ambiguous, or unsupported format, a file-level field missing, or a snapshot that breaks the
//! IR's own rules. A source record that cannot be normalized is left out and reported
//! ([`SourceDefect`]), and its section then states [`Completeness::Partial`]: it no longer holds
//! everything the file covered.
//!
//! [`read`] is pure over the bytes. [`read_path`] reads a file, and [`format_check`] writes the
//! developer report of `yata-daemon import check`.

pub mod codec;
mod mumu_snapshot_v1;
mod yata_snapshot;

use std::collections::BTreeMap;

use serde_json::{Map, Value};
use sha2::{Digest as _, Sha256};
use yata_core::fact::Digest;
use yata_core::import::admit::{AdmittedSnapshot, admit};
use yata_core::import::ir::{
    Completeness, FormatTag, IrError, SchemaVersion, Section, SectionKind, YataSnapshot, check,
    limits,
};

use codec::SnapshotDecodeError;

/// The JSON kind a field must have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Object,
    Array,
    String,
    Bool,
    Integer,
    NonNegativeInteger,
    Number,
    Null,
}

/// What is wrong with a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Problem {
    Missing,
    WrongKind { expected: Kind },
}

/// Why a file is refused. Nothing of it is imported.
#[derive(Debug, Clone, PartialEq)]
pub enum ImportError {
    /// Larger than the IR's file limit; nothing was read.
    TooLarge {
        bytes: u64,
    },
    /// Not JSON, or not a JSON object.
    MalformedSource {
        reason: String,
    },
    /// No format recognises the header. `stated` is the file's `format` value, if any.
    UnknownFormat {
        stated: Option<String>,
    },
    AmbiguousFormat {
        formats: Vec<Format>,
    },
    /// A yata-snapshot of a major version this build does not read.
    UnsupportedVersion {
        found: SchemaVersion,
    },
    /// A yata-snapshot whose schema value is not a snapshot.
    Decode(SnapshotDecodeError),
    /// A file-level field of a recognised format is missing or of the wrong kind.
    Shape {
        field: &'static str,
        problem: Problem,
    },
    /// A file-level value the format module does not support, such as a completeness other than
    /// the one it knows.
    UnsupportedSourceValue {
        field: &'static str,
        value: String,
    },
    /// The normalized snapshot breaks the IR's own rules.
    Ir(IrError),
}

/// Why one source record was left out during normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceReason {
    /// A field is missing or of the wrong kind. `field` is its path inside the record.
    Malformed { field: String, problem: Problem },
    /// A name the format does not have: an attribute, a currency.
    UnsupportedValue { field: String, value: String },
    /// A sub-attribute entry marked in a way the format does not define.
    UnknownSubAttribute { index: usize },
    /// More than one innate entry on one soul.
    SeveralInnate { count: usize },
    /// A text the IR refuses: empty, or too long.
    Text { field: String },
}

/// A source record left out, by section and position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDefect {
    pub section: SectionKind,
    pub index: usize,
    pub reason: SourceReason,
}

/// A file normalized: the snapshot, and the source records left out on the way.
#[derive(Debug, Clone, PartialEq)]
pub struct Normalized {
    pub snapshot: YataSnapshot,
    pub left_out: Vec<SourceDefect>,
}

/// The formats a file can be: Yata's own, or a community format (`spec/import-format.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    YataSnapshot,
    Community(FormatTag),
}

impl Format {
    fn all() -> Vec<Format> {
        std::iter::once(Format::YataSnapshot)
            .chain(FormatTag::ALL.into_iter().map(Format::Community))
            .collect()
    }
}

/// Whether a format recognises a header. Each rule reads the top-level object only.
fn recognises(format: Format, header: &Map<String, Value>) -> bool {
    match format {
        Format::YataSnapshot => yata_snapshot::recognises(header),
        Format::Community(FormatTag::MumuSnapshotV1) => mumu_snapshot_v1::recognises(header),
    }
}

/// The one format that recognises the header, or why there is not exactly one.
pub fn detect(header: &Map<String, Value>) -> Result<Format, ImportError> {
    let matching: Vec<Format> = Format::all()
        .into_iter()
        .filter(|&f| recognises(f, header))
        .collect();
    match matching.as_slice() {
        [one] => Ok(*one),
        [] => Err(ImportError::UnknownFormat {
            stated: header
                .get("format")
                .and_then(Value::as_str)
                .map(str::to_owned),
        }),
        _ => Err(ImportError::AmbiguousFormat { formats: matching }),
    }
}

/// SHA-256 of a file's bytes.
pub fn digest_of(bytes: &[u8]) -> Digest {
    Digest(Sha256::digest(bytes).into())
}

/// Normalize a file's bytes into the IR.
pub fn read(bytes: &[u8]) -> Result<Normalized, ImportError> {
    if bytes.len() as u64 > limits::FILE_BYTES {
        return Err(ImportError::TooLarge {
            bytes: bytes.len() as u64,
        });
    }
    let value: Value = serde_json::from_slice(bytes).map_err(|e| ImportError::MalformedSource {
        reason: e.to_string(),
    })?;
    let Value::Object(top) = value else {
        return Err(ImportError::MalformedSource {
            reason: "the top level is not an object".to_owned(),
        });
    };
    let original = digest_of(bytes);
    let normalized = match detect(&top)? {
        Format::YataSnapshot => yata_snapshot::normalize(top, original)?,
        Format::Community(FormatTag::MumuSnapshotV1) => {
            mumu_snapshot_v1::normalize(&top, original)?
        }
    };
    check(&normalized.snapshot).map_err(ImportError::Ir)?;
    Ok(normalized)
}

/// A section that lost records in normalization no longer holds everything the file covered.
pub(crate) fn partial_if<T>(section: Section<T>, lost: bool) -> Section<T> {
    match section {
        Section::Present { value, .. } if lost => Section::Present {
            completeness: Completeness::Partial,
            value,
        },
        s => s,
    }
}

/// A normalized snapshot as a yata-snapshot file's text (ADR-0031, rule 4).
pub fn export_json(n: &Normalized) -> Result<String, yata_protocol::snapshot_file::Unwritable> {
    yata_protocol::snapshot_file::to_json(&codec::to_proto(&n.snapshot))
}

/// Why a file on disk could not be imported.
#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    Import(ImportError),
}

/// Normalize the file at `path`, refusing one over the file limit before reading it.
pub fn read_path(path: &std::path::Path) -> Result<Normalized, ReadError> {
    let bytes = std::fs::metadata(path).map_err(ReadError::Io)?.len();
    if bytes > limits::FILE_BYTES {
        return Err(ReadError::Import(ImportError::TooLarge { bytes }));
    }
    let data = std::fs::read(path).map_err(ReadError::Io)?;
    read(&data).map_err(ReadError::Import)
}

fn section_line<T>(name: &str, s: &Section<T>, count: impl Fn(&T) -> usize) -> String {
    match s {
        Section::Absent => format!("{name}: absent\n"),
        Section::Present {
            completeness,
            value,
        } => format!("{name}: {} ({completeness:?})\n", count(value)),
    }
}

/// The report of `import check`: each section, the records left out and why, and how many souls
/// the game's rules find legal. No id is printed; a record is named by its position.
pub fn format_check(n: &Normalized) -> String {
    use yata_core::mechanics::assess;

    let s = &n.snapshot;
    let mut out = format!("format: {:?}\n", s.provenance.format);
    out += &format!(
        "captured at: {}\n",
        s.captured_at.as_deref().unwrap_or("(not stated)")
    );
    out += &section_line("souls", &s.souls, |v| v.souls.len());
    out += &section_line("shikigami", &s.shikigami, |v| v.instances.len());
    out += &section_line("presets", &s.presets, |v| v.presets.len());
    out += &section_line("assets", &s.assets, |v| {
        v.currencies.len() + v.realm_cards.len()
    });
    out += &section_line("guild", &s.guild, |_| 1);
    out += &format!("left out in normalization: {}\n", n.left_out.len());
    for d in &n.left_out {
        out += &format!("  {:?} {}: {:?}\n", d.section, d.index, d.reason);
    }
    let admitted: AdmittedSnapshot = match admit(s) {
        Ok(a) => a,
        Err(e) => return out + &format!("refused by the IR: {e:?}\n"),
    };
    if let Some((_, souls)) = admitted.souls.present() {
        out += &format!("souls admitted: {}\n", souls.values.len());
        for r in &souls.rejected {
            out += &format!("  soul {}: {:?}\n", r.index, r.reason);
        }
        let assessed: Vec<_> = souls.values.iter().map(|(_, s)| assess(s)).collect();
        let illegal = assessed.iter().filter(|a| !a.violations.is_empty()).count();
        let undecided = assessed.iter().filter(|a| !a.undecided.is_empty()).count();
        out += &format!("souls with violations: {illegal}, with undecided rules: {undecided}\n");
        let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
        for v in assessed.iter().flat_map(|a| &a.violations) {
            let name = format!("{v:?}");
            let kind = name
                .split([' ', '{', '('])
                .next()
                .unwrap_or(&name)
                .to_owned();
            *kinds.entry(kind).or_default() += 1;
        }
        for (kind, count) in kinds {
            out += &format!("  {kind}: {count}\n");
        }
    }
    if let Some((_, shikigami)) = admitted.shikigami.present() {
        out += &format!(
            "shikigami admitted: {}, left out: {}\n",
            shikigami.values.len(),
            shikigami.rejected.len()
        );
    }
    if let Some((_, presets)) = admitted.presets.present() {
        out += &format!(
            "presets admitted: {}, left out: {}\n",
            presets.values.len(),
            presets.rejected.len()
        );
    }
    out
}

#[cfg(test)]
mod tests;
