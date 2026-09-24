//! The analyses that turn readings into evidence about the game's layout: what the records hold,
//! how their values group, and whether the inherited suit codes survive an attested reading.
//!
//! They answer research questions, not user ones, and they never change what a reading says. A
//! mapping they support becomes established only when the maintainer records the experiment and
//! the reader states the new evidence (`probe-protocol.md`, "Evidence").

use std::collections::{BTreeMap, BTreeSet};

use super::observation::{RawKind, RawValue, SequenceKind, SoulField, SoulObservation};
use crate::scheme::mapping::{SOUL_BIT_COUNT, soul_set};

/// Where a research tool reads a value from each soul.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// A typed field.
    Field(SoulField),
    /// The key the record is stored under in its container.
    Container,
    /// An entry of the observed record, by key.
    Entry(String),
}

impl Source {
    /// `@soul_id`, `@suit_code`, … for a typed field, `@container` for the container key, and
    /// anything else for the observed entry of that key.
    pub fn parse(spec: &str) -> Option<Source> {
        let Some(name) = spec.strip_prefix('@') else {
            return (!spec.is_empty()).then(|| Source::Entry(spec.to_owned()));
        };
        if name == "container" {
            return Some(Source::Container);
        }
        SoulField::from_schema_name(&format!("SoulRecord.{name}")).map(Source::Field)
    }

    /// The value this source names on a soul, or `None` when the soul lacks it.
    pub fn value(&self, soul: &SoulObservation) -> Option<RawValue> {
        let integer = |n: Option<u32>| n.map(|n| RawValue::Integer(i64::from(n)));
        let pair = |code: u32, value: f64| RawValue::Sequence {
            kind: SequenceKind::Tuple,
            items: vec![RawValue::Integer(i64::from(code)), RawValue::Float(value)],
            length: 2,
        };
        match self {
            Source::Field(SoulField::SoulId) => soul.soul_id.clone().map(RawValue::Text),
            Source::Field(SoulField::SuitCode) => integer(soul.suit_code),
            Source::Field(SoulField::Star) => integer(soul.star),
            Source::Field(SoulField::Slot) => integer(soul.slot),
            Source::Field(SoulField::Level) => integer(soul.level),
            Source::Field(SoulField::Main) => soul.main.map(|a| pair(a.code, a.value)),
            Source::Field(SoulField::Innate) => soul.innate.map(|a| pair(a.code, a.value)),
            Source::Field(SoulField::Subs) => Some(RawValue::Sequence {
                kind: SequenceKind::List,
                items: soul.subs.iter().map(|s| pair(s.code, s.value)).collect(),
                length: soul.subs.len() as u64,
            }),
            Source::Field(SoulField::Locked) => soul.locked.map(RawValue::Bool),
            Source::Field(SoulField::Discarded) => soul.discarded.map(RawValue::Bool),
            Source::Container => soul.observed.as_ref()?.container_key.clone(),
            Source::Entry(key) => soul.observed.as_ref()?.get(key).cloned(),
        }
    }
}

/// A group's key: absent sorts first, then integers by value, then everything else by rendering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupKey {
    Absent,
    Integer(i64),
    Other(String),
}

impl GroupKey {
    pub fn of(value: Option<&RawValue>) -> GroupKey {
        match value {
            None => GroupKey::Absent,
            Some(RawValue::Integer(n)) => GroupKey::Integer(*n),
            Some(v) => GroupKey::Other(v.render()),
        }
    }

    pub fn render(&self) -> String {
        match self {
            GroupKey::Absent => "(absent)".to_owned(),
            GroupKey::Integer(n) => n.to_string(),
            GroupKey::Other(s) => s.clone(),
        }
    }
}

/// Distinct values a survey tracks per key before it stops counting them.
pub const DISTINCT_CAP: usize = 4096;
/// Example renderings a survey keeps per key.
pub const EXAMPLES: usize = 3;

