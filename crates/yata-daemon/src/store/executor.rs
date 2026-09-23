//! The one module that runs SQL (ADR-0019, rule 7).
//!
//! It opens the connection, runs a [`Batch`] that `yata-store` planned, and returns the raw rows.
//! It builds no statement, never branches on SQL text, and has no other entry point. A batch
//! marked atomic runs in one `IMMEDIATE` transaction; if any statement fails, or a guarded
//! statement changes a different number of rows than it must, the whole batch rolls back.

use std::path::Path;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags, TransactionBehavior};
use yata_store::{Batch, Param, Statement, StatementResult, Value};

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
    /// Open the database file. With `create`, a missing file is created empty; without it, a
    /// missing file is an error.
    pub fn open(path: &Path, create: bool) -> Result<Executor, ExecError> {
        let mut flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        if create {
            flags |= OpenFlags::SQLITE_OPEN_CREATE;
        }
        Connection::open_with_flags(path, flags)
            .map(|conn| Executor { conn })
            .map_err(|e| sqlite(None, &e))
    }

    /// Run every statement of the batch in order and return one result per statement.
    pub fn run(&mut self, batch: &Batch) -> Result<Vec<StatementResult>, ExecError> {
        if !batch.is_atomic() {
            return run_all(&self.conn, batch.statements());
        }
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

fn run_all(conn: &Connection, statements: &[Statement]) -> Result<Vec<StatementResult>, ExecError> {
    statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let r = run_one(conn, s).map_err(|e| sqlite(Some(i), &e))?;
            match s.must_change() {
                Some(must) if must != r.changes => Err(ExecError::Guard {
                    statement: i,
                    must_change: must,
                    changed: r.changes,
                }),
                _ => Ok(r),
            }
        })
        .collect()
}

fn run_one(conn: &Connection, s: &Statement) -> Result<StatementResult, rusqlite::Error> {
    let mut stmt = conn.prepare(s.sql())?;
    for (i, p) in s.params().iter().enumerate() {
        match p {
            Param::Integer(v) => stmt.raw_bind_parameter(i + 1, v)?,
            Param::Text(t) => stmt.raw_bind_parameter(i + 1, t)?,
            Param::Blob(b) => stmt.raw_bind_parameter(i + 1, b)?,
        }
    }
    let columns = stmt.column_count();
    if columns == 0 {
        let changes = stmt.raw_execute()?;
        return Ok(StatementResult {
            rows: vec![],
            changes: changes as u64,
        });
    }
    let mut rows = Vec::new();
    let mut cursor = stmt.raw_query();
    while let Some(row) = cursor.next()? {
        let mut values = Vec::with_capacity(columns);
        for c in 0..columns {
            values.push(value(row.get_ref(c)?));
        }
        rows.push(values);
    }
    Ok(StatementResult { rows, changes: 0 })
}

fn value(v: ValueRef<'_>) -> Value {
    match v {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(i) => Value::Integer(i),
        ValueRef::Real(r) => Value::Real(r),
        ValueRef::Text(t) => Value::Text(String::from_utf8_lossy(t).into_owned()),
        ValueRef::Blob(b) => Value::Blob(b.to_vec()),
    }
}
