---
id: ADR-0004
status: accepted
date: 2026-09-23
area: ui
supersedes: []
superseded-by: []
related: [ADR-0001, ADR-0002, ADR-0003, ADR-0005, ADR-0006, ADR-0012]
---

# ADR-0004: The core is a child process, `yata-daemon`, and Flutter reaches it through a typed protocol on stdio

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

ADR-0001 makes the domain core pure and puts every effect at the boundary. It
does not say how the Flutter application reaches the core, and that choice
decides whether the Flutter heap becomes a second inventory store. A Soul has
four sub-attributes, six slots, a set, a star rating, and two scores with
parameter versions and per-need breakdowns (ADR-0003). Lists of ten thousand of
them are filtered, sorted, and re-scored on parameter changes.

An earlier tool for the same game shows what goes wrong at this boundary. Its
Rust side spoke JSON over IPC to a web front end, and the contract was a large
file of hand-maintained TypeScript interfaces mirroring Rust structs field by
field. Nothing checked that they still matched; a renamed Rust field kept
compiling on the other side and arrived as `undefined` at runtime. Around that
boundary, filtering and pagination were pushed into SQL and derived results were
stored as strings.

Two causes are easy to treat as one. The cost of crossing a JSON boundary is why
someone _tried_ to avoid crossing. The contract being **hand-copied and
unchecked** is what let compensating logic grow on the far side. Avoiding the
crossing would have been harmless if the contract could not silently drift. An
in-process FFI bridge answers the first cause — crossing becomes nearly free —
and the argument for it is that only a cheap crossing lets the core stay the
sole owner of the inventory. The second cause is the decisive one.

The counterexample to that argument is BDL, the project whose engineering
conventions this one is adapted from. BDL ships a Flutter desktop application
whose Rust core is a **separate executable** (`bdld`) that Studio spawns as a
child process, speaking Protocol Buffers over length-prefixed stdio frames. It
has no FFI of any kind: no `flutter_rust_bridge`, no `cbindgen`, no `dart:ffi`,
no `cdylib`. BDL's own ADR-0002 records the choice and its reasoning, and its
Dart layer holds **no domain model classes at all** — _"beyond the protobuf
projection, Studio holds no BDL model classes."_ Crossing there is strictly more
expensive than FFI, and the core still owns everything.

Two further facts narrow the choice:

- **This project already has a child-process boundary.** `yata-reader` is a
  separate executable the daemon reaches over a pipe (ADR-0006). Choosing FFI
  for the UI would not remove that boundary; it would make this project contain
  two kinds of crossing, with different failure modes, different debugging
  stories, and different toolchain constraints.
- **The reader channel cannot be exercised live and is expensive to get wrong.**
  The property a process boundary buys — a crash on the far side does not take
  down the near side — is worth more here than in a compiler IDE, because one of
  the two far sides is reading another program's memory.

## Decision

**The Rust core runs as a separate process, `yata-daemon`, which the Flutter
application spawns as a child. The two speak a versioned protocol over
stdin/stdout.**

Rules the implementation follows:

1. **Crate shape.** The schema, the framing, and the Rust types generated from
   the schema live in `yata-protocol`. The single module that converts domain
   values to wire messages and back lives in `yata-daemon`, because
   `yata-protocol` is also consumed by the reader repository and must not depend
   on the core (ADR-0005). Conversion is directional: domain→wire is total
   (every value renders), wire→domain validates and returns a `Result`, because
   a client can send anything. That module is the only place a domain value
   becomes a message; outside it and `yata-protocol`, nothing in the workspace
   knows the wire exists.
2. **The wire is Protocol Buffers, with one schema file as the single source of
   truth.** Rust bindings are generated at build time with `protox` — pure Rust,
   so no external `protoc` is needed to build the workspace. Dart bindings are
   generated with `protoc` + `protoc-gen-dart`, **committed to the repository,
   and checked byte-for-byte in CI**. The asymmetry is deliberate: the Rust side
   can always regenerate, and the Dart side must not require every contributor
   to have `protoc` installed.
3. **Framing is a 4-byte big-endian length prefix followed by the payload**,
   with one hand-written codec per language that mirrors the other, and a
   maximum frame length enforced on both sides. A corrupt length prefix is
   rejected before it can request an allocation.
4. **stdout carries the protocol and nothing else; all logging goes to stderr.**
   This is the first bug in any stdio protocol, and it is a rule here rather
   than a convention.
5. **Every call is correlated by an explicit request id, and exactly one
   response is returned per request.** The transport is asynchronous; the core's
   session handling is serial.
6. **Calls fall into three categories**, and the category determines the
   contract:
   - **Queries** compute against current state, mutate nothing, and produce no
     revision and no event.
   - **Commands** write, and carry the `base_revision` they were formed against.
     A command whose base is stale is refused with a stable error code, not
     applied.
   - **Long-running jobs** are acknowledged immediately; the outcome arrives
     later as an event.
7. **Subscriptions are emitted only on semantic change, and carry the new
   revision rather than a value.** A notification tells the client _that_ the
   projection moved and to what revision; the client then issues a query for
   what it displays. UI-only changes — selection, expansion, sort order, layout
   — are not semantic changes: they do not advance the revision and emit
   nothing.
