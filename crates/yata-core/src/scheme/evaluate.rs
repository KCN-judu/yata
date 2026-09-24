//! Whether the game's filter picks a soul: `matches` of `scheme-code.md`, "Evaluation".
//!
//! This is the game's official filter and nothing else: no score, no affinity, no UI or store
//! state, no header. It decides only what the evidence decides. A group whose rule is still open
//! does not guess; it makes the verdict [`Verdict::Undetermined`] and names the open rule, unless
//! another group already rules the soul out.
//!
//! The rules, from the game's filter experiments of 2026-09-24 (`scheme-code.md`):
//!
//! - groups combine by AND: a soul is picked only if every group picks it
//! - a group with nothing chosen is no constraint (observed for 等级, 数量 and a disabled
//!   固有属性; extrapolated to the others)
//! - 类型, 位置, 星级, 等级, 主属性: the soul's value is among the choices; `AnySet` picks every soul
//! - 副属性 ○: the soul has every included attribute; ✕: it has none of the excluded ones
//! - 数量: the soul's number of sub-attributes, all of them, is among the choices
//!
//! And from the maintainer's filter observations of 2026-09-25:
//!
//! - 固有属性, with the soul's set itself chosen: a soul without an innate attribute passes; a
//!   boss soul passes when its innate attribute is chosen
//!
//! Open, each an [`OpenRule`]: a chosen 固有属性 where the soul's innate attribute is unknown, or
//! where a boss soul's set is not itself chosen; and a condition the model cannot see.

use crate::soul::{Innate, Soul};

use super::code::{DiscardScheme, StrengtheningPlan};
use super::selection::{
    InnateAttribute, LevelBand, SetChoice, SoulSelection, SubAttributeMode, SubCount,
};

/// What the game's filter does with a soul.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The game picks it.
    Matches,
    /// The game does not pick it: a decided group rules it out.
    DoesNotMatch,
    /// No decided group rules it out, and the outcome rests on rules still open.
    Undetermined(Vec<OpenRule>),
}

/// A rule of the game's filter that the evidence does not settle yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpenRule {
    /// A chosen 固有属性 that the evidence does not decide for this soul: its innate attribute is
    /// [`Innate::Unknown`], or it is a boss soul whose set is not itself chosen and whose innate
    /// attribute is not.
    Innate,
    /// The scheme selects on bits the model does not map.
    UnknownConditions,
}

/// Whether the game's filter of `selection` picks `soul`, as far as the evidence decides.
pub fn matches(selection: &SoulSelection, soul: &Soul) -> Verdict {
    let s = selection;
    // A group picks the soul when nothing is chosen in it, or when the soul's value is chosen.
    let picks = |empty: bool, chosen: bool| empty || chosen;
    let sets = match &s.sets {
        SetChoice::AnySet => true,
        SetChoice::Sets(sets) => picks(sets.is_empty(), sets.contains(&soul.set)),
    };
    let has = |a| soul.sub(a).is_some();
    let picked = sets
        && picks(s.slots.is_empty(), s.slots.contains(&soul.slot))
        && picks(s.stars.is_empty(), s.stars.contains(&soul.star))
        && picks(
            s.levels.is_empty(),
            LevelBand::of(soul.level).is_some_and(|b| s.levels.contains(&b)),
        )
        && picks(
            s.main_attributes.is_empty(),
            s.main_attributes.contains(&soul.main),
        )
        && s.sub_attributes.with(SubAttributeMode::Include).all(has)
        && !s.sub_attributes.with(SubAttributeMode::Exclude).any(has)
        && picks(
            s.sub_counts.is_empty(),
            SubCount::of(soul.subs.len()).is_some_and(|c| s.sub_counts.contains(&c)),
        );
    if !picked {
        return Verdict::DoesNotMatch;
    }
    match innate_picks(s, soul) {
        Some(true) => Verdict::Matches,
        Some(false) => Verdict::DoesNotMatch,
        None => Verdict::Undetermined(vec![OpenRule::Innate]),
    }
}

