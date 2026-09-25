//! The probe protocol's failures as values both peers share (`probe-protocol.md`, "Errors"): the
//! reader builds them and writes them as `ProbeError`s, the daemon parses `ProbeError`s back into
//! them. One vocabulary, so the two sides cannot disagree on what a code means or which failures
//! end the reader.
//!
//! The wire's `ProbeErrorCode` admits an unspecified value and puts every code in one enum. The
//! types here admit neither: a code is a [`SessionCode`], which ends the session and has a total
//! [`SessionCode::exit`], or a [`RequestCode`], which answers one request. The discovery detail
//! lives only on the one reason that has it.

use std::num::NonZeroU32;

use crate::probe::{Discovery, ProbeError, ProbeErrorCode, TargetProcess, probe_error::Detail};

/// The reader's process exit codes (`probe-protocol.md`, "Frame").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exit {
    /// Clean shutdown, requested by the daemon.
    Clean,
    /// Could not attach: the game was not found, was ambiguous, exited, or refused access.
    NotAttached,
    /// The target was found but no read strategy matched it.
    NoStrategy,
    /// The daemon sent a frame the reader could not decode, or broke the discipline.
    Protocol,
    /// An internal failure, with the reason in the reader's log file.
    Internal,
    /// The game is elevated and the reader is not.
    ElevationRequired,
}

impl Exit {
    pub const ALL: [Exit; 6] = [
        Exit::Clean,
        Exit::NotAttached,
        Exit::NoStrategy,
        Exit::Protocol,
        Exit::Internal,
        Exit::ElevationRequired,
    ];

    /// The process exit code.
    pub fn code(self) -> u8 {
        match self {
            Exit::Clean => 0,
            Exit::NotAttached => 1,
            Exit::NoStrategy => 2,
            Exit::Protocol => 3,
            Exit::Internal => 4,
            Exit::ElevationRequired => 5,
        }
    }

    pub fn from_code(code: i32) -> Option<Exit> {
        Exit::ALL.into_iter().find(|e| i32::from(e.code()) == code)
    }
}

/// A code that ends the session, and the reader with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionCode {
    NotFound,
    AmbiguousTarget,
    ElevationRequired,
    AccessDenied,
    ProcessExited,
    UnsupportedEnvironment,
    LayoutMismatch,
    ProtocolUnsupported,
    ProtocolError,
    Internal,
}

/// A code that answers one request; the session goes on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestCode {
    Cancelled,
    ScopeUnsupported,
    NotAttached,
}

/// Any code: which kind it is decides what it ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProbeCode {
    Session(SessionCode),
    Request(RequestCode),
}

impl SessionCode {
    /// The exit the reader ends with (`probe-protocol.md`, "Error codes").
    pub fn exit(self) -> Exit {
        match self {
            SessionCode::NotFound
            | SessionCode::AmbiguousTarget
            | SessionCode::AccessDenied
            | SessionCode::ProcessExited => Exit::NotAttached,
            SessionCode::ElevationRequired => Exit::ElevationRequired,
            SessionCode::UnsupportedEnvironment | SessionCode::LayoutMismatch => Exit::NoStrategy,
            SessionCode::ProtocolUnsupported | SessionCode::ProtocolError => Exit::Protocol,
            SessionCode::Internal => Exit::Internal,
        }
    }
}

impl ProbeCode {
    /// The code a wire value is, or `None` for the unspecified value or one this build does not
    /// know.
    pub fn parse(wire: i32) -> Option<ProbeCode> {
        use ProbeCode::{Request as R, Session as S};
        Some(match ProbeErrorCode::try_from(wire).ok()? {
            ProbeErrorCode::Unspecified => return None,
            ProbeErrorCode::NotFound => S(SessionCode::NotFound),
            ProbeErrorCode::AmbiguousTarget => S(SessionCode::AmbiguousTarget),
            ProbeErrorCode::ElevationRequired => S(SessionCode::ElevationRequired),
            ProbeErrorCode::AccessDenied => S(SessionCode::AccessDenied),
            ProbeErrorCode::ProcessExited => S(SessionCode::ProcessExited),
            ProbeErrorCode::UnsupportedEnvironment => S(SessionCode::UnsupportedEnvironment),
            ProbeErrorCode::LayoutMismatch => S(SessionCode::LayoutMismatch),
            ProbeErrorCode::ProtocolUnsupported => S(SessionCode::ProtocolUnsupported),
            ProbeErrorCode::ProtocolError => S(SessionCode::ProtocolError),
            ProbeErrorCode::Internal => S(SessionCode::Internal),
            ProbeErrorCode::Cancelled => R(RequestCode::Cancelled),
            ProbeErrorCode::ScopeUnsupported => R(RequestCode::ScopeUnsupported),
            ProbeErrorCode::NotAttached => R(RequestCode::NotAttached),
        })
    }

