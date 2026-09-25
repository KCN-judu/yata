//! § Inferring roll counts: I-Hits.

use super::Undecided;
use super::values::{VALUE_TOLERANCE, increment_range};
use crate::soul::{RollCount, Soul, SubAttribute};

/// `hits(a) = 1 + c(a)`: how many increments a sub-attribute's value is the sum of. At least one,
/// the increment it appeared with; held as the `c(a)` it is one more than, so both directions are
/// exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HitCount(RollCount);

impl HitCount {
    /// `h` increments, or `None` unless `1 ≤ h ≤ 256`.
    pub fn new(h: u16) -> Option<HitCount> {
        let rolls = u8::try_from(h.checked_sub(1)?).ok()?;
        Some(HitCount(RollCount::new(rolls)))
    }

    /// `1 + c(a)`.
    pub fn after(rolls: RollCount) -> HitCount {
        HitCount(rolls)
    }

    /// `hits(a) − 1 = c(a)`.
    pub fn rolls(self) -> RollCount {
        self.0
    }

    pub fn get(self) -> u16 {
        u16::from(self.0.get()) + 1
    }
}

/// The hit counts I-Hits admits: `min..=max`, with `1 ≤ min ≤ max`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitRange {
    min: HitCount,
    max: HitCount,
}

impl HitRange {
    /// `min..=max`, or `None` when `min > max`.
    pub fn new(min: HitCount, max: HitCount) -> Option<HitRange> {
        (min <= max).then_some(HitRange { min, max })
    }

    pub fn min(self) -> HitCount {
        self.min
    }

    pub fn max(self) -> HitCount {
        self.max
    }

    pub fn contains(self, h: HitCount) -> bool {
        (self.min..=self.max).contains(&h)
    }
}

/// What is known of `hits(a)` for one sub-attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hits {
    /// The reading carries `c(a)`, so `hits(a)` is `1 + c(a)`.
    Known(HitCount),
    /// Every count in the range is admitted by I-Hits and by the soul's level; `c(a)` is
    /// ambiguous when the range holds more than one.
    Admitted(HitRange),
    /// No number of increments reachable at this level sums to the value. Only a malformed soul
    /// has this.
    Unreachable,
}

impl Hits {
    /// Whether `h` increments are possible for this sub-attribute.
    pub fn admits(self, h: HitCount) -> bool {
        match self {
            Hits::Known(k) => k == h,
            Hits::Admitted(range) => range.contains(h),
            Hits::Unreachable => false,
        }
    }

    /// The least number of rolls, `min hits − 1`, or `None` if unreachable.
    pub fn least_rolls(self) -> Option<RollCount> {
        match self {
            Hits::Known(k) => Some(k.rolls()),
            Hits::Admitted(range) => Some(range.min().rolls()),
            Hits::Unreachable => None,
        }
    }
}

