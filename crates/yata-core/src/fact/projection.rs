//! The fold: facts to the projection (ADR-0002, layer 2).
//!
//! The projection is a value computed from the log and nothing else. Commits apply in `seq`
//! order, and the facts of a commit in their order within it; `recorded_at` orders nothing. A
//! commit that does not apply is an error that names its `seq`, never a fact skipped: a fold that
//! silently drops a fact produces a projection that looks valid and is not (`fact-format.md`,
//! § Reading old facts). The daemon applies a commit here before writing it, so the rules that
//! refuse a command are the rules that refuse a log.
//!
//! Imports are kept per profile, and read per section (ADR-0032, rule 6): an import that does not
//! carry a section is not part of it, so importing a guild alone never changes the souls.

use std::collections::BTreeMap;

use super::model::{
    Commit, Digest, Fact, FactBody, GameAccountId, GameSoulId, Mark, NoteText, ProfileId, Revision,
    Seq, SnapshotImport,
};
use crate::import::ir::{Completeness, SectionKind};

/// Everything the log says, as of `revision`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Projection {
    revision: Revision,
    profiles: BTreeMap<ProfileId, ProfileState>,
}

/// Whether a profile shows in the UI. A retired profile keeps its facts and can be restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileStatus {
    Active,
    Retired,
}

/// One profile's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileState {
    display_name: String,
    status: ProfileStatus,
    /// The account named when the profile was created, if any.
    account: Option<GameAccountId>,
    imports: Vec<ImportRecord>,
    marks: BTreeMap<GameSoulId, Mark>,
    notes: BTreeMap<GameSoulId, NoteText>,
}

/// Whether a later fact withdrew an import.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportStatus {
    Current,
    Retracted,
}

/// One `SnapshotImported`, where it landed, and whether a later fact withdrew it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportRecord {
    pub seq: Seq,
    pub import: SnapshotImport,
    pub status: ImportStatus,
}

/// The imports one `(profile, section)` is built from: the base, its latest complete import not
/// withdrawn, and the partial or unstated ones after it, oldest first. With no base, every live
/// import of the section is a layer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LiveSection {
    pub base: Option<(Seq, Digest)>,
    pub layers: Vec<Layer>,
}

/// A live import laid over the base, with the completeness it states for the section: `Partial`
/// or `Unstated`, never `Complete`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layer {
    pub seq: Seq,
    pub snapshot: Digest,
    pub completeness: Completeness,
}

impl LiveSection {
    /// Every snapshot the section reads, in the order it reads them, with its completeness.
    pub fn snapshots(&self) -> impl Iterator<Item = (Seq, Digest, Completeness)> + '_ {
        self.base
            .iter()
            .map(|&(seq, d)| (seq, d, Completeness::Complete))
            .chain(
                self.layers
                    .iter()
                    .map(|l| (l.seq, l.snapshot, l.completeness)),
            )
    }

    /// What the profile holds of the section, or `None` when no live import carries it.
    pub fn held(&self) -> Option<Completeness> {
        match self.base {
            Some(_) => Some(Completeness::Complete),
            None => self.layers.iter().map(|l| l.completeness).max(),
        }
    }
}

/// Why a commit does not apply. Each variant names the commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoldError {
    /// Commits apply densely from 1. `expected` is `None` only past `u64::MAX`.
    SeqNotNext { expected: Option<Seq>, found: Seq },
    /// A fact names a profile no earlier fact created.
    UnknownProfile { seq: Seq, profile: ProfileId },
    /// A profile is created once.
    ProfileExists { seq: Seq, profile: ProfileId },
    /// A retraction names no import of that snapshot in the profile that is not already
    /// withdrawn.
    RetractsNothing {
        seq: Seq,
        profile: ProfileId,
        snapshot: Digest,
    },
}

/// Fold a whole log, from the empty projection.
pub fn fold(commits: &[Commit]) -> Result<Projection, FoldError> {
    commits
        .iter()
        .try_fold(Projection::default(), Projection::apply)
}

impl Projection {
    /// The `seq` of the last commit applied.
    pub fn revision(&self) -> Revision {
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
        let expected = self.revision.next();
        if expected != Some(commit.seq) {
            return Err(FoldError::SeqNotNext {
                expected,
                found: commit.seq,
            });
        }
        for fact in commit.facts.as_slice() {
            self.apply_fact(commit.seq, fact)?;
        }
        self.revision = Revision::at(commit.seq);
        Ok(self)
    }

