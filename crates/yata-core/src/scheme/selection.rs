//! `SoulSelection`: what a scheme record means, in the game's filter-panel groups
//! (`scheme-code.md`, "SoulSelection").
//!
//! The selection holds domain values only: sets, slots, stars, attributes, bands. Where they sit
//! in a record is [`super::mapping`]'s; this module applies that mapping in both directions and
//! keeps everything it does not model in [`Preserved`], so a decode followed by an encode with no
//! edit gives back the record's fields byte for byte.

use std::collections::{BTreeMap, BTreeSet};

pub use crate::soul::InnateAttribute;
use crate::soul::{Level, SoulAttribute, SoulSet, SoulSlot, Star};

use super::layout::{MAX_FIELD_LEN, Record};
use super::mapping::{
    COUNT_BITS, FilterBit, INNATE_BITS, LEVEL_BITS, MAIN_BITS, SLOT_BITS, SOUL_BIT_COUNT,
    STAR_BITS, SUB_BITS, SoulBit, solved_filter_bits, soul_bit,
};
use crate::nonempty::NonEmptySet;

/// The souls a scheme picks, group by group, as the game's panel shows them.
///
/// An empty group is kept as chosen, not filled in: the game reads it as no constraint
/// (`scheme-code.md`, "Evaluation"), but writes it as nothing chosen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulSelection {
    /// 类型.
    pub sets: SetChoice,
    /// 位置.
    pub slots: BTreeSet<SoulSlot>,
    /// 星级.
    pub stars: BTreeSet<Star>,
    /// 等级.
    pub levels: BTreeSet<LevelBand>,
    /// 主属性.
    pub main_attributes: BTreeSet<SoulAttribute>,
    /// 固有属性; the game enables the group once a boss soul is chosen.
    pub innate: BTreeSet<InnateAttribute>,
    /// 副属性: ○, ✕, or neither, per attribute.
    pub sub_attributes: SubAttributeModes,
    /// 数量.
    pub sub_counts: BTreeSet<SubCount>,
}

impl SoulSelection {
    /// A selection of `sets` with every other group empty.
    pub fn new(sets: SetChoice) -> SoulSelection {
        SoulSelection {
            sets,
            slots: BTreeSet::new(),
            stars: BTreeSet::new(),
            levels: BTreeSet::new(),
            main_attributes: BTreeSet::new(),
            innate: BTreeSet::new(),
            sub_attributes: SubAttributeModes::default(),
            sub_counts: BTreeSet::new(),
        }
    }
}

/// 类型: the souls a selection chooses.
///
/// `AnySet` is not `Sets(every set)`: the game writes them differently and a round trip keeps
/// them apart. A discard scheme cannot hold `AnySet` ([`super::code::DiscardScheme`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetChoice {
    /// 全部: no soul chosen, so no restriction. Written as a soul mask with no bit set; the one
    /// encoding of "all souls".
    AnySet,
    /// The souls chosen: at least one, each a set the bit table maps or a bit beyond it.
    Sets(NonEmptySet<SetBit>),
}

impl SoulSelection {
    /// Whether 类型 chooses a soul bit beyond the mapped sets.
    pub fn has_unmapped_sets(&self) -> bool {
        match &self.sets {
            SetChoice::AnySet => false,
            SetChoice::Sets(bits) => bits.iter().any(|b| matches!(b, SetBit::Unmapped(_))),
        }
    }
}

impl SetChoice {
    /// The choice of these mapped sets: `AnySet` if there are none, since no soul chosen is all
    /// souls.
    pub fn of(sets: impl IntoIterator<Item = SchemeSet>) -> SetChoice {
        NonEmptySet::collect(sets.into_iter().map(SetBit::Mapped))
            .map_or(SetChoice::AnySet, SetChoice::Sets)
    }
}

/// One chosen soul of 类型, as the soul mask holds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SetBit {
    /// A set the bit table maps.
    Mapped(SchemeSet),
    /// A set bit beyond the mapped sets: a soul the model cannot name, written back as read.
    Unmapped(UnmappedSoulBit),
}

