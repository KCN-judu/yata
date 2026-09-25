//! Each instruction: its statements and the output it reads back.
//!
//! Nothing here updates or deletes a commit (ADR-0019, rule 5). Pruning named blobs is the one
//! deletion, and replacing the cache clears the one derived table.

use crate::instruction::{Digest, Instruction, MetaKey, Sealed, Seq, StoreId};
use crate::interpret::{
    CacheEntry, Check, Expected, StoreError, StoreKind, blob, changed, exactly, integer,
    optional_blob, optional_row, rows, scalar, seq, shape, text, unsigned,
};
use crate::schema::{APPLICATION_ID, CREATE_SCHEMA};
use crate::statement::{Param, Statement, StatementResult};

// PRAGMA takes no bound parameters, so the id is a literal; a test ties it to APPLICATION_ID.
pub(crate) const SET_APPLICATION_ID: &str = "PRAGMA application_id = 1497453633";

macro_rules! instruction {
    ($t:ty) => {
        impl Sealed for $t {}
    };
}

/// Whether the file is empty, a Yata store, or something else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inspect;
instruction!(Inspect);

impl Instruction for Inspect {
    type Output = StoreKind;

    fn statements(&self) -> Vec<Statement> {
        vec![
            Statement::new("PRAGMA application_id", vec![]),
            Statement::new("SELECT count(*) FROM sqlite_schema", vec![]),
        ]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<StoreKind, StoreError> {
        let [id, objects] = exactly(results)?;
        let id = integer(scalar(id, "Inspect")?, "Inspect")?;
        let objects = integer(scalar(objects, "Inspect")?, "Inspect")?;
        Ok(match (id, objects) {
            (APPLICATION_ID, _) => StoreKind::Yata,
            (0, 0) => StoreKind::Empty,
            (application_id, objects) => StoreKind::Foreign {
                application_id,
                objects,
            },
        })
    }
}

/// Create the tables of an empty file and write its identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Initialize {
    pub store_id: StoreId,
    pub store_format_version: u32,
}
instruction!(Initialize);

impl Instruction for Initialize {
    type Output = ();

    fn statements(&self) -> Vec<Statement> {
        let mut out: Vec<Statement> = CREATE_SCHEMA
            .iter()
            .map(|sql| Statement::new(sql, vec![]))
            .collect();
        out.push(Statement::new(SET_APPLICATION_ID, vec![]));
        out.push(meta_insert(
            MetaKey::StoreFormatVersion,
            self.store_format_version.to_be_bytes().to_vec(),
        ));
        out.push(meta_insert(MetaKey::StoreId, self.store_id.0.to_vec()));
        out
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<(), StoreError> {
        expect_count(results, CREATE_SCHEMA.len() + 3)
    }
}

fn meta_insert(key: MetaKey, value: Vec<u8>) -> Statement {
    Statement::new(
        "INSERT INTO meta (key, value) VALUES (?1, ?2)",
        vec![Param::Text(key.as_str()), Param::Blob(value)],
    )
}

fn expect_count(results: &[StatementResult], n: usize) -> Result<(), StoreError> {
    if results.len() == n {
        Ok(())
    } else {
        Err(StoreError::ResultCount {
            expected: n,
            actual: results.len(),
        })
    }
}

/// SQLite's fast structural check, run when the store opens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickCheck;
instruction!(QuickCheck);

/// SQLite's full check, run before a backup or a compaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityCheck;
instruction!(IntegrityCheck);

fn check(results: &[StatementResult], name: &'static str) -> Result<Check, StoreError> {
    let [r] = exactly(results)?;
    let lines: Vec<String> = rows(r, name)?
        .iter()
        .map(|row| match row.as_slice() {
            [v] => text(v, name).map(str::to_owned),
            _ => Err(shape(name, Expected::Columns(1))),
        })
        .collect::<Result<_, _>>()?;
    Ok(if lines == ["ok"] {
        Check::Ok
    } else {
        Check::Problems(lines)
    })
}

impl Instruction for QuickCheck {
    type Output = Check;

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new("PRAGMA quick_check", vec![])]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Check, StoreError> {
        check(results, "QuickCheck")
    }
}

impl Instruction for IntegrityCheck {
    type Output = Check;

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new("PRAGMA integrity_check", vec![])]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Check, StoreError> {
        check(results, "IntegrityCheck")
    }
}

/// One `meta` value; absent if never written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadMeta {
    pub key: MetaKey,
}
instruction!(ReadMeta);

impl Instruction for ReadMeta {
    type Output = Option<Vec<u8>>;

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new(
            "SELECT value FROM meta WHERE key = ?1",
            vec![Param::Text(self.key.as_str())],
        )]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Option<Vec<u8>>, StoreError> {
        let [r] = exactly(results)?;
        optional_blob(r, "ReadMeta")
    }
}

/// Append a commit at `seq`, which must be the last `seq` plus one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendCommit {
    pub seq: Seq,
    pub commit: Vec<u8>,
}
instruction!(AppendCommit);

impl Instruction for AppendCommit {
    type Output = ();

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::guarded(
            // The row lands only at the last seq plus one, and the guard voids the whole batch
            // otherwise, so the log stays dense and no blob of a failed commit is kept.
            "INSERT INTO log (seq, commit_bytes) SELECT ?1, ?2 \
             WHERE ?1 = (SELECT coalesce(max(seq), 0) + 1 FROM log)",
            vec![self.seq.param(), Param::Blob(self.commit.clone())],
            1,
        )]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<(), StoreError> {
        let [r] = exactly(results)?;
        match changed(r, "AppendCommit")? {
            1 => Ok(()),
            _ => Err(StoreError::SeqNotNext { seq: self.seq }),
        }
    }

    fn guard_error(&self, _statement: usize) -> Option<StoreError> {
        Some(StoreError::SeqNotNext { seq: self.seq })
    }
}