    /// The wire value.
    pub fn wire(self) -> ProbeErrorCode {
        match self {
            ProbeCode::Session(c) => match c {
                SessionCode::NotFound => ProbeErrorCode::NotFound,
                SessionCode::AmbiguousTarget => ProbeErrorCode::AmbiguousTarget,
                SessionCode::ElevationRequired => ProbeErrorCode::ElevationRequired,
                SessionCode::AccessDenied => ProbeErrorCode::AccessDenied,
                SessionCode::ProcessExited => ProbeErrorCode::ProcessExited,
                SessionCode::UnsupportedEnvironment => ProbeErrorCode::UnsupportedEnvironment,
                SessionCode::LayoutMismatch => ProbeErrorCode::LayoutMismatch,
                SessionCode::ProtocolUnsupported => ProbeErrorCode::ProtocolUnsupported,
                SessionCode::ProtocolError => ProbeErrorCode::ProtocolError,
                SessionCode::Internal => ProbeErrorCode::Internal,
            },
            ProbeCode::Request(c) => match c {
                RequestCode::Cancelled => ProbeErrorCode::Cancelled,
                RequestCode::ScopeUnsupported => ProbeErrorCode::ScopeUnsupported,
                RequestCode::NotAttached => ProbeErrorCode::NotAttached,
            },
        }
    }

    /// The code's stable dotted name, as the spec, logs, and the daemon's error details write it.
    pub fn name(self) -> &'static str {
        match self {
            ProbeCode::Session(c) => match c {
                SessionCode::NotFound => "probe.not_found",
                SessionCode::AmbiguousTarget => "probe.ambiguous_target",
                SessionCode::ElevationRequired => "probe.elevation_required",
                SessionCode::AccessDenied => "probe.access_denied",
                SessionCode::ProcessExited => "probe.process_exited",
                SessionCode::UnsupportedEnvironment => "probe.unsupported_environment",
                SessionCode::LayoutMismatch => "probe.layout_mismatch",
                SessionCode::ProtocolUnsupported => "probe.protocol_unsupported",
                SessionCode::ProtocolError => "probe.protocol_error",
                SessionCode::Internal => "probe.internal",
            },
            ProbeCode::Request(c) => match c {
                RequestCode::Cancelled => "probe.cancelled",
                RequestCode::ScopeUnsupported => "probe.scope_unsupported",
                RequestCode::NotAttached => "probe.not_attached",
            },
        }
    }
}

/// Why a session ends. Only the ambiguous target carries data: the candidates, ordered by pid.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionReason {
    NotFound,
    Ambiguous { candidates: Vec<TargetProcess> },
    ElevationRequired,
    AccessDenied,
    ProcessExited,
    UnsupportedEnvironment,
    LayoutMismatch,
    ProtocolUnsupported,
    ProtocolError,
    Internal,
}

impl SessionReason {
    pub fn code(&self) -> SessionCode {
        match self {
            SessionReason::NotFound => SessionCode::NotFound,
            SessionReason::Ambiguous { .. } => SessionCode::AmbiguousTarget,
            SessionReason::ElevationRequired => SessionCode::ElevationRequired,
            SessionReason::AccessDenied => SessionCode::AccessDenied,
            SessionReason::ProcessExited => SessionCode::ProcessExited,
            SessionReason::UnsupportedEnvironment => SessionCode::UnsupportedEnvironment,
            SessionReason::LayoutMismatch => SessionCode::LayoutMismatch,
            SessionReason::ProtocolUnsupported => SessionCode::ProtocolUnsupported,
            SessionReason::ProtocolError => SessionCode::ProtocolError,
            SessionReason::Internal => SessionCode::Internal,
        }
    }
}

