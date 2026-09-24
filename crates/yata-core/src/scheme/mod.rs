//! Scheme codes: the game's official format, from its transport to its meaning (ADR-0009,
//! `scheme-code.md`).
//!
//! Three levels, each on the one below: the transport recovers the game's binary payload from
//! the Base64 text a scheme QR code carries and writes one back; the layout reads the payload as
//! a header and records, byte for byte; the semantic level reads a record as a selection of souls.
//! Only what an import has confirmed is interpreted, and every other bit is kept.
//!
//! - [`transport`]: Base64 text ⇄ zlib stream ⇄ [`RawSchemePayload`], with bounded input.
//! - [`layout`]: the payload as its header (account and kind) and records, written back byte for
//!   byte.
//! - [`edit`]: records built or changed bit by bit, for solved bits only.
//! - [`inspect`]: a hex dump, a byte-and-bit diff of two payloads, and a low-level bit reader, for
//!   comparing controlled samples.
//! - [`selection`]: what a record means, as a [`selection::SoulSelection`] in the game's panel
//!   groups, decoded from and written over a record's fields. The bit positions it uses are
//!   `mapping`'s, the one table of them.
//! - [`code`]: a whole code as its schemes and plans, each a name and a selection.
//! - [`evaluate`]: whether the game's filter picks a soul, as far as the evidence tells.
//!
//! Reading a QR image and producing a QR module matrix are the daemon's (ADR-0009).

pub mod code;
pub mod edit;
pub mod evaluate;
pub mod inspect;
pub mod layout;
pub mod selection;
pub mod transport;

mod mapping;

mod payload;

pub use payload::RawSchemePayload;
