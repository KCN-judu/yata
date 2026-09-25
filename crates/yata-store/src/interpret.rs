//! Result rows to typed values: the outputs, the errors, and the readers every instruction
//! shares.

use crate::instruction::Seq;
use crate::statement::{StatementResult, Value};

/// What an opened file is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreKind {
    /// A new, empty file: initialize it.
    Empty,
    Yata,
    /// Something else. The daemon opens nothing and reports it.
    Foreign {
        application_id: i64,
        objects: i64,
    },
}

/// What SQLite's structural check reported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Check {
    /// SQLite said "ok".
    Ok,
    /// The problems it listed, at least one.
    Problems(Vec<String>),
}

/// The one projection cache entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry {
    pub seq: Seq,
    pub fold_version: u32,
    pub projection: Vec<u8>,
}

/// What a result should have been, when it was not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expected {
    /// A query's rows, not a write's change count.
    Rows,
    /// A write's change count, not rows.
    Changed,
    OneRow,
    /// At most one row.
    AtMostOneRow,
    Columns(usize),
    Integer,
    /// An integer from 0 up.
    NonNegative,
    /// A `seq`: an integer from 1 up.
    Seq,
    /// An integer that fits a `u32`.
    U32,
    Bytes,
    Text,
}

/// Why a plan's results could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    /// The commit was not appended because `seq` is not the last `seq` plus one.
    SeqNotNext { seq: Seq },
    /// The executor returned a different number of results than the plan has statements.
    ResultCount { expected: usize, actual: usize },
    /// A result did not have the shape its statement produces.
    Shape {
        instruction: &'static str,
        expected: Expected,
    },
    /// Commits read back are not dense.
    LogGap { expected: Seq, found: u64 },
    /// SQLite did not take a connection setting: what it reported, if anything.
    Setting {
        setting: &'static str,
        found: Option<String>,
    },
}

pub(crate) fn shape(instruction: &'static str, expected: Expected) -> StoreError {
    StoreError::Shape {
        instruction,
        expected,
    }
}

/// Exactly `N` results.
pub(crate) fn exactly<const N: usize>(
    results: &[StatementResult],
) -> Result<&[StatementResult; N], StoreError> {
    results.try_into().map_err(|_| StoreError::ResultCount {
        expected: N,
        actual: results.len(),
    })
}

pub(crate) fn rows<'a>(
    r: &'a StatementResult,
    name: &'static str,
) -> Result<&'a [Vec<Value>], StoreError> {
    match r {
        StatementResult::Rows(rows) => Ok(rows),
        StatementResult::Changed(_) => Err(shape(name, Expected::Rows)),
    }
}

pub(crate) fn changed(r: &StatementResult, name: &'static str) -> Result<u64, StoreError> {
    match r {
        StatementResult::Changed(n) => Ok(*n),
        StatementResult::Rows(_) => Err(shape(name, Expected::Changed)),
    }
}

/// The single value of a one-row, one-column result.
pub(crate) fn scalar<'a>(
    r: &'a StatementResult,
    name: &'static str,
) -> Result<&'a Value, StoreError> {
    match rows(r, name)? {
        [row] => match row.as_slice() {
            [v] => Ok(v),
            _ => Err(shape(name, Expected::Columns(1))),
        },
        _ => Err(shape(name, Expected::OneRow)),
    }
}

/// The single row of a zero-or-one-row result.
pub(crate) fn optional_row<'a>(
    r: &'a StatementResult,
    name: &'static str,
) -> Result<Option<&'a [Value]>, StoreError> {
    match rows(r, name)? {
        [] => Ok(None),
        [row] => Ok(Some(row)),
        _ => Err(shape(name, Expected::AtMostOneRow)),
    }
}

pub(crate) fn integer(v: &Value, name: &'static str) -> Result<i64, StoreError> {
    match v {
        Value::Integer(i) => Ok(*i),
        _ => Err(shape(name, Expected::Integer)),
    }
}

pub(crate) fn unsigned(v: &Value, name: &'static str) -> Result<u64, StoreError> {
    u64::try_from(integer(v, name)?).map_err(|_| shape(name, Expected::NonNegative))
}

pub(crate) fn seq(v: &Value, name: &'static str) -> Result<Seq, StoreError> {
    Seq::new(unsigned(v, name)?).ok_or(shape(name, Expected::Seq))
}

pub(crate) fn blob(v: &Value, name: &'static str) -> Result<Vec<u8>, StoreError> {
    match v {
        Value::Blob(b) => Ok(b.clone()),
        _ => Err(shape(name, Expected::Bytes)),
    }
}

pub(crate) fn text<'a>(v: &'a Value, name: &'static str) -> Result<&'a str, StoreError> {
    match v {
        Value::Text(t) => Ok(t),
        _ => Err(shape(name, Expected::Text)),
    }
}

/// The value of a zero-or-one-row, one-column bytes result.
pub(crate) fn optional_blob(
    r: &StatementResult,
    name: &'static str,
) -> Result<Option<Vec<u8>>, StoreError> {
    match optional_row(r, name)? {
        None => Ok(None),
        Some([v]) => blob(v, name).map(Some),
        Some(_) => Err(shape(name, Expected::Columns(1))),
    }
}
