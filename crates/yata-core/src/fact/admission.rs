//! What of a reading the fact log admits (`fact-format.md`, § Ingestion).
//!
//! The reading itself is `import::observation`'s [`SoulReading`]: typed fields, each with the
//! evidence behind its mapping. Two levels of check apply, and they fail differently:
//! - [`admit_reading`] refuses a reading that cannot be an observation of one account's souls:
//!   without an established soul id there is no soul identity, and so no inventory. An import of
//!   it is refused and nothing is committed.
//! - [`check_soul`] finds a record that cannot be a row: a field the row needs is not
//!   established, or is missing, or the values break a premise of W-Soul (`soul-mechanics.md`).
//!   The reading is kept as read, and the record is reported and left out of the inventory, as
//!   `query.md` says of a soul that failed to decode. Nothing is guessed or repaired.

use std::collections::BTreeSet;

use super::model::{Coverage, GameAccountId, GameSoulId};
use crate::import::observation::{self, Evidence, SoulField, SoulObservation, SoulReading};

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
    /// The reading does not say whether it is complete.
    UnstatedCoverage,
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
            AdmissionError::UnstatedCoverage | AdmissionError::MissingSoulId { .. } => {
                "import.malformed_reading"
            }
        }
    }
}

/// Admit a reading, or refuse it whole.
pub fn admit_reading(reading: &SoulReading) -> Result<Admitted, AdmissionError> {
    let evidence = reading.evidence_of(SoulField::SoulId);
    if evidence != Some(Evidence::Established) {
        return Err(AdmissionError::UnestablishedIdentity { evidence });
    }
    let coverage = match reading.coverage {
        observation::Coverage::Complete => Coverage::Complete,
        observation::Coverage::Partial => Coverage::Partial,
        observation::Coverage::Unstated => return Err(AdmissionError::UnstatedCoverage),
    };
    let mut seen = BTreeSet::new();
    let mut souls = Vec::with_capacity(reading.souls.len());
    for (index, soul) in reading.souls.iter().enumerate() {
        let id = soul
            .soul_id
            .clone()
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
            .account
            .clone()
            .and_then(|a| GameAccountId::new(a).ok()),
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
    RepeatedSub { attribute_code: u32 },
    /// Stored values are finite and not negative (`S : SoulAttribute ⇀ ℚ≥0`).
    Value(f64),
    /// `Σ c(a) ≤ nodes(ℓ)`, when the roll counts are recorded.
    Rolls { rolls: u64, nodes: u32 },
}

/// Check that a record of `reading` can be a row: every field of [`ROW_FIELDS`] established and
/// present, then the premises of W-Soul that need no code table.
pub fn check_soul(reading: &SoulReading, soul: &SoulObservation) -> Result<(), SoulDefectKind> {
    for field in ROW_FIELDS {
        if reading.evidence_of(field) != Some(Evidence::Established) {
            return Err(SoulDefectKind::Unestablished(field));
        }
    }
    let present = |field, value: Option<u32>| value.ok_or(SoulDefectKind::Missing(field));
    present(SoulField::SuitCode, soul.suit_code)?;
    present(SoulField::Slot, soul.slot)?;
    let star = present(SoulField::Star, soul.star)?;
    let level = present(SoulField::Level, soul.level)?;
    let main = soul.main.ok_or(SoulDefectKind::Missing(SoulField::Main))?;
    if !(1..=6).contains(&star) {
        return Err(SoulDefectKind::Star(star));
    }
    if level > 15 {
        return Err(SoulDefectKind::Level(level));
    }
    if soul.subs.len() > 4 {
        return Err(SoulDefectKind::TooManySubs(soul.subs.len()));
    }
    let mut codes = BTreeSet::new();
    for sub in &soul.subs {
        if !codes.insert(sub.code) {
            return Err(SoulDefectKind::RepeatedSub {
                attribute_code: sub.code,
            });
        }
    }
    let values = soul
        .subs
        .iter()
        .map(|s| s.value)
        .chain([main.value])
        .chain(soul.innate.map(|i| i.value));
    for v in values {
        if !v.is_finite() || v < 0.0 {
            return Err(SoulDefectKind::Value(v));
        }
    }
    let rolls: u64 = soul
        .subs
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
    use std::collections::BTreeMap;

    use super::*;
    use crate::import::observation::{AttributeReading, SubAttributeReading};

    /// A reading whose every typed field is established.
    pub(crate) fn established(
        coverage: observation::Coverage,
        souls: Vec<SoulObservation>,
    ) -> SoulReading {
        SoulReading {
            coverage,
            recognition: Some(Evidence::Established),
            account: None,
            evidence: SoulField::ALL
                .into_iter()
                .map(|f| (f, Evidence::Established))
                .collect(),
            souls,
        }
    }

    pub(crate) fn soul(id: &str, level: u32) -> SoulObservation {
        SoulObservation {
            soul_id: Some(id.into()),
            suit_code: Some(30),
            star: Some(6),
            slot: Some(2),
            level: Some(level),
            main: Some(AttributeReading {
                code: 7,
                value: 57.0,
            }),
            subs: vec![SubAttributeReading {
                code: 3,
                value: 2.7,
                roll_count: None,
            }],
            innate: None,
            locked: Some(false),
            discarded: Some(false),
            observed: None,
        }
    }

    fn complete(souls: Vec<SoulObservation>) -> SoulReading {
        established(observation::Coverage::Complete, souls)
    }

    #[test]
    fn a_reading_without_an_established_soul_id_is_refused() {
        let mut inherited = complete(vec![soul("a", 15)]);
        inherited
            .evidence
            .insert(SoulField::SoulId, Evidence::Inherited);
        let e = admit_reading(&inherited).expect_err("inherited");
        assert_eq!(
            e,
            AdmissionError::UnestablishedIdentity {
                evidence: Some(Evidence::Inherited)
            }
        );
        assert_eq!(e.code(), "import.unestablished_identity");
        // What the reader sends today: no typed field at all, only observed records.
        let unmapped = SoulReading {
            evidence: BTreeMap::new(),
            ..complete(vec![SoulObservation::default()])
        };
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
    fn a_record_without_an_id_or_a_reading_without_coverage_is_refused() {
        let mut no_id = soul("a", 15);
        no_id.soul_id = None;
        assert_eq!(
            admit_reading(&complete(vec![soul("b", 15), no_id])),
            Err(AdmissionError::MissingSoulId { index: 1 })
        );
        let unstated = established(observation::Coverage::Unstated, vec![]);
        assert_eq!(
            admit_reading(&unstated),
            Err(AdmissionError::UnstatedCoverage)
        );
    }

    #[test]
    fn an_admitted_reading_names_its_souls_and_account() {
        let mut r = complete(vec![soul("a", 15)]);
        r.account = Some("acct".into());
        let a = admit_reading(&r).expect("admitted");
        assert_eq!(a.coverage, Coverage::Complete);
        assert_eq!(a.account.as_ref().map(GameAccountId::as_str), Some("acct"));
        assert_eq!(a.souls, vec![GameSoulId::new("a").expect("id")]);
        // An empty complete reading is an account with no souls.
        assert!(admit_reading(&complete(vec![])).is_ok());
    }

    #[test]
    fn a_row_needs_every_row_field_established_and_present() {
        let r = complete(vec![]);
        assert_eq!(check_soul(&r, &soul("a", 15)), Ok(()));
        for field in ROW_FIELDS {
            let mut inherited = complete(vec![]);
            inherited.evidence.insert(field, Evidence::Inherited);
            assert_eq!(
                check_soul(&inherited, &soul("a", 15)),
                Err(SoulDefectKind::Unestablished(field))
            );
        }
        let mut no_star = soul("a", 15);
        no_star.star = None;
        assert_eq!(
            check_soul(&r, &no_star),
            Err(SoulDefectKind::Missing(SoulField::Star))
        );
        // Fields outside the row are carried as read, established or not.
        let mut unmapped_lock = complete(vec![]);
        unmapped_lock.evidence.remove(&SoulField::Locked);
        let mut s = soul("a", 15);
        s.locked = None;
        assert_eq!(check_soul(&unmapped_lock, &s), Ok(()));
    }

    #[test]
    fn each_premise_of_w_soul_that_needs_no_code_table_is_checked() {
        fn with(f: impl FnOnce(&mut SoulObservation)) -> Result<(), SoulDefectKind> {
            let mut s = soul("a", 15);
            f(&mut s);
            check_soul(&complete(vec![]), &s)
        }
        assert_eq!(with(|s| s.star = Some(0)), Err(SoulDefectKind::Star(0)));
        assert_eq!(with(|s| s.star = Some(7)), Err(SoulDefectKind::Star(7)));
        assert_eq!(with(|s| s.level = Some(16)), Err(SoulDefectKind::Level(16)));
        let sub = |code| SubAttributeReading {
            code,
            value: 1.0,
            roll_count: Some(1),
        };
        assert_eq!(
            with(|s| s.subs = vec![sub(1), sub(2), sub(3), sub(4), sub(5)]),
            Err(SoulDefectKind::TooManySubs(5))
        );
        assert_eq!(
            with(|s| s.subs = vec![sub(1), sub(1)]),
            Err(SoulDefectKind::RepeatedSub { attribute_code: 1 })
        );
        assert!(matches!(
            with(|s| s.main = Some(AttributeReading {
                code: 7,
                value: f64::NAN
            })),
            Err(SoulDefectKind::Value(_))
        ));
        assert_eq!(
            with(|s| s.subs[0].value = -0.5),
            Err(SoulDefectKind::Value(-0.5))
        );
    }

    #[test]
    fn recorded_rolls_cannot_exceed_the_nodes_reached() {
        let r = complete(vec![]);
        let mut s = soul("a", 5);
        s.subs[0].roll_count = Some(2);
        assert_eq!(
            check_soul(&r, &s),
            Err(SoulDefectKind::Rolls { rolls: 2, nodes: 1 })
        );
        s.level = Some(6);
        assert_eq!(check_soul(&r, &s), Ok(()));
    }
}
