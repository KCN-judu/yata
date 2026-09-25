//! What data reception keeps of the probe channel (ADR-0030): recordings and their replay, export
//! files, and conversion of readings into domain observations. No reader is started from here.
//!
//! - [`cli`]: the `yata-daemon probe …` commands.
//! - [`session`]: the live session over any byte stream, and replay of a recording through the
//!   same decoder and rules.
//! - [`convert`]: a `ReadResult` as a [`yata_core::import::observation::SoulReading`], and a
//!   reading's blob.
//! - [`input`]: readings from a recording file or an export file.
//! - [`report`]: the text the `probe` research commands print.

pub mod cli;
pub mod convert;
pub mod input;
pub mod report;
pub mod session;
