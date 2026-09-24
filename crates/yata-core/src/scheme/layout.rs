//! The payload's layout: a header, then records, as confirmed by import on 2026-09-24
//! (`scheme-code.md`, "Encoder stages").
//!
//! ```text
//! payload = header record*
//! header  = "ES" account[14] kind[1]
//! record  = u8 len, name (UTF-8) | u8 len, soul mask | u8 len, filter
//! ```
//!
//! A strengthening set holds its plans back to back, with no count and no separator; a discard
//! scheme holds one record. Parsing keeps every byte: the account segment is opaque, and a
//! record's three fields are kept as the bytes read, so [`serialize`] of a parsed payload is the
//! payload again. What the bits inside a mask mean is [`super::edit`]'s, and only for solved bits.

use std::fmt;

use super::RawSchemePayload;
use super::transport::TransportError;

/// The first two bytes of every scheme payload.
pub const MAGIC: [u8; 2] = *b"ES";

/// Bytes in the header: the magic, the account segment, the kind.
pub const HEADER_LEN: usize = 17;

/// Bytes in the account segment.
pub const ACCOUNT_LEN: usize = 14;

/// The longest field a record can hold: its length is one byte.
pub const MAX_FIELD_LEN: usize = u8::MAX as usize;

/// The segment of the header that identifies the account that exported the code. The game shows
/// that account on import. It is account-derived data: kept on the user's machine, never logged,
/// so `Debug` does not print it.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountSegment([u8; ACCOUNT_LEN]);

impl AccountSegment {
    pub fn from_bytes(bytes: [u8; ACCOUNT_LEN]) -> AccountSegment {
        AccountSegment(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; ACCOUNT_LEN] {
        &self.0
    }
}

impl fmt::Debug for AccountSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AccountSegment(<14 bytes, not shown>)")
    }
}

/// What a code holds, from the header's last byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemeKind {
    /// `00`: one discard scheme. Inferred from exported codes; not yet confirmed by an import.
    Discard,
    /// `01`: a strengthening scheme set. Confirmed by import.
    Strengthening,
}

impl SchemeKind {
    fn from_byte(b: u8) -> Option<SchemeKind> {
        match b {
            0 => Some(SchemeKind::Discard),
            1 => Some(SchemeKind::Strengthening),
            _ => None,
        }
    }

    fn byte(self) -> u8 {
        match self {
            SchemeKind::Discard => 0,
            SchemeKind::Strengthening => 1,
        }
    }
}

/// The header: who exported the code, and what it holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemeHeader {
    pub account: AccountSegment,
    pub kind: SchemeKind,
}

impl SchemeHeader {
    fn to_bytes(self) -> [u8; HEADER_LEN] {
        let mut out = [0u8; HEADER_LEN];
        out[..2].copy_from_slice(&MAGIC);
        out[2..2 + ACCOUNT_LEN].copy_from_slice(&self.account.0);
        out[HEADER_LEN - 1] = self.kind.byte();
        out
    }
}

/// One record: a strengthening plan, or a discard scheme's body. Each field is the bytes read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub(super) name: Vec<u8>,
    pub(super) soul_mask: Vec<u8>,
    pub(super) filter: Vec<u8>,
}

impl Record {
    /// The name, if it is UTF-8 (every observed name is).
    pub fn name(&self) -> Option<&str> {
        std::str::from_utf8(&self.name).ok()
    }

    pub fn name_bytes(&self) -> &[u8] {
        &self.name
    }

    /// The soul mask. Empty is the editor's "all souls" choice in a strengthening plan.
    pub fn soul_mask(&self) -> &[u8] {
        &self.soul_mask
    }

    pub fn filter(&self) -> &[u8] {
        &self.filter
    }
}

/// A payload read as its header and records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeLayout {
    pub header: SchemeHeader,
    pub records: Vec<Record>,
}

