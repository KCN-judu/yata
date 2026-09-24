//! The one check of a query against the vocabulary: `query.md`'s type table, its limits, and
//! ADR-0026's refusals. What it accepts is a tree evaluation cannot fail on.
//!
//! The walk counts nodes and depth as it descends and stops at the first limit, so a runaway tree
//! costs at most one node past the limit to refuse.

use std::collections::BTreeSet;

use super::eval::{BoolField, Cond, IntField, NumberField, SchemeEntry};
use super::page::SortField;
use super::vocabulary::{EnumValue, Expr, Field, SchemeRef, SortKey, SoulQuery, Test};
use super::{Direction, Limit, Malformation, QueryError, SchemeProblem};
use crate::scheme::code::{SchemeCode, decode_code};
use crate::scheme::layout::parse;
use crate::scheme::transport::decode_text;

/// A query that [`compile`] accepted: its filter and its order, ready to run.
#[derive(Debug, Clone)]
pub struct CompiledQuery {
    pub(super) filter: Option<Cond>,
    pub(super) order: Vec<(SortField, Direction)>,
}

/// The query, if it means something this build can evaluate; otherwise the first reason it does
/// not.
pub fn compile(query: SoulQuery) -> Result<CompiledQuery, QueryError> {
    let has_params = query.params.is_some();
    if query.sort.len() > Limit::SortKeys.max() {
        return Err(QueryError::TooComplex {
            limit: Limit::SortKeys,
            actual: query.sort.len(),
        });
    }
    let order = query
        .sort
        .into_iter()
        .map(|k| sort_key(k, has_params))
        .collect::<Result<_, _>>()?;
    let mut nodes = 0;
    let filter = query
        .filter
        .map(|e| cond(e, 1, &mut nodes, has_params))
        .transpose()?;
    Ok(CompiledQuery { filter, order })
}

fn cond(expr: Expr, depth: usize, nodes: &mut usize, has_params: bool) -> Result<Cond, QueryError> {
    *nodes += 1;
    if *nodes > Limit::ExprNodes.max() {
        return Err(QueryError::TooComplex {
            limit: Limit::ExprNodes,
            actual: *nodes,
        });
    }
    if depth > Limit::ExprDepth.max() {
        return Err(QueryError::TooComplex {
            limit: Limit::ExprDepth,
            actual: depth,
        });
    }
    let mut below = |es: Vec<Expr>| {
        es.into_iter()
            .map(|e| cond(e, depth + 1, nodes, has_params))
            .collect::<Result<Vec<_>, _>>()
    };
    match expr {
        Expr::And(es) => below(es).map(Cond::All),
        Expr::Or(es) => below(es).map(Cond::Any),
        Expr::Not(e) => below(vec![*e]).map(|mut v| Cond::Not(Box::new(v.remove(0)))),
        Expr::Pred(field, test) => pred(field, test, has_params),
        Expr::Matches(selection) => Ok(Cond::Selection(selection)),
        Expr::MatchesScheme(r) => resolve(r)
            .map(Cond::Scheme)
            .map_err(QueryError::UnknownScheme),
    }
}

/// A field this build cannot evaluate: the scores, until pass 1 exists (ADR-0026, rule 4).
fn available(field: Field, has_params: bool) -> Result<(), QueryError> {
    match field {
        Field::Quality(_) if !has_params => Err(QueryError::ParamSetRequired { field }),
        Field::Quality(_) => Err(QueryError::FieldUnavailable { field }),
        _ => Ok(()),
    }
}

fn pred(field: Field, test: Test, has_params: bool) -> Result<Cond, QueryError> {
    available(field, has_params)?;
    let mismatch = QueryError::TypeMismatch {
        field,
        test: test.kind(),
    };
    let set = |v| match v {
        EnumValue::Set(s) => Some(s),
        _ => None,
    };
    let slot = |v| match v {
        EnumValue::Slot(s) => Some(s),
        _ => None,
    };
    let attribute = |v| match v {
        EnumValue::Attribute(a) => Some(a),
        _ => None,
    };
    match (field, test) {
        (Field::Set, Test::In(vs)) => values(vs, set, mismatch).map(Cond::Sets),
        (Field::Slot, Test::In(vs)) => values(vs, slot, mismatch).map(Cond::Slots),
        (Field::MainAttribute, Test::In(vs)) => {
            values(vs, attribute, mismatch).map(Cond::MainAttributes)
        }
        (Field::Star, Test::IntRange { min, max }) => int(IntField::Star, min, max),
        (Field::Level, Test::IntRange { min, max }) => int(IntField::Level, min, max),
        (Field::SubCount, Test::IntRange { min, max }) => int(IntField::SubCount, min, max),
        (Field::MainValue, Test::NumberRange { min, max }) => {
            number(NumberField::MainValue, min, max)
        }
        (Field::SubValue(a), Test::NumberRange { min, max }) => {
            number(NumberField::SubValue(a), min, max)
        }
        (Field::HasSub(a), Test::Is(b)) => Ok(Cond::Is(BoolField::HasSub(a), b)),
        (Field::Pristine, Test::Is(b)) => Ok(Cond::Is(BoolField::Pristine, b)),
        _ => Err(mismatch),
    }
}

