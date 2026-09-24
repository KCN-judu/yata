//! A query's rows, one page at a time (`query.md`, "Sort"; `core-protocol.md`, "Queries and
//! pages").
//!
//! The order is the sort keys, then the row identity ascending, so two rows never compare equal
//! and a page boundary is one point in the order. A cursor is that point: the key values and the
//! id of the last row returned. The next page is the rows after it.
//!
//! The row identity is any ordered id: the game's soul id in a projection, whatever the caller
//! keys a supplied inventory by.

use std::cmp::Ordering;
use std::num::NonZeroUsize;

use super::compile::CompiledQuery;
use super::eval::{BoolField, IntField, NumberField, Outcome};
use super::{Direction, QueryError};
use crate::scheme::evaluate::Verdict;
use crate::soul::{Soul, SoulAttribute, SoulSet, SoulSlot};

/// A sortable field: every field but `has_sub` and the scores (`query.md`, "Sort").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SortField {
    Set,
    Slot,
    Star,
    Level,
    MainAttribute,
    MainValue,
    SubValue(SoulAttribute),
    SubCount,
    Pristine,
}

/// One sort key's value in a row. Enums order as their domain types do: sets by suit code, slots
/// and attributes in glossary order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortValue {
    Set(SoulSet),
    Slot(SoulSlot),
    Attribute(SoulAttribute),
    Int(i64),
    Number(f64),
    Bool(bool),
}

/// The kind of a [`SortValue`], for telling a foreign cursor from one of this order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ValueKind {
    Set,
    Slot,
    Attribute,
    Int,
    Number,
    Bool,
}

/// Where a page ends: the sort-key values and the id of its last row.
#[derive(Debug, Clone, PartialEq)]
pub struct Cursor<Id> {
    pub keys: Vec<SortValue>,
    pub id: Id,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageRequest<Id> {
    pub row_budget: NonZeroUsize,
    /// Absent: the first page.
    pub cursor: Option<Cursor<Id>>,
}

/// A row the filter keeps: exact, or open with the rules it rests on (ADR-0026, rule 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row<Id> {
    pub id: Id,
    /// `Matches` or `Undetermined`; never `DoesNotMatch`.
    pub verdict: Verdict,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Page<Id> {
    pub rows: Vec<Row<Id>>,
    /// Where the next page starts; absent when this page is the last.
    pub next: Option<Cursor<Id>>,
    /// Every row the filter keeps, exact or open, across all pages: the same on every page of a
    /// scan, whatever its cursor.
    pub total: usize,
}

impl SortField {
    fn value(self, soul: &Soul) -> SortValue {
        match self {
            SortField::Set => SortValue::Set(soul.set),
            SortField::Slot => SortValue::Slot(soul.slot),
            SortField::MainAttribute => SortValue::Attribute(soul.main),
            SortField::Star => SortValue::Int(IntField::Star.value(soul)),
            SortField::Level => SortValue::Int(IntField::Level.value(soul)),
            SortField::SubCount => SortValue::Int(IntField::SubCount.value(soul)),
            SortField::MainValue => SortValue::Number(NumberField::MainValue.value(soul)),
            SortField::SubValue(a) => SortValue::Number(NumberField::SubValue(a).value(soul)),
            SortField::Pristine => SortValue::Bool(BoolField::Pristine.value(soul)),
        }
    }
}

impl SortValue {
    fn kind(self) -> ValueKind {
        match self {
            SortValue::Set(_) => ValueKind::Set,
            SortValue::Slot(_) => ValueKind::Slot,
            SortValue::Attribute(_) => ValueKind::Attribute,
            SortValue::Int(_) => ValueKind::Int,
            SortValue::Number(_) => ValueKind::Number,
            SortValue::Bool(_) => ValueKind::Bool,
        }
    }

    /// A total order within a kind; values of different kinds only meet in a foreign cursor,
    /// which is refused before any comparison.
    fn cmp(self, other: SortValue) -> Ordering {
        match (self, other) {
            (SortValue::Set(a), SortValue::Set(b)) => a.cmp(&b),
            (SortValue::Slot(a), SortValue::Slot(b)) => a.cmp(&b),
            (SortValue::Attribute(a), SortValue::Attribute(b)) => a.cmp(&b),
            (SortValue::Int(a), SortValue::Int(b)) => a.cmp(&b),
            (SortValue::Number(a), SortValue::Number(b)) => a.total_cmp(&b),
            (SortValue::Bool(a), SortValue::Bool(b)) => a.cmp(&b),
            (a, b) => a.kind().cmp(&b.kind()),
        }
    }
}

/// A row on its way to a page: its key values, its id, its outcome.
type Candidate<'a, Id> = (Vec<SortValue>, &'a Id, Outcome);