/// A set the scheme's bit table maps: one a scheme can choose. Held as its bit, so writing it
/// needs no lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemeSet(SoulBit);

impl SchemeSet {
    /// `set`, or `None` for a suit code the bit table does not hold.
    pub fn new(set: SoulSet) -> Option<SchemeSet> {
        soul_bit(set).map(SchemeSet)
    }

    pub fn set(self) -> SoulSet {
        self.0.set()
    }
}

/// A soul-mask bit beyond the mapped sets, within a mask's longest length.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnmappedSoulBit(u16);

impl UnmappedSoulBit {
    /// `bit`, or `None` for a mapped bit or one past the longest mask.
    pub fn new(bit: u16) -> Option<UnmappedSoulBit> {
        (SOUL_BIT_COUNT..MAX_MASK_BITS)
            .contains(&bit)
            .then_some(UnmappedSoulBit(bit))
    }

    pub fn index(self) -> u16 {
        self.0
    }
}

/// Bits in the longest soul mask a record can hold.
const MAX_MASK_BITS: u16 = (MAX_FIELD_LEN * 8) as u16;

/// One sub-attribute's state in the 副属性 group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubAttributeMode {
    /// Neither ○ nor ✕: the attribute is not filtered on.
    Ignore,
    /// ○.
    Include,
    /// ✕.
    Exclude,
}

/// The 副属性 group: a mode per attribute, `Ignore` unless set. `Ignore` is never stored, so two
/// groups showing the same marks are equal.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SubAttributeModes(BTreeMap<SoulAttribute, SubAttributeMode>);

impl SubAttributeModes {
    pub fn get(&self, attribute: SoulAttribute) -> SubAttributeMode {
        self.0
            .get(&attribute)
            .copied()
            .unwrap_or(SubAttributeMode::Ignore)
    }

    pub fn set(&mut self, attribute: SoulAttribute, mode: SubAttributeMode) {
        match mode {
            SubAttributeMode::Ignore => self.0.remove(&attribute),
            _ => self.0.insert(attribute, mode),
        };
    }

    /// The attributes marked `mode`, in attribute order.
    pub fn with(&self, mode: SubAttributeMode) -> impl Iterator<Item = SoulAttribute> + '_ {
        self.0.iter().filter(move |e| *e.1 == mode).map(|e| *e.0)
    }
}

/// 等级: the six level bands of the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LevelBand {
    L0to2,
    L3to5,
    L6to8,
    L9to11,
    L12to14,
    L15,
}

impl LevelBand {
    pub const ALL: [LevelBand; 6] = [
        LevelBand::L0to2,
        LevelBand::L3to5,
        LevelBand::L6to8,
        LevelBand::L9to11,
        LevelBand::L12to14,
        LevelBand::L15,
    ];

    /// The band a level falls in, by the editor's labels 0–2 … 15. Every level is in one.
    pub fn of(level: Level) -> LevelBand {
        match level.get() {
            0..=2 => LevelBand::L0to2,
            3..=5 => LevelBand::L3to5,
            6..=8 => LevelBand::L6to8,
            9..=11 => LevelBand::L9to11,
            12..=14 => LevelBand::L12to14,
            _ => LevelBand::L15,
        }
    }
}

/// 数量: the editor's sub-attribute count choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SubCount {
    /// 不足2条.
    FewerThanTwo,
    /// 2条.
    Two,
    /// 3条.
    Three,
    /// 4条.
    Four,
}

impl SubCount {
    pub const ALL: [SubCount; 4] = [
        SubCount::FewerThanTwo,
        SubCount::Two,
        SubCount::Three,
        SubCount::Four,
    ];

    /// The choice a soul with `subs` sub-attributes falls in, by the editor's labels; `None`
    /// above four.
    pub fn of(subs: usize) -> Option<SubCount> {
        match subs {
            0 | 1 => Some(SubCount::FewerThanTwo),
            2 => Some(SubCount::Two),
            3 => Some(SubCount::Three),
            4 => Some(SubCount::Four),
            _ => None,
        }
    }
}

