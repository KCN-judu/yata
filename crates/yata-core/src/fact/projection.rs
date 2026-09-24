//! The fold: facts to the projection (ADR-0002, layer 2).
//!
//! The projection is a value computed from the log and nothing else. Commits apply in `seq`
//! order, and the facts of a commit in their order within it; `recorded_at` orders nothing. A
//! commit that does not apply is an error that names its `seq`, never a fact skipped: a fold that
//! silently drops a fact produces a projection that looks valid and is not (`fact-format.md`,
//! § Reading old facts). The daemon applies a commit here before writing it, so the rules that
//! refuse a command are the rules that refuse a log.

use std::collections::BTreeMap;

use super::model::{
    Acquisition, Commit, Coverage, Digest, Fact, FactBody, GameAccountId, GameSoulId, Mark,
    ProfileId, Scope,
};

/// Everything the log says, as of `revision`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Projection {
    revision: u64,
    profiles: BTreeMap<ProfileId, ProfileState>,
}

/// One profile's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileState {
    display_name: String,
    retired: bool,
    /// The account named when the profile was created, if any.
    created_account: Option<GameAccountId>,
    acquisitions: Vec<AcquisitionRecord>,
    marks: BTreeMap<GameSoulId, Mark>,
    notes: BTreeMap<GameSoulId, String>,
}

/// One `SnapshotAcquired`, where it landed, and whether a later fact withdrew it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcquisitionRecord {
    pub seq: u64,
    pub acquisition: Acquisition,
    pub retracted: bool,
}

/// The snapshots the inventory of one `(profile, scope)` is built from: the live complete one,
/// and the partial ones after it, oldest first.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LiveSnapshots {
    pub complete: Option<(u64, Digest)>,
    pub partials: Vec<(u64, Digest)>,
}

impl LiveSnapshots {
    /// Every digest the inventory reads, in the order it reads them.
    pub fn digests(&self) -> impl Iterator<Item = &Digest> {
        self.complete.iter().chain(&self.partials).map(|(_, d)| d)
    }
}

/// Why a commit does not apply. Each variant names the commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoldError {
    /// Commits apply densely from 1.
    SeqNotNext { expected: u64, found: u64 },
    /// A commit holds at least one fact.
    EmptyCommit { seq: u64 },
    /// A fact names a profile no earlier fact created.
    UnknownProfile { seq: u64, profile: ProfileId },
    /// A profile is created once.
    ProfileExists { seq: u64, profile: ProfileId },
    /// `import.profile_mismatch`: a reading of another account than the one the profile knows.
    ProfileMismatch {
        seq: u64,
        profile: ProfileId,
        known: GameAccountId,
        observed: GameAccountId,
    },
    /// A retraction names no acquisition of that blob in the profile that is not already
    /// withdrawn.
    RetractsNothing {
        seq: u64,
        profile: ProfileId,
        digest: Digest,
    },
}

/// Fold a whole log, from the empty projection.
pub fn fold(commits: &[Commit]) -> Result<Projection, FoldError> {
    commits
        .iter()
        .try_fold(Projection::default(), Projection::apply)
}