/// A failure that ends the session.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionFailure {
    pub reason: SessionReason,
    pub message: String,
    /// The operating-system error the failure came from, when there is one.
    pub os_error: Option<NonZeroU32>,
}

/// A failure that answers one request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestFailure {
    pub code: RequestCode,
    pub message: String,
}

/// A failure of either kind, as parsed from a `ProbeError`.
#[derive(Debug, Clone, PartialEq)]
pub enum Failure {
    Session(SessionFailure),
    Request(RequestFailure),
}

/// Why a `ProbeError` is not a failure this build reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureError {
    /// The code is unspecified, or one this build does not know.
    UnstatedCode,
    /// An ambiguous target without its candidates.
    NoCandidates,
    /// A discovery detail on a code that has none.
    DetailOnOtherCode(ProbeCode),
    /// An operating-system error on a request's failure, which comes from no system call.
    OsErrorOnRequest(RequestCode),
    /// An operating-system error number of 0, which is no error.
    ZeroOsError,
}

impl SessionFailure {
    pub fn new(reason: SessionReason, message: impl Into<String>) -> SessionFailure {
        SessionFailure {
            reason,
            message: message.into(),
            os_error: None,
        }
    }

    pub fn code(&self) -> SessionCode {
        self.reason.code()
    }

    pub fn exit(&self) -> Exit {
        self.code().exit()
    }

    pub fn name(&self) -> &'static str {
        ProbeCode::Session(self.code()).name()
    }

    pub fn to_wire(&self) -> ProbeError {
        ProbeError {
            code: ProbeCode::Session(self.code()).wire().into(),
            message: self.message.clone(),
            os_error: self.os_error.map(NonZeroU32::get),
            detail: match &self.reason {
                SessionReason::Ambiguous { candidates } => Some(Detail::Discovery(Discovery {
                    candidates: candidates.clone(),
                })),
                _ => None,
            },
        }
    }
}

impl RequestFailure {
    pub fn new(code: RequestCode, message: impl Into<String>) -> RequestFailure {
        RequestFailure {
            code,
            message: message.into(),
        }
    }

    pub fn name(&self) -> &'static str {
        ProbeCode::Request(self.code).name()
    }

    pub fn to_wire(&self) -> ProbeError {
        ProbeError {
            code: ProbeCode::Request(self.code).wire().into(),
            message: self.message.clone(),
            os_error: None,
            detail: None,
        }
    }
}

