//! The headless query endpoint (ADR-0026, rule 6): `EvaluateQuery` frames in, one
//! `EvaluateQueryResult` frame out for each, over an inventory each request supplies.
//!
//! The daemon owns the orchestration only: decode the message, convert it, ask
//! [`yata_core::query`] to check and run it, encode the page or the error. Every request is handled
//! on its own, from its bytes alone. Nothing is kept between requests, so a refused or even a
//! panicking request leaves nothing behind for the next one.

pub(crate) mod convert;
mod cursor;

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::num::NonZeroUsize;
use std::panic::{AssertUnwindSafe, catch_unwind};

use prost::Message;
use yata_core::fact::{GameSoulId, Revision};
use yata_core::query::{CompiledQuery, Page, PageRequest, QueryError, RowVerdict, compile};
use yata_core::scheme::evaluate::OpenRule;
use yata_core::soul::Soul;
use yata_protocol::core as wire;
use yata_protocol::frame::{self, FrameDecoder, FrameError};

use crate::wire::Code as _;
pub use convert::MAX_SOUL_ID_BYTES;

/// Rows per page when a request names no budget.
pub const DEFAULT_ROW_BUDGET: u32 = 256;
/// The largest row budget: a page of it stays far below a frame's 16 MiB.
pub const MAX_ROW_BUDGET: u32 = 10_000;
/// The largest inventory one request may supply.
pub const MAX_INVENTORY_SOULS: usize = 100_000;

/// Why a request produced an error rather than a page.
#[derive(Debug, Clone, PartialEq)]
pub enum RequestError {
    /// The payload is not an `EvaluateQuery` (`query.malformed`).
    Undecodable { reason: String },
    /// `session.protocol_unsupported`: built against a newer major version, or against none.
    ProtocolUnsupported {
        client: Option<wire::ProtocolVersion>,
    },
    /// `query.unknown_field`: a field number this build does not know.
    UnknownField { value: i32 },
    /// `query.malformed`: a value the schema can carry and the domain cannot.
    Wire(WireProblem),
    /// `query.malformed`: the soul at `index` of the inventory.
    BadSoul { index: usize, problem: WireProblem },
    /// `query.malformed`: the soul at `index` repeats an earlier soul's id.
    RepeatedSoulId { index: usize },
    /// `query.too_complex`: more than [`MAX_INVENTORY_SOULS`].
    InventoryTooLarge { souls: usize },
    /// `query.malformed`: a row budget of zero.
    RowBudgetZero,
    /// `query.too_complex`: a row budget above [`MAX_ROW_BUDGET`].
    RowBudgetTooLarge { budget: u32 },
    /// The core refused the query.
    Query(QueryError),
    /// `internal.panic`: a bug in the daemon; the request is abandoned and nothing else.
    Panicked,
}

