//! The analyses that turn readings into evidence about the game's layout: what the records hold,
//! how their values group, and whether the inherited suit codes survive an attested reading.
//!
//! They answer research questions, not user ones, and they never change what a reading says. A
//! mapping they support becomes established only when the maintainer records the experiment and
//! the reader states the new evidence (`probe-protocol.md`, "Evidence").

use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;

use super::observation::{
    InnateReading, RawKind, RawValue, SequenceKind, SoulField, SoulObservation,
};
use crate::scheme::edit::SoulBit;
use crate::scheme::mapping::{SOUL_BIT_COUNT, soul_set};
use crate::soul::SoulSet;

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

/// Why a source could not be read from its text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceError {
    Empty,
    /// `@name` where `name` is not a typed field or `container`.
    UnknownField(String),
}

impl Source {
    /// `@soul_id`, `@suit_code`, … for a typed field, `@container` for the container key, and
    /// anything else for the observed entry of that key.
    pub fn parse(spec: &str) -> Result<Source, SourceError> {
        match spec.strip_prefix('@') {
            _ if spec.is_empty() => Err(SourceError::Empty),
            None => Ok(Source::Entry(spec.to_owned())),
            Some("container") => Ok(Source::Container),
            Some(name) => SoulField::from_name(name)
                .map(Source::Field)
                .ok_or_else(|| SourceError::UnknownField(name.to_owned())),
        }
    }

    /// What this source finds on a soul: a field the reader does not map, nothing (a mapped
    /// field the record lacks, or an entry the record does not have), or a value. For `@innate`,
    /// `null` is a record that says the soul has no innate attribute.
    pub fn read(&self, soul: &SoulObservation) -> Found {
        let field = |f: SoulField| soul.evidence_of(f).is_none();
        match self {
            Source::Field(f) if field(*f) => Found::Unmapped,
            _ => self.value(soul).map_or(Found::Absent, Found::Value),
        }
    }

    /// The value this source names on a soul, or `None` when it holds none, mapped or not.
    pub fn value(&self, soul: &SoulObservation) -> Option<RawValue> {
        let integer = |n: u32| RawValue::Integer(i64::from(n));
        let pair = |code: u32, value: f64| RawValue::Sequence {
            kind: SequenceKind::Tuple,
            items: vec![integer(code), RawValue::Float(value)],
            full_length: None,
        };
        match self {
            Source::Field(SoulField::SoulId) => soul.soul_id.value().cloned().map(RawValue::Text),
            Source::Field(SoulField::SuitCode) => soul.suit_code.value().map(|c| integer(c.0)),
            Source::Field(SoulField::Star) => soul.star.value().map(|s| integer(s.0)),
            Source::Field(SoulField::Slot) => soul.slot.value().map(|s| integer(s.0)),
            Source::Field(SoulField::Level) => soul.level.value().map(|l| integer(l.0)),
            Source::Field(SoulField::Main) => soul.main.value().map(|a| pair(a.code.0, a.value)),
            Source::Field(SoulField::Innate) => soul.innate.value().map(|i| match i {
                InnateReading::None => RawValue::Null,
                InnateReading::Present(a) => pair(a.code.0, a.value),
            }),
            Source::Field(SoulField::Subs) => soul.subs.value().map(|subs| RawValue::Sequence {
                kind: SequenceKind::List,
                items: subs.iter().map(|s| pair(s.code.0, s.value)).collect(),
                full_length: None,
            }),
            Source::Field(SoulField::Locked) => soul.locked.value().copied().map(RawValue::Bool),
            Source::Field(SoulField::Discarded) => {
                soul.discarded.value().copied().map(RawValue::Bool)
            }
            Source::Container => soul.observed.as_ref()?.container_key.clone(),
            Source::Entry(key) => soul.observed.as_ref()?.get(key).cloned(),
        }
    }
}

