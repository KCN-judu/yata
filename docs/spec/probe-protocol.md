---
kind: spec
status: current
area: import
---

# Probe protocol

The wire between `yata-daemon` and `yata-reader`, the child process that reads
the game's memory. ADR-0006 decides that this channel speaks the same framing
and encoding as the core channel; this page defines its messages.

What of it is implemented and tested is in
[../project/status.md](../project/status.md).

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

The security descriptor is one access-allowed entry for the pipe's owner,
`D:P(A;;GA;;;OW)`: the owner is the user the unelevated daemon runs as, and an
elevated reader of the same user holds that SID. If the descriptor cannot be
built, the pipe is not created (R10). The reader refuses to connect to any name
that is not the daemon's prefix `\\.\pipe\yata-reader-` and 32 lowercase
hexadecimal digits.

**The pipe carries frames and nothing else.** Reader diagnostics, including the
ones that make memory reading debuggable at all, go to the reader's own log file
in its own data directory; an elevated reader has no stderr the daemon can read.
This includes panic output: a reader that panics still owes the daemon a
well-formed `Failed` message before it exits.

**Elevation is detected, not assumed.** The daemon starts the reader unelevated
first. A reader that cannot open the game process sends `Failed` with
`probe.elevation_required` and exits with code 5; the daemon then restarts it
through UAC, at most once per session. A declined prompt becomes
`import.elevation_declined` for the UI, which points to the export file. Before
the elevated start the daemon checks the reader file's SHA-256 against the
expected one (R7); until the release manifest exists (ADR-0011) the expected
hash is given by whoever starts the read, and without one elevation is refused
(`import.elevation_unverified`), never attempted unchecked.

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

## Choosing the process

The reader attaches during the handshake, so `HandshakeAck` names the process it
attached to (`TargetProcess`: pid, file name, parent pid, session, pointer
width, creation time, and never a path, which can carry the user's name).
Choosing is deterministic and never guesses:

- `Handshake.target_pid` other than 0 names the process, whatever its file name;
  a pid that does not exist is `probe.not_found`.
- Otherwise the candidates are the processes whose file name is one of the
  reader's game names. None is `probe.not_found`; more than one is
  `probe.ambiguous_target`, with every candidate in `ProbeError.candidates`,
  ordered by pid, so the user can choose one and the daemon can start a session
  with its pid. Only exactly one candidate is chosen.
- A 32-bit target is `probe.unsupported_environment`: the read strategies are
  for 64-bit processes.

The game names are the reader's, and are a hypothesis until a recording of the
game establishes them.

## Request discipline

A request id is chosen by the daemon, unique within the session, never reused:
each id is above every earlier one. **Exactly one `ReadResult` or `Failed`
response per request** — `Progress` may be emitted any number of times before
it, and never after it. Both peers run the same state machine for this
(`yata-protocol::discipline`): the daemon over the reader's answers, the reader
over the daemon's requests and cancels.

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
`ProbeError` also carries the candidates of a discovery and the operating
system's error number, where there is one.

### Error codes

The failures of attaching are kept apart, because each needs a different answer
from the user. The codes are constants in `yata-protocol` (`probe::code`), and
the exit codes are `probe::exit`.

| Code                            | Meaning                                                                          | Exit |
| ------------------------------- | -------------------------------------------------------------------------------- | ---- |
| `probe.not_found`               | no process matched the discovery rules, or the chosen pid                        | 1    |
| `probe.ambiguous_target`        | more than one process matched; the candidates are in the error                   | 1    |
| `probe.elevation_required`      | the game refused an unelevated reader                                            | 5    |
| `probe.access_denied`           | the game refused an elevated reader                                              | 1    |
| `probe.process_exited`          | the process was gone by the time it was opened                                   | 1    |
| `probe.unsupported_environment` | a host or a target no read strategy covers: not Windows, or a 32-bit target      | 2    |
| `probe.layout_mismatch`         | the target's memory is not a layout the reader reads: a changed or unknown build | 2    |
| `probe.protocol_unsupported`    | the peer's protocol major version is not this one's                              | 3    |
| `probe.protocol_error`          | a frame that does not decode, or a breach of the request discipline              | 3    |
| `probe.internal`                | a failure inside the reader, with the reason in its log file                     | 4    |
| `probe.cancelled`               | a request ended by `Cancel`                                                      | —    |
| `probe.scope_unsupported`       | a request for a scope the reader does not read                                   | —    |
| `probe.not_attached`            | a request with no game attached                                                  | —    |

A code with an exit code ends the reader after its session-level `Failed`
(request id 0); the others answer one request.

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

### Evidence

A typed field is only as good as the layout behind it, so a result says how the
reader knows each one. Every typed field of `SoulRecord` is optional: unset
means the reader did not map it, never zero or false. For each field it fills,
`ReadResult.field_evidence` holds one entry:

| `Evidence`             | Meaning                                                                                  |
| ---------------------- | ---------------------------------------------------------------------------------------- |
| `EVIDENCE_INHERITED`   | a hypothesis taken from the prior tool (ADR-0014), not yet re-established                |
| `EVIDENCE_ESTABLISHED` | re-established by this project's own recording or single-variable test, named in `basis` |

An entry named for the message alone, `SoulRecord`, states the rule that
recognised the objects as souls. The rules for the daemon:

- A typed value with no evidence entry is not mapped, and the daemon ignores it.
- Evidence is carried as the reader states it, into the observation and the
  export, and never raised by the daemon. Only the reader, after a recorded
  experiment, moves a field from inherited to established.
- A field that is mapped and absent from a record means the record lacks it: for
  `innate`, an ordinary soul. A record whose `innate` is not mapped cannot be a
  row, because nothing then says whether it is a boss soul (ADR-0029).

### Observed records

Beside its typed fields, each record carries what the reader saw, verbatim, in
`observed`: the runtime's name for the record's type, the key the record is
stored under in its container when there is one, and every entry of the record
as `RawEntry` pairs of `RawValue`s — null, boolean, 64-bit integer, float, text,
sequences, mappings, and `unread` with the runtime's type name and the reason
for anything else (an unknown kind, the depth limit, unreadable memory, a failed
layout check, an integer beyond 64 bits). Sequences and mappings keep their true
length when the reader's item limit cuts them.

Unknown entries stay here unmapped, so a recording made before a field is
understood can be re-read after it is. The daemon's research commands
(`probe survey`, `group`, `crosstab`, `suit-evidence`) work on these entries by
key, and on the container key as `@container`.

### Suit-code evidence

The scheme research establishes which soul each scheme bit selects, for every
mapped bit; the suit code of each soul is the prior tool's (ADR-0014,
`scheme-code.md`). A reading supplies the missing link, soul ↔ suit code, as
follows:

