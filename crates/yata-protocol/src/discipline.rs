//! The request discipline of `probe-protocol.md`, "Request discipline", as one state machine both
//! peers run: the daemon over what the reader answers, the reader over what the daemon asks.
//!
//! Ids are chosen by the daemon, start at 1, and only grow, so an id is never reused. Each request
//! gets exactly one terminal answer (`ReadResult` or `Failed`); `Progress` may come any number of
//! times before it and never after. A `Cancel` for an answered request is ignored, and one for an
//! id never issued is a protocol error. An id on the wire is a `u64`; it becomes a [`RequestId`]
//! once, where it arrives, and 0 is refused there.

use std::collections::BTreeSet;
use std::num::NonZeroU64;

/// A request's id: never 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestId(NonZeroU64);

impl RequestId {
    /// The id a wire value names, or [`Breach::ZeroId`].
    pub fn new(id: u64) -> Result<RequestId, Breach> {
        NonZeroU64::new(id).map(RequestId).ok_or(Breach::ZeroId)
    }

    /// The first id of a session.
    pub const FIRST: RequestId = RequestId(NonZeroU64::MIN);

    pub fn get(self) -> u64 {
        self.0.get()
    }

    /// The id after this one, or `None` past `u64::MAX`.
    pub fn next(self) -> Option<RequestId> {
        self.0.checked_add(1).map(RequestId)
    }
}

/// A breach of the discipline. The session ends with `probe.protocol_error`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breach {
    /// Id 0 is not a request.
    ZeroId,
    /// An id not above every id issued before it.
    Reused { id: RequestId, highest: RequestId },
    /// A message about an id never issued.
    Unknown { id: RequestId },
    /// A `Progress` or a second terminal answer for a request already answered.
    AfterTerminal { id: RequestId },
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
    highest: Option<RequestId>,
    open: BTreeSet<RequestId>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger::default()
    }

    /// The highest id issued so far.
    pub fn highest(&self) -> Option<RequestId> {
        self.highest
    }

    /// A new request: its id must be above every earlier one.
    pub fn issue(&mut self, id: RequestId) -> Result<(), Breach> {
        if let Some(highest) = self.highest
            && id <= highest
        {
            return Err(Breach::Reused { id, highest });
        }
        self.highest = Some(id);
        self.open.insert(id);
        Ok(())
    }

    /// A `Progress` for a request: only while it is open.
    pub fn progress(&self, id: RequestId) -> Result<(), Breach> {
        self.check_open(id)
    }

    /// The one terminal answer for a request.
    pub fn answer(&mut self, id: RequestId) -> Result<(), Breach> {
        self.check_open(id)?;
        self.open.remove(&id);
        Ok(())
    }

    /// A `Cancel` for a request.
    pub fn cancel(&self, id: RequestId) -> Result<CancelEffect, Breach> {
        match self.check_open(id) {
            Ok(()) => Ok(CancelEffect::Stop),
            Err(Breach::AfterTerminal { .. }) => Ok(CancelEffect::Ignore),
            Err(b) => Err(b),
        }
    }

    /// Requests issued and not yet answered, in id order.
    pub fn open(&self) -> impl Iterator<Item = RequestId> + '_ {
        self.open.iter().copied()
    }

    fn check_open(&self, id: RequestId) -> Result<(), Breach> {
        if self.open.contains(&id) {
            Ok(())
        } else if self.highest.is_some_and(|h| id <= h) {
            Err(Breach::AfterTerminal { id })
        } else {
            Err(Breach::Unknown { id })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u64) -> RequestId {
        RequestId::new(n).expect("nonzero")
    }

    #[test]
    fn zero_is_never_a_request_id() {
        assert_eq!(RequestId::new(0), Err(Breach::ZeroId));
        assert_eq!(RequestId::FIRST.get(), 1);
        assert_eq!(id(1).next(), Some(id(2)));
        assert_eq!(id(u64::MAX).next(), None);
    }

    #[test]
    fn a_request_gets_progress_then_exactly_one_answer() {
        let mut l = Ledger::new();
        assert_eq!(l.issue(id(1)), Ok(()));
        assert_eq!(l.progress(id(1)), Ok(()));
        assert_eq!(l.answer(id(1)), Ok(()));
        assert_eq!(l.answer(id(1)), Err(Breach::AfterTerminal { id: id(1) }));
        assert_eq!(l.progress(id(1)), Err(Breach::AfterTerminal { id: id(1) }));
        assert_eq!(l.open().count(), 0);
    }

    #[test]
    fn ids_only_grow() {
        let mut l = Ledger::new();
        assert_eq!(l.issue(id(3)), Ok(()));
        assert_eq!(
            l.issue(id(3)),
            Err(Breach::Reused {
                id: id(3),
                highest: id(3)
            })
        );
        assert_eq!(
            l.issue(id(2)),
            Err(Breach::Reused {
                id: id(2),
                highest: id(3)
            })
        );
        assert_eq!(l.issue(id(9)), Ok(()));
        assert_eq!(l.highest(), Some(id(9)));
        assert_eq!(l.open().collect::<Vec<_>>(), vec![id(3), id(9)]);
    }

    #[test]
    fn an_answer_for_an_id_never_issued_is_a_breach() {
        let mut l = Ledger::new();
        assert_eq!(l.answer(id(1)), Err(Breach::Unknown { id: id(1) }));
        assert_eq!(l.progress(id(5)), Err(Breach::Unknown { id: id(5) }));
    }

    #[test]
    fn a_cancel_stops_an_open_request_and_is_ignored_after_its_answer() {
        let mut l = Ledger::new();
        l.issue(id(1)).expect("fresh");
        assert_eq!(l.cancel(id(1)), Ok(CancelEffect::Stop));
        l.answer(id(1)).expect("open");
        assert_eq!(l.cancel(id(1)), Ok(CancelEffect::Ignore));
        assert_eq!(l.cancel(id(2)), Err(Breach::Unknown { id: id(2) }));
    }
}