/// What a source finds on one soul.
#[derive(Debug, Clone, PartialEq)]
pub enum Found {
    /// A typed field the reader does not map.
    Unmapped,
    /// Nothing: a mapped field the record lacks, or an entry the record does not have.
    Absent,
    Value(RawValue),
}

/// A group's key: unmapped sorts first, then absent, then integers by value, then everything else
/// by rendering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupKey {
    Unmapped,
    Absent,
    Integer(i64),
    Other(String),
}

impl GroupKey {
    pub fn of(found: &Found) -> GroupKey {
        match found {
            Found::Unmapped => GroupKey::Unmapped,
            Found::Absent => GroupKey::Absent,
            Found::Value(RawValue::Integer(n)) => GroupKey::Integer(*n),
            Found::Value(v) => GroupKey::Other(v.render()),
        }
    }

    pub fn render(&self) -> String {
        match self {
            GroupKey::Unmapped => "(unmapped)".to_owned(),
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

/// How many distinct values a key holds: exactly, or at least the cap when counting stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distinct {
    Exactly(usize),
    AtLeast(usize),
}

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
    pub distinct: Distinct,
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

/// The per-key state a survey accumulates.
#[derive(Default)]
struct KeyTally {
    records: u64,
    kinds: BTreeMap<RawKind, u64>,
    integer_range: Option<(i64, i64)>,
    seen: BTreeSet<String>,
    capped: bool,
    examples: Vec<String>,
}

impl KeyTally {
    fn add(mut self, v: &RawValue) -> KeyTally {
        self.records += 1;
        *self.kinds.entry(v.kind()).or_default() += 1;
        if let RawValue::Integer(n) = v {
            self.integer_range = Some(match self.integer_range {
                None => (*n, *n),
                Some((lo, hi)) => (lo.min(*n), hi.max(*n)),
            });
        }
        let r = v.render();
        if !self.seen.contains(&r) {
            if self.seen.len() < DISTINCT_CAP {
                if self.examples.len() < EXAMPLES {
                    self.examples.push(r.clone());
                }
                self.seen.insert(r);
            } else {
                self.capped = true;
            }
        }
        self
    }

    fn finish(self, key: String) -> KeySurvey {
        KeySurvey {
            key,
            records: self.records,
            kinds: self.kinds,
            integer_range: self.integer_range,
            distinct: if self.capped {
                Distinct::AtLeast(DISTINCT_CAP)
            } else {
                Distinct::Exactly(self.seen.len())
            },
            examples: self.examples,
        }
    }
}

/// Survey every observed record of a set of souls.
pub fn survey<'a>(souls: impl IntoIterator<Item = &'a SoulObservation>) -> Survey {
    let mut s = Survey::default();
    let mut keys: BTreeMap<String, KeyTally> = BTreeMap::new();
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
            let tally = keys.remove(&k.render()).unwrap_or_default().add(v);
            keys.insert(k.render(), tally);
        }
    }
    s.keys = keys
        .into_iter()
        .map(|(key, tally)| tally.finish(key))
        .collect();
    s
}

/// Souls grouped by the value a source names, in [`GroupKey`] order; each group keeps its souls
/// in reading order.
pub fn group<'a>(
    souls: impl IntoIterator<Item = &'a SoulObservation>,
    by: &Source,
) -> BTreeMap<GroupKey, Vec<&'a SoulObservation>> {
    souls.into_iter().fold(BTreeMap::new(), |mut groups, soul| {
        groups
            .entry(GroupKey::of(&by.read(soul)))
            .or_insert_with(Vec::new)
            .push(soul);
        groups
    })
}

/// How many souls hold each pair of values: rows by one source, columns by another. Unmapped,
/// absent, `null`, and each value are separate columns, which is what a presence question needs.
pub fn crosstab<'a>(
    souls: impl IntoIterator<Item = &'a SoulObservation>,
    rows: &Source,
    columns: &Source,
) -> BTreeMap<(GroupKey, GroupKey), u64> {
    souls.into_iter().fold(BTreeMap::new(), |mut table, soul| {
        let r = GroupKey::of(&rows.read(soul));
        let c = GroupKey::of(&columns.read(soul));
        *table.entry((r, c)).or_insert(0) += 1;
        table
    })
}

