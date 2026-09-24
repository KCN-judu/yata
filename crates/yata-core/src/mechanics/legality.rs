//! § Well-formed souls: W-Soul, and the M-Main check of § Main-attribute values.

use super::Undecided;
use super::inference::{Hits, hits};
use super::values::{VALUE_TOLERANCE, increment_range, main_value};
use crate::soul::{Soul, SoulAttribute, SoulSlot};

/// Why a soul is not well-formed. Each variant carries the values that broke the rule.
#[derive(Debug, Clone, PartialEq)]
pub enum Violation {
    StarOutOfRange {
        star: u8,
    },
    LevelOutOfRange {
        level: u8,
    },
    MainNotAllowed {
        slot: SoulSlot,
        main: SoulAttribute,
    },
    TooManySubAttributes {
        count: usize,
    },
    DuplicateSubAttribute {
        attribute: SoulAttribute,
    },
    /// A stored value that is negative, infinite, or not a number.
    InvalidValue {
        attribute: SoulAttribute,
        value: f64,
    },
    /// A recorded `c(a)` whose `hits · [lo, hi]` does not contain the value.
    ValueOutsideRange {
        attribute: SoulAttribute,
        value: f64,
        hits: u8,
        least: f64,
        most: f64,
    },
    /// No number of increments the level allows sums to the value.
    ValueUnreachable {
        attribute: SoulAttribute,
        value: f64,
        level: u8,
    },
    /// `Σ c(a) > nodes(ℓ)`: more rolls than the level has had. `rolls` is a lower bound when
    /// some `c(a)` are inferred.
    RollsExceedNodes {
        rolls: u32,
        nodes: u8,
    },
}

/// Something worth reporting that does not make the soul malformed.
#[derive(Debug, Clone, PartialEq)]
pub enum Warning {
    /// A 6★ main value that differs from M-Main. Reported, not rejected, until recordings show
    /// how the game stores it (`soul-mechanics.md`, § Main-attribute values).
    MainValueMismatch {
        main: SoulAttribute,
        expected: f64,
        actual: f64,
    },
}

/// `⊢ s ok`, as far as the rules can decide it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    WellFormed,
    Malformed,
    /// No rule is broken, but one could not be evaluated for want of a TBD value.
    Undecidable,
}

/// Every violation, every rule that could not be evaluated, and every warning, for one soul.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Assessment {
    pub violations: Vec<Violation>,
    pub undecided: Vec<Undecided>,
    pub warnings: Vec<Warning>,
}

impl Assessment {
    pub fn verdict(&self) -> Verdict {
        if !self.violations.is_empty() {
            Verdict::Malformed
        } else if !self.undecided.is_empty() {
            Verdict::Undecidable
        } else {
            Verdict::WellFormed
        }
    }
}

/// W-Soul for one soul. Every rule is evaluated, so one reading reports all of its problems.
pub fn assess(soul: &Soul) -> Assessment {
    let mut out = Assessment::default();
    let v = &mut out.violations;
    if !(1..=6).contains(&soul.star) {
        v.push(Violation::StarOutOfRange { star: soul.star });
    }
    if soul.level > 15 {
        v.push(Violation::LevelOutOfRange { level: soul.level });
    }
    if !soul.slot.main_options().contains(&soul.main) {
        v.push(Violation::MainNotAllowed {
            slot: soul.slot,
            main: soul.main,
        });
    }
    if soul.subs.len() > 4 {
        v.push(Violation::TooManySubAttributes {
            count: soul.subs.len(),
        });
    }
    for (i, sub) in soul.subs.iter().enumerate() {
        if soul.subs[..i].iter().any(|s| s.attribute == sub.attribute) {
            v.push(Violation::DuplicateSubAttribute {
                attribute: sub.attribute,
            });
        }
    }
    if !out.violations.is_empty() {
        // The value rules presume a star, a level and a sub-attribute set that can exist.
        return out;
    }
    let mut rolls: u32 = 0;
    for sub in &soul.subs {
        if !sub.value.is_finite() || sub.value < 0.0 {
            out.violations.push(Violation::InvalidValue {
                attribute: sub.attribute,
                value: sub.value,
            });
            continue;
        }
        match hits(soul, sub) {
            Err(u) => out.undecided.push(u),
            Ok(Hits::Unreachable) => out.violations.push(Violation::ValueUnreachable {
                attribute: sub.attribute,
                value: sub.value,
                level: soul.level,
            }),
            Ok(h @ Hits::Admitted { .. }) => rolls += u32::from(h.least_rolls().unwrap_or(0)),
            Ok(Hits::Known(k)) => {
                rolls += u32::from(k - 1);
                check_recorded(soul, sub.attribute, sub.value, k, &mut out);
            }
        }
    }
    let nodes = soul.nodes();
    if rolls > u32::from(nodes) {
        out.violations
            .push(Violation::RollsExceedNodes { rolls, nodes });
    }
    if let Some(expected) = main_value(soul.main, soul.star, soul.level)
        && (soul.main_value - expected).abs() > VALUE_TOLERANCE
    {
        out.warnings.push(Warning::MainValueMismatch {
            main: soul.main,
            expected,
            actual: soul.main_value,
        });
    }
    out
}