1. Record a reading of the game (a recording or an export). Find the entry that
   holds the suit value with `probe survey` and `probe group`, and the offset
   between it and the suit code, if any.
2. For souls the maintainer can identify in the game's own interface, write
   attestations: the soul's identity in the reading (its container key, or a
   mapped soul id), a tab, and the scheme soul bit whose soul the game shows.
3. `probe suit-evidence <reading> <identity> <suit> <offset> <attestations>`
   joins them. A bit is _re-established_ when every attested soul of it carries
   its inherited code and no other bit's soul does; _contradicted_ otherwise;
   _unattested_ without an attestation. Attestations that name no soul, several
   souls, or no suit value are reported, not dropped.

The inheritance is retired only when every mapped bit is re-established and no
attestation is left unjoined: one agreeing bit can agree by coincidence, all of
them, one code to one bit, cannot. The experiment and its attestations are
recorded in the local research records (ADR-0016), and the reader then states
`SoulRecord.suit_code` as established, citing it.

## Export file

The probe also writes its reading to a file, in a standalone export mode that
needs no daemon (ADR-0008). The file is the proto3 JSON mapping of a
`ProbeExport` message:

```text
ProbeExport {
  protocol_version, probe_build_id, engine,
  channel     : DesktopMemory | MumuAdb,
  results     : [ReadResult],
  captured_at : RFC 3339, UTC
  target      : TargetProcess
}
```

A file is read the same on every platform: UTF-8, a leading byte-order mark
ignored, line ends immaterial, at most 64 MiB. The protocol version is checked
before the body is parsed, so a file of a future major version is refused as
unsupported, not as malformed. A file of the same major and a newer minor is
accepted: its unknown fields are skipped, and an unknown enum name reads as the
enum's zero value, which the daemon then refuses where a value is required. The
export mode writes a new file and never overwrites one.

It is the macOS application's only inventory source, and on Windows it doubles
as a portable backup and a documented format for other tools. The version rules
of [protocol-versions.md](protocol-versions.md) apply to it unchanged, and every
field in the probe schema is therefore public: renaming one breaks outside
readers of the JSON as well as the daemon.

The daemon imports a file as a job. Each `ReadResult` is re-serialized as
protobuf and stored as the blob, so a reading has the same digest whether it
arrived by pipe or by file. The blob leaves out the request id, which belongs to
the session, not to the reading.

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

There is no wrapper, no header, no metadata block, and no re-encoding. The
daemon writes each byte it reads to the recording before it decodes it; when a
read is retried elevated, the recording is of the final attempt. Three
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
