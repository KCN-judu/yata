//! A scheme code as the game's editor holds it: a strengthening scheme set of named plans, or
//! discard schemes, each a name and a [`SoulSelection`] (`scheme-code.md`, "The model").
//!
//! The containers carry names and order; the selection carries the filtering; what neither
//! models stays in each record's [`Preserved`]. The header is the layout's: [`decode_code`] reads
//! the records of a [`SchemeLayout`], and [`encode_code`] writes them under the account it is given.

use super::layout::{
    AccountSegment, DiscardCannotSelectAll, DiscardRecord, LayoutError, Record, SchemeLayout,
};
use super::name::{NameTooLong, SchemeName};
use super::selection::{
    Preserved, SelectionError, SetChoice, SoulSelection, decode_selection, encode_selection,
};
use crate::nonempty::NonEmpty;

/// A scheme code's content, by kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemeCode {
    Strengthening(StrengtheningSchemeSet),
    /// One or more discard schemes: the game exported a discard code with two (2026-09-24).
    Discard(NonEmpty<DiscardScheme>),
}

/// 强化方案: the plans of one code, in code order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StrengtheningSchemeSet {
    pub plans: Vec<StrengtheningPlan>,
}

/// 强化子方案: a name and a selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrengtheningPlan {
    pub name: SchemeName,
    pub selection: SoulSelection,
    preserved: Preserved,
}

/// 弃置方案: a name and a selection that names its souls. The selection is reached through
/// [`DiscardScheme::selection`], so it can never be `AnySet` ([`DiscardCannotSelectAll`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardScheme {
    pub name: SchemeName,
    selection: SoulSelection,
    preserved: Preserved,
}

impl StrengtheningPlan {
    /// A new plan, with nothing preserved.
    pub fn new(name: SchemeName, selection: SoulSelection) -> StrengtheningPlan {
        StrengtheningPlan {
            name,
            selection,
            preserved: Preserved::none(),
        }
    }
}

impl DiscardScheme {
    /// A new discard scheme, with nothing preserved; refused if it chooses all souls.
    pub fn new(
        name: SchemeName,
        selection: SoulSelection,
    ) -> Result<DiscardScheme, DiscardCannotSelectAll> {
        DiscardScheme::with_preserved(name, selection, Preserved::none())
    }

    fn with_preserved(
        name: SchemeName,
        selection: SoulSelection,
        preserved: Preserved,
    ) -> Result<DiscardScheme, DiscardCannotSelectAll> {
        match selection.sets {
            SetChoice::AnySet => Err(DiscardCannotSelectAll),
            SetChoice::Sets(_) | SetChoice::OnlyUnmapped => Ok(DiscardScheme {
                name,
                selection,
                preserved,
            }),
        }
    }

    pub fn selection(&self) -> &SoulSelection {
        &self.selection
    }
}

macro_rules! scheme_entry {
    ($t:ty) => {
        impl $t {
            /// What the record this entry was decoded from held beyond its selection.
            pub fn preserved(&self) -> &Preserved {
                &self.preserved
            }

            /// Whether the entry selects on a condition the model cannot see; its evaluation is
            /// then never exact (`scheme-code.md`, "Evaluation").
            pub fn has_unknown_conditions(&self) -> bool {
                self.preserved.has_unknown_conditions()
            }
        }
    };
}

scheme_entry!(StrengtheningPlan);
scheme_entry!(DiscardScheme);

/// Why a layout has no scheme code, or a scheme code cannot be written. `record` is the index of
/// the plan or scheme at fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeError {
    /// A name that is not UTF-8.
    NameNotUtf8 { record: usize },
    /// A name the game refuses on import ([`SchemeName`]).
    NameTooLong { record: usize, length: NameTooLong },
    Selection {
        record: usize,
        error: SelectionError,
    },
    /// A discard scheme that chooses all souls.
    DiscardCannotSelectAll { record: usize },
    /// A record that cannot be written ([`LayoutError`]).
    Layout { record: usize, error: LayoutError },
}

/// An entry's name, selection, and preserved bits, from its record.
fn entry(i: usize, record: &Record) -> Result<(SchemeName, SoulSelection, Preserved), CodeError> {
    let name = record.name().ok_or(CodeError::NameNotUtf8 { record: i })?;
    let name =
        SchemeName::new(name).map_err(|length| CodeError::NameTooLong { record: i, length })?;
    let (selection, preserved) =
        decode_selection(record).map_err(|error| CodeError::Selection { record: i, error })?;
    Ok((name, selection, preserved))
}

