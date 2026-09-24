//! Looking at payloads without interpreting them: a hex dump, a diff, and a bit reader.
//!
//! These exist for controlled experiments: change one thing in the game, export the code, and
//! compare. Nothing here names a field or reads a value as a number, a flag, or text. A byte is
//! a byte until `research/scheme-code-protocol.md` (local research, not published) marks it
//! solved, and then the codec, not this module, interprets it.

use super::RawSchemePayload;

/// Bytes per row of a dump.
pub const DUMP_ROW_LEN: usize = 16;

/// One row of a dump: the offset of its first byte, and up to [`DUMP_ROW_LEN`] bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DumpRow<'a> {
    pub offset: usize,
    pub bytes: &'a [u8],
}

/// The payload as rows of [`DUMP_ROW_LEN`] bytes, in order. Formatting is the caller's.
pub fn dump(payload: &RawSchemePayload) -> Vec<DumpRow<'_>> {
    payload
        .as_bytes()
        .chunks(DUMP_ROW_LEN)
        .enumerate()
        .map(|(i, bytes)| DumpRow {
            offset: i * DUMP_ROW_LEN,
            bytes,
        })
        .collect()
}

/// One offset where two payloads differ. Positions are compared as they stand: no alignment is
/// attempted, because choosing one would be a claim about the layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteChange {
    pub offset: usize,
    /// `None` where the offset is past the end of the first payload.
    pub before: Option<u8>,
    /// `None` where the offset is past the end of the second payload.
    pub after: Option<u8>,
}

impl ByteChange {
    /// `before XOR after`, where both bytes exist.
    pub fn xor(self) -> Option<u8> {
        Some(self.before? ^ self.after?)
    }

    /// The bit positions within the byte, 0 (least significant) to 7, that differ, where both
    /// bytes exist.
    pub fn changed_bits(self) -> Vec<u8> {
        self.xor()
            .map(|x| (0..8).filter(|b| x >> b & 1 == 1).collect())
            .unwrap_or_default()
    }

    /// The same changed bits as absolute bit offsets in the payload, counting LSB-first within
    /// each byte as the research record's bitsets do: byte `n` bit `b` is bit `8n + b`.
    pub fn changed_bit_offsets(self) -> Vec<usize> {
        self.changed_bits()
            .into_iter()
            .map(|b| self.offset * 8 + usize::from(b))
            .collect()
    }
}

/// The comparison of two payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadDiff {
    pub before_len: usize,
    pub after_len: usize,
    /// Every differing offset, ascending, including every offset only one payload has.
    pub changes: Vec<ByteChange>,
}

impl PayloadDiff {
    pub fn is_identical(&self) -> bool {
        self.changes.is_empty()
    }
}

/// Compare two payloads offset by offset.
pub fn diff(before: &RawSchemePayload, after: &RawSchemePayload) -> PayloadDiff {
    let (a, b) = (before.as_bytes(), after.as_bytes());
    let changes = (0..a.len().max(b.len()))
        .map(|offset| ByteChange {
            offset,
            before: a.get(offset).copied(),
            after: b.get(offset).copied(),
        })
        .filter(|c| c.before != c.after)
        .collect();
    PayloadDiff {
        before_len: a.len(),
        after_len: b.len(),
        changes,
    }
}

/// A span of bits to read: `len` bits from absolute bit offset `start`, LSB-first within each
/// byte. At most 64 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitSpan {
    pub start: usize,
    pub len: u8,
}

/// Why a span could not be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitSpanError {
    /// Longer than 64 bits, or empty.
    BadLength { len: u8 },
    /// Past the end of the payload, which has `bits` bits.
    OutOfRange { span: BitSpan, bits: usize },
}