/// A recorded `c(a)` must put the value inside `hits · [lo, hi]`.
fn check_recorded(soul: &Soul, attribute: SoulAttribute, value: f64, k: u8, out: &mut Assessment) {
    let Some(range) = increment_range(attribute, soul.star) else {
        out.undecided.push(Undecided::IncrementRangeUnknown {
            attribute,
            star: soul.star,
        });
        return;
    };
    let (least, most) = (range.floor_of(k), range.ceiling_of(k));
    if value < least - VALUE_TOLERANCE || value > most + VALUE_TOLERANCE {
        out.violations.push(Violation::ValueOutsideRange {
            attribute,
            value,
            hits: k,
            least,
            most,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soul::SubAttribute;
    use SoulAttribute::*;

    fn sub(attribute: SoulAttribute, value: f64) -> SubAttribute {
        SubAttribute {
            attribute,
            value,
            enhancement_count: None,
        }
    }

    /// A 6★ +15 Slot 2 speed soul with four plausible legs.
    fn soul() -> Soul {
        Soul {
            set: crate::soul::SoulSet::from_suit_code(30),
            slot: SoulSlot::Slot2,
            star: 6,
            level: 15,
            main: Spd,
            main_value: 57.0,
            subs: vec![
                sub(Spd, 14.8),
                sub(Crit, 2.7),
                sub(AtkPercent, 3.0),
                sub(EffectHit, 3.5),
            ],
            innate: crate::soul::Innate::Absent,
        }
    }

    #[test]
    fn a_plausible_soul_is_well_formed() {
        let a = assess(&soul());
        assert_eq!(a, Assessment::default());
        assert_eq!(a.verdict(), Verdict::WellFormed);
    }

    #[test]
    fn a_sub_attribute_may_equal_the_main_attribute() {
        // 双速: nothing relates m to dom S.
        assert!(soul().sub(Spd).is_some());
        assert_eq!(assess(&soul()).verdict(), Verdict::WellFormed);
    }

    #[test]
    fn shape_violations_are_all_reported() {
        let mut s = soul();
        s.star = 7;
        s.level = 16;
        s.slot = SoulSlot::Slot1;
        s.subs.push(sub(Crit, 2.5));
        let v = assess(&s).violations;
        assert!(v.contains(&Violation::StarOutOfRange { star: 7 }));
        assert!(v.contains(&Violation::LevelOutOfRange { level: 16 }));
        assert!(v.contains(&Violation::MainNotAllowed {
            slot: SoulSlot::Slot1,
            main: Spd
        }));
        assert!(v.contains(&Violation::TooManySubAttributes { count: 5 }));
        assert!(v.contains(&Violation::DuplicateSubAttribute { attribute: Crit }));
    }

    #[test]
    fn more_inferred_rolls_than_nodes_is_malformed() {
        // Speed needs at least four rolls and crit at least two: six rolls, five nodes.
        let mut s = soul();
        s.subs = vec![sub(Spd, 14.8), sub(Crit, 8.0)];
        assert_eq!(
            assess(&s).violations,
            vec![Violation::RollsExceedNodes { rolls: 6, nodes: 5 }]
        );
    }

    #[test]
    fn a_value_no_roll_count_reaches_is_malformed() {
        let mut s = soul();
        s.level = 0;
        s.main_value = 12.0;
        s.subs = vec![sub(Spd, 4.0)];
        assert_eq!(assess(&s).verdict(), Verdict::Malformed);
    }

    #[test]
    fn a_recorded_count_must_agree_with_the_value() {
        let mut s = soul();
        s.subs[0].enhancement_count = Some(1); // two increments reach at most 6.0
        assert!(matches!(
            assess(&s).violations[..],
            [Violation::ValueOutsideRange { hits: 2, .. }]
        ));
    }

    #[test]
    fn invalid_values_are_malformed() {
        let mut s = soul();
        s.subs[1].value = f64::NAN;
        s.subs[2].value = -1.0;
        assert_eq!(assess(&s).violations.len(), 2);
    }

    #[test]
    fn below_six_stars_value_rules_are_undecidable() {
        let mut s = soul();
        s.star = 5;
        let a = assess(&s);
        assert_eq!(a.verdict(), Verdict::Undecidable);
        assert_eq!(a.undecided.len(), 4);
        assert!(a.warnings.is_empty(), "no main-value check below 6 stars");
    }

    #[test]
    fn a_main_value_off_the_growth_line_is_a_warning_only() {
        let mut s = soul();
        s.main_value = 56.0;
        let a = assess(&s);
        assert_eq!(a.verdict(), Verdict::WellFormed);
        assert_eq!(
            a.warnings,
            vec![Warning::MainValueMismatch {
                main: Spd,
                expected: 57.0,
                actual: 56.0
            }]
        );
    }
}
