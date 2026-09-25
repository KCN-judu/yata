---
id: ADR-0008
status: superseded
date: 2026-09-23
area: import
supersedes: []
superseded-by: [ADR-0030]
related: [ADR-0002, ADR-0004, ADR-0005, ADR-0006, ADR-0007]
---

# ADR-0008: The application runs on Windows and macOS; macOS gets its data from a reader export file

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

The application targets Windows and macOS. Both read channels run only on
Windows: desktop memory reading needs the Windows game client, and the MuMu
channel lives in the same Windows reader. On macOS the application therefore has
no way to read an inventory. Without one, a Mac user gets only the scheme-code
features.

The reader, `yata-reader`, lives in its own repository and speaks to the daemon
over a named pipe (ADR-0006). The data the reader produces does not have to
travel by pipe. A file that holds it can be copied to another machine, kept as a
backup, or read by another tool. That also meets a second need: the reader
exporting its reading as JSON, for use outside this application.

Two constraints come from the other initial decisions:

- **One schema per channel** (ADR-0006). A second, hand-written export format
  would be the drift that the shared schema exists to prevent: two descriptions
  of the same reading that agree only by convention.
- **The consumer defines the contract** (ADR-0006, `architecture/overview.md`).
  This repository owns the probe schema, so it owns the export format too.

## Decision

### Platforms

| Platform    | Application | Read channels        | Export file import |
| ----------- | ----------- | -------------------- | ------------------ |
| Windows x64 | yes         | desktop memory, MuMu | yes                |
| macOS arm64 | yes         | none                 | yes                |

The daemon builds on both. Its reader-spawning module compiles on Windows only
(`cfg(windows)`). On macOS, a request to read from the game is refused with
`import.channel_unavailable` and never reaches a code path that does not exist.
The Flutter application hides the read action where the daemon reports no
channel. It does not decide that from the platform itself.

### The export file

**The reader can write its reading to a file as well as to the pipe.** It has a
standalone export mode (a subcommand of the `yata-reader` binary) that attaches,
reads, and writes one file without a daemon.

**The file is the proto3 JSON mapping of a message in the probe schema.** There
is no second schema.

```text
ProbeExport {
  protocol_version : the probe protocol version the file was written against
  probe_build_id   : as in HandshakeAck
  engine           : the engine the reader found
  channel          : DesktopMemory | MumuAdb
  results          : [ReadResult]            // one per scope read
}
```

The ordinary proto3 JSON rules apply: field names in lowerCamelCase, enums as
names, 64-bit integers as strings. A file is valid if a proto3 JSON parser
accepts it as a `ProbeExport` of a protocol version the daemon accepts, and the
version rules of `spec/protocol-versions.md` apply to it unchanged.

**`ReadResult` carries typed records, not opaque bytes.** For a JSON export to
be readable by anything, each scope's payload must be a structured message in
the probe schema: a soul record carries its game soul id, suit code, star, slot,
level, main attribute, sub-attributes with roll counts, lock and discard flags,
and so on. These are the game's raw values, as the reader read them.
Interpreting them — suit code to `SoulSet`, attribute codes to `SoulAttribute` —
stays in `yata-core`. The reader reports and does not judge.

The export file is a data path, not a way of reading the game. The two-channel
rule in `architecture/overview.md` stands.

### Importing a file

The daemon imports an export file as a job, on either platform:

1. parse the JSON into a `ProbeExport`, refusing a protocol version it does not
   accept or a malformed file with `import.malformed_export`
2. for each `ReadResult`, serialize it as a protobuf message and store those
   bytes as the blob. The digest is over that serialization, so the same reading
   has the same identity whether it arrived by pipe or by file.
3. append one `SnapshotAcquired` per result, with the file's `channel` and
   `probe_build_id`, and with the source recorded as an export file rather than
   a live read

The profile rule applies as it does to a live read: a file whose game account
differs from the target profile's is refused with `import.profile_mismatch`
(`spec/fact-format.md`).

## Alternatives

- **macOS gets scheme codes only.** Rejected: most of the application is about
  the inventory, and a Mac user would get little.
- **A macOS read channel** (MuMu for Mac through ADB). Not rejected; deferred.
  It is a new channel, which needs its own ADR, and the export file covers macOS
  without one.
- **A hand-written JSON format owned by the reader.** Rejected on ADR-0006's
  reasoning: two descriptions of one reading drift, and the producer would own a
  contract the consumer must obey.
- **Export the daemon's store instead of the reader's reading.** Rejected as the
  macOS source: it needs a Windows installation of the whole application, not
  just the reader. A store export may still be worth having for backup; that is
  a separate decision.

## Consequences

**Easier.** A Mac user runs the reader once on a Windows machine, or asks
someone who has one, and imports the file. The same file is a backup, a way to
share an inventory, and a human-readable record of what the reader saw. Other
tools get a documented format for free, because the proto3 JSON mapping is
standard.

**Harder.** The probe schema has to define real record types before the first
export, which means designing the soul record now instead of shipping opaque
bytes. Every field in the probe schema is visible to outside readers of the
JSON, so renaming a field breaks them as well as the daemon. The version ledger
governs this, and the discipline is public. CI has a macOS job that builds the
daemon and runs its tests without the reader-spawning module.

**Where the rules live.** The typed `ReadResult` and the export file are
specified in `spec/probe-protocol.md`, the export source on `SnapshotAcquired`
in `spec/fact-format.md`, the per-platform feature set in
`spec/feature-scope.md`, and the export path in the data-path table of
`architecture/overview.md`. The protocol, its messages, and its error codes keep
the `probe` name (`spec/probe-protocol.md`, "Naming").