    fn apply_fact(&mut self, seq: Seq, fact: &Fact) -> Result<(), FoldError> {
        let profile = fact.profile;
        let Some(state) = self.profiles.get_mut(&profile) else {
            return match &fact.body {
                FactBody::ProfileCreated {
                    display_name,
                    account,
                } => {
                    self.profiles.insert(
                        profile,
                        ProfileState {
                            display_name: display_name.clone(),
                            status: ProfileStatus::Active,
                            account: account.clone(),
                            imports: Vec::new(),
                            marks: BTreeMap::new(),
                            notes: BTreeMap::new(),
                        },
                    );
                    Ok(())
                }
                _ => Err(FoldError::UnknownProfile { seq, profile }),
            };
        };
        match &fact.body {
            FactBody::ProfileCreated { .. } => Err(FoldError::ProfileExists { seq, profile }),
            FactBody::ProfileRenamed { display_name } => {
                state.display_name.clone_from(display_name);
                Ok(())
            }
            FactBody::ProfileRetired => {
                state.status = ProfileStatus::Retired;
                Ok(())
            }
            FactBody::ProfileRestored => {
                state.status = ProfileStatus::Active;
                Ok(())
            }
            FactBody::SnapshotImported(import) => {
                state.imports.push(ImportRecord {
                    seq,
                    import: import.clone(),
                    status: ImportStatus::Current,
                });
                Ok(())
            }
            FactBody::SnapshotRetracted { snapshot, .. } => {
                let withdrawn = state
                    .imports
                    .iter_mut()
                    .filter(|r| r.import.snapshot == *snapshot && r.status == ImportStatus::Current)
                    .map(|r| r.status = ImportStatus::Retracted)
                    .count();
                if withdrawn == 0 {
                    return Err(FoldError::RetractsNothing {
                        seq,
                        profile,
                        snapshot: *snapshot,
                    });
                }
                Ok(())
            }
            FactBody::SoulMarked { soul, mark } => {
                match mark {
                    Some(m) => state.marks.insert(soul.clone(), *m),
                    None => state.marks.remove(soul),
                };
                Ok(())
            }
            FactBody::SoulNoted { soul, note } => {
                match note {
                    Some(n) => state.notes.insert(soul.clone(), n.clone()),
                    None => state.notes.remove(soul),
                };
                Ok(())
            }
        }
    }
}

impl ProfileState {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn status(&self) -> ProfileStatus {
        self.status
    }

    /// The account named when the profile was created; `None` when none was.
    pub fn account(&self) -> Option<&GameAccountId> {
        self.account.as_ref()
    }

    /// Every import, in log order, withdrawn ones included.
    pub fn imports(&self) -> &[ImportRecord] {
        &self.imports
    }

    pub fn mark(&self, soul: &GameSoulId) -> Option<Mark> {
        self.marks.get(soul).copied()
    }

    pub fn note(&self, soul: &GameSoulId) -> Option<&NoteText> {
        self.notes.get(soul)
    }

    /// The live imports of one section (ADR-0032, rule 6).
    pub fn live(&self, kind: SectionKind) -> LiveSection {
        let current: Vec<(Seq, Digest, Completeness)> = self
            .imports
            .iter()
            .filter(|r| r.status == ImportStatus::Current)
            .filter_map(|r| {
                r.import
                    .sections
                    .get(kind)
                    .map(|c| (r.seq, r.import.snapshot, c))
            })
            .collect();
        let base = current
            .iter()
            .rposition(|&(_, _, c)| c == Completeness::Complete);
        let after = base.map_or(0, |i| i + 1);
        LiveSection {
            base: base.map(|i| (current[i].0, current[i].1)),
            layers: current[after..]
                .iter()
                .map(|&(seq, snapshot, completeness)| Layer {
                    seq,
                    snapshot,
                    completeness,
                })
                .collect(),
        }
    }

