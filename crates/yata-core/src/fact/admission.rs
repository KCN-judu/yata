//! What of a reading the fact log admits (`fact-format.md`, § Ingestion).
//!
//! The reading itself is `import::observation`'s [`SoulReading`]: typed fields, each saying
//! whether the reader maps it and with what evidence. Admission parses it, once, and fails at
//! two levels:
//! - the whole reading is refused ([`AdmissionError`]) when it cannot be an observation of one
//!   account's souls: without an established soul id there is no soul identity, and so no
//!   inventory. Nothing is committed.
//! - a record is a [`RecordDefect`] when it cannot be a row: a field the row needs is not
//!   established, or is missing, or the values break a premise of W-Soul (`soul-mechanics.md`).
//!   The reading is kept as read, and the record is reported and left out of the inventory, as
//!   `query.md` says of a soul that failed to decode. Nothing is guessed or repaired.
//!
//! What passes is an [`AdmittedSoul`]: the row's fields, each present and established, so no
//! later step checks them again.

use std::collections::BTreeSet;

use super::model::{Coverage, GameAccountId, GameSoulId};
use crate::import::observation::{
    self, AttributeReading, Evidence, Field, GameAttributeCode, GameLevel, GameSlot, GameStar,
    GameSuitCode, InnateReading, SoulField, SoulObservation, SoulReading, SubAttributeReading,
};

/// The fields a record must carry, established, to be an inventory row: every field W-Soul reads,
/// the set, the slot, and the innate attribute, which decides whether the soul is a boss soul
/// (ADR-0029). `locked` and `discarded` are carried as read.
pub const ROW_FIELDS: [SoulField; 7] = [
    SoulField::SuitCode,
    SoulField::Star,
    SoulField::Slot,
    SoulField::Level,
    SoulField::Main,
    SoulField::Subs,
    SoulField::Innate,
];

/// Evidence that falls short of established.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotEstablished {
    /// The reader does not map the field.
    Unmapped,
    /// The reader maps it on a hypothesis inherited from the prior tool (ADR-0014).
    Inherited,
}

/// A record the fact log admits as a row: every field the row needs, established and present.
/// `locked` and `discarded` are carried as read, with their evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct AdmittedSoul {
    pub id: GameSoulId,
    pub suit: GameSuitCode,
    pub star: GameStar,
    pub slot: GameSlot,
    pub level: GameLevel,
    pub main: AttributeReading,
    pub subs: Vec<SubAttributeReading>,
    /// Whether the soul is a boss soul, and with what (ADR-0029).
    pub innate: InnateReading,
    pub locked: Field<bool>,
    pub discarded: Field<bool>,
}

/// A record that is not a row, by its position in the reading.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordDefect {
    pub index: usize,
    pub soul: GameSoulId,
    pub kind: SoulDefectKind,
}

/// A reading the fact log admits: its coverage and account, its rows, and the records that are
/// not rows.
#[derive(Debug, Clone, PartialEq)]
pub struct Admitted {
    pub coverage: Coverage,
    pub account: Option<GameAccountId>,
    pub rows: Vec<AdmittedSoul>,
    pub defects: Vec<RecordDefect>,
}

/// Why a reading is refused. Nothing of it is committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionError {
    /// `import.unestablished_identity`: the soul id's mapping is not established, so no record
    /// has an identity the inventory can key on.
    UnestablishedIdentity { evidence: NotEstablished },
    /// A record without a soul id, by position, in a reading that maps it.
    MissingSoulId { index: usize },
    /// A record whose soul id is the empty string: the game names nothing so.
    EmptySoulId { index: usize },
    /// The reading names its account with the empty string.
    EmptyAccount,
    /// `import.duplicate_soul`: two records claim one soul. One soul has one state at one time,
    /// and neither record is preferred.
    DuplicateSoul { soul: GameSoulId },
}

