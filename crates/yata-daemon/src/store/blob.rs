//! The blob codec: a reading's bytes at rest (`fact-format.md`, § Blobs and content addressing).
//!
//! A blob is named by the SHA-256 of its uncompressed bytes and stored compressed, behind a
//! one-byte header naming the codec, so the codec can change without re-keying anything.
//! Opening a blob checks its digest: bytes that do not hash to their name are never returned.

use std::io::Read;

use sha2::{Digest as _, Sha256};
use yata_core::fact::Digest;
use yata_protocol::frame::MAX_FRAME_LEN;

/// The largest reading a blob holds: a reading arrives in one frame.
pub const MAX_BLOB_LEN: usize = MAX_FRAME_LEN as usize;

/// The codec byte of a zstd-compressed blob, the initial codec.
const CODEC_ZSTD: u8 = 1;
const ZSTD_LEVEL: i32 = 3;

/// Why a blob could not be sealed or opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobError {
    TooLarge,
    /// No header byte.
    Empty,
    UnknownCodec(u8),
    /// The compressed bytes do not decompress.
    Corrupt,
    /// The bytes do not hash to the digest they are stored under.
    DigestMismatch,
}

/// The SHA-256 digest of a reading's bytes.
pub fn digest_of(bytes: &[u8]) -> Digest {
    Sha256::digest(bytes).into()
}

/// A reading's digest and the bytes to store under it.
pub fn seal(bytes: &[u8]) -> Result<(Digest, Vec<u8>), BlobError> {
    if bytes.len() > MAX_BLOB_LEN {
        return Err(BlobError::TooLarge);
    }
    let compressed = zstd::bulk::compress(bytes, ZSTD_LEVEL).map_err(|_| BlobError::Corrupt)?;
    let mut stored = Vec::with_capacity(compressed.len() + 1);
    stored.push(CODEC_ZSTD);
    stored.extend(compressed);
    Ok((digest_of(bytes), stored))
}

/// The reading's bytes, checked against `digest`.
pub fn open(digest: &Digest, stored: &[u8]) -> Result<Vec<u8>, BlobError> {
    let (&codec, body) = stored.split_first().ok_or(BlobError::Empty)?;
    if codec != CODEC_ZSTD {
        return Err(BlobError::UnknownCodec(codec));
    }
    let decoder = zstd::stream::read::Decoder::new(body).map_err(|_| BlobError::Corrupt)?;
    let mut bytes = Vec::new();
    decoder
        .take(MAX_BLOB_LEN as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| BlobError::Corrupt)?;
    if bytes.len() > MAX_BLOB_LEN {
        return Err(BlobError::TooLarge);
    }
    if digest_of(&bytes) != *digest {
        return Err(BlobError::DigestMismatch);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_blob_opens_to_its_bytes() {
        let bytes = b"a reading, a reading, a reading".repeat(10);
        let (digest, stored) = seal(&bytes).expect("seals");
        assert_eq!(stored[0], CODEC_ZSTD);
        assert!(stored.len() < bytes.len());
        assert_eq!(open(&digest, &stored), Ok(bytes));
    }

    #[test]
    fn the_digest_is_sha256_of_the_uncompressed_bytes() {
        // SHA-256 of the empty string.
        let empty = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ];
        assert_eq!(seal(b"").map(|(d, _)| d), Ok(empty));
    }

    #[test]
    fn bytes_that_do_not_match_their_name_are_refused() {
        let (_, stored) = seal(b"one").expect("seals");
        let (other, _) = seal(b"two").expect("seals");
        assert_eq!(open(&other, &stored), Err(BlobError::DigestMismatch));
    }

    #[test]
    fn an_unknown_codec_or_damaged_body_is_refused() {
        let (digest, mut stored) = seal(b"bytes").expect("seals");
        assert_eq!(open(&digest, &[]), Err(BlobError::Empty));
        stored[0] = 9;
        assert_eq!(open(&digest, &stored), Err(BlobError::UnknownCodec(9)));
        assert_eq!(
            open(&digest, &[CODEC_ZSTD, 1, 2, 3]),
            Err(BlobError::Corrupt)
        );
    }

    #[test]
    fn a_blob_is_bounded_both_ways() {
        let big = vec![0u8; MAX_BLOB_LEN + 1];
        assert_eq!(seal(&big), Err(BlobError::TooLarge));
        // A small compressed body that expands past the bound is refused while decompressing.
        let mut stored = vec![CODEC_ZSTD];
        stored.extend(zstd::bulk::compress(&big, ZSTD_LEVEL).expect("compresses"));
        assert_eq!(open(&digest_of(&big), &stored), Err(BlobError::TooLarge));
    }
}
