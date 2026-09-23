//! § Inferring roll counts: I-Hits.

use super::Undecided;
use super::values::{VALUE_TOLERANCE, increment_range};
use crate::soul::{Soul, SubAttribute};

/// What is known of `hits(a) = 1 + c(a)` for one sub-attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hits {
    /// The reading carries `c(a)`.
    Known(u8),
    /// Every integer in `min..=max` is admitted by I-Hits and by the soul's level; `c(a)` is
    /// ambiguous when `min < max`.
    Admitted { min: u8, max: u8 },
    /// No number of increments reachable at this level sums to the value. Only a malformed soul
    /// has this.
    Unreachable,
}

impl Hits {
    /// Whether `h` increments are possible for this sub-attribute.
    pub fn admits(self, h: u8) -> bool {
        match self {
            Hits::Known(k) => k == h,
            Hits::Admitted { min, max } => (min..=max).contains(&h),
            Hits::Unreachable => false,
        }
    }

    /// The least number of rolls, `min hits − 1`, or `None` if unreachable.
    pub fn least_rolls(self) -> Option<u8> {
        match self {
            Hits::Known(k) => Some(k.saturating_sub(1)),
            Hits::Admitted { min, .. } => Some(min - 1),
            Hits::Unreachable => None,
        }
    }
}

/// I-Hits: `⌈S(a) / hi(a)⌉ ≤ hits(a) ≤ ⌊S(a) / lo(a)⌋`, intersected with what the level allows,
/// `1 ≤ hits(a) ≤ 1 + nodes(ℓ)`. A recorded `c(a)` is taken as given; whether it agrees with
/// the value is legality's question, not this one's.
pub fn hits(soul: &Soul, sub: &SubAttribute) -> Result<Hits, Undecided> {
    if let Some(c) = sub.enhancement_count {
        return Ok(Hits::Known(c.saturating_add(1)));
    }
    let range =
        increment_range(sub.attribute, soul.star).ok_or(Undecided::IncrementRangeUnknown {
            attribute: sub.attribute,
            star: soul.star,
        })?;
    let least = ((sub.value - VALUE_TOLERANCE) / range.hi()).ceil().max(1.0);
    let most = ((sub.value + VALUE_TOLERANCE) / range.lo())
        .floor()
        .min(f64::from(1 + soul.nodes()));
    if least.is_nan() || most.is_nan() || least > most {
        return Ok(Hits::Unreachable);
    }
    // Both bounds lie in 1..=6 here, so the conversions cannot truncate.
    Ok(Hits::Admitted {
        min: least as u8,
        max: most as u8,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soul::{SoulAttribute, SoulSlot};

    fn spd_soul(level: u8, value: f64) -> Soul {
        Soul {
            slot: SoulSlot::Slot2,
            star: 6,
            level,
            main: SoulAttribute::Spd,
            main_value: 57.0,
            subs: vec![SubAttribute {
                attribute: SoulAttribute::Spd,
                value,
                enhancement_count: None,
            }],
        }
    }

    fn hits_of(s: &Soul) -> Hits {
        hits(s, &s.subs[0]).expect("6-star ranges are known")
    }

    #[test]
    fn speed_fourteen_point_eight_admits_five_or_six_hits() {
        assert_eq!(
            hits_of(&spd_soul(15, 14.8)),
            Hits::Admitted { min: 5, max: 6 }
        );
    }

    #[test]
    fn a_value_above_five_times_hi_forces_six_hits() {
        assert_eq!(
            hits_of(&spd_soul(15, 15.1)),
            Hits::Admitted { min: 6, max: 6 }
        );
        assert_eq!(
            hits_of(&spd_soul(15, 18.0)),
            Hits::Admitted { min: 6, max: 6 }
        );
    }

    #[test]
    fn the_level_caps_the_hits() {
        // 14.8 needs five increments at least; at +9 only four are possible.
        assert_eq!(hits_of(&spd_soul(9, 14.8)), Hits::Unreachable);
        assert_eq!(
            hits_of(&spd_soul(0, 2.7)),
            Hits::Admitted { min: 1, max: 1 }
        );
    }

    #[test]
    fn a_value_between_ranges_is_unreachable() {
        // One increment tops out at 3.0 and two start at 4.8.
        assert_eq!(hits_of(&spd_soul(15, 3.5)), Hits::Unreachable);
    }

    #[test]
    fn exact_decimal_bounds_are_admitted_despite_float_error() {
        // 6 × 2.4 is 14.399999999999999 in binary floating point.
        assert!(hits_of(&spd_soul(15, 14.4)).admits(6));
        assert!(hits_of(&spd_soul(15, 2.4 * 6.0)).admits(6));
    }

    #[test]
    fn below_six_stars_the_inference_is_undecided() {
        let mut s = spd_soul(15, 10.0);
        s.star = 5;
        assert_eq!(
            hits(&s, &s.subs[0]),
            Err(Undecided::IncrementRangeUnknown {
                attribute: SoulAttribute::Spd,
                star: 5
            })
        );
    }

    #[test]
    fn a_recorded_count_is_taken_as_given() {
        let mut s = spd_soul(15, 10.0);
        s.subs[0].enhancement_count = Some(2);
        assert_eq!(hits_of(&s), Hits::Known(3));
    }
}
