//! Editing a record bit by bit, for solved bits only (`scheme-code.md`, "Codec rules").
//!
//! A soul bit is one of the 70 soul sets the research record maps; a filter bit is one of the
//! solved filter groups: slot, star, main attribute, the sub-attribute include/exclude pairs
//! that are located, sub-attribute count, level, and innate attribute.
//! Their constructors refuse every other bit, so an edit can never write an unsolved bit. An
//! edit changes that one bit and nothing else, with one documented exception: clearing a soul bit
//! trims trailing zero bytes from the soul mask, as the game writes masks.
//!
//! Which soul or attribute a bit means is the mapping of the research record, applied by the
//! codec above this module; here a bit is a position.

use super::layout::{LayoutError, MAX_FIELD_LEN, Record};

/// Soul bits in use: one per soul set, 0 to 69.
pub const SOUL_BIT_COUNT: u16 = 70;

/// The solved filter bits, every one from 0 to 60, each confirmed by a controlled export:
/// slot 0–5, star 6–11, main attribute 12–22; sub-attribute include and exclude 23–44, two bits
/// per attribute in the main-attribute order (`AtkFlat` 23–24 … `CritDmg` 43–44); sub-attribute
/// count 45–48; level 49–54; innate attribute 55–60. Bits 61 and above are open.
const SOLVED_FILTER_BITS: [(u16, u16); 1] = [(0, 60)];

/// The filter length every observed plan has; a filter built from scratch has it.
pub const FILTER_LEN: usize = 7;

/// The longest plan name the game imports, in characters. Names of 11 characters or more were
/// refused on import, and the game's own exports never exceed 10 (2026-09-24).
pub const MAX_NAME_CHARS: usize = 10;

/// The longest plan name, in UTF-8 bytes, known to import: 26. Whether the game's limit counts
/// characters or bytes is not yet told apart, so a name must meet both.
pub const MAX_NAME_BYTES: usize = 26;

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

/// A filter bit this codec may write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterBit(u16);

impl FilterBit {
    pub fn new(bit: u16) -> Option<FilterBit> {
        SOLVED_FILTER_BITS
            .iter()
            .any(|&(lo, hi)| (lo..=hi).contains(&bit))
            .then_some(FilterBit(bit))
    }

    pub fn index(self) -> u16 {
        self.0
    }
}

/// Why an edit was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditError {
    /// The edit would leave the soul mask empty, which the game reads as "all souls".
    EmptySoulMask,
    /// A record from scratch was given an empty list of souls; "all souls" is `None`.
    NoSouls,
    /// A name the game would refuse on import: longer than [`MAX_NAME_CHARS`] characters or
    /// [`MAX_NAME_BYTES`] bytes.
    NameTooLong {
        chars: usize,
        bytes: usize,
    },
    Layout(LayoutError),
}

fn get(bytes: &[u8], bit: usize) -> bool {
    bytes.get(bit / 8).is_some_and(|b| b >> (bit % 8) & 1 == 1)
}

fn set(bytes: &mut Vec<u8>, bit: usize, on: bool) -> Result<(), EditError> {
    if bytes.len() <= bit / 8 {
        if !on {
            return Ok(());
        }
        if bit / 8 + 1 > MAX_FIELD_LEN {
            return Err(EditError::Layout(LayoutError::FieldTooLong {
                length: bit / 8 + 1,
            }));
        }
        bytes.resize(bit / 8 + 1, 0);
    }
    let mask = 1u8 << (bit % 8);
    if on {
        bytes[bit / 8] |= mask;
    } else {
        bytes[bit / 8] &= !mask;
    }
    Ok(())
}

/// Every set bit of a mask, ascending, solved or not.
fn set_bits(bytes: &[u8]) -> Vec<u16> {
    (0..bytes.len() * 8)
        .filter(|&b| get(bytes, b))
        .filter_map(|b| u16::try_from(b).ok())
        .collect()
}

impl Record {
    /// Whether the record uses the editor's "all souls" choice (an empty mask).
    pub fn is_all_souls(&self) -> bool {
        self.soul_mask.is_empty()
    }