impl Failure {
    /// Parse a wire error, refusing every shape the schema's comments rule out.
    pub fn from_wire(e: ProbeError) -> Result<Failure, FailureError> {
        let code = ProbeCode::parse(e.code).ok_or(FailureError::UnstatedCode)?;
        let os_error = match e.os_error {
            None => None,
            Some(n) => Some(NonZeroU32::new(n).ok_or(FailureError::ZeroOsError)?),
        };
        match (code, e.detail) {
            (ProbeCode::Request(c), detail) => {
                if detail.is_some() {
                    return Err(FailureError::DetailOnOtherCode(code));
                }
                if os_error.is_some() {
                    return Err(FailureError::OsErrorOnRequest(c));
                }
                Ok(Failure::Request(RequestFailure {
                    code: c,
                    message: e.message,
                }))
            }
            (ProbeCode::Session(c), detail) => {
                let reason = match (c, detail) {
                    (SessionCode::AmbiguousTarget, Some(Detail::Discovery(d))) => {
                        SessionReason::Ambiguous {
                            candidates: d.candidates,
                        }
                    }
                    (SessionCode::AmbiguousTarget, None) => {
                        return Err(FailureError::NoCandidates);
                    }
                    (_, Some(_)) => return Err(FailureError::DetailOnOtherCode(code)),
                    (SessionCode::NotFound, None) => SessionReason::NotFound,
                    (SessionCode::ElevationRequired, None) => SessionReason::ElevationRequired,
                    (SessionCode::AccessDenied, None) => SessionReason::AccessDenied,
                    (SessionCode::ProcessExited, None) => SessionReason::ProcessExited,
                    (SessionCode::UnsupportedEnvironment, None) => {
                        SessionReason::UnsupportedEnvironment
                    }
                    (SessionCode::LayoutMismatch, None) => SessionReason::LayoutMismatch,
                    (SessionCode::ProtocolUnsupported, None) => SessionReason::ProtocolUnsupported,
                    (SessionCode::ProtocolError, None) => SessionReason::ProtocolError,
                    (SessionCode::Internal, None) => SessionReason::Internal,
                };
                Ok(Failure::Session(SessionFailure {
                    reason,
                    message: e.message,
                    os_error,
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every wire value this build's schema defines.
    fn every_wire_code() -> impl Iterator<Item = i32> {
        (0..).map_while(|n| ProbeErrorCode::try_from(n).ok().map(|_| n))
    }

    #[test]
    fn every_stated_code_parses_round_trips_and_has_a_distinct_name() {
        let codes: Vec<ProbeCode> = every_wire_code().filter_map(ProbeCode::parse).collect();
        // Every value but the unspecified one is a code.
        assert_eq!(codes.len() + 1, every_wire_code().count());
        let mut names = std::collections::BTreeSet::new();
        for c in &codes {
            assert_eq!(ProbeCode::parse(c.wire().into()), Some(*c));
            assert!(c.name().starts_with("probe."));
            assert!(names.insert(c.name()), "{c:?}");
        }
        assert_eq!(ProbeCode::parse(0), None);
        assert_eq!(ProbeCode::parse(9_999), None);
    }

    #[test]
    fn only_session_codes_end_the_reader_and_never_cleanly() {
        for c in every_wire_code().filter_map(ProbeCode::parse) {
            if let ProbeCode::Session(s) = c {
                assert_ne!(s.exit(), Exit::Clean, "{s:?}");
            }
        }
        for e in Exit::ALL {
            assert_eq!(Exit::from_code(e.code().into()), Some(e));
        }
    }

    fn wire(code: ProbeErrorCode, detail: Option<Detail>, os_error: Option<u32>) -> ProbeError {
        ProbeError {
            code: code.into(),
            message: "m".into(),
            os_error,
            detail,
        }
    }

    #[test]
    fn a_failure_round_trips_through_the_wire() {
        let ambiguous = SessionFailure {
            os_error: NonZeroU32::new(5),
            ..SessionFailure::new(
                SessionReason::Ambiguous {
                    candidates: vec![TargetProcess::default()],
                },
                "two",
            )
        };
        assert_eq!(
            Failure::from_wire(ambiguous.to_wire()),
            Ok(Failure::Session(ambiguous))
        );
        let cancelled = RequestFailure::new(RequestCode::Cancelled, "c");
        assert_eq!(
            Failure::from_wire(cancelled.to_wire()),
            Ok(Failure::Request(cancelled))
        );
    }

    #[test]
    fn shapes_the_schema_rules_out_are_refused() {
        let discovery = || Some(Detail::Discovery(Discovery::default()));
        let cases = [
            (
                wire(ProbeErrorCode::Unspecified, None, None),
                FailureError::UnstatedCode,
            ),
            (
                wire(ProbeErrorCode::AmbiguousTarget, None, None),
                FailureError::NoCandidates,
            ),
            (
                wire(ProbeErrorCode::ProtocolError, discovery(), None),
                FailureError::DetailOnOtherCode(ProbeCode::Session(SessionCode::ProtocolError)),
            ),
            (
                wire(ProbeErrorCode::Cancelled, discovery(), None),
                FailureError::DetailOnOtherCode(ProbeCode::Request(RequestCode::Cancelled)),
            ),
            (
                wire(ProbeErrorCode::Cancelled, None, Some(5)),
                FailureError::OsErrorOnRequest(RequestCode::Cancelled),
            ),
            (
                wire(ProbeErrorCode::AccessDenied, None, Some(0)),
                FailureError::ZeroOsError,
            ),
        ];
        for (e, expected) in cases {
            assert_eq!(Failure::from_wire(e), Err(expected));
        }
    }
}
