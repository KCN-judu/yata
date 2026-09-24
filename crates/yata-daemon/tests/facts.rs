//! The fact log against a real SQLite file: readings imported as blobs and facts, the inventory
//! derived from them, and the whole projection rebuilt from the log alone (ADR-0002).

use std::path::PathBuf;

use prost::Message;
use yata_core::fact::{
    Channel, Fact, FactBody, GameAccountId, GameSoulId, Inventory, Mark, Origin, ProbeVersion,
    ProfileId, SoulDefectKind, Source,
};
use yata_core::import::observation::SoulField;
use yata_daemon::probe::convert::blob_of;
use yata_daemon::store::blob::digest_of;
use yata_daemon::store::{
    CommitError, Committed, FactLog, IngestError, Provenance, Store, read_commits,
};
use yata_protocol::probe;
use yata_store::{Instruction, Output};

const P: ProfileId = ProfileId([1; 16]);
const Q: ProfileId = ProfileId([2; 16]);

struct Scratch(PathBuf);

impl Scratch {
    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn new(name: &str) -> Scratch {
        let dir = std::env::temp_dir()
            .join(format!("yata-facts-{}-{name}", std::process::id()))
            .join("数据 目录");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        Scratch(dir)
    }

    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
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

fn soul(id: &str, level: u32) -> probe::SoulRecord {
    probe::SoulRecord {
        soul_id: Some(id.into()),
        suit_code: Some(30),
        star: Some(6),
        slot: Some(2),
        level: Some(level),
        main: Some(probe::AttributeValue {
            attribute_code: 7,
            value: 57.0,
        }),
        subs: vec![],
        innate: None,
        locked: Some(false),
        discarded: Some(false),
        observed: None,
    }
}

/// Evidence for every typed field of a soul record, all established.
fn established() -> Vec<probe::FieldEvidence> {
    SoulField::ALL
        .into_iter()
        .map(|f| probe::FieldEvidence {
            field: f.schema_name().into(),
            evidence: probe::Evidence::Established.into(),
            basis: "test".into(),
        })
        .collect()
}

fn reading(
    account: &str,
    coverage: probe::Coverage,
    souls: Vec<probe::SoulRecord>,
) -> probe::ReadResult {
    probe::ReadResult {
        request_id: 1,
        scope: probe::Scope::Souls.into(),
        coverage: coverage.into(),
        observed_account_id: account.into(),
        records: Some(probe::read_result::Records::Souls(probe::SoulRecords {
            souls,
        })),
        field_evidence: established(),
        ..probe::ReadResult::default()
    }
}

fn complete(account: &str, souls: Vec<probe::SoulRecord>) -> probe::ReadResult {
    reading(account, probe::Coverage::Complete, souls)
}

fn provenance() -> Provenance {
    Provenance {
        channel: Channel::MumuAdb,
        source: Source::ExportFile,
        probe_build_id: "test".into(),
        probe_version: ProbeVersion { major: 1, minor: 0 },
    }
}

fn fact(profile: ProfileId, body: FactBody) -> Fact {
    Fact { profile, body }
}

fn created(profile: ProfileId) -> Fact {
    fact(
        profile,
        FactBody::ProfileCreated {
            display_name: "main".into(),
            account: None,
        },
    )
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn id(s: &str) -> GameSoulId {
    GameSoulId::new(s).expect("non-empty")
}

fn mark(profile: ProfileId, soul: &str, mark: Option<Mark>) -> Fact {
    fact(
        profile,
        FactBody::SoulMarked {
            soul: id(soul),
            mark,
        },
    )
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn ingest(log: &mut FactLog, profile: ProfileId, result: &probe::ReadResult) -> u64 {
    log.ingest(profile, result, provenance(), Origin::Job { job_id: 1 }, 0)
        .expect("ingests")
        .seq
}

fn souls(inv: &Inventory) -> Vec<(String, u32)> {
    inv.souls()
        .map(|s| (s.id.as_str().to_owned(), s.record.level.unwrap_or_default()))
        .collect()
}

/// Two profiles, three readings, a retraction, marks and a note: every kind of fact.
#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn populate(log: &mut FactLog) {
    let at = |s| Origin::Command { request_id: s };
    log.commit(at(1), 1_000, vec![created(P), created(Q)])
        .expect("profiles");
    ingest(log, P, &complete("p", vec![soul("a", 12), soul("b", 3)]));
    ingest(
        log,
        P,
        &reading("p", probe::Coverage::Partial, vec![soul("b", 6)]),
    );
    let wrong = complete("p", vec![soul("z", 0)]);
    ingest(log, P, &wrong);
    log.commit(
        at(2),
        2_000,
        vec![fact(
            P,
            FactBody::SnapshotRetracted {
                digest: digest_of(&blob_of(&wrong)),
                reason: "test".into(),
            },
        )],
    )
    .expect("retract");
    log.commit(
        at(3),
        3_000,
        vec![
            mark(P, "a", Some(Mark::Keep)),
            fact(
                P,
                FactBody::SoulNoted {
                    soul: id("b"),
                    text: "双速".into(),
                },
            ),
        ],
    )
    .expect("decisions");
    ingest(log, Q, &complete("q", vec![soul("a", 15)]));
}

#[test]
fn the_projection_rebuilds_from_the_log_alone() {
    let dir = Scratch::new("rebuild");
    let mut log = dir.log();
    populate(&mut log);
    let projection = log.projection().clone();
    let (p, q) = (log.inventory(P).expect("P"), log.inventory(Q).expect("Q"));
    assert_eq!(souls(&p), vec![("a".into(), 12), ("b".into(), 6)]);
    assert_eq!(p.get(&id("a")).and_then(|s| s.mark), Some(Mark::Keep));
    assert_eq!(
        p.get(&id("b")).and_then(|s| s.note.clone()),
        Some("双速".into())
    );

    // Destroy every derived thing: the in-memory projection goes with the log, and a projection
    // cache entry that disagrees with the log is planted. Neither is consulted on replay.
    let mut store = log.into_store();
    store
        .apply(vec![Instruction::ReplaceCache {
            seq: projection.revision(),
            fold_version: 0,
            projection: b"not a projection".to_vec(),
        }])
        .expect("cache");
    drop(store);

    let mut rebuilt = dir.log();
    assert_eq!(rebuilt.projection(), &projection);
    assert_eq!(rebuilt.inventory(P), Ok(p));
    assert_eq!(rebuilt.inventory(Q), Ok(q));
}

#[test]
fn the_log_copied_into_another_store_folds_to_the_same_state() {
    let (a, b) = (Scratch::new("copy-a"), Scratch::new("copy-b"));
    let mut original = a.log();
    populate(&mut original);
    let expected = original.projection().clone();
    let mut source = original.into_store();
    let commits = read_commits(&mut source).expect("reads");

    // Replay the same facts through a second log, commit by commit, with blobs copied by digest.
    let mut copy = b.log();
    for c in commits {
        for f in &c.facts {
            if let FactBody::SnapshotAcquired(acq) = &f.body {
                let fetched = source.apply(vec![Instruction::GetBlob { digest: acq.digest }]);
                let Ok([Output::Blob(Some(stored))]) = fetched.as_deref() else {
                    panic!("blob present")
                };
                let bytes =
                    yata_daemon::store::blob::open(&acq.digest, stored).expect("blob opens");
                let result = probe::ReadResult::decode(bytes.as_slice()).expect("a read result");
                copy.ingest(
                    f.profile,
                    &result,
                    Provenance {
                        channel: acq.channel,
                        source: acq.source,
                        probe_build_id: acq.probe_build_id.clone(),
                        probe_version: acq.probe_version,
                    },
                    c.origin,
                    c.recorded_at_ms,
                )
                .expect("ingests");
            }
        }
        let others: Vec<Fact> = c
            .facts
            .into_iter()
            .filter(|f| !matches!(f.body, FactBody::SnapshotAcquired(_)))
            .collect();
        if !others.is_empty() {
            copy.commit(c.origin, c.recorded_at_ms, others)
                .expect("commits");
        }
    }
    assert_eq!(copy.projection(), &expected);
}

#[test]
fn a_repeated_import_is_a_second_observation_of_one_blob() {
    let dir = Scratch::new("repeat");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    let bytes = complete("p", vec![soul("a", 15)]);
    let first = log
        .ingest(P, &bytes, provenance(), Origin::Job { job_id: 1 }, 0)
        .expect("first");
    let after_first = log.inventory(P).expect("inventory");
    let second = log
        .ingest(P, &bytes, provenance(), Origin::Job { job_id: 2 }, 0)
        .expect("second");
    assert_eq!((first.seq, second.seq), (2, 3));
    assert_eq!(first.digest, second.digest);
    let state = log.projection().profile(P).expect("P");
    assert_eq!(state.acquisitions().len(), 2);
    let again = log.inventory(P).expect("inventory");
    assert_eq!(souls(&again), souls(&after_first));
    assert_eq!(again.get(&id("a")).map(|s| s.observed_at), Some(3));
}

#[test]
fn a_newer_reading_updates_removes_and_restores_souls() {
    let dir = Scratch::new("newer");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    ingest(
        &mut log,
        P,
        &complete("p", vec![soul("a", 12), soul("b", 0)]),
    );
    log.commit(
        Origin::Maintenance,
        0,
        vec![mark(P, "b", Some(Mark::Discard))],
    )
    .expect("mark");
    ingest(&mut log, P, &complete("p", vec![soul("a", 15)]));
    let inv = log.inventory(P).expect("inventory");
    assert_eq!(souls(&inv), vec![("a".into(), 15)]);
    ingest(
        &mut log,
        P,
        &complete("p", vec![soul("a", 15), soul("b", 0)]),
    );
    let inv = log.inventory(P).expect("inventory");
    assert_eq!(inv.get(&id("b")).and_then(|s| s.mark), Some(Mark::Discard));
}

#[test]
fn profiles_are_isolated_and_a_reading_never_crosses_accounts() {
    let dir = Scratch::new("profiles");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P), created(Q)])
        .expect("profiles");
    ingest(&mut log, P, &complete("p", vec![soul("a", 15)]));
    ingest(
        &mut log,
        Q,
        &complete("q", vec![soul("a", 3), soul("c", 0)]),
    );
    log.commit(
        Origin::Maintenance,
        0,
        vec![mark(Q, "a", Some(Mark::Strengthen))],
    )
    .expect("mark");
    let p = log.inventory(P).expect("P");
    assert_eq!(souls(&p), vec![("a".into(), 15)]);
    assert_eq!(p.get(&id("a")).and_then(|s| s.mark), None);

    let revision = log.projection().revision();
    let e = log
        .ingest(
            P,
            &complete("q", vec![]),
            provenance(),
            Origin::Job { job_id: 9 },
            0,
        )
        .expect_err("another account");
    assert_eq!(e.code(), "import.profile_mismatch");
    assert_eq!(log.projection().revision(), revision);
    assert_eq!(log.inventory(P), Ok(p));
    assert_eq!(
        log.projection()
            .profile(P)
            .and_then(|s| s.known_account())
            .map(GameAccountId::as_str),
        Some("p")
    );
}