/// Why a payload does not have the layout, or a layout cannot be written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    /// Shorter than the header.
    TooShort { length: usize },
    /// `decode.unknown_format`: the first bytes are not the magic.
    UnknownFormat,
    /// The header's kind byte is neither known kind.
    UnknownKind { byte: u8 },
    /// A record's field runs past the end of the payload.
    Truncated { offset: usize },
    /// A discard code holds other than one record.
    DiscardRecordCount { count: usize },
    /// A discard scheme built with the "all souls" choice: that choice exists in the
    /// strengthening editor only (`scheme-code.md`, "SoulSelection"), so it is not encoded.
    DiscardAllSouls,
    /// A field longer than a one-byte length can state.
    FieldTooLong { length: usize },
    /// A written payload would exceed the transport's limit.
    Payload(TransportError),
}

/// The header of a payload alone: enough to learn a user's account from a code they share.
pub fn header_of(payload: &RawSchemePayload) -> Result<SchemeHeader, LayoutError> {
    let bytes = payload.as_bytes();
    let Some(header) = bytes.first_chunk::<HEADER_LEN>() else {
        return Err(LayoutError::TooShort {
            length: bytes.len(),
        });
    };
    if header[..2] != MAGIC {
        return Err(LayoutError::UnknownFormat);
    }
    let kind = SchemeKind::from_byte(header[HEADER_LEN - 1]).ok_or(LayoutError::UnknownKind {
        byte: header[HEADER_LEN - 1],
    })?;
    let mut account = [0u8; ACCOUNT_LEN];
    account.copy_from_slice(&header[2..2 + ACCOUNT_LEN]);
    Ok(SchemeHeader {
        account: AccountSegment(account),
        kind,
    })
}

/// The payload as its header and records.
pub fn parse(payload: &RawSchemePayload) -> Result<SchemeLayout, LayoutError> {
    let header = header_of(payload)?;
    let bytes = payload.as_bytes();
    let mut at = HEADER_LEN;
    let mut records = Vec::new();
    let field = |at: &mut usize| -> Result<Vec<u8>, LayoutError> {
        let len = usize::from(
            *bytes
                .get(*at)
                .ok_or(LayoutError::Truncated { offset: *at })?,
        );
        let start = *at + 1;
        let value = bytes
            .get(start..start + len)
            .ok_or(LayoutError::Truncated { offset: *at })?;
        *at = start + len;
        Ok(value.to_vec())
    };
    while at < bytes.len() {
        records.push(Record {
            name: field(&mut at)?,
            soul_mask: field(&mut at)?,
            filter: field(&mut at)?,
        });
    }
    let layout = SchemeLayout { header, records };
    check_kind(&layout)?;
    Ok(layout)
}

/// The payload of a layout: the exact inverse of [`parse`].
pub fn serialize(layout: &SchemeLayout) -> Result<RawSchemePayload, LayoutError> {
    check_kind(layout)?;
    let mut out = layout.header.to_bytes().to_vec();
    for r in &layout.records {
        for f in [&r.name, &r.soul_mask, &r.filter] {
            let len =
                u8::try_from(f.len()).map_err(|_| LayoutError::FieldTooLong { length: f.len() })?;
            out.push(len);
            out.extend_from_slice(f);
        }
    }
    RawSchemePayload::new(out).map_err(LayoutError::Payload)
}

fn check_kind(layout: &SchemeLayout) -> Result<(), LayoutError> {
    match (layout.header.kind, layout.records.len()) {
        (SchemeKind::Discard, 1) | (SchemeKind::Strengthening, _) => Ok(()),
        (SchemeKind::Discard, count) => Err(LayoutError::DiscardRecordCount { count }),
    }
}

impl SchemeLayout {
    /// A new strengthening scheme set for an account.
    pub fn strengthening(account: AccountSegment, plans: Vec<Record>) -> SchemeLayout {
        SchemeLayout {
            header: SchemeHeader {
                account,
                kind: SchemeKind::Strengthening,
            },
            records: plans,
        }
    }

