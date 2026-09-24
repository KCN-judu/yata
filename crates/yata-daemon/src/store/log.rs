//! The fact log over the store: the authoritative history, and the projection folded from it
//! (ADR-0002).
//!
//! Opening replays every commit through the codec and the fold; nothing derived is read from
//! disk. A write applies its commit to the projection first, so a commit the fold would refuse
//! is never written, and a commit that would change nothing is not written at all. The commit
//! and any blob it names land in one transaction; the in-memory projection moves only after
//! that transaction commits.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use prost::Message;
use yata_core::fact::{
    Acquisition, AdmissionError, Channel, Commit, Digest, Fact, FactBody, FoldError, Inventory,
    InventoryError, Origin, ProbeVersion, ProfileId, Projection, Scope, SoulDefect, Source,
    admit_reading, check_soul,
};
use yata_core::import::observation::SoulReading;
use yata_protocol::probe;
use yata_store::{Instruction, Output};

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

impl LoadError {
    pub fn code(&self) -> &'static str {
        match self {
            LoadError::Store(_) => "store.failure",
            LoadError::Fact(e) => e.code(),
            LoadError::Fold(_) => "store.invalid_log",
        }
    }
}

/// What a write did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Committed {
    /// A commit landed at this `seq`, which is the new revision.
    At(u64),
    /// The facts would change nothing; nothing was written and the revision did not move.
    Unchanged,
}

/// Why a write was refused. Nothing was written, and the projection is as it was.
#[derive(Debug, Clone, PartialEq)]
pub enum CommitError {
    Refused(FoldError),
    Encode(EncodeError),
    Store(Failure),
}

impl CommitError {
    pub fn code(&self) -> &'static str {
        match self {
            CommitError::Refused(FoldError::ProfileMismatch { .. }) => "import.profile_mismatch",
            CommitError::Refused(_) => "command.refused",
            CommitError::Encode(_) => "command.too_large",
            CommitError::Store(_) => "store.failure",
        }
    }
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

impl IngestError {
    pub fn code(&self) -> &'static str {
        match self {
            IngestError::Convert(_) => "import.malformed_reading",
            IngestError::Refused(e) => e.code(),
            IngestError::Blob(_) => "import.reading_too_large",
            IngestError::Commit(e) => e.code(),
        }
    }
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
    /// The blob is not a `ReadResult`.
    NotAReadResult {
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
    pub seq: u64,
    pub digest: Digest,
    /// The reading's records that cannot be souls: kept as read, left out of the inventory.
    pub defects: Vec<SoulDefect>,
}

/// The store, and the projection of everything in its log.
pub struct FactLog {
    store: Store,
    projection: Projection,
}