#[test]
fn a_fact_that_changes_nothing_is_not_written() {
    let dir = Scratch::new("noop");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    let keep = || vec![mark(P, "a", Some(Mark::Keep))];
    assert_eq!(
        log.commit(Origin::Maintenance, 0, keep()),
        Ok(Committed::At(2))
    );
    assert_eq!(
        log.commit(Origin::Maintenance, 0, keep()),
        Ok(Committed::Unchanged)
    );
    assert_eq!(
        log.commit(
            Origin::Maintenance,
            0,
            vec![mark(P, "a", None), mark(P, "a", Some(Mark::Keep))]
        ),
        Ok(Committed::Unchanged)
    );
    assert_eq!(log.projection().revision(), 2);
    let mut store = log.into_store();
    assert_eq!(read_commits(&mut store).map(|c| c.len()), Ok(2));
}

#[test]
fn an_impossible_reading_is_refused_and_writes_nothing() {
    let dir = Scratch::new("impossible");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    let twice = complete("p", vec![soul("a", 15), soul("a", 12)]);
    let e = log
        .ingest(P, &twice, provenance(), Origin::Job { job_id: 1 }, 0)
        .expect_err("duplicate soul");
    assert_eq!(e.code(), "import.duplicate_soul");
    let unknown = complete("p", vec![soul("a", 15)]);
    assert!(matches!(
        log.ingest(Q, &unknown, provenance(), Origin::Job { job_id: 1 }, 0),
        Err(IngestError::Commit(CommitError::Refused(_)))
    ));
    assert_eq!(log.projection().revision(), 1);
    let mut store = log.into_store();
    for bytes in [&twice, &unknown] {
        let digest = digest_of(&blob_of(bytes));
        assert_eq!(
            store.apply(vec![Instruction::GetBlob { digest }]),
            Ok(vec![Output::Blob(None)])
        );
    }
}

