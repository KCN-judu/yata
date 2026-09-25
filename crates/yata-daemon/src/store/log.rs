//! The fact log over the store: the authoritative history, and the projection folded from it
//! (ADR-0002).
//!
//! Opening replays every commit through the codec and the fold; nothing derived is read from
//! disk. A write applies its commit to the projection first, so a commit the fold would refuse
//! is never written. The commit and any blob it names land in one transaction; the in-memory
//! projection moves only after that transaction commits.
//!
//! Two kinds of write, with two result types:
//! - a command ([`FactLog::command`]) carries the revision it was formed against, and is
//!   [`CommandOutcome::Stale`] when that is not the current one, [`CommandOutcome::Unchanged`]
//!   when it would change nothing (no commit is written), or [`CommandOutcome::Applied`];
//! - an import ([`FactLog::ingest`]) is a job with no base, and always lands: an acquisition is
//!   an observation, and the log records every one.

use std::collections::BTreeMap;

use prost::Message;
use yata_core::fact::{
    Acquisition, AdmissionError, Channel, Commit, Digest, Fact, FactBody, Facts, FoldError,
    Inventory, InventoryError, Origin, ProbeVersion, ProfileId, Projection, RecordDefect, Revision,
    Scope, Seq, Source, admit_reading,
};
use yata_core::import::observation::SoulReading;
use yata_protocol::probe;
use yata_store::{AppendCommit, GetBlob, Instruction, PutBlob, ReadCommits};

use super::blob::{self, BlobError};
use super::fact::{EncodeError, FactError, decode_commit, encode_commit};
use super::{Failure, Store};
use crate::probe::convert::{ConvertError, blob_of, soul_reading};

/// Commits read per round trip while replaying.
const PAGE: u32 = 256;

/// Why the log could not be read or replayed. Each variant names what failed.
#[derive(Debug, Clone, PartialEq)]
pub enum LoadError {
    Store(Failure),
    Fact(FactError),
    Fold(FoldError),
}

/// What a command did (`core-protocol.md`, § Commands). A command maps to at most one commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOutcome {
    /// One commit landed; `revision` is its `seq`.
    Applied { revision: Revision },
    /// The facts would change nothing: no commit was written and the revision did not move.
    Unchanged { revision: Revision },
    /// `command.stale_revision`: the command was formed against `base`, not `current`. Nothing
    /// was applied.
    Stale { base: Revision, current: Revision },
}

/// Why a write was refused. Nothing was written, and the projection is as it was.
#[derive(Debug, Clone, PartialEq)]
pub enum CommitError {
    Refused(FoldError),
    Encode(EncodeError),
    /// The log is full: the next `seq` does not fit the store.
    LogFull,
    Store(Failure),
}

/// Why a reading could not be imported. Nothing was written.
#[derive(Debug, Clone, PartialEq)]
pub enum IngestError {
    /// The read result is not a soul reading.
    Convert(ConvertError),
    /// The reading is not one the fact log admits.
    Refused(AdmissionError),
    Blob(BlobError),
    Commit(CommitError),
}

/// Why an inventory could not be derived. Each is a damaged or inconsistent store.
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryReadError {
    Store(Failure),
    MissingBlob {
        digest: Digest,
    },
    Blob {
        digest: Digest,
        error: BlobError,
    },
    /// The blob is not a `Reading`.
    NotAReading {
        digest: Digest,
    },
    Reading {
        digest: Digest,
        error: ConvertError,
    },
    Derive(InventoryError),
}

/// Where a reading came from, as the probe's handshake or export file states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub channel: Channel,
    pub source: Source,
    pub probe_build_id: String,
    pub probe_version: ProbeVersion,
}

/// A landed import.
#[derive(Debug, Clone, PartialEq)]
pub struct Ingested {
    pub seq: Seq,
    pub digest: Digest,
    /// The reading's records that cannot be souls, by their position in the reading: kept as
    /// read, left out of the inventory.
    pub defects: Vec<RecordDefect>,
}