/// Read every commit of the log, in order, decoded and lifted. Nothing is folded.
pub fn read_commits(store: &mut Store) -> Result<Vec<Commit>, LoadError> {
    let mut commits = Vec::new();
    loop {
        let from = commits.len() as u64 + 1;
        let page = match store
            .apply(vec![Instruction::ReadCommits { from, limit: PAGE }])
            .map_err(LoadError::Store)?
            .pop()
        {
            Some(Output::Commits(page)) => page,
            _ => return Err(LoadError::Store(super::shape("ReadCommits"))),
        };
        let done = page.len() < PAGE as usize;
        for (seq, bytes) in page {
            commits.push(decode_commit(seq, &bytes).map_err(LoadError::Fact)?);
        }
        if done {
            return Ok(commits);
        }
    }
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

    /// Append the facts as one commit at the next `seq`, if they change anything.
    pub fn commit(
        &mut self,
        origin: Origin,
        recorded_at_ms: i64,
        facts: Vec<Fact>,
    ) -> Result<Committed, CommitError> {
        self.land(origin, recorded_at_ms, facts, Vec::new())
    }

    /// Import one reading into a profile: the read result becomes a blob ([`blob_of`], which
    /// leaves out the session's request id), and one `SnapshotAcquired` commit records it. The
    /// same reading imported again, by pipe or by file, is a second observation of one blob.
    pub fn ingest(
        &mut self,
        profile: ProfileId,
        read_result: &probe::ReadResult,
        provenance: Provenance,
        origin: Origin,
        recorded_at_ms: i64,
    ) -> Result<Ingested, IngestError> {
        let reading = soul_reading(read_result).map_err(IngestError::Convert)?;
        let admitted = admit_reading(&reading).map_err(IngestError::Refused)?;
        let (digest, stored) = blob::seal(&blob_of(read_result)).map_err(IngestError::Blob)?;
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
        let put = Instruction::PutBlob {
            digest,
            bytes: stored,
        };
        let seq = match self
            .land(origin, recorded_at_ms, vec![fact], vec![put])
            .map_err(IngestError::Commit)?
        {
            Committed::At(seq) => seq,
            // An acquisition always adds to the projection.
            Committed::Unchanged => self.projection.revision(),
        };
        let defects = admitted
            .souls
            .into_iter()
            .zip(&reading.souls)
            .filter_map(|(soul, record)| {
                check_soul(&reading, record).err().map(|kind| SoulDefect {
                    soul,
                    observed_at: seq,
                    kind,
                })
            })
            .collect();
        Ok(Ingested {
            seq,
            digest,
            defects,
        })
    }

    /// A profile's current soul inventory, from the projection and its live readings.
    pub fn inventory(&mut self, profile: ProfileId) -> Result<Inventory, InventoryReadError> {
        let live = self
            .projection
            .profile(profile)
            .map(|s| s.live(Scope::Souls))
            .unwrap_or_default();
        let mut readings: BTreeMap<Digest, SoulReading> = BTreeMap::new();
        for digest in live.digests() {
            if readings.contains_key(digest) {
                continue;
            }
            let stored = match self
                .store
                .apply(vec![Instruction::GetBlob { digest: *digest }])
                .map_err(InventoryReadError::Store)?
                .pop()
            {
                Some(Output::Blob(Some(bytes))) => bytes,
                Some(Output::Blob(None)) => {
                    return Err(InventoryReadError::MissingBlob { digest: *digest });
                }
                _ => return Err(InventoryReadError::Store(super::shape("GetBlob"))),
            };
            let bytes = blob::open(digest, &stored).map_err(|error| InventoryReadError::Blob {
                digest: *digest,
                error,
            })?;
            let result = probe::ReadResult::decode(bytes.as_slice())
                .map_err(|_| InventoryReadError::NotAReadResult { digest: *digest })?;
            let reading = soul_reading(&result).map_err(|error| InventoryReadError::Reading {
                digest: *digest,
                error,
            })?;
            readings.insert(*digest, reading);
        }
        Inventory::derive(&self.projection, profile, &readings).map_err(InventoryReadError::Derive)
    }

    fn land(
        &mut self,
        origin: Origin,
        recorded_at_ms: i64,
        facts: Vec<Fact>,
        mut with: Vec<Instruction>,
    ) -> Result<Committed, CommitError> {
        let commit = Commit {
            seq: self.projection.revision() + 1,
            recorded_at_ms,
            origin,
            facts,
        };
        let next = self
            .projection
            .clone()
            .apply(&commit)
            .map_err(CommitError::Refused)?;
        if next.same_state(&self.projection) {
            return Ok(Committed::Unchanged);
        }
        let bytes = encode_commit(&commit).map_err(CommitError::Encode)?;
        with.push(Instruction::AppendCommit {
            seq: commit.seq,
            commit: bytes,
        });
        self.store.apply(with).map_err(CommitError::Store)?;
        self.projection = next;
        Ok(Committed::At(commit.seq))
    }
}