/// What one observed key holds across a reading.
#[derive(Debug, Clone, PartialEq)]
pub struct KeySurvey {
    /// The key, rendered.
    pub key: String,
    /// Records that hold the key.
    pub records: u64,
    pub kinds: BTreeMap<RawKind, u64>,
    /// The smallest and largest integer value, when any value is an integer.
    pub integer_range: Option<(i64, i64)>,
    /// Distinct values, counted up to [`DISTINCT_CAP`].
    pub distinct: usize,
    pub examples: Vec<String>,
}

/// What a reading's observed records hold.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Survey {
    pub souls: u64,
    /// Souls whose observed record the reading carries.
    pub observed: u64,
    pub type_names: BTreeMap<String, u64>,
    pub container_kinds: BTreeMap<RawKind, u64>,
    /// One row per key, in rendering order.
    pub keys: Vec<KeySurvey>,
}

/// Survey every observed record of a set of souls.
pub fn survey<'a>(souls: impl IntoIterator<Item = &'a SoulObservation>) -> Survey {
    let mut s = Survey::default();
    let mut keys: BTreeMap<String, (KeySurvey, BTreeSet<String>)> = BTreeMap::new();
    for soul in souls {
        s.souls += 1;
        let Some(observed) = &soul.observed else {
            continue;
        };
        s.observed += 1;
        *s.type_names.entry(observed.type_name.clone()).or_default() += 1;
        if let Some(k) = &observed.container_key {
            *s.container_kinds.entry(k.kind()).or_default() += 1;
        }
        for (k, v) in &observed.entries {
            let name = k.render();
            let (row, seen) = keys.entry(name.clone()).or_insert_with(|| {
                (
                    KeySurvey {
                        key: name,
                        records: 0,
                        kinds: BTreeMap::new(),
                        integer_range: None,
                        distinct: 0,
                        examples: Vec::new(),
                    },
                    BTreeSet::new(),
                )
            });
            row.records += 1;
            *row.kinds.entry(v.kind()).or_default() += 1;
            if let RawValue::Integer(n) = v {
                row.integer_range = Some(match row.integer_range {
                    None => (*n, *n),
                    Some((lo, hi)) => (lo.min(*n), hi.max(*n)),
                });
            }
            if seen.len() < DISTINCT_CAP {
                let r = v.render();
                if seen.insert(r.clone()) && row.examples.len() < EXAMPLES {
                    row.examples.push(r);
                }
            }
            row.distinct = seen.len();
        }
    }
    s.keys = keys.into_values().map(|(row, _)| row).collect();
    s
}

/// Souls grouped by the value a source names, in [`GroupKey`] order; each group keeps its souls
/// in reading order.
pub fn group<'a>(
    souls: impl IntoIterator<Item = &'a SoulObservation>,
    by: &Source,
) -> BTreeMap<GroupKey, Vec<&'a SoulObservation>> {
    let mut groups: BTreeMap<GroupKey, Vec<&SoulObservation>> = BTreeMap::new();
    for soul in souls {
        groups
            .entry(GroupKey::of(by.value(soul).as_ref()))
            .or_default()
            .push(soul);
    }
    groups
}

/// How many souls hold each pair of values: rows by one source, columns by another. Absent,
/// `null`, and each value are separate columns, which is what a presence question needs.
pub fn crosstab<'a>(
    souls: impl IntoIterator<Item = &'a SoulObservation>,
    rows: &Source,
    columns: &Source,
) -> BTreeMap<(GroupKey, GroupKey), u64> {
    let mut table = BTreeMap::new();
    for soul in souls {
        let r = GroupKey::of(rows.value(soul).as_ref());
        let c = GroupKey::of(columns.value(soul).as_ref());
        *table.entry((r, c)).or_default() += 1;
    }
    table
}

/// A maintainer's statement that the soul with this identity is shown in the game as the soul
/// scheme bit `bit` selects. The bit ↔ soul link is the scheme research's (`scheme-code.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attestation {
    pub identity: String,
    pub bit: u16,
}

