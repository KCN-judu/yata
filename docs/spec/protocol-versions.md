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

A version is **reserved** now because both schemas are still specification. The
first schema file to land turns its row **current**.

## Core protocol — daemon ↔ Flutter

Schema: `docs/spec/core-protocol.md`. Schema file: not yet written.

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

## Probe protocol — daemon ↔ `yata-reader`

Schema: `docs/spec/probe-protocol.md`. Schema file:
`crates/yata-protocol/proto/probe.proto`, drafted 2026-09-24 and revised
2026-09-25: every typed soul field optional with its stated evidence, observed
records beside them, the target process, and the export's capture time. Version
1 stays reserved until a recording of the game establishes the soul record's
fields; it turns current when both sides build against that file.

| Version | Status   | Date       | What it introduced                                                                          | Refused below |
| ------- | -------- | ---------- | ------------------------------------------------------------------------------------------- | ------------- |
| 1       | reserved | 2026-09-23 | the initial schema: handshake, read requests, cancel, shutdown, results, progress, failures | —             |

**Compatibility rule.** Same rule as the core channel, **plus** the probe build
id, which is a separate field and moves independently. Both are recorded with
every capture, so a fixture that fails to replay answers _did the wire change,
or did the probe?_ from the pair rather than by bisecting commits.

## Version pairs in recordings

A recording carries the protocol version of the frames it contains. It does
**not** carry a version of its own, and it is never relabelled. A fixture
captured under version 1 is replayed against the version 1 decoder, whatever the
current version is.

This is why a retired version stays in this ledger: nothing else in the
repository knows what the bytes in an old fixture mean.

## Error-code aliases

A code is renamed only when its name was actively misleading. Both the old and
the new name decode; only the new one is emitted.

| Channel | Old code | New code | Date | Why renamed |
| ------- | -------- | -------- | ---- | ----------- |
| —       | —        | —        | —    | none yet    |

**Codes are not versioned, only aliased.** A client built against an older
schema keeps working across a rename, which is the property that makes a rename
affordable at all. A code that is _removed_ — rather than renamed — stays in
this table with an empty new-code cell, so a client that receives it can still
say what it was.

## What is not here

| Question                                | Where it belongs                                                             |
| --------------------------------------- | ---------------------------------------------------------------------------- |
| what each message means                 | [core-protocol.md](core-protocol.md), [probe-protocol.md](probe-protocol.md) |
| the app's own version, and the daemon's | the release process, not the wire                                            |
| a parameter set's version               | `scoring.md` — parameters version separately from the wire                   |
| the fact log's record versions          | `fact-format.md` — persistence versions separately from the wire             |

Three versioning stories exist in this project, and they are deliberately
independent: the wire, the parameter sets, and the fact records. Each moves on
its own schedule, and coupling them would mean a scoring weight change forced a
protocol bump.

## Related

- The core wire: [core-protocol.md](core-protocol.md)
- The probe wire: [probe-protocol.md](probe-protocol.md)
- Why there is a wire at all:
  [ADR-0004](../decisions/0004-core-process-boundary.md),
  [ADR-0006](../decisions/0006-reader-channel.md)
