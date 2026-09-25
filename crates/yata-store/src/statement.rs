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

/// How the executor runs a batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchMode {
    /// In one write transaction (`BEGIN IMMEDIATE … COMMIT`), rolled back whole if any statement
    /// fails.
    Transaction,
    /// Each statement on its own, outside any transaction: connection settings, which SQLite
    /// does not accept inside one.
    Autocommit,
}

/// The statements of a plan, in order, and how they run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    statements: Vec<Statement>,
    mode: BatchMode,
}

impl Batch {
    pub(crate) fn new(statements: Vec<Statement>, mode: BatchMode) -> Batch {
        Batch { statements, mode }
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    pub fn mode(&self) -> BatchMode {
        self.mode
    }
}

/// What the executor observed for one statement: the rows a query returned, or the number of
/// rows a write changed. A statement is one or the other, by whether it has result columns.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementResult {
    Rows(Vec<Vec<Value>>),
    Changed(u64),
}
