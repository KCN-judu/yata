//! A scheme code as the game's editor holds it: a strengthening scheme set of named plans, or
//! discard schemes, each a name and a [`SoulSelection`] (`scheme-code.md`, "The model").
//!
//! The containers carry names and order; the selection carries the filtering; what neither
//! models stays in each record's [`Preserved`]. The header is the layout's: [`decode_code`] reads
//! the records of a [`SchemeLayout`], and [`encode_code`] writes them under the account it is given.

use super::edit::importable_name;
use super::layout::{AccountSegment, Record, SchemeHeader, SchemeKind, SchemeLayout};
use super::selection::{
    Preserved, SelectionError, SoulSelection, decode_selection, encode_selection,
};

/// A scheme code's content, by kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemeCode {
    Strengthening(StrengtheningSchemeSet),
    /// One or more discard schemes: the game exported a discard code with two (2026-09-24).
    Discard(Vec<DiscardScheme>),
}

/// 强化方案: the plans of one code, in code order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StrengtheningSchemeSet {
    pub plans: Vec<StrengtheningPlan>,
}

/// 强化子方案: a name and a selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrengtheningPlan {
    pub name: String,
    pub selection: SoulSelection,
    preserved: Preserved,
}

/// 弃置方案: a name and a selection. The selection cannot be `AnySet`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardScheme {
    pub name: String,
    pub selection: SoulSelection,
    preserved: Preserved,
}