impl Projection {
    /// The `seq` of the last commit applied; 0 for the empty log.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn profile(&self, profile: ProfileId) -> Option<&ProfileState> {
        self.profiles.get(&profile)
    }

    pub fn profiles(&self) -> impl Iterator<Item = (&ProfileId, &ProfileState)> {
        self.profiles.iter()
    }

    /// Whether two projections hold the same state, whatever their revisions. A command whose
    /// commit would leave the state as it is changes nothing and is not written
    /// (`fact-format.md`: the log never records a no-op).
    pub fn same_state(&self, other: &Projection) -> bool {
        self.profiles == other.profiles
    }

    /// Apply the next commit. Consumes the projection: a caller that must keep it on failure
    /// clones it first.
    pub fn apply(mut self, commit: &Commit) -> Result<Projection, FoldError> {
        let expected = self.revision + 1;
        if commit.seq != expected {
            return Err(FoldError::SeqNotNext {
                expected,
                found: commit.seq,
            });
        }
        if commit.facts.is_empty() {
            return Err(FoldError::EmptyCommit { seq: commit.seq });
        }
        for fact in &commit.facts {
            self.apply_fact(commit.seq, fact)?;
        }
        self.revision = commit.seq;
        Ok(self)
    }

    fn apply_fact(&mut self, seq: u64, fact: &Fact) -> Result<(), FoldError> {
        let profile = fact.profile;
        if let FactBody::ProfileCreated {
            display_name,
            account,
        } = &fact.body
        {
            if self.profiles.contains_key(&profile) {
                return Err(FoldError::ProfileExists { seq, profile });
            }
            self.profiles.insert(
                profile,
                ProfileState {
                    display_name: display_name.clone(),
                    retired: false,
                    created_account: account.clone(),
                    acquisitions: Vec::new(),
                    marks: BTreeMap::new(),
                    notes: BTreeMap::new(),
                },
            );
            return Ok(());
        }
        let state = self
            .profiles
            .get_mut(&profile)
            .ok_or(FoldError::UnknownProfile { seq, profile })?;
        match &fact.body {
            FactBody::ProfileCreated { .. } => {}
            FactBody::ProfileRenamed { display_name } => {
                state.display_name.clone_from(display_name);
            }
            FactBody::ProfileRetired => state.retired = true,
            FactBody::ProfileRestored => state.retired = false,
            FactBody::SnapshotAcquired(acquisition) => {
                if let (Some(known), Some(observed)) =
                    (state.known_account(), &acquisition.observed_account)
                    && known != observed
                {
                    return Err(FoldError::ProfileMismatch {
                        seq,
                        profile,
                        known: known.clone(),
                        observed: observed.clone(),
                    });
                }
                state.acquisitions.push(AcquisitionRecord {
                    seq,
                    acquisition: acquisition.clone(),
                    retracted: false,
                });
            }
            FactBody::SnapshotRetracted { digest, .. } => {
                let mut withdrawn = 0;
                for r in &mut state.acquisitions {
                    if r.acquisition.digest == *digest && !r.retracted {
                        r.retracted = true;
                        withdrawn += 1;
                    }
                }
                if withdrawn == 0 {
                    return Err(FoldError::RetractsNothing {
                        seq,
                        profile,
                        digest: *digest,
                    });
                }
            }
            FactBody::SoulMarked { soul, mark } => match mark {
                Some(m) => {
                    state.marks.insert(soul.clone(), *m);
                }
                None => {
                    state.marks.remove(soul);
                }
            },
            FactBody::SoulNoted { soul, text } => {
                if text.is_empty() {
                    state.notes.remove(soul);
                } else {
                    state.notes.insert(soul.clone(), text.clone());
                }
            }
        }
        Ok(())
    }
}

impl ProfileState {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn is_retired(&self) -> bool {
        self.retired
    }

    /// The account this profile's readings must belong to: the one named at creation, else the
    /// one of its earliest acquisition that carries one and is not withdrawn. `None` until
    /// either exists.
    pub fn known_account(&self) -> Option<&GameAccountId> {
        self.created_account.as_ref().or_else(|| {
            self.acquisitions
                .iter()
                .filter(|r| !r.retracted)
                .find_map(|r| r.acquisition.observed_account.as_ref())
        })
    }

    /// Every acquisition, in log order, withdrawn ones included.
    pub fn acquisitions(&self) -> &[AcquisitionRecord] {
        &self.acquisitions
    }

    pub fn mark(&self, soul: &GameSoulId) -> Option<Mark> {
        self.marks.get(soul).copied()
    }

    pub fn note(&self, soul: &GameSoulId) -> Option<&str> {
        self.notes.get(soul).map(String::as_str)
    }

    /// The live complete snapshot of `scope`, its latest complete acquisition not withdrawn, and
    /// the partial ones after it. With no complete snapshot, every partial one not withdrawn.
    pub fn live(&self, scope: Scope) -> LiveSnapshots {
        let current: Vec<&AcquisitionRecord> = self
            .acquisitions
            .iter()
            .filter(|r| !r.retracted && r.acquisition.scope == scope)
            .collect();
        let base = current
            .iter()
            .rposition(|r| r.acquisition.coverage == Coverage::Complete);
        let after = base.map_or(0, |i| i + 1);
        LiveSnapshots {
            complete: base.map(|i| (current[i].seq, current[i].acquisition.digest)),
            partials: current[after..]
                .iter()
                .map(|r| (r.seq, r.acquisition.digest))
                .collect(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::fact::model::{Channel, Origin, ProbeVersion, Source};

    pub(crate) const P: ProfileId = ProfileId([1; 16]);
    pub(crate) const Q: ProfileId = ProfileId([2; 16]);

    pub(crate) fn commit(seq: u64, facts: Vec<Fact>) -> Commit {
        Commit {
            seq,
            recorded_at_ms: 0,
            origin: Origin::Command { request_id: seq },
            facts,
        }
    }

    pub(crate) fn created(profile: ProfileId, account: Option<&str>) -> Fact {
        Fact {
            profile,
            body: FactBody::ProfileCreated {
                display_name: "main".into(),
                account: account.map(|a| GameAccountId::new(a).expect("non-empty")),
            },
        }
    }

    pub(crate) fn acquired(
        profile: ProfileId,
        digest: u8,
        coverage: Coverage,
        account: Option<&str>,
    ) -> Fact {
        Fact {
            profile,
            body: FactBody::SnapshotAcquired(Acquisition {
                digest: [digest; 32],
                scope: Scope::Souls,
                coverage,
                channel: Channel::MumuAdb,
                source: Source::ExportFile,
                probe_build_id: "test".into(),
                probe_version: ProbeVersion { major: 1, minor: 0 },
                observed_account: account.map(|a| GameAccountId::new(a).expect("non-empty")),
            }),
        }
    }

    fn retracted(profile: ProfileId, digest: u8) -> Fact {
        Fact {
            profile,
            body: FactBody::SnapshotRetracted {
                digest: [digest; 32],
                reason: String::new(),
            },
        }
    }

    pub(crate) fn marked(profile: ProfileId, soul: &str, mark: Option<Mark>) -> Fact {
        Fact {
            profile,
            body: FactBody::SoulMarked {
                soul: GameSoulId::new(soul).expect("non-empty"),
                mark,
            },
        }
    }

    fn id(s: &str) -> GameSoulId {
        GameSoulId::new(s).expect("non-empty")
    }

    #[test]
    fn the_fold_is_a_function_of_the_log() {
        let log = vec![
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 7, Coverage::Complete, Some("acct"))]),
            commit(3, vec![marked(P, "a", Some(Mark::Keep))]),
        ];
        let once = fold(&log).expect("folds");
        assert_eq!(fold(&log), Ok(once.clone()));
        assert_eq!(once.revision(), 3);
    }

