//! The game's soul rules, as `docs/spec/soul-mechanics.md` states them.
//!
//! Each public function implements one named rule of that page, and its tests pin the page's
//! own worked numbers. A case the page does not cover is not decided here: a rule that needs a
//! value the page marks TBD returns [`Undecided`] instead of guessing.

mod distribution;
mod inference;
mod legality;
mod predicates;
mod values;

pub use distribution::{
    class_weight, draw_weight, first_roll_adds_leg, stays_three_leg, stays_three_leg_odds,
};
pub use inference::{HitCount, HitRange, Hits, hits};
pub use legality::{Assessment, Verdict, Violation, Warning, assess};
pub use predicates::{Truth, double_speed, maxed, top_band, true_n};
pub use values::{IncrementRange, VALUE_TOLERANCE, increment_range, main_value};

use crate::soul::SoulAttribute;

/// A rule could not be evaluated because a value it needs is TBD in the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Undecided {
    /// `[lo(a), hi(a)]` is not sourced for this star (`soul-mechanics.md`, § Values).
    IncrementRangeUnknown { attribute: SoulAttribute, star: u8 },
}
