//! The scheme-code research commands: read a code from a file; print a payload, a diff, or the
//! plans; read a plan file for building a code.
//!
//! This is the effectful edge of the scheme-code work: it reads files and formats text for a
//! developer. Every judgment is `yata-core::scheme`'s or [`crate::qr`]'s. The output is English
//! and austere, for comparing controlled samples during the reverse-engineering of the format.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use yata_core::nonempty::NonEmptySet;
use yata_core::scheme::edit::{FilterBit, SoulBit, SoulChoice};
use yata_core::scheme::inspect::{PayloadDiff, dump};
use yata_core::scheme::layout::{Record, SchemeKind, SchemeLayout};
use yata_core::scheme::name::{NameTooLong, SchemeName};
use yata_core::scheme::transport::{MAX_PAYLOAD_LEN, MAX_SCHEME_TEXT_LEN, TransportError};
use yata_core::scheme::{RawSchemePayload, transport};

use crate::qr::{self, MAX_IMAGE_BYTES, QrError};

const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";

/// Why a code or payload file could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputError {
    Io {
        path: PathBuf,
        reason: String,
    },
    /// Larger than any code, image, or payload this reads.
    TooLarge {
        path: PathBuf,
        limit: usize,
    },
    /// A text file that is not UTF-8.
    NotText {
        path: PathBuf,
    },
    Qr(QrError),
    Transport(TransportError),
}

/// Read at most `limit` bytes; a longer file is an error, found without reading it whole.
fn read_bounded(path: &Path, limit: usize) -> Result<Vec<u8>, InputError> {
    let io = |e: std::io::Error| InputError::Io {
        path: path.to_owned(),
        reason: e.to_string(),
    };
    let file = fs::File::open(path).map_err(io)?;
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(io)?;
    if bytes.len() > limit {
        return Err(InputError::TooLarge {
            path: path.to_owned(),
            limit,
        });
    }
    Ok(bytes)
}

/// The Base64 text of a code file: the code in a PNG image, or the file's text. Whitespace
/// around a text file's content is a file convention, not part of the code, and is trimmed.
pub fn read_code_text(path: &Path) -> Result<String, InputError> {
    let bytes = read_bounded(path, MAX_IMAGE_BYTES)?;
    if bytes.starts_with(PNG_SIGNATURE) {
        return qr::decode_png(&bytes).map_err(InputError::Qr);
    }
    if bytes.len() > MAX_SCHEME_TEXT_LEN + 64 {
        return Err(InputError::TooLarge {
            path: path.to_owned(),
            limit: MAX_SCHEME_TEXT_LEN,
        });
    }
    let text = String::from_utf8(bytes).map_err(|_| InputError::NotText {
        path: path.to_owned(),
    })?;
    Ok(text.trim_ascii().to_owned())
}

/// The payload of a code file.
pub fn read_code(path: &Path) -> Result<RawSchemePayload, InputError> {
    transport::decode_text(&read_code_text(path)?).map_err(InputError::Transport)
}

/// A raw payload file, as written by `scheme decode`.
pub fn read_payload(path: &Path) -> Result<RawSchemePayload, InputError> {
    RawSchemePayload::new(read_bounded(path, MAX_PAYLOAD_LEN)?).map_err(InputError::Transport)
}

/// The dump of a payload: its length, then rows of offset, hex bytes, and the bytes as ASCII
/// where printable. The ASCII column is a reading aid, not a claim that a span is text.
pub fn format_dump(payload: &RawSchemePayload) -> String {
    let mut out = format!("payload: {} bytes\n", payload.len());
    for row in dump(payload) {
        let hex: Vec<String> = (0..16)
            .map(|i| {
                row.bytes
                    .get(i)
                    .map_or_else(|| "  ".to_owned(), |b| format!("{b:02x}"))
            })
            .collect();
        let ascii: String = row
            .bytes
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        out.push_str(&format!(
            "{:08x}  {}  {}  |{ascii}|\n",
            row.offset,
            hex[..8].join(" "),
            hex[8..].join(" ")
        ));
    }
    out
}

