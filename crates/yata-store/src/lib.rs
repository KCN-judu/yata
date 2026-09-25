//! The store's instruction set and its translation into SQL (ADR-0019).
//!
//! The rest of the project never writes SQL. It builds instruction values, asks [`plan`] for
//! the [`Plan`] that carries them out, hands the plan's [`Batch`] to the daemon's executor, and
//! gives the rows the executor returns back to [`Plan::interpret`], which answers in the
//! instruction's own output type: each instruction is a type with its [`Instruction::Output`],
//! and instructions that land together are a tuple or a `Vec` whose output has the same shape.
//! This crate is the only place SQL text exists, and it is pure: planning and interpreting are
//! functions of their arguments, tested without a database.
//!
//! What this crate deliberately does not do:
//! - run anything: the executor in `yata-daemon` is the one module that touches SQLite;
//! - know what a fact means: commits and blobs are bytes here, encoded by the daemon;
//! - offer a way to write SQL: [`Statement`] and [`Batch`] have no public constructor, no
//!   instruction carries SQL text, and [`Instruction`] is sealed.
//!
//! The instruction set is append-only by construction (ADR-0019, rule 5): nothing updates or
//! deletes a commit. Pruning named blobs is the one deletion.

mod instruction;
mod interpret;
mod ops;
mod plan;
mod schema;
mod statement;

pub use instruction::{Digest, Instruction, MetaKey, Seq, StoreId};
pub use interpret::{CacheEntry, Check, Expected, StoreError, StoreKind};
pub use ops::{
    AppendCommit, GetBlob, Initialize, Inspect, IntegrityCheck, LastSeq, PruneBlobs, PutBlob,
    QuickCheck, ReadCache, ReadCommits, ReadMeta, ReplaceCache,
};
pub use plan::{Plan, plan};
pub use schema::{APPLICATION_ID, ConnectionSettings, SCHEMA_VERSION, connection_settings};
pub use statement::{Batch, BatchMode, Param, Statement, StatementResult, Value};
