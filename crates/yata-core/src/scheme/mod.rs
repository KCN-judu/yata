//! Scheme codes: the transport of the game's official format (ADR-0009, `scheme-code.md`).
//!
//! This module is the research instrument the rest of the scheme-code work stands on. It
//! recovers the game's binary payload from the Base64 text a scheme QR code carries, keeps every
//! byte of it, and writes a payload back. Inside the payload it interprets only what an import has
//! confirmed: the header and the record framing, and, for editing, the solved bits.
//!
//! - [`transport`]: Base64 text ⇄ zlib stream ⇄ [`RawSchemePayload`], with bounded input.
//! - [`layout`]: the payload as its header (account and kind) and records, written back byte for
//!   byte.
//! - [`edit`]: records built or changed bit by bit, for solved bits only.
//! - [`inspect`]: a hex dump, a byte-and-bit diff of two payloads, and a low-level bit reader, for
//!   comparing controlled samples.
//!
//! Reading a QR image and producing a QR module matrix are the daemon's (ADR-0009).

pub mod edit;
pub mod inspect;
pub mod layout;
pub mod transport;

mod payload;

pub use payload::RawSchemePayload;