impl CompiledQuery {
    /// `evaluate(query, soul)`: whether the filter keeps `soul`, as far as the evidence decides.
    pub fn verdict(&self, soul: &Soul) -> Verdict {
        self.outcome(soul).verdict()
    }

    fn outcome(&self, soul: &Soul) -> Outcome {
        self.filter
            .as_ref()
            .map_or(Outcome::Yes, |f| f.outcome(soul))
    }

    fn keys(&self, soul: &Soul) -> Vec<SortValue> {
        self.order.iter().map(|(f, _)| f.value(soul)).collect()
    }

    /// The order of two rows: each key in its direction, then the id ascending.
    fn compare<Id: Ord>(&self, a: (&[SortValue], &Id), b: (&[SortValue], &Id)) -> Ordering {
        self.order
            .iter()
            .zip(a.0.iter().zip(b.0))
            .map(|((_, direction), (x, y))| match direction {
                Direction::Asc => x.cmp(*y),
                Direction::Desc => y.cmp(*x),
            })
            .find(|o| o.is_ne())
            .unwrap_or_else(|| a.1.cmp(b.1))
    }

    /// The page of `souls` that `request` asks for. The ids of `souls` are distinct, as in a
    /// projection; the result is the same for any order they come in.
    pub fn page<'a, Id: Ord + Clone + 'a>(
        &self,
        souls: impl IntoIterator<Item = (&'a Id, &'a Soul)>,
        request: &PageRequest<Id>,
    ) -> Result<Page<Id>, QueryError> {
        let after = match &request.cursor {
            Some(c) if !self.continues(c) => return Err(QueryError::MalformedCursor),
            c => c.as_ref(),
        };
        let mut kept: Vec<Candidate<'a, Id>> = souls
            .into_iter()
            .filter_map(|(id, soul)| match self.outcome(soul) {
                Outcome::No => None,
                o => Some((self.keys(soul), id, o)),
            })
            .collect();
        // Counted before the cursor narrows the rows, so every page of a scan reports the same.
        let total = kept.len();
        kept.retain(|(keys, id, _)| {
            after.is_none_or(|c| self.compare((keys, *id), (&c.keys, &c.id)).is_gt())
        });
        let budget = request.row_budget.get();
        let by_order =
            |a: &Candidate<'_, Id>, b: &Candidate<'_, Id>| self.compare((&a.0, a.1), (&b.0, b.1));
        let more = kept.len() > budget;
        if more {
            // Only the first `budget` rows are returned: select them, then sort only those.
            kept.select_nth_unstable_by(budget, by_order);
            kept.truncate(budget);
        }
        kept.sort_unstable_by(by_order);
        let next = kept.last().filter(|_| more).map(|(keys, id, _)| Cursor {
            keys: keys.clone(),
            id: (*id).clone(),
        });
        let rows = kept
            .into_iter()
            .map(|(_, id, o)| Row {
                id: id.clone(),
                verdict: o.verdict(),
            })
            .collect();
        Ok(Page { rows, next, total })
    }

    /// Whether a cursor is a point in this query's order: one value per key, each of its kind.
    fn continues<Id>(&self, cursor: &Cursor<Id>) -> bool {
        cursor.keys.len() == self.order.len()
            && self
                .order
                .iter()
                .zip(&cursor.keys)
                .all(|((f, _), v)| f.kind() == v.kind())
    }
}

impl SortField {
    fn kind(self) -> ValueKind {
        match self {
            SortField::Set => ValueKind::Set,
            SortField::Slot => ValueKind::Slot,
            SortField::MainAttribute => ValueKind::Attribute,
            SortField::Star | SortField::Level | SortField::SubCount => ValueKind::Int,
            SortField::MainValue | SortField::SubValue(_) => ValueKind::Number,
            SortField::Pristine => ValueKind::Bool,
        }
    }
}
