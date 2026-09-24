//! Yata's pure core: the soul domain and the game's rules over it.
//!
//! Everything here is a function of its arguments. The crate performs no I/O, reads no clock,
//! and draws no randomness (ADR-0001); its `clippy.toml` makes those entry points lint errors.
//! Semantic truth lives here and nowhere else: the daemon persists and transports what this
//! crate decides, and the application only shows it (`docs/architecture/overview.md`).
//!
//! - [`soul`]: the vocabulary of `docs/spec/glossary.md` — attributes, slots, a soul as read.
//! - [`fact`]: the facts of the durable log as typed values, the fold into the projection, and
//!   the soul inventory derived from it (ADR-0002).
//! - [`import`]: readings of the game as typed observations, and the evidence analyses over them.
//! - [`mechanics`]: the inference rules of `docs/spec/soul-mechanics.md` — legality, roll-count
//!   inference, the community predicates, and the roll distribution.

pub mod fact;
pub mod import;
pub mod mechanics;
pub mod scheme;
pub mod soul;
