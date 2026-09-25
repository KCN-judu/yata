//! A profile's current soul inventory: the live snapshots' souls sections overlaid, with the
//! user's decisions attached (`fact-format.md`, § Imports; ADR-0032, rule 7). This is the value
//! queries iterate over.
//!
//! A soul absent from the base is absent from the account. A layer replaces the records of the
//! souls it contains and says nothing about the others. A decision about a soul that is not in
//! the inventory is kept in the projection and applies again if the soul reappears.

use std::collections::BTreeMap;

use super::model::{Digest, GameSoulId, Mark, NoteText, ProfileId, Revision, Seq};
use super::projection::Projection;
use crate::import::admit::{SoulAdmissionError, admit};
use crate::import::ir::{Completeness, IrError, SectionKind, YataSnapshot};
use crate::soul::Soul;

/// One profile's souls as of one revision.
#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    revision: Revision,
    souls: BTreeMap<GameSoulId, InventorySoul>,
    defects: Vec<SoulDefect>,
}

/// One soul of the inventory: its admitted value, the import it was last seen in, and the user's
/// decisions.
#[derive(Debug, Clone, PartialEq)]
pub struct InventorySoul {
    pub id: GameSoulId,
    pub soul: Soul,
    /// The import this record comes from.
    pub observed_at: Seq,
    pub mark: Option<Mark>,
    pub note: Option<NoteText>,
}

/// A live record that admission refused. It is reported and is not a row.
#[derive(Debug, Clone, PartialEq)]
pub struct SoulDefect {
    /// The record's id; `None` only for a record that has none.
    pub soul: Option<GameSoulId>,
    pub observed_at: Seq,
    pub reason: SoulAdmissionError,
}

/// Why an inventory could not be derived. Each is a store inconsistency, not a user error.
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryError {
    UnknownProfile {
        profile: ProfileId,
    },
    /// A live snapshot was not supplied.
    MissingSnapshot {
        snapshot: Digest,
    },
    /// A snapshot's souls section is not what its import recorded.
    SnapshotDisagrees {
        seq: Seq,
        snapshot: Digest,
        recorded: Completeness,
        found: Option<Completeness>,
    },
    /// A live snapshot breaks the IR's own rules; it was checked when it landed.
    SnapshotRefused {
        seq: Seq,
        snapshot: Digest,
        error: IrError,
    },
}

/// The latest live record of one soul: a row, or a defect.
enum Latest {
    Row(Seq, Soul),
    Defect(Seq, SoulAdmissionError),
}

impl Inventory {
    /// Derive a profile's soul inventory. `snapshots` must hold every snapshot of
    /// [`crate::fact::ProfileState::live`] for [`SectionKind::Souls`]; others are ignored.
    pub fn derive(
        projection: &Projection,
        profile: ProfileId,
        snapshots: &BTreeMap<Digest, YataSnapshot>,
    ) -> Result<Inventory, InventoryError> {
        let state = projection
            .profile(profile)
            .ok_or(InventoryError::UnknownProfile { profile })?;
        let mut latest: BTreeMap<GameSoulId, Latest> = BTreeMap::new();
        let mut anonymous = Vec::new();
        for (seq, digest, recorded) in state.live(SectionKind::Souls).snapshots() {
            let snapshot = snapshots
                .get(&digest)
                .ok_or(InventoryError::MissingSnapshot { snapshot: digest })?;
            let admitted = admit(snapshot).map_err(|error| InventoryError::SnapshotRefused {
                seq,
                snapshot: digest,
                error,
            })?;
            let Some((_, souls)) = admitted
                .souls
                .present()
                .filter(|(found, _)| *found == recorded)
            else {
                return Err(InventoryError::SnapshotDisagrees {
                    seq,
                    snapshot: digest,
                    recorded,
                    found: admitted.souls.present().map(|(c, _)| c),
                });
            };
            latest.extend(
                souls
                    .values
                    .iter()
                    .map(|(id, soul)| (id.clone(), Latest::Row(seq, soul.clone()))),
            );
            for r in &souls.rejected {
                match &r.id {
                    Some(id) => {
                        latest.insert(
                            GameSoulId::from_source(id),
                            Latest::Defect(seq, r.reason.clone()),
                        );
                    }
                    None => anonymous.push(SoulDefect {
                        soul: None,
                        observed_at: seq,
                        reason: r.reason.clone(),
                    }),
                }
            }
        }
        let mut souls = BTreeMap::new();
        let mut defects = Vec::new();
        for (id, entry) in latest {
            match entry {
                Latest::Row(observed_at, soul) => {
                    let row = InventorySoul {
                        observed_at,
                        mark: state.mark(&id),
                        note: state.note(&id).cloned(),
                        soul,
                        id: id.clone(),
                    };
                    souls.insert(id, row);
                }
                Latest::Defect(observed_at, reason) => defects.push(SoulDefect {
                    soul: Some(id),
                    observed_at,
                    reason,
                }),
            }
        }
        defects.extend(anonymous);
        Ok(Inventory {
            revision: projection.revision(),
            souls,
            defects,
        })
    }

