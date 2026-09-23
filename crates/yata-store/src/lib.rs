//! The store's instruction set and its translation into SQL (ADR-0019).
//!
//! The rest of the project never writes SQL. It builds [`Instruction`] values, asks [`plan`] for
//! the [`Plan`] that carries them out, hands the plan's [`Batch`] to the daemon's executor, and
//! gives the rows the executor returns back to [`Plan::interpret`]. This crate is the only place
//! SQL text exists, and it is pure: planning and interpreting are functions of their arguments,
//! tested without a database.
//!
//! What this crate deliberately does not do:
//! - run anything: the executor in `yata-daemon` is the one module that touches SQLite;
//! - know what a fact means: commits and blobs are bytes here, encoded by the daemon;
//! - offer a way to write SQL: [`Statement`] and [`Batch`] have no public constructor, and no
//!   instruction carries SQL text.
//!
//! The instruction set is append-only by construction (ADR-0019, rule 5): nothing updates or
//! deletes a commit. Pruning named blobs is the one deletion.

mod instruction;
mod interpret;
mod plan;
mod schema;
mod statement;

pub use instruction::{Digest, Instruction, MetaKey, StoreId};
pub use interpret::{CacheEntry, Output, StoreError, StoreKind};
pub use plan::{Plan, plan};
pub use schema::{APPLICATION_ID, SCHEMA_VERSION, check_settings, connection_settings};
pub use statement::{Batch, Param, Statement, StatementResult, Value};