/// The store, and the projection of everything in its log.
pub struct FactLog {
    store: Store,
    projection: Projection,
}

fn store_digest(d: &Digest) -> yata_store::Digest {
    yata_store::Digest(d.0)
}

/// Read every commit of the log, in order, decoded and lifted. Nothing is folded.
pub fn read_commits(store: &mut Store) -> Result<Vec<Commit>, LoadError> {
    let mut commits = Vec::new();
    let mut from = Some(yata_store::Seq::FIRST);
    while let Some(start) = from {
        let page = store
            .apply(ReadCommits {
                from: start,
                limit: PAGE,
            })
            .map_err(LoadError::Store)?;
        from = match page.last() {
            Some((last, _)) if page.len() == PAGE as usize => last.next(),
            _ => None,
        };
        for (key, bytes) in page {
            let seq = Seq::from(key.get_nonzero());
            commits.push(decode_commit(seq, &bytes).map_err(LoadError::Fact)?);
        }
    }
    Ok(commits)
}

impl FactLog {
    /// Replay the store's log into a projection.
    pub fn open(mut store: Store) -> Result<FactLog, LoadError> {
        let commits = read_commits(&mut store)?;
        let projection = yata_core::fact::fold(&commits).map_err(LoadError::Fold)?;
        Ok(FactLog { store, projection })
    }

    pub fn projection(&self) -> &Projection {
        &self.projection
    }

    /// Give the store back, dropping the projection.
    pub fn into_store(self) -> Store {
        self.store
    }

    /// Apply a command formed against `base`: its facts as one commit at the next `seq`, if the
    /// base is current and the facts change anything.
    pub fn command(
        &mut self,
        base: Revision,
        origin: Origin,
        recorded_at_ms: i64,
        facts: Facts,
    ) -> Result<CommandOutcome, CommitError> {
        let current = self.projection.revision();
        if base != current {
            return Ok(CommandOutcome::Stale { base, current });
        }
        let (commit, next) = self.next_commit(origin, recorded_at_ms, facts)?;
        if next.same_state(&self.projection) {
            return Ok(CommandOutcome::Unchanged { revision: current });
        }
        let seq = self.land(commit, next, Vec::<PutBlob>::new())?;
        Ok(CommandOutcome::Applied {
            revision: Revision::at(seq),
        })
    }

    /// Import one reading into a profile: the reading becomes a blob ([`blob_of`]), and one
    /// `SnapshotAcquired` commit records it. The same reading imported again, by pipe or by
    /// file, is a second observation of one blob.
    pub fn ingest(
        &mut self,
        profile: ProfileId,
        wire: &probe::Reading,
        provenance: Provenance,
        origin: Origin,
        recorded_at_ms: i64,
    ) -> Result<Ingested, IngestError> {
        let reading = soul_reading(wire).map_err(IngestError::Convert)?;
        let admitted = admit_reading(&reading).map_err(IngestError::Refused)?;
        let (digest, stored) = blob::seal(&blob_of(wire)).map_err(IngestError::Blob)?;
        let fact = Fact {
            profile,
            body: FactBody::SnapshotAcquired(Acquisition {
                digest,
                scope: Scope::Souls,
                coverage: admitted.coverage,
                channel: provenance.channel,
                source: provenance.source,
                probe_build_id: provenance.probe_build_id,
                probe_version: provenance.probe_version,
                observed_account: admitted.account,
            }),
        };
        let (commit, next) = self
            .next_commit(origin, recorded_at_ms, Facts::one(fact))
            .map_err(IngestError::Commit)?;
        let put = PutBlob {
            digest: store_digest(&digest),
            bytes: stored,
        };
        let seq = self.land(commit, next, put).map_err(IngestError::Commit)?;
        Ok(Ingested {
            seq,
            digest,
            defects: admitted.defects,
        })
    }