/// The scheme code a layout holds.
pub fn decode_code(layout: &SchemeLayout) -> Result<SchemeCode, CodeError> {
    Ok(match layout {
        SchemeLayout::Strengthening { plans, .. } => {
            SchemeCode::Strengthening(StrengtheningSchemeSet {
                plans: plans
                    .iter()
                    .enumerate()
                    .map(|(i, r)| {
                        let (name, selection, preserved) = entry(i, r)?;
                        Ok(StrengtheningPlan {
                            name,
                            selection,
                            preserved,
                        })
                    })
                    .collect::<Result<_, CodeError>>()?,
            })
        }
        SchemeLayout::Discard { schemes, .. } => {
            SchemeCode::Discard(schemes.try_map_enumerated(|i, r| {
                let (name, selection, preserved) = entry(i, r.record())?;
                DiscardScheme::with_preserved(name, selection, preserved).map_err(
                    |DiscardCannotSelectAll| CodeError::DiscardCannotSelectAll { record: i },
                )
            })?)
        }
    })
}

/// The record of one entry.
fn record_of(
    i: usize,
    name: &SchemeName,
    selection: &SoulSelection,
    preserved: &Preserved,
) -> Result<Record, CodeError> {
    let (soul_mask, filter) = encode_selection(selection, preserved)
        .map_err(|error| CodeError::Selection { record: i, error })?;
    Record::new(name.as_str(), soul_mask, filter)
        .map_err(|error| CodeError::Layout { record: i, error })
}