/// Whether the 固有属性 group picks `soul`; `None` where the evidence does not decide it.
///
/// Observed with boss and ordinary souls chosen together in 类型: ordinary souls all pass, and a
/// boss soul passes exactly when its innate attribute is chosen. The editor enables the group
/// only once a boss soul is chosen, so a boss soul whose set is not itself chosen (`AnySet`) is
/// not observed; it is open unless its innate attribute is chosen, when either reading picks it.
fn innate_picks(s: &SoulSelection, soul: &Soul) -> Option<bool> {
    if s.innate.is_empty() {
        return Some(true);
    }
    let set_chosen = matches!(&s.sets, SetChoice::Sets(sets) if sets.contains(&soul.set));
    match soul.innate {
        Innate::Absent => Some(true),
        Innate::Present(a) if InnateAttribute::new(a).is_some_and(|i| s.innate.contains(&i)) => {
            Some(true)
        }
        Innate::Present(_) if set_chosen => Some(false),
        Innate::Present(_) | Innate::Unknown => None,
    }
}

/// [`matches`], and never exact while the entry has unknown conditions.
fn with_unknown(verdict: Verdict, unknown: bool) -> Verdict {
    match verdict {
        Verdict::Matches if unknown => Verdict::Undetermined(vec![OpenRule::UnknownConditions]),
        Verdict::Undetermined(mut open) if unknown => {
            open.push(OpenRule::UnknownConditions);
            open.sort_unstable();
            Verdict::Undetermined(open)
        }
        v => v,
    }
}

impl StrengtheningPlan {
    /// Whether this plan picks `soul`; see [`matches`].
    pub fn matches(&self, soul: &Soul) -> Verdict {
        with_unknown(
            matches(&self.selection, soul),
            self.has_unknown_conditions(),
        )
    }
}

