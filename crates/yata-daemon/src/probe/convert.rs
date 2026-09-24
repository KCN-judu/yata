//! Domain ↔ message conversion for the probe channel (ADR-0004, rule 1): a `ReadResult` becomes a
//! [`SoulReading`], and a reading's blob is its `ReadResult` serialized (ADR-0008, "Importing a
//! file", step 2).
//!
//! Conversion changes no value. Evidence is carried as the reader states it and never raised; an
//! evidence entry for a field this build does not know is dropped with the field.

use std::collections::BTreeMap;

use prost::Message;
use yata_core::import::observation::{
    AttributeReading, Coverage, Evidence, ObservedRecord, RawValue, SequenceKind, SoulField,
    SoulObservation, SoulReading, SubAttributeReading, UnreadReason,
};
use yata_protocol::probe;

/// Why a `ReadResult` is not a soul reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertError {
    /// A scope this build does not read, or none.
    NotSouls { scope: i32 },
    /// A field's evidence stated twice.
    DuplicateEvidence { field: String },
    /// Evidence with no stated strength.
    UnstatedEvidence { field: String },
}

/// The blob of a reading: its `ReadResult` as protobuf bytes, without the request id, which is
/// the session's and not the reading's. The same reading has the same blob whether it arrived by
/// pipe or by file.
pub fn blob_of(result: &probe::ReadResult) -> Vec<u8> {
    probe::ReadResult {
        request_id: 0,
        ..result.clone()
    }
    .encode_to_vec()
}

/// The `FieldEvidence.field` that names the soul recognition rule.
pub const SOUL_RECOGNITION: &str = "SoulRecord";

pub fn soul_reading(result: &probe::ReadResult) -> Result<SoulReading, ConvertError> {
    let souls = match (&result.records, result.scope()) {
        (Some(probe::read_result::Records::Souls(s)), probe::Scope::Souls) => &s.souls,
        (None, probe::Scope::Souls) => &Vec::new(),
        _ => {
            return Err(ConvertError::NotSouls {
                scope: result.scope,
            });
        }
    };
    let mut evidence = BTreeMap::new();
    let mut recognition = None;
    for e in &result.field_evidence {
        let strength = match e.evidence() {
            probe::Evidence::Inherited => Evidence::Inherited,
            probe::Evidence::Established => Evidence::Established,
            probe::Evidence::Unspecified => {
                return Err(ConvertError::UnstatedEvidence {
                    field: e.field.clone(),
                });
            }
        };
        if e.field == SOUL_RECOGNITION {
            if recognition.replace(strength).is_some() {
                return Err(ConvertError::DuplicateEvidence {
                    field: e.field.clone(),
                });
            }
            continue;
        }
        let Some(field) = SoulField::from_schema_name(&e.field) else {
            continue;
        };
        if evidence.insert(field, strength).is_some() {
            return Err(ConvertError::DuplicateEvidence {
                field: e.field.clone(),
            });
        }
    }
    // A typed value for a field with no evidence entry is not trusted as mapped.
    let mapped = |f: SoulField| evidence.contains_key(&f);
    let souls = souls
        .iter()
        .map(|s| SoulObservation {
            soul_id: s.soul_id.clone().filter(|_| mapped(SoulField::SoulId)),
            suit_code: s.suit_code.filter(|_| mapped(SoulField::SuitCode)),
            star: s.star.filter(|_| mapped(SoulField::Star)),
            slot: s.slot.filter(|_| mapped(SoulField::Slot)),
            level: s.level.filter(|_| mapped(SoulField::Level)),
            main: s
                .main
                .as_ref()
                .filter(|_| mapped(SoulField::Main))
                .map(attribute),
            subs: if mapped(SoulField::Subs) {
                s.subs
                    .iter()
                    .map(|a| SubAttributeReading {
                        code: a.attribute_code,
                        value: a.value,
                        roll_count: a.roll_count,
                    })
                    .collect()
            } else {
                Vec::new()
            },
            innate: s
                .innate
                .as_ref()
                .filter(|_| mapped(SoulField::Innate))
                .map(attribute),
            locked: s.locked.filter(|_| mapped(SoulField::Locked)),
            discarded: s.discarded.filter(|_| mapped(SoulField::Discarded)),
            observed: s.observed.as_ref().map(observed),
        })
        .collect();
    Ok(SoulReading {
        coverage: match result.coverage() {
            probe::Coverage::Complete => Coverage::Complete,
            probe::Coverage::Partial => Coverage::Partial,
            probe::Coverage::Unspecified => Coverage::Unstated,
        },
        account: Some(result.observed_account_id.clone()).filter(|a| !a.is_empty()),
        recognition,
        evidence,
        souls,
    })
}

fn attribute(a: &probe::AttributeValue) -> AttributeReading {
    AttributeReading {
        code: a.attribute_code,
        value: a.value,
    }
}

fn observed(r: &probe::ObservedRecord) -> ObservedRecord {
    ObservedRecord {
        type_name: r.type_name.clone(),
        container_key: r.container_key.as_ref().map(raw),
        entries: r.entries.iter().map(entry).collect(),
    }
}

fn entry(e: &probe::RawEntry) -> (RawValue, RawValue) {
    let side = |v: &Option<probe::RawValue>| v.as_ref().map_or_else(unstated, raw);
    (side(&e.key), side(&e.value))
}

fn unstated() -> RawValue {
    RawValue::Unread {
        type_name: String::new(),
        reason: UnreadReason::Unstated,
    }
}

