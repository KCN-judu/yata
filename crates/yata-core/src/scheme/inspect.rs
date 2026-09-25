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
    pub difference: ByteDifference,
}

/// How the bytes at one offset differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteDifference {
    /// Both payloads have a byte here, and they differ.
    Changed { before: u8, after: u8 },
    /// Only the first payload is this long.
    OnlyBefore(u8),
    /// Only the second payload is this long.
    OnlyAfter(u8),
}

impl ByteChange {
    /// The first payload's byte, if it has one here.
    pub fn before(self) -> Option<u8> {
        match self.difference {
            ByteDifference::Changed { before, .. } | ByteDifference::OnlyBefore(before) => {
                Some(before)
            }
            ByteDifference::OnlyAfter(_) => None,
        }
    }

    /// The second payload's byte, if it has one here.
    pub fn after(self) -> Option<u8> {
        match self.difference {
            ByteDifference::Changed { after, .. } | ByteDifference::OnlyAfter(after) => Some(after),
            ByteDifference::OnlyBefore(_) => None,
        }
    }

    /// `before XOR after`, where both bytes exist.
    pub fn xor(self) -> Option<u8> {
        match self.difference {
            ByteDifference::Changed { before, after } => Some(before ^ after),
            ByteDifference::OnlyBefore(_) | ByteDifference::OnlyAfter(_) => None,
        }
    }

    /// The bit positions within the byte, 0 (least significant) to 7, that differ. Where only one
    /// payload has the byte, every bit differs: the other has no bit there at all.
    pub fn changed_bits(self) -> Vec<u8> {
        match self.xor() {
            Some(x) => (0..8).filter(|b| x >> b & 1 == 1).collect(),
            None => (0..8).collect(),
        }
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
        .filter_map(|offset| {
            let difference = match (a.get(offset).copied(), b.get(offset).copied()) {
                (Some(x), Some(y)) if x == y => return None,
                (Some(before), Some(after)) => ByteDifference::Changed { before, after },
                (Some(x), None) => ByteDifference::OnlyBefore(x),
                (None, Some(y)) => ByteDifference::OnlyAfter(y),
                (None, None) => return None,
            };
            Some(ByteChange { offset, difference })
        })
        .collect();
    PayloadDiff {
        before_len: a.len(),
        after_len: b.len(),
        changes,
    }
}

/// A span of bits to read: `len` bits from absolute bit offset `start`, LSB-first within each
/// byte. From 1 to 64 bits, by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitSpan {
    start: usize,
    len: u8,
}

impl BitSpan {
    /// `len` bits from `start`, or `None` unless `1 ≤ len ≤ 64`.
    pub fn new(start: usize, len: u8) -> Option<BitSpan> {
        (1..=64).contains(&len).then_some(BitSpan { start, len })
    }

    pub fn start(self) -> usize {
        self.start
    }

    pub fn width(self) -> u8 {
        self.len
    }
}

/// A span past the end of the payload, which has `bits` bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpanOutOfRange {
    pub span: BitSpan,
    pub bits: usize,
}

/// The bits of a span, bit `start` as the least significant bit of the result. A raw reading
/// for inspection: whether the span is a field, and of what, is not this function's claim.
pub fn read_bits(payload: &RawSchemePayload, span: BitSpan) -> Result<u64, SpanOutOfRange> {
    let bytes = payload.as_bytes();
    let bits = bytes.len() * 8;
    let end = span.start.checked_add(usize::from(span.len));
    if end.is_none_or(|e| e > bits) {
        return Err(SpanOutOfRange { span, bits });
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
                    difference: ByteDifference::OnlyAfter(7)
                },
                ByteChange {
                    offset: 3,
                    difference: ByteDifference::OnlyAfter(6)
                },
            ]
        );
        assert_eq!(d.changes[0].xor(), None);
        // A byte only one payload has differs in every bit.
        assert_eq!(d.changes[0].changed_bits(), (0..8).collect::<Vec<u8>>());
        assert_eq!(d.changes[0].changed_bit_offsets()[0], 16);
    }

    #[test]
    fn bits_are_read_lsb_first_across_bytes() {
        // 0x05 is bits 0 and 2; 0x01 in the second byte is bit 8.
        let p = payload(&[0x05, 0x01]);
        let span = |start, len| BitSpan::new(start, len).expect("1 to 64 bits");
        assert_eq!(read_bits(&p, span(0, 3)), Ok(0b101));
        assert_eq!(read_bits(&p, span(2, 7)), Ok(0b100_0001));
        assert_eq!(read_bits(&p, span(8, 1)), Ok(1));
    }

    #[test]
    fn a_span_past_the_end_or_of_no_length_is_refused() {
        let p = payload(&[0xff]);
        let span = |start, len| BitSpan::new(start, len).expect("1 to 64 bits");
        assert!(matches!(
            read_bits(&p, span(4, 5)),
            Err(SpanOutOfRange { bits: 8, .. })
        ));
        assert_eq!(BitSpan::new(0, 0), None);
        assert_eq!(BitSpan::new(0, 65), None);
        assert!(read_bits(&p, span(usize::MAX, 8)).is_err());
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
                    prop_assert_eq!((f.offset, f.before(), f.after()), (r.offset, r.after(), r.before()));
                }
            }

            #[test]
            fn a_payload_never_differs_from_itself(a in bytes()) {
                prop_assert!(diff(&payload(&a), &payload(&a)).is_identical());
            }
        }
    }
}
