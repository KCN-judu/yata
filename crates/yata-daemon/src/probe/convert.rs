//! Domain ↔ message conversion for the probe channel (ADR-0004, rule 1): a wire `Reading` becomes
//! a [`SoulReading`], and a reading's blob is its `Reading` serialized (ADR-0008, "Importing a
//! file", step 2).
//!
//! Conversion changes no value and repairs nothing. Every shape the schema says is refused —
//! unset coverage, an unset oneof, an unspecified enum, a cut sequence whose full length is not
//! above its items — is a [`ConvertError`], never a default.

use prost::Message;
use yata_core::import::observation::{
    AttributeReading, Coverage, GameAttributeCode, GameLevel, GameSlot, GameStar, GameSuitCode,
    InnateReading, Mapping, ObservedRecord, RawSoul, RawValue, SequenceKind, SoulMappings,
    SoulReading, SubAttributeReading, UnreadReason,
};
use yata_protocol::probe;

/// Why a wire reading is not a reading this build accepts: `import.malformed_reading`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertError {
    /// A reading with no records: its scope is unknown, and no empty inventory is assumed.
    NoRecords,
    /// Coverage unspecified.
    UnstatedCoverage,
    /// A mapping whose evidence is unset, by field name (`recognition` for the rule).
    UnstatedEvidence { field: &'static str },
    /// An innate reading with neither case set.
    UnstatedInnate,
    /// A raw value, a sequence kind, or an unread reason unset or unspecified.
    UnstatedValue,
    /// A cut sequence or mapping whose full length is not above what it holds.
    BadLength { shown: u64, full: u64 },
}

/// The blob of a reading: its `Reading` as protobuf bytes. The request id is not in it, so the
/// same reading has the same blob whether it arrived by pipe or by file.
pub fn blob_of(reading: &probe::Reading) -> Vec<u8> {
    reading.encode_to_vec()
}

pub fn soul_reading(reading: &probe::Reading) -> Result<SoulReading, ConvertError> {
    let Some(probe::reading::Records::Souls(records)) = &reading.records else {
        return Err(ConvertError::NoRecords);
    };
    let coverage = match reading.coverage() {
        probe::Coverage::Complete => Coverage::Complete,
        probe::Coverage::Partial => Coverage::Partial,
        probe::Coverage::Unspecified => return Err(ConvertError::UnstatedCoverage),
    };
    let recognition = records
        .recognition
        .as_ref()
        .map(|m| mapping(m, "recognition"))
        .transpose()?;
    let mappings = mappings(records.mappings.as_ref())?;
    let souls = records
        .souls
        .iter()
        .map(raw_soul)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SoulReading::new(
        coverage,
        reading.observed_account_id.clone(),
        recognition,
        mappings,
        souls,
    ))
}

fn mapping(m: &probe::Mapping, field: &'static str) -> Result<Mapping, ConvertError> {
    match &m.evidence {
        Some(probe::mapping::Evidence::Inherited(_)) => Ok(Mapping::Inherited),
        Some(probe::mapping::Evidence::Established(e)) => Ok(Mapping::Established {
            basis: e.basis.clone(),
        }),
        None => Err(ConvertError::UnstatedEvidence { field }),
    }
}

fn mappings(m: Option<&probe::SoulMappings>) -> Result<SoulMappings, ConvertError> {
    let Some(m) = m else {
        return Ok(SoulMappings::default());
    };
    let one = |m: &Option<probe::Mapping>, field| m.as_ref().map(|m| mapping(m, field)).transpose();
    Ok(SoulMappings {
        soul_id: one(&m.soul_id, "soul_id")?,
        suit_code: one(&m.suit_code, "suit_code")?,
        star: one(&m.star, "star")?,
        slot: one(&m.slot, "slot")?,
        level: one(&m.level, "level")?,
        main: one(&m.main, "main")?,
        subs: one(&m.subs, "subs")?,
        innate: one(&m.innate, "innate")?,
        locked: one(&m.locked, "locked")?,
        discarded: one(&m.discarded, "discarded")?,
    })
}

