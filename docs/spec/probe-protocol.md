---
kind: spec
status: current
area: import
---

# Probe protocol

The wire between `yata-daemon` and `yata-reader`, the child process that reads
the game's memory. ADR-0006 decides that this channel speaks the same framing
and encoding as the core channel; this page defines its messages.

Everything here is _designed_. Nothing is _implemented_.

**Naming.** The repository and binary are `yata-reader`. The protocol keeps the
name _probe protocol_, and its messages keep the `Probe` prefix (`ProbeMessage`,
`ProbeExport`, `probe.*` error codes): the name describes the channel's role,
and renaming the protocol would touch every accepted record for no change in
meaning.

The probe is the one boundary no CI can exercise against a live game. **The
recording format defined at the bottom of this page is therefore the entire test
strategy**, and it is the reason this protocol has a schema rather than an
informal envelope.

## Who owns what

| Concern                                         | Owner                            | Not                                      |
| ----------------------------------------------- | -------------------------------- | ---------------------------------------- |
| the schema file, and every message in it        | **this repository**              | the probe repository                     |
| frame length, framing codec                     | shared, vendored from here       | independently implemented                |
| memory reading, scanning, offsets, AOB patterns | the probe repository             | this one                                 |
| engine detection, per-engine read strategy      | the probe repository             | this one                                 |
| decoding a message into a domain value          | **this repository**, `yata-core` | the probe                                |
| the decision to trust a reading                 | this repository                  | the probe; it reports, it does not judge |

The probe reports what it read. It does not decide whether a reading is
plausible, complete, or an improvement on the last one — those are domain
judgments and they belong to the pure crates.

## Frame

Identical to [core-protocol.md](core-protocol.md): a 4-byte big-endian length
prefix counting the payload only, then exactly that many bytes of serialized
protobuf. Maximum frame length 16 MiB. A corrupt or oversized prefix is rejected
before allocation.

**The transport is a named pipe the daemon creates** (ADR-0006), elevated or
not: a random per-session name passed on the reader's command line, restricted
to the current user's SID, first instance only, remote clients rejected, and the
connected client's process id checked against the process the daemon started.

**The pipe carries frames and nothing else.** Reader diagnostics, including the
ones that make memory reading debuggable at all, go to the reader's own log file
in its own data directory; an elevated reader has no stderr the daemon can read.
This includes panic output: a reader that panics still owes the daemon a
well-formed `Failed` message before it exits.

**Elevation is detected, not assumed.** The daemon starts the reader unelevated
first. A reader that cannot open the game process sends `Failed` with
`probe.elevation_required` and exits with code 5; the daemon then restarts it
through UAC, at most once per session. A declined prompt becomes
`import.elevation_declined` for the UI, which points to the export file.

The reader exits only when the daemon closes the pipe or sends `Shutdown`. Its
exit codes:

| Code | Meaning                                                                      |
| ---- | ---------------------------------------------------------------------------- |
| 0    | clean shutdown, requested by the daemon                                      |
| 1    | could not attach: the game was not found, or the process could not be opened |
| 2    | the engine was recognised but no read strategy matched it                    |
| 3    | protocol error: the daemon sent a frame the probe could not decode           |
| 4    | internal failure, with the reason in the reader's log file                   |
| 5    | elevation required: the game process is elevated and the reader is not       |

## Message kinds

`ProbeMessage` is a `oneof`. Both directions use the same envelope, so a frame
is self-describing before its payload is parsed.

### Daemon to probe

| Kind          | Purpose                                                                        |
| ------------- | ------------------------------------------------------------------------------ |
| `Handshake`   | opens the session: protocol version, the engine the daemon believes is running |
| `ReadRequest` | read a named region and report it (a snapshot, one soul page, a delta)         |
| `Cancel`      | abandon a request by id; the probe stops at its next checkpoint                |
| `Shutdown`    | exit cleanly; the probe finishes nothing in flight                             |

### Probe to daemon

| Kind           | Purpose                                                                 |
| -------------- | ----------------------------------------------------------------------- |
| `HandshakeAck` | the negotiated version, the engine actually found, the probe's build id |
| `ReadResult`   | the typed records read for one scope, for a request id (ADR-0008)       |
| `Progress`     | how far a request has got, for a request id                             |
| `Failed`       | an error for a request id, or a session-level failure                   |
| `Log`          | a diagnostic the daemon may surface; never a state change               |

`Log` is not a substitute for the reader's log file. It exists for the one case
the file cannot serve: a message the _user_ should eventually see, which the
daemon may translate and surface.

## Request discipline

A request id is chosen by the daemon, unique within the session, never reused.
**Exactly one `ReadResult`, `Failed`, or `Cancel` response per request** —
`Progress` may be emitted any number of times before it, and never after it.

A `Cancel` for a request id the probe has already answered is accepted and
ignored; a `Cancel` for an unknown id is a protocol error. The daemon may cancel
at any time, including before the probe has started; a request cancelled before
it begins produces no `ReadResult` and one `Failed` with code `probe.cancelled`.

Cancellation is cooperative and checkpoints are engine-specific. The probe stops
at its next checkpoint, so a cancel during a long region read returns a `Failed`
after some delay rather than immediately. The daemon must not assume a cancel is
instantaneous, and must not send a second one for the same id.

