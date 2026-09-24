//! Whether the game's filter picks a soul: `matches` of `scheme-code.md`, "Evaluation".
//!
//! This is the game's official filter and nothing else: no score, no affinity, no UI or store
//! state, no header. It decides only what the evidence decides. A group whose rule is still open
//! does not guess; it makes the verdict [`Verdict::Undetermined`] and names the open rule, unless
//! another group already rules the soul out.
//!
//! What is decided, and on what:
//!
//! - groups combine by AND: a soul is picked only if every group picks it (`scheme-code.md`)
//! - 类型, 位置, 星级, 等级, 主属性 with something chosen: the soul's value is among the choices ✓
//! - `AnySet` picks every soul ✓
//! - 副属性 ✕: a soul with that sub-attribute is not picked ◎
//! - 副属性 ○, with 数量 empty: a soul with every included attribute passes, and one with none of
//!   them fails, under either reading of several includes (AND or OR); anything between is open
//!
//! Open, each an [`OpenRule`]: an empty group; several includes (AND or OR); 数量, alone or with
//! includes; 固有属性, which a [`Soul`] does not carry yet; and a condition the model cannot see.

use crate::soul::Soul;

use super::code::{DiscardScheme, StrengtheningPlan};
use super::selection::{LevelBand, SetChoice, SoulSelection, SubAttributeMode};

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
    /// Whether a group with nothing chosen is no constraint or picks nothing.
    EmptyGroup(Group),
    /// Whether several ○ require all of them (AND) or one (OR); open only for a soul with some
    /// but not all of the included attributes.
    SeveralIncludes,
    /// What 数量 counts, alone and together with ○.
    SubCount,
    /// 固有属性: a soul's innate attribute is not in the domain model yet.
    Innate,
    /// The scheme selects on bits the model does not map.
    UnknownConditions,
}

/// A group of the game's panel, for naming an empty one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Group {
    Sets,
    Slots,
    Stars,
    Levels,
    MainAttributes,
    Innate,
    SubCounts,
}

