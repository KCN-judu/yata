---
kind: spec
status: current
area: import
---

# Reader security baseline

What `yata-reader` and the daemon code that drives it may and may not do to the
user's system. Each requirement closes one way a reader of another process's
memory could harm the machine it runs on or hide what it does. The decision to
hold the reader to this baseline and to publish its source is
[ADR-0007](../decisions/0007-open-source-read-only-reader.md).

Each requirement names the check that enforces it; a requirement without a check
is a defect in this page. Which checks exist is in the status pages of this
repository and of the reader's.

## Privilege

**R1 — Only the reader is ever elevated, and only when needed.** The Flutter
application and the daemon never run elevated. The reader is started unelevated
and escalates through UAC only after access to the game is denied (ADR-0006).
Nothing creates scheduled tasks, services, drivers, autorun entries, or registry
values, or changes firewall, Defender, or security policy settings, at any
privilege level. _Check:_ a preflight search of the reader and daemon sources
for the APIs and commands involved (`schtasks`, `CreateService`, `RegSetValue*`,
`RegCreateKey*`, `netsh`, `MpPreference`, and similar).

**R2 — The desktop channel opens the game read-only.** Access rights are exactly
`PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ`. The reader never requests
write, operation, thread-creation, terminate, or handle-duplication rights,
never enables `SeDebugPrivilege`, never injects code, never writes to the game's
memory, and never calls a function inside the game. It reads with
`ReadProcessMemory` and interprets the game's data structures outside the game.
_Check:_ the access mask is one constant with a unit test asserting its value,
and the Win32 API features enabled for the reader's `windows` dependency exclude
the write and remote-thread APIs, so they cannot be called at all.

**R3 — The MuMu channel is read-only inside the emulator and never elevated on
the host.**

- It switches the emulator's adbd to root only for the duration of a read.
- It reads the game's memory through `/proc/<pid>/maps` and `/proc/<pid>/mem`.
  It does not attach with ptrace, load a library into the game, or leave
  anything running.
- Any helper it pushes goes under `/data/local/tmp/yata-reader-*` and is deleted
  after the read.
- It never runs `setenforce`, never remounts, never writes `/system`, and never
  changes emulator settings.
- If a previous run died midway, the next start deletes leftover `yata-reader-*`
  files and restores adbd to unrooted.
- On the host this channel needs no administrator rights and never runs
  elevated.

_Check:_ the device command strings are constants in one module with tests, and
the startup cleanup has a test.

**R4 — No command is built from untrusted text.** Text read from the emulator or
the game enters a shell command only after validation. A PID must be digits
only. A package name must match `[A-Za-z0-9._]+`. Anything else refuses the
read. _Check:_ unit tests with hostile inputs.

## Code provenance

**R5 — Elevated or pushed code comes only from the reader's own installation.**
In release builds, executables and libraries are loaded or pushed by absolute
path from the reader's own directory. No environment variable, `PATH` lookup,
parent-directory search, or running-process path chooses them. DLL loading is
restricted to the application directory and System32 at process start.
Debug-only overrides are compiled out of release builds. _Check:_ a preflight
search for `env::var` in the reader's release code paths, plus a test that
release builds reject override variables.

**R6 — Every shipped binary is built by CI from public source.** The reader
repository commits no binaries. Release builds use pinned toolchains, and the
release manifest (ADR-0011) records each file's SHA-256 and the commit it was
built from. Anyone can rebuild a release and compare hashes. _Check:_ CI fails
on any committed executable or library file.

**R7 — The daemon checks the reader before elevating it.** Before starting the
reader through UAC, the daemon verifies the reader binary's hash against the
installed release manifest, and refuses to start a mismatch.

_Residual risk, stated plainly:_ the per-user install directory is writable by
the user, so malware already running as the user could swap the file between the
check and the launch. The requirement guarantees only that this application
never creates an elevation path easier than the UAC prompt the user sees. It
cannot protect a user whose account is already compromised.

## Data and I/O

**R8 — No network.** The reader has no networking code. Its only sockets are the
named pipe to the daemon and, for MuMu, the loopback connection to the adb
server. _Check:_ `cargo-deny` bans HTTP, TLS, and general networking crates in
the reader.

**R9 — The elevated reader writes almost nothing.** It writes its own log files
in its own data directory and, in export mode, the one output file the user
named. It creates no temporary files for anyone else to read, and nothing in a
shared directory.

**R10 — The pipe fails closed.** The pipe is restricted to the current user's
SID, is the first instance, rejects remote clients, and is accepted only from
the process the daemon started (ADR-0006). If the security descriptor cannot be
built, the daemon refuses to create the pipe. It never falls back to default
permissions. _Check:_ a Windows CI test that a second user's token cannot
connect.

**R11 — Data scope.** The reader reads the user's own account data: souls,
items, realm cards, and similar. It also reads guild information visible to that
account, including the member list (a maintainer decision, ADR-0007). Everything
read stays on the machine (R8). It reads nothing else and changes nothing in the
game.

## Related

- Why the reader is open source and held to this baseline:
  [ADR-0007](../decisions/0007-open-source-read-only-reader.md)
- Transport and elevation: [ADR-0006](../decisions/0006-reader-channel.md)
- Release manifest: [ADR-0011](../decisions/0011-packaging-and-self-update.md)
- The wire the reader speaks: [probe-protocol.md](probe-protocol.md)