/// Where the suit-code inheritance stands for one scheme bit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitStatus {
    /// No attestation names this bit.
    Unattested,
    /// Every attested soul of this bit carries the inherited code, and no soul of another bit
    /// carries it.
    Reestablished,
    /// An attested soul carries another code, or the code is attested for two bits.
    Contradicted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitRow {
    pub bit: u16,
    /// The suit code the prior tool gives the soul of this bit.
    pub inherited: u8,
    /// The codes read from the attested souls, after the offset.
    pub observed: BTreeSet<i64>,
    pub attestations: usize,
    pub status: BitStatus,
}

/// Why an attestation could not be joined to the reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unjoined {
    /// No soul has this identity.
    NoSoul { identity: String },
    /// More than one soul has it.
    Ambiguous { identity: String, souls: usize },
    /// The soul has no integer at the suit source.
    NoSuitValue { identity: String },
    /// A bit beyond the mapped sets.
    NoSuchBit { identity: String, bit: u16 },
}

/// The suit-code ledger: every mapped scheme bit, with what the attested reading says about the
/// code the prior tool gives it (ADR-0014).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuitLedger {
    pub rows: Vec<BitRow>,
    pub unjoined: Vec<Unjoined>,
}

impl SuitLedger {
    /// Whether the reading re-establishes every mapped bit's code, with nothing left unjoined:
    /// the condition for retiring the inherited suit codes.
    pub fn retires_inheritance(&self) -> bool {
        self.unjoined.is_empty()
            && self
                .rows
                .iter()
                .all(|r| r.status == BitStatus::Reestablished)
    }
}