/// The layout of a scheme code, with `account` in its header.
pub fn encode_code(code: &SchemeCode, account: AccountSegment) -> Result<SchemeLayout, CodeError> {
    Ok(match code {
        SchemeCode::Strengthening(set) => SchemeLayout::Strengthening {
            account,
            plans: set
                .plans
                .iter()
                .enumerate()
                .map(|(i, p)| record_of(i, &p.name, &p.selection, &p.preserved))
                .collect::<Result<_, _>>()?,
        },
        SchemeCode::Discard(schemes) => SchemeLayout::Discard {
            account,
            schemes: schemes.try_map_enumerated(|i, d| {
                let r = record_of(i, &d.name, &d.selection, &d.preserved)?;
                DiscardRecord::new(r).map_err(|DiscardCannotSelectAll| {
                    CodeError::DiscardCannotSelectAll { record: i }
                })
            })?,
        },
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::super::edit::{FilterBit, SoulBit, SoulChoice};
    use super::super::layout::{SchemeKind, parse, serialize};
    use super::super::selection::{InnateAttribute, LevelBand, SubAttributeMode};
    use super::*;
    use crate::nonempty::NonEmptySet;
    use crate::soul::{SoulAttribute, SoulSet, SoulSlot};

    fn account() -> AccountSegment {
        AccountSegment::from_bytes([7; 14])
    }

    fn name(s: &str) -> SchemeName {
        SchemeName::new(s).expect("an importable name")
    }

    fn one_set(code: u8) -> SetChoice {
        SetChoice::Sets(NonEmptySet::one(SoulSet::from_suit_code(code)))
    }

    fn raw(n: &str, souls: &[u16], filter: &[u16]) -> Record {
        let souls = NonEmptySet::collect(souls.iter().filter_map(|&b| SoulBit::new(b)))
            .map_or(SoulChoice::All, SoulChoice::Souls);
        let filter: Vec<FilterBit> = filter.iter().filter_map(|&b| FilterBit::new(b)).collect();
        Record::from_bits(&name(n), &souls, &filter)
    }

    #[test]
    fn the_discard_schemes_the_game_exported_read_as_the_editor_showed_them() {
        // The game's discard export of 2026-09-24, rebuilt from its bits (see `edit`).
        let layout = SchemeLayout::discard(
            account(),
            vec![
                raw("二号位双速招财", &[7], &[1, 18, 35]),
                raw("一号位针女", &[27], &[0]),
            ],
        )
        .expect("valid");
        let SchemeCode::Discard(schemes) = decode_code(&layout).expect("ok") else {
            panic!("a discard code");
        };
        let [first, second] = schemes.as_slice() else {
            panic!("two schemes");
        };
        assert_eq!(first.name, name("二号位双速招财"));
        assert_eq!(first.selection().sets, one_set(10));
        assert_eq!(first.selection().slots, BTreeSet::from([SoulSlot::Slot2]));
        assert_eq!(
            first.selection().main_attributes,
            BTreeSet::from([SoulAttribute::Spd])
        );
        assert_eq!(
            first.selection().sub_attributes.get(SoulAttribute::Spd),
            SubAttributeMode::Include
        );
        assert_eq!(second.selection().sets, one_set(36));
        assert_eq!(second.selection().slots, BTreeSet::from([SoulSlot::Slot1]));
        let code = SchemeCode::Discard(schemes);
        assert_eq!(encode_code(&code, account()), Ok(layout));
    }

    #[test]
    fn a_set_built_from_selections_reads_back_as_built() {
        let mut spd = SoulSelection::new(SetChoice::AnySet);
        spd.slots = BTreeSet::from([SoulSlot::Slot2]);
        spd.main_attributes = BTreeSet::from([SoulAttribute::Spd]);
        spd.levels = BTreeSet::from([LevelBand::L0to2]);
        let mut boss = SoulSelection::new(one_set(50));
        boss.innate = BTreeSet::from([InnateAttribute::ALL[5]]);
        boss.sub_attributes
            .set(SoulAttribute::HpFlat, SubAttributeMode::Exclude);
        let code = SchemeCode::Strengthening(StrengtheningSchemeSet {
            plans: vec![
                StrengtheningPlan::new(name("二号速度"), spd),
                StrengtheningPlan::new(name("土蜘蛛暴击"), boss),
            ],
        });
        let layout = encode_code(&code, account()).expect("encodable");
        assert_eq!(layout.kind(), SchemeKind::Strengthening);
        let payload = serialize(&layout).expect("writable");
        let back = decode_code(&parse(&payload).expect("parsable")).expect("decodable");
        assert_eq!(back, code);
    }

    #[test]
    fn a_whole_code_with_unmapped_bits_is_written_back_byte_for_byte() {
        let mut filter = vec![0u8; 8];
        filter[7] = 0b0110_0000; // bits 61 and 62
        filter[0] = 0b10; // slot 2
        let plans = vec![
            Record::new("甲", vec![], filter.clone()).expect("short"),
            Record::new("乙", vec![0, 0, 0, 0, 0, 0, 0, 0, 0x40], vec![1]).expect("short"),
        ];
        let layout = SchemeLayout::Strengthening {
            account: account(),
            plans,
        };
        let payload = serialize(&layout).expect("writable");
        let code = decode_code(&parse(&payload).expect("parsable")).expect("decodable");
        let SchemeCode::Strengthening(set) = &code else {
            panic!("a strengthening set");
        };
        assert!(
            set.plans
                .iter()
                .all(StrengtheningPlan::has_unknown_conditions)
        );
        // 乙 chooses soul bit 70 only, which no set maps.
        assert_eq!(set.plans[1].selection.sets, SetChoice::OnlyUnmapped);
        let again = serialize(&encode_code(&code, account()).expect("encodable"));
        assert_eq!(again, Ok(payload));
    }

    #[test]
    fn a_discard_scheme_cannot_choose_all_souls() {
        let any = SoulSelection::new(SetChoice::AnySet);
        assert_eq!(
            DiscardScheme::new(name("全部"), any),
            Err(DiscardCannotSelectAll)
        );
        assert!(DiscardScheme::new(name("一个"), SoulSelection::new(one_set(10))).is_ok());
    }

    #[test]
    fn names_the_game_would_refuse_have_no_scheme_code() {
        let mut long = Record::new("x", vec![], vec![]).expect("short");
        long.name = "一二三四五六七八九十十".as_bytes().to_vec();
        let layout = SchemeLayout::Strengthening {
            account: account(),
            plans: vec![long],
        };
        assert!(matches!(
            decode_code(&layout),
            Err(CodeError::NameTooLong { record: 0, .. })
        ));
    }

    #[test]
    fn a_name_that_is_not_utf8_has_no_scheme_code() {
        let mut record = Record::new("x", vec![], vec![]).expect("short");
        record.name = vec![0xff];
        let layout = SchemeLayout::Strengthening {
            account: account(),
            plans: vec![record],
        };
        assert_eq!(
            decode_code(&layout),
            Err(CodeError::NameNotUtf8 { record: 0 })
        );
    }
}
