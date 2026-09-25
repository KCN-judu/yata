//! The one authority for what each solved scheme bit means.
//!
//! Every number that places a domain value in a soul mask or a filter is in this file, and
//! nowhere else: the semantic codec ([`super::selection`]) and the bit editor ([`super::edit`])
//! both read these tables. The source of every row is `research/scheme-code-protocol.md` (local
//! research, not published), "Soul type bitset" and "Filter bitset"; each row there is confirmed
//! by a controlled export or an import read (2026-09-24). A change to a mapping is a change to one
//! row here and to its test.
//!
//! One limit of the evidence: the scheme bit ↔ soul relation is confirmed by import, by the soul
//! the game shows for each bit; the suit code of each soul is the prior tool's identifier, a
//! hypothesis until a reader recording re-establishes it (ADR-0014).

use crate::soul::{SoulAttribute, SoulSet, SoulSlot, Star};

use super::selection::{InnateAttribute, LevelBand, SubCount};

/// A soul-mask bit this codec may write: one per mapped soul set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SoulBit(u16);

impl SoulBit {
    pub fn new(bit: u16) -> Option<SoulBit> {
        (bit < SOUL_BIT_COUNT).then_some(SoulBit(bit))
    }

    pub fn index(self) -> u16 {
        self.0
    }

    /// The set this bit selects.
    pub fn set(self) -> SoulSet {
        SoulSet::from_suit_code(SUIT_CODE_BY_SOUL_BIT[usize::from(self.0)])
    }
}

/// A filter bit this codec may write: one of a solved group of the tables below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterBit(u16);

impl FilterBit {
    pub fn new(bit: u16) -> Option<FilterBit> {
        is_solved_filter_bit(bit).then_some(FilterBit(bit))
    }

    pub fn index(self) -> u16 {
        self.0
    }
}

/// The two filter bits of one sub-attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SubBits {
    /// ○.
    pub include: FilterBit,
    /// ✕.
    pub exclude: FilterBit,
}

impl SubBits {
    const fn new(include: u16, exclude: u16) -> SubBits {
        SubBits {
            include: FilterBit(include),
            exclude: FilterBit(exclude),
        }
    }
}

/// The suit code of the soul each soul-mask bit selects: bit `n` is `SUIT_CODE_BY_SOUL_BIT[n]`.
/// Not derivable from the suit code: 薙魂 (21) and 木魅 (23) sit at bits 29 and 28, and codes
/// 55–60 at bits 64–69, after codes 94–99.
const SUIT_CODE_BY_SOUL_BIT: [u8; 70] = [
    2, 3, 4, 6, 7, 8, 9, 10, 11, 12, // bits 0–9
    13, 14, 15, 18, 19, 20, 22, 24, 26, 27, // bits 10–19
    29, 30, 31, 32, 33, 34, 35, 36, 23, 21, // bits 20–29
    39, 48, 49, 50, 51, 52, 53, 54, 73, 74, // bits 30–39
    75, 76, 77, 79, 80, 81, 82, 83, 84, 85, // bits 40–49
    86, 87, 88, 89, 90, 91, 92, 93, 94, 95, // bits 50–59
    96, 97, 98, 99, 55, 56, 57, 58, 59, 60, // bits 60–69
];

/// Soul-mask bits in use: one per mapped soul set.
pub(crate) const SOUL_BIT_COUNT: u16 = SUIT_CODE_BY_SOUL_BIT.len() as u16;

/// 位置: bit `n` is slot `n + 1`.
pub(crate) const SLOT_BITS: [(SoulSlot, FilterBit); 6] = [
    (SoulSlot::Slot1, FilterBit(0)),
    (SoulSlot::Slot2, FilterBit(1)),
    (SoulSlot::Slot3, FilterBit(2)),
    (SoulSlot::Slot4, FilterBit(3)),
    (SoulSlot::Slot5, FilterBit(4)),
    (SoulSlot::Slot6, FilterBit(5)),
];

