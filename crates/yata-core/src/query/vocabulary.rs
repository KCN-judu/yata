//! The query vocabulary of `query.md` as the wire carries it: a field and a test, side by side.
//!
//! These types can hold a tree the vocabulary cannot mean — a test that does not fit its field, an
//! empty `In`, a tree past the limits — because the wire can. [`super::compile`] is the one place
//! that decides whether a tree means something, and evaluation runs only what it accepted.

use crate::scheme::selection::SoulSelection;
use crate::soul::{SoulAttribute, SoulSet, SoulSlot};

/// Expression nodes in one filter (`query.md`, "Limits").
pub const MAX_EXPR_NODES: usize = 256;
/// Nesting depth of one filter; the root is at depth 1.
pub const MAX_EXPR_DEPTH: usize = 16;
/// Sort keys in one query, before the row identity the evaluator appends.
pub const MAX_SORT_KEYS: usize = 8;
/// Values in one `In` test, and in each group of an inline selection: as many as a suit code has.
pub const MAX_TEST_VALUES: usize = 256;

/// A query over the `Souls` collection: which rows, in which order.
#[derive(Debug, Clone, PartialEq)]
pub struct SoulQuery {
    /// Absent: every row.
    pub filter: Option<Expr>,
    /// Absent: row identity alone.
    pub sort: Vec<SortKey>,
    /// Required by any score field (`query.param_set_required`).
    pub params: Option<ParamSetRef>,
}

/// A filter expression (`query.md`, "Filter expressions").
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Empty: true.
    And(Vec<Expr>),
    /// Empty: false.
    Or(Vec<Expr>),
    Not(Box<Expr>),
    Pred(Field, Test),
    /// The official filter, inline.
    Matches(SoulSelection),
    /// A scheme, evaluated by the one scheme evaluator.
    MatchesScheme(SchemeRef),
}

/// A field of a soul row. Only fields this build can name are here; `query.md` lists the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Field {
    Set,
    Slot,
    Star,
    Level,
    MainAttribute,
    MainValue,
    /// The stored value of a sub-attribute; 0 when the soul does not have it.
    SubValue(SoulAttribute),
    HasSub(SoulAttribute),
    SubCount,
    /// `PristineSoul` (glossary): at +0 with four sub-attributes.
    Pristine,
    /// A pass-1 score component; needs a parameter set.
    Quality(QualityComponent),
}

/// The components of a quality score shown with it (`quality-model.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityComponent {
    Total,
    Depth,
    Breadth,
}

/// The type of a field, which fixes the tests it takes (`query.md`, "Filter expressions").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldType {
    SoulSet,
    SoulSlot,
    SoulAttribute,
    Int,
    Number,
    Score,
    Bool,
}

impl Field {
    pub fn field_type(self) -> FieldType {
        match self {
            Field::Set => FieldType::SoulSet,
            Field::Slot => FieldType::SoulSlot,
            Field::MainAttribute => FieldType::SoulAttribute,
            Field::Star | Field::Level | Field::SubCount => FieldType::Int,
            Field::MainValue | Field::SubValue(_) => FieldType::Number,
            Field::Quality(_) => FieldType::Score,
            Field::HasSub(_) | Field::Pristine => FieldType::Bool,
        }
    }
}

/// A test on a field's value. Which one fits is fixed by the field's type.
#[derive(Debug, Clone, PartialEq)]
pub enum Test {
    /// For an enum field: the value is one of these. Never empty.
    In(Vec<EnumValue>),
    /// For an int field.
    IntRange(Bound<i64>),
    /// For a number or score field; every bound finite.
    NumberRange(Bound<f64>),
    Is(bool),
}

/// The bounds of a range test, inclusive. A range with no bound cannot be written; a minimum
/// above its maximum can, and is `query.malformed`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound<T> {
    AtLeast(T),
    AtMost(T),
    Between { min: T, max: T },
}

impl<T: Copy> Bound<T> {
    pub fn min(self) -> Option<T> {
        match self {
            Bound::AtLeast(min) | Bound::Between { min, .. } => Some(min),
            Bound::AtMost(_) => None,
        }
    }

    pub fn max(self) -> Option<T> {
        match self {
            Bound::AtMost(max) | Bound::Between { max, .. } => Some(max),
            Bound::AtLeast(_) => None,
        }
    }
}

/// A value of an enum field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumValue {
    Set(SoulSet),
    Slot(SoulSlot),
    Attribute(SoulAttribute),
}

/// A scheme to filter by: an inline scheme code's text, and which of its entries.
///
/// A saved scheme is named here once saved schemes exist (`fact-format.md`, `SchemeSaved`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeRef {
    /// The Base64 text a scheme QR code carries.
    pub code: SchemeCodeText,
    /// The plan or discard scheme, in code order. Required when the code has more than one
    /// (ADR-0026, rule 5).
    pub entry: Option<usize>,
}

/// A parameter set, by identity and version (`scoring.md`, "Parameter set").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamSetRef {
    pub id: ParamSetId,
    pub version: u32,
}

/// The text of a scheme code, as a scheme QR code carries it. Whether it decodes is the scheme
/// codec's to say, when the reference is compiled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeCodeText(pub String);

/// A parameter set's identity: non-empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParamSetId(String);

impl ParamSetId {
    pub fn new(id: impl Into<String>) -> Option<ParamSetId> {
        let id = id.into();
        (!id.is_empty()).then_some(ParamSetId(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SortKey {
    pub field: Field,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Asc,
    Desc,
}

impl Test {
    pub fn kind(&self) -> super::TestKind {
        match self {
            Test::In(_) => super::TestKind::In,
            Test::IntRange(_) => super::TestKind::IntRange,
            Test::NumberRange(_) => super::TestKind::NumberRange,
            Test::Is(_) => super::TestKind::Is,
        }
    }
}
