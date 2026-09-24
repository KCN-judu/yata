//! Yata's core process: the effectful shell around the pure crates (ADR-0001, ADR-0004).
//!
//! The daemon owns every effect — files, the clock, randomness, child processes — and calls the
//! pure crates for every judgment. This library is what the `yata-daemon` binary runs; it is a
//! library so that its integration tests can drive it directly.
//!
//! - [`qr`]: QR matrices for scheme texts, and scheme texts from QR images (ADR-0009).
//! - [`scheme`]: the scheme-code research commands' file reading and output.
//! - [`store`]: the SQLite store behind `yata-store`'s instructions (ADR-0019).

pub mod qr;
pub mod scheme;
pub mod store;