#[test]
fn a_record_that_cannot_be_a_soul_is_kept_reported_and_left_out() {
    let dir = Scratch::new("defect");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    let mut bad = soul("b", 15);
    bad.star = Some(0);
    let landed = log
        .ingest(
            P,
            &complete("p", vec![soul("a", 15), bad]),
            provenance(),
            Origin::Job { job_id: 1 },
            0,
        )
        .expect("ingests");
    assert_eq!(landed.defects.len(), 1);
    assert_eq!(landed.defects[0].kind, SoulDefectKind::Star(0));
    let inv = log.inventory(P).expect("inventory");
    assert_eq!(souls(&inv), vec![("a".into(), 15)]);
    assert_eq!(inv.defects(), landed.defects.as_slice());
}

#[test]
fn a_reading_without_an_established_soul_id_is_refused_and_writes_nothing() {
    let dir = Scratch::new("unestablished");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    // What the reader sends today: records, and no typed field established.
    let mut today = complete("p", vec![soul("a", 15)]);
    today.field_evidence = vec![probe::FieldEvidence {
        field: "SoulRecord.soul_id".into(),
        evidence: probe::Evidence::Inherited.into(),
        basis: "prior tool".into(),
    }];
    let e = log
        .ingest(P, &today, provenance(), Origin::Job { job_id: 1 }, 0)
        .expect_err("unestablished");
    assert_eq!(e.code(), "import.unestablished_identity");
    today.field_evidence.clear();
    let e = log
        .ingest(P, &today, provenance(), Origin::Job { job_id: 2 }, 0)
        .expect_err("unmapped");
    assert_eq!(e.code(), "import.unestablished_identity");
    assert_eq!(log.projection().revision(), 1);
}