    #[test]
    fn commits_apply_densely_in_seq_order() {
        let a = commit(1, vec![created(P, None)]);
        let b = commit(2, vec![marked(P, "a", Some(Mark::Keep))]);
        assert_eq!(
            fold(&[b.clone(), a.clone()]),
            Err(FoldError::SeqNotNext {
                expected: 1,
                found: 2
            })
        );
        assert_eq!(
            fold(&[a.clone(), a.clone()]),
            Err(FoldError::SeqNotNext {
                expected: 2,
                found: 1
            })
        );
        assert!(fold(&[a, b]).is_ok());
    }

    #[test]
    fn facts_within_a_commit_apply_in_their_order() {
        let keep_then_clear = commit(
            2,
            vec![marked(P, "a", Some(Mark::Keep)), marked(P, "a", None)],
        );
        let clear_then_keep = commit(
            2,
            vec![marked(P, "a", None), marked(P, "a", Some(Mark::Keep))],
        );
        let base = commit(1, vec![created(P, None)]);
        let first = fold(&[base.clone(), keep_then_clear]).expect("folds");
        let second = fold(&[base, clear_then_keep]).expect("folds");
        let mark = |p: &Projection| p.profile(P).and_then(|s| s.mark(&id("a")));
        assert_eq!(mark(&first), None);
        assert_eq!(mark(&second), Some(Mark::Keep));
    }

    #[test]
    fn a_commit_without_facts_does_not_apply() {
        assert_eq!(
            fold(&[commit(1, vec![])]),
            Err(FoldError::EmptyCommit { seq: 1 })
        );
    }

    #[test]
    fn a_fact_needs_its_profile_to_exist_and_a_profile_is_created_once() {
        assert_eq!(
            fold(&[commit(1, vec![marked(P, "a", Some(Mark::Keep))])]),
            Err(FoldError::UnknownProfile { seq: 1, profile: P })
        );
        assert_eq!(
            fold(&[
                commit(1, vec![created(P, None)]),
                commit(2, vec![created(P, None)])
            ]),
            Err(FoldError::ProfileExists { seq: 2, profile: P })
        );
    }

    #[test]
    fn profiles_share_nothing() {
        let p = fold(&[
            commit(1, vec![created(P, Some("x")), created(Q, Some("y"))]),
            commit(2, vec![marked(P, "a", Some(Mark::Discard))]),
            commit(3, vec![acquired(Q, 9, Coverage::Complete, Some("y"))]),
        ])
        .expect("folds");
        let (sp, sq) = (p.profile(P).expect("P"), p.profile(Q).expect("Q"));
        assert_eq!(sp.mark(&id("a")), Some(Mark::Discard));
        assert_eq!(sq.mark(&id("a")), None);
        assert!(sp.acquisitions().is_empty());
        assert_eq!(sq.live(Scope::Souls).complete, Some((3, [9; 32])));
    }

