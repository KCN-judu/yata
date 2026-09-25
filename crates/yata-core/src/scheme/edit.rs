//! Editing a record bit by bit, for solved bits only (`scheme-code.md`, "Codec rules").
//!
//! A soul bit is one of the 70 soul sets the research record maps; a filter bit is one of the
//! solved filter groups: slot, star, main attribute, the sub-attribute include/exclude pairs
//! that are located, sub-attribute count, level, and innate attribute.
//! Their constructors refuse every other bit, so an edit can never write an unsolved bit, and no
//! edit can fail. An edit changes that one bit and nothing else, with one documented exception:
//! clearing a soul bit trims trailing zero bytes from the soul mask, as the game writes masks.
//!
//! Which soul or attribute a bit means is [`super::mapping`]'s, applied by [`super::selection`];
//! here a bit is a position. This is the research tool's level; the application edits selections.

use super::layout::Record;
use super::mapping;
use super::name::SchemeName;
use crate::nonempty::NonEmptySet;

/// Soul bits in use: one per mapped soul set ([`super::mapping`]).
pub const SOUL_BIT_COUNT: u16 = mapping::SOUL_BIT_COUNT;

/// A soul-set bit this codec may write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SoulBit(u16);

impl SoulBit {
    pub fn new(bit: u16) -> Option<SoulBit> {
        (bit < SOUL_BIT_COUNT).then_some(SoulBit(bit))
    }

    pub fn index(self) -> u16 {
        self.0
    }
}

/// A filter bit this codec may write: one of a solved group of [`super::mapping`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterBit(u16);

impl FilterBit {
    pub fn new(bit: u16) -> Option<FilterBit> {
        mapping::is_solved_filter_bit(bit).then_some(FilterBit(bit))
    }

    pub fn index(self) -> u16 {
        self.0
    }
}

/// 类型 as bits: the editor's "all souls", or the souls chosen.
///
/// "All souls" is the game's meaning of a mask with no soul chosen: no restriction. It has this
/// one name, and is written as a mask with no bit set. `B` is [`SoulBit`] for what this codec
/// writes, and a raw bit position for what a record holds, where a bit beyond the mapped sets can
/// be set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoulChoice<B: Ord = SoulBit> {
    All,
    Souls(NonEmptySet<B>),
}

/// Whether an edit sets a bit or clears it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitState {
    On,
    Off,
}

fn get(bytes: &[u8], bit: u16) -> bool {
    let bit = usize::from(bit);
    bytes.get(bit / 8).is_some_and(|b| b >> (bit % 8) & 1 == 1)
}

/// Set or clear one bit; a field grows for a set bit beyond its end. Every bit this module names
/// is below 72, so a field it writes stays within nine bytes, far inside a one-byte length.
fn set(bytes: &mut Vec<u8>, bit: u16, state: BitState) {
    let bit = usize::from(bit);
    if bytes.len() <= bit / 8 {
        if state == BitState::Off {
            return;
        }
        bytes.resize(bit / 8 + 1, 0);
    }
    let mask = 1u8 << (bit % 8);
    match state {
        BitState::On => bytes[bit / 8] |= mask,
        BitState::Off => bytes[bit / 8] &= !mask,
    }
}

/// Every set bit of a field, ascending, solved or not.
fn set_bits(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    (0..bytes.len() * 8)
        .filter_map(|b| u16::try_from(b).ok())
        .filter(|&b| get(bytes, b))
}

impl Record {
    /// The souls the mask chooses, every set bit included, mapped or not. A mask with no bit set,
    /// empty or not, is [`SoulChoice::All`].
    pub fn souls(&self) -> SoulChoice<u16> {
        NonEmptySet::collect(set_bits(&self.soul_mask)).map_or(SoulChoice::All, SoulChoice::Souls)
    }

    /// Every set filter bit, ascending, solved or not.
    pub fn filter_bits(&self) -> Vec<u16> {
        set_bits(&self.filter).collect()
    }

    pub fn has_soul(&self, bit: SoulBit) -> bool {
        get(&self.soul_mask, bit.0)
    }

    pub fn has_filter(&self, bit: FilterBit) -> bool {
        get(&self.filter, bit.0)
    }

    /// Set or clear one soul bit. Setting a bit on an "all souls" record makes it choose that one
    /// soul; clearing the last one makes it "all souls" again. Clearing trims trailing zero bytes,
    /// as the game writes masks.
    pub fn set_soul(&mut self, bit: SoulBit, state: BitState) {
        set(&mut self.soul_mask, bit.0, state);
        if state == BitState::Off {
            while self.soul_mask.last() == Some(&0) {
                self.soul_mask.pop();
            }
        }
    }

    /// Set or clear one solved filter bit, and nothing else.
    pub fn set_filter(&mut self, bit: FilterBit, state: BitState) {
        set(&mut self.filter, bit.0, state);
    }

