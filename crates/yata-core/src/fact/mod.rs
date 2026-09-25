//! The facts of the durable log (`fact-format.md`), as typed values, and the pure functions over
//! them: folding facts into the [`Projection`], and deriving a profile's [`Inventory`] from the
//! projection and its live snapshots.
//!
//! A fact here is always in its kind's current form. Bytes, versions, and lifting older
//! payloads to the current form are the daemon's fact codec (ADR-0002, `architecture/overview.md`):
//! nothing in this module knows how a fact is encoded, and nothing reads a clock, draws an id,
//! or touches the store.

mod inventory;
mod model;
mod projection;

pub use inventory::{Inventory, InventoryError, InventorySoul, SoulDefect};
pub use model::{
    Commit, Digest, Fact, FactBody, Facts, GameAccountId, GameSoulId, IdError, Mark, NoteText,
    Origin, ProfileId, Revision, Sections, Seq, SnapshotImport,
};
pub use projection::{
    FoldError, ImportRecord, ImportStatus, Layer, LiveSection, ProfileState, ProfileStatus,
    Projection, fold,
};