/// I-Hits: `⌈S(a) / hi(a)⌉ ≤ hits(a) ≤ ⌊S(a) / lo(a)⌋`, intersected with what the level allows,
/// `1 ≤ hits(a) ≤ 1 + nodes(ℓ)`. A recorded `c(a)` is taken as given; whether it agrees with
/// the value is legality's question, not this one's.
pub fn hits(soul: &Soul, sub: &SubAttribute) -> Result<Hits, Undecided> {
    if let Some(c) = sub.enhancement_count {
        return Ok(Hits::Known(HitCount::after(c)));
    }
    let range =
        increment_range(sub.attribute, soul.star).ok_or(Undecided::IncrementRangeUnknown {
            attribute: sub.attribute,
            star: soul.star,
        })?;
    let least = ((sub.value.get() - VALUE_TOLERANCE) / range.hi())
        .ceil()
        .max(1.0);
    let most = ((sub.value.get() + VALUE_TOLERANCE) / range.lo())
        .floor()
        .min(f64::from(1 + soul.nodes()));
    // Both bounds lie in 1..=6 when finite and ordered, so the conversions cannot truncate; a
    // NaN or an empty interval leaves no admitted count.
    let count = |x: f64| {
        (x.is_finite() && x >= 1.0)
            .then(|| HitCount::new(x as u16))
            .flatten()
    };
    Ok(match (count(least), count(most)) {
        (Some(min), Some(max)) => HitRange::new(min, max).map_or(Hits::Unreachable, Hits::Admitted),
        _ => Hits::Unreachable,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soul::{Level, SoulAttribute, SoulSlot, Star, StoredValue};

    fn spd_soul(level: u8, value: f64) -> Soul {
        let level = Level::new(level).expect("a level");
        Soul {
            set: crate::soul::SoulSet::from_suit_code(30),
            slot: SoulSlot::Slot2,
            star: Star::Six,
            level,
            main: SoulAttribute::Spd,
            main_value: StoredValue::new(57.0).expect("stored"),
            subs: vec![SubAttribute {
                attribute: SoulAttribute::Spd,
                value: StoredValue::new(value).expect("stored"),
                enhancement_count: None,
            }],
            kind: crate::soul::SoulKind::Ordinary,
        }
    }

    fn hit(h: u16) -> HitCount {
        HitCount::new(h).expect("at least one")
    }

    fn admitted(min: u16, max: u16) -> Hits {
        Hits::Admitted(HitRange::new(hit(min), hit(max)).expect("ordered"))
    }

    fn hits_of(s: &Soul) -> Hits {
        hits(s, &s.subs[0]).expect("6-star ranges are known")
    }

    #[test]
    fn speed_fourteen_point_eight_admits_five_or_six_hits() {
        assert_eq!(hits_of(&spd_soul(15, 14.8)), admitted(5, 6));
    }

    #[test]
    fn a_value_above_five_times_hi_forces_six_hits() {
        assert_eq!(hits_of(&spd_soul(15, 15.1)), admitted(6, 6));
        assert_eq!(hits_of(&spd_soul(15, 18.0)), admitted(6, 6));
    }

    #[test]
    fn the_level_caps_the_hits() {
        // 14.8 needs five increments at least; at +9 only four are possible.
        assert_eq!(hits_of(&spd_soul(9, 14.8)), Hits::Unreachable);
        assert_eq!(hits_of(&spd_soul(0, 2.7)), admitted(1, 1));
    }

    #[test]
    fn a_value_between_ranges_is_unreachable() {
        // One increment tops out at 3.0 and two start at 4.8.
        assert_eq!(hits_of(&spd_soul(15, 3.5)), Hits::Unreachable);
    }

    #[test]
    fn exact_decimal_bounds_are_admitted_despite_float_error() {
        // 6 × 2.4 is 14.399999999999999 in binary floating point.
        assert!(hits_of(&spd_soul(15, 14.4)).admits(hit(6)));
        assert!(hits_of(&spd_soul(15, 2.4 * 6.0)).admits(hit(6)));
    }

    #[test]
    fn below_six_stars_the_inference_is_undecided() {
        let mut s = spd_soul(15, 10.0);
        s.star = Star::Five;
        assert_eq!(
            hits(&s, &s.subs[0]),
            Err(Undecided::IncrementRangeUnknown {
                attribute: SoulAttribute::Spd,
                star: Star::Five
            })
        );
    }

    #[test]
    fn a_hit_count_is_one_more_than_its_rolls_both_ways() {
        for c in [0, 1, 5, u8::MAX] {
            let h = HitCount::after(RollCount::new(c));
            assert_eq!(h.get(), u16::from(c) + 1);
            assert_eq!(h.rolls(), RollCount::new(c));
            assert_eq!(HitCount::new(h.get()), Some(h));
        }
        assert_eq!(HitCount::new(0), None);
        assert_eq!(HitCount::new(257), None);
    }

    #[test]
    fn a_hit_range_is_ordered() {
        assert!(HitRange::new(hit(2), hit(1)).is_none());
        let r = HitRange::new(hit(1), hit(1)).expect("one count");
        assert_eq!(Hits::Admitted(r).least_rolls(), Some(RollCount::new(0)));
        assert_eq!(Hits::Unreachable.least_rolls(), None);
    }

    #[test]
    fn a_recorded_count_is_taken_as_given() {
        let mut s = spd_soul(15, 10.0);
        s.subs[0].enhancement_count = Some(RollCount::new(2));
        assert_eq!(hits_of(&s), Hits::Known(hit(3)));
    }
}