/// A wire value with no domain meaning.
#[derive(Debug, Clone, PartialEq)]
pub enum WireProblem {
    /// A required message or oneof is absent.
    Missing(&'static str),
    /// An enum number that is unspecified or unknown.
    Enum { value: i32 },
    /// A number past the domain type's range, such as a suit code above 255.
    OutOfRange,
    /// Not the `Souls` collection.
    Collection { value: i32 },
    /// A chosen 类型 with no set: "every set" is `AnySet`, and has no second encoding.
    NoSets,
    /// One attribute given two ○ or ✕ choices in 副属性.
    SubAttributeTwice,
    /// An innate choice, or a boss soul's innate attribute, outside the six innate attributes.
    NotInnate,
    /// A soul id that is empty or longer than [`MAX_SOUL_ID_BYTES`].
    SoulId,
    /// More sub-attributes than attributes.
    TooManySubs,
    /// One attribute listed twice among a soul's sub-attributes.
    RepeatedSub,
}

impl RequestError {
    /// The failure this is on the wire (`core-protocol.md`, "Errors"), with its debug record.
    pub fn kind(&self) -> wire::error::Kind {
        use wire::error::Kind;
        let problem = || format!("{self:?}");
        let field = |f: &dyn std::fmt::Debug| format!("{f:?}");
        match self {
            RequestError::Undecodable { .. }
            | RequestError::Wire(_)
            | RequestError::BadSoul { .. }
            | RequestError::RepeatedSoulId { .. }
            | RequestError::RowBudgetZero
            | RequestError::Query(QueryError::Malformed(_)) => {
                Kind::QueryMalformed(wire::QueryMalformed { problem: problem() })
            }
            RequestError::ProtocolUnsupported { client } => {
                Kind::SessionProtocolUnsupported(wire::SessionProtocolUnsupported {
                    client: *client,
                    daemon: Some(wire::VERSION),
                })
            }
            RequestError::UnknownField { value } => {
                Kind::QueryUnknownField(wire::QueryUnknownField { value: *value })
            }
            RequestError::InventoryTooLarge { souls } => {
                too_complex("inventory souls", *souls, MAX_INVENTORY_SOULS)
            }
            RequestError::RowBudgetTooLarge { budget } => {
                too_complex("row budget", *budget as usize, MAX_ROW_BUDGET as usize)
            }
            RequestError::Query(QueryError::TooComplex { limit, actual }) => {
                too_complex(&format!("{limit:?}"), *actual, limit.max())
            }
            RequestError::Query(
                QueryError::TypeMismatch { .. } | QueryError::NotSortable { .. },
            ) => Kind::QueryTypeMismatch(wire::QueryTypeMismatch { problem: problem() }),
            RequestError::Query(QueryError::ParamSetRequired { field: f }) => {
                Kind::QueryParamSetRequired(wire::QueryParamSetRequired { field: field(f) })
            }
            RequestError::Query(QueryError::FieldUnavailable { field: f }) => {
                Kind::QueryFieldUnavailable(wire::QueryFieldUnavailable { field: field(f) })
            }
            RequestError::Query(QueryError::UnknownScheme(_)) => {
                Kind::QueryUnknownScheme(wire::QueryUnknownScheme { problem: problem() })
            }
            RequestError::Query(QueryError::MalformedCursor) => {
                Kind::QueryMalformedCursor(wire::QueryMalformedCursor {})
            }
            RequestError::Panicked => Kind::InternalPanic(wire::InternalPanic {}),
        }
    }

    /// The dotted code, for logs and tests.
    pub fn code(&self) -> &'static str {
        self.kind().code()
    }

    pub fn to_wire(&self) -> wire::Error {
        let message = match self {
            RequestError::Panicked => "the query handler panicked; see stderr".to_owned(),
            e => format!("{e:?}"),
        };
        wire::Error {
            message,
            kind: Some(self.kind()),
        }
    }
}

fn too_complex(limit: &str, found: usize, max: usize) -> wire::error::Kind {
    wire::error::Kind::QueryTooComplex(wire::QueryTooComplex {
        limit: limit.to_owned(),
        found: found as u64,
        max: max as u64,
    })
}

/// The answer to one request payload. Total: every payload gets exactly one result.
pub fn handle(payload: &[u8]) -> wire::EvaluateQueryResult {
    use wire::evaluate_query_result::{Outcome, Subject};
    let (subject, outcome) = match wire::EvaluateQuery::decode(payload) {
        Err(e) => (
            Subject::Undecodable(wire::Undecodable {}),
            Err(RequestError::Undecodable {
                reason: e.to_string(),
            }),
        ),
        Ok(request) => {
            let id = request.id;
            // The core is total and the conversion panics on nothing; a panic is a bug, and it
            // must cost this request only. No state is shared, so nothing can be left half-done.
            let outcome = catch_unwind(AssertUnwindSafe(|| evaluate(request)))
                .unwrap_or(Err(RequestError::Panicked));
            (Subject::Id(id), outcome)
        }
    };
    wire::EvaluateQueryResult {
        subject: Some(subject),
        outcome: Some(match outcome {
            Ok(page) => Outcome::Page(page),
            Err(e) => Outcome::Error(e.to_wire()),
        }),
    }
}

