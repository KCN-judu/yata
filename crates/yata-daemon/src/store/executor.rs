//! The one module that runs SQL (ADR-0019, rule 7).
//!
//! It opens the connection, runs a [`Batch`] that `yata-store` planned, and returns the raw rows.
//! It builds no statement, never branches on SQL text, and has no other entry point. A batch in
//! [`BatchMode::Transaction`] runs in one `IMMEDIATE` transaction; if any statement fails, or a
//! guarded statement changes a different number of rows than it must, the whole batch rolls back.

use std::path::Path;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags, TransactionBehavior};
use yata_store::{Batch, BatchMode, Param, Statement, StatementResult, Value};

/// What opening does when the file does not exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfMissing {
    /// Create it empty.
    Create,
    /// Fail.
    Refuse,
}

/// Why the executor could not run a batch.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecError {
    /// SQLite reported an error. `statement` is the batch index, `None` when opening or
    /// committing; `code` is SQLite's extended result code when it gave one.
    Sqlite {
        statement: Option<usize>,
        code: Option<i32>,
        message: String,
    },
    /// A guarded statement changed a different number of rows than it must. The batch was
    /// rolled back.
    Guard {
        statement: usize,
        must_change: u64,
        changed: u64,
    },
    /// A guarded statement returned rows instead of changing any: a plan defect. The batch was
    /// rolled back.
    GuardedQuery { statement: usize },
    /// A text column held bytes that are not UTF-8.
    NotUtf8 { statement: usize, column: usize },
}

fn sqlite(statement: Option<usize>, e: &rusqlite::Error) -> ExecError {
    ExecError::Sqlite {
        statement,
        code: e.sqlite_error().map(|f| f.extended_code),
        message: e.to_string(),
    }
}

pub struct Executor {
    conn: Connection,
}

impl Executor {
    /// Open the database file.
    pub fn open(path: &Path, if_missing: IfMissing) -> Result<Executor, ExecError> {
        let mut flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        if if_missing == IfMissing::Create {
            flags |= OpenFlags::SQLITE_OPEN_CREATE;
        }
        Connection::open_with_flags(path, flags)
            .map(|conn| Executor { conn })
            .map_err(|e| sqlite(None, &e))
    }

    /// Run every statement of the batch in order and return one result per statement.
    pub fn run(&mut self, batch: &Batch) -> Result<Vec<StatementResult>, ExecError> {
        match batch.mode() {
            BatchMode::Autocommit => run_all(&self.conn, batch.statements()),
            BatchMode::Transaction => {
                let tx = self
                    .conn
                    .transaction_with_behavior(TransactionBehavior::Immediate)
                    .map_err(|e| sqlite(None, &e))?;
                // Dropping `tx` on an early return rolls the batch back.
                let results = run_all(&tx, batch.statements())?;
                tx.commit().map_err(|e| sqlite(None, &e))?;
                Ok(results)
            }
        }
    }
}

fn run_all(conn: &Connection, statements: &[Statement]) -> Result<Vec<StatementResult>, ExecError> {
    statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let r = run_one(conn, i, s)?;
            match (s.must_change(), &r) {
                (None, _) => Ok(r),
                (Some(must), StatementResult::Changed(changed)) if must == *changed => Ok(r),
                (Some(must), StatementResult::Changed(changed)) => Err(ExecError::Guard {
                    statement: i,
                    must_change: must,
                    changed: *changed,
                }),
                (Some(_), StatementResult::Rows(_)) => {
                    Err(ExecError::GuardedQuery { statement: i })
                }
            }
        })
        .collect()
}

fn run_one(conn: &Connection, index: usize, s: &Statement) -> Result<StatementResult, ExecError> {
    let e = |e: rusqlite::Error| sqlite(Some(index), &e);
    let mut stmt = conn.prepare(s.sql()).map_err(e)?;
    for (i, p) in s.params().iter().enumerate() {
        match p {
            Param::Integer(v) => stmt.raw_bind_parameter(i + 1, v),
            Param::Text(t) => stmt.raw_bind_parameter(i + 1, t),
            Param::Blob(b) => stmt.raw_bind_parameter(i + 1, b),
        }
        .map_err(e)?;
    }
    let columns = stmt.column_count();
    if columns == 0 {
        let changes = stmt.raw_execute().map_err(e)?;
        return Ok(StatementResult::Changed(changes as u64));
    }
    let mut rows = Vec::new();
    let mut cursor = stmt.raw_query();
    while let Some(row) = cursor.next().map_err(e)? {
        let mut values = Vec::with_capacity(columns);
        for c in 0..columns {
            let v = row.get_ref(c).map_err(e)?;
            values.push(value(v).ok_or(ExecError::NotUtf8 {
                statement: index,
                column: c,
            })?);
        }
        rows.push(values);
    }
    Ok(StatementResult::Rows(rows))
}

/// A column value, or `None` for text that is not UTF-8.
fn value(v: ValueRef<'_>) -> Option<Value> {
    Some(match v {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(i) => Value::Integer(i),
        ValueRef::Real(r) => Value::Real(r),
        ValueRef::Text(t) => Value::Text(String::from_utf8(t.to_vec()).ok()?),
        ValueRef::Blob(b) => Value::Blob(b.to_vec()),
    })
}
