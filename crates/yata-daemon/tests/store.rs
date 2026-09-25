//! The store against a real SQLite file: every instruction of `yata-store` runs, the log stays
//! dense and append-only, a failed commit keeps none of its writes, and a path with Chinese
//! characters and a space works (ADR-0010).

use std::path::{Path, PathBuf};

use yata_daemon::store::{Failure, OpenError, STORE_FORMAT_VERSION, Store};
use yata_store::{
    AppendCommit, CacheEntry, Check, Digest, GetBlob, LastSeq, MetaKey, PruneBlobs, PutBlob,
    ReadCache, ReadCommits, ReadMeta, ReplaceCache, Seq, StoreError, StoreId,
};

/// A fresh directory under the system temp directory, with a non-ASCII name and a space.
struct Scratch(PathBuf);

impl Scratch {
    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn new(name: &str) -> Scratch {
        let dir = std::env::temp_dir()
            .join(format!("yata-test-{}-{name}", std::process::id()))
            .join("数据 目录");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        Scratch(dir)
    }

    fn store(&self) -> PathBuf {
        self.0.join("store.sqlite3")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Some(parent) = self.0.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }
    }
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn create(path: &Path) -> Store {
    Store::create_or_open(path, || StoreId([7; 16])).expect("create")
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn seq(n: u64) -> Seq {
    Seq::new(n).expect("a seq")
}

fn append(n: u64) -> AppendCommit {
    AppendCommit {
        seq: seq(n),
        commit: vec![n as u8],
    }
}

fn put(b: u8) -> PutBlob {
    PutBlob {
        digest: Digest([b; 32]),
        bytes: vec![b],
    }
}

#[test]
fn a_store_is_created_initialized_and_reopened() {
    let dir = Scratch::new("create");
    let mut s = create(&dir.store());
    assert_eq!(
        s.apply((
            ReadMeta {
                key: MetaKey::StoreFormatVersion
            },
            ReadMeta {
                key: MetaKey::StoreId
            },
        )),
        Ok((
            Some(STORE_FORMAT_VERSION.to_be_bytes().to_vec()),
            Some(vec![7; 16]),
        ))
    );
    drop(s);
    let mut reopened = Store::open(&dir.store()).expect("reopen");
    assert_eq!(reopened.integrity_check(), Ok(Check::Ok));
    assert_eq!(reopened.apply(LastSeq), Ok(None));
}

#[test]
fn opening_never_creates_a_missing_store() {
    let dir = Scratch::new("missing");
    assert!(matches!(
        Store::open(&dir.store()),
        Err(OpenError::Missing { .. })
    ));
    assert!(!dir.store().exists());
}

#[test]
fn commits_append_densely_and_read_back_in_order() {
    let dir = Scratch::new("dense");
    let mut s = create(&dir.store());
    s.apply(vec![append(1), append(2)]).expect("append");
    assert_eq!(
        s.apply(append(4)),
        Err(Failure::Store(StoreError::SeqNotNext { seq: seq(4) }))
    );
    assert_eq!(
        s.apply(append(2)),
        Err(Failure::Store(StoreError::SeqNotNext { seq: seq(2) }))
    );
    assert_eq!(
        s.apply(ReadCommits {
            from: seq(1),
            limit: 10
        }),
        Ok(vec![(seq(1), vec![1]), (seq(2), vec![2])])
    );
    assert_eq!(s.apply(LastSeq), Ok(Some(seq(2))));
}

#[test]
fn a_failed_commit_keeps_none_of_its_writes() {
    let dir = Scratch::new("atomic");
    let mut s = create(&dir.store());
    assert!(s.apply((put(9), append(5))).is_err());
    assert_eq!(
        s.apply(GetBlob {
            digest: Digest([9; 32])
        }),
        Ok(None)
    );
}

#[test]
fn blobs_are_stored_once_and_pruned_by_digest() {
    let dir = Scratch::new("blobs");
    let mut s = create(&dir.store());
    s.apply((vec![put(1), put(1), put(2)], append(1)))
        .expect("put");
    assert_eq!(
        s.apply((
            PruneBlobs {
                digests: vec![Digest([1; 32]), Digest([3; 32])]
            },
            append(2),
        )),
        Ok((1, ()))
    );
    assert_eq!(
        s.apply((
            GetBlob {
                digest: Digest([1; 32])
            },
            GetBlob {
                digest: Digest([2; 32])
            },
        )),
        Ok((None, Some(vec![2])))
    );
}

#[test]
fn the_projection_cache_keeps_one_entry() {
    let dir = Scratch::new("cache");
    let mut s = create(&dir.store());
    let replace = |n: u64| ReplaceCache {
        seq: seq(n),
        fold_version: 3,
        projection: vec![n as u8],
    };
    s.apply(replace(1)).expect("cache");
    s.apply(replace(2)).expect("cache");
    assert_eq!(
        s.apply(ReadCache),
        Ok(Some(CacheEntry {
            seq: seq(2),
            fold_version: 3,
            projection: vec![2]
        }))
    );
}

#[test]
fn a_foreign_file_is_refused_and_left_untouched() {
    let dir = Scratch::new("foreign");
    std::fs::write(dir.store(), b"not a database, just some text").expect("write");
    let before = std::fs::read(dir.store()).expect("read");
    assert_eq!(
        Store::open(&dir.store()).err(),
        Some(OpenError::NotADatabase)
    );
    assert_eq!(
        Store::create_or_open(&dir.store(), || StoreId([0; 16])).err(),
        Some(OpenError::NotADatabase)
    );
    assert_eq!(std::fs::read(dir.store()).expect("read"), before);
}

#[test]
fn an_existing_store_is_opened_without_a_new_id() {
    let dir = Scratch::new("existing");
    drop(create(&dir.store()));
    let reopened = Store::create_or_open(&dir.store(), || panic!("the store exists"));
    assert!(reopened.is_ok());
}

#[test]
fn an_empty_file_is_uninitialized_until_creation_is_asked_for() {
    let dir = Scratch::new("empty");
    std::fs::write(dir.store(), b"").expect("write");
    assert_eq!(
        Store::open(&dir.store()).err(),
        Some(OpenError::Uninitialized)
    );
    assert!(Store::create_or_open(&dir.store(), || StoreId([1; 16])).is_ok());
}