    /// Every set soul bit, ascending, including bits beyond the known soul sets.
    pub fn soul_bits(&self) -> Vec<u16> {
        set_bits(&self.soul_mask)
    }

    /// Every set filter bit, ascending, solved or not.
    pub fn filter_bits(&self) -> Vec<u16> {
        set_bits(&self.filter)
    }

    pub fn has_soul(&self, bit: SoulBit) -> bool {
        get(&self.soul_mask, usize::from(bit.0))
    }

    pub fn has_filter(&self, bit: FilterBit) -> bool {
        get(&self.filter, usize::from(bit.0))
    }

    /// Set or clear one soul bit. Setting a bit on an "all souls" record makes it choose that one
    /// soul. Clearing trims trailing zero bytes, and is refused if it would empty the mask.
    pub fn set_soul(&mut self, bit: SoulBit, on: bool) -> Result<(), EditError> {
        let mut mask = self.soul_mask.clone();
        set(&mut mask, usize::from(bit.0), on)?;
        if !on {
            while mask.last() == Some(&0) {
                mask.pop();
            }
            if mask.is_empty() && !self.soul_mask.is_empty() {
                return Err(EditError::EmptySoulMask);
            }
        }
        self.soul_mask = mask;
        Ok(())
    }

    /// Set or clear one solved filter bit, and nothing else.
    pub fn set_filter(&mut self, bit: FilterBit, on: bool) -> Result<(), EditError> {
        set(&mut self.filter, usize::from(bit.0), on)
    }

    /// A record from nothing: `souls` of `None` is "all souls"; the filter is [`FILTER_LEN`]
    /// bytes with only the given solved bits set, every open bit clear. The name must be one the
    /// game imports.
    pub fn from_bits(
        name: &str,
        souls: Option<&[SoulBit]>,
        filter: &[FilterBit],
    ) -> Result<Record, EditError> {
        let chars = name.chars().count();
        if chars > MAX_NAME_CHARS || name.len() > MAX_NAME_BYTES {
            return Err(EditError::NameTooLong {
                chars,
                bytes: name.len(),
            });
        }
        let mut soul_mask = Vec::new();
        if let Some(souls) = souls {
            if souls.is_empty() {
                return Err(EditError::NoSouls);
            }
            for s in souls {
                set(&mut soul_mask, usize::from(s.0), true)?;
            }
        }
        let mut filter_bytes = vec![0u8; FILTER_LEN];
        for f in filter {
            set(&mut filter_bytes, usize::from(f.0), true)?;
        }
        Record::new(name, soul_mask, filter_bytes).map_err(EditError::Layout)
    }
}

#[cfg(test)]
mod tests {
    use super::super::inspect::diff;
    use super::super::layout::{AccountSegment, SchemeHeader, SchemeKind, SchemeLayout, serialize};
    use super::*;

    fn soul(n: u16) -> SoulBit {
        SoulBit::new(n).expect("a known soul bit")
    }

    fn filt(n: u16) -> FilterBit {
        FilterBit::new(n).expect("a solved filter bit")
    }

