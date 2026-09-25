//! The tables of `fact-format.md`, § Tables, and the connection settings of ADR-0019.

use crate::interpret::{StoreError, exactly};
use crate::statement::{Batch, BatchMode, Statement, StatementResult, Value};

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

/// The connection settings, run once per connection before anything else and outside any
/// transaction. The order matters: exclusive locking comes before WAL, so SQLite keeps the WAL
/// index in memory and creates no shared-memory file (ADR-0019, rule 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionSettings {
    batch: Batch,
}

/// The settings in the order the batch runs them.
const SETTINGS: [&str; 4] = [
    "PRAGMA locking_mode = EXCLUSIVE",
    "PRAGMA journal_mode = WAL",
    "PRAGMA synchronous = FULL",
    "PRAGMA foreign_keys = OFF",
];

pub fn connection_settings() -> ConnectionSettings {
    ConnectionSettings {
        batch: Batch::new(
            SETTINGS
                .iter()
                .map(|sql| Statement::new(sql, vec![]))
                .collect(),
            BatchMode::Autocommit,
        ),
    }
}

impl ConnectionSettings {
    pub fn batch(&self) -> &Batch {
        &self.batch
    }

    /// SQLite answers a `locking_mode` or `journal_mode` assignment with the mode it actually
    /// took, which may differ (an in-memory database cannot use WAL). Anything but the requested
    /// modes refuses the connection.
    pub fn check(&self, results: &[StatementResult]) -> Result<(), StoreError> {
        let [locking, journal, _synchronous, _foreign_keys] = exactly(results)?;
        let mode = |r: &StatementResult| -> Option<String> {
            match r {
                StatementResult::Rows(rows) => match rows.first()?.first()? {
                    Value::Text(t) => Some(t.to_ascii_lowercase()),
                    _ => None,
                },
                StatementResult::Changed(_) => None,
            }
        };
        for (r, setting, wanted) in [
            (locking, "locking_mode", "exclusive"),
            (journal, "journal_mode", "wal"),
        ] {
            let found = mode(r);
            if found.as_deref() != Some(wanted) {
                return Err(StoreError::Setting { setting, found });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(t: &str) -> StatementResult {
        StatementResult::Rows(vec![vec![Value::Text(t.to_owned())]])
    }

    #[test]
    fn settings_hold_only_if_sqlite_took_them() {
        let s = connection_settings();
        let ok = [
            text("exclusive"),
            text("wal"),
            StatementResult::Changed(0),
            StatementResult::Changed(0),
        ];
        assert_eq!(s.check(&ok), Ok(()));
        let memory = [
            text("exclusive"),
            text("memory"),
            StatementResult::Changed(0),
            StatementResult::Changed(0),
        ];
        assert_eq!(
            s.check(&memory),
            Err(StoreError::Setting {
                setting: "journal_mode",
                found: Some("memory".to_owned())
            })
        );
        let silent = [
            text("exclusive"),
            StatementResult::Rows(vec![]),
            StatementResult::Changed(0),
            StatementResult::Changed(0),
        ];
        assert_eq!(
            s.check(&silent),
            Err(StoreError::Setting {
                setting: "journal_mode",
                found: None
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
    fn exclusive_locking_comes_before_wal_outside_a_transaction() {
        let s = connection_settings();
        assert_eq!(s.batch().mode(), BatchMode::Autocommit);
        let sql: Vec<_> = s.batch().statements().iter().map(|st| st.sql()).collect();
        let lock = sql.iter().position(|q| q.contains("locking_mode"));
        let wal = sql.iter().position(|q| q.contains("journal_mode"));
        assert!(lock < wal);
    }
}