    /// A record from nothing. The filter has only the given solved bits set and, like the soul
    /// mask, is trimmed to its highest set bit, as the game writes it.
    pub fn from_bits(name: &SchemeName, souls: &SoulChoice, filter: &[FilterBit]) -> Record {
        let mut soul_mask = Vec::new();
        if let SoulChoice::Souls(souls) = souls {
            for s in souls {
                set(&mut soul_mask, s.0, BitState::On);
            }
        }
        let mut filter_bytes = Vec::new();
        for f in filter {
            set(&mut filter_bytes, f.0, BitState::On);
        }
        Record {
            name: name.as_str().as_bytes().to_vec(),
            soul_mask,
            filter: filter_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::inspect::diff;
    use super::super::layout::{AccountSegment, SchemeLayout, serialize};
    use super::*;

    fn soul(n: u16) -> SoulBit {
        SoulBit::new(n).expect("a known soul bit")
    }

    fn filt(n: u16) -> FilterBit {
        FilterBit::new(n).expect("a solved filter bit")
    }

    fn name(s: &str) -> SchemeName {
        SchemeName::new(s).expect("an importable name")
    }

    fn souls(bits: &[u16]) -> SoulChoice {
        SoulChoice::Souls(NonEmptySet::collect(bits.iter().map(|&b| soul(b))).expect("some"))
    }

    fn set_of(plans: Vec<Record>) -> SchemeLayout {
        SchemeLayout::Strengthening {
            account: AccountSegment::from_bytes([7; 14]),
            plans,
        }
    }

    #[test]
    fn only_solved_bits_can_be_named() {
        assert!(SoulBit::new(69).is_some() && SoulBit::new(70).is_none());
        for open in [61, 62, 63, 64, 200] {
            assert_eq!(FilterBit::new(open), None, "{open}");
        }
        for solved in [
            0, 11, 22, 23, 24, 27, 28, 31, 32, 44, 45, 48, 49, 54, 55, 60,
        ] {
            assert!(FilterBit::new(solved).is_some(), "{solved}");
        }
    }

    #[test]
    fn a_record_from_scratch_trims_its_soul_mask_and_its_filter() {
        let r = Record::from_bits(
            &name("测试位39"),
            &souls(&[39]),
            &[filt(0), filt(11), filt(49)],
        );
        assert_eq!(r.soul_mask(), &[0, 0, 0, 0, 0x80]);
        assert_eq!(r.filter(), &[0x01, 0x08, 0, 0, 0, 0, 0x02]);
        assert_eq!(r.souls(), SoulChoice::Souls(NonEmptySet::one(39)));
    }

    #[test]
    fn the_discard_schemes_the_game_exported_are_rebuilt() {
        // A discard code exported by the game on 2026-09-24, two schemes, filters trimmed.
        let first = Record::from_bits(
            &name("二号位双速招财"),
            &souls(&[7]),
            &[1, 18, 35].map(filt),
        );
        assert_eq!(first.filter(), &[0x02, 0x00, 0x04, 0x00, 0x08]);
        let second = Record::from_bits(&name("一号位针女"), &souls(&[27]), &[filt(0)]);
        assert_eq!(second.filter(), &[0x01]);
        assert_eq!(second.soul_mask(), &[0, 0, 0, 0x08]);
    }

    #[test]
    fn all_souls_is_a_mask_with_no_bit_set() {
        let all = Record::from_bits(&name("全部"), &SoulChoice::All, &[]);
        assert_eq!(all.soul_mask(), &[] as &[u8]);
        assert_eq!(all.souls(), SoulChoice::All);
        // A mask of zero bytes chooses no soul either: the same choice, not another one.
        let zeros = Record::new("x", vec![0, 0], Vec::new()).expect("short");
        assert_eq!(zeros.souls(), SoulChoice::All);
    }

    #[test]
    fn the_experiment_plan_matches_the_imported_bytes() {
        // Plan 测试位39 of the import confirmed on 2026-09-24: filter copied from the game.
        let r = Record::from_bits(
            &name("测试位39"),
            &souls(&[39]),
            &[0, 1, 2, 3, 4, 5, 11, 49].map(filt),
        );
        assert_eq!(r.filter(), &[0x3f, 0x08, 0, 0, 0, 0, 0x02]);
    }

    #[test]
    fn the_controlled_plans_exported_on_2026_09_24_are_rebuilt() {
        // The game's own export of the controlled plans: one extra bit each over the baseline.
        let base = [0u16, 1, 2, 3, 4, 5, 11, 49];
        let with = |extra: &[u16]| {
            let bits: Vec<FilterBit> = base.iter().chain(extra).map(|&b| filt(b)).collect();
            Record::from_bits(&name("p"), &souls(&[33]), &bits)
        };
        assert_eq!(with(&[45]).filter(), &[0x3f, 0x08, 0, 0, 0, 0x20, 0x02]);
        // The flat sub-attributes, from the export of the third template.
        assert_eq!(with(&[23]).filter(), &[0x3f, 0x08, 0x80, 0, 0, 0, 0x02]);
        assert_eq!(with(&[28]).filter(), &[0x3f, 0x08, 0, 0x10, 0, 0, 0x02]);
        assert_eq!(with(&[32]).filter(), &[0x3f, 0x08, 0, 0, 0x01, 0, 0x02]);
        assert_eq!(with(&[48]).filter(), &[0x3f, 0x08, 0, 0, 0, 0, 0x03]);
        assert_eq!(with(&[55]).filter(), &[0x3f, 0x08, 0, 0, 0, 0, 0x82]);
        assert_eq!(with(&[56]).filter(), &[0x3f, 0x08, 0, 0, 0, 0, 0x02, 0x01]);
        assert_eq!(
            with(&[55, 56, 57, 58, 59, 60]).filter(),
            &[0x3f, 0x08, 0, 0, 0, 0, 0x82, 0x1f]
        );
    }

    #[test]
    fn clearing_the_last_soul_chooses_all_souls() {
        let mut r = Record::from_bits(&name("x"), &souls(&[5]), &[]);
        r.set_soul(soul(5), BitState::Off);
        assert_eq!(r.souls(), SoulChoice::All);
        assert_eq!(r.soul_mask(), &[] as &[u8]);
    }

    #[test]
    fn clearing_the_highest_soul_trims_the_mask() {
        let mut r = Record::from_bits(&name("x"), &souls(&[3, 69]), &[]);
        assert_eq!(r.soul_mask().len(), 9);
        r.set_soul(soul(69), BitState::Off);
        assert_eq!(r.soul_mask(), &[0x08]);
    }

    #[test]
    fn choosing_a_soul_on_an_all_souls_record_selects_it() {
        let mut r = Record::from_bits(&name("x"), &SoulChoice::All, &[]);
        r.set_soul(soul(0), BitState::On);
        assert_eq!(r.souls(), SoulChoice::Souls(NonEmptySet::one(0)));
    }

    #[test]
    fn a_filter_edit_changes_exactly_that_bit_of_the_payload() {
        let r = Record::from_bits(&name("x"), &souls(&[2]), &[filt(1), filt(49)]);
        let before = serialize(&set_of(vec![r.clone()])).expect("writable");
        for bit in [0u16, 6, 12, 22, 25, 46, 54, 55] {
            let mut edited = r.clone();
            let state = if edited.has_filter(filt(bit)) {
                BitState::Off
            } else {
                BitState::On
            };
            edited.set_filter(filt(bit), state);
            let after = serialize(&set_of(vec![edited])).expect("writable");
            let d = diff(&before, &after);
            let bits: Vec<usize> = d
                .changes
                .iter()
                .flat_map(|c| c.changed_bit_offsets())
                .collect();
            // The filter starts after the header, the name field (2) and the soul field (2), plus
            // its own length byte.
            let filter_start = (17 + 2 + 2 + 1) * 8;
            assert_eq!(bits, vec![filter_start + usize::from(bit)], "bit {bit}");
        }
    }

    #[test]
    fn a_soul_edit_inside_the_mask_changes_exactly_that_bit() {
        let r = Record::from_bits(&name("x"), &souls(&[20]), &[]);
        let before = serialize(&set_of(vec![r.clone()])).expect("writable");
        let mut edited = r;
        edited.set_soul(soul(9), BitState::On);
        let after = serialize(&set_of(vec![edited])).expect("writable");
        let bits: Vec<usize> = diff(&before, &after)
            .changes
            .iter()
            .flat_map(|c| c.changed_bit_offsets())
            .collect();
        let mask_start = (17 + 2 + 1) * 8;
        assert_eq!(bits, vec![mask_start + 9]);
    }

    #[test]
    fn unsolved_bits_already_set_are_kept_by_every_edit() {
        // A filter as read, with open bits 61, 62 and 63 set.
        let mut filter = vec![0u8; 8];
        for b in [61usize, 62, 63] {
            filter[b / 8] |= 1 << (b % 8);
        }
        let mut r = Record::new("x", vec![0x01], filter).expect("valid");
        r.set_filter(filt(12), BitState::On);
        r.set_filter(filt(12), BitState::Off);
        r.set_soul(soul(1), BitState::On);
        assert_eq!(r.filter_bits(), vec![61, 62, 63]);
    }

    mod properties {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn editing_nothing_is_identity(mask in proptest::collection::vec(1u8..=255, 1..9), filter in proptest::collection::vec(any::<u8>(), 7..8)) {
                let r = Record::new("p", mask, filter).expect("valid");
                let layout = set_of(vec![r]);
                let p = serialize(&layout).expect("writable");
                prop_assert_eq!(serialize(&layout.clone()), Ok(p));
            }

            #[test]
            fn toggling_a_solved_filter_bit_twice_restores_the_record(
                filter in proptest::collection::vec(any::<u8>(), 7..8),
                bit in (0u16..55).prop_filter("solved", |b| FilterBit::new(*b).is_some()),
            ) {
                let original = Record::new("p", vec![0x01], filter).expect("valid");
                let mut r = original.clone();
                let b = filt(bit);
                let (first, back) = if r.has_filter(b) {
                    (BitState::Off, BitState::On)
                } else {
                    (BitState::On, BitState::Off)
                };
                r.set_filter(b, first);
                r.set_filter(b, back);
                prop_assert_eq!(r, original);
            }
        }
    }
}