    /// What the profile holds of each section: the input of
    /// [`crate::import::capability::availability`]. A section no live import carries is absent.
    pub fn held(&self) -> BTreeMap<SectionKind, Completeness> {
        SectionKind::ALL
            .into_iter()
            .filter_map(|k| self.live(k).held().map(|c| (k, c)))
            .collect()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::fact::model::{Facts, Origin, Sections};
    use crate::import::ir::{FormatTag, SourceFormat};

    pub(crate) const P: ProfileId = ProfileId([1; 16]);
    pub(crate) const Q: ProfileId = ProfileId([2; 16]);

    pub(crate) fn seq(n: u64) -> Seq {
        Seq::new(n).expect("a seq")
    }

    pub(crate) fn commit(n: u64, facts: Vec<Fact>) -> Commit {
        Commit {
            seq: seq(n),
            recorded_at_ms: 0,
            origin: Origin::Command { request_id: n },
            facts: Facts::new(facts).expect("at least one fact"),
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

    /// An import of snapshot `digest` holding `sections`.
    pub(crate) fn imported(
        profile: ProfileId,
        digest: u8,
        sections: &[(SectionKind, Completeness)],
    ) -> Fact {
        Fact {
            profile,
            body: FactBody::SnapshotImported(SnapshotImport {
                snapshot: Digest([digest; 32]),
                original: Digest([digest ^ 0xFF; 32]),
                source: SourceFormat::Community(FormatTag::MumuSnapshotV1),
                sections: Sections::new(sections.iter().copied().collect())
                    .expect("at least one section"),
            }),
        }
    }

    /// An import of snapshot `digest` holding the souls alone.
    pub(crate) fn souls(profile: ProfileId, digest: u8, completeness: Completeness) -> Fact {
        imported(profile, digest, &[(SectionKind::Souls, completeness)])
    }

    pub(crate) fn retracted(profile: ProfileId, digest: u8) -> Fact {
        Fact {
            profile,
            body: FactBody::SnapshotRetracted {
                snapshot: Digest([digest; 32]),
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

    fn d(n: u8) -> Digest {
        Digest([n; 32])
    }

    fn layer(n: u64, digest: u8, completeness: Completeness) -> Layer {
        Layer {
            seq: seq(n),
            snapshot: d(digest),
            completeness,
        }
    }

    use Completeness::{Complete, Partial, Unstated};
    use SectionKind::{Guild, Shikigami, Souls};

    #[test]
    fn the_fold_is_a_function_of_the_log() {
        let log = vec![
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 7, Complete)]),
            commit(3, vec![marked(P, "a", Some(Mark::Keep))]),
        ];
        let once = fold(&log).expect("folds");
        assert_eq!(fold(&log), Ok(once.clone()));
        assert_eq!(once.revision(), Revision::at(seq(3)));
    }

    #[test]
    fn commits_apply_densely_in_seq_order() {
        let a = commit(1, vec![created(P, None)]);
        let b = commit(2, vec![marked(P, "a", Some(Mark::Keep))]);
        assert_eq!(
            fold(&[b.clone(), a.clone()]),
            Err(FoldError::SeqNotNext {
                expected: Some(seq(1)),
                found: seq(2)
            })
        );
        assert_eq!(
            fold(&[a.clone(), a.clone()]),
            Err(FoldError::SeqNotNext {
                expected: Some(seq(2)),
                found: seq(1)
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
    fn a_fact_needs_its_profile_to_exist_and_a_profile_is_created_once() {
        assert_eq!(
            fold(&[commit(1, vec![marked(P, "a", Some(Mark::Keep))])]),
            Err(FoldError::UnknownProfile {
                seq: seq(1),
                profile: P
            })
        );
        assert_eq!(
            fold(&[
                commit(1, vec![created(P, None)]),
                commit(2, vec![created(P, None)])
            ]),
            Err(FoldError::ProfileExists {
                seq: seq(2),
                profile: P
            })
        );
    }

    #[test]
    fn profiles_share_nothing() {
        let p = fold(&[
            commit(1, vec![created(P, Some("x")), created(Q, Some("y"))]),
            commit(2, vec![marked(P, "a", Some(Mark::Discard))]),
            commit(3, vec![souls(Q, 9, Complete)]),
        ])
        .expect("folds");
        let (sp, sq) = (p.profile(P).expect("P"), p.profile(Q).expect("Q"));
        assert_eq!(sp.mark(&id("a")), Some(Mark::Discard));
        assert_eq!(sq.mark(&id("a")), None);
        assert!(sp.imports().is_empty());
        assert_eq!(sp.held(), BTreeMap::new());
        assert_eq!(sq.live(Souls).base, Some((seq(3), d(9))));
    }

    #[test]
    fn the_base_is_the_latest_complete_import_not_withdrawn() {
        let log = vec![
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Partial)]),
            commit(3, vec![souls(P, 2, Complete)]),
            commit(4, vec![souls(P, 3, Unstated)]),
            commit(5, vec![souls(P, 4, Complete)]),
            commit(6, vec![souls(P, 5, Partial)]),
        ];
        let p = fold(&log).expect("folds");
        assert_eq!(
            p.profile(P).map(|s| s.live(Souls)),
            Some(LiveSection {
                base: Some((seq(5), d(4))),
                layers: vec![layer(6, 5, Partial)],
            })
        );
        let mut withdrawn = log;
        withdrawn.push(commit(7, vec![retracted(P, 4)]));
        let p = fold(&withdrawn).expect("folds");
        assert_eq!(
            p.profile(P).map(|s| s.live(Souls)),
            Some(LiveSection {
                base: Some((seq(3), d(2))),
                layers: vec![layer(4, 3, Unstated), layer(6, 5, Partial)],
            })
        );
    }

    #[test]
    fn without_a_complete_import_every_live_one_is_a_layer() {
        let p = fold(&[
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Partial)]),
            commit(3, vec![souls(P, 2, Unstated)]),
        ])
        .expect("folds");
        let s = p.profile(P).expect("P");
        assert_eq!(
            s.live(Souls),
            LiveSection {
                base: None,
                layers: vec![layer(2, 1, Partial), layer(3, 2, Unstated)],
            }
        );
        assert_eq!(s.held(), BTreeMap::from([(Souls, Partial)]));
    }

    #[test]
    fn an_import_without_a_section_leaves_that_section_alone() {
        let p = fold(&[
            commit(1, vec![created(P, None)]),
            commit(
                2,
                vec![imported(P, 1, &[(Souls, Complete), (Guild, Complete)])],
            ),
            commit(3, vec![imported(P, 2, &[(Guild, Complete)])]),
            commit(4, vec![imported(P, 3, &[(Souls, Complete)])]),
        ])
        .expect("folds");
        let s = p.profile(P).expect("P");
        assert_eq!(s.live(Souls).base, Some((seq(4), d(3))));
        assert_eq!(s.live(Guild).base, Some((seq(3), d(2))));
        assert_eq!(s.live(Shikigami), LiveSection::default());
        assert_eq!(
            s.held(),
            BTreeMap::from([(Souls, Complete), (Guild, Complete)])
        );
    }

    #[test]
    fn a_retraction_withdraws_every_section_of_its_snapshot() {
        let p = fold(&[
            commit(1, vec![created(P, None)]),
            commit(
                2,
                vec![imported(P, 1, &[(Souls, Complete), (Guild, Partial)])],
            ),
            commit(3, vec![imported(P, 2, &[(Souls, Partial)])]),
            commit(4, vec![retracted(P, 1)]),
        ])
        .expect("folds");
        let s = p.profile(P).expect("P");
        assert_eq!(
            s.live(Souls),
            LiveSection {
                base: None,
                layers: vec![layer(3, 2, Partial)],
            }
        );
        assert_eq!(s.live(Guild), LiveSection::default());
        assert_eq!(s.held(), BTreeMap::from([(Souls, Partial)]));
    }

    #[test]
    fn a_retraction_withdraws_only_what_came_before_it() {
        let p = fold(&[
            commit(1, vec![created(P, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
            commit(3, vec![souls(P, 1, Complete)]),
            commit(4, vec![retracted(P, 1)]),
            commit(5, vec![souls(P, 1, Complete)]),
        ])
        .expect("folds");
        let s = p.profile(P).expect("P");
        let statuses: Vec<ImportStatus> = s.imports().iter().map(|r| r.status).collect();
        assert_eq!(
            statuses,
            vec![
                ImportStatus::Retracted,
                ImportStatus::Retracted,
                ImportStatus::Current
            ]
        );
        assert_eq!(s.live(Souls).base, Some((seq(5), d(1))));
    }

    #[test]
    fn a_retraction_must_withdraw_something_in_its_own_profile() {
        let base = vec![
            commit(1, vec![created(P, None), created(Q, None)]),
            commit(2, vec![souls(P, 1, Complete)]),
        ];
        let mut other_profile = base.clone();
        other_profile.push(commit(3, vec![retracted(Q, 1)]));
        assert!(matches!(
            fold(&other_profile),
            Err(FoldError::RetractsNothing { seq: s, .. }) if s == seq(3)
        ));
        let mut twice = base;
        twice.push(commit(3, vec![retracted(P, 1)]));
        twice.push(commit(4, vec![retracted(P, 1)]));
        assert!(matches!(
            fold(&twice),
            Err(FoldError::RetractsNothing { seq: s, .. }) if s == seq(4)
        ));
    }

    #[test]
    fn held_is_complete_over_a_base_and_the_strongest_layer_without_one() {
        let over_base = LiveSection {
            base: Some((seq(1), d(1))),
            layers: vec![layer(2, 2, Unstated)],
        };
        assert_eq!(over_base.held(), Some(Complete));
        let layers_only = LiveSection {
            base: None,
            layers: vec![layer(1, 1, Unstated), layer(2, 2, Partial)],
        };
        assert_eq!(layers_only.held(), Some(Partial));
        assert_eq!(LiveSection::default().held(), None);
        assert_eq!(
            over_base
                .snapshots()
                .map(|(_, d, c)| (d, c))
                .collect::<Vec<_>>(),
            vec![(d(1), Complete), (d(2), Unstated)]
        );
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
