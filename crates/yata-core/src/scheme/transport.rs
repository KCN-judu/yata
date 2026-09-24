//! The transport layers of a scheme code: Base64 text ⇄ zlib stream ⇄ payload.
//!
//! What the code relies on, and why (`scheme-code.md`, § Transport):
//!
//! - **Base64 is the standard alphabet with `+` and `/`**, as every observed code uses. Decoding
//!   is strict: no whitespace, canonical padding, no stray trailing bits. Whether the game
//!   emits or accepts other forms is not known, so nothing else is accepted.
//! - **zlib framing, whole and alone.** The stream must end with its checksum, and no byte may
//!   follow it. Observed codes are exactly one zlib stream.
//! - **Every size is bounded before it is allocated**: the text, the compressed stream, and the
//!   payload, whose expansion stops at [`MAX_PAYLOAD_LEN`] however the stream is built.
//!
//! The guarantee is payload identity: decoding what [`encode_text`] produced returns the same
//! payload, byte for byte. The compressed stream and the text are not promised to match a code
//! the game produced, because the game's compressor and its settings are not known.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use miniz_oxide::deflate::compress_to_vec_zlib;
use miniz_oxide::inflate::TINFLStatus;
use miniz_oxide::inflate::core::{DecompressorOxide, decompress, inflate_flags};

use super::RawSchemePayload;

/// The longest scheme text accepted. A QR code carries at most 2 953 bytes in byte mode, so no
/// code the game shows as a QR code is longer; the rest is room for pasted text.
pub const MAX_SCHEME_TEXT_LEN: usize = 4096;

/// The longest compressed stream accepted: what [`MAX_SCHEME_TEXT_LEN`] characters of Base64
/// can carry.
pub const MAX_COMPRESSED_LEN: usize = MAX_SCHEME_TEXT_LEN / 4 * 3;

/// The largest payload accepted. The largest observed payload is 1 102 bytes (a set of 30
/// plans); the limit leaves room for far larger sets while bounding what a hostile stream can
/// make the decoder allocate.
pub const MAX_PAYLOAD_LEN: usize = 64 * 1024;

/// The compression level of [`encode_text`]: the maximum, which gives the smallest text and so
/// the smallest QR code. It is fixed so the same payload always yields the same text.
const COMPRESSION_LEVEL: u8 = 9;

/// Why a scheme text could not be decoded or a payload encoded. Each stage has its own variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    /// The text is longer than [`MAX_SCHEME_TEXT_LEN`].
    TextTooLong { length: usize, limit: usize },
    /// The text is not strict standard Base64.
    Base64(Base64Problem),
    /// The compressed stream is longer than [`MAX_COMPRESSED_LEN`].
    CompressedTooLong { length: usize, limit: usize },
    /// The bytes are not one valid zlib stream.
    Zlib(ZlibProblem),
    /// Bytes follow the end of the zlib stream.
    TrailingBytes { count: usize },
    /// The payload expands beyond [`MAX_PAYLOAD_LEN`]. Decompression stopped at the limit.
    PayloadTooLarge { limit: usize },
    /// The payload is empty. No scheme is.
    EmptyPayload,
}

/// Where the Base64 layer failed. Offsets count characters from the start of the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base64Problem {
    /// A character outside the standard alphabet, whitespace included.
    InvalidCharacter { offset: usize, byte: u8 },
    /// A length that no Base64 text has.
    InvalidLength { length: usize },
    /// A last character whose unused bits are not zero, so the text is not canonical.
    NonCanonicalEnding { offset: usize, byte: u8 },
    /// Missing, extra, or misplaced `=` padding.
    InvalidPadding,
}

/// Where the zlib layer failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZlibProblem {
    /// The stream ends before its final block and checksum.
    Truncated,
    /// The stream's checksum does not match its content.
    ChecksumMismatch,
    /// The header or a block is malformed.
    Malformed,
}

/// Scheme text as this project encodes it: strict standard Base64 of one zlib stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncodedSchemeText(String);

