---
kind: spec
status: current
area: process
---

# Core protocol

The wire between the Flutter application and `yata-daemon`. ADR-0004 decides
that the boundary exists and what its discipline is; this page defines the
messages that cross it.

The schema is `crates/yata-protocol/proto/core.proto`, a draft until version 1
is current ([protocol-versions.md](protocol-versions.md)). What of this page is
built is in [status.md](../project/status.md).

## Frame

```text
┌──────────────────────┬────────────────────────────────────────┐
│ length: uint32, BE    │ payload: serialized protobuf message   │
│ 4 bytes               │ exactly `length` bytes                 │
└──────────────────────┴────────────────────────────────────────┘
```

- `length` counts the payload only. It never counts itself.
- **Maximum frame length is 16 MiB** (`1 << 24`). A prefix above it is a
  protocol error, not an allocation request: the reader rejects the frame before
  allocating anything.
- A prefix of zero is a protocol error. An empty payload is not a valid
  `ClientMessage`.
- The payload is one complete `ClientMessage` or `ServerMessage`. Messages are
  never concatenated inside a frame and never split across frames.
- Both directions use the same framing. The codec is one module, mirrored — not
  two implementations that agree today.

**A broken frame ends the session.** A prefix the codec refuses, or a payload
that is not a `ClientMessage`, is answered with a `session_failure` event
carrying a `SessionFailed` of kind `session.malformed_frame` or
`session.malformed_message`, and the daemon exits. No request id is known for
such a frame, so no response can carry the failure ("Errors" below).

**Payloads are canonical proto3.** Whether a field is written depends on its
kind, and both encoders follow the same rule:

| Field kind                                   | Written when                               |
| -------------------------------------------- | ------------------------------------------ |
| scalar or enum with implicit presence        | its value differs from the default         |
| `optional` scalar, message field, oneof case | it is present, whatever its value (even 0) |
| `repeated`                                   | it has at least one element                |

prost follows the rule by construction. The Dart runtime writes any field that
was set, so the Dart client never sets an implicit-presence field to its
default, and sets an explicit-presence field exactly when the value is present.
The two encoders then produce the same bytes, which the shared session recording
checks in both languages.

**stdout carries frames and nothing else.** Every log line, warning, and panic
message goes to stderr. A stray `println!` in the daemon corrupts the stream,
and the symptom is a parse failure some messages later, which is why this is a
rule rather than a convention.

## Version

`OpenSession.client_version` carries the protocol version the client was built
against, once per session. The daemon compares it against its own at session
start:

| Client                | Daemon   | Outcome                                                              |
| --------------------- | -------- | -------------------------------------------------------------------- |
| same major, any minor | accepted | session opens                                                        |
| lower major           | accepted | session opens with a warning event naming the oldest supported major |
| higher major          | refused  | `session.protocol_unsupported`, session does not open                |
| absent                | refused  | `session.protocol_unsupported`, session does not open                |

The version is compared at `OpenSession`, the first request of every session.
Any other request before it is refused with `session.not_open`, and a refused
`OpenSession` leaves the session closed.

A minor version is additive only: a higher minor may add fields and message
kinds, never change the meaning of an existing tag. The ledger of assigned
versions is [protocol-versions.md](protocol-versions.md).

## Requests and responses

Every request carries an `id`, chosen by the client, unique within the session,
and never reused. **Exactly one response is returned per request** — never zero,
never two. A request whose handler fails produces a response carrying `Error`,
not silence.

The transport is asynchronous. The session handler is serial. Two requests may
be in flight; their responses may arrive in any order; the client correlates
them by `id`. In practice the client issues at most one query at a time, because
a query's page is what it renders next.

A response for an unknown `id` is a protocol error and the session is closed.
This is the one case where a silently ignored message would hide a real bug, so
it is fatal instead.

## The three call categories

The category fixes the contract, and the message kind is part of exactly one
category.

