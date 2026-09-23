/// A parameter bound to a statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Param {
    Integer(i64),
    Text(&'static str),
    Blob(Vec<u8>),
}

/// A column value as SQLite returns it, before interpretation.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

/// One SQL statement with its parameters. Only this crate can make one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    sql: &'static str,
    params: Vec<Param>,
    must_change: Option<u64>,
}

impl Statement {
    pub(crate) fn new(sql: &'static str, params: Vec<Param>) -> Statement {
        Statement {
            sql,
            params,
            must_change: None,
        }
    }

    /// A statement whose batch is void unless it changes exactly `rows` rows. The executor
    /// checks it and rolls the whole batch back, so a guarded write never lands half a plan.
    pub(crate) fn guarded(sql: &'static str, params: Vec<Param>, rows: u64) -> Statement {
        Statement {
            sql,
            params,
            must_change: Some(rows),
        }
    }

    pub fn sql(&self) -> &'static str {
        self.sql
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    /// The exact number of rows this statement must change, if it is guarded.
    pub fn must_change(&self) -> Option<u64> {
        self.must_change
    }
}

/// The statements of a plan, in order. When `atomic`, the executor runs them in one write
/// transaction (`BEGIN IMMEDIATE … COMMIT`) and rolls all of them back if any fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    statements: Vec<Statement>,
    atomic: bool,
}

impl Batch {
    pub(crate) fn new(statements: Vec<Statement>, atomic: bool) -> Batch {
        Batch { statements, atomic }
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    pub fn is_atomic(&self) -> bool {
        self.atomic
    }
}

/// What the executor observed for one statement: the rows it returned, and how many rows it
/// changed.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StatementResult {
    pub rows: Vec<Vec<Value>>,
    pub changes: u64,
}