/// Admit a reading: refuse it whole, or split it into rows and defects.
pub fn admit_reading(reading: &SoulReading) -> Result<Admitted, AdmissionError> {
    match reading
        .mappings()
        .get(SoulField::SoulId)
        .map(observation::Mapping::evidence)
    {
        Some(Evidence::Established) => {}
        Some(Evidence::Inherited) => {
            return Err(AdmissionError::UnestablishedIdentity {
                evidence: NotEstablished::Inherited,
            });
        }
        None => {
            return Err(AdmissionError::UnestablishedIdentity {
                evidence: NotEstablished::Unmapped,
            });
        }
    }
    let coverage = match reading.coverage() {
        observation::Coverage::Complete => Coverage::Complete,
        observation::Coverage::Partial => Coverage::Partial,
    };
    let account = reading
        .account()
        .map(|a| GameAccountId::new(a).map_err(|_| AdmissionError::EmptyAccount))
        .transpose()?;
    let identified: Vec<(usize, GameSoulId, &SoulObservation)> = reading
        .souls()
        .iter()
        .enumerate()
        .map(|(index, soul)| match soul.soul_id.value() {
            None => Err(AdmissionError::MissingSoulId { index }),
            Some(id) => GameSoulId::new(id.as_str())
                .map(|id| (index, id, soul))
                .map_err(|_| AdmissionError::EmptySoulId { index }),
        })
        .collect::<Result<_, _>>()?;
    let mut seen = BTreeSet::new();
    if let Some((_, id, _)) = identified.iter().find(|(_, id, _)| !seen.insert(id)) {
        return Err(AdmissionError::DuplicateSoul { soul: id.clone() });
    }
    let (rows, defects) = identified.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut rows, mut defects), (index, id, soul)| {
            match check_soul(id.clone(), soul) {
                Ok(row) => rows.push(row),
                Err(kind) => defects.push(RecordDefect {
                    index,
                    soul: id,
                    kind,
                }),
            }
            (rows, defects)
        },
    );
    Ok(Admitted {
        coverage,
        account,
        rows,
        defects,
    })
}

/// Why a record is not an inventory row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoulDefectKind {
    /// The reading does not establish a field the row needs.
    Unestablished {
        field: SoulField,
        evidence: NotEstablished,
    },
    /// The field is established and this record lacks it.
    Missing(SoulField),
    /// `1 ≤ σ ≤ 6`.
    Star(u32),
    /// `0 ≤ ℓ ≤ 15`.
    Level(u32),
    /// `|dom S| ≤ 4`.
    TooManySubs(usize),
    /// `S` is a partial map: an attribute is in its domain at most once.
    RepeatedSub { attribute_code: GameAttributeCode },
    /// Stored values are finite and not negative (`S : SoulAttribute ⇀ ℚ≥0`).
    Value(f64),
    /// `Σ c(a) ≤ nodes(ℓ)`, when the roll counts are recorded.
    Rolls { rolls: u64, nodes: u32 },
}

/// A row field's value: established and present, or the defect that says why not.
fn row<T: Clone>(field: SoulField, value: &Field<T>) -> Result<T, SoulDefectKind> {
    let unestablished = |evidence| SoulDefectKind::Unestablished { field, evidence };
    match value {
        Field::Mapped {
            evidence: Evidence::Established,
            value: Some(v),
        } => Ok(v.clone()),
        Field::Mapped {
            evidence: Evidence::Established,
            value: None,
        } => Err(SoulDefectKind::Missing(field)),
        Field::Mapped {
            evidence: Evidence::Inherited,
            ..
        } => Err(unestablished(NotEstablished::Inherited)),
        Field::Unmapped => Err(unestablished(NotEstablished::Unmapped)),
    }
}