fn raw_soul(s: &probe::SoulRecord) -> Result<RawSoul, ConvertError> {
    Ok(RawSoul {
        soul_id: s.soul_id.clone(),
        suit_code: s.suit_code.map(GameSuitCode),
        star: s.star.map(GameStar),
        slot: s.slot.map(GameSlot),
        level: s.level.map(GameLevel),
        main: s.main.as_ref().map(attribute),
        subs: s.subs.as_ref().map(|subs| {
            subs.items
                .iter()
                .map(|a| SubAttributeReading {
                    code: GameAttributeCode(a.attribute_code),
                    value: a.value,
                    roll_count: a.roll_count,
                })
                .collect()
        }),
        innate: s.innate.as_ref().map(innate).transpose()?,
        locked: s.locked,
        discarded: s.discarded,
        observed: s.observed.as_ref().map(observed).transpose()?,
    })
}

fn attribute(a: &probe::AttributeValue) -> AttributeReading {
    AttributeReading {
        code: GameAttributeCode(a.attribute_code),
        value: a.value,
    }
}

fn innate(i: &probe::InnateReading) -> Result<InnateReading, ConvertError> {
    match &i.state {
        Some(probe::innate_reading::State::None(_)) => Ok(InnateReading::None),
        Some(probe::innate_reading::State::Present(a)) => Ok(InnateReading::Present(attribute(a))),
        None => Err(ConvertError::UnstatedInnate),
    }
}

fn observed(r: &probe::ObservedRecord) -> Result<ObservedRecord, ConvertError> {
    Ok(ObservedRecord {
        type_name: r.type_name.clone(),
        container_key: r.container_key.as_ref().map(raw).transpose()?,
        entries: r.entries.iter().map(entry).collect::<Result<_, _>>()?,
    })
}

fn entry(e: &probe::RawEntry) -> Result<(RawValue, RawValue), ConvertError> {
    let side =
        |v: &Option<probe::RawValue>| v.as_ref().ok_or(ConvertError::UnstatedValue).and_then(raw);
    Ok((side(&e.key)?, side(&e.value)?))
}

/// A cut collection's full length, which must be above what it holds.
fn full_length(shown: usize, full: Option<u64>) -> Result<Option<u64>, ConvertError> {
    match full {
        Some(f) if f <= shown as u64 => Err(ConvertError::BadLength {
            shown: shown as u64,
            full: f,
        }),
        other => Ok(other),
    }
}

