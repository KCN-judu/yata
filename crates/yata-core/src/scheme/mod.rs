//! Scheme codes: the transport of the game's official format (ADR-0009, `scheme-code.md`).
//!
//! This module is the research instrument the rest of the scheme-code work stands on. It
//! recovers the game's binary payload from the Base64 text a scheme QR code carries, keeps every
//! byte of it, and writes a payload back. It interprets nothing inside the payload: the layout is
//! still being recovered, and a byte is a byte until the layout marks it solved.
//!
//! - [`transport`]: Base64 text ⇄ zlib stream ⇄ [`RawSchemePayload`], with bounded input.
//!
//! Reading a QR image and producing a QR module matrix are the daemon's (ADR-0009).

pub mod transport;

mod payload;

pub use payload::RawSchemePayload;