/// A commit in readable form, for `yata-daemon log`. Account ids are account-derived data and
/// are shown only as present.
pub fn format_commit(c: &Commit) -> String {
    let mut out = format!("#{} at {} ms, {:?}\n", c.seq, c.recorded_at_ms, c.origin);
    for f in &c.facts {
        let body = match &f.body {
            FactBody::ProfileCreated {
                display_name,
                account,
            } => format!(
                "ProfileCreated {display_name:?}{}",
                if account.is_some() {
                    ", account set"
                } else {
                    ""
                }
            ),
            FactBody::ProfileRenamed { display_name } => format!("ProfileRenamed {display_name:?}"),
            FactBody::ProfileRetired => "ProfileRetired".to_owned(),
            FactBody::ProfileRestored => "ProfileRestored".to_owned(),
            FactBody::SnapshotAcquired(a) => format!(
                "SnapshotAcquired {} {:?} {:?} via {:?}/{:?}, probe {} v{}.{}{}",
                hex(&a.digest),
                a.scope,
                a.coverage,
                a.channel,
                a.source,
                a.probe_build_id,
                a.probe_version.major,
                a.probe_version.minor,
                if a.observed_account.is_some() {
                    ", account set"
                } else {
                    ""
                }
            ),
            FactBody::SnapshotRetracted { digest, reason } => {
                format!("SnapshotRetracted {} {reason:?}", hex(digest))
            }
            FactBody::SoulMarked { soul, mark } => {
                format!("SoulMarked {} {mark:?}", soul.as_str())
            }
            FactBody::SoulNoted { soul, text } => format!("SoulNoted {} {text:?}", soul.as_str()),
        };
        let _ = writeln!(out, "  {} {body}", hex(&f.profile.0));
    }
    out
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

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
            let store =
                Store::create_or_open(&self.0.join("store.sqlite3"), || [7; 16]).expect("store");
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

    fn reading() -> probe::ReadResult {
        probe::ReadResult {
            request_id: 1,
            scope: probe::Scope::Souls.into(),
            coverage: probe::Coverage::Complete.into(),
            records: Some(probe::read_result::Records::Souls(probe::SoulRecords {
                souls: vec![],
            })),
            field_evidence: vec![probe::FieldEvidence {
                field: "SoulRecord.soul_id".into(),
                evidence: probe::Evidence::Established.into(),
                basis: "test".into(),
            }],
            ..probe::ReadResult::default()
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

    #[test]
    fn a_failed_transaction_keeps_neither_the_blob_nor_the_projection() {
        let dir = Scratch::new("rollback");
        let mut log = dir.log();
        log.commit(Origin::Maintenance, 0, vec![created()])
            .expect("commit");
        // A commit lands behind the projection's back, so the next append's seq is taken: the
        // blob is written inside the transaction and the append's guard then fails.
        let behind = encode_commit(&Commit {
            seq: 2,
            recorded_at_ms: 0,
            origin: Origin::Maintenance,
            facts: vec![Fact {
                profile: P,
                body: FactBody::SoulMarked {
                    soul: GameSoulId::new("a").expect("id"),
                    mark: Some(Mark::Keep),
                },
            }],
        })
        .expect("encodes");
        log.store
            .apply(vec![Instruction::AppendCommit {
                seq: 2,
                commit: behind,
            }])
            .expect("append");
        let before = log.projection().clone();
        let e = log
            .ingest(P, &reading(), provenance(), Origin::Job { job_id: 1 }, 0)
            .expect_err("seq 2 is taken");
        assert_eq!(e.code(), "store.failure");
        assert_eq!(log.projection(), &before);
        let digest = blob::digest_of(&blob_of(&reading()));
        assert_eq!(
            log.store.apply(vec![Instruction::GetBlob { digest }]),
            Ok(vec![Output::Blob(None)])
        );
        // The log on disk is still the two commits, and replaying it sees the mark.
        let reopened = FactLog::open(log.into_store()).expect("replays");
        assert_eq!(reopened.projection().revision(), 2);
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
            log.commit(Origin::Maintenance, 0, vec![orphan]),
            Err(CommitError::Refused(FoldError::UnknownProfile { .. }))
        ));
        assert_eq!(read_commits(&mut log.store), Ok(vec![]));
    }

    #[test]
    fn a_malformed_commit_in_the_log_refuses_the_store() {
        let dir = Scratch::new("malformed");
        let mut store = dir.log().into_store();
        store
            .apply(vec![Instruction::AppendCommit {
                seq: 1,
                commit: vec![0xFF, 0x01],
            }])
            .expect("append");
        let e = FactLog::open(store).err().expect("refused");
        assert_eq!(e.code(), "store.malformed_commit");
        assert!(matches!(
            e,
            LoadError::Fact(FactError::Malformed { seq: 1, .. })
        ));
    }

    #[test]
    fn a_log_the_fold_refuses_refuses_the_store() {
        let dir = Scratch::new("invalid");
        let mut store = dir.log().into_store();
        let orphan = encode_commit(&Commit {
            seq: 1,
            recorded_at_ms: 0,
            origin: Origin::Maintenance,
            facts: vec![Fact {
                profile: P,
                body: FactBody::ProfileRetired,
            }],
        })
        .expect("encodes");
        store
            .apply(vec![Instruction::AppendCommit {
                seq: 1,
                commit: orphan,
            }])
            .expect("append");
        let e = FactLog::open(store).err().expect("refused");
        assert_eq!(e.code(), "store.invalid_log");
    }

    #[test]
    fn the_dump_shows_every_fact_and_hides_account_ids() {
        let c = Commit {
            seq: 1,
            recorded_at_ms: 5,
            origin: Origin::Command { request_id: 2 },
            facts: vec![Fact {
                profile: P,
                body: FactBody::ProfileCreated {
                    display_name: "main".into(),
                    account: Some(yata_core::fact::GameAccountId::new("secret").expect("id")),
                },
            }],
        };
        let text = format_commit(&c);
        assert!(text.starts_with("#1 at 5 ms, Command { request_id: 2 }\n"));
        assert!(
            text.contains("01010101010101010101010101010101 ProfileCreated \"main\", account set")
        );
        assert!(!text.contains("secret"));
    }
}
