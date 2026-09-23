---
id: ADR-0006
status: accepted
date: 2026-09-23
area: import
supersedes: []
superseded-by: []
related: [ADR-0004, ADR-0005, ADR-0007, ADR-0008, ISS-0002]
---

# ADR-0006: The reader channel speaks the core channel's wire, over a named pipe the daemon creates

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

The reader, `yata-reader`, is a separate executable in a separate repository
(ADR-0007). The daemon starts it and is the only process that talks to it
(ADR-0004). The core channel between the daemon and the Flutter application is
Protocol Buffers over length-prefixed frames, with generated bindings on both
sides (ADR-0004). The reader channel needs a wire and a transport, and four
facts bound the choice.

**The reader channel is the one boundary no CI can exercise against a live
game.** Recorded sessions are therefore its entire test strategy, and the
recording format is the highest-value contract in the project. It needs
generated types more than any other contract does.

**A producer in a separate repository conforms by reading a document** unless it
is given a schema. The reader repository is the only contributor that must
implement this contract, and without a shared schema it never receives a
compiler error for violating it. A schema file that generates both sides is
worth more here than on the core channel, because here one of the two sides is
in a repository that cannot be reached by a refactor. This is the decisive
argument.

**Data must not travel through temporary files.** Windows Mandatory Integrity
Labels make a file written by an elevated process un-deletable by a
lower-integrity reader, which silently breaks snapshot garbage collection. A
pipe does not have this problem.

**The reader must sometimes run elevated.** When the game runs elevated, only an
elevated process can open it for reading. Windows starts an elevated process
through the `runas` verb, and a process started that way does not inherit
handles from its parent. The daemon therefore cannot hand an elevated reader its
stdin and stdout: a transport over inherited stdio stops working exactly when
the game is elevated. The maintainer chose to detect first, escalate through UAC
only when needed, and keep the export file (ADR-0008) as the fallback.

## Decision

**The reader speaks the same wire as the core: Protocol Buffers over a 4-byte
big-endian length prefix, in both directions. The daemon and the reader always
talk over a named pipe the daemon creates, elevated or not.** One wire, one
transport, one code path, one thing to test.

### Wire

1. **One schema file, one message family.** `docs/spec/probe-protocol.md`
   defines `ProbeMessage` as a `oneof` over the message kinds. The core
   channel's schema is a separate file with a separate `package`, and no message
   is reused across the two, so a change to one cannot silently change the
   other.
2. **Rust bindings for the probe schema are generated with `protox`**, in the
   reader repository's build and in this one's — the same pure-Rust toolchain as
   the core channel, so neither repository needs an external `protoc`.
3. **Framing is the core channel's codec, unchanged**: a 4-byte big-endian
   length prefix, a maximum frame length of 16 MiB enforced on both sides, a
   corrupt prefix rejected before it can request an allocation. The one codec is
   shared as a small vendored module rather than reimplemented, so the two
   repositories cannot disagree about how a frame is read.
4. **The request/response discipline is the core channel's**: explicit request
   ids, exactly one response per request, three categories (a read job, a
   control command, a progress event), and one structured error envelope with
   stable namespaced codes (`probe.` prefix, distinct from `query.` and
   `command.` on the core channel).
5. **The core defines the contract; the reader conforms.** The schema file lives
   in this repository, and the reader repository consumes a pinned copy. A
   reader-side need for a new message is a change to this repository first.

### Transport

6. **The daemon creates the pipe before starting the reader.** The name is
   random per session. The pipe is created as the first instance, rejects remote
   clients, and has a security descriptor that grants access to the current
   user's SID only. An elevated process running as the same user has that SID,
   so it can connect.
7. **The daemon passes the pipe name on the reader's command line** and nothing
   else. The reader connects as the only client. The daemon checks that the
   connected client's process id is the process it started, and closes the pipe
   otherwise.
8. **The pipe carries frames and nothing else.** A reader that writes a
   diagnostic into the stream corrupts it, and that is the failure mode hardest
   to reproduce. Diagnostics go to the reader's own log file, in the reader's
   own data directory, written and rotated by the reader; an elevated reader has
   no stderr the daemon can read. `Log` messages carry only what the user should
   see.
9. **Nothing is written to a temporary file for the daemon to read**, at any
   privilege level.

### Elevation

10. **Detect, then escalate.** The daemon first starts the reader unelevated. If
    the reader cannot open the game, it sends `Failed` with
    `probe.elevation_required` and exits with code 5. The daemon then starts it
    again through `runas`, which shows one UAC prompt. The reader lives for the
    rest of the session, so the prompt appears at most once per session.
11. **A declined prompt is not an error in the reader.** The daemon reports
    `import.elevation_declined` to the UI, and the UI points to the export file:
    run the reader's export mode as administrator and import the file.

