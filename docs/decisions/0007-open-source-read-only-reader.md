---
id: ADR-0007
status: accepted
date: 2026-09-23
area: import
supersedes: []
superseded-by: []
related: [ADR-0006, ADR-0008, ADR-0011, ADR-0016]
---

# ADR-0007: The reader is open source, reads the game from outside without writing to it, and is held to a security baseline

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

The reader, `yata-reader`, is the only part of this project that touches another
process, and the only part that ever runs elevated (ADR-0006). A program that
reads another process's memory, sometimes with administrator rights granted
through UAC, is the most security-sensitive component the application ships.
Three requirements follow from that position, independently of how the reader is
built.

**Users must be able to audit it.** A user who approves a UAC prompt for the
reader is trusting it with their whole system. That trust is reasonable only if
the user, or anyone they rely on, can read the source and check that the binary
they run was built from it. The maintainer requires that the reader's source be
public, so that anyone can check that it does not harm their system.

**It must need no more than reading.** Reading another process's memory can be
done in two ways: from inside, by injecting code into the process and calling
its functions, or from outside, by opening it with read rights and interpreting
its data structures in the reader's own address space. Injection needs write,
operation, and thread-creation rights on the target, and in an emulator a
root-level injection as well. External reading needs read rights alone, and it
is enough for what the reader has to report. Every right the reader does not
hold is a thing it cannot do by mistake or be made to do.

**It must leave nothing behind.** A reader that installs scheduled tasks,
services, or other persistent configuration changes the user's system beyond the
moment of a read, and leaves an elevation path that outlives the prompt the user
saw. A read is a bounded act, and the reader's footprint must be bounded the
same way.

## Decision

1. **The reader repository is open source, licensed `MIT OR Apache-2.0`.** Every
   release is built by CI from that public source. The repository commits no
   binaries (R6 in `spec/reader-security.md`).
2. **Both channels read from outside, without writing to the game.**
   - The desktop channel opens the game with
     `PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ` only, and interprets
     its data structures outside the process.
   - The MuMu channel reads `/proc/<pid>/mem` under a temporary adbd root,
     without ptrace injection.
   - Neither channel injects code, writes to game memory, or calls game
     functions.
3. **The reader and the daemon code that drives it meet
   `spec/reader-security.md`**, requirements R1–R11. Each requirement names its
   check, and those checks run in CI in both repositories.
4. **Data scope** is the user's own account data plus guild information visible
   to that account, including the member list, as the maintainer chose. Nothing
   leaves the machine.

This repository, the application, is public as well
([ADR-0016](0016-public-repository-local-research.md)).

## Alternatives

- **Read by injection** — a library loaded into the game that runs code inside
  it on the desktop, and ptrace injection on MuMu. Rejected: it needs write,
  operation, and thread-creation rights on the game, and a root injection in the
  emulator, to do what external reading does with read rights alone. It also
  increases exposure to the game's anti-cheat.
- **Both techniques, with injection as a fallback.** Rejected: two code paths,
  and the fallback would keep the most dangerous rights in the codebase.
- **A closed-source reader.** Rejected by the maintainer's requirement: users
  cannot be asked to trust an elevated binary they cannot inspect.
- **GPL-3.0 or MPL-2.0.** Offered and not chosen. `MIT OR Apache-2.0` puts no
  barrier on use or audit, and it matches the Rust ecosystem convention.

## Consequences

**Easier.** The reader needs the smallest rights reading can have. There is
nothing in the game to clean up after a crash. An auditor can read one public
repository and rebuild its binaries.

**Harder.** Parsing the game's data from outside ties the reader to the game's
in-memory data layout. When the game changes that layout, the reader has to
follow. The recorded fixtures (`spec/probe-protocol.md`) are what make such
breakage detectable. Publishing the source also publishes exactly how the game
is read, which may prompt the game to change its data layout.

**Where the rules live.** The requirements, their checks, and the residual risks
are `spec/reader-security.md`; the probe-boundary table is in
`architecture/overview.md`.