/// Join attestations to a reading. `identity` names each soul the way the attestations do;
/// `suit` is where its suit value is read, and `offset` is subtracted from that value before it
/// is compared with a suit code.
pub fn suit_ledger(
    souls: &[SoulObservation],
    identity: &Source,
    suit: &Source,
    offset: i64,
    attestations: &[Attestation],
) -> SuitLedger {
    let name_of = |s: &SoulObservation| match identity.value(s) {
        Some(RawValue::Text(t)) => Some(t),
        Some(v) => Some(v.render()),
        None => None,
    };
    let mut observed: BTreeMap<u16, (BTreeSet<i64>, usize)> = BTreeMap::new();
    let mut unjoined = Vec::new();
    for a in attestations {
        if soul_set(a.bit).is_none() {
            unjoined.push(Unjoined::NoSuchBit {
                identity: a.identity.clone(),
                bit: a.bit,
            });
            continue;
        }
        let matching: Vec<&SoulObservation> = souls
            .iter()
            .filter(|s| name_of(s).as_deref() == Some(a.identity.as_str()))
            .collect();
        let soul = match matching.as_slice() {
            [] => {
                unjoined.push(Unjoined::NoSoul {
                    identity: a.identity.clone(),
                });
                continue;
            }
            [one] => *one,
            many => {
                unjoined.push(Unjoined::Ambiguous {
                    identity: a.identity.clone(),
                    souls: many.len(),
                });
                continue;
            }
        };
        let Some(RawValue::Integer(v)) = suit.value(soul) else {
            unjoined.push(Unjoined::NoSuitValue {
                identity: a.identity.clone(),
            });
            continue;
        };
        let entry = observed.entry(a.bit).or_default();
        entry.0.insert(v.saturating_sub(offset));
        entry.1 += 1;
    }
    let mut bits_of_code: BTreeMap<i64, BTreeSet<u16>> = BTreeMap::new();
    for (bit, (codes, _)) in &observed {
        for c in codes {
            bits_of_code.entry(*c).or_default().insert(*bit);
        }
    }
    let rows = (0..SOUL_BIT_COUNT)
        .filter_map(|bit| {
            let inherited = soul_set(bit)?.suit_code();
            let (codes, attestations) = observed.remove(&bit).unwrap_or_default();
            let status = if attestations == 0 {
                BitStatus::Unattested
            } else if codes.len() == 1
                && codes.contains(&i64::from(inherited))
                && bits_of_code
                    .get(&i64::from(inherited))
                    .is_some_and(|b| b.len() == 1)
            {
                BitStatus::Reestablished
            } else {
                BitStatus::Contradicted
            };
            Some(BitRow {
                bit,
                inherited,
                observed: codes,
                attestations,
                status,
            })
        })
        .collect();
    SuitLedger { rows, unjoined }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::observation::ObservedRecord;

    fn soul(id: &str, entries: Vec<(&str, RawValue)>) -> SoulObservation {
        SoulObservation {
            observed: Some(ObservedRecord {
                type_name: "record".into(),
                container_key: Some(RawValue::Text(id.into())),
                entries: entries
                    .into_iter()
                    .map(|(k, v)| (RawValue::Text(k.into()), v))
                    .collect(),
            }),
            ..SoulObservation::default()
        }
    }

    #[test]
    fn sources_parse_fields_the_container_and_entries() {
        assert_eq!(
            Source::parse("@suit_code"),
            Some(Source::Field(SoulField::SuitCode))
        );
        assert_eq!(Source::parse("@container"), Some(Source::Container));
        assert_eq!(Source::parse("x"), Some(Source::Entry("x".into())));
        assert_eq!(Source::parse("@nothing"), None);
        assert_eq!(Source::parse(""), None);
    }

    #[test]
    fn a_survey_counts_kinds_ranges_and_distinct_values() {
        let souls = vec![
            soul(
                "a",
                vec![("n", RawValue::Integer(5)), ("t", RawValue::Null)],
            ),
            soul("b", vec![("n", RawValue::Integer(-2))]),
            soul(
                "c",
                vec![("n", RawValue::Integer(5)), ("t", RawValue::Integer(9))],
            ),
            SoulObservation::default(),
        ];
        let s = survey(&souls);
        assert_eq!((s.souls, s.observed), (4, 3));
        assert_eq!(s.container_kinds.get(&RawKind::Text), Some(&3));
        let n = &s.keys[0];
        assert_eq!(n.key, "\"n\"");
        assert_eq!(n.records, 3);
        assert_eq!(n.integer_range, Some((-2, 5)));
        assert_eq!(n.distinct, 2);
        assert_eq!(n.examples, vec!["5", "-2"]);
        let t = &s.keys[1];
        assert_eq!(t.kinds.get(&RawKind::Null), Some(&1));
        assert_eq!(t.kinds.get(&RawKind::Integer), Some(&1));
    }

    #[test]
    fn groups_order_absent_then_integers_numerically() {
        let souls = vec![
            soul("a", vec![("g", RawValue::Integer(10))]),
            soul("b", vec![("g", RawValue::Integer(9))]),
            soul("c", vec![]),
            soul("d", vec![("g", RawValue::Integer(10))]),
        ];
        let g = group(&souls, &Source::Entry("g".into()));
        let keys: Vec<_> = g.keys().cloned().collect();
        assert_eq!(
            keys,
            vec![
                GroupKey::Absent,
                GroupKey::Integer(9),
                GroupKey::Integer(10)
            ]
        );
        assert_eq!(g[&GroupKey::Integer(10)].len(), 2);
    }

    #[test]
    fn a_crosstab_separates_absent_null_and_values() {
        let souls = vec![
            soul(
                "a",
                vec![("s", RawValue::Integer(1)), ("i", RawValue::Null)],
            ),
            soul("b", vec![("s", RawValue::Integer(1))]),
            soul(
                "c",
                vec![("s", RawValue::Integer(2)), ("i", RawValue::Integer(4))],
            ),
        ];
        let t = crosstab(
            &souls,
            &Source::Entry("s".into()),
            &Source::Entry("i".into()),
        );
        assert_eq!(
            t.get(&(GroupKey::Integer(1), GroupKey::Other("null".into()))),
            Some(&1)
        );
        assert_eq!(t.get(&(GroupKey::Integer(1), GroupKey::Absent)), Some(&1));
        assert_eq!(
            t.get(&(GroupKey::Integer(2), GroupKey::Integer(4))),
            Some(&1)
        );
    }

    fn suit_souls() -> Vec<SoulObservation> {
        // Bits 0 and 1 carry codes 2 and 3 in the inherited table.
        vec![
            soul("s0", vec![("suit", RawValue::Integer(300_002))]),
            soul("s1", vec![("suit", RawValue::Integer(300_003))]),
            soul("s2", vec![("suit", RawValue::Integer(300_003))]),
        ]
    }

    fn attest(identity: &str, bit: u16) -> Attestation {
        Attestation {
            identity: identity.into(),
            bit,
        }
    }

    #[test]
    fn matching_codes_reestablish_their_bits() {
        let l = suit_ledger(
            &suit_souls(),
            &Source::Container,
            &Source::Entry("suit".into()),
            300_000,
            &[attest("s0", 0), attest("s1", 1), attest("s2", 1)],
        );
        assert_eq!(l.rows.len(), usize::from(SOUL_BIT_COUNT));
        assert_eq!(l.rows[0].status, BitStatus::Reestablished);
        assert_eq!(l.rows[1].status, BitStatus::Reestablished);
        assert_eq!(l.rows[1].attestations, 2);
        assert_eq!(l.rows[2].status, BitStatus::Unattested);
        assert!(l.unjoined.is_empty());
        assert!(!l.retires_inheritance());
    }

    #[test]
    fn a_code_attested_for_two_bits_contradicts_both() {
        let l = suit_ledger(
            &suit_souls(),
            &Source::Container,
            &Source::Entry("suit".into()),
            300_000,
            // s1 carries code 3, which the inherited table gives bit 1, not bit 0.
            &[attest("s0", 0), attest("s1", 0), attest("s2", 1)],
        );
        assert_eq!(l.rows[0].status, BitStatus::Contradicted);
        assert_eq!(l.rows[0].observed, BTreeSet::from([2, 3]));
        assert_eq!(l.rows[1].status, BitStatus::Contradicted);
    }

    #[test]
    fn unjoinable_attestations_are_reported_not_dropped() {
        let mut souls = suit_souls();
        souls.push(soul("s0", vec![]));
        souls.push(soul("nosuit", vec![]));
        let l = suit_ledger(
            &souls,
            &Source::Container,
            &Source::Entry("suit".into()),
            0,
            &[
                attest("s0", 0),
                attest("missing", 0),
                attest("nosuit", 0),
                attest("s1", 999),
            ],
        );
        assert_eq!(
            l.unjoined,
            vec![
                Unjoined::Ambiguous {
                    identity: "s0".into(),
                    souls: 2
                },
                Unjoined::NoSoul {
                    identity: "missing".into()
                },
                Unjoined::NoSuitValue {
                    identity: "nosuit".into()
                },
                Unjoined::NoSuchBit {
                    identity: "s1".into(),
                    bit: 999
                },
            ]
        );
    }

    #[test]
    fn every_bit_reestablished_retires_the_inheritance() {
        let souls: Vec<SoulObservation> = (0..SOUL_BIT_COUNT)
            .map(|bit| {
                let code = soul_set(bit).expect("mapped").suit_code();
                soul(
                    &format!("s{bit}"),
                    vec![("suit", RawValue::Integer(i64::from(code)))],
                )
            })
            .collect();
        let attestations: Vec<Attestation> = (0..SOUL_BIT_COUNT)
            .map(|bit| attest(&format!("s{bit}"), bit))
            .collect();
        let l = suit_ledger(
            &souls,
            &Source::Container,
            &Source::Entry("suit".into()),
            0,
            &attestations,
        );
        assert!(l.retires_inheritance());
    }
}
