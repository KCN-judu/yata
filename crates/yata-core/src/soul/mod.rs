//! The soul vocabulary of `docs/spec/glossary.md`.
//!
//! Only the domain's names and shapes live here. Which values are legal, and what can be
//! inferred from them, is [`crate::mechanics`].

mod attribute;
mod innate;
mod model;
mod set;
mod slot;

pub use attribute::{AttributeCategory, RollClass, SoulAttribute};
pub use innate::InnateAttribute;
pub use model::{Soul, SoulKind, SubAttribute};
pub use set::SoulSet;
pub use slot::SoulSlot;