/// The bits of a span, bit `start` as the least significant bit of the result. A raw reading
/// for inspection: whether the span is a field, and of what, is not this function's claim.
pub fn read_bits(payload: &RawSchemePayload, span: BitSpan) -> Result<u64, BitSpanError> {
    if span.len == 0 || span.len > 64 {
        return Err(BitSpanError::BadLength { len: span.len });
    }
    let bytes = payload.as_bytes();
    let bits = bytes.len() * 8;
    let end = span.start.checked_add(usize::from(span.len));
    if end.is_none_or(|e| e > bits) {
        return Err(BitSpanError::OutOfRange { span, bits });
    }
    Ok((0..usize::from(span.len)).fold(0u64, |acc, i| {
        let at = span.start + i;
        let bit = u64::from(bytes[at / 8] >> (at % 8) & 1);
        acc | bit << i
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(bytes: &[u8]) -> RawSchemePayload {
        RawSchemePayload::new(bytes.to_vec()).expect("valid payload")
    }

    #[test]
    fn a_dump_is_rows_of_sixteen_in_order() {
        let bytes: Vec<u8> = (0..40).collect();
        let p = payload(&bytes);
        let rows = dump(&p);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1].offset, 16);
        assert_eq!(rows[1].bytes, &bytes[16..32]);
        assert_eq!(rows[2].bytes, &bytes[32..40]);
        assert_eq!(dump(&p), rows, "a dump is deterministic");
    }

    #[test]
    fn a_payload_does_not_differ_from_itself() {
        let p = payload(b"ES\x00\x01");
        let d = diff(&p, &p);
        assert!(d.is_identical());
        assert_eq!((d.before_len, d.after_len), (4, 4));
    }

    #[test]
    fn a_single_bit_difference_is_located_to_the_bit() {
        let d = diff(&payload(&[0x00, 0x05]), &payload(&[0x00, 0x15]));
        let [c] = d.changes.as_slice() else {
            panic!("one change expected: {d:?}")
        };
        assert_eq!(c.offset, 1);
        assert_eq!(c.xor(), Some(0x10));
        assert_eq!(c.changed_bits(), vec![4]);
        assert_eq!(c.changed_bit_offsets(), vec![12]);
    }

    #[test]
    fn several_changed_bytes_are_listed_in_order() {
        let d = diff(&payload(&[1, 2, 3, 4]), &payload(&[1, 0xff, 3, 0]));
        let offsets: Vec<_> = d.changes.iter().map(|c| c.offset).collect();
        assert_eq!(offsets, vec![1, 3]);
        assert_eq!(d.changes[0].xor(), Some(2 ^ 0xff));
        assert_eq!(d.changes[1].changed_bits(), vec![2]);
    }

    #[test]
    fn a_length_mismatch_reports_the_extra_bytes_without_alignment() {
        let d = diff(&payload(&[9, 8]), &payload(&[9, 8, 7, 6]));
        assert_eq!((d.before_len, d.after_len), (2, 4));
        assert_eq!(
            d.changes,
            vec![
                ByteChange {
                    offset: 2,
                    before: None,
                    after: Some(7)
                },
                ByteChange {
                    offset: 3,
                    before: None,
                    after: Some(6)
                },
            ]
        );
        assert_eq!(d.changes[0].xor(), None);
        assert!(d.changes[0].changed_bits().is_empty());
    }

    #[test]
    fn bits_are_read_lsb_first_across_bytes() {
        // 0x05 is bits 0 and 2; 0x01 in the second byte is bit 8.
        let p = payload(&[0x05, 0x01]);
        assert_eq!(read_bits(&p, BitSpan { start: 0, len: 3 }), Ok(0b101));
        assert_eq!(read_bits(&p, BitSpan { start: 2, len: 7 }), Ok(0b100_0001));
        assert_eq!(read_bits(&p, BitSpan { start: 8, len: 1 }), Ok(1));
    }

    #[test]
    fn a_span_past_the_end_or_of_no_length_is_refused() {
        let p = payload(&[0xff]);
        assert!(matches!(
            read_bits(&p, BitSpan { start: 4, len: 5 }),
            Err(BitSpanError::OutOfRange { bits: 8, .. })
        ));
        assert_eq!(
            read_bits(&p, BitSpan { start: 0, len: 0 }),
            Err(BitSpanError::BadLength { len: 0 })
        );
        assert!(matches!(
            read_bits(
                &p,
                BitSpan {
                    start: usize::MAX,
                    len: 8
                }
            ),
            Err(BitSpanError::OutOfRange { .. })
        ));
    }

    mod properties {
        use proptest::prelude::*;

        use super::*;

        fn bytes() -> impl Strategy<Value = Vec<u8>> {
            proptest::collection::vec(any::<u8>(), 1..256)
        }

        proptest! {
            #[test]
            fn changes_are_exactly_the_offsets_whose_xor_is_nonzero(pair in (1usize..256).prop_flat_map(|n| (
                proptest::collection::vec(any::<u8>(), n),
                proptest::collection::vec(any::<u8>(), n),
            ))) {
                let (a, b) = pair;
                let d = diff(&payload(&a), &payload(&b));
                let expected: Vec<usize> = (0..a.len()).filter(|&i| a[i] ^ b[i] != 0).collect();
                let got: Vec<usize> = d.changes.iter().map(|c| c.offset).collect();
                prop_assert_eq!(got, expected);
                for c in &d.changes {
                    prop_assert_eq!(c.xor(), Some(a[c.offset] ^ b[c.offset]));
                }
            }

            #[test]
            fn diff_is_symmetric(a in bytes(), b in bytes()) {
                let (pa, pb) = (payload(&a), payload(&b));
                let forward = diff(&pa, &pb);
                let back = diff(&pb, &pa);
                prop_assert_eq!(forward.changes.len(), back.changes.len());
                for (f, r) in forward.changes.iter().zip(&back.changes) {
                    prop_assert_eq!((f.offset, f.before, f.after), (r.offset, r.after, r.before));
                }
            }

            #[test]
            fn a_payload_never_differs_from_itself(a in bytes()) {
                prop_assert!(diff(&payload(&a), &payload(&a)).is_identical());
            }
        }
    }
}