impl DiscardScheme {
    /// Whether this scheme picks `soul`; see [`matches`].
    pub fn matches(&self, soul: &Soul) -> Verdict {
        with_unknown(
            matches(&self.selection, soul),
            self.has_unknown_conditions(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::super::layout::{Record, SchemeKind};
    use super::super::mapping::soul_set;
    use super::super::selection::decode_selection;
    use super::*;
    use crate::soul::{SoulAttribute, SoulSet, SoulSlot, SubAttribute};

    use SoulAttribute::*;

    /// A 破势 (30) soul with main attribute `Spd` and the given sub-attributes.
    fn with_subs(slot: SoulSlot, star: u8, level: u8, subs: &[SoulAttribute]) -> Soul {
        Soul {
            set: SoulSet::from_suit_code(30),
            slot,
            star,
            level,
            main: Spd,
            main_value: 57.0,
            subs: subs
                .iter()
                .map(|&attribute| SubAttribute {
                    attribute,
                    value: 1.0,
                    enhancement_count: None,
                })
                .collect(),
            innate: Innate::Absent,
        }
    }

    /// 破势, slot 2, 6★, +15, main Spd; subs Crit, CritDmg, AtkPercent.
    fn soul() -> Soul {
        with_subs(SoulSlot::Slot2, 6, 15, &[Crit, CritDmg, AtkPercent])
    }

    /// Every group chosen so that `soul()` passes it; 数量 and 固有属性 left empty.
    fn passing() -> SoulSelection {
        let mut s = SoulSelection::new(SetChoice::Sets(BTreeSet::from([
            SoulSet::from_suit_code(30),
            SoulSet::from_suit_code(10),
        ])));
        s.slots = BTreeSet::from([SoulSlot::Slot2, SoulSlot::Slot4]);
        s.stars = BTreeSet::from([6]);
        s.levels = BTreeSet::from([LevelBand::L15]);
        s.main_attributes = BTreeSet::from([Spd]);
        s
    }

    #[test]
    fn a_soul_every_group_picks_matches() {
        assert_eq!(matches(&passing(), &soul()), Verdict::Matches);
    }

    #[test]
    fn each_group_rules_a_soul_out() {
        let s = soul();
        let mut out = Vec::new();
        let mut sel = passing();
        sel.sets = SetChoice::Sets(BTreeSet::from([SoulSet::from_suit_code(36)]));
        out.push(sel);
        let mut sel = passing();
        sel.slots = BTreeSet::from([SoulSlot::Slot1]);
        out.push(sel);
        let mut sel = passing();
        sel.stars = BTreeSet::from([5]);
        out.push(sel);
        let mut sel = passing();
        sel.levels = BTreeSet::from([LevelBand::L12to14]);
        out.push(sel);
        let mut sel = passing();
        sel.main_attributes = BTreeSet::from([AtkPercent]);
        out.push(sel);
        let mut sel = passing();
        sel.sub_attributes.set(Crit, SubAttributeMode::Exclude);
        out.push(sel);
        let mut sel = passing();
        sel.sub_attributes.set(EffectHit, SubAttributeMode::Include);
        out.push(sel);
        let mut sel = passing();
        sel.sub_counts = BTreeSet::from([SubCount::Four]);
        out.push(sel);
        for sel in out {
            assert_eq!(matches(&sel, &s), Verdict::DoesNotMatch, "{sel:?}");
        }
    }

    #[test]
    fn an_empty_group_is_no_constraint() {
        let everything_empty = SoulSelection::new(SetChoice::Sets(BTreeSet::new()));
        assert_eq!(matches(&everything_empty, &soul()), Verdict::Matches);
        let mut beyond_every_band = soul();
        beyond_every_band.level = 16;
        let mut no_levels = passing();
        no_levels.levels.clear();
        assert_eq!(matches(&no_levels, &beyond_every_band), Verdict::Matches);
    }

    #[test]
    fn several_includes_require_every_one() {
        let mut both_present = passing();
        both_present
            .sub_attributes
            .set(Crit, SubAttributeMode::Include);
        both_present
            .sub_attributes
            .set(CritDmg, SubAttributeMode::Include);
        assert_eq!(matches(&both_present, &soul()), Verdict::Matches);
        let mut one_missing = passing();
        one_missing
            .sub_attributes
            .set(Crit, SubAttributeMode::Include);
        one_missing
            .sub_attributes
            .set(Spd, SubAttributeMode::Include);
        assert_eq!(matches(&one_missing, &soul()), Verdict::DoesNotMatch);
    }

    #[test]
    fn a_count_is_the_number_of_all_sub_attributes() {
        let slot = SoulSlot::Slot2;
        let by_count = [
            (vec![], SubCount::FewerThanTwo),
            (vec![Crit], SubCount::FewerThanTwo),
            (vec![Crit, CritDmg], SubCount::Two),
            (vec![Crit, CritDmg, AtkPercent], SubCount::Three),
            (vec![Crit, CritDmg, AtkPercent, HpPercent], SubCount::Four),
        ];
        for (subs, count) in &by_count {
            let s = with_subs(slot, 6, 15, subs);
            for other in SubCount::ALL {
                let mut sel = passing();
                sel.sub_counts = BTreeSet::from([other]);
                let expected = if other == *count {
                    Verdict::Matches
                } else {
                    Verdict::DoesNotMatch
                };
                assert_eq!(matches(&sel, &s), expected, "{subs:?} {other:?}");
            }
        }
        let mut several = passing();
        several.sub_counts = BTreeSet::from([SubCount::Three, SubCount::Four]);
        assert_eq!(matches(&several, &soul()), Verdict::Matches);
        let five = with_subs(slot, 6, 15, &[Crit, CritDmg, AtkPercent, HpPercent, Spd]);
        let mut any_count = passing();
        any_count.sub_counts = SubCount::ALL.into_iter().collect();
        assert_eq!(matches(&any_count, &five), Verdict::DoesNotMatch);
    }

    #[test]
    fn the_filter_experiments_read_as_the_game_showed_them() {
        // The five plans of the 2026-09-24 filter experiment (local research), over one pool:
        // all souls, slot 1, 6★, every level, main AtkFlat; and the maintainer's observations.
        let mut pool = SoulSelection::new(SetChoice::AnySet);
        pool.slots = BTreeSet::from([SoulSlot::Slot1]);
        pool.stars = BTreeSet::from([6]);
        pool.levels = LevelBand::ALL.into_iter().collect();
        pool.main_attributes = BTreeSet::from([AtkFlat]);
        let mut empty_levels = pool.clone();
        empty_levels.levels.clear();
        let mut two_includes = pool.clone();
        two_includes
            .sub_attributes
            .set(Spd, SubAttributeMode::Include);
        two_includes
            .sub_attributes
            .set(Crit, SubAttributeMode::Include);
        let mut includes_count = two_includes.clone();
        includes_count.sub_counts = BTreeSet::from([SubCount::Two]);
        let mut count_alone = pool.clone();
        count_alone.sub_counts = BTreeSet::from([SubCount::Four]);

        let slot1 = |level, subs: &[SoulAttribute]| Soul {
            main: AtkFlat,
            ..with_subs(SoulSlot::Slot1, 6, level, subs)
        };
        let both_four = slot1(15, &[Spd, Crit, CritDmg, AtkPercent]);
        let spd_only = slot1(15, &[Spd, CritDmg, AtkPercent, HpPercent]);
        let crit_only = slot1(15, &[Crit, CritDmg, AtkPercent, HpPercent]);
        let neither_two = slot1(0, &[AtkPercent, DefPercent]);
        let neither_three = slot1(0, &[AtkPercent, DefPercent, HpPercent]);
        let all = [
            &both_four,
            &spd_only,
            &crit_only,
            &neither_two,
            &neither_three,
        ];
        let picks = |sel: &SoulSelection, s: &Soul| matches(sel, s) == Verdict::Matches;

        // 基础对照 picks the pool; 空组测试 picks the same souls.
        for s in all {
            assert!(picks(&pool, s));
            assert!(picks(&empty_levels, s));
        }
        // 双含测试: only souls with both.
        assert!(picks(&two_includes, &both_four));
        assert!(!picks(&two_includes, &spd_only));
        assert!(!picks(&two_includes, &crit_only));
        // 数量测试: a soul with both and four sub-attributes is not picked.
        assert!(!picks(&includes_count, &both_four));
        // 数量无含: only souls with four sub-attributes.
        assert!(picks(&count_alone, &both_four));
        assert!(picks(&count_alone, &spd_only));
        assert!(!picks(&count_alone, &neither_two));
        assert!(!picks(&count_alone, &neither_three));
    }

    /// 土蜘蛛 (soul bit 33), a boss soul.
    fn boss_set() -> SoulSet {
        soul_set(33).expect("mapped")
    }

    /// `soul()` as a 土蜘蛛 with innate attribute `innate`.
    fn boss(innate: Innate) -> Soul {
        Soul {
            set: boss_set(),
            innate,
            ..soul()
        }
    }

    /// `passing()` with 土蜘蛛 and 破势 chosen together, as the maintainer did, and 固有属性 `chosen`.
    fn with_innate(chosen: &[SoulAttribute]) -> SoulSelection {
        let mut sel = passing();
        sel.sets = SetChoice::Sets(BTreeSet::from([boss_set(), SoulSet::from_suit_code(30)]));
        sel.innate = chosen
            .iter()
            .map(|&a| InnateAttribute::new(a).expect("an innate attribute"))
            .collect();
        sel
    }

    #[test]
    fn a_boss_soul_passes_exactly_when_its_innate_attribute_is_chosen() {
        let crit = boss(Innate::Present(Crit));
        assert_eq!(matches(&with_innate(&[Crit]), &crit), Verdict::Matches);
        assert_eq!(
            matches(&with_innate(&[AtkPercent]), &crit),
            Verdict::DoesNotMatch
        );
        let every = InnateAttribute::ALL.map(InnateAttribute::attribute);
        for a in every {
            let others: Vec<_> = every.into_iter().filter(|&o| o != a).collect();
            let s = boss(Innate::Present(a));
            assert_eq!(matches(&with_innate(&every), &s), Verdict::Matches);
            assert_eq!(matches(&with_innate(&others), &s), Verdict::DoesNotMatch);
        }
    }

    #[test]
    fn a_soul_without_an_innate_attribute_passes_a_chosen_one() {
        for i in InnateAttribute::ALL {
            assert_eq!(
                matches(&with_innate(&[i.attribute()]), &soul()),
                Verdict::Matches
            );
        }
    }

    #[test]
    fn an_empty_innate_group_is_no_constraint() {
        let sel = with_innate(&[]);
        for innate in [Innate::Absent, Innate::Present(Crit), Innate::Unknown] {
            assert_eq!(matches(&sel, &boss(innate)), Verdict::Matches, "{innate:?}");
        }
    }

    #[test]
    fn an_unknown_innate_attribute_is_open() {
        let open = Verdict::Undetermined(vec![OpenRule::Innate]);
        assert_eq!(matches(&with_innate(&[Crit]), &boss(Innate::Unknown)), open);
        let mut ordinary = soul();
        ordinary.innate = Innate::Unknown;
        assert_eq!(matches(&with_innate(&[Crit]), &ordinary), open);
    }

    #[test]
    fn a_boss_soul_whose_set_is_not_chosen_is_open_unless_its_innate_attribute_is() {
        let mut sel = with_innate(&[Crit]);
        sel.sets = SetChoice::AnySet;
        assert_eq!(
            matches(&sel, &boss(Innate::Present(Crit))),
            Verdict::Matches
        );
        assert_eq!(
            matches(&sel, &boss(Innate::Present(AtkPercent))),
            Verdict::Undetermined(vec![OpenRule::Innate])
        );
        assert_eq!(matches(&sel, &soul()), Verdict::Matches);
    }

    #[test]
    fn the_sub_attribute_group_does_not_see_the_innate_attribute() {
        // 2026-09-25: 副属性 X ○ leaves out a boss soul whose X is only its innate attribute, and
        // picks one that has X as both.
        let mut sel = with_innate(&[]);
        sel.sub_attributes.set(EffectHit, SubAttributeMode::Include);
        let innate_only = boss(Innate::Present(EffectHit));
        assert_eq!(matches(&sel, &innate_only), Verdict::DoesNotMatch);
        let mut both = innate_only.clone();
        both.subs.push(SubAttribute {
            attribute: EffectHit,
            value: 1.0,
            enhancement_count: None,
        });
        assert_eq!(matches(&sel, &both), Verdict::Matches);
    }

    #[test]
    fn a_decided_failure_wins_over_an_open_innate_choice() {
        let mut sel = with_innate(&[Crit]);
        sel.stars = BTreeSet::from([4]);
        assert_eq!(matches(&sel, &boss(Innate::Unknown)), Verdict::DoesNotMatch);
        let mut any = sel.clone();
        any.sets = SetChoice::AnySet;
        assert_eq!(
            matches(&any, &boss(Innate::Present(AtkPercent))),
            Verdict::DoesNotMatch
        );
    }

    #[test]
    fn an_unchosen_innate_attribute_fails_with_every_other_group() {
        let mut sel = with_innate(&[AtkPercent]);
        sel.slots = BTreeSet::from([SoulSlot::Slot1]);
        assert_eq!(
            matches(&sel, &boss(Innate::Present(Crit))),
            Verdict::DoesNotMatch
        );
    }

    #[test]
    fn any_set_picks_every_soul() {
        let mut sel = passing();
        sel.sets = SetChoice::AnySet;
        assert_eq!(matches(&sel, &soul()), Verdict::Matches);
    }

    #[test]
    fn a_level_above_fifteen_is_in_no_band() {
        let mut s = soul();
        s.level = 16;
        assert_eq!(matches(&passing(), &s), Verdict::DoesNotMatch);
    }

    /// The record of one discard scheme with these soul-mask bytes and filter bits.
    fn record(souls: Vec<u8>, filter_bits: &[usize]) -> Record {
        let mut filter = vec![0u8; 8];
        for bit in filter_bits {
            filter[bit / 8] |= 1 << (bit % 8);
        }
        Record::new("x", souls, filter).expect("short")
    }

    /// The discard scheme of `record`.
    fn discard(record: Record) -> DiscardScheme {
        let layout = super::super::layout::SchemeLayout::discard(
            super::super::layout::AccountSegment::from_bytes([7; 14]),
            vec![record],
        )
        .expect("valid");
        let code = super::super::code::decode_code(&layout).expect("ok");
        let super::super::code::SchemeCode::Discard(mut schemes) = code else {
            panic!("a discard code");
        };
        schemes.remove(0)
    }

    #[test]
    fn unknown_conditions_make_a_plan_never_exact() {
        // Slot 2, 6★, +15, Spd main; soul 破势 (bit 21); filter bit 61 unmapped.
        let record = record(vec![0, 0, 0x20], &[1, 11, 18, 54, 61]);
        let (selection, _) = decode_selection(&record, SchemeKind::Discard).expect("ok");
        let scheme = &discard(record);
        assert!(scheme.has_unknown_conditions());
        assert_eq!(scheme.selection, selection);
        assert_eq!(matches(&selection, &soul()), Verdict::Matches);
        assert_eq!(
            scheme.matches(&soul()),
            Verdict::Undetermined(vec![OpenRule::UnknownConditions])
        );
        let mut other = soul();
        other.star = 5;
        assert_eq!(scheme.matches(&other), Verdict::DoesNotMatch);
    }

    #[test]
    fn unknown_conditions_join_the_innate_rule() {
        // As above, with 土蜘蛛 (bit 33) beside 破势 and 固有属性 暴击 (bit 60).
        let scheme = discard(record(vec![0, 0, 0x20, 0, 0x02], &[1, 11, 18, 54, 60, 61]));
        assert!(scheme.has_unknown_conditions());
        let unknown = Verdict::Undetermined(vec![OpenRule::UnknownConditions]);
        assert_eq!(scheme.matches(&boss(Innate::Present(Crit))), unknown);
        assert_eq!(scheme.matches(&soul()), unknown);
        assert_eq!(
            scheme.matches(&boss(Innate::Present(AtkPercent))),
            Verdict::DoesNotMatch
        );
        assert_eq!(
            scheme.matches(&boss(Innate::Unknown)),
            Verdict::Undetermined(vec![OpenRule::Innate, OpenRule::UnknownConditions])
        );
    }
}
