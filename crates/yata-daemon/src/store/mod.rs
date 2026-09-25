//! The store: a SQLite file reached only through `yata-store`'s plans (ADR-0019).
//!
//! This module decides what opening a store means — refusing a foreign file, a newer format,
//! or a damaged one — and turns instructions into plans, plans into runs, and results back into
//! outputs. It holds no SQL; [`executor`] runs it.
//!
//! - [`fact`]: the fact codec, `fact.proto` and the lift chain (ADR-0002).
//! - [`blob`]: a snapshot's or a file's bytes at rest, named by their SHA-256.
//! - [`FactLog`]: the log replayed into the projection; commits, imports, and inventories.

pub mod blob;
mod executor;
pub mod fact;
mod log;

use std::path::{Path, PathBuf};

pub use executor::ExecError;
use executor::{Executor, IfMissing};
pub use log::{
    CanonicalCodec, CommandOutcome, CommitError, FactLog, ImportError, Imported,
    InventoryReadError, LoadError, SnapshotCodec, format_commit, read_commits,
};
use yata_store::{
    Check, Initialize, Inspect, Instruction, IntegrityCheck, MetaKey, QuickCheck, ReadMeta,
    StoreError, StoreId, StoreKind, connection_settings, plan,
};

/// The store format this build writes and reads (`fact-format.md`, § Reading old facts). It
/// moves when the fact envelope, a fact kind or version, a fact enum's values, or the table
/// layout changes (ADR-0032, rule 2).
pub const STORE_FORMAT_VERSION: u32 = 2;

/// SQLite's `SQLITE_NOTADB`: the file is not a database. SQLite reads the header lazily, so the
/// first statement on the connection reports it.
const SQLITE_NOTADB: i32 = 26;

/// Why a store could not be opened. Each variant carries what the caller needs to report it.
#[derive(Debug, Clone, PartialEq)]
pub enum OpenError {
    Missing {
        path: PathBuf,
    },
    /// The file is not an SQLite database at all. Nothing was written to it.
    NotADatabase,
    /// The file is empty, and opening was asked not to create a store in it. Nothing was
    /// written to it.
    Uninitialized,
    /// The file is a database but not a Yata store. Nothing was written to it.
    Foreign {
        application_id: i64,
        objects: i64,
    },
    /// `store.newer_format`: written by a newer build. Nothing was written to it.
    NewerFormat {
        found: u32,
        known: u32,
    },
    /// Written in a format no build reads any more: format 1, whose facts held the retired
    /// reader's readings (ADR-0032, rule 3). The store is recreated. Nothing was written to it.
    RetiredFormat {
        found: u32,
    },
    /// The format version in `meta` is absent or unreadable.
    NoFormatVersion,
    /// SQLite's quick check reported problems.
    Damaged {
        problems: Vec<String>,
    },
    Failure(Failure),
}

/// A failure to run instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum Failure {
    Exec(ExecError),
    Store(StoreError),
}

impl From<Failure> for OpenError {
    fn from(f: Failure) -> OpenError {
        OpenError::Failure(f)
    }
}

pub struct Store {
    exec: Executor,
}

impl Store {
    /// Open an existing store; a missing file is an error and is not created.
    pub fn open(path: &Path) -> Result<Store, OpenError> {
        if !path.is_file() {
            return Err(OpenError::Missing {
                path: path.to_owned(),
            });
        }
        let mut store = Store::connect(path, IfMissing::Refuse)?;
        store.admit(None::<fn() -> StoreId>)?;
        Ok(store)
    }

    /// Open the store, creating and initializing it when the file is missing or empty.
    /// `new_id` is called only when a store is created.
    pub fn create_or_open(
        path: &Path,
        new_id: impl FnOnce() -> StoreId,
    ) -> Result<Store, OpenError> {
        let mut store = Store::connect(path, IfMissing::Create)?;
        store.admit(Some(new_id))?;
        Ok(store)
    }

