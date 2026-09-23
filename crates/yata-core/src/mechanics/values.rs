//! The value tables of `soul-mechanics.md`: § Values and § Main-attribute values.

use crate::soul::SoulAttribute;

/// The tolerance of every comparison between a stored value and a table bound, in display
/// units (`soul-mechanics.md`, "Comparing values"). Stored values are binary floats; the
/// tables are decimals whose smallest step is 0.1.
pub const VALUE_TOLERANCE: f64 = 1e-6;

/// `[lo(a), hi(a)]`: the range of one increment of a sub-attribute at one star. Held in tenths
/// of a display unit, so the table itself is exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IncrementRange {
    lo_tenths: u32,
    hi_tenths: u32,
}

impl IncrementRange {
    pub fn lo(self) -> f64 {
        f64::from(self.lo_tenths) / 10.0
    }

    pub fn hi(self) -> f64 {
        f64::from(self.hi_tenths) / 10.0
    }

    /// `hits · lo`, the least value `hits` increments can sum to.
    pub fn floor_of(self, hits: u8) -> f64 {
        f64::from(u32::from(hits) * self.lo_tenths) / 10.0
    }

    /// `hits · hi`, the greatest value `hits` increments can sum to.
    pub fn ceiling_of(self, hits: u8) -> f64 {
        f64::from(u32::from(hits) * self.hi_tenths) / 10.0
    }

    /// `Max(a) = 6 · hi(a)`.
    pub fn max(self) -> f64 {
        self.ceiling_of(6)
    }
}

const fn range(lo_tenths: u32, hi_tenths: u32) -> IncrementRange {
    IncrementRange {
        lo_tenths,
        hi_tenths,
    }
}

/// `[lo(a), hi(a)]` at a star, or `None` where the spec marks it TBD (every star below 6).
pub fn increment_range(attribute: SoulAttribute, star: u8) -> Option<IncrementRange> {
    use SoulAttribute::*;
    if star != 6 {
        return None;
    }
    Some(match attribute {
        Spd | Crit | AtkPercent | HpPercent | DefPercent => range(24, 30),
        CritDmg | EffectHit | EffectRes => range(32, 40),
        AtkFlat => range(216, 270),
        HpFlat => range(912, 1140),
        DefFlat => range(40, 50),
    })
}

/// M-Main: `main(m, ℓ) = base(m) + ℓ · step(m)` for a 6★ soul, or `None` below 6★ (TBD).
pub fn main_value(attribute: SoulAttribute, star: u8, level: u8) -> Option<f64> {
    use SoulAttribute::*;
    if star != 6 {
        return None;
    }
    let (base, step): (u32, u32) = match attribute {
        AtkFlat => (81, 27),
        DefFlat => (14, 6),
        HpFlat => (342, 114),
        Spd => (12, 3),
        CritDmg => (14, 5),
        AtkPercent | DefPercent | HpPercent | Crit | EffectHit | EffectRes => (10, 3),
    };
    Some(f64::from(base + u32::from(level) * step))
}

#[cfg(test)]
mod tests {
    use super::*;
    use SoulAttribute::*;

    fn six(a: SoulAttribute) -> IncrementRange {
        increment_range(a, 6).expect("every attribute has a 6-star range")
    }

    #[test]
    fn maxima_match_the_values_table() {
        let expected = [
            (Spd, 18.0),
            (Crit, 18.0),
            (AtkPercent, 18.0),
            (HpPercent, 18.0),
            (DefPercent, 18.0),
            (CritDmg, 24.0),
            (EffectHit, 24.0),
            (EffectRes, 24.0),
            (AtkFlat, 162.0),
            (HpFlat, 684.0),
            (DefFlat, 30.0),
        ];
        for (a, max) in expected {
            assert!((six(a).max() - max).abs() < VALUE_TOLERANCE, "{a:?}");
        }
    }

    #[test]
    fn every_six_star_range_has_lo_four_fifths_of_hi() {
        for a in SoulAttribute::ALL {
            let r = six(a);
            assert_eq!(r.lo_tenths * 5, r.hi_tenths * 4, "{a:?}");
        }
    }

    #[test]
    fn ranges_below_six_stars_are_undecided() {
        for star in 1..6 {
            assert_eq!(increment_range(Spd, star), None);
            assert_eq!(main_value(Spd, star, 15), None);
        }
    }

    #[test]
    fn main_values_at_plus_fifteen_match_the_table() {
        let expected = [
            (AtkFlat, 486.0),
            (DefFlat, 104.0),
            (HpFlat, 2052.0),
            (Spd, 57.0),
            (CritDmg, 89.0),
            (Crit, 55.0),
            (AtkPercent, 55.0),
            (EffectRes, 55.0),
        ];
        for (a, v) in expected {
            assert_eq!(main_value(a, 6, 15), Some(v), "{a:?}");
        }
    }

    #[test]
    fn main_values_at_the_nodes_match_the_growth_table() {
        let atk = [81.0, 162.0, 243.0, 324.0, 405.0, 486.0];
        for (i, v) in atk.into_iter().enumerate() {
            let level = u8::try_from(i * 3).expect("small");
            assert_eq!(main_value(AtkFlat, 6, level), Some(v));
        }
        assert_eq!(main_value(CritDmg, 6, 0), Some(14.0));
        assert_eq!(main_value(CritDmg, 6, 9), Some(59.0));
    }
}
