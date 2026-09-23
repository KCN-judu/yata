//! Result rows to typed values.

use crate::instruction::Instruction;
use crate::plan::Plan;
use crate::schema::APPLICATION_ID;
use crate::statement::{StatementResult, Value};

/// What an opened file is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreKind {
    /// A new, empty file: initialize it.
    Empty,
    Yata,
    /// Something else. The daemon opens nothing and reports it.
    Foreign {
        application_id: i64,
        objects: i64,
    },
}

/// The typed result of one instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    Done,
    Kind(StoreKind),
    /// The problems SQLite's check reported; empty when it said "ok".
    Check(Vec<String>),
    Meta(Option<Vec<u8>>),
    LastSeq(u64),
    Commits(Vec<(u64, Vec<u8>)>),
    Blob(Option<Vec<u8>>),
    Pruned(u64),
    Cache(Option<CacheEntry>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry {
    pub seq: u64,
    pub fold_version: u32,
    pub projection: Vec<u8>,
}

/// Why a plan could not be made or its results could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    /// SQLite integers are signed 64-bit; a larger `seq` cannot be stored.
    SeqOutOfRange { seq: u64 },
    /// The commit was not appended because `seq` is not the last `seq` plus one.
    SeqNotNext { seq: u64 },
    /// The executor returned a different number of results than the plan has statements.
    ResultCount { expected: usize, actual: usize },
    /// A result did not have the shape its statement produces.
    Shape {
        instruction: &'static str,
        detail: &'static str,
    },
    /// Commits read back are not dense.
    LogGap { expected: u64, found: u64 },
    /// SQLite did not take a connection setting (`found` is what it reported).
    Setting {
        setting: &'static str,
        found: String,
    },
}

impl Plan {
    /// Read the executor's results, one per statement in the plan's batch, back into one output
    /// per instruction.
    pub fn interpret(&self, results: Vec<StatementResult>) -> Result<Vec<Output>, StoreError> {
        let expected = self.batch().statements().len();
        if results.len() != expected {
            return Err(StoreError::ResultCount {
                expected,
                actual: results.len(),
            });
        }
        self.instructions
            .iter()
            .zip(&self.spans)
            .map(|(i, span)| output(i, &results[span.clone()]))
            .collect()
    }
}

fn shape(instruction: &'static str, detail: &'static str) -> StoreError {
    StoreError::Shape {
        instruction,
        detail,
    }
}

/// The single value of a one-row, one-column result.
fn scalar<'a>(r: &'a StatementResult, name: &'static str) -> Result<&'a Value, StoreError> {
    match r.rows.as_slice() {
        [row] => match row.as_slice() {
            [v] => Ok(v),
            _ => Err(shape(name, "expected one column")),
        },
        _ => Err(shape(name, "expected one row")),
    }
}

fn integer(v: &Value, name: &'static str) -> Result<i64, StoreError> {
    match v {
        Value::Integer(i) => Ok(*i),
        _ => Err(shape(name, "expected an integer")),
    }
}

fn unsigned(v: &Value, name: &'static str) -> Result<u64, StoreError> {
    u64::try_from(integer(v, name)?).map_err(|_| shape(name, "expected a non-negative integer"))
}

fn blob(v: &Value, name: &'static str) -> Result<Vec<u8>, StoreError> {
    match v {
        Value::Blob(b) => Ok(b.clone()),
        _ => Err(shape(name, "expected bytes")),
    }
}

/// The value of a zero-or-one-row, one-column result.
fn optional_blob(r: &StatementResult, name: &'static str) -> Result<Option<Vec<u8>>, StoreError> {
    if r.rows.is_empty() {
        return Ok(None);
    }
    blob(scalar(r, name)?, name).map(Some)
}

fn output(instruction: &Instruction, results: &[StatementResult]) -> Result<Output, StoreError> {
    use Instruction::*;
    match instruction {
        Inspect => {
            let [id, objects] = results else {
                return Err(shape("Inspect", "expected two results"));
            };
            let id = integer(scalar(id, "Inspect")?, "Inspect")?;
            let objects = integer(scalar(objects, "Inspect")?, "Inspect")?;
            Ok(Output::Kind(match (id, objects) {
                (APPLICATION_ID, _) => StoreKind::Yata,
                (0, 0) => StoreKind::Empty,
                (application_id, objects) => StoreKind::Foreign {
                    application_id,
                    objects,
                },
            }))
        }
        QuickCheck | IntegrityCheck => {
            let [r] = results else {
                return Err(shape("Check", "expected one result"));
            };
            let lines: Vec<String> = r
                .rows
                .iter()
                .map(|row| match row.as_slice() {
                    [Value::Text(t)] => Ok(t.clone()),
                    _ => Err(shape("Check", "expected one text column")),
                })
                .collect::<Result<_, _>>()?;
            Ok(Output::Check(if lines == ["ok"] { vec![] } else { lines }))
        }
        ReadMeta { .. } => Ok(Output::Meta(optional_blob(only(results)?, "ReadMeta")?)),
        AppendCommit { seq, .. } => match only(results)?.changes {
            1 => Ok(Output::Done),
            _ => Err(StoreError::SeqNotNext { seq: *seq }),
        },
        LastSeq => Ok(Output::LastSeq(unsigned(
            scalar(only(results)?, "LastSeq")?,
            "LastSeq",
        )?)),
        ReadCommits { from, .. } => {
            let mut expected = *from;
            let mut commits = Vec::new();
            for row in &only(results)?.rows {
                let [seq, bytes] = row.as_slice() else {
                    return Err(shape("ReadCommits", "expected two columns"));
                };
                let seq = unsigned(seq, "ReadCommits")?;
                if seq != expected {
                    return Err(StoreError::LogGap {
                        expected,
                        found: seq,
                    });
                }
                commits.push((seq, blob(bytes, "ReadCommits")?));
                expected += 1;
            }
            Ok(Output::Commits(commits))
        }
        GetBlob { .. } => Ok(Output::Blob(optional_blob(only(results)?, "GetBlob")?)),
        PruneBlobs { .. } => Ok(Output::Pruned(results.iter().map(|r| r.changes).sum())),
        ReadCache => {
            let r = only(results)?;
            let Some(row) = r.rows.first() else {
                return Ok(Output::Cache(None));
            };
            let [seq, version, projection] = row.as_slice() else {
                return Err(shape("ReadCache", "expected three columns"));
            };
            let fold_version = u32::try_from(integer(version, "ReadCache")?)
                .map_err(|_| shape("ReadCache", "fold version out of range"))?;
            Ok(Output::Cache(Some(CacheEntry {
                seq: unsigned(seq, "ReadCache")?,
                fold_version,
                projection: blob(projection, "ReadCache")?,
            })))
        }
        Initialize { .. } | PutBlob { .. } | ReplaceCache { .. } => Ok(Output::Done),
    }
}