fn raw(v: &probe::RawValue) -> RawValue {
    use probe::raw_value::Kind;
    match &v.kind {
        None => unstated(),
        Some(Kind::Null(_)) => RawValue::Null,
        Some(Kind::Boolean(b)) => RawValue::Bool(*b),
        Some(Kind::Integer(n)) => RawValue::Integer(*n),
        Some(Kind::Float(x)) => RawValue::Float(*x),
        Some(Kind::Text(t)) => RawValue::Text(t.clone()),
        Some(Kind::Sequence(s)) => RawValue::Sequence {
            kind: match s.kind() {
                probe::SequenceKind::List => SequenceKind::List,
                probe::SequenceKind::Tuple => SequenceKind::Tuple,
                probe::SequenceKind::Unspecified => SequenceKind::Unstated,
            },
            items: s.items.iter().map(raw).collect(),
            length: s.length.max(s.items.len() as u64),
        },
        Some(Kind::Mapping(m)) => RawValue::Mapping {
            entries: m.entries.iter().map(entry).collect(),
            length: m.length.max(m.entries.len() as u64),
        },
        Some(Kind::Unread(u)) => RawValue::Unread {
            type_name: u.type_name.clone(),
            reason: match u.reason() {
                probe::UnreadReason::UnknownKind => UnreadReason::UnknownKind,
                probe::UnreadReason::DepthLimit => UnreadReason::DepthLimit,
                probe::UnreadReason::Unreadable => UnreadReason::Unreadable,
                probe::UnreadReason::Malformed => UnreadReason::Malformed,
                probe::UnreadReason::OutOfRange => UnreadReason::OutOfRange,
                probe::UnreadReason::Unspecified => UnreadReason::Unstated,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use probe::{FieldEvidence, ReadResult, SoulRecord, SoulRecords, read_result::Records};

    fn result(souls: Vec<SoulRecord>, evidence: Vec<(&str, probe::Evidence)>) -> ReadResult {
        ReadResult {
            request_id: 1,
            scope: probe::Scope::Souls.into(),
            coverage: probe::Coverage::Complete.into(),
            records: Some(Records::Souls(SoulRecords { souls })),
            field_evidence: evidence
                .into_iter()
                .map(|(f, e)| FieldEvidence {
                    field: f.into(),
                    evidence: e.into(),
                    basis: String::new(),
                })
                .collect(),
            ..ReadResult::default()
        }
    }

    #[test]
    fn a_typed_value_without_evidence_is_not_taken_as_mapped() {
        let r = result(
            vec![SoulRecord {
                soul_id: Some("x".into()),
                star: Some(6),
                ..SoulRecord::default()
            }],
            vec![("SoulRecord.soul_id", probe::Evidence::Inherited)],
        );
        let reading = soul_reading(&r).expect("souls");
        assert_eq!(reading.souls[0].soul_id.as_deref(), Some("x"));
        assert_eq!(reading.souls[0].star, None);
        assert_eq!(
            reading.evidence_of(SoulField::SoulId),
            Some(Evidence::Inherited)
        );
        assert_eq!(reading.evidence_of(SoulField::Star), None);
    }

    #[test]
    fn evidence_is_carried_as_stated_and_never_raised() {
        let r = result(
            vec![],
            vec![
                ("SoulRecord.suit_code", probe::Evidence::Established),
                ("SoulRecord.future_field", probe::Evidence::Established),
            ],
        );
        let reading = soul_reading(&r).expect("souls");
        assert_eq!(
            reading.evidence_of(SoulField::SuitCode),
            Some(Evidence::Established)
        );
        assert_eq!(reading.evidence.len(), 1);
    }

    #[test]
    fn unstated_or_repeated_evidence_is_refused() {
        let unstated = result(
            vec![],
            vec![("SoulRecord.star", probe::Evidence::Unspecified)],
        );
        assert!(matches!(
            soul_reading(&unstated),
            Err(ConvertError::UnstatedEvidence { .. })
        ));
        let twice = result(
            vec![],
            vec![
                ("SoulRecord.star", probe::Evidence::Inherited),
                ("SoulRecord.star", probe::Evidence::Established),
            ],
        );
        assert!(matches!(
            soul_reading(&twice),
            Err(ConvertError::DuplicateEvidence { .. })
        ));
    }

    #[test]
    fn the_recognition_rule_has_its_own_evidence() {
        let r = result(vec![], vec![("SoulRecord", probe::Evidence::Inherited)]);
        let reading = soul_reading(&r).expect("souls");
        assert_eq!(reading.recognition, Some(Evidence::Inherited));
        assert!(reading.evidence.is_empty());
    }

    #[test]
    fn the_request_id_is_not_part_of_the_blob() {
        let a = result(vec![SoulRecord::default()], vec![]);
        let b = ReadResult {
            request_id: 7,
            ..a.clone()
        };
        assert_eq!(blob_of(&a), blob_of(&b));
    }

    #[test]
    fn another_scope_is_not_a_soul_reading() {
        let r = ReadResult::default();
        assert_eq!(soul_reading(&r), Err(ConvertError::NotSouls { scope: 0 }));
    }

    #[test]
    fn raw_values_convert_without_change() {
        use probe::raw_value::Kind;
        let v = probe::RawValue {
            kind: Some(Kind::Sequence(probe::RawSequence {
                kind: probe::SequenceKind::Tuple.into(),
                items: vec![
                    probe::RawValue {
                        kind: Some(Kind::Integer(-3)),
                    },
                    probe::RawValue { kind: None },
                ],
                truncated: true,
                length: 9,
            })),
        };
        assert_eq!(
            raw(&v),
            RawValue::Sequence {
                kind: SequenceKind::Tuple,
                items: vec![RawValue::Integer(-3), unstated()],
                length: 9,
            }
        );
    }
}