| Category | Message kinds                                                          | Writes | Produces a revision | Carries `base_revision` |
| -------- | ---------------------------------------------------------------------- | ------ | ------------------- | ----------------------- |
| Query    | `ListProfiles`, `SessionQuery`, `GetSoul`, `DecodeSchemeCode`, `Match` | no     | no                  | no                      |
| Command  | `Command`                                                              | yes    | yes, on success     | yes, required           |
| Job      | `RunJob`                                                               | later  | later               | no                      |

**Session messages** belong to no category. `OpenSession` opens the session,
`Subscribe` asks for projection changes, and `Shutdown` ends it: the daemon
answers, flushes its output, and exits.

**Queries** compute against the current projection, mutate nothing, and produce
no revision and no event. A query that would write is a command and is rejected
under this kind, not silently allowed.

**Commands** carry the `base_revision` they were formed against. The daemon
compares it to the current revision at the moment it applies the command:

- equal → applied, revision advances, the response carries the new revision
- different → refused with `command.stale_revision`, and the response carries
  both the base the client sent and the current revision, so the client can
  refetch and retry

A command is never applied against a projection it was not formed against. There
is no merge, no last-write-wins, and no partial application: a command either
applies whole or does not apply at all.

**Jobs** are acknowledged immediately with the `job_id` and the expected
progress shape. The outcome arrives later as events on the same session, ending
in exactly one terminal event (`JobSucceeded`, `JobFailed`, or `JobCancelled`).
A job that reaches a terminal state emits nothing further. Job kinds are
declared in `JobRequest`, so a client can tell an unknown job kind from a failed
one.

The commands and job kinds in this schema are the ones that exist today. A new
one is an additive schema change, which is a minor version bump.

## Queries and pages

A query (`query.md`) says what is asked. The call that carries it says where in
a scan the page is, and, in the session, which profile it reads. Each call
carries exactly the position it can use:

```text
Cursor           = { b : bytes | b ≠ [] }           -- opaque: written by the daemon, sent back unmodified
Revision         = { n : u64 }                      -- the seq of the last commit; 0 is the empty log
RowBudget        = { n : u32 | 1 ≤ n ≤ 10 000 }     -- absent: the daemon's default
ProfileId        = 32 lowercase hexadecimal digits  -- one spelling per id

-- the session
record SessionQuery     { profile : ProfileId, query : Query, row_budget : RowBudget?,
                          position : First | Next { cursor : Cursor, scan : Revision } }
record SessionQueryPage { rows : [SessionRow], next : Cursor?, total : u64, revision : Revision }
record SessionRow       { soul : Soul, verdict : Verdict }

-- the headless endpoint: souls supplied, no projection, so no revision
record EvaluateQuery    { id, protocol_version, inventory : [Soul], query : Query,
                          page : { row_budget : RowBudget?, cursor : Cursor? } }
record QueryPage        { rows : [QueryRow], next : Cursor?, total : u64 }
record QueryRow         { soul_id : SoulId, verdict : Verdict }

Verdict          = Exact | Open(NonEmpty<Set<OpenRule>>)
```

**A cursor never travels without its scan.** In the session, `Next` carries the
cursor and the revision the scan's first page was valid at, together; a request
cannot name one without the other, and a `SessionQuery` with no position is
`query.malformed`. If the projection has advanced since, the page is refused
with `query.stale_revision` rather than returned against different data, and the
client restarts from `First`. A cursor carries no revision of its own.

**The cursor is opaque.** It is a serialized sort-key continuation produced by
the daemon, not an offset, not a row number, and not a SQL `LIMIT`. A client
that parses it is depending on something the protocol does not promise. An
invalid or foreign cursor is `query.malformed_cursor`, refused rather than
approximated; a present empty cursor is one.

**The row budget bounds the response.** When a result set exceeds it, the daemon
returns a full page and a `next`; a page with no `next` is the last, and there
is no second field saying so. `total` counts every row the query keeps, exact or
open, across all pages, so every page of a scan reports the same total.