fn only(results: &[StatementResult]) -> Result<&StatementResult, StoreError> {
    match results {
        [r] => Ok(r),
        _ => Err(shape("instruction", "expected one result")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::plan;

    fn rows(rows: Vec<Vec<Value>>) -> StatementResult {
        StatementResult { rows, changes: 0 }
    }

    fn changed(n: u64) -> StatementResult {
        StatementResult {
            rows: vec![],
            changes: n,
        }
    }

    fn one(i: Instruction, r: Vec<StatementResult>) -> Result<Output, StoreError> {
        let p = plan(vec![i]).expect("plannable");
        p.interpret(r).map(|mut o| o.remove(0))
    }

    #[test]
    fn inspect_tells_empty_yata_and_foreign_files_apart() {
        let kind = |id, n| {
            one(
                Instruction::Inspect,
                vec![
                    rows(vec![vec![Value::Integer(id)]]),
                    rows(vec![vec![Value::Integer(n)]]),
                ],
            )
        };
        assert_eq!(kind(0, 0), Ok(Output::Kind(StoreKind::Empty)));
        assert_eq!(kind(APPLICATION_ID, 4), Ok(Output::Kind(StoreKind::Yata)));
        assert_eq!(
            kind(0, 3),
            Ok(Output::Kind(StoreKind::Foreign {
                application_id: 0,
                objects: 3
            }))
        );
    }

    #[test]
    fn an_append_that_changed_nothing_was_not_next() {
        let i = || Instruction::AppendCommit {
            seq: 5,
            commit: vec![],
        };
        assert_eq!(one(i(), vec![changed(1)]), Ok(Output::Done));
        assert_eq!(
            one(i(), vec![changed(0)]),
            Err(StoreError::SeqNotNext { seq: 5 })
        );
    }

    #[test]
    fn commits_read_back_must_be_dense() {
        let row = |s: i64| vec![Value::Integer(s), Value::Blob(vec![])];
        let read = |r| {
            one(
                Instruction::ReadCommits { from: 3, limit: 9 },
                vec![rows(r)],
            )
        };
        assert!(matches!(read(vec![row(3), row(4)]), Ok(Output::Commits(c)) if c.len() == 2));
        assert_eq!(
            read(vec![row(3), row(5)]),
            Err(StoreError::LogGap {
                expected: 4,
                found: 5
            })
        );
    }

    #[test]
    fn a_check_that_said_ok_has_no_problems() {
        let text = |t: &str| vec![Value::Text(t.to_owned())];
        assert_eq!(
            one(Instruction::QuickCheck, vec![rows(vec![text("ok")])]),
            Ok(Output::Check(vec![]))
        );
        assert_eq!(
            one(
                Instruction::IntegrityCheck,
                vec![rows(vec![text("page 3: bad")])]
            ),
            Ok(Output::Check(vec!["page 3: bad".to_owned()]))
        );
    }

    #[test]
    fn missing_rows_read_as_absent() {
        assert_eq!(
            one(Instruction::GetBlob { digest: [0; 32] }, vec![rows(vec![])]),
            Ok(Output::Blob(None))
        );
        assert_eq!(
            one(Instruction::ReadCache, vec![rows(vec![])]),
            Ok(Output::Cache(None))
        );
    }

    #[test]
    fn pruning_counts_the_blobs_removed() {
        let i = Instruction::PruneBlobs {
            digests: vec![[1; 32], [2; 32]],
        };
        assert_eq!(one(i, vec![changed(1), changed(0)]), Ok(Output::Pruned(1)));
    }

    #[test]
    fn a_result_of_the_wrong_shape_is_an_error() {
        assert!(matches!(
            one(
                Instruction::LastSeq,
                vec![rows(vec![vec![Value::Text("x".into())]])]
            ),
            Err(StoreError::Shape { .. })
        ));
        assert_eq!(
            one(Instruction::LastSeq, vec![]),
            Err(StoreError::ResultCount {
                expected: 1,
                actual: 0
            })
        );
    }
}
