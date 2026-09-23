//! The store: a SQLite file reached only through `yata-store`'s plans (ADR-0019).
//!
//! This module decides what opening a store means — refusing a foreign file, a newer format,
//! or a damaged one — and turns instructions into plans, plans into runs, and results back into
//! outputs. It holds no SQL; [`executor`] runs it.

mod executor;

use std::path::{Path, PathBuf};

pub use executor::ExecError;
use executor::Executor;
use yata_store::{
    Instruction, MetaKey, Output, StoreError, StoreId, StoreKind, check_settings,
    connection_settings, plan,
};

/// The store format this build writes and reads (`fact-format.md`, § Reading old facts). It
/// moves only when the fact envelope or the table layout changes.
pub const STORE_FORMAT_VERSION: u32 = 1;

/// Why a store could not be opened. Each variant carries what the caller needs to report it.
#[derive(Debug, Clone, PartialEq)]
pub enum OpenError {
    Missing {
        path: PathBuf,
    },
    /// The file is not an SQLite database at all. Nothing was written to it.
    NotADatabase,
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
        let mut store = Store::connect(path, false)?;
        store.admit(None::<fn() -> StoreId>)?;
        Ok(store)
    }

    /// Open the store, creating and initializing it when the file is missing or empty.
    /// `new_id` is called only when a store is created.
    pub fn create_or_open(
        path: &Path,
        new_id: impl FnOnce() -> StoreId,
    ) -> Result<Store, OpenError> {
        let mut store = Store::connect(path, true)?;
        store.admit(Some(new_id))?;
        Ok(store)
    }

    /// Run instructions that must land together, as one atomic batch.
    pub fn apply(&mut self, instructions: Vec<Instruction>) -> Result<Vec<Output>, Failure> {
        let p = plan(instructions).map_err(Failure::Store)?;
        let results = match self.exec.run(p.batch()) {
            Ok(r) => r,
            Err(ExecError::Guard { statement, .. }) => {
                return Err(Failure::Store(p.guard_error(statement)));
            }
            Err(e) => return Err(Failure::Exec(e)),
        };
        p.interpret(results).map_err(Failure::Store)
    }

    /// SQLite's full integrity check: the problems it reports, or none.
    pub fn integrity_check(&mut self) -> Result<Vec<String>, Failure> {
        match self.apply(vec![Instruction::IntegrityCheck])?.pop() {
            Some(Output::Check(problems)) => Ok(problems),
            _ => Err(shape("IntegrityCheck")),
        }
    }

    fn connect(path: &Path, create: bool) -> Result<Store, OpenError> {
        // SQLITE_NOTADB: SQLite reads the header lazily, so the first statement reports it.
        const NOT_A_DATABASE: i32 = 26;
        let mut exec = Executor::open(path, create).map_err(Failure::Exec)?;
        let results = exec.run(&connection_settings()).map_err(|e| match e {
            ExecError::Sqlite {
                code: Some(NOT_A_DATABASE),
                ..
            } => OpenError::NotADatabase,
            e => Failure::Exec(e).into(),
        })?;
        check_settings(&results).map_err(Failure::Store)?;
        Ok(Store { exec })
    }

    /// Decide whether the file may be used, initializing an empty one when `new_id` is given.
    fn admit(&mut self, new_id: Option<impl FnOnce() -> StoreId>) -> Result<(), OpenError> {
        match (self.single(Instruction::Inspect)?, new_id) {
            (Output::Kind(StoreKind::Yata), _) => {}
            (Output::Kind(StoreKind::Empty), Some(new_id)) => {
                self.apply(vec![Instruction::Initialize {
                    store_id: new_id(),
                    store_format_version: STORE_FORMAT_VERSION,
                }])?;
            }
            (Output::Kind(StoreKind::Empty), None) => {
                return Err(OpenError::Foreign {
                    application_id: 0,
                    objects: 0,
                });
            }
            (
                Output::Kind(StoreKind::Foreign {
                    application_id,
                    objects,
                }),
                _,
            ) => {
                return Err(OpenError::Foreign {
                    application_id,
                    objects,
                });
            }
            (_, _) => return Err(shape("Inspect").into()),
        }
        let found = match self.single(Instruction::ReadMeta {
            key: MetaKey::StoreFormatVersion,
        })? {
            Output::Meta(Some(bytes)) => <[u8; 4]>::try_from(bytes.as_slice())
                .map(u32::from_be_bytes)
                .map_err(|_| OpenError::NoFormatVersion)?,
            _ => return Err(OpenError::NoFormatVersion),
        };
        if found > STORE_FORMAT_VERSION {
            return Err(OpenError::NewerFormat {
                found,
                known: STORE_FORMAT_VERSION,
            });
        }
        match self.single(Instruction::QuickCheck)? {
            Output::Check(problems) if problems.is_empty() => Ok(()),
            Output::Check(problems) => Err(OpenError::Damaged { problems }),
            _ => Err(shape("QuickCheck").into()),
        }
    }

    fn single(&mut self, instruction: Instruction) -> Result<Output, Failure> {
        self.apply(vec![instruction])?
            .pop()
            .ok_or_else(|| shape("instruction"))
    }
}

fn shape(instruction: &'static str) -> Failure {
    Failure::Store(StoreError::Shape {
        instruction,
        detail: "unexpected output",
    })
}

/// A fresh random store id, from the operating system's generator.
pub fn random_store_id() -> Result<StoreId, getrandom::Error> {
    let mut id = [0u8; 16];
    getrandom::fill(&mut id)?;
    Ok(id)
}
