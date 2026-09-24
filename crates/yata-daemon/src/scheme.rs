//! The scheme-code research commands: read a code from a file, and print a payload or a diff.
//!
//! This is the effectful edge of the scheme-code work: it reads files and formats text for a
//! developer. Every judgment is `yata-core::scheme`'s or [`crate::qr`]'s. The output is English
//! and austere, for comparing controlled samples during the reverse-engineering of the format.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use yata_core::scheme::inspect::{PayloadDiff, dump};
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
/// absolute bit offsets, LSB-first. `--` marks a byte one payload does not have.
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
            byte(c.before),
            byte(c.after),
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
             0x00000002  --      07     --   --\n"
        );
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