    pub fn revision(&self) -> Revision {
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

    /// The live records left out because admission refused them, ordered by game soul id.
    pub fn defects(&self) -> &[SoulDefect] {
        &self.defects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fact::model::Commit;
    use crate::fact::projection::fold;
    use crate::fact::projection::tests::{
        P, Q, commit, created, imported, marked, retracted, seq, souls,
    };
    use crate::import::fixture::{complete, empty, soul, with_souls};
    use crate::import::ir::{Guild, Section, SoulRecord, Souls};
    use crate::soul::Level;

    use Completeness::{Complete, Partial, Unstated};

    fn id(s: &str) -> GameSoulId {
        GameSoulId::new(s).expect("non-empty")
    }

    fn at(level: i64, name: &str) -> SoulRecord {
        SoulRecord {
            level,
            ..soul(name, 2)
        }
    }

    fn snapshot(completeness: Completeness, records: Vec<SoulRecord>) -> YataSnapshot {
        YataSnapshot {
            souls: Section::Present {
                completeness,
                value: Souls { souls: records },
            },
            ..empty()
        }
    }

    fn derive(log: &[Commit], snapshots: &[(u8, YataSnapshot)], profile: ProfileId) -> Inventory {
        let p = fold(log).expect("folds");
        let snapshots = snapshots
            .iter()
            .map(|(d, s)| (Digest([*d; 32]), s.clone()))
            .collect();
        Inventory::derive(&p, profile, &snapshots).expect("derives")
    }

    fn ids(inv: &Inventory) -> Vec<&str> {
        inv.souls().map(|s| s.id.as_str()).collect()
    }

    fn level(n: u32) -> Level {
        Level::try_from(n).expect("a level")
    }

    #[test]
    fn a_newer_complete_import_replaces_the_inventory() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
            commit(3, vec![souls(P, 2, Complete)]),
        ];
        let snapshots = [
            (1, snapshot(Complete, vec![at(12, "a"), at(3, "b")])),
            (2, snapshot(Complete, vec![at(15, "a"), at(0, "c")])),
        ];
        let inv = derive(&log, &snapshots, P);
        assert_eq!(ids(&inv), vec!["a", "c"]);
        let a = inv.get(&id("a")).expect("a");
        assert_eq!((a.soul.level, a.observed_at), (level(15), seq(3)));
    }