### Recordings

12. **A recording is captured on the daemon's side:** the frames the daemon read
    from the pipe, verbatim, with no re-encoding and no envelope of its own. The
    replayer feeds those bytes to the same decoder the live path uses. Nothing
    distinguishes a recording from a live reader except the process on the other
    end, and the capture does not depend on the transport.
13. **The recording format is versioned as part of the wire, not separately.** A
    recording carries the protocol version it was captured under and is replayed
    against the decoder for that version; a fixture is never silently
    reinterpreted under a newer schema. This is what lets a fixture recorded
    today stay testable after the game patches and the reader changes.

## Alternatives

- **Line-delimited JSON for the reader channel.** Rejected. Its one advantage —
  a recording can be read with `cat` — is not worth the property that matters: a
  producer in a separate repository cannot be handed a compiler error. The two
  channels would also diverge in framing, error handling, and versioning
  discipline, and every divergence is a thing a contributor has to learn twice.
- **Protobuf for the data messages, JSON lines for progress and control.**
  Rejected: two encodings on one pipe means a reader of the stream cannot know
  which it is looking at without a discriminator that is itself an encoding
  decision, and the recording format becomes a concatenation of two formats. The
  whole value of one codec is that there is one.
- **An IDL other than protobuf** (FlatBuffers, Cap'n Proto, `serde`-based schema
  generation). Rejected: protobuf is the core channel's format and the format
  BDL's precedent uses; a second IDL in a two-process system is a second
  toolchain for no additional expressiveness.
- **Generate the reader's bindings here and ship them, rather than pinning a
  copy.** Rejected for now, and this is the closest call. A pinned copy can
  drift from the schema here; a generated-and-shipped artifact needs a release
  mechanism between two repositories with different cadences. The mitigation is
  rule 13: the version is in the recording, so a drift is detectable rather than
  silent. If drift becomes a real failure, this is the choice to revisit, and it
  is a change to how the schema is distributed, not to the wire.
- **The reader's inherited stdin and stdout.** Rejected: an elevated process
  started through `runas` does not inherit them, so the transport fails exactly
  when the game is elevated.
- **Stdio when unelevated, a named pipe only when elevated.** Rejected: two
  transports means two code paths, and the elevated one is the one that is
  hardest to test.
- **A shared memory region.** Rejected: it needs framing of its own beside the
  shared codec and buys nothing the pipe does not already give.
- **A scheduled task at `HIGHEST` to run the reader elevated.** Rejected: it
  leaves system configuration behind, needs rights the user may not have, and
  exchanges the result through a temporary file.
- **Relaunch the whole application elevated.** Rejected: it puts the UI, the
  daemon, and the store at the highest privilege to serve one read, and files
  the elevated daemon writes become the integrity-label problem in the user's
  own data directory.
- **The export file as the only elevated path.** Rejected as the primary route,
  kept as the fallback: it turns every read into a manual step whenever the game
  is elevated.

## Consequences

**Easier.** The reader is the third implementation of one wire rather than the
second implementation of a different one, so there is one framing codec, one
error envelope shape, and one versioning story to explain. Golden fixtures have
a real format: a recorded session is a byte stream the live decoder accepts, so
the replay test is not a simulation of the live path but the live path with a
different writer. A new message kind is a schema change both sides notice at
build time, most importantly the reader. The elevated case works with one prompt
per session and no leftover system state, and the unelevated case uses the same
code, so it tests the elevated path's transport as well.

**Harder.** The reader repository needs the codec module and the generated
bindings, so a wire change takes effect there only after the schema is pinned
and regenerated. Recordings are not readable by eye; debugging a capture needs a
decode step, which is why a decode subcommand on the daemon is worth having
rather than a one-off script. The pinned copy is a drift risk that is mitigated,
not eliminated. The daemon owns Windows-specific pipe creation and
security-descriptor code, behind `cfg(windows)`, and it must be exercised in a
Windows CI job. Reader diagnostics are in a separate file the user may need to
send with a bug report, and the reader has a second exit reason to handle.

**Reversible in part.** The wire format is isolated behind the shared codec, as
on the core channel. The frames-only pipe, request ids, recording-as-bytes, and
versioned recordings are format-independent and would survive a change of
encoding.

**Not covered here.** This record adds no way of reading the game; the two read
channels are the desktop memory channel and the MuMu channel (ADR-0007). The
message kinds, their fields, the tag numbers, the exit codes, and the version
constants are specification, in `docs/spec/probe-protocol.md` and
`docs/spec/protocol-versions.md`. The pipe's security requirements are R10 in
`docs/spec/reader-security.md`. Where the vendored codec copy lives is a
repository-layout question, not a decision.
