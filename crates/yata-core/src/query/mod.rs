//! Typed queries over souls: the vocabulary of `query.md`, evaluated as ADR-0026 decides.
//!
//! A query is a closed expression tree, never text (ADR-0002). It is checked once and then run:
//!
//! - [`vocabulary`]: the tree as the wire carries it — a field beside a test, a scheme by its code.
//! - [`compile`]: the one check of a tree against the vocabulary and its limits. It resolves each
//!   scheme reference with the scheme codec, and refuses what this build cannot evaluate.
//! - [`CompiledQuery::verdict`]: `evaluate(query, soul)`. Predicates are decided; a scheme gives
//!   the verdict of [`crate::scheme::evaluate::matches`], and the tree combines verdicts by strong
//!   Kleene logic, so an open verdict stays open and names its open rules.
//! - [`CompiledQuery::page`]: the rows a filter keeps, ordered by the sort keys and then the row
//!   identity, one page at a time.
//!
//! Nothing here reads a projection or a store: the caller supplies the souls. Scores are not
//! evaluated here until pass 1 exists; a score field is refused (ADR-0026, rule 4).

mod compile;
mod eval;
mod page;
pub mod vocabulary;

#[cfg(test)]
mod tests;

pub use crate::scheme::evaluate::OpenRules;
pub use compile::{CompiledQuery, compile};
pub use page::RowVerdict;
pub use page::{Cursor, Page, PageRequest, Row, SortValue};
pub use vocabulary::{
    Bound, Direction, EnumValue, Expr, Field, FieldType, MAX_EXPR_DEPTH, MAX_EXPR_NODES,
    MAX_SORT_KEYS, MAX_TEST_VALUES, ParamSetId, ParamSetRef, QualityComponent, SchemeCodeText,
    SchemeRef, SortKey, SoulQuery, Test,
};

use crate::scheme::code::CodeError;
use crate::scheme::layout::LayoutError;
use crate::scheme::transport::TransportError;

/// Why a query is refused. Each variant is one `query.*` code of `query.md`, "Errors", except
/// `query.unknown_field`, which only a wire value can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum QueryError {
    /// `query.type_mismatch`: a test that does not fit its field's type.
    TypeMismatch { field: Field, test: TestKind },
    /// `query.type_mismatch`: a sort key over a field that is not sortable.
    NotSortable { field: Field },
    /// `query.param_set_required`: a score field in a query without a parameter set.
    ParamSetRequired { field: Field },
    /// `query.field_unavailable`: a field of the vocabulary this build cannot evaluate yet.
    FieldUnavailable { field: Field },
    /// `query.unknown_scheme`: a scheme reference that names no plan or scheme.
    UnknownScheme(SchemeProblem),
    /// `query.too_complex`: a limit of `query.md` exceeded. `actual` is the count reached when the
    /// check stopped, which for nodes and depth is one past the limit.
    TooComplex { limit: Limit, actual: usize },
    /// `query.malformed`: a tree the wire can carry and the vocabulary cannot mean.
    Malformed(Malformation),
    /// `query.malformed_cursor`: a cursor that does not continue this query's order.
    MalformedCursor,
}

/// The kind of a [`Test`], named in a type mismatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestKind {
    In,
    IntRange,
    NumberRange,
    Is,
}

/// A limit of `query.md`, "Limits".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limit {
    ExprNodes,
    ExprDepth,
    SortKeys,
    TestValues,
}

impl Limit {
    pub fn max(self) -> usize {
        match self {
            Limit::ExprNodes => MAX_EXPR_NODES,
            Limit::ExprDepth => MAX_EXPR_DEPTH,
            Limit::SortKeys => MAX_SORT_KEYS,
            Limit::TestValues => MAX_TEST_VALUES,
        }
    }
}

/// What makes a tree malformed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Malformation {
    /// `In []`: always a client bug, so refused rather than read as false.
    EmptyIn,
    /// A range whose minimum is above its maximum.
    RangeInverted,
    /// A number bound that is NaN or infinite.
    NonFiniteBound,
}

/// Why a scheme reference names no plan or scheme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemeProblem {
    Transport(TransportError),
    Layout(LayoutError),
    Code(CodeError),
    /// The code has `entries` plans or schemes, and the reference names none of them.
    EntryRequired {
        entries: usize,
    },
    NoSuchEntry {
        entry: usize,
        entries: usize,
    },
    /// The code holds no plan and no scheme.
    NoEntries,
}