impl EncodedSchemeText {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

/// Base64 text → payload, through every transport layer.
pub fn decode_text(text: &str) -> Result<RawSchemePayload, TransportError> {
    if text.len() > MAX_SCHEME_TEXT_LEN {
        return Err(TransportError::TextTooLong {
            length: text.len(),
            limit: MAX_SCHEME_TEXT_LEN,
        });
    }
    let compressed = STANDARD
        .decode(text)
        .map_err(|e| TransportError::Base64(base64_problem(e)))?;
    decompress_payload(&compressed)
}

/// Payload → Base64 text, through every transport layer.
pub fn encode_text(payload: &RawSchemePayload) -> Result<EncodedSchemeText, TransportError> {
    let compressed = compress_to_vec_zlib(payload.as_bytes(), COMPRESSION_LEVEL);
    if compressed.len() > MAX_COMPRESSED_LEN {
        return Err(TransportError::CompressedTooLong {
            length: compressed.len(),
            limit: MAX_COMPRESSED_LEN,
        });
    }
    let text = STANDARD.encode(&compressed);
    if text.len() > MAX_SCHEME_TEXT_LEN {
        return Err(TransportError::TextTooLong {
            length: text.len(),
            limit: MAX_SCHEME_TEXT_LEN,
        });
    }
    Ok(EncodedSchemeText(text))
}

fn base64_problem(e: base64::DecodeError) -> Base64Problem {
    match e {
        base64::DecodeError::InvalidByte(offset, byte) => {
            Base64Problem::InvalidCharacter { offset, byte }
        }
        base64::DecodeError::InvalidLength(length) => Base64Problem::InvalidLength { length },
        base64::DecodeError::InvalidLastSymbol { offset, symbol, .. } => {
            Base64Problem::NonCanonicalEnding {
                offset,
                byte: symbol,
            }
        }
        base64::DecodeError::InvalidPadding => Base64Problem::InvalidPadding,
    }
}

/// One whole zlib stream → payload, never writing more than [`MAX_PAYLOAD_LEN`] bytes.
fn decompress_payload(compressed: &[u8]) -> Result<RawSchemePayload, TransportError> {
    if compressed.len() > MAX_COMPRESSED_LEN {
        return Err(TransportError::CompressedTooLong {
            length: compressed.len(),
            limit: MAX_COMPRESSED_LEN,
        });
    }
    // One byte beyond the limit, so a stream that fills it is known to be too large rather than
    // exactly at the limit.
    let mut out = vec![0u8; MAX_PAYLOAD_LEN + 1];
    let flags = inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER
        | inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF;
    let mut state = DecompressorOxide::new();
    let (status, consumed, written) = decompress(&mut state, compressed, &mut out, 0, flags);
    match status {
        TINFLStatus::Done => {}
        TINFLStatus::HasMoreOutput => {
            return Err(TransportError::PayloadTooLarge {
                limit: MAX_PAYLOAD_LEN,
            });
        }
        TINFLStatus::NeedsMoreInput | TINFLStatus::FailedCannotMakeProgress => {
            return Err(TransportError::Zlib(ZlibProblem::Truncated));
        }
        TINFLStatus::Adler32Mismatch => {
            return Err(TransportError::Zlib(ZlibProblem::ChecksumMismatch));
        }
        // Any other status, including ones a later miniz_oxide adds, is a malformed stream.
        _ => return Err(TransportError::Zlib(ZlibProblem::Malformed)),
    }
    if written > MAX_PAYLOAD_LEN {
        return Err(TransportError::PayloadTooLarge {
            limit: MAX_PAYLOAD_LEN,
        });
    }
    if consumed < compressed.len() {
        return Err(TransportError::TrailingBytes {
            count: compressed.len() - consumed,
        });
    }
    out.truncate(written);
    RawSchemePayload::new(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Made by Python's `zlib.compress(payload, 9)` and `base64.b64encode`: an independent
    /// implementation, so decoding it tests this codec against something it did not produce.
    /// The payload is synthetic and says nothing about the game's format.
    const SYNTHETIC_TEXT: &str = "eNqrTCxJVCiuzCvJSC3JTFYoKUrMKy7ILypRSMusKCktStVRyMsvUQAqSc5IzU3lYmBkYmZhZWPn4OTi5uHl4xcQFBIWERUTl5CUkpaRlZNXUFRSVlFVU9fQ1NLW0dXTNzA0MjYxNTO3sLSytrG1s3dwdHJ2cXVz9/D08vbx9fMPCAwKDgkNC4+IjIqOiY2LT0hMSk5JTUvPyMzKzsnNyy8oLCouKS0rr6isqq6pratvaGxqbmlta+/o7Oru6e3rnzBx0uQpU6dNnzFz1uw5c+fNX7Bw0eIlS5ctX7Fy1eo1a9et37Bx0+YtW7dt37Fz1+49e/ftP3Dw0OEjR48dP3Hy1OkzZ8+dv3Dx0uUrV69dv3Hz1u07d+/df/Dw0eMnT589f/Hy1es3b9+9//Dx0+cvX799//Hz1+8/f//9Z6AQAABLr5Ec";

    fn synthetic_payload() -> Vec<u8> {
        let mut p = b"yata synthetic transport fixture, not a scheme\n".to_vec();
        p.extend(0u8..=255);
        p.extend([0u8; 64]);
        p
    }

    fn payload(bytes: &[u8]) -> RawSchemePayload {
        RawSchemePayload::new(bytes.to_vec()).expect("valid payload")
    }

    #[test]
    fn a_text_from_another_implementation_decodes() {
        assert_eq!(
            decode_text(SYNTHETIC_TEXT).map(RawSchemePayload::into_bytes),
            Ok(synthetic_payload())
        );
    }

    #[test]
    fn padding_and_a_stored_block_decode() {
        // Python: zlib.compress(b"yata pad", 9) and at level 0, which stores the block.
        assert_eq!(
            decode_text("eNqrTCxJVChITAEADgoDBQ=="),
            Ok(payload(b"yata pad"))
        );
        assert_eq!(
            decode_text("eAEBCAD3/3lhdGEgcGFkDgoDBQ=="),
            Ok(payload(b"yata pad"))
        );
    }

    #[test]
    fn malformed_base64_is_a_base64_error() {
        assert_eq!(
            decode_text("eNqr TCxJ"),
            Err(TransportError::Base64(Base64Problem::InvalidCharacter {
                offset: 4,
                byte: b' '
            }))
        );
        assert_eq!(
            decode_text("eNqrTCxJVChITAEADgoDBQ=="[..23].trim_end_matches('=')),
            Err(TransportError::Base64(Base64Problem::InvalidPadding))
        );
        assert!(matches!(
            decode_text("eNqrT"),
            Err(TransportError::Base64(Base64Problem::InvalidLength { .. }))
        ));
        assert!(matches!(
            decode_text("eNqrTCxJVChITAEADgoDBR=="),
            Err(TransportError::Base64(
                Base64Problem::NonCanonicalEnding { .. }
            ))
        ));
        // The URL-safe alphabet is not the game's.
        assert!(matches!(
            decode_text("eAEBCAD3_3lhdGEgcGFkDgoDBQ=="),
            Err(TransportError::Base64(Base64Problem::InvalidCharacter {
                byte: b'_',
                ..
            }))
        ));
    }

    #[test]
    fn surrounding_whitespace_is_not_part_of_a_code() {
        assert!(matches!(
            decode_text("eNqrTCxJVChITAEADgoDBQ==\n"),
            Err(TransportError::Base64(_))
        ));
    }

    #[test]
    fn a_truncated_stream_is_a_zlib_error() {
        let text = encode_text(&payload(&synthetic_payload())).expect("encodable");
        let compressed = STANDARD.decode(text.as_str()).expect("base64");
        let cut = STANDARD.encode(&compressed[..compressed.len() - 6]);
        assert_eq!(
            decode_text(&cut),
            Err(TransportError::Zlib(ZlibProblem::Truncated))
        );
    }

    #[test]
    fn a_corrupted_checksum_is_a_zlib_error() {
        let mut compressed = STANDARD.decode("eNqrTCxJVChITAEADgoDBQ==").expect("base64");
        let last = compressed.len() - 1;
        compressed[last] ^= 0x01;
        assert_eq!(
            decode_text(&STANDARD.encode(&compressed)),
            Err(TransportError::Zlib(ZlibProblem::ChecksumMismatch))
        );
    }

    #[test]
    fn bytes_that_are_not_zlib_are_a_zlib_error() {
        assert_eq!(
            decode_text(&STANDARD.encode(b"not zlib at all")),
            Err(TransportError::Zlib(ZlibProblem::Malformed))
        );
    }

    #[test]
    fn bytes_after_the_stream_are_refused() {
        let mut compressed = STANDARD.decode("eNqrTCxJVChITAEADgoDBQ==").expect("base64");
        compressed.extend([0, 0]);
        assert_eq!(
            decode_text(&STANDARD.encode(&compressed)),
            Err(TransportError::TrailingBytes { count: 2 })
        );
    }

    #[test]
    fn a_decompression_bomb_stops_at_the_limit() {
        // One MiB of zeros compresses to about a kilobyte, well inside the text limit.
        let bomb = compress_to_vec_zlib(&vec![0u8; 1 << 20], 9);
        let text = STANDARD.encode(&bomb);
        assert!(text.len() <= MAX_SCHEME_TEXT_LEN);
        assert_eq!(
            decode_text(&text),
            Err(TransportError::PayloadTooLarge {
                limit: MAX_PAYLOAD_LEN
            })
        );
    }

    #[test]
    fn the_payload_limit_is_inclusive() {
        let at = compress_to_vec_zlib(&vec![7u8; MAX_PAYLOAD_LEN], 9);
        let over = compress_to_vec_zlib(&vec![7u8; MAX_PAYLOAD_LEN + 1], 9);
        assert_eq!(
            decode_text(&STANDARD.encode(&at)).map(|p| p.len()),
            Ok(MAX_PAYLOAD_LEN)
        );
        assert_eq!(
            decode_text(&STANDARD.encode(&over)),
            Err(TransportError::PayloadTooLarge {
                limit: MAX_PAYLOAD_LEN
            })
        );
    }

    #[test]
    fn an_empty_stream_is_not_a_scheme() {
        let empty = STANDARD.encode(compress_to_vec_zlib(b"", 9));
        assert_eq!(decode_text(&empty), Err(TransportError::EmptyPayload));
    }

    #[test]
    fn an_overlong_text_is_refused_before_decoding() {
        let text = "A".repeat(MAX_SCHEME_TEXT_LEN + 4);
        assert_eq!(
            decode_text(&text),
            Err(TransportError::TextTooLong {
                length: MAX_SCHEME_TEXT_LEN + 4,
                limit: MAX_SCHEME_TEXT_LEN
            })
        );
    }

    #[test]
    fn an_incompressible_payload_too_big_for_a_code_is_refused() {
        // A xorshift sequence: incompressible, and deterministic.
        let mut x: u32 = 0x9e37_79b9;
        let noise: Vec<u8> = (0..4000)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 17;
                x ^= x << 5;
                x.to_le_bytes()[0]
            })
            .collect();
        assert!(matches!(
            encode_text(&payload(&noise)),
            Err(TransportError::CompressedTooLong { .. })
        ));
    }

    #[test]
    fn encoding_is_deterministic_and_decodes_to_the_same_payload() {
        let p = payload(&synthetic_payload());
        let a = encode_text(&p).expect("encodable");
        assert_eq!(encode_text(&p), Ok(a.clone()));
        assert_eq!(decode_text(a.as_str()), Ok(p));
    }

    mod properties {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn every_encodable_payload_round_trips(bytes in proptest::collection::vec(any::<u8>(), 1..2048)) {
                let p = payload(&bytes);
                let text = encode_text(&p).expect("2 KiB always fits");
                prop_assert_eq!(decode_text(text.as_str()), Ok(p));
            }
        }
    }
}