/// The values of an `In`, each of the field's type: never empty, at most [`Limit::TestValues`].
fn values<T: Ord>(
    vs: Vec<EnumValue>,
    of_type: impl Fn(EnumValue) -> Option<T>,
    mismatch: QueryError,
) -> Result<BTreeSet<T>, QueryError> {
    if vs.is_empty() {
        return Err(QueryError::Malformed(Malformation::EmptyIn));
    }
    if vs.len() > Limit::TestValues.max() {
        return Err(QueryError::TooComplex {
            limit: Limit::TestValues,
            actual: vs.len(),
        });
    }
    vs.into_iter()
        .map(of_type)
        .collect::<Option<_>>()
        .ok_or(mismatch)
}

fn int(field: IntField, min: Option<i64>, max: Option<i64>) -> Result<Cond, QueryError> {
    match (min, max) {
        (None, None) => Err(QueryError::Malformed(Malformation::RangeWithoutBound)),
        (Some(lo), Some(hi)) if lo > hi => Err(QueryError::Malformed(Malformation::RangeInverted)),
        _ => Ok(Cond::Int(
            field,
            min.unwrap_or(i64::MIN)..=max.unwrap_or(i64::MAX),
        )),
    }
}

fn number(field: NumberField, min: Option<f64>, max: Option<f64>) -> Result<Cond, QueryError> {
    let finite = [min, max].into_iter().flatten().all(f64::is_finite);
    match (min, max) {
        (None, None) => Err(QueryError::Malformed(Malformation::RangeWithoutBound)),
        _ if !finite => Err(QueryError::Malformed(Malformation::NonFiniteBound)),
        (Some(lo), Some(hi)) if lo > hi => Err(QueryError::Malformed(Malformation::RangeInverted)),
        _ => Ok(Cond::Number(field, min, max)),
    }
}

fn sort_key(key: SortKey, has_params: bool) -> Result<(SortField, Direction), QueryError> {
    available(key.field, has_params)?;
    let field = match key.field {
        Field::Set => SortField::Set,
        Field::Slot => SortField::Slot,
        Field::Star => SortField::Star,
        Field::Level => SortField::Level,
        Field::MainAttribute => SortField::MainAttribute,
        Field::MainValue => SortField::MainValue,
        Field::SubValue(a) => SortField::SubValue(a),
        Field::SubCount => SortField::SubCount,
        Field::Pristine => SortField::Pristine,
        // Sorting by it is a filter in disguise (`query.md`, "Sort"); scores are refused above.
        field @ (Field::HasSub(_) | Field::Quality(_)) => {
            return Err(QueryError::NotSortable { field });
        }
    };
    Ok((field, key.direction))
}

/// The plan or discard scheme a reference names, decoded by the scheme codec: the query never
/// reads a scheme's conditions itself (`query.md`, `MatchesScheme`).
fn resolve(r: SchemeRef) -> Result<SchemeEntry, SchemeProblem> {
    let payload = decode_text(&r.code).map_err(SchemeProblem::Transport)?;
    let layout = parse(&payload).map_err(SchemeProblem::Layout)?;
    let mut entries: Vec<SchemeEntry> = match decode_code(&layout).map_err(SchemeProblem::Code)? {
        SchemeCode::Strengthening(set) => set.plans.into_iter().map(SchemeEntry::Plan).collect(),
        SchemeCode::Discard(schemes) => schemes.into_iter().map(SchemeEntry::Discard).collect(),
    };
    let n = entries.len();
    let entry = match r.entry {
        Some(i) => i,
        None if n == 1 => 0,
        None if n > 1 => return Err(SchemeProblem::EntryRequired { entries: n }),
        None => {
            return Err(SchemeProblem::NoSuchEntry {
                entry: 0,
                entries: 0,
            });
        }
    };
    if entry >= n {
        return Err(SchemeProblem::NoSuchEntry { entry, entries: n });
    }
    Ok(entries.swap_remove(entry))
}
