//! Snapshot files at the import boundary (`spec/import-format.md`, PRP-0008): recognise the
//! format from the header, parse it with that format's module, and turn each soul into a domain
//! soul.
//!
//! A file is refused whole ([`ImportError`]) when it cannot be one inventory: not JSON, a format
//! Yata does not read or cannot tell apart, not complete, or two records claiming one soul. A
//! record that cannot be a soul is a [`RecordDefect`]: it is reported and left out, and the rest
//! of the file is kept. Nothing is guessed or repaired.
//!
//! [`read`] is pure over the bytes. [`read_path`] reads a file, and [`format_check`] writes the
//! developer report of `yata-daemon import check`.

mod mumu_snapshot_v1;

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};
use yata_core::fact::GameSoulId;
use yata_core::import::snapshot::{FormatTag, SnapshotSoul, SoulDefect, soul_of};
use yata_core::soul::Soul;

/// The largest file the importer reads: far above any inventory's size.
pub const MAX_IMPORT_BYTES: u64 = 64 * 1024 * 1024;

/// The JSON kind a field must have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Object,
    Array,
    String,
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
    NotJson {
        reason: String,
    },
    NotAnObject,
    /// No format recognises the header. `stated` is the file's `format` value, if it has one.
    Unrecognised {
        stated: Option<String>,
    },
    /// More than one format recognises the header.
    Ambiguous {
        formats: Vec<FormatTag>,
    },
    /// The file does not say it is a complete inventory.
    NotComplete {
        stated: String,
    },
    /// A top-level field is missing or of the wrong kind.
    Shape {
        field: &'static str,
        problem: Problem,
    },
    /// Two records claim one soul. One soul has one state at one time, and neither is preferred.
    DuplicateSoul {
        id: String,
    },
}

/// Why one record is not a soul.
#[derive(Debug, Clone, PartialEq)]
pub enum RecordReason {
    /// A field of the record is missing or of the wrong kind. `field` is its path in the record.
    Shape { field: String, problem: Problem },
    /// An attribute name the format does not have.
    UnknownAttribute { field: String, name: String },
    /// A sub-attribute entry marked in a way the format does not define.
    UnknownSubAttribute { index: usize },
    /// Parsed, but not a domain soul.
    Soul(SoulDefect),
}

/// A record left out, by its position in the file, with its id when it has one.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordDefect {
    pub index: usize,
    pub id: Option<String>,
    pub reason: RecordReason,
}

/// A file imported: its format, when it says it was captured, its souls, and the records that
/// are not souls.
#[derive(Debug, Clone, PartialEq)]
pub struct Imported {
    pub format: FormatTag,
    /// As the file states it; informational.
    pub captured_at: String,
    pub souls: Vec<(GameSoulId, Soul)>,
    pub defects: Vec<RecordDefect>,
}

/// One record as a format's parser returns it: its id when it has one, and the soul or the reason
/// it is none.
pub(crate) struct Record {
    pub id: Option<String>,
    pub soul: Result<SnapshotSoul, RecordReason>,
}

/// What a format's parser returns for a whole file.
pub(crate) struct Parsed {
    pub captured_at: String,
    pub records: Vec<Record>,
}

/// Whether a format recognises a header. Each format's rule reads the top-level object only.
fn recognises(format: FormatTag, header: &Map<String, Value>) -> bool {
    match format {
        FormatTag::MumuSnapshotV1 => mumu_snapshot_v1::recognises(header),
    }
}

/// The one format that recognises the header, or why there is not exactly one.
pub fn detect(header: &Map<String, Value>) -> Result<FormatTag, ImportError> {
    let matching: Vec<FormatTag> = FormatTag::ALL
        .into_iter()
        .filter(|&f| recognises(f, header))
        .collect();
    match matching.as_slice() {
        [one] => Ok(*one),
        [] => Err(ImportError::Unrecognised {
            stated: header
                .get("format")
                .and_then(Value::as_str)
                .map(str::to_owned),
        }),
        _ => Err(ImportError::Ambiguous { formats: matching }),
    }
}

/// Import a file's bytes.
pub fn read(bytes: &[u8]) -> Result<Imported, ImportError> {
    let value: Value = serde_json::from_slice(bytes).map_err(|e| ImportError::NotJson {
        reason: e.to_string(),
    })?;
    let Value::Object(top) = value else {
        return Err(ImportError::NotAnObject);
    };
    let format = detect(&top)?;
    let parsed = match format {
        FormatTag::MumuSnapshotV1 => mumu_snapshot_v1::parse(&top)?,
    };
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for r in &parsed.records {
        if let Some(id) = r.id.as_deref()
            && !seen.insert(id)
        {
            return Err(ImportError::DuplicateSoul { id: id.to_owned() });
        }
    }
    let mut souls = Vec::new();
    let mut defects = Vec::new();
    for (index, r) in parsed.records.into_iter().enumerate() {
        match r.soul.and_then(|s| soul_of(&s).map_err(RecordReason::Soul)) {
            Ok(soul) => souls.push(soul),
            Err(reason) => defects.push(RecordDefect {
                index,
                id: r.id,
                reason,
            }),
        }
    }
    Ok(Imported {
        format,
        captured_at: parsed.captured_at,
        souls,
        defects,
    })
}

/// Why a file on disk could not be imported.
#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    /// Larger than [`MAX_IMPORT_BYTES`]; nothing was read.
    TooLarge {
        bytes: u64,
    },
    Import(ImportError),
}

/// Import the file at `path`, refusing one larger than [`MAX_IMPORT_BYTES`] before reading it.
pub fn read_path(path: &std::path::Path) -> Result<Imported, ReadError> {
    let bytes = std::fs::metadata(path).map_err(ReadError::Io)?.len();
    if bytes > MAX_IMPORT_BYTES {
        return Err(ReadError::TooLarge { bytes });
    }
    let data = std::fs::read(path).map_err(ReadError::Io)?;
    read(&data).map_err(ReadError::Import)
}

/// The report of `import check`: the format, the counts, each defect, and how many souls the game's
/// rules find legal (`mechanics::assess`). Soul ids are not printed; a defect names its record by
/// position.
pub fn format_check(imported: &Imported) -> String {
    use std::fmt::Write as _;
    use yata_core::mechanics::assess;

    let mut out = String::new();
    let _ = writeln!(out, "format: {}", imported.format.name());
    let _ = writeln!(out, "captured at: {}", imported.captured_at);
    let _ = writeln!(out, "souls: {}", imported.souls.len());
    let _ = writeln!(out, "records left out: {}", imported.defects.len());
    for d in &imported.defects {
        let _ = writeln!(out, "  record {}: {:?}", d.index, d.reason);
    }
    let assessed: Vec<_> = imported.souls.iter().map(|(_, s)| assess(s)).collect();
    let with =
        |f: fn(&yata_core::mechanics::Assessment) -> bool| assessed.iter().filter(|a| f(a)).count();
    let _ = writeln!(
        out,
        "with violations: {}",
        with(|a| !a.violations.is_empty())
    );
    let _ = writeln!(
        out,
        "with undecided rules: {}",
        with(|a| !a.undecided.is_empty())
    );
    let _ = writeln!(out, "with warnings: {}", with(|a| !a.warnings.is_empty()));
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
    for (kind, n) in kinds {
        let _ = writeln!(out, "  {kind}: {n}");
    }
    out
}

#[cfg(test)]
mod tests;