**A session row is its soul.** `SessionRow` carries the soul's values and the
verdict, and the soul's id once, inside the soul; the headless row carries the
id alone, because its caller supplied the souls.

**A revision is always a real one.** A session page carries the revision it is
valid at, and 0 is the empty log's. The headless endpoint has no projection, and
its page has no revision field at all, rather than a 0 that would read as the
empty log.

Both calls run a query through the same check and evaluation (ADR-0026); the row
budget's default and ceiling are theirs.

**Aggregates are not paginated.** `Aggregate` returns counts and sums over the
whole selection in one response, because an aggregate that had to be paged would
be a client-side sum and the client is not allowed to compute one.

## Revisions and subscriptions

**A revision is a monotonic counter of semantic change.** It advances when the
projection changes in a way a user could observe: an import lands, a lock
changes, a parameter set is activated. It does not advance for UI-only changes —
selection, expansion, sort order, column widths, layout.

The distinction is the point. A client that selected a row, changed a sort, or
expanded a panel has changed nothing the daemon knows about, and the daemon
emits nothing. The client learns the revision from whatever it last received: a
response, or a subscription event.

`Subscribe` takes the revisions the client already holds. The daemon then emits
`ProjectionChanged` whenever the projection advances:

| Direction      | Permitted                                       |
| -------------- | ----------------------------------------------- |
| lower → higher | emitted                                         |
| equal → equal  | not emitted                                     |
| higher → lower | not emitted; the daemon does not move backwards |

**A `ProjectionChanged` carries a revision, not a value.** It says _that_ the
projection moved and to what revision. The client then issues a `SessionQuery`
for what it displays. Sending the changed value down the event would put a
second copy of the truth in Dart, where it can go stale silently; a revision
cannot.

Two events ride the same stream and are _not_ semantic changes: `JobProgress` (a
job is running) and `Warning` (something the client should surface but that did
not change state). Neither advances the revision.

## Errors

A failure is one of three messages, chosen by what its receiver does with it:

| Message         | Crosses the wire | Ends                          | Carried by                                         |
| --------------- | ---------------- | ----------------------------- | -------------------------------------------------- |
| `Error`         | yes              | one request                   | `Response.error`, `EvaluateQueryResult.error`      |
| `SessionFailed` | yes              | the session; the daemon exits | the `session_failure` event                        |
| `ClientFailure` | never            | what the application decides  | raised by the application for what only it can see |

```text
Error         { message : string, kind : ErrorKind }          -- one case per code
SessionFailed { message : string, kind : SessionFailedKind }
ClientFailure { kind : ClientFailureKind }
Warning       { message : string, kind : WarningKind }        -- not a failure; no code
```

- **The case is the code.** Each message has one oneof, `kind`. Its field name
  with the first `_` read as `.` is the dotted code that logs and bug reports
  quote: the case `query_stale_revision` is `query.stale_revision`. There is no
  second field naming the code, so a code and its details cannot disagree. The
  namespaces are `session`, `query`, `command`, `job`, `decode`, `import`,
  `store`, `internal`, and `client` for `ClientFailure` alone; no code is a case
  of two messages.
- **Every case carries its own debug record**, a message of the same name
  (`QueryStaleRevision { scan, current }`,
  `SessionProtocolUnsupported { client, daemon }`,
  `ClientTimeout { request_id, limit_ms }`). A record is for developers: it goes
  to logs and the application's detail view, never into the user's text. A code
  with nothing more to say has an empty record.
- **`message` is for developers and is never parsed.** English; its wording may
  change in a patch release.
- **The user reads an explanation, not a code.** For every code the application
  holds a Chinese cause (what happened) and remedy (what to do), chosen by an
  exhaustive match over the generated cases with no default arm, so a code added
  to the schema does not compile in the application until it has both. Codes the
  user acts on alike share one explanation. The explanation and the debug record
  are separate types; neither is derived from the other.
- **An unknown case is kept, not guessed.** A code from a newer peer arrives as
  an unset `kind` with the field's tag in the unknown fields. The application
  shows the generic explanation and the tag.