/// A maintainer's statement that the soul with this identity is shown in the game as the soul
/// scheme bit `bit` selects. The bit ↔ soul link is the scheme research's (`scheme-code.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attestation {
    pub identity: String,
    pub bit: SoulBit,
}

/// What the attested souls of one bit say about its inherited code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Every attested soul carries the inherited code, and no soul of another bit carries it.
    Reestablished,
    /// An attested soul carries another code, or the code is attested for two bits.
    Contradicted,
}

/// Where the suit-code inheritance stands for one scheme bit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitStatus {
    /// No attestation names this bit.
    Unattested,
    Attested {
        /// The codes read from the attested souls, after the offset: never empty.
        observed: BTreeSet<i64>,
        attestations: NonZeroUsize,
        outcome: Outcome,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitRow {
    pub bit: SoulBit,
    /// The set the prior tool gives the soul of this bit, by its suit code.
    pub inherited: SoulSet,
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
    /// The suit value minus the offset is outside the integers.
    SuitOutOfRange { identity: String, value: i64 },
}

/// A soul bit the scheme table does not map to a set: the table has a gap, and no ledger is made
/// that would leave the bit out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableGap {
    pub bit: u16,
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
            && self.rows.iter().all(|r| {
                matches!(
                    r.status,
                    BitStatus::Attested {
                        outcome: Outcome::Reestablished,
                        ..
                    }
                )
            })
    }
}