/// What a record holds beyond its selection: its soul mask's length, and its filter as read. The
/// semantic codec writes a selection over these bytes, so every filter bit it does not model, and
/// each field's length, survives. Every soul bit, mapped or not, is the selection's. Opaque: the
/// only fact it exposes is whether the filter selects on something unmodelled.
///
/// Two values are equal when they hold the same unmapped filter bits: the rest belongs to the
/// selection, and a field's length is layout, which the layout level compares byte for byte.
#[derive(Debug, Clone, Default)]
pub struct Preserved {
    soul_mask: Vec<u8>,
    filter: Vec<u8>,
}

impl PartialEq for Preserved {
    fn eq(&self, other: &Preserved) -> bool {
        self.unmapped() == other.unmapped()
    }
}

impl Eq for Preserved {}

impl Preserved {
    /// The filter bits outside every solved group that are set.
    fn unmapped(&self) -> BTreeSet<u16> {
        let solved: BTreeSet<u16> = solved_filter_bits().collect();
        ones(&self.filter).filter(|b| !solved.contains(b)).collect()
    }

    /// Nothing preserved: the base of a selection built from scratch.
    pub fn none() -> Preserved {
        Preserved::default()
    }

    /// Whether a filter bit outside every solved group is set: the record then selects on a
    /// condition the model cannot see.
    pub fn has_unknown_filter(&self) -> bool {
        !self.unmapped().is_empty()
    }
}

/// Why a record has no selection, or a selection cannot be written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionError {
    /// Both ○ and ✕ are set for one attribute, which the editor cannot show.
    IncludeAndExclude(SoulAttribute),
}

fn get(bytes: &[u8], bit: u16) -> bool {
    let bit = usize::from(bit);
    bytes.get(bit / 8).is_some_and(|b| b >> (bit % 8) & 1 == 1)
}

/// Set or clear one bit; a field grows for a set bit beyond its end and never shrinks. Every bit
/// written is a filter bit below 72 or a soul bit below `MAX_MASK_BITS`, so a field stays within
/// its one-byte length.
fn put(bytes: &mut Vec<u8>, bit: u16, on: bool) {
    let bit = usize::from(bit);
    if bytes.len() <= bit / 8 {
        if !on {
            return;
        }
        bytes.resize(bit / 8 + 1, 0);
    }
    let mask = 1u8 << (bit % 8);
    if on {
        bytes[bit / 8] |= mask;
    } else {
        bytes[bit / 8] &= !mask;
    }
}

fn ones(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    (0..bytes.len() * 8)
        .filter_map(|b| u16::try_from(b).ok())
        .filter(move |&b| get(bytes, b))
}

/// The values of a one-bit-per-value group that are set in `filter`.
fn read_group<T: Copy + Ord>(filter: &[u8], table: &[(T, FilterBit)]) -> BTreeSet<T> {
    table
        .iter()
        .filter(|e| get(filter, e.1.index()))
        .map(|e| e.0)
        .collect()
}

/// Write a one-bit-per-value group: each value's bit set exactly when it is chosen.
fn write_group<T: Ord>(filter: &mut Vec<u8>, table: &[(T, FilterBit)], chosen: &BTreeSet<T>) {
    for (value, bit) in table {
        put(filter, bit.index(), chosen.contains(value));
    }
}

/// The soul-mask bit a set position is: mapped to a set, or beyond the mapped sets. A mask holds
/// at most `MAX_FIELD_LEN` bytes, so every position read from one has an answer.
fn set_bit(bit: u16) -> Option<SetBit> {
    match SoulBit::new(bit) {
        Some(mapped) => Some(SetBit::Mapped(SchemeSet(mapped))),
        None => UnmappedSoulBit::new(bit).map(SetBit::Unmapped),
    }
}

