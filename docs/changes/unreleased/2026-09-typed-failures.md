# Failures are typed by their code, and the session query is its own call

- Date: 2026-09-25
- Area: process
- Affected: developers, the application
- Related: ADR-0004, ADR-0012, ADR-0026

## What changed

- `core.proto`, core protocol version 1, still reserved:
  - `Error` has no `code` string and no `details` bytes. Its `oneof kind` has
    one case per code, and each case is a debug record of its own type
    (`QueryStaleRevision { scan, current }`, …). The dotted code is the case's
    field name with the first `_` read as `.`.
  - A failure that ends the session is a `SessionFailed`, sent in the
    `session_failure` event, with its own cases. The application's own failures
    are `ClientFailure`, which never crosses the wire, adding
    `client.unexpected`. New `Error` cases: `store.uninitialized` and
    `internal.page_without_soul`.
  - `Warning` has a `oneof kind` in place of its code; `client_outdated` carries
    both versions.
  - The protocol version moves from every `ClientMessage` to
    `OpenSession.client_version`.
  - The session reads pages with `SessionQuery` (a profile, the query, a row
    budget, and a position: `first`, or `next` with the cursor and the scan's
    revision) and receives a `SessionQueryPage` of `SessionRow`s, each a soul
    with its verdict, plus `total` and `revision`. `Query` keeps what is asked
    only: `profile_id`, `page`, and `scan_revision` are gone, `QueryRow.soul`
    and `QueryPage.revision` too. The headless `EvaluateQuery` carries its
    `PageRequest` itself.
  - A profile id is 32 lowercase hexadecimal digits.
  - Removed tags and names are reserved.
- `yata-daemon`: the dotted names are generated from the schema at build time;
  `wire` maps every store, fact-log, and admission error to its case, so no
  crate below the daemon knows a code.
- `app/`: a failure is a `RequestFailure`, `SessionFailure`, or `RaisedFailure`
  holding the generated message. The user reads a cause and a remedy per code,
  chosen by an exhaustive switch; the debug record is shown only in the detail
  view. Daemon health, inventory paging, scheme import, and icon keys are sealed
  types.
- The `error-codes` preflight check reads the codes from the schema's three
  `oneof kind`s and fails on a code spelled as a string outside the schema.

## Compatibility and migration

Nothing to migrate: the schema is a draft, and the committed bindings and the
session recording change with it. A client branches on the `kind` case, never on
a string; there is no code string left to compare.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Core protocol session" and
"The application".
