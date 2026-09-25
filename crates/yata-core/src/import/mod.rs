//! Imported data, from the snapshot IR to domain values (ADR-0031).
//!
//! - [`ir`]: the snapshot IR — what Yata received from one file, as independent sections
//!   (`spec/snapshot-ir.md`).
//! - [`admit`]: the IR's records as domain values, or why each is not one.
//! - [`capability`]: what a profile can do, from the sections it holds.
//!
//! The daemon turns the files' bytes into the IR; this module never sees them.

pub mod admit;
pub mod capability;
#[cfg(test)]
pub(crate) mod fixture;
pub mod ir;