    /// Run an instruction, or instructions that must land together, as one atomic batch, and
    /// answer in its output type.
    pub fn apply<I: Instruction>(&mut self, instruction: I) -> Result<I::Output, Failure> {
        let p = plan(instruction);
        let results = self.exec.run(p.batch()).map_err(|e| match e {
            ExecError::Guard { statement, .. } => match p.guard_error(statement) {
                Some(meaning) => Failure::Store(meaning),
                None => Failure::Exec(e),
            },
            e => Failure::Exec(e),
        })?;
        p.interpret(&results).map_err(Failure::Store)
    }

    /// SQLite's full integrity check.
    pub fn integrity_check(&mut self) -> Result<Check, Failure> {
        self.apply(IntegrityCheck)
    }

    fn connect(path: &Path, if_missing: IfMissing) -> Result<Store, OpenError> {
        let mut exec = Executor::open(path, if_missing).map_err(Failure::Exec)?;
        let settings = connection_settings();
        let results = exec.run(settings.batch()).map_err(|e| match e {
            ExecError::Sqlite {
                code: Some(SQLITE_NOTADB),
                ..
            } => OpenError::NotADatabase,
            e => Failure::Exec(e).into(),
        })?;
        settings.check(&results).map_err(Failure::Store)?;
        Ok(Store { exec })
    }

    /// Decide whether the file may be used, initializing an empty one when `new_id` is given.
    fn admit(&mut self, new_id: Option<impl FnOnce() -> StoreId>) -> Result<(), OpenError> {
        match (self.apply(Inspect)?, new_id) {
            (StoreKind::Yata, _) => {}
            (StoreKind::Empty, Some(new_id)) => self.apply(Initialize {
                store_id: new_id(),
                store_format_version: STORE_FORMAT_VERSION,
            })?,
            (StoreKind::Empty, None) => return Err(OpenError::Uninitialized),
            (
                StoreKind::Foreign {
                    application_id,
                    objects,
                },
                _,
            ) => {
                return Err(OpenError::Foreign {
                    application_id,
                    objects,
                });
            }
        }
        let found = self
            .apply(ReadMeta {
                key: MetaKey::StoreFormatVersion,
            })?
            .and_then(|bytes| <[u8; 4]>::try_from(bytes.as_slice()).ok())
            .map(u32::from_be_bytes)
            .ok_or(OpenError::NoFormatVersion)?;
        if found > STORE_FORMAT_VERSION {
            return Err(OpenError::NewerFormat {
                found,
                known: STORE_FORMAT_VERSION,
            });
        }
        if found < STORE_FORMAT_VERSION {
            return Err(OpenError::RetiredFormat { found });
        }
        match self.apply(QuickCheck)? {
            Check::Ok => Ok(()),
            Check::Problems(problems) => Err(OpenError::Damaged { problems }),
        }
    }
}

/// A fresh random store id, from the operating system's generator.
pub fn random_store_id() -> Result<StoreId, getrandom::Error> {
    let mut id = [0u8; 16];
    getrandom::fill(&mut id)?;
    Ok(StoreId(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A store initialized at `format`, through the store's own instructions.
    fn initialized_at(path: &Path, format: u32) {
        let mut store = Store::connect(path, IfMissing::Create).expect("connect");
        assert_eq!(store.apply(Inspect), Ok(StoreKind::Empty));
        store
            .apply(Initialize {
                store_id: StoreId([7; 16]),
                store_format_version: format,
            })
            .expect("initialize");
    }

    #[test]
    fn a_format_one_store_is_retired_and_a_newer_one_refused() {
        let dir = std::env::temp_dir()
            .join(format!("yata-unit-{}-format", std::process::id()))
            .join("数据 目录");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        let open = |format: u32| {
            let path = dir.join(format!("store-{format}.sqlite3"));
            initialized_at(&path, format);
            Store::open(&path).err()
        };
        assert_eq!(open(1), Some(OpenError::RetiredFormat { found: 1 }));
        assert_eq!(
            open(STORE_FORMAT_VERSION + 1),
            Some(OpenError::NewerFormat {
                found: STORE_FORMAT_VERSION + 1,
                known: STORE_FORMAT_VERSION
            })
        );
        assert_eq!(open(STORE_FORMAT_VERSION), None);
        if let Some(parent) = dir.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }
    }
}
