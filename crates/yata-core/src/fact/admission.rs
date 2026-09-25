//! What of a reading the fact log admits (`fact-format.md`, § Ingestion).
//!
//! The reading itself is `import::observation`'s [`SoulReading`]: typed fields, each saying
//! whether the reader maps it and with what evidence. Two levels of check apply, and they fail
//! differently:
//! - [`admit_reading`] refuses a reading that cannot be an observation of one account's souls:
//!   without an established soul id there is no soul identity, and so no inventory. An import of
//!   it is refused and nothing is committed.
//! - [`check_soul`] finds a record that cannot be a row: a field the row needs is not
//!   established, or is missing, or the values break a premise of W-Soul (`soul-mechanics.md`).
//!   The reading is kept as read, and the record is reported and left out of the inventory, as
//!   `query.md` says of a soul that failed to decode. Nothing is guessed or repaired.

use std::collections::BTreeSet;

use super::model::{Coverage, GameAccountId, GameSoulId};
use crate::import::observation::{
    self, Evidence, Field, GameAttributeCode, InnateReading, SoulField, SoulObservation,
    SoulReading,
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

/// A reading the fact log admits: its coverage, account, and the identity of each record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admitted {
    pub coverage: Coverage,
    pub account: Option<GameAccountId>,
    /// One per record, in the reading's order.
    pub souls: Vec<GameSoulId>,
}

/// Why a reading is refused. Nothing of it is committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionError {
    /// `import.unestablished_identity`: the soul id's mapping is not established, so no record
    /// has an identity the inventory can key on.
    UnestablishedIdentity { evidence: Option<Evidence> },
    /// A record without a soul id, by position, in a reading that maps it.
    MissingSoulId { index: usize },
    /// `import.duplicate_soul`: two records claim one soul. One soul has one state at one time,
    /// and neither record is preferred.
    DuplicateSoul { soul: GameSoulId },
}

impl AdmissionError {
    /// The stable error code (`core-protocol.md`, § Errors).
    pub fn code(&self) -> &'static str {
        match self {
            AdmissionError::UnestablishedIdentity { .. } => "import.unestablished_identity",
            AdmissionError::DuplicateSoul { .. } => "import.duplicate_soul",
            AdmissionError::MissingSoulId { .. } => "import.malformed_reading",
        }
    }
}

/// Admit a reading, or refuse it whole.
pub fn admit_reading(reading: &SoulReading) -> Result<Admitted, AdmissionError> {
    let evidence = reading
        .mappings()
        .get(SoulField::SoulId)
        .map(observation::Mapping::evidence);
    if evidence != Some(Evidence::Established) {
        return Err(AdmissionError::UnestablishedIdentity { evidence });
    }
    let coverage = match reading.coverage() {
        observation::Coverage::Complete => Coverage::Complete,
        observation::Coverage::Partial => Coverage::Partial,
    };
    let mut seen = BTreeSet::new();
    let mut souls = Vec::with_capacity(reading.souls().len());
    for (index, soul) in reading.souls().iter().enumerate() {
        let id = soul
            .soul_id
            .value()
            .cloned()
            .and_then(|id| GameSoulId::new(id).ok())
            .ok_or(AdmissionError::MissingSoulId { index })?;
        if !seen.insert(id.clone()) {
            return Err(AdmissionError::DuplicateSoul { soul: id });
        }
        souls.push(id);
    }
    Ok(Admitted {
        coverage,
        account: reading
            .account()
            .and_then(|a| GameAccountId::new(a.to_owned()).ok()),
        souls,
    })
}

/// Why a record is not an inventory row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoulDefectKind {
    /// The reading does not establish a field the row needs.
    Unestablished(SoulField),
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
fn row<T>(field: SoulField, value: &Field<T>) -> Result<&T, SoulDefectKind> {
    match value {
        Field::Mapped {
            evidence: Evidence::Established,
            value: Some(v),
        } => Ok(v),
        Field::Mapped {
            evidence: Evidence::Established,
            value: None,
        } => Err(SoulDefectKind::Missing(field)),
        _ => Err(SoulDefectKind::Unestablished(field)),
    }
}

/// Check that a record can be a row: every field of [`ROW_FIELDS`] established and present, then
/// the premises of W-Soul that need no code table.
pub fn check_soul(soul: &SoulObservation) -> Result<(), SoulDefectKind> {
    for field in ROW_FIELDS {
        if soul.evidence_of(field) != Some(Evidence::Established) {
            return Err(SoulDefectKind::Unestablished(field));
        }
    }
    row(SoulField::SuitCode, &soul.suit_code)?;
    row(SoulField::Slot, &soul.slot)?;
    let star = row(SoulField::Star, &soul.star)?.0;
    let level = row(SoulField::Level, &soul.level)?.0;
    let main = row(SoulField::Main, &soul.main)?;
    let subs = row(SoulField::Subs, &soul.subs)?;
    if !(1..=6).contains(&star) {
        return Err(SoulDefectKind::Star(star));
    }
    if level > 15 {
        return Err(SoulDefectKind::Level(level));
    }
    if subs.len() > 4 {
        return Err(SoulDefectKind::TooManySubs(subs.len()));
    }
    let mut codes = BTreeSet::new();
    for sub in subs {
        if !codes.insert(sub.code) {
            return Err(SoulDefectKind::RepeatedSub {
                attribute_code: sub.code,
            });
        }
    }
    // A lacking innate field is `Missing`: only a stated `None` is an ordinary soul (ADR-0029).
    let innate = match row(SoulField::Innate, &soul.innate)? {
        InnateReading::Present(a) => Some(a.value),
        InnateReading::None => None,
    };
    let values = subs
        .iter()
        .map(|s| s.value)
        .chain([main.value])
        .chain(innate);
    for v in values {
        if !v.is_finite() || v < 0.0 {
            return Err(SoulDefectKind::Value(v));
        }
    }
    let rolls: u64 = subs
        .iter()
        .filter_map(|s| s.roll_count)
        .map(u64::from)
        .sum();
    let nodes = level / 3;
    if rolls > u64::from(nodes) {
        return Err(SoulDefectKind::Rolls { rolls, nodes });
    }
    Ok(())
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

    /// The check of one record under all-established mappings.
    fn check(raw: RawSoul) -> Result<(), SoulDefectKind> {
        check_soul(&complete(vec![raw]).souls()[0])
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
                evidence: Some(Evidence::Inherited)
            }
        );
        assert_eq!(e.code(), "import.unestablished_identity");
        // What the reader sends today: no typed field at all, only observed records.
        let unmapped = with_mappings(SoulMappings::default(), vec![RawSoul::default()]);
        assert_eq!(
            admit_reading(&unmapped),
            Err(AdmissionError::UnestablishedIdentity { evidence: None })
        );
    }

    #[test]
    fn a_reading_with_one_soul_twice_is_refused() {
        let e = admit_reading(&complete(vec![soul("a", 15), soul("b", 15), soul("a", 12)]))
            .expect_err("duplicate");
        assert_eq!(e.code(), "import.duplicate_soul");
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
        assert_eq!(a.souls, vec![GameSoulId::new("a").expect("id")]);
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
                check_soul(&r.souls()[0]),
                Err(SoulDefectKind::Unestablished(field))
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
        assert_eq!(check_soul(&unmapped_lock.souls()[0]), Ok(()));
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
}
