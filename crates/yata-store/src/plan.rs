//! Instructions to one batch, and the batch's results back to the instructions' outputs.

use crate::instruction::Instruction;
use crate::interpret::StoreError;
use crate::statement::{Batch, BatchMode, StatementResult};

/// An instruction, or instructions that must land together, and the one transaction that
/// carries them out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan<I> {
    instruction: I,
    batch: Batch,
}

/// The plan for an instruction: one atomic batch, its statements in order.
pub fn plan<I: Instruction>(instruction: I) -> Plan<I> {
    let batch = Batch::new(instruction.statements(), BatchMode::Transaction);
    Plan { instruction, batch }
}

impl<I: Instruction> Plan<I> {
    pub fn batch(&self) -> &Batch {
        &self.batch
    }

    pub fn instruction(&self) -> &I {
        &self.instruction
    }

    /// Read the executor's results, one per statement of the batch.
    pub fn interpret(&self, results: &[StatementResult]) -> Result<I::Output, StoreError> {
        let expected = self.batch.statements().len();
        if results.len() != expected {
            return Err(StoreError::ResultCount {
                expected,
                actual: results.len(),
            });
        }
        self.instruction.interpret(results)
    }

    /// What the failure of the guarded statement at `statement` in the batch means; `None` if
    /// that statement has no guard.
    pub fn guard_error(&self, statement: usize) -> Option<StoreError> {
        self.instruction.guard_error(statement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{Digest, MetaKey, Seq, StoreId};
    use crate::interpret::{CacheEntry, Check, StoreKind};
    use crate::ops::*;
    use crate::schema::{APPLICATION_ID, CREATE_SCHEMA};
    use crate::statement::{Param, Value};

    fn seq(n: u64) -> Seq {
        Seq::new(n).expect("a seq")
    }

    fn sql_of<I: Instruction>(p: &Plan<I>) -> Vec<&'static str> {
        p.batch().statements().iter().map(|s| s.sql()).collect()
    }

    fn rows(rows: Vec<Vec<Value>>) -> StatementResult {
        StatementResult::Rows(rows)
    }

    fn changed(n: u64) -> StatementResult {
        StatementResult::Changed(n)
    }

    fn one<I: Instruction>(i: I, r: Vec<StatementResult>) -> Result<I::Output, StoreError> {
        plan(i).interpret(&r)
    }

    #[test]
    fn the_application_id_literal_is_the_constant() {
        assert!(SET_APPLICATION_ID.ends_with(&format!("= {APPLICATION_ID}")));
    }

    #[test]
    fn instructions_that_land_together_share_one_atomic_batch_and_answer_in_their_shape() {
        let p = plan((
            PutBlob {
                digest: Digest([1; 32]),
                bytes: vec![9],
            },
            AppendCommit {
                seq: seq(1),
                commit: vec![7],
            },
        ));
        assert_eq!(p.batch().mode(), BatchMode::Transaction);
        assert_eq!(p.batch().statements().len(), 2);
        assert_eq!(p.interpret(&[changed(1), changed(1)]), Ok(((), ())));
    }

    #[test]
    fn appending_binds_seq_and_bytes_and_guards_density() {
        let p = plan(AppendCommit {
            seq: seq(42),
            commit: vec![1, 2],
        });
        let s = &p.batch().statements()[0];
        assert!(s.sql().contains("max(seq), 0) + 1"));
        assert_eq!(s.must_change(), Some(1));
        assert_eq!(s.params(), &[Param::Integer(42), Param::Blob(vec![1, 2])]);
    }

    #[test]
    fn a_failed_append_guard_means_the_seq_was_not_next() {
        let p = plan((
            PutBlob {
                digest: Digest([1; 32]),
                bytes: vec![],
            },
            AppendCommit {
                seq: seq(9),
                commit: vec![],
            },
        ));
        assert_eq!(
            p.guard_error(1),
            Some(StoreError::SeqNotNext { seq: seq(9) })
        );
        assert_eq!(p.guard_error(0), None);
        let i = || AppendCommit {
            seq: seq(5),
            commit: vec![],
        };
        assert_eq!(one(i(), vec![changed(1)]), Ok(()));
        assert_eq!(
            one(i(), vec![changed(0)]),
            Err(StoreError::SeqNotNext { seq: seq(5) })
        );
    }

    #[test]
    fn initializing_creates_four_tables_then_writes_identity() {
        let p = plan(Initialize {
            store_id: StoreId([3; 16]),
            store_format_version: 1,
        });
        let sql = sql_of(&p);
        assert_eq!(
            sql.iter().filter(|q| q.starts_with("CREATE TABLE")).count(),
            4
        );
        assert_eq!(sql[CREATE_SCHEMA.len()], SET_APPLICATION_ID);
        assert_eq!(
            p.batch().statements()[CREATE_SCHEMA.len() + 1].params(),
            &[
                Param::Text("store_format_version"),
                Param::Blob(vec![0, 0, 0, 1])
            ]
        );
    }

    #[test]
    fn pruning_deletes_each_named_blob_and_counts_them() {
        let i = PruneBlobs {
            digests: vec![Digest([1; 32]), Digest([2; 32])],
        };
        assert_eq!(
            sql_of(&plan(i.clone())),
            vec!["DELETE FROM blobs WHERE digest = ?1"; 2]
        );
        assert_eq!(one(i, vec![changed(1), changed(0)]), Ok(1));
    }

    #[test]
    fn no_instruction_updates_or_deletes_a_commit() {
        let p = plan((
            (
                (
                    Inspect,
                    Initialize {
                        store_id: StoreId([0; 16]),
                        store_format_version: 1,
                    },
                ),
                (
                    (QuickCheck, IntegrityCheck),
                    (
                        ReadMeta {
                            key: MetaKey::StoreId,
                        },
                        LastSeq,
                    ),
                ),
            ),
            (
                (
                    AppendCommit {
                        seq: seq(1),
                        commit: vec![],
                    },
                    ReadCommits {
                        from: seq(1),
                        limit: 10,
                    },
                ),
                (
                    (
                        PutBlob {
                            digest: Digest([0; 32]),
                            bytes: vec![],
                        },
                        GetBlob {
                            digest: Digest([0; 32]),
                        },
                    ),
                    (
                        PruneBlobs {
                            digests: vec![Digest([0; 32])],
                        },
                        (
                            ReplaceCache {
                                seq: seq(1),
                                fold_version: 1,
                                projection: vec![],
                            },
                            ReadCache,
                        ),
                    ),
                ),
            ),
        ));
        for sql in sql_of(&p) {
            let upper = sql.to_ascii_uppercase();
            assert!(!upper.starts_with("UPDATE"), "{sql}");
            assert!(!upper.starts_with("DELETE FROM LOG"), "{sql}");
            assert!(!upper.starts_with("DROP"), "{sql}");
        }
    }

    #[test]
    fn inspect_tells_empty_yata_and_foreign_files_apart() {
        let kind = |id, n| {
            one(
                Inspect,
                vec![
                    rows(vec![vec![Value::Integer(id)]]),
                    rows(vec![vec![Value::Integer(n)]]),
                ],
            )
        };
        assert_eq!(kind(0, 0), Ok(StoreKind::Empty));
        assert_eq!(kind(APPLICATION_ID, 4), Ok(StoreKind::Yata));
        assert_eq!(
            kind(0, 3),
            Ok(StoreKind::Foreign {
                application_id: 0,
                objects: 3
            })
        );
    }

    #[test]
    fn commits_read_back_must_be_dense() {
        let row = |s: i64| vec![Value::Integer(s), Value::Blob(vec![])];
        let read = |r| {
            one(
                ReadCommits {
                    from: seq(3),
                    limit: 9,
                },
                vec![rows(r)],
            )
        };
        assert_eq!(read(vec![row(3), row(4)]).map(|c| c.len()), Ok(2));
        assert_eq!(
            read(vec![row(3), row(5)]),
            Err(StoreError::LogGap {
                expected: seq(4),
                found: 5
            })
        );
    }

    #[test]
    fn the_last_seq_of_an_empty_log_is_none() {
        assert_eq!(one(LastSeq, vec![rows(vec![vec![Value::Null]])]), Ok(None));
        assert_eq!(
            one(LastSeq, vec![rows(vec![vec![Value::Integer(7)]])]),
            Ok(Some(seq(7)))
        );
    }

    #[test]
    fn a_check_that_said_ok_has_no_problems() {
        let text = |t: &str| vec![Value::Text(t.to_owned())];
        assert_eq!(one(QuickCheck, vec![rows(vec![text("ok")])]), Ok(Check::Ok));
        assert_eq!(
            one(IntegrityCheck, vec![rows(vec![text("page 3: bad")])]),
            Ok(Check::Problems(vec!["page 3: bad".to_owned()]))
        );
    }

    #[test]
    fn missing_rows_read_as_absent() {
        assert_eq!(
            one(
                GetBlob {
                    digest: Digest([0; 32])
                },
                vec![rows(vec![])]
            ),
            Ok(None)
        );
        assert_eq!(one(ReadCache, vec![rows(vec![])]), Ok(None));
        assert_eq!(
            one(
                ReadCache,
                vec![rows(vec![vec![
                    Value::Integer(2),
                    Value::Integer(3),
                    Value::Blob(vec![1])
                ]])]
            ),
            Ok(Some(CacheEntry {
                seq: seq(2),
                fold_version: 3,
                projection: vec![1]
            }))
        );
    }

    #[test]
    fn a_result_of_the_wrong_shape_is_an_error() {
        use crate::interpret::Expected;
        assert_eq!(
            one(LastSeq, vec![rows(vec![vec![Value::Text("x".into())]])]),
            Err(StoreError::Shape {
                instruction: "LastSeq",
                expected: Expected::Integer
            })
        );
        assert_eq!(
            one(LastSeq, vec![changed(0)]),
            Err(StoreError::Shape {
                instruction: "LastSeq",
                expected: Expected::Rows
            })
        );
        assert_eq!(
            one(LastSeq, vec![]),
            Err(StoreError::ResultCount {
                expected: 1,
                actual: 0
            })
        );
    }
}
