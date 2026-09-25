//! Yata's core process: the effectful shell around the pure crates (ADR-0001, ADR-0004).
//!
//! The daemon owns every effect — files, the clock, randomness, child processes — and calls the
//! pure crates for every judgment. This library is what the `yata-daemon` binary runs; it is a
//! library so that its integration tests can drive it directly.
//!
//! - [`import`]: snapshot files at the import boundary: format detection, one parser per format,
//!   and the conversion of their souls into domain souls (ADR-0030, PRP-0008).
//! - [`probe`]: the reader channel — sessions, recordings, export files, and the conversion of
//!   readings into domain observations (ADR-0006, ADR-0008).
//! - [`qr`]: QR matrices for scheme texts, and scheme texts from QR images (ADR-0009).
//! - [`query`]: the headless query endpoint, over an inventory each request supplies (ADR-0026).
//! - [`scheme`]: the scheme-code research commands' file reading and output.
//! - [`serve`]: the core protocol session the application spawns (ADR-0004).
//! - [`store`]: the SQLite store behind `yata-store`'s instructions (ADR-0019), and the fact log
//!   over it (ADR-0002).
//! - [`wire`]: the one conversion between domain values and core-protocol messages.

pub mod import;
pub mod probe;
pub mod qr;
pub mod query;
pub mod scheme;
pub mod serve;
pub mod store;
pub mod wire;