    /// A profile's current soul inventory, from the projection and its live readings.
    pub fn inventory(&mut self, profile: ProfileId) -> Result<Inventory, InventoryReadError> {
        let live = self
            .projection
            .profile(profile)
            .ok_or(InventoryReadError::Derive(InventoryError::UnknownProfile {
                profile,
            }))?
            .live(Scope::Souls);
        let mut readings: BTreeMap<Digest, SoulReading> = BTreeMap::new();
        for digest in live.digests() {
            if readings.contains_key(digest) {
                continue;
            }
            let stored = self
                .store
                .apply(GetBlob {
                    digest: store_digest(digest),
                })
                .map_err(InventoryReadError::Store)?
                .ok_or(InventoryReadError::MissingBlob { digest: *digest })?;
            let bytes = blob::open(digest, &stored).map_err(|error| InventoryReadError::Blob {
                digest: *digest,
                error,
            })?;
            let wire = probe::Reading::decode(bytes.as_slice())
                .map_err(|_| InventoryReadError::NotAReading { digest: *digest })?;
            let reading = soul_reading(&wire).map_err(|error| InventoryReadError::Reading {
                digest: *digest,
                error,
            })?;
            readings.insert(*digest, reading);
        }
        Inventory::derive(&self.projection, profile, &readings).map_err(InventoryReadError::Derive)
    }

    /// The next commit of these facts, and the projection it would leave.
    fn next_commit(
        &self,
        origin: Origin,
        recorded_at_ms: i64,
        facts: Facts,
    ) -> Result<(Commit, Projection), CommitError> {
        let seq = self
            .projection
            .revision()
            .next()
            .ok_or(CommitError::LogFull)?;
        let commit = Commit {
            seq,
            recorded_at_ms,
            origin,
            facts,
        };
        let next = self
            .projection
            .clone()
            .apply(&commit)
            .map_err(CommitError::Refused)?;
        Ok((commit, next))
    }

    /// Write the commit, with the instructions that must land with it, and move the projection.
    fn land<W: Instruction>(
        &mut self,
        commit: Commit,
        next: Projection,
        with: W,
    ) -> Result<Seq, CommitError> {
        let bytes = encode_commit(&commit).map_err(CommitError::Encode)?;
        let seq = yata_store::Seq::new(commit.seq.get()).ok_or(CommitError::LogFull)?;
        self.store
            .apply((with, AppendCommit { seq, commit: bytes }))
            .map_err(CommitError::Store)?;
        self.projection = next;
        Ok(commit.seq)
    }
}