    #[test]
    fn a_reading_of_another_account_does_not_apply() {
        let mismatch =
            |log: &[Commit]| matches!(fold(log), Err(FoldError::ProfileMismatch { seq: 3, .. }));
        // known from creation
        assert!(mismatch(&[
            commit(1, vec![created(P, Some("x"))]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, Some("x"))]),
            commit(3, vec![acquired(P, 2, Coverage::Complete, Some("y"))]),
        ]));
        // learned from the first reading that carried one
        assert!(mismatch(&[
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, Some("x"))]),
            commit(3, vec![acquired(P, 2, Coverage::Partial, Some("y"))]),
        ]));
        // a reading that did not read the account is not a mismatch
        assert!(
            fold(&[
                commit(1, vec![created(P, Some("x"))]),
                commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
            ])
            .is_ok()
        );
    }

    #[test]
    fn a_withdrawn_reading_no_longer_decides_the_account() {
        let p = fold(&[
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, Some("x"))]),
            commit(3, vec![retracted(P, 1)]),
            commit(4, vec![acquired(P, 2, Coverage::Complete, Some("y"))]),
        ])
        .expect("folds");
        assert_eq!(
            p.profile(P).and_then(ProfileState::known_account),
            Some(&GameAccountId::new("y").expect("non-empty"))
        );
    }

    #[test]
    fn the_live_snapshot_is_the_latest_complete_one_not_withdrawn() {
        let log = vec![
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Partial, None)]),
            commit(3, vec![acquired(P, 2, Coverage::Complete, None)]),
            commit(4, vec![acquired(P, 3, Coverage::Partial, None)]),
            commit(5, vec![acquired(P, 4, Coverage::Complete, None)]),
            commit(6, vec![acquired(P, 5, Coverage::Partial, None)]),
        ];
        let p = fold(&log).expect("folds");
        assert_eq!(
            p.profile(P).map(|s| s.live(Scope::Souls)),
            Some(LiveSnapshots {
                complete: Some((5, [4; 32])),
                partials: vec![(6, [5; 32])],
            })
        );
        let mut withdrawn = log;
        withdrawn.push(commit(7, vec![retracted(P, 4)]));
        let p = fold(&withdrawn).expect("folds");
        assert_eq!(
            p.profile(P).map(|s| s.live(Scope::Souls)),
            Some(LiveSnapshots {
                complete: Some((3, [2; 32])),
                partials: vec![(4, [3; 32]), (6, [5; 32])],
            })
        );
    }

    #[test]
    fn without_a_complete_snapshot_every_partial_one_is_live() {
        let p = fold(&[
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Partial, None)]),
            commit(3, vec![acquired(P, 2, Coverage::Partial, None)]),
        ])
        .expect("folds");
        assert_eq!(
            p.profile(P).map(|s| s.live(Scope::Souls)),
            Some(LiveSnapshots {
                complete: None,
                partials: vec![(2, [1; 32]), (3, [2; 32])],
            })
        );
    }

    #[test]
    fn a_retraction_withdraws_only_what_came_before_it() {
        let p = fold(&[
            commit(1, vec![created(P, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
            commit(3, vec![acquired(P, 1, Coverage::Complete, None)]),
            commit(4, vec![retracted(P, 1)]),
            commit(5, vec![acquired(P, 1, Coverage::Complete, None)]),
        ])
        .expect("folds");
        let s = p.profile(P).expect("P");
        let withdrawn: Vec<bool> = s.acquisitions().iter().map(|r| r.retracted).collect();
        assert_eq!(withdrawn, vec![true, true, false]);
        assert_eq!(s.live(Scope::Souls).complete, Some((5, [1; 32])));
    }

    #[test]
    fn a_retraction_must_withdraw_something_in_its_own_profile() {
        let base = vec![
            commit(1, vec![created(P, None), created(Q, None)]),
            commit(2, vec![acquired(P, 1, Coverage::Complete, None)]),
        ];
        let mut other_profile = base.clone();
        other_profile.push(commit(3, vec![retracted(Q, 1)]));
        assert!(matches!(
            fold(&other_profile),
            Err(FoldError::RetractsNothing { seq: 3, .. })
        ));
        let mut twice = base;
        twice.push(commit(3, vec![retracted(P, 1)]));
        twice.push(commit(4, vec![retracted(P, 1)]));
        assert!(matches!(
            fold(&twice),
            Err(FoldError::RetractsNothing { seq: 4, .. })
        ));
    }

    #[test]
    fn a_state_is_the_same_whatever_the_revision() {
        let base = fold(&[commit(1, vec![created(P, None)])]).expect("folds");
        let keep = base
            .clone()
            .apply(&commit(2, vec![marked(P, "a", Some(Mark::Keep))]))
            .expect("applies");
        let undone = keep
            .clone()
            .apply(&commit(3, vec![marked(P, "a", None)]))
            .expect("applies");
        assert!(!base.same_state(&keep));
        assert!(base.same_state(&undone));
        assert_ne!(base, undone);
    }
}
