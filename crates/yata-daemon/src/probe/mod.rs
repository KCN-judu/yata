//! The probe channel's daemon side (ADR-0006, ADR-0008): sessions with `yata-reader`, recordings
//! and their replay, export files, and conversion of readings into domain observations.
//!
//! - [`cli`]: the `yata-daemon probe …` commands.
//! - [`session`]: the live session over any byte stream, and replay of a recording through the
//!   same decoder and rules.
//! - [`convert`]: a `ReadResult` as a [`yata_core::import::observation::SoulReading`], and a
//!   reading's blob.
//! - [`input`]: readings from a recording file or an export file.
//! - [`launch`]: the named pipe and the reader process, detect-then-elevate (Windows only).
//! - [`report`]: the text the `probe` research commands print.

pub mod cli;
pub mod convert;
pub mod input;
#[cfg(windows)]
pub mod launch;
pub mod report;
pub mod session;
