//! A profile's current soul inventory: the live readings overlaid, with the user's decisions
//! attached (`fact-format.md`, § Acquisition). This is the value queries iterate over.
//!
//! A soul absent from the live complete reading is absent from the account. A partial reading
//! replaces the records of the souls it contains and says nothing about the others. A decision
//! about a soul that is not in the inventory is kept in the projection and applies again if the
//! soul reappears.

use std::collections::BTreeMap;

use super::admission::{AdmissionError, SoulDefectKind, admit_reading, check_soul};
use super::model::{Coverage, Digest, GameSoulId, Mark, ProfileId, Scope};
use super::projection::Projection;
use crate::import::observation::{SoulObservation, SoulReading};

/// One profile's souls as of one revision.
#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    revision: u64,
    souls: BTreeMap<GameSoulId, InventorySoul>,
    defects: Vec<SoulDefect>,
}

/// One soul of the inventory.
#[derive(Debug, Clone, PartialEq)]
pub struct InventorySoul {
    pub id: GameSoulId,
    /// The record as read, every typed field row-checked (`admission::ROW_FIELDS`).
    pub record: SoulObservation,
    /// The `seq` of the acquisition this record was read in.
    pub observed_at: u64,
    pub mark: Option<Mark>,
    pub note: Option<String>,
}

/// A live record that cannot be a soul the game holds. It is reported and is not a row.
#[derive(Debug, Clone, PartialEq)]
pub struct SoulDefect {
    pub soul: GameSoulId,
    pub observed_at: u64,
    pub kind: SoulDefectKind,
}

/// Why an inventory could not be derived. Each is a store inconsistency, not a user error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InventoryError {
    UnknownProfile {
        profile: ProfileId,
    },
    /// A live snapshot's reading was not supplied.
    MissingReading {
        digest: Digest,
    },
    /// A reading's own coverage is not what its acquisition recorded.
    ReadingDisagrees {
        seq: u64,
        digest: Digest,
    },
    /// A live reading the fact log would not admit now; it was admitted when it landed.
    ReadingRefused {
        seq: u64,
        digest: Digest,
        error: AdmissionError,
    },
}

impl Inventory {
    /// Derive a profile's soul inventory. `readings` must hold the reading of every digest of
    /// [`crate::fact::ProfileState::live`] for [`Scope::Souls`]; others are ignored.
    pub fn derive(
        projection: &Projection,
        profile: ProfileId,
        readings: &BTreeMap<Digest, SoulReading>,
    ) -> Result<Inventory, InventoryError> {
        let state = projection
            .profile(profile)
            .ok_or(InventoryError::UnknownProfile { profile })?;
        let live = state.live(Scope::Souls);
        let layers = live
            .complete
            .iter()
            .map(|l| (l, Coverage::Complete))
            .chain(live.partials.iter().map(|l| (l, Coverage::Partial)));
        let mut records: BTreeMap<GameSoulId, (u64, &SoulObservation)> = BTreeMap::new();
        for (&(seq, digest), coverage) in layers {
            let reading = readings
                .get(&digest)
                .ok_or(InventoryError::MissingReading { digest })?;
            let admitted = admit_reading(reading)
                .map_err(|error| InventoryError::ReadingRefused { seq, digest, error })?;
            if admitted.coverage != coverage {
                return Err(InventoryError::ReadingDisagrees { seq, digest });
            }
            for (id, soul) in admitted.souls.into_iter().zip(reading.souls()) {
                records.insert(id, (seq, soul));
            }
        }
        let mut souls = BTreeMap::new();
        let mut defects = Vec::new();
        for (id, (observed_at, record)) in records {
            match check_soul(record) {
                Ok(()) => {
                    let row = InventorySoul {
                        id: id.clone(),
                        record: record.clone(),
                        observed_at,
                        mark: state.mark(&id),
                        note: state.note(&id).map(str::to_owned),
                    };
                    souls.insert(id, row);
                }
                Err(kind) => defects.push(SoulDefect {
                    soul: id,
                    observed_at,
                    kind,
                }),
            }
        }
        Ok(Inventory {
            revision: projection.revision(),
            souls,
            defects,
        })
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Every soul, ordered by game soul id.
    pub fn souls(&self) -> impl Iterator<Item = &InventorySoul> {
        self.souls.values()
    }

    pub fn get(&self, soul: &GameSoulId) -> Option<&InventorySoul> {
        self.souls.get(soul)
    }

    pub fn len(&self) -> usize {
        self.souls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.souls.is_empty()
    }

    /// The live records left out because they cannot be souls, ordered by game soul id.
    pub fn defects(&self) -> &[SoulDefect] {
        &self.defects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fact::admission::tests::{established, soul};
    use crate::fact::model::Commit;
    use crate::fact::projection::fold;
    use crate::fact::projection::tests::{P, Q, acquired, commit, created, marked};
    use crate::import::observation::{self, GameStar, RawSoul, SoulMappings};

    fn id(s: &str) -> GameSoulId {
        GameSoulId::new(s).expect("non-empty")
    }

    fn reading(coverage: Coverage, souls: Vec<RawSoul>) -> SoulReading {
        let coverage = match coverage {
            Coverage::Complete => observation::Coverage::Complete,
            Coverage::Partial => observation::Coverage::Partial,
        };
        established(coverage, souls)
    }

    fn derive(log: &[Commit], readings: &[(u8, SoulReading)], profile: ProfileId) -> Inventory {
        let p = fold(log).expect("folds");
        let readings = readings
            .iter()
            .map(|(d, r)| ([*d; 32], r.clone()))
            .collect();
        Inventory::derive(&p, profile, &readings).expect("derives")
    }

    fn ids(inv: &Inventory) -> Vec<&str> {
        inv.souls().map(|s| s.id.as_str()).collect()
    }

    #[test]
    fn a_newer_complete_reading_replaces_the_inventory() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
            commit(3, vec![acquired(P, 2, Coverage::Complete, None)]),
        ];
        let readings = [
            (
                1,
                reading(Coverage::Complete, vec![soul("a", 12), soul("b", 3)]),
            ),
            (
                2,
                reading(Coverage::Complete, vec![soul("a", 15), soul("c", 0)]),
            ),
        ];
        let inv = derive(&log, &readings, P);
        assert_eq!(ids(&inv), vec!["a", "c"]);
        let a = inv.get(&id("a")).expect("a");
        assert_eq!(
            (a.record.level.value().map(|l| l.0), a.observed_at),
            (Some(15), 3)
        );
    }