/// The `seq` of the last commit; `None` for an empty log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastSeq;
instruction!(LastSeq);

impl Instruction for LastSeq {
    type Output = Option<Seq>;

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new("SELECT max(seq) FROM log", vec![])]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Option<Seq>, StoreError> {
        let [r] = exactly(results)?;
        match scalar(r, "LastSeq")? {
            crate::statement::Value::Null => Ok(None),
            v => seq(v, "LastSeq").map(Some),
        }
    }
}

/// Up to `limit` commits from `from` on, in order and checked to be dense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadCommits {
    pub from: Seq,
    pub limit: u32,
}
instruction!(ReadCommits);

impl Instruction for ReadCommits {
    type Output = Vec<(Seq, Vec<u8>)>;

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new(
            "SELECT seq, commit_bytes FROM log WHERE seq >= ?1 ORDER BY seq LIMIT ?2",
            vec![self.from.param(), Param::Integer(i64::from(self.limit))],
        )]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Self::Output, StoreError> {
        let [r] = exactly(results)?;
        let mut expected = Some(self.from);
        let mut commits = Vec::new();
        for row in rows(r, "ReadCommits")? {
            let [s, bytes] = row.as_slice() else {
                return Err(shape("ReadCommits", Expected::Columns(2)));
            };
            let found = unsigned(s, "ReadCommits")?;
            let Some(want) = expected.filter(|w| w.get() == found) else {
                return Err(StoreError::LogGap {
                    expected: expected.unwrap_or(self.from),
                    found,
                });
            };
            commits.push((want, blob(bytes, "ReadCommits")?));
            expected = want.next();
        }
        Ok(commits)
    }
}

/// Store a blob's bytes under its digest. Identical bytes are stored once, so a blob already
/// present is left as it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutBlob {
    pub digest: Digest,
    pub bytes: Vec<u8>,
}
instruction!(PutBlob);

impl Instruction for PutBlob {
    type Output = ();

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new(
            "INSERT OR IGNORE INTO blobs (digest, bytes) VALUES (?1, ?2)",
            vec![
                Param::Blob(self.digest.0.to_vec()),
                Param::Blob(self.bytes.clone()),
            ],
        )]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<(), StoreError> {
        let [r] = exactly(results)?;
        changed(r, "PutBlob").map(|_| ())
    }
}

/// A blob's bytes; absent if never stored or pruned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlob {
    pub digest: Digest,
}
instruction!(GetBlob);

impl Instruction for GetBlob {
    type Output = Option<Vec<u8>>;

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new(
            "SELECT bytes FROM blobs WHERE digest = ?1",
            vec![Param::Blob(self.digest.0.to_vec())],
        )]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Option<Vec<u8>>, StoreError> {
        let [r] = exactly(results)?;
        optional_blob(r, "GetBlob")
    }
}

/// Remove the named blobs' bytes (compaction), answering how many were removed. Always planned
/// together with the `BlobsPruned` commit that records it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PruneBlobs {
    pub digests: Vec<Digest>,
}
instruction!(PruneBlobs);

impl Instruction for PruneBlobs {
    type Output = u64;

    fn statements(&self) -> Vec<Statement> {
        self.digests
            .iter()
            .map(|d| {
                Statement::new(
                    "DELETE FROM blobs WHERE digest = ?1",
                    vec![Param::Blob(d.0.to_vec())],
                )
            })
            .collect()
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<u64, StoreError> {
        expect_count(results, self.digests.len())?;
        results.iter().map(|r| changed(r, "PruneBlobs")).sum()
    }
}

/// Replace the one projection cache entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaceCache {
    pub seq: Seq,
    pub fold_version: u32,
    pub projection: Vec<u8>,
}
instruction!(ReplaceCache);

impl Instruction for ReplaceCache {
    type Output = ();

    fn statements(&self) -> Vec<Statement> {
        vec![
            Statement::new("DELETE FROM projection_cache", vec![]),
            Statement::new(
                "INSERT INTO projection_cache (seq, fold_version, projection) VALUES (?1, ?2, ?3)",
                vec![
                    self.seq.param(),
                    Param::Integer(i64::from(self.fold_version)),
                    Param::Blob(self.projection.clone()),
                ],
            ),
        ]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<(), StoreError> {
        let [clear, insert] = exactly(results)?;
        changed(clear, "ReplaceCache")?;
        changed(insert, "ReplaceCache").map(|_| ())
    }
}

/// The projection cache entry, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadCache;
instruction!(ReadCache);

impl Instruction for ReadCache {
    type Output = Option<CacheEntry>;

    fn statements(&self) -> Vec<Statement> {
        vec![Statement::new(
            "SELECT seq, fold_version, projection FROM projection_cache ORDER BY seq DESC LIMIT 1",
            vec![],
        )]
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Option<CacheEntry>, StoreError> {
        let [r] = exactly(results)?;
        let Some(row) = optional_row(r, "ReadCache")? else {
            return Ok(None);
        };
        let [s, version, projection] = row else {
            return Err(shape("ReadCache", Expected::Columns(3)));
        };
        let fold_version = u32::try_from(integer(version, "ReadCache")?)
            .map_err(|_| shape("ReadCache", Expected::U32))?;
        Ok(Some(CacheEntry {
            seq: seq(s, "ReadCache")?,
            fold_version,
            projection: blob(projection, "ReadCache")?,
        }))
    }
}