    /// A new discard scheme for an account: one record, which must name its souls.
    pub fn discard(account: AccountSegment, scheme: Record) -> Result<SchemeLayout, LayoutError> {
        if scheme.soul_mask.is_empty() {
            return Err(LayoutError::DiscardAllSouls);
        }
        Ok(SchemeLayout {
            header: SchemeHeader {
                account,
                kind: SchemeKind::Discard,
            },
            records: vec![scheme],
        })
    }

    /// The same layout with another header's account (`scheme-code.md`, "The header and the
    /// user's account"). The kind is kept.
    pub fn with_account(mut self, account: AccountSegment) -> SchemeLayout {
        self.header.account = account;
        self
    }
}

impl Record {
    /// A record from its three fields, each at most [`MAX_FIELD_LEN`] bytes, taken as given.
    pub fn new(name: &str, soul_mask: Vec<u8>, filter: Vec<u8>) -> Result<Record, LayoutError> {
        for f in [name.as_bytes(), &soul_mask, &filter] {
            if f.len() > MAX_FIELD_LEN {
                return Err(LayoutError::FieldTooLong { length: f.len() });
            }
        }
        Ok(Record {
            name: name.as_bytes().to_vec(),
            soul_mask,
            filter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACCOUNT: [u8; 14] = [0xa5; 14];

    fn header(kind: SchemeKind) -> SchemeHeader {
        SchemeHeader {
            account: AccountSegment::from_bytes(ACCOUNT),
            kind,
        }
    }

    fn record(name: &str, soul: &[u8], filter: &[u8]) -> Record {
        Record::new(name, soul.to_vec(), filter.to_vec()).expect("short fields")
    }

    fn payload(bytes: Vec<u8>) -> RawSchemePayload {
        RawSchemePayload::new(bytes).expect("valid payload")
    }

    /// A synthetic set: a synthetic account, a plan choosing all souls, and a plan choosing one.
    fn synthetic_set() -> SchemeLayout {
        SchemeLayout {
            header: header(SchemeKind::Strengthening),
            records: vec![
                record("全部", &[], &[0x3f, 0x08, 0, 0, 0, 0, 0x02]),
                record(
                    "one",
                    &[0, 0, 0, 0, 0x80],
                    &[0x02, 0x08, 0x04, 0, 0x08, 0, 0x03],
                ),
            ],
        }
    }

    #[test]
    fn the_wire_is_header_then_length_prefixed_fields() {
        let bytes = serialize(&synthetic_set()).expect("writable").into_bytes();
        assert_eq!(&bytes[..2], b"ES");
        assert_eq!(&bytes[2..16], &ACCOUNT);
        assert_eq!(bytes[16], 1);
        // "全部" is 6 bytes of UTF-8, then an empty soul mask, then a 7-byte filter.
        assert_eq!(bytes[17], 6);
        assert_eq!(bytes[24], 0);
        assert_eq!(bytes[25], 7);
        assert_eq!(
            bytes.len(),
            17 + (1 + 6 + 1 + 1 + 7) + (1 + 3 + 1 + 5 + 1 + 7)
        );
    }

    #[test]
    fn parsing_what_was_written_gives_the_layout_back() {
        let layout = synthetic_set();
        let p = serialize(&layout).expect("writable");
        assert_eq!(parse(&p), Ok(layout));
    }

    #[test]
    fn a_payload_that_does_not_start_with_the_magic_is_unknown() {
        let mut bytes = serialize(&synthetic_set()).expect("writable").into_bytes();
        bytes[0] = b'X';
        assert_eq!(parse(&payload(bytes)), Err(LayoutError::UnknownFormat));
    }

    #[test]
    fn an_unknown_kind_is_refused() {
        let mut bytes = serialize(&synthetic_set()).expect("writable").into_bytes();
        bytes[16] = 7;
        assert_eq!(
            parse(&payload(bytes)),
            Err(LayoutError::UnknownKind { byte: 7 })
        );
    }

    #[test]
    fn a_payload_shorter_than_a_header_is_refused() {
        assert_eq!(
            parse(&payload(b"ES".to_vec())),
            Err(LayoutError::TooShort { length: 2 })
        );
    }

    #[test]
    fn a_field_running_past_the_end_is_truncated() {
        let mut bytes = serialize(&synthetic_set()).expect("writable").into_bytes();
        bytes.pop();
        assert!(matches!(
            parse(&payload(bytes)),
            Err(LayoutError::Truncated { .. })
        ));
    }

    #[test]
    fn a_discard_code_holds_exactly_one_record() {
        let mut layout = synthetic_set();
        layout.header.kind = SchemeKind::Discard;
        assert_eq!(
            serialize(&layout),
            Err(LayoutError::DiscardRecordCount { count: 2 })
        );
        layout.records.pop();
        assert!(serialize(&layout).is_ok());
    }

    #[test]
    fn a_new_discard_scheme_is_one_record_with_kind_zero() {
        let account = AccountSegment::from_bytes(ACCOUNT);
        let scheme = record("弃置", &[0x01], &[0x3f, 0x08, 0, 0, 0, 0, 0x02]);
        let layout = SchemeLayout::discard(account, scheme).expect("names its souls");
        let bytes = serialize(&layout).expect("writable").into_bytes();
        assert_eq!(bytes[16], 0);
        assert_eq!(parse(&payload(bytes)), Ok(layout));
    }

    #[test]
    fn a_new_discard_scheme_cannot_choose_all_souls() {
        let account = AccountSegment::from_bytes(ACCOUNT);
        assert_eq!(
            SchemeLayout::discard(account, record("x", &[], &[0x01])),
            Err(LayoutError::DiscardAllSouls)
        );
    }

    #[test]
    fn a_set_with_no_plans_is_only_a_header() {
        let layout = SchemeLayout {
            header: header(SchemeKind::Strengthening),
            records: vec![],
        };
        let p = serialize(&layout).expect("writable");
        assert_eq!(p.len(), HEADER_LEN);
        assert_eq!(parse(&p), Ok(layout));
    }

    #[test]
    fn a_field_too_long_for_its_length_byte_is_refused() {
        assert_eq!(
            Record::new(&"x".repeat(256), vec![], vec![]),
            Err(LayoutError::FieldTooLong { length: 256 })
        );
    }

    #[test]
    fn changing_the_account_changes_only_the_account_bytes() {
        let before = serialize(&synthetic_set()).expect("writable");
        let other = AccountSegment::from_bytes([0x11; 14]);
        let after = serialize(&synthetic_set().with_account(other)).expect("writable");
        let d = super::super::inspect::diff(&before, &after);
        assert!(d.changes.iter().all(|c| (2..16).contains(&c.offset)));
        assert_eq!(d.changes.len(), 14);
        assert_eq!(header_of(&after).map(|h| h.account), Ok(other));
    }

    #[test]
    fn the_account_is_never_printed() {
        let text = format!("{:?}", header(SchemeKind::Strengthening));
        assert!(!text.contains("a5") && !text.contains("165"), "{text}");
    }

    mod properties {
        use proptest::prelude::*;

        use super::*;

        fn field(max: usize) -> impl Strategy<Value = Vec<u8>> {
            proptest::collection::vec(any::<u8>(), 0..max)
        }

        proptest! {
            #[test]
            fn serialize_and_parse_are_inverse(
                account in proptest::array::uniform14(any::<u8>()),
                records in proptest::collection::vec((field(40), field(12), field(10)), 0..20),
            ) {
                let layout = SchemeLayout {
                    header: SchemeHeader { account: AccountSegment::from_bytes(account), kind: SchemeKind::Strengthening },
                    records: records.into_iter().map(|(n, s, f)| Record { name: n, soul_mask: s, filter: f }).collect(),
                };
                let p = serialize(&layout).expect("small layouts fit");
                prop_assert_eq!(parse(&p), Ok(layout.clone()));
                prop_assert_eq!(serialize(&parse(&p).expect("parses")), Ok(p));
            }
        }
    }
}