## Errors

```text
ProbeError {
  code    : string   // stable, `probe.`-prefixed
  message : string   // human-readable, English, never parsed
  details : bytes    // optional typed payload
}
```

The namespace is disjoint from the core channel's on purpose:
`probe.not_attached` and `command.stale_revision` cannot be confused by a reader
or a log filter, even though both are `Error`-shaped. Codes are renamed only
through the alias table in [protocol-versions.md](protocol-versions.md).

The daemon's response to a probe failure is a core-channel error, not a
`ProbeError` passed through. A probe that cannot attach becomes
`import.probe_not_attached` for the UI; the `ProbeError` detail is preserved in
that error's `details`, so an expert view can see the original.

## Read results are typed records

A `ReadResult` carries a message per scope, not opaque bytes (ADR-0008). A soul
record holds the game's raw values as read: game soul id, suit code, star, slot,
level, main attribute, sub-attributes with their roll counts, innate attribute
where present, lock and discard flags. Nothing in it is interpreted. The mapping
from suit code to `SoulSet` and from attribute code to `SoulAttribute` is
decode, and decode is `yata-core`'s. The field list is fixed in the schema file;
the project's documented hypotheses about the game's soul records are the
starting evidence for it.

## Export file

The probe also writes its reading to a file, in a standalone export mode that
needs no daemon (ADR-0008). The file is the proto3 JSON mapping of a
`ProbeExport` message:

```text
ProbeExport {
  protocol_version, probe_build_id, engine,
  channel : DesktopMemory | MumuAdb,
  results : [ReadResult]
}
```

It is the macOS application's only inventory source, and on Windows it doubles
as a portable backup and a documented format for other tools. The version rules
of [protocol-versions.md](protocol-versions.md) apply to it unchanged, and every
field in the probe schema is therefore public: renaming one breaks outside
readers of the JSON as well as the daemon.

The daemon imports a file as a job. Each `ReadResult` is re-serialized as
protobuf and stored as the blob, so a reading has the same digest whether it
arrived by pipe or by file.

An export file is not a recording. A recording is the frame stream and is what
replay tests consume; an export is one reading, stripped of the session around
it.

## Recording format

**A recording is the frame stream the daemon read from the pipe during a
session, captured verbatim on the daemon's side** (ADR-0006). It does not depend
on the transport.

```text
recording := frame*          // the same frames the daemon would have read
```

There is no wrapper, no header, no metadata block, and no re-encoding. Three
consequences follow, and they are the reason for the choice:

- **Replay is the live path.** The fixture is fed to the same decoder the live
  pipe uses, so a test cannot exercise a simulation that drifts from what
  actually arrives.
- **A recording cannot claim to be something it is not.** Its protocol version
  is the frame's, so a recording is never silently reinterpreted under a newer
  schema.
- **A truncated recording is detectable.** It ends mid-frame, which the codec
  reports as a truncated frame rather than as a clean end of stream. A fixture
  that ends cleanly without a `Failed` or a `ReadResult` for a pending request
  is a broken capture.

Recordings live beside the test that consumes them and are named for what they
capture, not for when: `mumu-6star-page-partial.frames`, not
`capture-2026-09-23.frames`. A fixture whose meaning depends on a date is a
fixture that will be kept long after it explains anything.

**Every recording is followed by a `ReadResult` or a `Failed` for each request
in it.** A recording that contains requests the daemon answered in a later
capture is a recording that will fail replay, and that is correct — it is a
broken capture, not a decoder bug.

## Versioning

Two independent versions meet here, and they are separate fields because they
move independently:

| Version          | Set by               | Moves when                                                       |
| ---------------- | -------------------- | ---------------------------------------------------------------- |
| protocol version | this repository      | a message kind or field is added or changed                      |
| probe build id   | the probe repository | the probe's own code changes, including its offsets and patterns |

The daemon records both. When a fixture fails to replay, the pair answers the
first question that matters: _did the wire change, or did the probe?_ A protocol
version mismatch is a schema problem in this repository; a build id change with
the same protocol version means the probe's behaviour moved and the recording is
from a different probe than the one running.

A probe whose protocol major version is older than the daemon's oldest supported
version is refused at handshake with `probe.protocol_unsupported`. The probe
never silently degrades to an older schema.

## What this page does not define

| Question                                       | Where it belongs                                 |
| ---------------------------------------------- | ------------------------------------------------ |
| the tag numbers and field types                | the schema file, which is this page's other half |
| how a message becomes a domain value           | the import spec, and `yata-core`                 |
| what "a soul page" means on the wire           | the schema; its _meaning_ is `souls.md`          |
| the offsets and patterns the probe itself uses | the probe repository                             |
| where a vendored codec copy lives              | `repo/layout.md`                                 |

## Related

- Why this channel uses this wire:
  [ADR-0006](../decisions/0006-reader-channel.md)
- The sibling channel: [core-protocol.md](core-protocol.md)
- Assigned protocol versions: [protocol-versions.md](protocol-versions.md)
- Where the probe sits:
  [../architecture/overview.md](../architecture/overview.md)