- `internal.*` is a bug in the daemon, not a user-error path. Its record and
  message name where the daemon failed.

A code is renamed only through the alias table in
[protocol-versions.md](protocol-versions.md). A rename keeps the tag, so a
client built against an older schema keeps decoding it under the old name. The
`error-codes` preflight check reads the cases from the schema and fails on a
code spelled out as a string anywhere else in the daemon or the application.

The codes of `Error` today:

| Code                           | When                                                                               |
| ------------------------------ | ---------------------------------------------------------------------------------- |
| `session.protocol_unsupported` | `OpenSession` from a higher major, or with no version                              |
| `session.not_open`             | a request before `OpenSession`                                                     |
| `session.already_open`         | a second `OpenSession`                                                             |
| `session.invalid_request_id`   | a request id of zero                                                               |
| `session.unknown_request`      | a request of no kind this daemon knows                                             |
| `query.unknown_profile`        | a profile id the projection does not hold, or not spelled as one                   |
| `query.stale_revision`         | a later page of a scan whose revision has moved                                    |
| `query.malformed`              | a `SessionQuery` with no query or no position                                      |
| the other `query.*` codes      | the query's check and evaluation refused it ([query.md](query.md), "Errors")       |
| `decode.no_input`              | `DecodeSchemeCode` with neither text nor image                                     |
| `decode.malformed_text`        | the text is not a scheme code's transport (`scheme-code.md`, § Transport)          |
| `decode.unknown_format`        | the payload is not a scheme code                                                   |
| `decode.malformed_layout`      | the payload's header or records do not parse                                       |
| `decode.malformed_scheme`      | a record does not read as a selection                                              |
| `decode.image_invalid`         | the image is not a PNG this reader accepts                                         |
| `decode.no_qr_code`            | the image holds no QR code                                                         |
| `decode.several_qr_codes`      | the image holds more than one                                                      |
| `decode.qr_unreadable`         | the QR code does not decode to text                                                |
| `import.*`, `command.*`        | the fact log refused a reading or a command ([fact-format.md](fact-format.md))     |
| `store.*`                      | the store could not be opened, read, or written ([fact-format.md](fact-format.md)) |
| `internal.panic`               | a query's evaluation panicked                                                      |
| `internal.qr_too_long`         | a scheme code too long for the largest QR code the daemon draws                    |
| `internal.response_too_large`  | a page that would exceed the frame limit                                           |
| `internal.page_without_soul`   | a session page naming a soul the projection does not hold                          |

The codes of `SessionFailed`: `session.malformed_frame` (the framing broke),
`session.malformed_message` (a payload is not a `ClientMessage`), and
`internal.io` (the daemon could not read or write its pipes).

The codes of `ClientFailure`, which never cross the wire:
`client.daemon_not_found`, `client.daemon_start_failed`, `client.daemon_exited`,
`client.timeout`, `client.protocol_error` (a malformed frame from the daemon, or
a response to no request), `client.not_connected`, and `client.unexpected` (a
failure in the application itself).

The one warning, `client_outdated` with the two versions, accompanies an
`OpenSession` from a lower major. A warning's kind is not a code: it ends
nothing.

## What this page does not define

| Question                                          | Where it belongs                                 |
| ------------------------------------------------- | ------------------------------------------------ |
| the fields of each message, and their tag numbers | the schema file, which is this page's other half |
| the filter, sort, and group expression vocabulary | [query.md](query.md)                             |
| the scoring functions a query returns             | `scoring.md`                                     |
| which commands exist and what each means          | the feature scope, per feature                   |
| the daemon's subcommands and exit codes           | ADR-0004 rule 9, and the toolchain pages         |

## Related

- Why the boundary is a process:
  [ADR-0004](../decisions/0004-core-process-boundary.md)
- Assigned protocol versions: [protocol-versions.md](protocol-versions.md)
- Where the crate lives:
  [../architecture/overview.md](../architecture/overview.md)
