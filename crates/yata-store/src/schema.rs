//! The tables of `fact-format.md`, § Tables, and the connection settings of ADR-0019.

use crate::interpret::StoreError;
use crate::statement::{Batch, Statement, StatementResult, Value};

/// The value of SQLite's `application_id` header field in a Yata store: "YATA" in ASCII.
pub const APPLICATION_ID: i64 = 0x5941_5441;

/// The version of the table layout below. Part of the store format version: changing a table
/// is a store format change (`fact-format.md`, § Reading old facts).
pub const SCHEMA_VERSION: u32 = 1;

/// Every column holds an integer or bytes; SQL interprets no value (ADR-0019, rule 3). The two
/// triggers make the log append-only in the database itself, beneath the instruction set.
pub(crate) const CREATE_SCHEMA: [&str; 6] = [
    "CREATE TABLE meta (key TEXT PRIMARY KEY NOT NULL, value BLOB NOT NULL) STRICT, WITHOUT ROWID",
    "CREATE TABLE log (seq INTEGER PRIMARY KEY NOT NULL, commit_bytes BLOB NOT NULL) STRICT",
    "CREATE TABLE blobs (digest BLOB PRIMARY KEY NOT NULL, bytes BLOB NOT NULL) STRICT, WITHOUT ROWID",
    "CREATE TABLE projection_cache (seq INTEGER PRIMARY KEY NOT NULL, fold_version INTEGER NOT NULL, \
     projection BLOB NOT NULL) STRICT",
    "CREATE TRIGGER log_no_update BEFORE UPDATE ON log BEGIN SELECT RAISE(ABORT, 'the log is append-only'); END",
    "CREATE TRIGGER log_no_delete BEFORE DELETE ON log BEGIN SELECT RAISE(ABORT, 'the log is append-only'); END",
];

/// Run once per connection, before anything else and outside any transaction. The order
/// matters: exclusive locking comes before WAL, so SQLite keeps the WAL index in memory and
/// creates no shared-memory file (ADR-0019, rule 2).
pub fn connection_settings() -> Batch {
    Batch::new(
        vec![
            Statement::new("PRAGMA locking_mode = EXCLUSIVE", vec![]),
            Statement::new("PRAGMA journal_mode = WAL", vec![]),
            Statement::new("PRAGMA synchronous = FULL", vec![]),
            Statement::new("PRAGMA foreign_keys = OFF", vec![]),
        ],
        false,
    )
}

/// Read back the results of [`connection_settings`]: SQLite answers a `journal_mode` or
/// `locking_mode` assignment with the mode it actually took, which may differ (an in-memory
/// database cannot use WAL). Anything but the requested modes refuses the connection.
pub fn check_settings(results: &[StatementResult]) -> Result<(), StoreError> {
    let mode = |i: usize| -> Option<String> {
        match results.get(i)?.rows.first()?.first()? {
            Value::Text(t) => Some(t.to_ascii_lowercase()),
            _ => None,
        }
    };
    for (i, setting, wanted) in [(0, "locking_mode", "exclusive"), (1, "journal_mode", "wal")] {
        let found = mode(i);
        if found.as_deref() != Some(wanted) {
            return Err(StoreError::Setting {
                setting,
                found: found.unwrap_or_default(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(t: &str) -> StatementResult {
        StatementResult {
            rows: vec![vec![Value::Text(t.to_owned())]],
            changes: 0,
        }
    }

    #[test]
    fn settings_hold_only_if_sqlite_took_them() {
        let ok = [
            text("exclusive"),
            text("wal"),
            StatementResult::default(),
            StatementResult::default(),
        ];
        assert_eq!(check_settings(&ok), Ok(()));
        let memory = [text("exclusive"), text("memory")];
        assert_eq!(
            check_settings(&memory),
            Err(StoreError::Setting {
                setting: "journal_mode",
                found: "memory".to_owned()
            })
        );
    }

    #[test]
    fn application_id_spells_yata() {
        assert_eq!(APPLICATION_ID.to_be_bytes()[4..], *b"YATA");
    }

    #[test]
    fn every_table_is_strict() {
        for sql in CREATE_SCHEMA
            .iter()
            .filter(|s| s.starts_with("CREATE TABLE"))
        {
            assert!(sql.contains(") STRICT"), "{sql}");
        }
    }

    #[test]
    fn exclusive_locking_comes_before_wal() {
        let s = connection_settings();
        assert!(!s.is_atomic());
        let sql: Vec<_> = s.statements().iter().map(|st| st.sql()).collect();
        let lock = sql.iter().position(|q| q.contains("locking_mode"));
        let wal = sql.iter().position(|q| q.contains("journal_mode"));
        assert!(lock < wal);
    }
}
