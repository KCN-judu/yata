//! `evaluate(query, soul)`: a checked filter over one soul, by ADR-0026's three-valued rule.
//!
//! A predicate reads a field and is decided. A selection or a scheme asks the scheme evaluator and
//! keeps its verdict. `And`, `Or` and `Not` are strong Kleene logic: a decided operand settles the
//! node when it can, and an open result carries the union of the open rules beneath it.

use std::collections::BTreeSet;
use std::ops::RangeInclusive;

use crate::mechanics::VALUE_TOLERANCE;
use crate::scheme::code::{DiscardScheme, StrengtheningPlan};
use crate::scheme::evaluate::{OpenRule, Verdict, matches};
use crate::scheme::selection::SoulSelection;
use crate::soul::{Soul, SoulAttribute, SoulSet, SoulSlot};

/// A filter [`super::compile`] accepted. Each variant is total over every soul.
#[derive(Debug, Clone)]
pub(super) enum Cond {
    All(Vec<Cond>),
    Any(Vec<Cond>),
    Not(Box<Cond>),
    Sets(BTreeSet<SoulSet>),
    Slots(BTreeSet<SoulSlot>),
    MainAttributes(BTreeSet<SoulAttribute>),
    Int(IntField, RangeInclusive<i64>),
    Number(NumberField, Option<f64>, Option<f64>),
    Is(BoolField, bool),
    Selection(SoulSelection),
    Scheme(SchemeEntry),
}

#[derive(Debug, Clone, Copy)]
pub(super) enum IntField {
    Star,
    Level,
    SubCount,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum NumberField {
    MainValue,
    SubValue(SoulAttribute),
}

#[derive(Debug, Clone, Copy)]
pub(super) enum BoolField {
    HasSub(SoulAttribute),
    Pristine,
}

/// The one plan or discard scheme a scheme reference named.
#[derive(Debug, Clone)]
pub(super) enum SchemeEntry {
    Plan(StrengtheningPlan),
    Discard(DiscardScheme),
}

/// The outcome of a filter over one soul: a [`Verdict`] with its open rules as a set, so that
/// combining outcomes allocates nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Outcome {
    Yes,
    No,
    Open(OpenRules),
}

/// A set of [`OpenRule`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct OpenRules(u8);

impl OpenRules {
    fn bit(rule: OpenRule) -> u8 {
        match rule {
            OpenRule::Innate => 1,
            OpenRule::UnknownConditions => 2,
        }
    }

    pub(super) fn of(rules: &[OpenRule]) -> OpenRules {
        OpenRules(rules.iter().fold(0, |m, &r| m | OpenRules::bit(r)))
    }

    fn union(self, other: OpenRules) -> OpenRules {
        OpenRules(self.0 | other.0)
    }

    /// The rules, in [`OpenRule`] order.
    fn rules(self) -> Vec<OpenRule> {
        [OpenRule::Innate, OpenRule::UnknownConditions]
            .into_iter()
            .filter(|&r| self.0 & OpenRules::bit(r) != 0)
            .collect()
    }
}

impl Outcome {
    pub(super) fn of(verdict: &Verdict) -> Outcome {
        match verdict {
            Verdict::Matches => Outcome::Yes,
            Verdict::DoesNotMatch => Outcome::No,
            Verdict::Undetermined(rules) => Outcome::Open(OpenRules::of(rules)),
        }
    }

    fn decided(b: bool) -> Outcome {
        if b { Outcome::Yes } else { Outcome::No }
    }

    pub(super) fn verdict(self) -> Verdict {
        match self {
            Outcome::Yes => Verdict::Matches,
            Outcome::No => Verdict::DoesNotMatch,
            Outcome::Open(rules) => Verdict::Undetermined(rules.rules()),
        }
    }

    pub(super) fn and(self, other: Outcome) -> Outcome {
        match (self, other) {
            (Outcome::No, _) | (_, Outcome::No) => Outcome::No,
            (Outcome::Open(a), Outcome::Open(b)) => Outcome::Open(a.union(b)),
            (Outcome::Open(a), Outcome::Yes) | (Outcome::Yes, Outcome::Open(a)) => Outcome::Open(a),
            (Outcome::Yes, Outcome::Yes) => Outcome::Yes,
        }
    }

    pub(super) fn or(self, other: Outcome) -> Outcome {
        self.not().and(other.not()).not()
    }

    pub(super) fn not(self) -> Outcome {
        match self {
            Outcome::Yes => Outcome::No,
            Outcome::No => Outcome::Yes,
            open => open,
        }
    }
}

impl Cond {
    pub(super) fn outcome(&self, soul: &Soul) -> Outcome {
        match self {
            // Folds that stop at the first operand that settles the node.
            Cond::All(cs) => cs
                .iter()
                .try_fold(Outcome::Yes, |acc, c| match acc.and(c.outcome(soul)) {
                    Outcome::No => Err(Outcome::No),
                    o => Ok(o),
                })
                .unwrap_or_else(|settled| settled),
            Cond::Any(cs) => cs
                .iter()
                .try_fold(Outcome::No, |acc, c| match acc.or(c.outcome(soul)) {
                    Outcome::Yes => Err(Outcome::Yes),
                    o => Ok(o),
                })
                .unwrap_or_else(|settled| settled),
            Cond::Not(c) => c.outcome(soul).not(),
            Cond::Sets(sets) => Outcome::decided(sets.contains(&soul.set)),
            Cond::Slots(slots) => Outcome::decided(slots.contains(&soul.slot)),
            Cond::MainAttributes(attributes) => Outcome::decided(attributes.contains(&soul.main)),
            Cond::Int(field, range) => Outcome::decided(range.contains(&field.value(soul))),
            Cond::Number(field, min, max) => {
                let v = field.value(soul);
                // Stored values compare with the domain's tolerance, as 真 n does (ADR-0026, 3).
                Outcome::decided(
                    min.is_none_or(|lo| v + VALUE_TOLERANCE >= lo)
                        && max.is_none_or(|hi| v - VALUE_TOLERANCE <= hi),
                )
            }
            Cond::Is(field, b) => Outcome::decided(field.value(soul) == *b),
            Cond::Selection(selection) => Outcome::of(&matches(selection, soul)),
            Cond::Scheme(SchemeEntry::Plan(plan)) => Outcome::of(&plan.matches(soul)),
            Cond::Scheme(SchemeEntry::Discard(scheme)) => Outcome::of(&scheme.matches(soul)),
        }
    }
}

impl IntField {
    pub(super) fn value(self, soul: &Soul) -> i64 {
        match self {
            IntField::Star => i64::from(soul.star),
            IntField::Level => i64::from(soul.level),
            // A soul holds at most a handful of sub-attributes; the count always fits.
            IntField::SubCount => i64::try_from(soul.subs.len()).unwrap_or(i64::MAX),
        }
    }
}

impl NumberField {
    pub(super) fn value(self, soul: &Soul) -> f64 {
        match self {
            NumberField::MainValue => soul.main_value,
            // No nulls (`query.md`): a sub-attribute the soul lacks has value 0.
            NumberField::SubValue(a) => soul.sub(a).map_or(0.0, |s| s.value),
        }
    }
}

impl BoolField {
    pub(super) fn value(self, soul: &Soul) -> bool {
        match self {
            BoolField::HasSub(a) => soul.sub(a).is_some(),
            // At +0 no roll has happened, so the sub-attributes held are the initial ones.
            BoolField::Pristine => soul.level == 0 && soul.subs.len() == 4,
        }
    }
}