/// 星级: bit `5 + n` is `n` stars.
pub(crate) const STAR_BITS: [(Star, FilterBit); 6] = [
    (Star::One, FilterBit(6)),
    (Star::Two, FilterBit(7)),
    (Star::Three, FilterBit(8)),
    (Star::Four, FilterBit(9)),
    (Star::Five, FilterBit(10)),
    (Star::Six, FilterBit(11)),
];

/// 主属性, in the game's attribute order.
pub(crate) const MAIN_BITS: [(SoulAttribute, FilterBit); 11] = [
    (SoulAttribute::AtkFlat, FilterBit(12)),
    (SoulAttribute::AtkPercent, FilterBit(13)),
    (SoulAttribute::DefFlat, FilterBit(14)),
    (SoulAttribute::DefPercent, FilterBit(15)),
    (SoulAttribute::HpFlat, FilterBit(16)),
    (SoulAttribute::HpPercent, FilterBit(17)),
    (SoulAttribute::Spd, FilterBit(18)),
    (SoulAttribute::EffectHit, FilterBit(19)),
    (SoulAttribute::EffectRes, FilterBit(20)),
    (SoulAttribute::Crit, FilterBit(21)),
    (SoulAttribute::CritDmg, FilterBit(22)),
];

/// 副属性: each attribute's include bit ○ and exclude bit ✕, in the main-attribute order.
pub(crate) const SUB_BITS: [(SoulAttribute, SubBits); 11] = [
    (SoulAttribute::AtkFlat, SubBits::new(23, 24)),
    (SoulAttribute::AtkPercent, SubBits::new(25, 26)),
    (SoulAttribute::DefFlat, SubBits::new(27, 28)),
    (SoulAttribute::DefPercent, SubBits::new(29, 30)),
    (SoulAttribute::HpFlat, SubBits::new(31, 32)),
    (SoulAttribute::HpPercent, SubBits::new(33, 34)),
    (SoulAttribute::Spd, SubBits::new(35, 36)),
    (SoulAttribute::EffectHit, SubBits::new(37, 38)),
    (SoulAttribute::EffectRes, SubBits::new(39, 40)),
    (SoulAttribute::Crit, SubBits::new(41, 42)),
    (SoulAttribute::CritDmg, SubBits::new(43, 44)),
];

/// 数量.
pub(crate) const COUNT_BITS: [(SubCount, FilterBit); 4] = [
    (SubCount::FewerThanTwo, FilterBit(45)),
    (SubCount::Two, FilterBit(46)),
    (SubCount::Three, FilterBit(47)),
    (SubCount::Four, FilterBit(48)),
];

/// 等级.
pub(crate) const LEVEL_BITS: [(LevelBand, FilterBit); 6] = [
    (LevelBand::L0to2, FilterBit(49)),
    (LevelBand::L3to5, FilterBit(50)),
    (LevelBand::L6to8, FilterBit(51)),
    (LevelBand::L9to11, FilterBit(52)),
    (LevelBand::L12to14, FilterBit(53)),
    (LevelBand::L15, FilterBit(54)),
];

/// 固有属性, left to right in the editor.
pub(crate) const INNATE_BITS: [(InnateAttribute, FilterBit); 6] = [
    (InnateAttribute(SoulAttribute::AtkPercent), FilterBit(55)),
    (InnateAttribute(SoulAttribute::DefPercent), FilterBit(56)),
    (InnateAttribute(SoulAttribute::HpPercent), FilterBit(57)),
    (InnateAttribute(SoulAttribute::EffectHit), FilterBit(58)),
    (InnateAttribute(SoulAttribute::EffectRes), FilterBit(59)),
    (InnateAttribute(SoulAttribute::Crit), FilterBit(60)),
];

/// The soul-mask bit of a set, or `None` for a suit code no bit is mapped to.
pub(crate) fn soul_bit(set: SoulSet) -> Option<SoulBit> {
    SUIT_CODE_BY_SOUL_BIT
        .iter()
        .position(|&c| c == set.suit_code())
        .and_then(|i| u16::try_from(i).ok())
        .map(SoulBit)
}

/// The set a raw soul-mask position selects, or `None` for a position beyond the mapped sets.
/// Only tests read a raw position now; the suit-code ledger that did left with the probe.
#[cfg(test)]
pub(crate) fn soul_set(bit: u16) -> Option<SoulSet> {
    SoulBit::new(bit).map(SoulBit::set)
}

