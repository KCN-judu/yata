//! Yata's wire, shared by the daemon and `yata-reader` (ADR-0004, ADR-0006).
//!
//! - [`frame`]: the one frame codec both channels use, as `core-protocol.md`, § Frame states it.
//!
//! The crate is pure and depends on no workspace crate, so the reader can depend on it without
//! compiling the domain (ADR-0005, trigger 3). It moves bytes between frames and payloads and
//! does no I/O: the daemon and the reader read and write the pipes, and feed the bytes here.

pub mod frame;