macro_rules! scheme_entry {
    ($t:ty) => {
        impl $t {
            /// A new entry, with nothing preserved.
            pub fn new(name: impl Into<String>, selection: SoulSelection) -> $t {
                Self {
                    name: name.into(),
                    selection,
                    preserved: Preserved::none(),
                }
            }

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
    /// A name the game refuses on import (`edit::MAX_NAME_CHARS`, `edit::MAX_NAME_BYTES`).
    NameTooLong { record: usize },
    Selection {
        record: usize,
        error: SelectionError,
    },
    /// A discard code with no scheme.
    NoDiscardScheme,
}

/// The scheme code a layout holds.
pub fn decode_code(layout: &SchemeLayout) -> Result<SchemeCode, CodeError> {
    let kind = layout.header.kind;
    let mut entries = Vec::with_capacity(layout.records.len());
    for (i, record) in layout.records.iter().enumerate() {
        let name = record
            .name()
            .ok_or(CodeError::NameNotUtf8 { record: i })?
            .to_owned();
        let (selection, preserved) = decode_selection(record, kind)
            .map_err(|error| CodeError::Selection { record: i, error })?;
        entries.push((name, selection, preserved));
    }
    Ok(match kind {
        SchemeKind::Strengthening => SchemeCode::Strengthening(StrengtheningSchemeSet {
            plans: entries
                .into_iter()
                .map(|(name, selection, preserved)| StrengtheningPlan {
                    name,
                    selection,
                    preserved,
                })
                .collect(),
        }),
        SchemeKind::Discard => SchemeCode::Discard(
            entries
                .into_iter()
                .map(|(name, selection, preserved)| DiscardScheme {
                    name,
                    selection,
                    preserved,
                })
                .collect(),
        ),
    })
}

/// The layout of a scheme code, with `account` in its header.
pub fn encode_code(code: &SchemeCode, account: AccountSegment) -> Result<SchemeLayout, CodeError> {
    let (kind, entries): (SchemeKind, Vec<(&str, &SoulSelection, &Preserved)>) = match code {
        SchemeCode::Strengthening(set) => (
            SchemeKind::Strengthening,
            set.plans
                .iter()
                .map(|p| (p.name.as_str(), &p.selection, &p.preserved))
                .collect(),
        ),
        SchemeCode::Discard(schemes) if schemes.is_empty() => {
            return Err(CodeError::NoDiscardScheme);
        }
        SchemeCode::Discard(schemes) => (
            SchemeKind::Discard,
            schemes
                .iter()
                .map(|s| (s.name.as_str(), &s.selection, &s.preserved))
                .collect(),
        ),
    };
    let mut records = Vec::with_capacity(entries.len());
    for (i, (name, selection, preserved)) in entries.into_iter().enumerate() {
        if !importable_name(name) {
            return Err(CodeError::NameTooLong { record: i });
        }
        let at = |error| CodeError::Selection { record: i, error };
        let (soul_mask, filter) = encode_selection(selection, kind, preserved).map_err(at)?;
        records.push(
            Record::new(name, soul_mask, filter).map_err(|_| at(SelectionError::FieldTooLong))?,
        );
    }
    Ok(SchemeLayout {
        header: SchemeHeader { account, kind },
        records,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::super::edit::{FilterBit, SoulBit};
    use super::super::layout::{parse, serialize};
    use super::super::selection::{InnateAttribute, LevelBand, SetChoice, SubAttributeMode};
    use super::*;
    use crate::soul::{SoulAttribute, SoulSet, SoulSlot};

    fn account() -> AccountSegment {
        AccountSegment::from_bytes([7; 14])
    }

    fn raw(name: &str, souls: Option<&[u16]>, filter: &[u16]) -> Record {
        let souls: Option<Vec<SoulBit>> =
            souls.map(|s| s.iter().filter_map(|&b| SoulBit::new(b)).collect());
        let filter: Vec<FilterBit> = filter.iter().filter_map(|&b| FilterBit::new(b)).collect();
        Record::from_bits(name, souls.as_deref(), &filter).expect("valid")
    }

    #[test]
    fn the_discard_schemes_the_game_exported_read_as_the_editor_showed_them() {
        // The game's discard export of 2026-09-24, rebuilt from its bits (see `edit`).
        let layout = SchemeLayout::discard(
            account(),
            vec![
                raw("二号位双速招财", Some(&[7]), &[1, 18, 35]),
                raw("一号位针女", Some(&[27]), &[0]),
            ],
        )
        .expect("valid");
        let SchemeCode::Discard(schemes) = decode_code(&layout).expect("ok") else {
            panic!("a discard code");
        };
        let first = &schemes[0].selection;
        assert_eq!(schemes[0].name, "二号位双速招财");
        assert_eq!(
            first.sets,
            SetChoice::Sets(BTreeSet::from([SoulSet::from_suit_code(10)]))
        );
        assert_eq!(first.slots, BTreeSet::from([SoulSlot::Slot2]));
        assert_eq!(first.main_attributes, BTreeSet::from([SoulAttribute::Spd]));
        assert_eq!(
            first.sub_attributes.get(SoulAttribute::Spd),
            SubAttributeMode::Include
        );
        let second = &schemes[1].selection;
        assert_eq!(
            second.sets,
            SetChoice::Sets(BTreeSet::from([SoulSet::from_suit_code(36)]))
        );
        assert_eq!(second.slots, BTreeSet::from([SoulSlot::Slot1]));
        let code = SchemeCode::Discard(schemes);
        assert_eq!(encode_code(&code, account()), Ok(layout));
    }

    #[test]
    fn a_set_built_from_selections_reads_back_as_built() {
        let mut spd = SoulSelection::new(SetChoice::AnySet);
        spd.slots = BTreeSet::from([SoulSlot::Slot2]);
        spd.main_attributes = BTreeSet::from([SoulAttribute::Spd]);
        spd.levels = BTreeSet::from([LevelBand::L0to2]);
        let mut boss =
            SoulSelection::new(SetChoice::Sets(BTreeSet::from([SoulSet::from_suit_code(
                50,
            )])));
        boss.innate = BTreeSet::from([InnateAttribute::ALL[5]]);
        boss.sub_attributes
            .set(SoulAttribute::HpFlat, SubAttributeMode::Exclude);
        let code = SchemeCode::Strengthening(StrengtheningSchemeSet {
            plans: vec![
                StrengtheningPlan::new("二号速度", spd),
                StrengtheningPlan::new("土蜘蛛暴击", boss),
            ],
        });
        let layout = encode_code(&code, account()).expect("encodable");
        assert_eq!(layout.header.kind, SchemeKind::Strengthening);
        let payload = serialize(&layout).expect("writable");
        let back = decode_code(&parse(&payload).expect("parsable")).expect("decodable");
        assert_eq!(back, code);
    }

    #[test]
    fn a_whole_code_with_unmapped_bits_is_written_back_byte_for_byte() {
        let mut filter = vec![0u8; 8];
        filter[7] = 0b0110_0000; // bits 61 and 62
        filter[0] = 0b10; // slot 2
        let records = vec![
            Record::new("甲", vec![], filter.clone()).expect("short"),
            Record::new("乙", vec![0, 0, 0, 0, 0, 0, 0, 0, 0x40], vec![1]).expect("short"),
        ];
        let layout = SchemeLayout::strengthening(account(), records);
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
        let again = serialize(&encode_code(&code, account()).expect("encodable"));
        assert_eq!(again, Ok(payload));
    }

    #[test]
    fn codes_the_game_would_refuse_are_not_written() {
        let any = SoulSelection::new(SetChoice::AnySet);
        let discard = SchemeCode::Discard(vec![DiscardScheme::new("全部", any.clone())]);
        assert_eq!(
            encode_code(&discard, account()),
            Err(CodeError::Selection {
                record: 0,
                error: SelectionError::AnySetInDiscard
            })
        );
        assert_eq!(
            encode_code(&SchemeCode::Discard(vec![]), account()),
            Err(CodeError::NoDiscardScheme)
        );
        let long = SchemeCode::Strengthening(StrengtheningSchemeSet {
            plans: vec![StrengtheningPlan::new("一二三四五六七八九十十", any)],
        });
        assert_eq!(
            encode_code(&long, account()),
            Err(CodeError::NameTooLong { record: 0 })
        );
    }

    #[test]
    fn a_name_that_is_not_utf8_has_no_scheme_code() {
        let mut record = Record::new("x", vec![], vec![]).expect("short");
        record.name = vec![0xff];
        let layout = SchemeLayout::strengthening(account(), vec![record]);
        assert_eq!(
            decode_code(&layout),
            Err(CodeError::NameNotUtf8 { record: 0 })
        );
    }
}