    fn set_of(records: Vec<Record>) -> SchemeLayout {
        SchemeLayout {
            header: SchemeHeader {
                account: AccountSegment::from_bytes([7; 14]),
                kind: SchemeKind::Strengthening,
            },
            records,
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
    fn a_record_from_scratch_trims_its_soul_mask_and_has_a_seven_byte_filter() {
        let r = Record::from_bits(
            "测试位39",
            Some(&[soul(39)]),
            &[filt(0), filt(11), filt(49)],
        )
        .expect("valid");
        assert_eq!(r.soul_mask(), &[0, 0, 0, 0, 0x80]);
        assert_eq!(r.filter(), &[0x01, 0x08, 0, 0, 0, 0, 0x02]);
        assert!(!r.is_all_souls());
    }

    #[test]
    fn all_souls_is_none_and_an_empty_list_is_refused() {
        let all = Record::from_bits("全部", None, &[]).expect("valid");
        assert!(all.is_all_souls());
        assert_eq!(
            Record::from_bits("x", Some(&[]), &[]),
            Err(EditError::NoSouls)
        );
    }

    #[test]
    fn the_experiment_plan_matches_the_imported_bytes() {
        // Plan 测试位39 of the import confirmed on 2026-09-24: filter copied from the game.
        let r = Record::from_bits(
            "测试位39",
            Some(&[soul(39)]),
            &[0, 1, 2, 3, 4, 5, 11, 49].map(filt),
        )
        .expect("valid");
        assert_eq!(r.filter(), &[0x3f, 0x08, 0, 0, 0, 0, 0x02]);
    }

    #[test]
    fn the_controlled_plans_exported_on_2026_09_24_are_rebuilt() {
        // The game's own export of the controlled plans: one extra bit each over the baseline.
        let base = [0u16, 1, 2, 3, 4, 5, 11, 49];
        let with = |extra: &[u16]| {
            let bits: Vec<FilterBit> = base.iter().chain(extra).map(|&b| filt(b)).collect();
            Record::from_bits("p", Some(&[soul(33)]), &bits).expect("valid")
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
    fn names_the_game_refuses_are_refused() {
        // Imported on 2026-09-24: 10 characters, 26 bytes.
        assert!(Record::from_bits("基准-不要改-雪幽魂", None, &[]).is_ok());
        // Refused on import: 11 characters, 29 bytes.
        assert_eq!(
            Record::from_bits("攻击固定值-排除-蝠翼", None, &[]),
            Err(EditError::NameTooLong {
                chars: 11,
                bytes: 29
            })
        );
        // Ten characters, but past the longest byte length known to import.
        assert!(matches!(
            Record::from_bits("攻击攻击攻击攻击攻击", None, &[]),
            Err(EditError::NameTooLong { chars: 10, .. })
        ));
    }

    #[test]
    fn clearing_the_last_soul_is_refused() {
        let mut r = Record::from_bits("x", Some(&[soul(5)]), &[]).expect("valid");
        assert_eq!(r.set_soul(soul(5), false), Err(EditError::EmptySoulMask));
        assert!(r.has_soul(soul(5)));
    }

    #[test]
    fn clearing_the_highest_soul_trims_the_mask() {
        let mut r = Record::from_bits("x", Some(&[soul(3), soul(69)]), &[]).expect("valid");
        assert_eq!(r.soul_mask().len(), 9);
        r.set_soul(soul(69), false).expect("one soul remains");
        assert_eq!(r.soul_mask(), &[0x08]);
    }

    #[test]
    fn choosing_a_soul_on_an_all_souls_record_selects_it() {
        let mut r = Record::from_bits("x", None, &[]).expect("valid");
        r.set_soul(soul(0), true).expect("valid");
        assert_eq!(r.soul_bits(), vec![0]);
    }

    #[test]
    fn a_filter_edit_changes_exactly_that_bit_of_the_payload() {
        let r = Record::from_bits("x", Some(&[soul(2)]), &[filt(1), filt(49)]).expect("valid");
        let before = serialize(&set_of(vec![r.clone()])).expect("writable");
        for bit in [0u16, 6, 12, 22, 25, 46, 54, 55] {
            let mut edited = r.clone();
            let on = !edited.has_filter(filt(bit));
            edited.set_filter(filt(bit), on).expect("valid");
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
        let r = Record::from_bits("x", Some(&[soul(20)]), &[]).expect("valid");
        let before = serialize(&set_of(vec![r.clone()])).expect("writable");
        let mut edited = r;
        edited.set_soul(soul(9), true).expect("valid");
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
        r.set_filter(filt(12), true).expect("valid");
        r.set_filter(filt(12), false).expect("valid");
        r.set_soul(soul(1), true).expect("valid");
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
                let on = r.has_filter(b);
                r.set_filter(b, !on).expect("valid");
                r.set_filter(b, on).expect("valid");
                prop_assert_eq!(r, original);
            }
        }
    }
}