fn raw(v: &probe::RawValue) -> Result<RawValue, ConvertError> {
    use probe::raw_value::Kind;
    Ok(match v.kind.as_ref().ok_or(ConvertError::UnstatedValue)? {
        Kind::Null(_) => RawValue::Null,
        Kind::Boolean(b) => RawValue::Bool(*b),
        Kind::Integer(n) => RawValue::Integer(*n),
        Kind::Float(x) => RawValue::Float(*x),
        Kind::Text(t) => RawValue::Text(t.clone()),
        Kind::Sequence(s) => RawValue::Sequence {
            kind: match s.kind() {
                probe::SequenceKind::List => SequenceKind::List,
                probe::SequenceKind::Tuple => SequenceKind::Tuple,
                probe::SequenceKind::Unspecified => return Err(ConvertError::UnstatedValue),
            },
            full_length: full_length(s.items.len(), s.full_length)?,
            items: s.items.iter().map(raw).collect::<Result<_, _>>()?,
        },
        Kind::Mapping(m) => RawValue::Mapping {
            full_length: full_length(m.entries.len(), m.full_length)?,
            entries: m.entries.iter().map(entry).collect::<Result<_, _>>()?,
        },
        Kind::Unread(u) => RawValue::Unread {
            type_name: u.type_name.clone(),
            reason: match u.reason() {
                probe::UnreadReason::UnknownKind => UnreadReason::UnknownKind,
                probe::UnreadReason::DepthLimit => UnreadReason::DepthLimit,
                probe::UnreadReason::Unreadable => UnreadReason::Unreadable,
                probe::UnreadReason::Malformed => UnreadReason::Malformed,
                probe::UnreadReason::OutOfRange => UnreadReason::OutOfRange,
                probe::UnreadReason::Unspecified => return Err(ConvertError::UnstatedValue),
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use probe::{
        Established, Inherited, Reading, SoulRecord, SoulRecords, mapping::Evidence as E,
        reading::Records,
    };
    use yata_core::import::observation::{Evidence, Field};

    fn inherited() -> probe::Mapping {
        probe::Mapping {
            evidence: Some(E::Inherited(Inherited {})),
        }
    }

    fn reading(souls: Vec<SoulRecord>, mappings: probe::SoulMappings) -> Reading {
        Reading {
            coverage: probe::Coverage::Complete.into(),
            records: Some(Records::Souls(SoulRecords {
                souls,
                recognition: Some(inherited()),
                mappings: Some(mappings),
            })),
            ..Reading::default()
        }
    }

    #[test]
    fn a_value_for_an_unmapped_field_is_not_taken() {
        let r = reading(
            vec![SoulRecord {
                soul_id: Some("x".into()),
                star: Some(6),
                ..SoulRecord::default()
            }],
            probe::SoulMappings {
                soul_id: Some(inherited()),
                ..probe::SoulMappings::default()
            },
        );
        let reading = soul_reading(&r).expect("souls");
        let soul = &reading.souls()[0];
        assert_eq!(soul.soul_id.value().map(String::as_str), Some("x"));
        assert_eq!(soul.soul_id.evidence(), Some(Evidence::Inherited));
        assert_eq!(soul.star, Field::Unmapped);
        assert_eq!(reading.recognition(), Some(&Mapping::Inherited));
    }

    #[test]
    fn an_established_mapping_keeps_its_basis() {
        let r = reading(
            vec![],
            probe::SoulMappings {
                suit_code: Some(probe::Mapping {
                    evidence: Some(E::Established(Established {
                        basis: "exp-1".into(),
                    })),
                }),
                ..probe::SoulMappings::default()
            },
        );
        let reading = soul_reading(&r).expect("souls");
        assert_eq!(
            reading.mappings().suit_code,
            Some(Mapping::Established {
                basis: "exp-1".into()
            })
        );
    }

    #[test]
    fn unstated_shapes_are_refused_not_defaulted() {
        let no_records = Reading {
            coverage: probe::Coverage::Complete.into(),
            ..Reading::default()
        };
        assert_eq!(soul_reading(&no_records), Err(ConvertError::NoRecords));
        let mut no_coverage = reading(vec![], probe::SoulMappings::default());
        no_coverage.coverage = probe::Coverage::Unspecified.into();
        assert_eq!(
            soul_reading(&no_coverage),
            Err(ConvertError::UnstatedCoverage)
        );
        let unstated = reading(
            vec![],
            probe::SoulMappings {
                star: Some(probe::Mapping { evidence: None }),
                ..probe::SoulMappings::default()
            },
        );
        assert_eq!(
            soul_reading(&unstated),
            Err(ConvertError::UnstatedEvidence { field: "star" })
        );
        let no_innate_case = reading(
            vec![SoulRecord {
                innate: Some(probe::InnateReading { state: None }),
                ..SoulRecord::default()
            }],
            probe::SoulMappings::default(),
        );
        assert_eq!(
            soul_reading(&no_innate_case),
            Err(ConvertError::UnstatedInnate)
        );
    }

    #[test]
    fn a_cut_length_must_be_above_what_is_held() {
        use probe::raw_value::Kind;
        let seq = |full| probe::RawValue {
            kind: Some(Kind::Sequence(probe::RawSequence {
                kind: probe::SequenceKind::List.into(),
                items: vec![probe::RawValue {
                    kind: Some(Kind::Integer(1)),
                }],
                full_length: full,
            })),
        };
        assert!(matches!(
            raw(&seq(Some(3))),
            Ok(RawValue::Sequence {
                full_length: Some(3),
                ..
            })
        ));
        assert_eq!(
            raw(&seq(Some(1))),
            Err(ConvertError::BadLength { shown: 1, full: 1 })
        );
        assert_eq!(
            raw(&probe::RawValue { kind: None }),
            Err(ConvertError::UnstatedValue)
        );
    }

    #[test]
    fn a_blob_is_the_reading_alone() {
        let r = reading(vec![SoulRecord::default()], probe::SoulMappings::default());
        let a = probe::ReadResult {
            request_id: 1,
            reading: Some(r.clone()),
        };
        let b = probe::ReadResult {
            request_id: 7,
            reading: Some(r.clone()),
        };
        assert_eq!(
            a.reading.as_ref().map(blob_of),
            b.reading.as_ref().map(blob_of)
        );
    }
}
