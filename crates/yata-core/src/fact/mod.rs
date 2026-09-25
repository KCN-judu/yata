//! The facts of the durable log (`fact-format.md`), as typed values, and the pure functions over
//! them: validating a reading before it becomes a fact, folding facts into the [`Projection`],
//! and deriving a profile's [`Inventory`] from the projection and its live readings.
//!
//! A fact here is always in its kind's current form. Bytes, versions, and lifting older
//! payloads to the current form are the daemon's fact codec (ADR-0002, `architecture/overview.md`):
//! nothing in this module knows how a fact is encoded, and nothing reads a clock, draws an id,
//! or touches the store.

mod admission;
mod inventory;
mod model;
mod projection;

pub use admission::{
    AdmissionError, Admitted, AdmittedSoul, NotEstablished, ROW_FIELDS, RecordDefect,
    SoulDefectKind, admit_reading, check_soul,
};
pub use inventory::{Inventory, InventoryError, InventorySoul, SoulDefect};
pub use model::{
    Acquisition, Channel, Commit, Coverage, Digest, Fact, FactBody, Facts, GameAccountId,
    GameSoulId, IdError, Mark, NoteText, Origin, ProbeVersion, ProfileId, Revision, Scope, Seq,
    Source,
};
pub use projection::{
    AcquisitionRecord, AcquisitionStatus, FoldError, LiveSnapshots, ProfileState, ProfileStatus,
    Projection, fold,
};
