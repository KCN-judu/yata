---
kind: spec
status: current
area: process
---

# Protocol versions

The ledger of assigned protocol versions and retired error codes, for both
wires. Two independent release cadences meet in this project — the probes and
the daemon ship separately — so a version that is assigned and never written
down is a version nobody can diagnose six months later.

`records/governance.md` dropped BDL's ledger with the note _"no wire protocol
yet — add one if an IPC boundary appears."_ Two have now appeared (ADR-0004,
ADR-0006), so this page restores it. Unlike every other page in `docs/spec/`,
this one is **append-only**: a version that has shipped is never edited, and a
code that has been renamed keeps its entry.

## Status vocabulary

| Status         | Means                                                                                     |
| -------------- | ----------------------------------------------------------------------------------------- |
| **reserved**   | the number is claimed by an accepted decision; no schema exists yet                       |
| **current**    | the schema exists and is what both sides build against                                    |
| **superseded** | the schema exists, is still accepted on the wire, and is not what new code builds against |
| **retired**    | the version is refused at handshake; kept here so an old recording can still be explained |

A version is **reserved** until a schema file lands and both sides build against
it; then its row turns **current**.

## Core protocol — daemon ↔ Flutter

Schema: `docs/spec/core-protocol.md`. Schema file:
`crates/yata-protocol/proto/core.proto`, drafted 2026-09-25: the query messages,
the headless query endpoint, and the session the application builds against
(envelopes, subscriptions, profiles, soul pages, scheme-code decoding); then,
the same day, each failure typed by its code — the code a `oneof` case of
`Error`, `SessionFailed`, or `ClientFailure` with a debug record per code — and
the session's query given its own call, `SessionQuery`; then, the same day, the
reader's `import.*` codes replaced by the import failure taxonomy of ADR-0031
(tags 50 to 57; 60 to 64 retired); then, the same day, each profile's
capabilities, derived from the sections it holds (`Profile.capabilities`, tag
3), and `store.retired_format` (tag 90) for a store in a format no build reads;
then, on 2026-09-26, `import.account_mismatch` (tag 58) for a file of another
account than the profile's, and `internal.import_mismatch` (tag 104) for an
import paired with other original bytes. Version 1 stays reserved until commands
and jobs are in the file; until then a tag may still change.

| Version | Status   | Date       | What it introduced                                                                                                        | Refused below |
| ------- | -------- | ---------- | ------------------------------------------------------------------------------------------------------------------------- | ------------- |
| 1       | reserved | 2026-09-23 | the initial schema: frames, the three call categories, pages and cursors, revisions and subscriptions, the error envelope | —             |

**Compatibility rule.** Same major is accepted, any minor. A higher major is
refused at session start with `session.protocol_unsupported`. A minor bump is
additive only: a field or message kind may be added, and the meaning of an
existing tag number may never change.

**Tag numbers are never reused.** A removed field's number is retired, not
recycled, so a recording or a message from an older peer can never be decoded
into the wrong field. The schema file marks retired numbers in a comment.

## Probe protocol — retired

The probe protocol, the wire to the retired reader, never had a current version:
version 1 was reserved on 2026-09-23 and is retired with the protocol (ADR-0030,
ADR-0031, rule 10). No build reads or writes it.

## Snapshot schema — `yata-snapshot`

The IR's schema (ADR-0031). Schema: `docs/spec/snapshot-ir.md`. Schema file:
`crates/yata-protocol/proto/snapshot.proto`. Its version moves only when the
IR's meaning moves, never with scoring parameters.

| Version | Status  | Date       | What it introduced                                                                                                                                     | Refused below |
| ------- | ------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------- |
| 1.0     | current | 2026-09-25 | the header, provenance, the optional account, and five sections — souls, Shikigami, game presets, assets, guild — each absent or with its completeness | —             |

**Compatibility rule.** Same as the core channel: the same major is read, any
minor; a higher major is refused. A newer minor's unknown field or section is
skipped, and an unknown enum value is never reinterpreted (`snapshot-ir.md`).

## Version pairs in recordings

A recording carries the protocol version of the frames it contains. It does
**not** carry a version of its own, and it is never relabelled. A fixture
captured under version 1 is replayed against the version 1 decoder, whatever the
current version is.

This is why a retired version stays in this ledger: nothing else in the
repository knows what the bytes in an old fixture mean.

## Error-code aliases

A code is renamed only when its name was actively misleading. A code is a
`oneof` case, so a rename changes the field's name and keeps its tag: both the
old and the new name decode, and only the new one is emitted.

| Channel | Old code                                 | New code | Date       | Why renamed                                                                                                                                            |
| ------- | ---------------------------------------- | -------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| core    | `import.profile_mismatch` (tag 60)       | removed  | 2026-09-25 | the reader's; a refused write is `command.refused`, and a file of another account is the new `import.account_mismatch` (tag 58), not this code renamed |
| core    | `import.malformed_reading` (tag 61)      | removed  | 2026-09-25 | the reader's (ADR-0030); a file is typed by ADR-0031's taxonomy                                                                                        |
| core    | `import.reading_too_large` (tag 62)      | removed  | 2026-09-25 | the reader's (ADR-0030)                                                                                                                                |
| core    | `import.unestablished_identity` (tag 63) | removed  | 2026-09-25 | the reader's (ADR-0030)                                                                                                                                |
| core    | `import.duplicate_soul` (tag 64)         | removed  | 2026-09-25 | the reader's; an id twice in a file is `import.normalization_failed`                                                                                   |

**Codes are not versioned, only aliased.** A client built against an older
schema keeps working across a rename, which is the property that makes a rename
affordable at all. Each row is one of two cases:

```text
type Alias = Renamed { old: Code, new: Code } | Removed { old: Code }
```

A removed code stays in the table as `Removed`, written `removed` in the
new-code column, so a client that receives it can still say what it was.

## What is not here

| Question                                | Where it belongs                                                 |
| --------------------------------------- | ---------------------------------------------------------------- |
| what each message means                 | [core-protocol.md](core-protocol.md)                             |
| the app's own version, and the daemon's | the release process, not the wire                                |
| a parameter set's version               | `scoring.md` — parameters version separately from the wire       |
| the fact log's record versions          | `fact-format.md` — persistence versions separately from the wire |

Three versioning stories exist in this project, and they are deliberately
independent: the wire, the parameter sets, and the fact records. Each moves on
its own schedule, and coupling them would mean a scoring weight change forced a
protocol bump.

## Related

- The core wire: [core-protocol.md](core-protocol.md)
- Why there is a wire at all:
  [ADR-0004](../decisions/0004-core-process-boundary.md),
  [ADR-0006](../decisions/0006-reader-channel.md)