/// Parse one identified record into a row: every row field established and present, in the
/// order the defects are reported, then the premises of W-Soul that need no code table.
pub fn check_soul(id: GameSoulId, soul: &SoulObservation) -> Result<AdmittedSoul, SoulDefectKind> {
    let suit = row(SoulField::SuitCode, &soul.suit_code)?;
    let star = row(SoulField::Star, &soul.star)?;
    let slot = row(SoulField::Slot, &soul.slot)?;
    let level = row(SoulField::Level, &soul.level)?;
    let main = row(SoulField::Main, &soul.main)?;
    let subs = row(SoulField::Subs, &soul.subs)?;
    // A lacking innate field is `Missing`: only a stated `None` is an ordinary soul (ADR-0029).
    let innate = row(SoulField::Innate, &soul.innate)?;
    if !(1..=6).contains(&star.0) {
        return Err(SoulDefectKind::Star(star.0));
    }
    if level.0 > 15 {
        return Err(SoulDefectKind::Level(level.0));
    }
    if subs.len() > 4 {
        return Err(SoulDefectKind::TooManySubs(subs.len()));
    }
    let mut codes = BTreeSet::new();
    if let Some(repeated) = subs.iter().find(|s| !codes.insert(s.code)) {
        return Err(SoulDefectKind::RepeatedSub {
            attribute_code: repeated.code,
        });
    }
    let innate_value = match &innate {
        InnateReading::Present(a) => Some(a.value),
        InnateReading::None => None,
    };
    let values = subs
        .iter()
        .map(|s| s.value)
        .chain([main.value])
        .chain(innate_value);
    if let Some(v) = values.into_iter().find(|v| !v.is_finite() || *v < 0.0) {
        return Err(SoulDefectKind::Value(v));
    }
    let rolls: u64 = subs
        .iter()
        .filter_map(|s| s.roll_count)
        .map(u64::from)
        .sum();
    let nodes = level.0 / 3;
    if rolls > u64::from(nodes) {
        return Err(SoulDefectKind::Rolls { rolls, nodes });
    }
    Ok(AdmittedSoul {
        id,
        suit,
        star,
        slot,
        level,
        main,
        subs,
        innate,
        locked: soul.locked.clone(),
        discarded: soul.discarded.clone(),
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::import::observation::{
        AttributeReading, GameLevel, GameSlot, GameStar, GameSuitCode, Mapping, RawSoul,
        SoulMappings, SubAttributeReading,
    };

    fn established_mapping() -> Option<Mapping> {
        Some(Mapping::Established {
            basis: "test".into(),
        })
    }

    /// Every typed field mapped, established.
    pub(crate) fn all_established() -> SoulMappings {
        let e = established_mapping;
        SoulMappings {
            soul_id: e(),
            suit_code: e(),
            star: e(),
            slot: e(),
            level: e(),
            main: e(),
            subs: e(),
            innate: e(),
            locked: e(),
            discarded: e(),
        }
    }

    /// A reading whose every typed field is established.
    pub(crate) fn established(coverage: observation::Coverage, souls: Vec<RawSoul>) -> SoulReading {
        SoulReading::new(
            coverage,
            None,
            established_mapping(),
            all_established(),
            souls,
        )
    }

    pub(crate) fn soul(id: &str, level: u32) -> RawSoul {
        RawSoul {
            soul_id: Some(id.into()),
            suit_code: Some(GameSuitCode(30)),
            star: Some(GameStar(6)),
            slot: Some(GameSlot(2)),
            level: Some(GameLevel(level)),
            main: Some(AttributeReading {
                code: GameAttributeCode(7),
                value: 57.0,
            }),
            subs: Some(vec![SubAttributeReading {
                code: GameAttributeCode(3),
                value: 2.7,
                roll_count: None,
            }]),
            innate: Some(InnateReading::None),
            locked: Some(false),
            discarded: Some(false),
            observed: None,
        }
    }

    fn complete(souls: Vec<RawSoul>) -> SoulReading {
        established(observation::Coverage::Complete, souls)
    }

    fn with_mappings(mappings: SoulMappings, souls: Vec<RawSoul>) -> SoulReading {
        SoulReading::new(observation::Coverage::Complete, None, None, mappings, souls)
    }

    fn id(s: &str) -> GameSoulId {
        GameSoulId::new(s).expect("an id")
    }

    /// The check of one record under all-established mappings.
    fn check(raw: RawSoul) -> Result<(), SoulDefectKind> {
        check_soul(id("a"), &complete(vec![raw]).souls()[0]).map(drop)
    }

    #[test]
    fn a_reading_without_an_established_soul_id_is_refused() {
        let inherited = with_mappings(
            SoulMappings {
                soul_id: Some(Mapping::Inherited),
                ..all_established()
            },
            vec![soul("a", 15)],
        );
        let e = admit_reading(&inherited).expect_err("inherited");
        assert_eq!(
            e,
            AdmissionError::UnestablishedIdentity {
                evidence: NotEstablished::Inherited
            }
        );
        // What the reader sends today: no typed field at all, only observed records.
        let unmapped = with_mappings(SoulMappings::default(), vec![RawSoul::default()]);
        assert_eq!(
            admit_reading(&unmapped),
            Err(AdmissionError::UnestablishedIdentity {
                evidence: NotEstablished::Unmapped
            })
        );
    }

    #[test]
    fn a_reading_with_one_soul_twice_is_refused() {
        let e = admit_reading(&complete(vec![soul("a", 15), soul("b", 15), soul("a", 12)]))
            .expect_err("duplicate");
        assert!(matches!(e, AdmissionError::DuplicateSoul { .. }), "{e:?}");
    }

    #[test]
    fn a_record_without_an_id_is_refused() {
        let mut no_id = soul("a", 15);
        no_id.soul_id = None;
        assert_eq!(
            admit_reading(&complete(vec![soul("b", 15), no_id])),
            Err(AdmissionError::MissingSoulId { index: 1 })
        );
    }

    #[test]
    fn an_admitted_reading_names_its_souls_and_account() {
        let r = SoulReading::new(
            observation::Coverage::Complete,
            Some("acct".into()),
            None,
            all_established(),
            vec![soul("a", 15)],
        );
        let a = admit_reading(&r).expect("admitted");
        assert_eq!(a.coverage, Coverage::Complete);
        assert_eq!(a.account.as_ref().map(GameAccountId::as_str), Some("acct"));
        let ids: Vec<&GameSoulId> = a.rows.iter().map(|r| &r.id).collect();
        assert_eq!(ids, vec![&id("a")]);
        assert!(a.defects.is_empty());
        // An empty complete reading is an account with no souls.
        assert!(admit_reading(&complete(vec![])).is_ok());
    }

    #[test]
    fn a_row_needs_every_row_field_established_and_present() {
        assert_eq!(check(soul("a", 15)), Ok(()));
        for field in ROW_FIELDS {
            let mut mappings = all_established();
            match field {
                SoulField::SuitCode => mappings.suit_code = Some(Mapping::Inherited),
                SoulField::Star => mappings.star = Some(Mapping::Inherited),
                SoulField::Slot => mappings.slot = Some(Mapping::Inherited),
                SoulField::Level => mappings.level = Some(Mapping::Inherited),
                SoulField::Main => mappings.main = Some(Mapping::Inherited),
                SoulField::Subs => mappings.subs = Some(Mapping::Inherited),
                SoulField::Innate => mappings.innate = Some(Mapping::Inherited),
                _ => {}
            }
            let r = with_mappings(mappings, vec![soul("a", 15)]);
            assert_eq!(
                check_soul(id("a"), &r.souls()[0]).map(drop),
                Err(SoulDefectKind::Unestablished {
                    field,
                    evidence: NotEstablished::Inherited
                })
            );
        }
        let mut no_star = soul("a", 15);
        no_star.star = None;
        assert_eq!(
            check(no_star),
            Err(SoulDefectKind::Missing(SoulField::Star))
        );
        // A mapped sub-attribute list the record lacks is missing, not a soul with none.
        let mut no_subs = soul("a", 15);
        no_subs.subs = None;
        assert_eq!(
            check(no_subs),
            Err(SoulDefectKind::Missing(SoulField::Subs))
        );
        let mut none = soul("a", 15);
        none.subs = Some(vec![]);
        assert_eq!(check(none), Ok(()));
        // A mapped innate attribute the record lacks is missing, not an ordinary soul
        // (ADR-0029); a stated one makes a boss soul.
        let mut no_innate = soul("a", 15);
        no_innate.innate = None;
        assert_eq!(
            check(no_innate),
            Err(SoulDefectKind::Missing(SoulField::Innate))
        );
        let mut boss = soul("a", 15);
        boss.innate = Some(InnateReading::Present(AttributeReading {
            code: GameAttributeCode(4),
            value: 15.0,
        }));
        assert_eq!(check(boss), Ok(()));
        // Fields outside the row are carried as read, mapped or not.
        let unmapped_lock = with_mappings(
            SoulMappings {
                locked: None,
                ..all_established()
            },
            vec![soul("a", 15)],
        );
        let row = check_soul(id("a"), &unmapped_lock.souls()[0]).expect("a row");
        assert_eq!(row.locked, Field::Unmapped);
    }

    #[test]
    fn each_premise_of_w_soul_that_needs_no_code_table_is_checked() {
        fn with(f: impl FnOnce(&mut RawSoul)) -> Result<(), SoulDefectKind> {
            let mut s = soul("a", 15);
            f(&mut s);
            check(s)
        }
        assert_eq!(
            with(|s| s.star = Some(GameStar(0))),
            Err(SoulDefectKind::Star(0))
        );
        assert_eq!(
            with(|s| s.star = Some(GameStar(7))),
            Err(SoulDefectKind::Star(7))
        );
        assert_eq!(
            with(|s| s.level = Some(GameLevel(16))),
            Err(SoulDefectKind::Level(16))
        );
        let sub = |code| SubAttributeReading {
            code: GameAttributeCode(code),
            value: 1.0,
            roll_count: Some(1),
        };
        assert_eq!(
            with(|s| s.subs = Some(vec![sub(1), sub(2), sub(3), sub(4), sub(5)])),
            Err(SoulDefectKind::TooManySubs(5))
        );
        assert_eq!(
            with(|s| s.subs = Some(vec![sub(1), sub(1)])),
            Err(SoulDefectKind::RepeatedSub {
                attribute_code: GameAttributeCode(1)
            })
        );
        assert!(matches!(
            with(|s| s.main = Some(AttributeReading {
                code: GameAttributeCode(7),
                value: f64::NAN
            })),
            Err(SoulDefectKind::Value(_))
        ));
        assert_eq!(
            with(|s| s.subs = Some(vec![SubAttributeReading {
                value: -0.5,
                ..sub(3)
            }])),
            Err(SoulDefectKind::Value(-0.5))
        );
    }

    #[test]
    fn recorded_rolls_cannot_exceed_the_nodes_reached() {
        let rolled = |level: u32| {
            let mut s = soul("a", level);
            s.subs = Some(vec![SubAttributeReading {
                code: GameAttributeCode(3),
                value: 2.7,
                roll_count: Some(2),
            }]);
            s
        };
        assert_eq!(
            check(rolled(5)),
            Err(SoulDefectKind::Rolls { rolls: 2, nodes: 1 })
        );
        assert_eq!(check(rolled(6)), Ok(()));
    }

    #[test]
    fn an_unmapped_row_field_is_named_as_unmapped() {
        let r = with_mappings(
            SoulMappings {
                level: None,
                ..all_established()
            },
            vec![soul("a", 15)],
        );
        assert_eq!(
            check_soul(id("a"), &r.souls()[0]).map(drop),
            Err(SoulDefectKind::Unestablished {
                field: SoulField::Level,
                evidence: NotEstablished::Unmapped
            })
        );
    }

    #[test]
    fn admission_splits_rows_from_defects_by_record() {
        let mut bad = soul("b", 15);
        bad.star = Some(GameStar(9));
        let a = admit_reading(&complete(vec![soul("a", 15), bad, soul("c", 3)])).expect("admitted");
        let ids: Vec<&str> = a.rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, vec!["a", "c"]);
        assert_eq!(
            a.defects,
            vec![RecordDefect {
                index: 1,
                soul: id("b"),
                kind: SoulDefectKind::Star(9)
            }]
        );
        let c = &a.rows[1];
        assert_eq!(
            (c.level, c.star, c.innate),
            (GameLevel(3), GameStar(6), InnateReading::None)
        );
    }

    #[test]
    fn an_empty_soul_id_is_refused_by_name() {
        let mut empty = soul("a", 15);
        empty.soul_id = Some(String::new());
        assert_eq!(
            admit_reading(&complete(vec![empty])),
            Err(AdmissionError::EmptySoulId { index: 0 })
        );
    }
}