fn evaluate(request: wire::EvaluateQuery) -> Result<wire::QueryPage, RequestError> {
    match request.protocol_version {
        Some(v) if v.major <= wire::VERSION.major => {}
        client => return Err(RequestError::ProtocolUnsupported { client }),
    }
    let query = request
        .query
        .ok_or(RequestError::Wire(WireProblem::Missing(
            "EvaluateQuery.query",
        )))?;
    let page = request.page.unwrap_or_default();
    let prepared = prepare(
        query,
        Paging {
            row_budget: page.row_budget,
            cursor: page.cursor.as_deref(),
        },
    )?;
    let souls = convert::inventory(request.inventory)?;
    run(&prepared, &souls).map(render_headless)
}

/// A query checked and compiled, with its page request: everything but the souls.
pub struct Prepared {
    compiled: CompiledQuery,
    request: PageRequest<GameSoulId>,
}

/// Which page a request asks for, from whichever message carries it: `EvaluateQuery.page`, or
/// the session's `SessionQuery`. A shared `Query` carries no page.
#[derive(Debug, Clone, Copy, Default)]
pub struct Paging<'a> {
    /// Absent: [`DEFAULT_ROW_BUDGET`].
    pub row_budget: Option<u32>,
    /// Absent: the first page.
    pub cursor: Option<&'a [u8]>,
}

/// Check and compile a wire query with its page. The headless endpoint and the session both start
/// here, so a query means the same whichever way it arrives.
pub fn prepare(query: wire::Query, paging: Paging<'_>) -> Result<Prepared, RequestError> {
    let budget = paging.row_budget.unwrap_or(DEFAULT_ROW_BUDGET);
    if budget > MAX_ROW_BUDGET {
        return Err(RequestError::RowBudgetTooLarge { budget });
    }
    let row_budget = NonZeroUsize::new(budget as usize).ok_or(RequestError::RowBudgetZero)?;
    let cursor = paging
        .cursor
        .map(|bytes| cursor::decode(bytes).ok_or(RequestError::Query(QueryError::MalformedCursor)))
        .transpose()?;
    let compiled = compile(convert::query(query)?).map_err(RequestError::Query)?;
    Ok(Prepared {
        compiled,
        request: PageRequest { row_budget, cursor },
    })
}

/// The page a prepared query selects from `souls`, with the core's count of every row it keeps.
pub fn run(
    prepared: &Prepared,
    souls: &BTreeMap<GameSoulId, Soul>,
) -> Result<Page<GameSoulId>, RequestError> {
    prepared
        .compiled
        .page(souls, &prepared.request)
        .map_err(RequestError::Query)
}

/// A page as the headless endpoint answers it: ids and verdicts, with no row values (its caller
/// supplied the souls) and no revision (it has no projection).
pub fn render_headless(page: Page<GameSoulId>) -> wire::QueryPage {
    use wire::query_row::Verdict;
    let rows = page
        .rows
        .into_iter()
        .map(|row| wire::QueryRow {
            soul_id: row.id.as_str().to_owned(),
            verdict: Some(match row.verdict {
                RowVerdict::Exact => Verdict::Exact(wire::ExactVerdict {}),
                RowVerdict::Open(rules) => Verdict::Open(wire::OpenVerdict {
                    rules: rules.iter().map(open_rule).collect(),
                }),
            }),
        })
        .collect();
    wire::QueryPage {
        rows,
        next_cursor: page.next.as_ref().map(cursor::encode),
        total: page.total as u64,
    }
}

