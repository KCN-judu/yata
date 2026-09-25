//! Imported data, from the snapshot IR to domain values (ADR-0031), and the reader's readings
//! that are still here until the fact log stops storing them.
//!
//! - [`ir`]: the snapshot IR — what Yata received from one file, as independent sections
//!   (`spec/snapshot-ir.md`).
//! - [`admit`]: the IR's records as domain values, or why each is not one.
//! - [`capability`]: what a profile can do, from the sections it holds.
//! - [`observation`], [`evidence`]: the retired reader's readings and the research analyses over
//!   them; removed with the probe schema (ADR-0031, rule 10).
//!
//! The daemon turns the files' bytes into the IR; this module never sees them.

pub mod admit;
pub mod capability;
pub mod evidence;
#[cfg(test)]
mod fixture;
pub mod ir;
pub mod observation;
