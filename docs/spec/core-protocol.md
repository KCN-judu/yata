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
that is not a `ClientMessage`, is answered with a `session_failed` event
carrying `session.malformed_frame` or `session.malformed_message`, and the
daemon exits. No request id is known for such a frame, so no response can carry
the error.

**Payloads are canonical proto3.** A scalar at its default value is not written.
prost never writes one; the Dart runtime writes any field that was set, so the
Dart client sets only the fields that differ from their default. The two
encoders then produce the same bytes, which the shared session recording checks
in both languages.

**stdout carries frames and nothing else.** Every log line, warning, and panic
message goes to stderr. A stray `println!` in the daemon corrupts the stream,
and the symptom is a parse failure some messages later, which is why this is a
rule rather than a convention.

## Version

`ClientMessage.protocol_version` carries the protocol version the client was
built against. The daemon compares it against its own at session start:

| Client                | Daemon   | Outcome                                                              |
| --------------------- | -------- | -------------------------------------------------------------------- |
| same major, any minor | accepted | session opens                                                        |
| lower major           | accepted | session opens with a warning event naming the oldest supported major |
| higher major          | refused  | `session.protocol_unsupported`, session does not open                |

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

| Category | Message kinds                                                   | Writes | Produces a revision | Carries `base_revision` |
| -------- | --------------------------------------------------------------- | ------ | ------------------- | ----------------------- |
| Query    | `ListProfiles`, `Query`, `GetSoul`, `DecodeSchemeCode`, `Match` | no     | no                  | no                      |
| Command  | `Command`                                                       | yes    | yes, on success     | yes, required           |
| Job      | `RunJob`                                                        | later  | later               | no                      |

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

A query returns a page. `QueryPage` always carries the `revision` the page is
valid at; a page is never returned without the revision it describes.

**The cursor is opaque.** `QueryPage.next_cursor` is a serialized sort-key
continuation produced by the daemon. It is not an offset, not a row number, and
not a SQL `LIMIT`. A client stores it and sends it back unmodified; a client
that parses it is depending on something the protocol does not promise. An
invalid or foreign cursor is `query.malformed_cursor`, refused rather than
approximated. `PageRequest.cursor` is absent on a scan's first page; a present
cursor is one the daemon wrote, and a present empty one is malformed.

**The row budget bounds the response.** `PageRequest.row_budget` states the
maximum rows the daemon may return for that query, chosen by the client. When a
result set exceeds it, the daemon returns a full page and a `next_cursor`; when
the client omits the budget, the daemon applies its default. A page with no
`next_cursor` is the last: there is no second field saying so.

**A page is valid at one revision.** If the projection advances between two
pages of the same scan, the second page is refused with `query.stale_revision`
rather than returned against different data. The client restarts the scan from
the first page. A cursor carries no revision of its own: the revision is the
query's, and a paged scan keeps issuing its original base, as
`Query.scan_revision`, which the first page leaves absent.

In the session, every row of a page carries its soul's values (`QueryRow.soul`)
and the page its `total`, the count of every row the query keeps. The session
and the headless endpoint run a query through the same check and evaluation
(ADR-0026); the row budget's default and ceiling are theirs.

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
projection moved and to what revision. The client then issues a `Query` for what
it displays. Sending the changed value down the event would put a second copy of
the truth in Dart, where it can go stale silently; a revision cannot.

Two events ride the same stream and are _not_ semantic changes: `JobProgress` (a
job is running) and `Warning` (something the client should surface but that did
not change state). Neither advances the revision.

## Errors

Every failure crosses as `Error`:

```text
Error {
  code    : string   // stable, namespaced, machine-readable
  message : string   // human-readable, English, not parsed
  details : bytes    // optional typed payload, serialized protobuf
}
```

- **`code` is the contract.** It is namespaced per area: `session.*`, `query.*`,
  `command.*`, `job.*`, `decode.*`, `import.*`, `store.*`, `internal.*`. Codes
  are renamed only through the alias table in
  [protocol-versions.md](protocol-versions.md), so a client built against an
  older schema keeps decoding.
- **`message` is for humans and is never parsed.** Wording may change in a patch
  release.
- **`details` is optional and typed.** A consumer that understands it renders an
  expert view; a consumer that does not ignores it. The `details` type is named
  by the `code`.
- `internal.*` is a bug in the daemon, not a user-error path. It always carries
  a message naming where the daemon failed.

The Dart side receives an exception carrying the same three fields. Nothing is
re-derived.

The codes the session raises today:

| Code                           | When                                                                         |
| ------------------------------ | ---------------------------------------------------------------------------- |
| `session.protocol_unsupported` | `OpenSession` from a higher major, or with no version                        |
| `session.not_open`             | a request before `OpenSession`                                               |
| `session.already_open`         | a second `OpenSession`                                                       |
| `session.invalid_request_id`   | a request id of zero                                                         |
| `session.unknown_request`      | a request of no kind this daemon knows                                       |
| `session.malformed_frame`      | in `session_failed`: the framing broke                                       |
| `session.malformed_message`    | in `session_failed`: a payload is not a `ClientMessage`                      |
| `query.unknown_profile`        | a profile id the projection does not hold                                    |
| `query.stale_revision`         | a later page of a scan whose revision has moved                              |
| the other `query.*` codes      | the query's check and evaluation refused it ([query.md](query.md), "Errors") |
| `decode.no_input`              | `DecodeSchemeCode` with neither text nor image                               |
| `decode.malformed_text`        | the text is not a scheme code's transport (`scheme-code.md`, § Transport)    |
| `decode.unknown_format`        | the payload is not a scheme code                                             |
| `decode.malformed_layout`      | the payload's header or records do not parse                                 |
| `decode.malformed_scheme`      | a record does not read as a selection                                        |
| `decode.image_invalid`         | the image is not a PNG this reader accepts                                   |
| `decode.no_qr_code`            | the image holds no QR code                                                   |
| `decode.several_qr_codes`      | the image holds more than one                                                |
| `decode.qr_unreadable`         | the QR code does not decode to text                                          |

The warning `session.client_outdated` accompanies an `OpenSession` from a lower
major.

**Client-side codes** are raised by the application for failures only it can
see, and never cross the wire: `client.daemon_not_found`,
`client.daemon_start_failed`, `client.daemon_exited`, `client.timeout`,
`client.protocol_error` (a malformed frame from the daemon, or a response to no
request), and `client.not_connected`. Every code the application can meet has
its own Chinese text; the `error-codes` preflight check reports one that does
not.

## What this page does not define

| Question                                          | Where it belongs                                 |
| ------------------------------------------------- | ------------------------------------------------ |
| the fields of each message, and their tag numbers | the schema file, which is this page's other half |
| the filter, sort, and group expression vocabulary | [query.md](query.md)                             |
| the scoring functions a query returns             | `scoring.md`                                     |
| which commands exist and what each means          | the feature scope, per feature                   |
| the daemon's subcommands and exit codes           | ADR-0004 rule 9, and the toolchain pages         |
| the probe channel                                 | [probe-protocol.md](probe-protocol.md)           |

## Related

- Why the boundary is a process:
  [ADR-0004](../decisions/0004-core-process-boundary.md)
- The probe's wire: [probe-protocol.md](probe-protocol.md)
- Assigned protocol versions: [protocol-versions.md](protocol-versions.md)
- Where the crate lives:
  [../architecture/overview.md](../architecture/overview.md)