/// The selection a record encodes, and what it preserves. A mask with no bit set is `AnySet`.
pub fn decode_selection(record: &Record) -> Result<(SoulSelection, Preserved), SelectionError> {
    let mask = record.soul_mask();
    let filter = record.filter();
    let sets = NonEmptySet::collect(ones(mask).filter_map(set_bit))
        .map_or(SetChoice::AnySet, SetChoice::Sets);
    let mut sub_attributes = SubAttributeModes::default();
    for &(attribute, bits) in &SUB_BITS {
        let mode = match (
            get(filter, bits.include.index()),
            get(filter, bits.exclude.index()),
        ) {
            (true, true) => return Err(SelectionError::IncludeAndExclude(attribute)),
            (true, false) => SubAttributeMode::Include,
            (false, true) => SubAttributeMode::Exclude,
            (false, false) => SubAttributeMode::Ignore,
        };
        sub_attributes.set(attribute, mode);
    }
    let selection = SoulSelection {
        sets,
        slots: read_group(filter, &SLOT_BITS),
        stars: read_group(filter, &STAR_BITS),
        levels: read_group(filter, &LEVEL_BITS),
        main_attributes: read_group(filter, &MAIN_BITS),
        innate: read_group(filter, &INNATE_BITS),
        sub_attributes,
        sub_counts: read_group(filter, &COUNT_BITS),
    };
    let preserved = Preserved {
        soul_mask: mask.to_vec(),
        filter: filter.to_vec(),
    };
    Ok((selection, preserved))
}