/// A commit in readable form, for `yata-daemon log`. Account ids are account-derived data and
/// are shown only as present.
pub fn format_commit(c: &Commit) -> String {
    let set = |present: bool| if present { ", account set" } else { "" };
    let mut out = format!(
        "#{} at {} ms, {:?}\n",
        c.seq.get(),
        c.recorded_at_ms,
        c.origin
    );
    for f in c.facts.as_slice() {
        let body = match &f.body {
            FactBody::ProfileCreated {
                display_name,
                account,
            } => format!("ProfileCreated {display_name:?}{}", set(account.is_some())),
            FactBody::ProfileRenamed { display_name } => format!("ProfileRenamed {display_name:?}"),
            FactBody::ProfileRetired => "ProfileRetired".to_owned(),
            FactBody::ProfileRestored => "ProfileRestored".to_owned(),
            FactBody::SnapshotAcquired(a) => format!(
                "SnapshotAcquired {} {:?} {:?} via {:?}/{:?}, probe {} v{}.{}{}",
                hex(&a.digest.0),
                a.scope,
                a.coverage,
                a.channel,
                a.source,
                a.probe_build_id,
                a.probe_version.major,
                a.probe_version.minor,
                set(a.observed_account.is_some())
            ),
            FactBody::SnapshotRetracted { digest, reason } => {
                format!("SnapshotRetracted {} {reason:?}", hex(&digest.0))
            }
            FactBody::SoulMarked { soul, mark } => {
                format!("SoulMarked {} {mark:?}", soul.as_str())
            }
            FactBody::SoulNoted { soul, note } => match note {
                Some(text) => format!("SoulNoted {} {:?}", soul.as_str(), text.as_str()),
                None => format!("SoulNoted {} cleared", soul.as_str()),
            },
        };
        out.push_str(&format!("  {} {body}\n", hex(&f.profile.0)));
    }
    out
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::wire::Code as _;

    use yata_core::fact::{GameSoulId, Mark};

    use super::*;

    const P: ProfileId = ProfileId([1; 16]);

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Scratch {
            let dir = std::env::temp_dir()
                .join(format!("yata-unit-{}-{name}", std::process::id()))
                .join("数据 目录");
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("temp dir");
            Scratch(dir)
        }

        fn log(&self) -> FactLog {
            let store = Store::create_or_open(&self.0.join("store.sqlite3"), || {
                yata_store::StoreId([7; 16])
            })
            .expect("store");
            FactLog::open(store).expect("log")
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            if let Some(parent) = self.0.parent() {
                let _ = std::fs::remove_dir_all(parent);
            }
        }
    }

    fn reading() -> probe::Reading {
        let established = || probe::Mapping {
            evidence: Some(probe::mapping::Evidence::Established(probe::Established {
                basis: "test".into(),
            })),
        };
        probe::Reading {
            coverage: probe::Coverage::Complete.into(),
            records: Some(probe::reading::Records::Souls(probe::SoulRecords {
                souls: vec![],
                recognition: None,
                mappings: Some(probe::SoulMappings {
                    soul_id: Some(established()),
                    ..probe::SoulMappings::default()
                }),
            })),
            ..probe::Reading::default()
        }
    }

    fn provenance() -> Provenance {
        Provenance {
            channel: Channel::MumuAdb,
            source: Source::ExportFile,
            probe_build_id: "t".into(),
            probe_version: ProbeVersion { major: 1, minor: 0 },
        }
    }

    fn created() -> Fact {
        Fact {
            profile: P,
            body: FactBody::ProfileCreated {
                display_name: "main".into(),
                account: None,
            },
        }
    }

    fn seq(n: u64) -> Seq {
        Seq::new(n).expect("a seq")
    }

    fn store_seq(n: u64) -> yata_store::Seq {
        yata_store::Seq::new(n).expect("a seq")
    }

    #[test]
    fn a_failed_transaction_keeps_neither_the_blob_nor_the_projection() {
        let dir = Scratch::new("rollback");
        let mut log = dir.log();
        log.command(
            Revision::EMPTY,
            Origin::Maintenance,
            0,
            Facts::one(created()),
        )
        .expect("commit");
        // A commit lands behind the projection's back, so the next append's seq is taken: the
        // blob is written inside the transaction and the append's guard then fails.
        let behind = encode_commit(&Commit {
            seq: seq(2),
            recorded_at_ms: 0,
            origin: Origin::Maintenance,
            facts: Facts::one(Fact {
                profile: P,
                body: FactBody::SoulMarked {
                    soul: GameSoulId::new("a").expect("id"),
                    mark: Some(Mark::Keep),
                },
            }),
        })
        .expect("encodes");
        log.store
            .apply(AppendCommit {
                seq: store_seq(2),
                commit: behind,
            })
            .expect("append");
        let before = log.projection().clone();
        let e = log
            .ingest(P, &reading(), provenance(), Origin::Job { job_id: 1 }, 0)
            .expect_err("seq 2 is taken");
        assert_eq!(crate::wire::ingest_failure(&e).code(), "store.failure");
        assert_eq!(log.projection(), &before);
        let digest = store_digest(&blob::digest_of(&blob_of(&reading())));
        assert_eq!(log.store.apply(GetBlob { digest }), Ok(None));
        // The log on disk is still the two commits, and replaying it sees the mark.
        let reopened = FactLog::open(log.into_store()).expect("replays");
        assert_eq!(reopened.projection().revision(), Revision::at(seq(2)));
        assert_eq!(
            reopened
                .projection()
                .profile(P)
                .and_then(|s| s.mark(&GameSoulId::new("a").expect("id"))),
            Some(Mark::Keep)
        );
    }

    #[test]
    fn a_refused_commit_writes_nothing() {
        let dir = Scratch::new("refused");
        let mut log = dir.log();
        let orphan = Fact {
            profile: P,
            body: FactBody::ProfileRetired,
        };
        assert!(matches!(
            log.command(Revision::EMPTY, Origin::Maintenance, 0, Facts::one(orphan)),
            Err(CommitError::Refused(FoldError::UnknownProfile { .. }))
        ));
        assert_eq!(read_commits(&mut log.store), Ok(vec![]));
    }

    #[test]
    fn a_malformed_commit_in_the_log_refuses_the_store() {
        let dir = Scratch::new("malformed");
        let mut store = dir.log().into_store();
        store
            .apply(AppendCommit {
                seq: store_seq(1),
                commit: vec![0xFF, 0x01],
            })
            .expect("append");
        let e = FactLog::open(store).err().expect("refused");
        assert_eq!(
            crate::wire::load_failure(&e).code(),
            "store.malformed_commit"
        );
        assert_eq!(
            e,
            LoadError::Fact(FactError::Malformed {
                seq: seq(1),
                what: super::super::fact::Malformation::NotACommit
            })
        );
    }

    #[test]
    fn a_log_the_fold_refuses_refuses_the_store() {
        let dir = Scratch::new("invalid");
        let mut store = dir.log().into_store();
        let orphan = encode_commit(&Commit {
            seq: seq(1),
            recorded_at_ms: 0,
            origin: Origin::Maintenance,
            facts: Facts::one(Fact {
                profile: P,
                body: FactBody::ProfileRetired,
            }),
        })
        .expect("encodes");
        store
            .apply(AppendCommit {
                seq: store_seq(1),
                commit: orphan,
            })
            .expect("append");
        let e = FactLog::open(store).err().expect("refused");
        assert_eq!(crate::wire::load_failure(&e).code(), "store.invalid_log");
    }

    #[test]
    fn the_dump_shows_every_fact_and_hides_account_ids() {
        let c = Commit {
            seq: seq(1),
            recorded_at_ms: 5,
            origin: Origin::Command { request_id: 2 },
            facts: Facts::one(Fact {
                profile: P,
                body: FactBody::ProfileCreated {
                    display_name: "main".into(),
                    account: Some(yata_core::fact::GameAccountId::new("secret").expect("id")),
                },
            }),
        };
        let text = format_commit(&c);
        assert!(text.starts_with("#1 at 5 ms, Command { request_id: 2 }\n"));
        assert!(
            text.contains("01010101010101010101010101010101 ProfileCreated \"main\", account set")
        );
        assert!(!text.contains("secret"));
    }

    #[test]
    fn a_command_is_stale_unchanged_or_applied() {
        let dir = Scratch::new("outcome");
        let mut log = dir.log();
        let keep = || {
            Facts::one(Fact {
                profile: P,
                body: FactBody::SoulMarked {
                    soul: GameSoulId::new("a").expect("id"),
                    mark: Some(Mark::Keep),
                },
            })
        };
        let r1 = Revision::at(seq(1));
        let r2 = Revision::at(seq(2));
        assert_eq!(
            log.command(
                Revision::EMPTY,
                Origin::Maintenance,
                0,
                Facts::one(created())
            ),
            Ok(CommandOutcome::Applied { revision: r1 })
        );
        assert_eq!(
            log.command(Revision::EMPTY, Origin::Maintenance, 0, keep()),
            Ok(CommandOutcome::Stale {
                base: Revision::EMPTY,
                current: r1
            })
        );
        assert_eq!(
            log.command(r1, Origin::Maintenance, 0, keep()),
            Ok(CommandOutcome::Applied { revision: r2 })
        );
        assert_eq!(
            log.command(r2, Origin::Maintenance, 0, keep()),
            Ok(CommandOutcome::Unchanged { revision: r2 })
        );
        assert_eq!(read_commits(&mut log.store).map(|c| c.len()), Ok(2));
    }
}