/// Every mapped filter bit, ascending: the bits the semantic codec owns.
pub(crate) fn solved_filter_bits() -> impl Iterator<Item = u16> {
    let mut bits: Vec<u16> = SLOT_BITS
        .iter()
        .map(|e| e.1)
        .chain(STAR_BITS.iter().map(|e| e.1))
        .chain(MAIN_BITS.iter().map(|e| e.1))
        .chain(SUB_BITS.iter().flat_map(|e| [e.1.include, e.1.exclude]))
        .chain(COUNT_BITS.iter().map(|e| e.1))
        .chain(LEVEL_BITS.iter().map(|e| e.1))
        .chain(INNATE_BITS.iter().map(|e| e.1))
        .map(FilterBit::index)
        .collect();
    bits.sort_unstable();
    bits.into_iter()
}

pub(crate) fn is_solved_filter_bit(bit: u16) -> bool {
    solved_filter_bits().any(|b| b == bit)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn seventy_soul_bits_map_to_seventy_distinct_suit_codes() {
        let codes: BTreeSet<u8> = SUIT_CODE_BY_SOUL_BIT.iter().copied().collect();
        assert_eq!(codes.len(), 70);
        assert_eq!(SOUL_BIT_COUNT, 70);
        for bit in 0..SOUL_BIT_COUNT {
            let set = soul_set(bit).expect("mapped");
            assert_eq!(soul_bit(set).map(SoulBit::index), Some(bit));
        }
        assert_eq!(soul_set(70), None);
    }

    #[test]
    fn the_anchors_of_the_research_table_hold() {
        // 雪幽魂 bit 0 = code 2; 破势 bit 21 = code 30; 木魅 bit 28 = 23; 薙魂 bit 29 = 21;
        // 土蜘蛛 bit 33 = 50; 八咫镜 bit 58 = 94; 片叶之苇 bit 64 = 55; 雨降 bit 69 = 60.
        for (bit, code) in [
            (0, 2),
            (21, 30),
            (28, 23),
            (29, 21),
            (33, 50),
            (58, 94),
            (64, 55),
            (69, 60),
        ] {
            assert_eq!(soul_set(bit), Some(SoulSet::from_suit_code(code)), "{bit}");
        }
        for unused in [0, 1, 5, 16, 17, 25, 28, 37, 38, 40, 47, 78, 100, 255] {
            assert_eq!(soul_bit(SoulSet::from_suit_code(unused)), None, "{unused}");
        }
    }

    #[test]
    fn the_filter_groups_tile_bits_zero_to_sixty_without_overlap() {
        let bits: Vec<u16> = solved_filter_bits().collect();
        assert_eq!(bits, (0..=60).collect::<Vec<u16>>());
    }

    #[test]
    fn every_domain_value_has_exactly_one_row() {
        let slots: BTreeSet<_> = SLOT_BITS.iter().map(|e| e.0).collect();
        assert_eq!(slots.len(), SoulSlot::ALL.len());
        let mains: BTreeSet<_> = MAIN_BITS.iter().map(|e| e.0).collect();
        assert_eq!(mains.len(), SoulAttribute::ALL.len());
        let subs: BTreeSet<_> = SUB_BITS.iter().map(|e| e.0).collect();
        assert_eq!(subs.len(), SoulAttribute::ALL.len());
        let stars: BTreeSet<_> = STAR_BITS.iter().map(|e| e.0).collect();
        assert_eq!(stars, Star::ALL.into_iter().collect());
        let levels: BTreeSet<_> = LEVEL_BITS.iter().map(|e| e.0).collect();
        assert_eq!(levels.len(), LevelBand::ALL.len());
        let counts: BTreeSet<_> = COUNT_BITS.iter().map(|e| e.0).collect();
        assert_eq!(counts.len(), SubCount::ALL.len());
        let innate: BTreeSet<_> = INNATE_BITS.iter().map(|e| e.0).collect();
        assert_eq!(innate.len(), InnateAttribute::ALL.len());
    }
}