    #[test]
    fn a_partial_or_unstated_import_overlays_and_removes_nothing() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
            commit(3, vec![souls(P, 2, Partial)]),
            commit(4, vec![souls(P, 3, Unstated)]),
        ];
        let snapshots = [
            (1, snapshot(Complete, vec![at(12, "a"), at(3, "b")])),
            (2, snapshot(Partial, vec![at(6, "b"), at(0, "d")])),
            (3, snapshot(Unstated, vec![at(9, "e")])),
        ];
        let inv = derive(&log, &snapshots, P);
        assert_eq!(ids(&inv), vec!["a", "b", "d", "e"]);
        assert_eq!(inv.get(&id("a")).map(|s| s.observed_at), Some(seq(2)));
        assert_eq!(inv.get(&id("b")).map(|s| s.soul.level), Some(level(6)));
    }

    #[test]
    fn an_import_without_souls_leaves_the_inventory_alone() {
        let guild_only = YataSnapshot {
            guild: complete(Guild {
                level: 10,
                member_count: 30,
            }),
            ..empty()
        };
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
            commit(3, vec![imported(P, 2, &[(SectionKind::Guild, Complete)])]),
        ];
        let snapshots = [(1, with_souls(vec![at(15, "a")])), (2, guild_only)];
        assert_eq!(ids(&derive(&log, &snapshots, P)), vec!["a"]);
    }

    #[test]
    fn importing_the_same_snapshot_again_changes_no_soul() {
        let once = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
        ];
        let mut twice = once.to_vec();
        twice.push(commit(3, vec![souls(P, 1, Complete)]));
        let snapshots = [(1, with_souls(vec![at(15, "a"), at(3, "b")]))];
        let (a, b) = (derive(&once, &snapshots, P), derive(&twice, &snapshots, P));
        let rows = |inv: &Inventory| {
            inv.souls()
                .map(|s| (s.id.clone(), s.soul.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(rows(&a), rows(&b));
    }

    #[test]
    fn a_retracted_import_leaves_the_inventory() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
            commit(3, vec![souls(P, 2, Complete)]),
            commit(4, vec![retracted(P, 2)]),
        ];
        let snapshots = [
            (1, with_souls(vec![at(15, "a")])),
            (2, with_souls(vec![at(15, "b")])),
        ];
        assert_eq!(ids(&derive(&log, &snapshots, P)), vec!["a"]);
    }

    #[test]
    fn a_decision_outlives_its_soul_and_applies_when_it_returns() {
        let mut log = vec![
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
            commit(3, vec![marked(P, "a", Some(Mark::Keep))]),
            commit(4, vec![souls(P, 2, Complete)]),
        ];
        let snapshots = [
            (1, with_souls(vec![at(15, "a")])),
            (2, with_souls(vec![])),
            (3, with_souls(vec![at(15, "a")])),
        ];
        assert!(derive(&log, &snapshots, P).is_empty());
        log.push(commit(5, vec![souls(P, 3, Complete)]));
        let back = derive(&log, &snapshots, P);
        assert_eq!(back.get(&id("a")).and_then(|s| s.mark), Some(Mark::Keep));
    }

    #[test]
    fn decisions_belong_to_one_profile() {
        let log = [
            commit(1, vec![created(P, None), created(Q, None)]),
            commit(2, vec![souls(P, 1, Complete), souls(Q, 1, Complete)]),
            commit(3, vec![marked(P, "a", Some(Mark::Discard))]),
        ];
        let snapshots = [(1, with_souls(vec![at(15, "a")]))];
        let mark = |p| {
            derive(&log, &snapshots, p)
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
            commit(2, vec![souls(P, 1, Complete)]),
        ];
        let bad = SoulRecord {
            star: 9,
            ..at(15, "b")
        };
        let snapshots = [(1, with_souls(vec![at(15, "a"), bad]))];
        let inv = derive(&log, &snapshots, P);
        assert_eq!(ids(&inv), vec!["a"]);
        assert_eq!(
            inv.defects(),
            &[SoulDefect {
                soul: Some(id("b")),
                observed_at: seq(2),
                reason: SoulAdmissionError::NotAStar { n: 9 },
            }]
        );
    }

    #[test]
    fn a_missing_disagreeing_or_refused_snapshot_is_an_error() {
        let log = [
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
        ];
        let p = fold(&log).expect("folds");
        let d = Digest([1; 32]);
        assert_eq!(
            Inventory::derive(&p, P, &BTreeMap::new()),
            Err(InventoryError::MissingSnapshot { snapshot: d })
        );
        let partial = BTreeMap::from([(d, snapshot(Partial, vec![]))]);
        assert_eq!(
            Inventory::derive(&p, P, &partial),
            Err(InventoryError::SnapshotDisagrees {
                seq: seq(2),
                snapshot: d,
                recorded: Complete,
                found: Some(Partial),
            })
        );
        assert_eq!(
            Inventory::derive(&p, P, &BTreeMap::from([(d, empty())])),
            Err(InventoryError::SnapshotDisagrees {
                seq: seq(2),
                snapshot: d,
                recorded: Complete,
                found: None,
            })
        );
        assert_eq!(
            Inventory::derive(&p, Q, &partial),
            Err(InventoryError::UnknownProfile { profile: Q })
        );
        let duplicated = BTreeMap::from([(d, with_souls(vec![at(1, "a"), at(2, "a")]))]);
        assert!(matches!(
            Inventory::derive(&p, P, &duplicated),
            Err(InventoryError::SnapshotRefused { seq: s, .. }) if s == seq(2)
        ));
    }
}