/// A page as the session answers it: each row with its soul's values, the page's revision, and
/// the total. Every row's id was selected from `souls`, so a row without one is a daemon bug,
/// reported as `internal.page_without_soul` rather than dropped.
pub fn render_session(
    page: Page<GameSoulId>,
    revision: Revision,
    souls: &BTreeMap<GameSoulId, Soul>,
) -> Result<wire::SessionQueryPage, crate::wire::Failure> {
    use wire::session_row::Verdict;
    let rows = page
        .rows
        .into_iter()
        .map(|row| {
            let soul = souls.get(&row.id).ok_or_else(|| {
                crate::wire::Failure::new(
                    wire::error::Kind::InternalPageWithoutSoul(wire::InternalPageWithoutSoul {
                        soul_id: row.id.as_str().to_owned(),
                    }),
                    "a page row is not among the souls it was selected from",
                )
            })?;
            Ok(wire::SessionRow {
                soul: Some(crate::wire::soul(row.id.as_str(), soul)),
                verdict: Some(match row.verdict {
                    RowVerdict::Exact => Verdict::Exact(wire::ExactVerdict {}),
                    RowVerdict::Open(rules) => Verdict::Open(wire::OpenVerdict {
                        rules: rules.iter().map(open_rule).collect(),
                    }),
                }),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(wire::SessionQueryPage {
        rows,
        next_cursor: page.next.as_ref().map(cursor::encode),
        total: page.total as u64,
        revision: crate::wire::revision(revision),
    })
}

fn open_rule(r: OpenRule) -> i32 {
    match r {
        OpenRule::Innate => wire::OpenRule::Innate as i32,
        OpenRule::UnknownConditions => wire::OpenRule::UnknownConditions as i32,
    }
}

/// Why serving stopped before its input ended cleanly.
#[derive(Debug)]
pub enum ServeError {
    /// The framing broke: after it the stream is unusable (`core-protocol.md`, "Frame").
    Frame(FrameError),
    Io(std::io::Error),
}

/// Answer every request frame on `input` with one result frame on `output`, until `input` ends.
///
/// A request that is refused, malformed, or panics gets its error result and serving goes on; only
/// a broken frame or a failed read or write ends it.
pub fn serve(input: &mut impl Read, output: &mut impl Write) -> Result<(), ServeError> {
    let mut decoder = FrameDecoder::new();
    let mut buffer = vec![0u8; 64 * 1024];
    loop {
        let n = input.read(&mut buffer).map_err(ServeError::Io)?;
        if n == 0 {
            return decoder.finish().map_err(ServeError::Frame);
        }
        decoder.push(&buffer[..n]);
        while let Some(payload) = decoder.next_frame().map_err(ServeError::Frame)? {
            output
                .write_all(&respond(&payload))
                .and_then(|()| output.flush())
                .map_err(ServeError::Io)?;
        }
    }
}

/// The result frame for one request payload.
pub fn respond(payload: &[u8]) -> Vec<u8> {
    let result = handle(payload);
    // A page is bounded by the row budget and the id length, far below a frame; were it not, the
    // client still gets exactly one answer, as an error.
    frame::encode(&result.encode_to_vec()).unwrap_or_else(|_| too_large(result.subject))
}

/// The error frame for a result too large to frame. Its payload is a few dozen bytes whatever the
/// subject, so it is framed directly and cannot fail (`the_too_large_frame_decodes`).
fn too_large(subject: Option<wire::evaluate_query_result::Subject>) -> Vec<u8> {
    let payload = wire::EvaluateQueryResult {
        subject,
        outcome: Some(wire::evaluate_query_result::Outcome::Error(wire::Error {
            message: "the page does not fit a frame".to_owned(),
            kind: Some(wire::error::Kind::InternalResponseTooLarge(
                wire::InternalResponseTooLarge {
                    problem: "the page does not fit a frame".to_owned(),
                },
            )),
        })),
    }
    .encode_to_vec();
    // Under 100 bytes: at most a ten-byte id and two short strings.
    let length = payload.len() as u32;
    [length.to_be_bytes().as_slice(), &payload].concat()
}

#[cfg(test)]
mod tests {
    use yata_protocol::frame::decode_all;

    use super::*;

    #[test]
    fn the_too_large_frame_decodes() {
        use wire::evaluate_query_result::Subject;
        for subject in [
            Some(Subject::Id(u64::MAX)),
            Some(Subject::Undecodable(wire::Undecodable {})),
        ] {
            let frames = decode_all(&too_large(subject)).expect("one frame");
            let [payload] = frames.as_slice() else {
                panic!("exactly one frame")
            };
            let result = wire::EvaluateQueryResult::decode(payload.as_slice()).expect("decodes");
            assert_eq!(result.subject, subject);
            assert!(payload.len() < 100);
        }
    }
}
