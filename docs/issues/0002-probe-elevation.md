---
id: ISS-0002
state: resolved
area: import
opened: 2026-09-23
resolved-by: [ADR-0006]
related: [ADR-0008]
---

# ISS-0002: The reader must run elevated when the game does, and stdio pipes cannot cross that boundary

## Problem

When the game process runs elevated, only an elevated process can open it for
reading. An unelevated daemon cannot hand its stdio handles to an elevated
child: Windows starts an elevated process through the `runas` verb, and that
path does not inherit handles. A reader spawned with stdio pipes therefore
cannot both read an elevated game and talk to the daemon.

## Why it matters

The desktop memory channel does not work at all in that case, and it is the
primary channel. On Windows, the export file (ADR-0008) is the only route that
still works.

## Current evidence

- Whether the game needs elevation depends on how it was started, so elevation
  must be conditional: an attempt to open the game, and escalation only when
  access is denied.
- A file written by an elevated process cannot be deleted by an unelevated one,
  so any exchange through temporary files breaks under elevation.
- A named pipe created by the daemon, with a security descriptor that grants the
  current user's SID only, can be opened by an elevated process running as the
  same user.

## Candidate answers

- **The daemon starts the reader through `runas` and serves it a named pipe**
  restricted to the current user's SID. The frame format is unchanged and only
  the transport differs. There is one UAC prompt per session, because the reader
  lives for the whole session.
- **The export file only, when elevated.** The user runs the reader's export
  mode as administrator and the application imports the file. It needs no new
  mechanism, but the read is manual.
- **A scheduled task at the highest run level.** No prompt, but it leaves system
  configuration behind, needs rights the user may not have, and brings back the
  temporary-file exchange.

These can be combined: detect first, and escalate only on access denied.

## Dependencies

The reader repository must exist, and a test must establish under which
conditions the current game client runs elevated.

## Resolution

**Resolved 2026-09-23 by [ADR-0006](../decisions/0006-reader-channel.md).** The
maintainer chose the first candidate: detect first, escalate through UAC only on
access denied, over a user-restricted named pipe, with the export file as
fallback.
