//! The soul vocabulary of `docs/spec/glossary.md`.
//!
//! Only the domain's names and shapes live here. Which values are legal, and what can be
//! inferred from them, is [`crate::mechanics`].

mod attribute;
mod innate;
mod level;
mod model;
mod set;
mod slot;
mod star;
mod value;

pub use attribute::{AttributeCategory, RollClass, SoulAttribute};
pub use innate::InnateAttribute;
pub use level::{Level, NotALevel};
pub use model::{RollCount, Soul, SoulKind, SubAttribute};
pub use set::SoulSet;
pub use slot::SoulSlot;
pub use star::{NotAStar, Star};
pub use value::StoredValue;