/// A diff: both lengths, then one line per changed offset with the XOR and the changed bits as
/// absolute bit offsets, LSB-first. `--` marks a byte one payload does not have; every bit of a
/// byte only one payload has is changed.
pub fn format_diff(diff: &PayloadDiff) -> String {
    let mut out = format!(
        "before: {} bytes\nafter: {} bytes\nchanged: {} bytes\n",
        diff.before_len,
        diff.after_len,
        diff.changes.len()
    );
    if diff.is_identical() {
        out.push_str("identical\n");
        return out;
    }
    out.push_str("offset      before  after  xor  bits\n");
    let byte = |b: Option<u8>| b.map_or_else(|| "--".to_owned(), |b| format!("{b:02x}"));
    for c in &diff.changes {
        let bits: Vec<String> = c
            .changed_bit_offsets()
            .iter()
            .map(usize::to_string)
            .collect();
        out.push_str(&format!(
            "0x{:08x}  {}      {}     {}   {}\n",
            c.offset,
            byte(c.before()),
            byte(c.after()),
            byte(c.xor()),
            if bits.is_empty() {
                "--".to_owned()
            } else {
                bits.join(" ")
            }
        ));
    }
    out
}

/// The plans of a layout, one line each: index, name, souls (`all` or bit numbers), the filter in
/// hex, and its set bits, with `*` on each bit that is not solved. The account is never printed.
pub fn format_plans(layout: &SchemeLayout) -> String {
    let kind = match layout.kind() {
        SchemeKind::Discard => "discard",
        SchemeKind::Strengthening => "strengthening",
    };
    let mut out = format!(
        "kind: {kind}\naccount: present (not shown)\nrecords: {}\n",
        layout.records().len()
    );
    for (i, r) in layout.records().into_iter().enumerate() {
        let name = r.name().map_or_else(
            || format!("<not UTF-8: {} bytes>", r.name_bytes().len()),
            str::to_owned,
        );
        let souls = match r.souls() {
            SoulChoice::All => "all".to_owned(),
            SoulChoice::Souls(bits) => join(&bits.into_iter().collect::<Vec<_>>(), |b| {
                if SoulBit::new(*b).is_some() {
                    b.to_string()
                } else {
                    format!("{b}*")
                }
            }),
        };
        let filter_hex: String = r.filter().iter().map(|b| format!("{b:02x}")).collect();
        let filter_bits = join(&r.filter_bits(), |b| {
            if FilterBit::new(*b).is_some() {
                b.to_string()
            } else {
                format!("{b}*")
            }
        });
        out.push_str(&format!(
            "{i:>3}  {name}\n     souls: {souls}\n     filter: {filter_hex}  bits: {filter_bits}\n"
        ));
    }
    out
}

fn join(bits: &[u16], show: impl Fn(&u16) -> String) -> String {
    if bits.is_empty() {
        return "none".to_owned();
    }
    bits.iter().map(show).collect::<Vec<_>>().join(",")
}

/// Why a line of a plan file was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanLineProblem {
    /// Not three fields separated by `|`.
    Shape,
    EmptyName,
    /// A name the game would refuse on import.
    NameTooLong(NameTooLong),
    /// A souls field with no number; all souls is written `all`.
    EmptySouls,
    NotANumber {
        text: String,
    },
    /// A soul bit beyond the 70 mapped soul sets.
    UnknownSoulBit {
        bit: u16,
    },
    /// A filter bit outside the solved groups; open bits are never written.
    UnsolvedFilterBit {
        bit: u16,
    },
}

/// A refused line of a plan file, by its 1-based line number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanFileError {
    pub line: usize,
    pub problem: PlanLineProblem,
}

/// The records of a plan file: one plan per line, `name | souls | filter bits`, where souls are
/// `all` or bit numbers and filter bits are solved bit numbers, both comma-separated. Blank lines
/// and lines starting with `#` are skipped. Every line is checked; the first refusal is returned.
pub fn parse_plan_file(text: &str) -> Result<Vec<Record>, PlanFileError> {
    let mut records = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let err = |problem| PlanFileError {
            line: i + 1,
            problem,
        };
        records.push(parse_plan_line(line).map_err(err)?);
    }
    Ok(records)
}

fn parse_plan_line(line: &str) -> Result<Record, PlanLineProblem> {
    let fields: Vec<&str> = line.split('|').map(str::trim).collect();
    let [name, souls, filter] = fields.as_slice() else {
        return Err(PlanLineProblem::Shape);
    };
    if name.is_empty() {
        return Err(PlanLineProblem::EmptyName);
    }
    let name = SchemeName::new(*name).map_err(PlanLineProblem::NameTooLong)?;
    let numbers = |text: &str| -> Result<Vec<u16>, PlanLineProblem> {
        text.split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(|t| {
                t.parse::<u16>()
                    .map_err(|_| PlanLineProblem::NotANumber { text: t.to_owned() })
            })
            .collect()
    };
    let souls = if souls.eq_ignore_ascii_case("all") {
        SoulChoice::All
    } else {
        let bits = numbers(souls)?
            .into_iter()
            .map(|b| SoulBit::new(b).ok_or(PlanLineProblem::UnknownSoulBit { bit: b }))
            .collect::<Result<Vec<_>, _>>()?;
        SoulChoice::Souls(NonEmptySet::collect(bits).ok_or(PlanLineProblem::EmptySouls)?)
    };
    let filter = numbers(filter)?
        .into_iter()
        .map(|b| FilterBit::new(b).ok_or(PlanLineProblem::UnsolvedFilterBit { bit: b }))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Record::from_bits(&name, &souls, &filter))
}

