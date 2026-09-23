//! Instructions to SQL.

use std::ops::Range;

use crate::instruction::{Instruction, MetaKey};
use crate::interpret::StoreError;
use crate::schema::CREATE_SCHEMA;
use crate::statement::{Batch, Param, Statement};

// PRAGMA takes no bound parameters, so the id is a literal; a test ties it to APPLICATION_ID.
const SET_APPLICATION_ID: &str = "PRAGMA application_id = 1497453633";

impl Plan {
    /// The error a guarded statement's failure means, given its index in the batch.
    pub fn guard_error(&self, statement: usize) -> StoreError {
        let owner = self.spans.iter().position(|s| s.contains(&statement));
        match owner.map(|i| &self.instructions[i]) {
            Some(Instruction::AppendCommit { seq, .. }) => StoreError::SeqNotNext { seq: *seq },
            _ => StoreError::Shape {
                instruction: "guard",
                detail: "a statement without a guard reported a guard failure",
            },
        }
    }
}

/// The instructions, the one batch that carries them out, and which statements belong to which
/// instruction, so [`Plan::interpret`] can read the results back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) spans: Vec<Range<usize>>,
    batch: Batch,
}

impl Plan {
    pub fn batch(&self) -> &Batch {
        &self.batch
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}

/// The plan for instructions that must land together: one atomic batch, in order.
pub fn plan(instructions: Vec<Instruction>) -> Result<Plan, StoreError> {
    let mut statements = Vec::new();
    let mut spans = Vec::with_capacity(instructions.len());
    for i in &instructions {
        let start = statements.len();
        statements.extend(statements_for(i)?);
        spans.push(start..statements.len());
    }
    Ok(Plan {
        instructions,
        spans,
        batch: Batch::new(statements, true),
    })
}

fn seq_param(seq: u64) -> Result<Param, StoreError> {
    i64::try_from(seq)
        .map(Param::Integer)
        .map_err(|_| StoreError::SeqOutOfRange { seq })
}

fn statements_for(instruction: &Instruction) -> Result<Vec<Statement>, StoreError> {
    use Instruction::*;
    let one = |sql, params| Ok(vec![Statement::new(sql, params)]);
    match instruction {
        Inspect => Ok(vec![
            Statement::new("PRAGMA application_id", vec![]),
            Statement::new("SELECT count(*) FROM sqlite_schema", vec![]),
        ]),
        Initialize {
            store_id,
            store_format_version,
        } => {
            let mut out: Vec<Statement> = CREATE_SCHEMA
                .iter()
                .map(|sql| Statement::new(sql, vec![]))
                .collect();
            out.push(Statement::new(SET_APPLICATION_ID, vec![]));
            out.push(meta_insert(
                MetaKey::StoreFormatVersion,
                store_format_version.to_be_bytes().to_vec(),
            ));
            out.push(meta_insert(MetaKey::StoreId, store_id.to_vec()));
            Ok(out)
        }
        QuickCheck => one("PRAGMA quick_check", vec![]),
        IntegrityCheck => one("PRAGMA integrity_check", vec![]),
        ReadMeta { key } => one(
            "SELECT value FROM meta WHERE key = ?1",
            vec![Param::Text(key.as_str())],
        ),
        AppendCommit { seq, commit } => Ok(vec![Statement::guarded(
            // The row lands only at the last seq plus one, and the guard voids the whole batch
            // otherwise, so the log stays dense and no blob of a failed commit is kept.
            "INSERT INTO log (seq, commit_bytes) SELECT ?1, ?2 \
             WHERE ?1 = (SELECT coalesce(max(seq), 0) + 1 FROM log)",
            vec![seq_param(*seq)?, Param::Blob(commit.clone())],
            1,
        )]),
        LastSeq => one("SELECT coalesce(max(seq), 0) FROM log", vec![]),
        ReadCommits { from, limit } => one(
            "SELECT seq, commit_bytes FROM log WHERE seq >= ?1 ORDER BY seq LIMIT ?2",
            vec![seq_param(*from)?, Param::Integer(i64::from(*limit))],
        ),
        PutBlob { digest, bytes } => one(
            "INSERT OR IGNORE INTO blobs (digest, bytes) VALUES (?1, ?2)",
            vec![Param::Blob(digest.to_vec()), Param::Blob(bytes.clone())],
        ),
        GetBlob { digest } => one(
            "SELECT bytes FROM blobs WHERE digest = ?1",
            vec![Param::Blob(digest.to_vec())],
        ),
        PruneBlobs { digests } => Ok(digests
            .iter()
            .map(|d| {
                Statement::new(
                    "DELETE FROM blobs WHERE digest = ?1",
                    vec![Param::Blob(d.to_vec())],
                )
            })
            .collect()),
        ReplaceCache {
            seq,
            fold_version,
            projection,
        } => Ok(vec![
            Statement::new("DELETE FROM projection_cache", vec![]),
            Statement::new(
                "INSERT INTO projection_cache (seq, fold_version, projection) VALUES (?1, ?2, ?3)",
                vec![
                    seq_param(*seq)?,
                    Param::Integer(i64::from(*fold_version)),
                    Param::Blob(projection.clone()),
                ],
            ),
        ]),
        ReadCache => one(
            "SELECT seq, fold_version, projection FROM projection_cache ORDER BY seq DESC LIMIT 1",
            vec![],
        ),
    }
}

fn meta_insert(key: MetaKey, value: Vec<u8>) -> Statement {
    Statement::new(
        "INSERT INTO meta (key, value) VALUES (?1, ?2)",
        vec![Param::Text(key.as_str()), Param::Blob(value)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::APPLICATION_ID;

    fn sql_of(p: &Plan) -> Vec<&'static str> {
        p.batch().statements().iter().map(|s| s.sql()).collect()
    }

    #[test]
    fn the_application_id_literal_is_the_constant() {
        assert!(SET_APPLICATION_ID.ends_with(&format!("= {APPLICATION_ID}")));
    }

    #[test]
    fn instructions_that_land_together_share_one_atomic_batch() {
        let p = plan(vec![
            Instruction::PutBlob {
                digest: [1; 32],
                bytes: vec![9],
            },
            Instruction::AppendCommit {
                seq: 1,
                commit: vec![7],
            },
        ])
        .expect("plannable");
        assert!(p.batch().is_atomic());
        assert_eq!(p.batch().statements().len(), 2);
        assert_eq!(p.spans, vec![0..1, 1..2]);
    }

    #[test]
    fn appending_binds_seq_and_bytes_and_guards_density() {
        let p = plan(vec![Instruction::AppendCommit {
            seq: 42,
            commit: vec![1, 2],
        }])
        .expect("plannable");
        let s = &p.batch().statements()[0];
        assert!(s.sql().contains("max(seq), 0) + 1"));
        assert_eq!(s.must_change(), Some(1));
        assert_eq!(s.params(), &[Param::Integer(42), Param::Blob(vec![1, 2])]);
    }

    #[test]
    fn a_failed_append_guard_means_the_seq_was_not_next() {
        let p = plan(vec![
            Instruction::PutBlob {
                digest: [1; 32],
                bytes: vec![],
            },
            Instruction::AppendCommit {
                seq: 9,
                commit: vec![],
            },
        ])
        .expect("plannable");
        assert_eq!(p.guard_error(1), StoreError::SeqNotNext { seq: 9 });
        assert!(matches!(p.guard_error(0), StoreError::Shape { .. }));
    }

    #[test]
    fn a_seq_beyond_sqlite_integers_is_refused() {
        let e = plan(vec![
            Instruction::LastSeq,
            Instruction::ReadCommits {
                from: u64::MAX,
                limit: 1,
            },
        ]);
        assert_eq!(e, Err(StoreError::SeqOutOfRange { seq: u64::MAX }));
    }

    #[test]
    fn initializing_creates_four_tables_then_writes_identity() {
        let p = plan(vec![Instruction::Initialize {
            store_id: [3; 16],
            store_format_version: 1,
        }])
        .expect("plannable");
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
    fn pruning_deletes_each_named_blob_and_nothing_else() {
        let p = plan(vec![Instruction::PruneBlobs {
            digests: vec![[1; 32], [2; 32]],
        }])
        .expect("plannable");
        assert_eq!(sql_of(&p), vec!["DELETE FROM blobs WHERE digest = ?1"; 2]);
    }

    #[test]
    fn no_instruction_updates_or_deletes_a_commit() {
        let every = vec![
            Instruction::Inspect,
            Instruction::Initialize {
                store_id: [0; 16],
                store_format_version: 1,
            },
            Instruction::QuickCheck,
            Instruction::IntegrityCheck,
            Instruction::ReadMeta {
                key: MetaKey::StoreId,
            },
            Instruction::AppendCommit {
                seq: 1,
                commit: vec![],
            },
            Instruction::LastSeq,
            Instruction::ReadCommits { from: 1, limit: 10 },
            Instruction::PutBlob {
                digest: [0; 32],
                bytes: vec![],
            },
            Instruction::GetBlob { digest: [0; 32] },
            Instruction::PruneBlobs {
                digests: vec![[0; 32]],
            },
            Instruction::ReplaceCache {
                seq: 1,
                fold_version: 1,
                projection: vec![],
            },
            Instruction::ReadCache,
        ];
        for sql in sql_of(&plan(every).expect("plannable")) {
            let upper = sql.to_ascii_uppercase();
            assert!(!upper.starts_with("UPDATE"), "{sql}");
            assert!(!upper.starts_with("DELETE FROM LOG"), "{sql}");
            assert!(!upper.starts_with("DROP"), "{sql}");
        }
    }
}
