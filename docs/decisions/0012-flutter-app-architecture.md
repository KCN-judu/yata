---
id: ADR-0012
status: accepted
date: 2026-09-23
area: ui
supersedes: []
superseded-by: []
related: [ADR-0004, ADR-0005, ISS-0004]
---

# ADR-0012: The Flutter application's engineering architecture

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published. It resolves ISS-0004.

## Context

The UI/UX design is handed to a separate agent. ISS-0004 asked whether that
handoff includes the engineering choices on the Flutter side. The maintainer
decided that it does not. Engineering is decided here. Information architecture,
interaction, visual design, and wording belong to the UI agent.

The constraints already in force:

- The Dart side holds no domain model beyond the generated protobuf types
  (ADR-0004).
- It computes no score and decides no domain fact (`architecture/overview.md`,
  layer table).
- A subscription carries a revision, never a value. The UI queries for what it
  shows (`spec/core-protocol.md`).
- A command carries the `base_revision` it was formed against.
- An error's `message` is English and is never parsed. Its `code` is the
  contract.

## Decision

### Who decides what

| Concern                              | Decided by   |
| ------------------------------------ | ------------ |
| package layout                       | ADRs         |
| state management                     | ADRs         |
| the daemon client                    | ADRs         |
| error handling                       | ADRs         |
| the localization mechanism           | ADRs         |
| testing                              | ADRs         |
| information architecture             | the UI agent |
| navigation structure                 | the UI agent |
| layouts                              | the UI agent |
| interaction                          | the UI agent |
| visual design                        | the UI agent |
| the Chinese wording in the ARB files | the UI agent |

The UI agent works inside this architecture. A UI need that the architecture
cannot meet is raised as a change to this record, not worked around in a widget.

### Package layout

**One Flutter package, `app/`, split only by ADR-0005's triggers.** Inside it,
three layers with a one-way dependency:

```text
lib/
  ui/       widgets and screens            → state
  state/    Riverpod providers             → daemon
  daemon/   the daemon client              → generated protobuf types
```

- **`daemon/`** starts `yata-daemon`, speaks the frame codec (a Dart mirror of
  the Rust codec, tested against the same fixtures), correlates responses by
  request id, turns every `Error` into one exception type carrying `code`,
  `message`, and `details`, exposes the subscription events as a stream, and
  restarts the daemon if it exits. When the daemon exits, every request in
  flight fails with a client-side error, and the state layer shows that the core
  restarted.
- **`state/`** holds what the UI shows and nothing it computes.
- **`ui/`** never imports `daemon/` directly. A preflight check enforces the
  import direction.

### State management: Riverpod

- **One provider per query, keyed by its parameters.** A page carries the
  revision it is valid at.
- **Revision-driven invalidation.** A single provider listens to
  `ProjectionChanged` and invalidates the query providers whose data is older
  than the new revision. They refetch when watched. The UI never patches a
  cached value from an event.
- **Paged scans restart on `query.stale_revision`,** from the first page, as the
  protocol requires.
- **Commands take `base_revision` from the latest revision the state layer
  holds.** On `command.stale_revision` the state layer refetches and presents
  the conflict. It never retries silently.
- **Jobs are streams** of progress events ending in exactly one terminal event.
- **No provider derives a domain value.** Sorting and filtering are query
  parameters sent to the daemon, not list operations in Dart.

Riverpod is used without its code generator for now, to keep `build_runner` out
of the build. That is reversible, and adopting the generator later needs no new
ADR.

### Routing

Deferred until the UI agent delivers the information architecture. A desktop
tool is more likely a multi-pane workspace than a stack of pages, and the
routing approach follows from the layout. Milestone 1 uses the simplest
navigation that shows its screens.

### Localization and error text

- **All user-visible text goes through `gen-l10n` ARB files** from the first
  commit. Chinese (`zh`) is the only locale. No user-visible string literal
  appears in a widget.
- **An error is shown by its `code`.** Each error code the UI can meet maps to a
  Chinese ARB message. An unmapped code shows a generic Chinese message with the
  code visible for bug reports. The English `message` is shown only in an expert
  or detail view, never as the main text.

### Tests

Widget tests run against a fake daemon client, as `architecture/overview.md`
states. They assert presentation and interaction only. There is no score to
assert, because the UI has none. The frame codec mirror is tested against the
same recorded frames as the Rust codec.

## Alternatives

- **Hand the whole Flutter side to the UI agent.** Rejected by the maintainer:
  the engineering choices are tied to the protocol's revision and error rules,
  which are decided here.
- **Bloc.** Rejected: more boilerplate, and "invalidate and refetch on revision"
  is less direct than with keyed providers.
- **Flutter's built-in notifiers only.** Rejected: query caching, invalidation,
  and swapping in a fake client for tests would all be written by hand.
- **A separate client package from the start.** Rejected under ADR-0005: no
  trigger holds yet.
- **Hard-coded Chinese strings.** Rejected: the wording is the UI agent's to own
  and revise, and a single ARB file is where it can do that without touching
  widgets.

## Consequences

**Easier.** The UI agent has a fixed frame to design in. Every screen gets
correct staleness handling from the state layer instead of re-implementing it.
Error text is consistent and Chinese by construction.

**Harder.** The Dart frame codec is a second implementation of the Rust one and
must be tested against shared fixtures to stay identical. Every new error code
needs an ARB entry, or the generic message appears. A preflight check can report
codes with no mapping.

The Flutter row of the layer table in `architecture/overview.md` points to this
record.
