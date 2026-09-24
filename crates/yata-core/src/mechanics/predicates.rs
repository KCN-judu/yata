//! § Community predicates: 双速, 拉满, 真 n, 顶段.
//!
//! Each predicate presumes a well-formed soul. Where `c(a)` is inferred and ambiguous, a
//! predicate is evaluated on every admitted value (`soul-mechanics.md`, § Inferring roll counts).

use super::Undecided;
use super::inference::{Hits, hits};
use super::values::{VALUE_TOLERANCE, increment_range};
use crate::soul::{Soul, SoulAttribute, SoulSlot};

/// The truth of a predicate over every admitted roll count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Truth {
    /// Holds for every admitted value.
    Certain,
    /// Holds for none. The UI shows nothing.
    Impossible,
    /// Holds for some. The UI shows the estimate marker (`~`).
    Undetermined,
}

impl Truth {
    fn of(b: bool) -> Truth {
        if b { Truth::Certain } else { Truth::Impossible }
    }
}

/// 双速: a Slot 2 soul whose main attribute and a sub-attribute are both `Spd`.
pub fn double_speed(soul: &Soul) -> Truth {
    Truth::of(
        soul.slot == SoulSlot::Slot2
            && soul.main == SoulAttribute::Spd
            && soul.sub(SoulAttribute::Spd).is_some(),
    )
}

/// 拉满: `ℓ = 15`, `a ∈ dom S`, `c(a) = 5` — the attribute was present at +0 and took every roll.
pub fn maxed(soul: &Soul, attribute: SoulAttribute) -> Result<Truth, Undecided> {
    let Some(sub) = soul.sub(attribute) else {
        return Ok(Truth::Impossible);
    };
    if soul.level != 15 {
        return Ok(Truth::Impossible);
    }
    Ok(match hits(soul, sub)? {
        Hits::Known(k) => Truth::of(k == 6),
        Hits::Admitted { min: 6, max: 6 } => Truth::Certain,
        Hits::Admitted { max, .. } if max < 6 => Truth::Impossible,
        Hits::Admitted { .. } => Truth::Undetermined,
        Hits::Unreachable => Truth::Impossible,
    })
}

/// 真 n: `a ∈ dom S` and the stored `S(a) ≥ n`. 真 17 速 is `true_n(s, Spd, 17.0)`; a stored 16.5
/// that displays as 17 does not satisfy it.
pub fn true_n(soul: &Soul, attribute: SoulAttribute, n: f64) -> Truth {
    Truth::of(
        soul.sub(attribute)
            .is_some_and(|s| s.value + VALUE_TOLERANCE >= n),
    )
}

/// 顶段: `maxed(s, a)` and `S(a) ≥ Max(a) − 1`.
pub fn top_band(soul: &Soul, attribute: SoulAttribute) -> Result<Truth, Undecided> {
    let Some(sub) = soul.sub(attribute) else {
        return Ok(Truth::Impossible);
    };
    let range = increment_range(attribute, soul.star).ok_or(Undecided::IncrementRangeUnknown {
        attribute,
        star: soul.star,
    })?;
    if sub.value + VALUE_TOLERANCE < range.max() - 1.0 {
        return Ok(Truth::Impossible);
    }
    maxed(soul, attribute)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soul::SubAttribute;
    use SoulAttribute::*;

    fn soul(slot: SoulSlot, main: SoulAttribute, subs: &[(SoulAttribute, f64)]) -> Soul {
        Soul {
            set: crate::soul::SoulSet::from_suit_code(30),
            slot,
            star: 6,
            level: 15,
            main,
            main_value: 0.0,
            subs: subs
                .iter()
                .map(|&(attribute, value)| SubAttribute {
                    attribute,
                    value,
                    enhancement_count: None,
                })
                .collect(),
        }
    }

    #[test]
    fn double_speed_needs_slot_two_speed_main_and_speed_sub() {
        assert_eq!(
            double_speed(&soul(SoulSlot::Slot2, Spd, &[(Spd, 3.0)])),
            Truth::Certain
        );
        assert_eq!(
            double_speed(&soul(SoulSlot::Slot2, AtkPercent, &[(Spd, 3.0)])),
            Truth::Impossible
        );
        assert_eq!(
            double_speed(&soul(SoulSlot::Slot2, Spd, &[(Crit, 3.0)])),
            Truth::Impossible
        );
    }

    #[test]
    fn maxed_is_undetermined_when_five_or_six_hits_are_admitted() {
        let s = soul(SoulSlot::Slot4, EffectHit, &[(Spd, 14.8)]);
        assert_eq!(maxed(&s, Spd), Ok(Truth::Undetermined));
    }

    #[test]
    fn maxed_is_certain_above_five_times_hi() {
        let s = soul(SoulSlot::Slot4, EffectHit, &[(Spd, 15.5)]);
        assert_eq!(maxed(&s, Spd), Ok(Truth::Certain));
    }

    #[test]
    fn maxed_needs_plus_fifteen() {
        let mut s = soul(SoulSlot::Slot4, EffectHit, &[(Spd, 12.0)]);
        s.level = 12;
        assert_eq!(maxed(&s, Spd), Ok(Truth::Impossible));
    }

    #[test]
    fn a_recorded_count_decides_maxed() {
        let mut s = soul(SoulSlot::Slot4, EffectHit, &[(Spd, 14.8)]);
        s.subs[0].enhancement_count = Some(5);
        assert_eq!(maxed(&s, Spd), Ok(Truth::Certain));
        s.subs[0].enhancement_count = Some(4);
        assert_eq!(maxed(&s, Spd), Ok(Truth::Impossible));
    }

    #[test]
    fn true_seventeen_speed_is_the_stored_value() {
        let shown_17 = soul(SoulSlot::Slot4, EffectHit, &[(Spd, 16.5)]);
        let stored_17 = soul(SoulSlot::Slot4, EffectHit, &[(Spd, 17.0)]);
        assert_eq!(true_n(&shown_17, Spd, 17.0), Truth::Impossible);
        assert_eq!(true_n(&stored_17, Spd, 17.0), Truth::Certain);
    }

    #[test]
    fn top_band_spans_the_last_point_below_the_maximum() {
        let seventeen = soul(SoulSlot::Slot2, Spd, &[(Spd, 17.0)]);
        let sixteen_nine = soul(SoulSlot::Slot2, Spd, &[(Spd, 16.9)]);
        let crit_dmg = soul(SoulSlot::Slot2, Spd, &[(CritDmg, 23.0)]);
        assert_eq!(top_band(&seventeen, Spd), Ok(Truth::Certain));
        assert_eq!(top_band(&sixteen_nine, Spd), Ok(Truth::Impossible));
        assert_eq!(top_band(&crit_dmg, CritDmg), Ok(Truth::Certain));
    }

    #[test]
    fn top_band_is_decidable_from_values_alone_for_every_attribute() {
        // Max − 1 > 5 · hi for every 6★ attribute, so the band forces six hits.
        for a in SoulAttribute::ALL {
            let r = increment_range(a, 6).expect("known at 6 stars");
            let s = soul(SoulSlot::Slot1, AtkFlat, &[(a, r.max() - 1.0)]);
            assert_eq!(top_band(&s, a), Ok(Truth::Certain), "{a:?}");
        }
    }

    #[test]
    fn top_band_below_six_stars_is_undecided() {
        let mut s = soul(SoulSlot::Slot2, Spd, &[(Spd, 17.0)]);
        s.star = 5;
        assert!(top_band(&s, Spd).is_err());
    }
}
