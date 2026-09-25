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
use yata_core::query::{CompiledQuery, Page, PageRequest, QueryError, RowVerdict, compile};
use yata_core::scheme::evaluate::OpenRule;
use yata_core::soul::Soul;
use yata_protocol::core as wire;
use yata_protocol::frame::{self, FrameDecoder, FrameError};

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
    ProtocolUnsupported { major: Option<u32> },
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
    /// `query.too_complex`: a budget above [`MAX_ROW_BUDGET`]; `query.malformed` for zero.
    RowBudget { budget: u32 },
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
    /// An attribute on a field that takes none, or none on a field that takes one.
    FieldAttribute { field: i32 },
    /// A selection with `any_set` and suit codes both.
    AnySetWithSets,
    /// An attribute both included and excluded.
    IncludedAndExcluded,
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
    /// The stable code of `core-protocol.md`, "Errors".
    pub fn code(&self) -> &'static str {
        match self {
            RequestError::Undecodable { .. }
            | RequestError::Wire(_)
            | RequestError::BadSoul { .. }
            | RequestError::RepeatedSoulId { .. }
            | RequestError::RowBudget { budget: 0 } => "query.malformed",
            RequestError::ProtocolUnsupported { .. } => "session.protocol_unsupported",
            RequestError::UnknownField { .. } => "query.unknown_field",
            RequestError::InventoryTooLarge { .. } | RequestError::RowBudget { .. } => {
                "query.too_complex"
            }
            RequestError::Query(e) => match e {
                QueryError::TypeMismatch { .. } | QueryError::NotSortable { .. } => {
                    "query.type_mismatch"
                }
                QueryError::ParamSetRequired { .. } => "query.param_set_required",
                QueryError::FieldUnavailable { .. } => "query.field_unavailable",
                QueryError::UnknownScheme(_) => "query.unknown_scheme",
                QueryError::TooComplex { .. } => "query.too_complex",
                QueryError::Malformed(_) => "query.malformed",
                QueryError::MalformedCursor => "query.malformed_cursor",
            },
            RequestError::Panicked => "internal.panic",
        }
    }

    fn to_wire(&self) -> wire::Error {
        let message = match self {
            RequestError::Panicked => "the query handler panicked; see stderr".to_owned(),
            e => format!("{e:?}"),
        };
        wire::Error {
            code: self.code().to_owned(),
            message,
            details: Vec::new(),
        }
    }
}

/// The answer to one request payload. Total: every payload gets exactly one result.
pub fn handle(payload: &[u8]) -> wire::EvaluateQueryResult {
    let (id, outcome) = match wire::EvaluateQuery::decode(payload) {
        Err(e) => (
            0,
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
            (id, outcome)
        }
    };
    wire::EvaluateQueryResult {
        id,
        outcome: Some(match outcome {
            Ok(page) => wire::evaluate_query_result::Outcome::Page(page),
            Err(e) => wire::evaluate_query_result::Outcome::Error(e.to_wire()),
        }),
    }
}

fn evaluate(request: wire::EvaluateQuery) -> Result<wire::QueryPage, RequestError> {
    match request.protocol_version {
        Some(v) if v.major <= wire::VERSION.major => {}
        v => {
            return Err(RequestError::ProtocolUnsupported {
                major: v.map(|v| v.major),
            });
        }
    }
    let query = request
        .query
        .ok_or(RequestError::Wire(WireProblem::Missing(
            "EvaluateQuery.query",
        )))?;
    let prepared = prepare(query)?;
    let souls = convert::inventory(request.inventory)?;
    run(&prepared, &souls)
}

/// A query checked and compiled, with its page request: everything but the souls.
pub struct Prepared {
    compiled: CompiledQuery,
    request: PageRequest<String>,
}

/// Check and compile a wire query. The headless endpoint and the session both start here, so a
/// query means the same whichever way it arrives.
pub fn prepare(query: wire::Query) -> Result<Prepared, RequestError> {
    let page = query.page.clone().unwrap_or_default();
    let budget = page.row_budget.unwrap_or(DEFAULT_ROW_BUDGET);
    let row_budget = NonZeroUsize::new(budget as usize)
        .filter(|_| budget <= MAX_ROW_BUDGET)
        .ok_or(RequestError::RowBudget { budget })?;
    let cursor = match page.cursor.as_slice() {
        [] => None,
        bytes => {
            Some(cursor::decode(bytes).ok_or(RequestError::Query(QueryError::MalformedCursor))?)
        }
    };
    let compiled = compile(convert::query(query)?).map_err(RequestError::Query)?;
    Ok(Prepared {
        compiled,
        request: PageRequest { row_budget, cursor },
    })
}

/// The page a prepared query selects from `souls`, with the core's count of every row it keeps.
/// The rows carry ids and verdicts; a caller that holds the souls adds their values.
pub fn run(
    prepared: &Prepared,
    souls: &BTreeMap<String, Soul>,
) -> Result<wire::QueryPage, RequestError> {
    let page = prepared
        .compiled
        .page(souls, &prepared.request)
        .map_err(RequestError::Query)?;
    Ok(to_wire(page))
}

fn to_wire(page: Page<String>) -> wire::QueryPage {
    let rule = |r: OpenRule| match r {
        OpenRule::Innate => wire::OpenRule::Innate as i32,
        OpenRule::UnknownConditions => wire::OpenRule::UnknownConditions as i32,
    };
    let rows = page
        .rows
        .into_iter()
        .map(|row| wire::QueryRow {
            soul: None,
            soul_id: row.id,
            open_rules: match row.verdict {
                RowVerdict::Open(rules) => rules.iter().map(rule).collect(),
                RowVerdict::Exact => Vec::new(),
            },
        })
        .collect();
    wire::QueryPage {
        rows,
        has_more: page.next.is_some(),
        cursor: page.next.as_ref().map(cursor::encode).unwrap_or_default(),
        revision: 0,
        total: page.total as u64,
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
    frame::encode(&result.encode_to_vec()).unwrap_or_else(|e| {
        let error = wire::EvaluateQueryResult {
            id: result.id,
            outcome: Some(wire::evaluate_query_result::Outcome::Error(wire::Error {
                code: "internal.response_too_large".to_owned(),
                message: format!("{e:?}"),
                details: Vec::new(),
            })),
        };
        // A short error message always fits a frame.
        frame::encode(&error.encode_to_vec()).unwrap_or_default()
    })
}