#[cfg(test)]
mod tests {
    use yata_core::scheme::inspect::diff;

    use super::*;

    fn payload(bytes: &[u8]) -> RawSchemePayload {
        RawSchemePayload::new(bytes.to_vec()).expect("valid payload")
    }

    #[test]
    fn a_dump_is_fixed_width_rows_with_an_ascii_column() {
        let text = format_dump(&payload(b"ES\x00\x01abcdefghijklmnopq"));
        assert_eq!(
            text,
            "payload: 21 bytes\n\
             00000000  45 53 00 01 61 62 63 64  65 66 67 68 69 6a 6b 6c  |ES..abcdefghijkl|\n\
             00000010  6d 6e 6f 70 71                                    |mnopq|\n"
        );
    }

    #[test]
    fn a_diff_lists_each_changed_byte_with_its_bits() {
        let d = diff(&payload(&[0x00, 0x05]), &payload(&[0x00, 0x15, 0x07]));
        assert_eq!(
            format_diff(&d),
            "before: 2 bytes\nafter: 3 bytes\nchanged: 2 bytes\n\
             offset      before  after  xor  bits\n\
             0x00000001  05      15     10   12\n\
             0x00000002  --      07     --   16 17 18 19 20 21 22 23\n"
        );
    }

    #[test]
    fn a_plan_file_reads_names_souls_and_solved_filter_bits() {
        let records = parse_plan_file(
            "# name | souls | filter bits\n\n测试位39 | 39 | 0,1,2,3,4,5,11,49\n全部 | all | 1, 11\n",
        )
        .expect("valid");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].soul_mask(), &[0, 0, 0, 0, 0x80]);
        assert_eq!(records[0].filter(), &[0x3f, 0x08, 0, 0, 0, 0, 0x02]);
        assert_eq!(records[1].souls(), SoulChoice::All);
    }

    #[test]
    fn a_plan_file_refuses_open_bits_and_names_the_line() {
        assert_eq!(
            parse_plan_file("ok | all | 1\nbad | all | 1,61\n"),
            Err(PlanFileError {
                line: 2,
                problem: PlanLineProblem::UnsolvedFilterBit { bit: 61 }
            })
        );
        assert_eq!(
            parse_plan_file("x | 70 | 1").map_err(|e| e.problem),
            Err(PlanLineProblem::UnknownSoulBit { bit: 70 })
        );
        assert_eq!(
            parse_plan_file("x | all").map_err(|e| e.problem),
            Err(PlanLineProblem::Shape)
        );
        assert_eq!(
            parse_plan_file(" | all | 1").map_err(|e| e.problem),
            Err(PlanLineProblem::EmptyName)
        );
        assert_eq!(
            parse_plan_file("x |  | 1").map_err(|e| e.problem),
            Err(PlanLineProblem::EmptySouls)
        );
        assert!(matches!(
            parse_plan_file("攻击固定值-排除-蝠翼 | all | 1").map_err(|e| e.problem),
            Err(PlanLineProblem::NameTooLong(_))
        ));
    }

    #[test]
    fn plans_list_marks_open_bits_and_never_prints_the_account() {
        use yata_core::scheme::layout::AccountSegment;
        let mut filter = vec![0u8; 8];
        filter[6] = 0x02; // level bit 49
        filter[7] = 0x20; // open bit 61
        let layout = SchemeLayout::Strengthening {
            account: AccountSegment::from_bytes([0xab; 14]),
            plans: vec![Record::new("p", vec![0x01], filter).expect("valid")],
        };
        let text = format_plans(&layout);
        assert_eq!(
            text,
            "kind: strengthening\n\
             account: present (not shown)\n\
             records: 1\n  \
             0  p\n     \
             souls: 0\n     \
             filter: 0000000000000220  bits: 49,61*\n"
        );
        assert!(!text.to_lowercase().contains("ab ab"));
    }

    #[test]
    fn an_identical_diff_says_so() {
        let p = payload(b"ES");
        assert_eq!(
            format_diff(&diff(&p, &p)),
            "before: 2 bytes\nafter: 2 bytes\nchanged: 0 bytes\nidentical\n"
        );
    }
}