/// The soul mask and filter of `selection`, written over `preserved`: every soul bit and every
/// mapped filter bit takes the selection's value, every other filter bit and each field's length
/// are kept, and a field grows only for a set bit beyond its end. With [`Preserved::none`] the
/// fields are trimmed to their highest set bit, as the game writes them. Every selection can be
/// written.
pub fn encode_selection(selection: &SoulSelection, preserved: &Preserved) -> (Vec<u8>, Vec<u8>) {
    let mut soul_mask = vec![0; preserved.soul_mask.len()];
    if let SetChoice::Sets(sets) = &selection.sets {
        for bit in sets {
            let index = match *bit {
                SetBit::Mapped(set) => set.0.index(),
                SetBit::Unmapped(b) => b.index(),
            };
            put(&mut soul_mask, index, true);
        }
    }
    let mut filter = preserved.filter.clone();
    let f = &mut filter;
    write_group(f, &SLOT_BITS, &selection.slots);
    write_group(f, &STAR_BITS, &selection.stars);
    write_group(f, &MAIN_BITS, &selection.main_attributes);
    for &(attribute, bits) in &SUB_BITS {
        let mode = selection.sub_attributes.get(attribute);
        put(f, bits.include.index(), mode == SubAttributeMode::Include);
        put(f, bits.exclude.index(), mode == SubAttributeMode::Exclude);
    }
    write_group(f, &COUNT_BITS, &selection.sub_counts);
    write_group(f, &LEVEL_BITS, &selection.levels);
    write_group(f, &INNATE_BITS, &selection.innate);
    (soul_mask, filter)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest::sample::subsequence;

    use super::*;

    fn record(soul_mask: &[u8], filter: &[u8]) -> Record {
        Record::new("x", soul_mask.to_vec(), filter.to_vec()).expect("short fields")
    }

    fn bits(bytes: &[u8]) -> BTreeSet<u16> {
        ones(bytes).collect()
    }

    fn with_bits(set: &[u16]) -> Vec<u8> {
        let mut v = Vec::new();
        for &b in set {
            put(&mut v, b, true);
        }
        v
    }

    fn any_set() -> SoulSelection {
        SoulSelection::new(SetChoice::AnySet)
    }

    /// The filter of a selection written from scratch, as bit positions.
    fn filter_of(selection: &SoulSelection) -> BTreeSet<u16> {
        let (_, filter) = encode_selection(selection, &Preserved::none());
        bits(&filter)
    }

    fn decoded(filter: &[u16]) -> SoulSelection {
        decode_selection(&record(&[], &with_bits(filter)))
            .expect("decodable")
            .0
    }

    /// One value's selection is exactly its bit, and that bit alone decodes to it.
    fn assert_one_bit(s: &SoulSelection, bit: u16) {
        assert_eq!(filter_of(s), BTreeSet::from([bit]), "{s:?}");
        assert_eq!(&decoded(&[bit]), s, "bit {bit}");
    }

    // A and B: every value of every group, both ways. The expected positions are the research
    // record's tables, restated here independently of `mapping`.

    #[test]
    fn every_slot_is_its_bit() {
        for (bit, slot) in (0u16..).zip(SoulSlot::ALL) {
            let mut s = any_set();
            s.slots.insert(slot);
            assert_one_bit(&s, bit);
        }
    }

    #[test]
    fn every_star_is_its_bit() {
        for star in Star::ALL {
            let mut s = any_set();
            s.stars.insert(star);
            assert_one_bit(&s, 5 + u16::from(star.get()));
        }
    }

    #[test]
    fn every_main_attribute_is_its_bit() {
        for (bit, a) in (12u16..).zip(SoulAttribute::ALL) {
            let mut s = any_set();
            s.main_attributes.insert(a);
            assert_one_bit(&s, bit);
        }
    }

    #[test]
    fn every_sub_attribute_includes_and_excludes_at_its_pair() {
        for (include, a) in (23u16..).step_by(2).zip(SoulAttribute::ALL) {
            for (mode, bit) in [
                (SubAttributeMode::Include, include),
                (SubAttributeMode::Exclude, include + 1),
            ] {
                let mut s = any_set();
                s.sub_attributes.set(a, mode);
                assert_one_bit(&s, bit);
            }
        }
    }

    #[test]
    fn every_count_option_is_its_bit() {
        for (bit, c) in (45u16..).zip(SubCount::ALL) {
            let mut s = any_set();
            s.sub_counts.insert(c);
            assert_one_bit(&s, bit);
        }
    }

    #[test]
    fn every_level_band_is_its_bit() {
        for (bit, l) in (49u16..).zip(LevelBand::ALL) {
            let mut s = any_set();
            s.levels.insert(l);
            assert_one_bit(&s, bit);
        }
    }

    #[test]
    fn every_innate_option_is_its_bit() {
        for (bit, i) in (55u16..).zip(InnateAttribute::ALL) {
            let mut s = any_set();
            s.innate.insert(i);
            assert_one_bit(&s, bit);
        }
    }

    #[test]
    fn innate_attributes_are_the_six_of_the_editor() {
        let six = [
            SoulAttribute::AtkPercent,
            SoulAttribute::DefPercent,
            SoulAttribute::HpPercent,
            SoulAttribute::EffectHit,
            SoulAttribute::EffectRes,
            SoulAttribute::Crit,
        ];
        for a in SoulAttribute::ALL {
            let innate = InnateAttribute::new(a);
            assert_eq!(innate.is_some(), six.contains(&a), "{a:?}");
            assert_eq!(innate.map(InnateAttribute::attribute).unwrap_or(a), a);
        }
    }

    #[test]
    fn sub_counts_follow_the_editor_labels() {
        let expected = [
            (0, Some(SubCount::FewerThanTwo)),
            (1, Some(SubCount::FewerThanTwo)),
            (2, Some(SubCount::Two)),
            (3, Some(SubCount::Three)),
            (4, Some(SubCount::Four)),
            (5, None),
        ];
        for (subs, count) in expected {
            assert_eq!(SubCount::of(subs), count, "{subs}");
        }
    }

    #[test]
    fn level_bands_follow_the_editor_labels() {
        use LevelBand::*;
        let expected = [
            (0, L0to2),
            (2, L0to2),
            (3, L3to5),
            (5, L3to5),
            (6, L6to8),
            (8, L6to8),
            (9, L9to11),
            (11, L9to11),
            (12, L12to14),
            (14, L12to14),
            (15, L15),
        ];
        for (level, band) in expected {
            let l = Level::new(level).expect("a level");
            assert_eq!(LevelBand::of(l), band, "{level}");
        }
    }

    // Sets.

    fn sets(codes: &[u8]) -> SetChoice {
        SetChoice::of(
            codes
                .iter()
                .map(|&c| SchemeSet::new(SoulSet::from_suit_code(c)).expect("a mapped set")),
        )
    }

    fn mapped(bit: u16) -> SetBit {
        SetBit::Mapped(SchemeSet(SoulBit::new(bit).expect("a mapped bit")))
    }

    fn unmapped(bit: u16) -> SetBit {
        SetBit::Unmapped(UnmappedSoulBit::new(bit).expect("an unmapped bit"))
    }

    #[test]
    fn chosen_sets_are_written_through_their_suit_codes() {
        // 招财猫 (code 10) is bit 7, 针女 (36) bit 27, 薙魂 (21) bit 29: not in code order.
        let s = SoulSelection::new(sets(&[10, 36, 21]));
        let (mask, _) = encode_selection(&s, &Preserved::none());
        assert_eq!(bits(&mask), BTreeSet::from([7, 27, 29]));
        let (back, _) = decode_selection(&record(&mask, &[])).expect("ok");
        assert_eq!(back, s);
    }

    #[test]
    fn any_set_is_not_every_set() {
        let (mask, _) = encode_selection(&any_set(), &Preserved::none());
        assert!(mask.is_empty());
        let every = NonEmptySet::collect((0..SOUL_BIT_COUNT).map(mapped)).expect("70");
        let all = SoulSelection::new(SetChoice::Sets(every));
        let (mask, _) = encode_selection(&all, &Preserved::none());
        // The game's discard "all souls": FF × 8, then 3F.
        assert_eq!(mask, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3f]);
        let (back, _) = decode_selection(&record(&mask, &[])).expect("ok");
        assert_eq!(back, all);
    }

    #[test]
    fn no_soul_chosen_is_any_set_however_long_the_mask() {
        // "All souls" has one meaning: no soul chosen. An empty mask and a mask of zero bytes are
        // both it, and each is written back as it was read.
        for mask in [&[][..], &[0], &[0, 0, 0]] {
            let (s, preserved) = decode_selection(&record(mask, &[])).expect("ok");
            assert_eq!(s.sets, SetChoice::AnySet, "{mask:?}");
            let (back, _) = encode_selection(&s, &preserved);
            assert_eq!(back, mask);
        }
    }

    #[test]
    fn unmapped_soul_bits_are_chosen_souls_not_no_choice() {
        // A mask of unmapped bits only chooses souls the model cannot name. It is never read as
        // no soul chosen (all souls), and the bits are written back as read.
        let (s, preserved) = decode_selection(&record(&with_bits(&[70, 75]), &[])).expect("ok");
        let chosen = NonEmptySet::collect([unmapped(70), unmapped(75)]).expect("two");
        assert_eq!(s.sets, SetChoice::Sets(chosen));
        assert!(s.has_unmapped_sets());
        assert!(!preserved.has_unknown_filter());
        let (mask, _) = encode_selection(&s, &preserved);
        assert_eq!(bits(&mask), BTreeSet::from([70, 75]));
        // Beside a mapped set.
        let (s, _) = decode_selection(&record(&with_bits(&[3, 70]), &[])).expect("ok");
        let chosen = NonEmptySet::collect([mapped(3), unmapped(70)]).expect("two");
        assert_eq!(s.sets, SetChoice::Sets(chosen));
    }

    #[test]
    fn choices_the_game_cannot_hold_cannot_be_built() {
        // A suit code with no scheme bit is no `SchemeSet`, and an unmapped bit is none of the 70.
        assert_eq!(SchemeSet::new(SoulSet::from_suit_code(1)), None);
        assert!(SchemeSet::new(SoulSet::from_suit_code(30)).is_some());
        assert_eq!(UnmappedSoulBit::new(69), None);
        assert!(UnmappedSoulBit::new(70).is_some());
        assert_eq!(UnmappedSoulBit::new(255 * 8), None);
        // No mapped set chosen is all souls.
        assert_eq!(SetChoice::of([]), SetChoice::AnySet);
    }

    #[test]
    fn records_the_editor_cannot_produce_are_refused() {
        assert_eq!(
            decode_selection(&record(&[1], &with_bits(&[41, 42]))),
            Err(SelectionError::IncludeAndExclude(SoulAttribute::Crit))
        );
    }

    // Preserved bits.

    #[test]
    fn unmapped_bits_are_kept_by_an_edit_of_another_group() {
        // Filter bit 61 and soul bit 70: never seen set. The filter bit is preserved, the soul bit
        // is part of 类型; both survive an edit of 位置.
        let record = record(&with_bits(&[3, 70]), &with_bits(&[0, 61]));
        let (mut s, preserved) = decode_selection(&record).expect("ok");
        assert!(preserved.has_unknown_filter());
        assert!(!Preserved::none().has_unknown_filter());
        assert_eq!(s.slots, BTreeSet::from([SoulSlot::Slot1]));
        s.slots = BTreeSet::from([SoulSlot::Slot6]);
        let (mask, filter) = encode_selection(&s, &preserved);
        assert_eq!(bits(&filter), BTreeSet::from([5, 61]));
        assert_eq!(bits(&mask), BTreeSet::from([3, 70]));
        // An edit of 类型 is the new choice, unmapped bits included.
        s.sets = sets(&[30]);
        let (mask, _) = encode_selection(&s, &preserved);
        assert_eq!(bits(&mask), BTreeSet::from([21]));
    }

    #[test]
    fn a_field_keeps_its_length_and_grows_only_for_a_set_bit() {
        let game_like = record(&with_bits(&[39]), &[1, 0, 0, 0, 0, 0, 0]);
        let (mut s, preserved) = decode_selection(&game_like).expect("ok");
        s.slots.clear();
        let (_, filter) = encode_selection(&s, &preserved);
        assert_eq!(filter, [0; 7]);
        s.innate.insert(InnateAttribute::ALL[5]);
        let (_, filter) = encode_selection(&s, &preserved);
        assert_eq!(filter.len(), 8);
    }

    // Round trips and write sets, by property.

    fn arb_selection() -> impl Strategy<Value = SoulSelection> {
        let sets = prop_oneof![
            Just(SetChoice::AnySet),
            proptest::collection::btree_set(0..SOUL_BIT_COUNT, 1..6).prop_map(|b| SetChoice::Sets(
                NonEmptySet::collect(b.into_iter().map(mapped)).expect("some"),
            )),
        ];
        let mode = prop_oneof![
            Just(SubAttributeMode::Ignore),
            Just(SubAttributeMode::Include),
            Just(SubAttributeMode::Exclude),
        ];
        (
            sets,
            subsequence(SoulSlot::ALL.to_vec(), 0..=6),
            subsequence(Star::ALL.to_vec(), 0..=6),
            subsequence(LevelBand::ALL.to_vec(), 0..=6),
            subsequence(SoulAttribute::ALL.to_vec(), 0..=11),
            subsequence(InnateAttribute::ALL.to_vec(), 0..=6),
            proptest::collection::vec(mode, 11),
            subsequence(SubCount::ALL.to_vec(), 0..=4),
        )
            .prop_map(
                |(sets, slots, stars, levels, mains, innate, modes, counts)| {
                    let mut sub_attributes = SubAttributeModes::default();
                    for (a, m) in SoulAttribute::ALL.into_iter().zip(modes) {
                        sub_attributes.set(a, m);
                    }
                    SoulSelection {
                        sets,
                        slots: slots.into_iter().collect(),
                        stars: stars.into_iter().collect(),
                        levels: levels.into_iter().collect(),
                        main_attributes: mains.into_iter().collect(),
                        innate: innate.into_iter().collect(),
                        sub_attributes,
                        sub_counts: counts.into_iter().collect(),
                    }
                },
            )
    }

    /// Unmapped bits only, to lie under a selection: filter bits 61–71, and soul bits 70–79 that
    /// only lengthen the mask, since every soul bit is the selection's.
    fn arb_unknown() -> impl Strategy<Value = Preserved> {
        (
            proptest::collection::vec(61u16..72, 0..3),
            proptest::collection::vec(70u16..80, 0..2),
        )
            .prop_map(|(f, m)| Preserved {
                soul_mask: with_bits(&m),
                filter: with_bits(&f),
            })
    }

    fn changed(a: &[u8], b: &[u8]) -> BTreeSet<u16> {
        bits(a).symmetric_difference(&bits(b)).copied().collect()
    }

    fn group_bits<T>(table: &[(T, FilterBit)]) -> BTreeSet<u16> {
        table.iter().map(|e| e.1.index()).collect()
    }

    /// Each single-group edit of `base` towards `other`, with the filter bits it may change.
    fn single_group_edits(
        base: &SoulSelection,
        other: &SoulSelection,
        attribute: usize,
    ) -> Vec<(SoulSelection, BTreeSet<u16>)> {
        let (a, bits) = SUB_BITS[attribute];
        let (include, exclude) = (bits.include.index(), bits.exclude.index());
        let mut sub = base.clone();
        sub.sub_attributes.set(a, other.sub_attributes.get(a));
        let b = || base.clone();
        vec![
            (
                SoulSelection {
                    slots: other.slots.clone(),
                    ..b()
                },
                group_bits(&SLOT_BITS),
            ),
            (
                SoulSelection {
                    stars: other.stars.clone(),
                    ..b()
                },
                group_bits(&STAR_BITS),
            ),
            (
                SoulSelection {
                    levels: other.levels.clone(),
                    ..b()
                },
                group_bits(&LEVEL_BITS),
            ),
            (
                SoulSelection {
                    main_attributes: other.main_attributes.clone(),
                    ..b()
                },
                group_bits(&MAIN_BITS),
            ),
            (
                SoulSelection {
                    innate: other.innate.clone(),
                    ..b()
                },
                group_bits(&INNATE_BITS),
            ),
            (
                SoulSelection {
                    sub_counts: other.sub_counts.clone(),
                    ..b()
                },
                group_bits(&COUNT_BITS),
            ),
            (sub, BTreeSet::from([include, exclude])),
            (
                SoulSelection {
                    sets: other.sets.clone(),
                    ..b()
                },
                BTreeSet::new(),
            ),
        ]
    }

    proptest! {
        #[test]
        fn decoding_what_was_encoded_gives_the_selection_back(s in arb_selection()) {
            let (mask, filter) = encode_selection(&s, &Preserved::none());
            let (back, preserved) = decode_selection(&record(&mask, &filter)).expect("ok");
            prop_assert_eq!(back, s);
            prop_assert!(!preserved.has_unknown_filter());
        }

        #[test]
        fn an_unedited_record_is_written_back_byte_for_byte(
            mask in proptest::collection::vec(any::<u8>(), 0..11),
            filter in proptest::collection::vec(any::<u8>(), 0..10),
        ) {
            if let Ok((s, preserved)) = decode_selection(&record(&mask, &filter)) {
                let (m, f) = encode_selection(&s, &preserved);
                prop_assert_eq!(m, mask);
                prop_assert_eq!(f, filter);
            }
        }

        #[test]
        fn changing_one_group_changes_only_its_bits(
            base in arb_selection(),
            other in arb_selection(),
            unknown in arb_unknown(),
            attribute in 0usize..11,
        ) {
            let encode = |s: &SoulSelection| encode_selection(s, &unknown);
            let (base_mask, base_filter) = encode(&base);
            for (edited, allowed) in single_group_edits(&base, &other, attribute) {
                let (mask, filter) = encode(&edited);
                let f = changed(&base_filter, &filter);
                prop_assert!(f.is_subset(&allowed), "filter bits {:?} outside {:?}", f, allowed);
                if edited.sets == base.sets {
                    prop_assert_eq!(&mask, &base_mask);
                } else {
                    prop_assert!(changed(&base_mask, &mask).iter().all(|&b| b < SOUL_BIT_COUNT));
                }
                let (back, preserved) = decode_selection(&record(&mask, &filter)).expect("ok");
                prop_assert_eq!(back, edited);
                prop_assert_eq!(preserved.has_unknown_filter(), unknown.has_unknown_filter());
            }
        }
    }
}
