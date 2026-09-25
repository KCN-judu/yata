//! The instruction set: one type per instruction, each with the output it reads back.
//!
//! The set is closed. [`Instruction`] is sealed, so only this crate defines instructions; a new
//! need is a new type here, reviewed as a persistence change (ADR-0019, rule 5). Instructions
//! that must land together are planned as a tuple or a `Vec`, and their outputs come back in the
//! same shape, so a caller never matches an output by position.

use std::num::NonZeroU64;

use crate::interpret::StoreError;
use crate::statement::{Statement, StatementResult};

/// A SHA-256 digest: the identity of a blob (`fact-format.md`, § Blobs and content addressing).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Digest(pub [u8; 32]);

/// The store's own random identity, written once when the store is created.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreId(pub [u8; 16]);

/// A position in the log: from 1, and within SQLite's signed 64-bit integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Seq(NonZeroU64);

impl Seq {
    pub const FIRST: Seq = Seq(NonZeroU64::MIN);

    /// `None` for 0, and for a value SQLite cannot store.
    pub fn new(n: u64) -> Option<Seq> {
        if i64::try_from(n).is_err() {
            return None;
        }
        NonZeroU64::new(n).map(Seq)
    }

    pub fn get(self) -> u64 {
        self.0.get()
    }

    pub fn get_nonzero(self) -> NonZeroU64 {
        self.0
    }

    /// The next position, if SQLite can store it.
    pub fn next(self) -> Option<Seq> {
        Seq::new(self.get() + 1)
    }

    pub(crate) fn param(self) -> crate::statement::Param {
        // `new` admits only values that fit.
        crate::statement::Param::Integer(self.get().cast_signed())
    }
}

/// The fixed keys of the `meta` table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaKey {
    /// The store format version, which moves only when the envelope or the table layout changes.
    StoreFormatVersion,
    StoreId,
}

impl MetaKey {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            MetaKey::StoreFormatVersion => "store_format_version",
            MetaKey::StoreId => "store_id",
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Something the store can be asked to do, and what it answers.
pub trait Instruction: sealed::Sealed {
    type Output;

    /// The statements that carry it out, in order.
    fn statements(&self) -> Vec<Statement>;

    /// Read the results of exactly [`Instruction::statements`] back into the output.
    fn interpret(&self, results: &[StatementResult]) -> Result<Self::Output, StoreError>;

    /// What a guarded statement's failure means, by its index among this instruction's
    /// statements; `None` if that statement has no guard.
    fn guard_error(&self, _statement: usize) -> Option<StoreError> {
        None
    }
}

pub(crate) use sealed::Sealed;

/// Split a combined result list after the first part's `at` results.
fn split(
    results: &[StatementResult],
    at: usize,
) -> Result<(&[StatementResult], &[StatementResult]), StoreError> {
    if at > results.len() {
        return Err(StoreError::ResultCount {
            expected: at,
            actual: results.len(),
        });
    }
    Ok(results.split_at(at))
}

impl<A: Instruction, B: Instruction> Sealed for (A, B) {}

impl<A: Instruction, B: Instruction> Instruction for (A, B) {
    type Output = (A::Output, B::Output);

    fn statements(&self) -> Vec<Statement> {
        let mut out = self.0.statements();
        out.extend(self.1.statements());
        out
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Self::Output, StoreError> {
        let (a, b) = split(results, self.0.statements().len())?;
        Ok((self.0.interpret(a)?, self.1.interpret(b)?))
    }

    fn guard_error(&self, statement: usize) -> Option<StoreError> {
        let n = self.0.statements().len();
        if statement < n {
            self.0.guard_error(statement)
        } else {
            self.1.guard_error(statement - n)
        }
    }
}

impl<A: Instruction> Sealed for Vec<A> {}

impl<A: Instruction> Instruction for Vec<A> {
    type Output = Vec<A::Output>;

    fn statements(&self) -> Vec<Statement> {
        self.iter().flat_map(Instruction::statements).collect()
    }

    fn interpret(&self, results: &[StatementResult]) -> Result<Self::Output, StoreError> {
        let mut rest = results;
        let mut out = Vec::with_capacity(self.len());
        for i in self {
            let (mine, after) = split(rest, i.statements().len())?;
            out.push(i.interpret(mine)?);
            rest = after;
        }
        Ok(out)
    }

    fn guard_error(&self, statement: usize) -> Option<StoreError> {
        let mut offset = 0;
        for i in self {
            let n = i.statements().len();
            if statement < offset + n {
                return i.guard_error(statement - offset);
            }
            offset += n;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seq_is_positive_and_fits_sqlite() {
        assert_eq!(Seq::new(0), None);
        assert_eq!(Seq::new(1), Some(Seq::FIRST));
        assert_eq!(Seq::new(u64::MAX), None);
        assert!(Seq::new(i64::MAX as u64).is_some());
        assert_eq!(Seq::new(i64::MAX as u64).and_then(Seq::next), None);
    }
}