/// Join attestations to a reading. `identity` names each soul the way the attestations do (a
/// text key by its text, an integer key by its value); `suit` is where its suit value is read,
/// and `offset` is subtracted from that value before it is compared with a suit code.
pub fn suit_ledger(
    souls: &[SoulObservation],
    identity: &Source,
    suit: &Source,
    offset: i64,
    attestations: &[Attestation],
) -> Result<SuitLedger, TableGap> {
    let join = |a: &Attestation| -> Result<(SoulBit, i64), Unjoined> {
        let id = || a.identity.clone();
        let matching: Vec<&SoulObservation> = souls
            .iter()
            .filter(|s| {
                identity
                    .value(s)
                    .is_some_and(|v| v.matches_key(&a.identity))
            })
            .collect();
        let soul = match matching.as_slice() {
            [] => return Err(Unjoined::NoSoul { identity: id() }),
            [one] => *one,
            many => {
                return Err(Unjoined::Ambiguous {
                    identity: id(),
                    souls: many.len(),
                });
            }
        };
        let Some(RawValue::Integer(v)) = suit.value(soul) else {
            return Err(Unjoined::NoSuitValue { identity: id() });
        };
        let code = v.checked_sub(offset).ok_or(Unjoined::SuitOutOfRange {
            identity: id(),
            value: v,
        })?;
        Ok((a.bit, code))
    };
    // Each attested bit's codes and attestation count; a count starts at one, so it is never 0.
    let (mut observed, unjoined) = attestations.iter().map(join).fold(
        (
            BTreeMap::<SoulBit, (BTreeSet<i64>, NonZeroUsize)>::new(),
            Vec::new(),
        ),
        |(mut observed, mut unjoined), joined| {
            match joined {
                Ok((bit, code)) => {
                    observed
                        .entry(bit)
                        .and_modify(|(codes, n)| {
                            codes.insert(code);
                            *n = n.saturating_add(1);
                        })
                        .or_insert_with(|| (BTreeSet::from([code]), NonZeroUsize::MIN));
                }
                Err(u) => unjoined.push(u),
            }
            (observed, unjoined)
        },
    );
    let bits_of_code: BTreeMap<i64, BTreeSet<SoulBit>> =
        observed
            .iter()
            .fold(BTreeMap::new(), |mut by_code, (bit, (codes, _))| {
                for c in codes {
                    by_code.entry(*c).or_insert_with(BTreeSet::new).insert(*bit);
                }
                by_code
            });
    let rows = (0..SOUL_BIT_COUNT)
        .map(|index| {
            let gap = TableGap { bit: index };
            let bit = SoulBit::new(index).ok_or(gap)?;
            let inherited = soul_set(index).ok_or(gap)?;
            let status = match observed.remove(&bit) {
                None => BitStatus::Unattested,
                Some((codes, attestations)) => {
                    let code = i64::from(inherited.suit_code());
                    let alone = bits_of_code.get(&code).is_some_and(|b| b.len() == 1);
                    let outcome = if codes.len() == 1 && codes.contains(&code) && alone {
                        Outcome::Reestablished
                    } else {
                        Outcome::Contradicted
                    };
                    BitStatus::Attested {
                        observed: codes,
                        attestations,
                        outcome,
                    }
                }
            };
            Ok(BitRow {
                bit,
                inherited,
                status,
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(SuitLedger { rows, unjoined })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::observation::{
        Coverage, ObservedRecord, RawSoul, SoulMappings, SoulReading,
    };

    fn raw(id: &str, entries: Vec<(&str, RawValue)>) -> RawSoul {
        RawSoul {
            observed: Some(ObservedRecord {
                type_name: "record".into(),
                container_key: Some(RawValue::Text(id.into())),
                entries: entries
                    .into_iter()
                    .map(|(k, v)| (RawValue::Text(k.into()), v))
                    .collect(),
            }),
            ..RawSoul::default()
        }
    }

    fn souls(raws: Vec<RawSoul>) -> Vec<SoulObservation> {
        SoulReading::new(Coverage::Partial, None, None, SoulMappings::default(), raws)
            .expect("consistent")
            .souls()
            .to_vec()
    }

    fn bit(n: u16) -> SoulBit {
        SoulBit::new(n).expect("mapped")
    }

    #[test]
    fn sources_parse_fields_the_container_and_entries() {
        assert_eq!(
            Source::parse("@suit_code"),
            Ok(Source::Field(SoulField::SuitCode))
        );
        assert_eq!(Source::parse("@container"), Ok(Source::Container));
        assert_eq!(Source::parse("x"), Ok(Source::Entry("x".into())));
        assert_eq!(
            Source::parse("@nothing"),
            Err(SourceError::UnknownField("nothing".into()))
        );
        assert_eq!(Source::parse(""), Err(SourceError::Empty));
    }

    #[test]
    fn a_survey_counts_kinds_ranges_and_distinct_values() {
        let mut raws = vec![
            raw(
                "a",
                vec![("n", RawValue::Integer(5)), ("t", RawValue::Null)],
            ),
            raw("b", vec![("n", RawValue::Integer(-2))]),
            raw(
                "c",
                vec![("n", RawValue::Integer(5)), ("t", RawValue::Integer(9))],
            ),
        ];
        raws.push(RawSoul::default());
        let s = survey(&souls(raws));
        assert_eq!((s.souls, s.observed), (4, 3));
        assert_eq!(s.container_kinds.get(&RawKind::Text), Some(&3));
        let n = &s.keys[0];
        assert_eq!(n.key, "\"n\"");
        assert_eq!(n.records, 3);
        assert_eq!(n.integer_range, Some((-2, 5)));
        assert_eq!(n.distinct, Distinct::Exactly(2));
        assert_eq!(n.examples, vec!["5", "-2"]);
        let t = &s.keys[1];
        assert_eq!(t.kinds.get(&RawKind::Null), Some(&1));
        assert_eq!(t.kinds.get(&RawKind::Integer), Some(&1));
    }

    #[test]
    fn a_survey_says_when_it_stopped_counting() {
        let raws: Vec<RawSoul> = (0..=DISTINCT_CAP as i64)
            .map(|i| raw("x", vec![("n", RawValue::Integer(i))]))
            .collect();
        let s = survey(&souls(raws));
        assert_eq!(s.keys[0].distinct, Distinct::AtLeast(DISTINCT_CAP));
    }

    #[test]
    fn groups_order_absent_then_integers_numerically() {
        let all = souls(vec![
            raw("a", vec![("g", RawValue::Integer(10))]),
            raw("b", vec![("g", RawValue::Integer(9))]),
            raw("c", vec![]),
            raw("d", vec![("g", RawValue::Integer(10))]),
        ]);
        let g = group(&all, &Source::Entry("g".into()));
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
        let all = souls(vec![
            raw(
                "a",
                vec![("s", RawValue::Integer(1)), ("i", RawValue::Null)],
            ),
            raw("b", vec![("s", RawValue::Integer(1))]),
            raw(
                "c",
                vec![("s", RawValue::Integer(2)), ("i", RawValue::Integer(4))],
            ),
        ]);
        let t = crosstab(&all, &Source::Entry("s".into()), &Source::Entry("i".into()));
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
        souls(vec![
            raw("s0", vec![("suit", RawValue::Integer(300_002))]),
            raw("s1", vec![("suit", RawValue::Integer(300_003))]),
            raw("s2", vec![("suit", RawValue::Integer(300_003))]),
        ])
    }

    fn attest(identity: &str, n: u16) -> Attestation {
        Attestation {
            identity: identity.into(),
            bit: bit(n),
        }
    }

    fn outcome(row: &BitRow) -> Option<Outcome> {
        match &row.status {
            BitStatus::Unattested => None,
            BitStatus::Attested { outcome, .. } => Some(*outcome),
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
        )
        .expect("the table has no gap");
        assert_eq!(l.rows.len(), usize::from(SOUL_BIT_COUNT));
        assert_eq!(outcome(&l.rows[0]), Some(Outcome::Reestablished));
        assert_eq!(outcome(&l.rows[1]), Some(Outcome::Reestablished));
        assert!(matches!(
            l.rows[1].status,
            BitStatus::Attested { attestations, .. } if attestations.get() == 2
        ));
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
        )
        .expect("the table has no gap");
        assert_eq!(outcome(&l.rows[0]), Some(Outcome::Contradicted));
        assert!(matches!(
            &l.rows[0].status,
            BitStatus::Attested { observed, .. } if *observed == BTreeSet::from([2, 3])
        ));
        assert_eq!(outcome(&l.rows[1]), Some(Outcome::Contradicted));
    }

    #[test]
    fn unjoinable_attestations_are_reported_not_dropped() {
        let mut all = suit_souls();
        all.extend(souls(vec![
            raw("s0", vec![]),
            raw("nosuit", vec![]),
            raw("low", vec![("suit", RawValue::Integer(i64::MIN))]),
        ]));
        let l = suit_ledger(
            &all,
            &Source::Container,
            &Source::Entry("suit".into()),
            1,
            &[
                attest("s0", 0),
                attest("missing", 0),
                attest("nosuit", 0),
                attest("low", 0),
            ],
        )
        .expect("the table has no gap");
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
                Unjoined::SuitOutOfRange {
                    identity: "low".into(),
                    value: i64::MIN
                },
            ]
        );
    }

    #[test]
    fn every_bit_reestablished_retires_the_inheritance() {
        let all = souls(
            (0..SOUL_BIT_COUNT)
                .map(|b| {
                    let code = soul_set(b).expect("mapped").suit_code();
                    raw(
                        &format!("s{b}"),
                        vec![("suit", RawValue::Integer(i64::from(code)))],
                    )
                })
                .collect(),
        );
        let attestations: Vec<Attestation> = (0..SOUL_BIT_COUNT)
            .map(|b| attest(&format!("s{b}"), b))
            .collect();
        let l = suit_ledger(
            &all,
            &Source::Container,
            &Source::Entry("suit".into()),
            0,
            &attestations,
        )
        .expect("the table has no gap");
        assert!(l.retires_inheritance());
    }
}