/// Whether the game's filter of `selection` picks `soul`, as far as the evidence decides.
pub fn matches(selection: &SoulSelection, soul: &Soul) -> Verdict {
    let mut open = Vec::new();
    let mut fails = false;
    let mut chosen = |group: Group, empty: bool, contains: bool| {
        if empty {
            open.push(OpenRule::EmptyGroup(group));
        } else if !contains {
            fails = true;
        }
    };
    match &selection.sets {
        SetChoice::AnySet => {}
        SetChoice::Sets(sets) => chosen(Group::Sets, sets.is_empty(), sets.contains(&soul.set)),
    }
    let s = selection;
    chosen(
        Group::Slots,
        s.slots.is_empty(),
        s.slots.contains(&soul.slot),
    );
    chosen(
        Group::Stars,
        s.stars.is_empty(),
        s.stars.contains(&soul.star),
    );
    let band = LevelBand::of(soul.level);
    chosen(
        Group::Levels,
        s.levels.is_empty(),
        band.is_some_and(|b| s.levels.contains(&b)),
    );
    chosen(
        Group::MainAttributes,
        s.main_attributes.is_empty(),
        s.main_attributes.contains(&soul.main),
    );
    if s.innate.is_empty() {
        open.push(OpenRule::EmptyGroup(Group::Innate));
    } else {
        open.push(OpenRule::Innate);
    }
    let has = |a| soul.sub(a).is_some();
    if s.sub_attributes.with(SubAttributeMode::Exclude).any(has) {
        fails = true;
    }
    let included: Vec<_> = s.sub_attributes.with(SubAttributeMode::Include).collect();
    if s.sub_counts.is_empty() {
        open.push(OpenRule::EmptyGroup(Group::SubCounts));
        let present = included.iter().filter(|&&a| has(a)).count();
        if !included.is_empty() && present == 0 {
            fails = true;
        } else if present < included.len() {
            open.push(OpenRule::SeveralIncludes);
        }
    } else {
        open.push(OpenRule::SubCount);
    }
    if fails {
        Verdict::DoesNotMatch
    } else if open.is_empty() {
        Verdict::Matches
    } else {
        open.sort_unstable();
        open.dedup();
        Verdict::Undetermined(open)
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
    use super::super::selection::{InnateAttribute, SubCount, decode_selection};
    use super::*;
    use crate::soul::{SoulAttribute, SoulSet, SoulSlot, SubAttribute};

    use SoulAttribute::*;

    /// 破势 (30), slot 2, 6★, +15, main Spd; subs Crit, CritDmg, AtkPercent.
    fn soul() -> Soul {
        Soul {
            set: SoulSet::from_suit_code(30),
            slot: SoulSlot::Slot2,
            star: 6,
            level: 15,
            main: Spd,
            main_value: 57.0,
            subs: [Crit, CritDmg, AtkPercent]
                .map(|attribute| SubAttribute {
                    attribute,
                    value: 1.0,
                    enhancement_count: None,
                })
                .to_vec(),
        }
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

    fn open(rules: &[OpenRule]) -> Verdict {
        Verdict::Undetermined(rules.to_vec())
    }

    const EMPTY_TAIL: [OpenRule; 2] = [
        OpenRule::EmptyGroup(Group::Innate),
        OpenRule::EmptyGroup(Group::SubCounts),
    ];

    #[test]
    fn a_soul_every_decided_group_picks_rests_on_the_open_rules_only() {
        assert_eq!(matches(&passing(), &soul()), open(&EMPTY_TAIL));
    }

    #[test]
    fn each_decided_group_rules_a_soul_out() {
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
        sel.sub_attributes.set(HpPercent, SubAttributeMode::Include);
        out.push(sel);
        for sel in out {
            assert_eq!(matches(&sel, &s), Verdict::DoesNotMatch, "{sel:?}");
        }
    }

    #[test]
    fn a_decided_failure_wins_over_every_open_rule() {
        let mut sel = SoulSelection::new(SetChoice::AnySet);
        sel.stars = BTreeSet::from([4]);
        sel.innate = BTreeSet::from([InnateAttribute::ALL[0]]);
        sel.sub_counts = BTreeSet::from([SubCount::Four]);
        assert_eq!(matches(&sel, &soul()), Verdict::DoesNotMatch);
    }

    #[test]
    fn any_set_picks_every_soul_and_is_not_an_empty_group() {
        let mut sel = passing();
        sel.sets = SetChoice::AnySet;
        assert_eq!(matches(&sel, &soul()), open(&EMPTY_TAIL));
    }

    #[test]
    fn empty_groups_are_open_and_named() {
        let sel = SoulSelection::new(SetChoice::Sets(BTreeSet::new()));
        assert_eq!(
            matches(&sel, &soul()),
            open(&[
                OpenRule::EmptyGroup(Group::Sets),
                OpenRule::EmptyGroup(Group::Slots),
                OpenRule::EmptyGroup(Group::Stars),
                OpenRule::EmptyGroup(Group::Levels),
                OpenRule::EmptyGroup(Group::MainAttributes),
                OpenRule::EmptyGroup(Group::Innate),
                OpenRule::EmptyGroup(Group::SubCounts),
            ])
        );
    }

    #[test]
    fn includes_are_decided_only_where_and_and_or_agree() {
        let s = soul();
        let mut both = passing();
        both.sub_attributes.set(Crit, SubAttributeMode::Include);
        both.sub_attributes.set(CritDmg, SubAttributeMode::Include);
        assert_eq!(matches(&both, &s), open(&EMPTY_TAIL));
        let mut one_of_two = passing();
        one_of_two
            .sub_attributes
            .set(Crit, SubAttributeMode::Include);
        one_of_two
            .sub_attributes
            .set(Spd, SubAttributeMode::Include);
        assert_eq!(
            matches(&one_of_two, &s),
            open(&[
                OpenRule::EmptyGroup(Group::Innate),
                OpenRule::EmptyGroup(Group::SubCounts),
                OpenRule::SeveralIncludes,
            ])
        );
    }

    #[test]
    fn with_a_count_chosen_includes_are_not_decided() {
        let mut sel = passing();
        sel.sub_attributes.set(EffectHit, SubAttributeMode::Include);
        sel.sub_counts = BTreeSet::from([SubCount::FewerThanTwo]);
        assert_eq!(
            matches(&sel, &soul()),
            open(&[OpenRule::EmptyGroup(Group::Innate), OpenRule::SubCount])
        );
    }

    #[test]
    fn an_innate_choice_is_open_while_a_soul_has_no_innate_attribute() {
        let mut sel = passing();
        sel.innate = BTreeSet::from([InnateAttribute::ALL[5]]);
        assert_eq!(
            matches(&sel, &soul()),
            open(&[OpenRule::EmptyGroup(Group::SubCounts), OpenRule::Innate])
        );
    }

    #[test]
    fn a_level_above_fifteen_is_in_no_band() {
        let mut s = soul();
        s.level = 16;
        assert_eq!(matches(&passing(), &s), Verdict::DoesNotMatch);
    }

    #[test]
    fn unknown_conditions_make_a_plan_never_exact() {
        // Slot 2, 6★, +15, Spd main; soul 破势 (bit 21); filter bit 61 unmapped.
        let mut filter = vec![0u8; 8];
        for bit in [1usize, 11, 18, 54, 61] {
            filter[bit / 8] |= 1 << (bit % 8);
        }
        let record = Record::new("x", vec![0, 0, 0x20], filter).expect("short");
        let (selection, _) = decode_selection(&record, SchemeKind::Discard).expect("ok");
        let layout = super::super::layout::SchemeLayout::discard(
            super::super::layout::AccountSegment::from_bytes([7; 14]),
            vec![record],
        )
        .expect("valid");
        let code = super::super::code::decode_code(&layout).expect("ok");
        let super::super::code::SchemeCode::Discard(schemes) = code else {
            panic!("a discard code");
        };
        let scheme = &schemes[0];
        assert!(scheme.has_unknown_conditions());
        assert_eq!(scheme.selection, selection);
        let mut expected = EMPTY_TAIL.to_vec();
        expected.push(OpenRule::UnknownConditions);
        assert_eq!(scheme.matches(&soul()), Verdict::Undetermined(expected));
        let mut other = soul();
        other.star = 5;
        assert_eq!(scheme.matches(&other), Verdict::DoesNotMatch);
    }
}