8. **Errors cross as one structured envelope**: a stable machine-readable `code`
   (namespaced per area, e.g. `query.stale_revision`), a human-readable
   `message`, and the typed error serialized into a `details` field for an
   expert view. Renamed codes stay decodable through a central alias table. The
   Rust side produces `Result`; the Dart side receives an exception carrying the
   same three fields.
9. **The daemon binary is also the headless entry point.** `yata-daemon serve`
   runs the protocol on stdio; one-shot subcommands (`replay`, `score`, `check`,
   …) run the same session code with no UI. Exit codes distinguish success, a
   failed check, and a failure to open, so CI can gate on them. There is one
   binary rather than a separate CLI, which is what keeps a subcommand and a
   protocol call from drifting apart.
10. **The daemon owns the reader.** The Flutter application never sees
    `yata-reader`; it asks the daemon for a reading. Exactly one child process
    is known to the UI layer.
11. **The UI holds no domain truth.** It may hold: the loaded page, the current
    filter/sort expression, view state, optimistic echoes pending a revision. It
    may not hold: a decoded Soul it built itself, a score it computed or
    interpolated, a set-id→name mapping, a slot decision, an affinity judgment.
12. **Queries return pages**: the rows, a total, an opaque cursor that is a
    domain sort-key continuation rather than an offset, and the revision the
    page is valid at. A row budget bounds the response, so the Flutter heap is
    never the inventory store.

**Where this deliberately differs from BDL.** BDL pushes the full projection
with each change event, which is right when the projection is a design document
and wrong when it is an account's inventory. Rule 7 keeps the revision-only
notification: our projection is ten thousand Souls, and pushing it wholesale per
change is O(inventory) per edit. The page-and-cursor query path is what makes
the refetch bounded. This is the one place the precedent is not followed, and it
is the part most likely to need revisiting under measurement.

## Alternatives

- **`flutter_rust_bridge`** (in-process FFI with generated bindings). Rejected
  because it optimizes the wrong resource. In-process FFI makes a crossing
  nearly free, and pays for it with `unsafe` or an FFI runtime at the boundary;
  a hard abort in the core terminating the UI process; the Flutter↔Rust ABI
  becoming a build constraint that co-evolves with every toolchain bump; and
  nothing reusable by a non-Flutter client. The premise it rests on — that
  crossing cost is what distorts a data model — is contradicted by BDL.
- **A local HTTP or WebSocket server** (the core as a sidecar, Flutter as a
  client). Rejected: a port is a global resource, the surface is reachable by
  anything on the machine and needs authentication to be safe, and the process
  lifetime becomes a thing the user can break by killing the wrong task. All
  three objections are objections to _the network_, not to _the process
  boundary_. A pipe gets crash isolation, language independence, and toolchain
  independence with none of them.
- **`cbindgen` + `dart:ffi` by hand.** Rejected: it moves marshalling from a
  generator to every call site, and puts memory-lifetime decisions in Dart,
  where `cargo test` cannot reach them.
- **`rinf`.** Rejected: the actor model fits the subscription path and fits the
  query path poorly, where the UI wants a typed return value and would have to
  build request/response correlation on top of it.
- **A web-view shell with JSON over IPC and generated types** (Tauri or
  similar). Rejected. Generated types would fix the hand-copied contract, and a
  JSON IPC boundary is simpler to debug than FFI. What it does not fix is that
  JSON-over-IPC has no schema, no generated conversion, and no version
  discipline — so the _shape_ of what crosses stays a matter of convention, and
  convention is what drifts. Protobuf over a pipe is also a boundary with a
  cost; the difference is that its contract is generated, complete, and checked.

## Consequences

**Easier.** A crash, a panic, or a hang in the core cannot corrupt the UI
process: the client observes a closed pipe and reconnects, and the failure has a
name. The core is reusable by everything — `cargo test`, the same binary's
subcommands, CI, and a future front end not written in Dart. The wire contract
is generated from one schema, so a renamed field is a build failure on both
sides rather than an `undefined` at runtime, which makes the hand-copied
contract class of bug structurally impossible. The workspace contains no
`unsafe`, and the process boundary is what makes that enforceable rather than
decorative. And this project's two boundaries have one shape: a child process
reached over a pipe, with the same framing and encoding (ADR-0006).

**Harder.** Every crossing costs a serialization, a copy, a write, a read, and a
parse, so the design must be deliberate about _what_ crosses — which is the
pressure rule 12 answers with pages. A process boundary needs infrastructure
that in-process FFI does not: spawning, locating the binary, stderr discipline,
framing, backpressure, timeouts, cancellation, and restart-on-crash. Development
is two processes, so "which side is wrong" needs a deliberate answer, which is
what the `replay` subcommand is for. And the codegen asymmetry means a schema
change produces a large generated diff that a reviewer has to read as a _schema_
change rather than as a code change.

**Reversible in part.** The three call categories, the revision rule, the error
envelope, and the page shape (rules 5–8, 11, 12) are transport-independent and
would survive a change of wire format. The transport itself (rules 1–3) is
isolated in `yata-protocol`, the conversion module, and the two framing codecs;
replacing protobuf with another typed format, or the pipe with a socket, is a
change there and needs no new decision record.

**Where the details live.** The schema, the frame limits, the error-code
namespace, and the query and page shapes are
[core-protocol.md](../spec/core-protocol.md). Assigned protocol versions and the
error-code alias table are [protocol-versions.md](../spec/protocol-versions.md).
The boundary diagram, the crate graph, and the data-exchange paths are
[overview.md](../architecture/overview.md).
