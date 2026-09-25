//! Decode of the reader's readings into domain observations (`probe-protocol.md`,
//! `architecture/overview.md`: decode belongs to this crate).
//!
//! - [`observation`]: a reading as typed values with the evidence behind each field, and the
//!   records the reader saw, verbatim.
//! - [`evidence`]: the research analyses over readings — what the records hold, how they group,
//!   and whether the inherited suit codes survive an attested reading (ADR-0014).
//!
//! - [`snapshot`]: a snapshot file's souls, as every format's parser reads them, and how each
//!   becomes a domain soul (`spec/import-format.md`).
//!
//! The daemon converts the wire's messages and the files' bytes into these types; this module
//! never sees either.

pub mod evidence;
pub mod observation;
pub mod snapshot;