    #[test]
    fn a_partial_reading_overlays_and_removes_nothing() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
            commit(3, vec![acquired(P, 2, Coverage::Partial, None)]),
        ];
        let readings = [
            (
                1,
                reading(Coverage::Complete, vec![soul("a", 12), soul("b", 3)]),
            ),
            (
                2,
                reading(Coverage::Partial, vec![soul("b", 6), soul("d", 0)]),
            ),
        ];
        let inv = derive(&log, &readings, P);
        assert_eq!(ids(&inv), vec!["a", "b", "d"]);
        assert_eq!(inv.get(&id("a")).map(|s| s.observed_at), Some(2));
        assert_eq!(
            inv.get(&id("b"))
                .and_then(|s| s.record.level.value().map(|l| l.0)),
            Some(6)
        );
    }

    #[test]
    fn a_decision_outlives_its_soul_and_applies_when_it_returns() {
        let mut log = vec![
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
            commit(3, vec![marked(P, "a", Some(Mark::Keep))]),
            commit(4, vec![acquired(P, 2, Coverage::Complete, None)]),
        ];
        let readings = [
            (1, reading(Coverage::Complete, vec![soul("a", 15)])),
            (2, reading(Coverage::Complete, vec![])),
            (3, reading(Coverage::Complete, vec![soul("a", 15)])),
        ];
        assert!(derive(&log, &readings, P).is_empty());
        log.push(commit(5, vec![acquired(P, 3, Coverage::Complete, None)]));
        let back = derive(&log, &readings, P);
        assert_eq!(back.get(&id("a")).and_then(|s| s.mark), Some(Mark::Keep));
    }

    #[test]
    fn decisions_belong_to_one_profile() {
        let log = [
            commit(1, vec![created(P, None), created(Q, None)]),
            commit(
                2,
                vec![
                    acquired(P, 1, Coverage::Complete, None),
                    acquired(Q, 1, Coverage::Complete, None),
                ],
            ),
            commit(3, vec![marked(P, "a", Some(Mark::Discard))]),
        ];
        let readings = [(1, reading(Coverage::Complete, vec![soul("a", 15)]))];
        let mark = |p| {
            derive(&log, &readings, p)
                .get(&id("a"))
                .and_then(|s| s.mark)
        };
        assert_eq!(mark(P), Some(Mark::Discard));
        assert_eq!(mark(Q), None);
    }

    #[test]
    fn a_record_that_cannot_be_a_soul_is_reported_not_repaired() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
        ];
        let mut bad = soul("b", 15);
        bad.star = Some(GameStar(9));
        let readings = [(1, reading(Coverage::Complete, vec![soul("a", 15), bad]))];
        let inv = derive(&log, &readings, P);
        assert_eq!(ids(&inv), vec!["a"]);
        assert_eq!(
            inv.defects(),
            &[SoulDefect {
                soul: id("b"),
                observed_at: 2,
                kind: SoulDefectKind::Star(9),
            }]
        );
    }

    #[test]
    fn a_missing_or_disagreeing_reading_is_an_error() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
        ];
        let p = fold(&log).expect("folds");
        assert_eq!(
            Inventory::derive(&p, P, &BTreeMap::new()),
            Err(InventoryError::MissingReading { digest: [1; 32] })
        );
        let partial = BTreeMap::from([([1; 32], reading(Coverage::Partial, vec![]))]);
        assert_eq!(
            Inventory::derive(&p, P, &partial),
            Err(InventoryError::ReadingDisagrees {
                seq: 2,
                digest: [1; 32]
            })
        );
        assert_eq!(
            Inventory::derive(&p, Q, &partial),
            Err(InventoryError::UnknownProfile { profile: Q })
        );
        let unmapped = SoulReading::new(
            observation::Coverage::Complete,
            None,
            None,
            SoulMappings::default(),
            vec![],
        );
        assert!(matches!(
            Inventory::derive(&p, P, &BTreeMap::from([([1; 32], unmapped)])),
            Err(InventoryError::ReadingRefused { seq: 2, .. })
        ));
    }
}
