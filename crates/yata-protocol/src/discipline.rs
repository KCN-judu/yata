//! The request discipline of `probe-protocol.md`, "Request discipline", as one state machine both
//! peers run: the daemon over what the reader answers, the reader over what the daemon asks.
//!
//! Ids are chosen by the daemon, start at 1, and only grow, so an id is never reused. Each request
//! gets exactly one terminal answer (`ReadResult` or `Failed`); `Progress` may come any number of
//! times before it and never after. A `Cancel` for an answered request is ignored, and one for an
//! id never issued is a protocol error.

use std::collections::BTreeSet;

/// A breach of the discipline. The session ends with `probe.protocol_error`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breach {
    /// Id 0 names the session, not a request.
    ZeroId,
    /// An id not above every id issued before it.
    Reused { id: u64, highest: u64 },
    /// A message about an id never issued.
    Unknown { id: u64 },
    /// A `Progress` or a second terminal answer for a request already answered.
    AfterTerminal { id: u64 },
}

/// What a `Cancel` means where it arrives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelEffect {
    /// The request is still open: stop it at the next checkpoint and answer `probe.cancelled`.
    Stop,
    /// The request was already answered; the cancel is ignored.
    Ignore,
}

/// The requests of one session.
#[derive(Debug, Default)]
pub struct Ledger {
    highest: u64,
    open: BTreeSet<u64>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger::default()
    }

    /// A new request: its id must be above every earlier one.
    pub fn issue(&mut self, id: u64) -> Result<(), Breach> {
        if id == 0 {
            return Err(Breach::ZeroId);
        }
        if id <= self.highest {
            return Err(Breach::Reused {
                id,
                highest: self.highest,
            });
        }
        self.highest = id;
        self.open.insert(id);
        Ok(())
    }

    /// A `Progress` for a request: only while it is open.
    pub fn progress(&self, id: u64) -> Result<(), Breach> {
        self.check_open(id)
    }

    /// The one terminal answer for a request.
    pub fn answer(&mut self, id: u64) -> Result<(), Breach> {
        self.check_open(id)?;
        self.open.remove(&id);
        Ok(())
    }

    /// A `Cancel` for a request.
    pub fn cancel(&self, id: u64) -> Result<CancelEffect, Breach> {
        match self.check_open(id) {
            Ok(()) => Ok(CancelEffect::Stop),
            Err(Breach::AfterTerminal { .. }) => Ok(CancelEffect::Ignore),
            Err(b) => Err(b),
        }
    }

    /// Requests issued and not yet answered, in id order.
    pub fn open(&self) -> impl Iterator<Item = u64> + '_ {
        self.open.iter().copied()
    }

    fn check_open(&self, id: u64) -> Result<(), Breach> {
        if id == 0 {
            Err(Breach::ZeroId)
        } else if self.open.contains(&id) {
            Ok(())
        } else if id <= self.highest {
            Err(Breach::AfterTerminal { id })
        } else {
            Err(Breach::Unknown { id })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_gets_progress_then_exactly_one_answer() {
        let mut l = Ledger::new();
        assert_eq!(l.issue(1), Ok(()));
        assert_eq!(l.progress(1), Ok(()));
        assert_eq!(l.progress(1), Ok(()));
        assert_eq!(l.answer(1), Ok(()));
        assert_eq!(l.answer(1), Err(Breach::AfterTerminal { id: 1 }));
        assert_eq!(l.progress(1), Err(Breach::AfterTerminal { id: 1 }));
        assert_eq!(l.open().count(), 0);
    }

    #[test]
    fn ids_start_at_one_and_only_grow() {
        let mut l = Ledger::new();
        assert_eq!(l.issue(0), Err(Breach::ZeroId));
        assert_eq!(l.issue(3), Ok(()));
        assert_eq!(l.issue(3), Err(Breach::Reused { id: 3, highest: 3 }));
        assert_eq!(l.issue(2), Err(Breach::Reused { id: 2, highest: 3 }));
        assert_eq!(l.issue(9), Ok(()));
        assert_eq!(l.open().collect::<Vec<_>>(), vec![3, 9]);
    }

    #[test]
    fn an_answer_for_an_id_never_issued_is_a_breach() {
        let mut l = Ledger::new();
        assert_eq!(l.answer(1), Err(Breach::Unknown { id: 1 }));
        assert_eq!(l.progress(5), Err(Breach::Unknown { id: 5 }));
    }

    #[test]
    fn a_cancel_stops_an_open_request_and_is_ignored_after_its_answer() {
        let mut l = Ledger::new();
        l.issue(1).expect("fresh");
        assert_eq!(l.cancel(1), Ok(CancelEffect::Stop));
        l.answer(1).expect("open");
        assert_eq!(l.cancel(1), Ok(CancelEffect::Ignore));
        assert_eq!(l.cancel(2), Err(Breach::Unknown { id: 2 }));
        assert_eq!(l.cancel(0), Err(Breach::ZeroId));
    }
}
