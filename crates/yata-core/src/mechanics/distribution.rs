//! § Roll distribution: the official class weights, P-Four and P-Draw.
//!
//! The weights are the notice's 36% / 36% / 28%, shared equally within a class. The contested
//! reading of 1/11 per attribute is not used (`soul-mechanics.md`, § Roll distribution).

use crate::soul::{RollClass, SoulAttribute};

/// The weight of a roll class, from the official probability notice.
pub fn class_weight(class: RollClass) -> f64 {
    match class {
        RollClass::Attack | RollClass::Defense => 0.36,
        RollClass::Utility => 0.28,
    }
}

/// `w(a)`: the class weight shared equally among the class's members.
pub fn draw_weight(attribute: SoulAttribute) -> f64 {
    let class = attribute.roll_class();
    let members = SoulAttribute::ALL
        .iter()
        .filter(|a| a.roll_class() == class)
        .count();
    class_weight(class) / members as f64
}

/// `q(S)`: the chance one draw lands on an attribute the soul already has.
fn q(attributes: &[SoulAttribute]) -> f64 {
    attributes.iter().map(|&a| draw_weight(a)).sum()
}

/// `1 − q(S)`: the chance a roll below four sub-attributes adds a leg (P-Draw).
pub fn first_roll_adds_leg(attributes: &[SoulAttribute]) -> f64 {
    1.0 - q(attributes)
}

/// `q(S)⁵`: the chance a three-leg soul at +0 is still three-leg at +15.
pub fn stays_three_leg(attributes: &[SoulAttribute; 3]) -> f64 {
    q(attributes).powi(5)
}

/// The same chance as "about one in N", rounded to the nearest whole N.
pub fn stays_three_leg_odds(attributes: &[SoulAttribute; 3]) -> u32 {
    // At most one in a few thousand, so the rounded inverse fits a u32.
    (1.0 / stays_three_leg(attributes)).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use SoulAttribute::*;

    #[test]
    fn the_eleven_draw_weights_sum_to_one() {
        let total: f64 = SoulAttribute::ALL.iter().map(|&a| draw_weight(a)).sum();
        assert!((total - 1.0).abs() < 1e-12);
    }

    #[test]
    fn member_weights_are_nine_percent_or_a_third_of_twenty_eight() {
        assert!((draw_weight(Crit) - 0.09).abs() < 1e-12);
        assert!((draw_weight(HpFlat) - 0.09).abs() < 1e-12);
        assert!((draw_weight(Spd) - 0.28 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn three_leg_odds_match_the_spec_table() {
        // (utility attributes among the three, q(S)⁵ in percent, about one in)
        let cases = [
            ([Crit, CritDmg, HpFlat], 0.143, 697),
            ([Crit, CritDmg, Spd], 0.153, 655),
            ([Crit, EffectHit, Spd], 0.162, 617),
            ([Spd, EffectHit, EffectRes], 0.172, 581),
        ];
        for (legs, percent, one_in) in cases {
            let p = stays_three_leg(&legs) * 100.0;
            assert!((p - percent).abs() < 0.0005, "{legs:?}: {p}");
            assert_eq!(stays_three_leg_odds(&legs), one_in, "{legs:?}");
        }
    }

    #[test]
    fn the_first_roll_adds_the_fourth_leg_about_three_times_in_four() {
        let low = first_roll_adds_leg(&[Spd, EffectHit, EffectRes]);
        let high = first_roll_adds_leg(&[Crit, CritDmg, HpFlat]);
        assert!((0.72..=0.73).contains(&low) && (0.72..=0.73).contains(&high));
    }
}