#[test]
fn a_record_whose_row_fields_are_not_established_is_reported_not_a_row() {
    let dir = Scratch::new("inherited-fields");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    let mut r = complete("p", vec![soul("a", 15)]);
    for e in &mut r.field_evidence {
        if e.field == "SoulRecord.suit_code" {
            e.evidence = probe::Evidence::Inherited.into();
        }
    }
    let landed = log
        .ingest(P, &r, provenance(), Origin::Job { job_id: 1 }, 0)
        .expect("ingests");
    assert_eq!(
        landed.defects[0].kind,
        SoulDefectKind::Unestablished(SoulField::SuitCode)
    );
    assert!(log.inventory(P).expect("inventory").is_empty());
}

#[test]
fn a_reading_by_pipe_and_by_file_is_one_blob() {
    let dir = Scratch::new("request-id");
    let mut log = dir.log();
    log.commit(Origin::Maintenance, 0, vec![created(P)])
        .expect("profile");
    let by_pipe = complete("p", vec![soul("a", 15)]);
    let by_file = probe::ReadResult {
        request_id: 0,
        ..by_pipe.clone()
    };
    let a = log
        .ingest(P, &by_pipe, provenance(), Origin::Job { job_id: 1 }, 0)
        .expect("pipe");
    let b = log
        .ingest(P, &by_file, provenance(), Origin::Job { job_id: 2 }, 0)
        .expect("file");
    assert_eq!(a.digest, b.digest);
}
