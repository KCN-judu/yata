# The probe channel reads, records, and replays; readings carry their evidence

- Date: 2026-09-25
- Area: import
- Affected: developers, the reader repository
- Related: ADR-0006, ADR-0007, ADR-0008, ADR-0014

## What changed

- `probe.proto`: every typed field of `SoulRecord` is optional, and a reading
  states one `Mapping` per field (inherited or established, with its basis), and
  one for the rule that recognised the records. Each record carries what the
  reader saw, verbatim, as an `ObservedRecord` of raw values. The innate
  attribute is `none` or `present`; sub-attributes are one message, so lacking
  them differs from having none.
- Each shape has one encoding: `Handshake.target` is `pid` or `discover`; a
  `Failed` names a request or the session; error codes are the `ProbeErrorCode`
  enum, with discovery's candidates as a typed detail and the system's error
  number optional; a `ReadResult` is a request id around a `Reading`, and
  `ProbeExport` carries `readings`; a cut sequence or mapping states its full
  length, an uncut one none; an unread value's type name is absent when unknown;
  `TargetProcess` facts are unset when the system did not say, and the pointer
  width is an enum.
- `yata-protocol` reads and writes the export file as proto3 JSON, checking the
  version before the body; names each error code and the exit it ends a reader
  with; and runs the request discipline, over nonzero request ids, as one state
  machine for both peers.
- `yata-core::import`: a reading as typed observations, each field unmapped or
  mapped with its evidence and value, and the research analyses — survey,
  grouping, cross-tabulation, and the suit-code ledger against maintainer
  attestations, by scheme bit.
- The daemon refuses what the schema leaves unstated — coverage, the mappings
  message, a mapping's evidence or basis, an innate reading's case, a raw value,
  an error code, a channel — rather than defaulting it, and refuses what a
  reading contradicts: a value on a field it does not map, an empty account or
  soul id, a failure whose subject disagrees with its code.
- Both peers read failures through one vocabulary in `yata-protocol::failure`: a
  session failure, with a total exit, or a request failure. The daemon elevates
  only on a session failure that asks for it.
- `yata-daemon`: the probe session over any stream, recording every byte it
  reads, and replay through the same decoder and rules; the Windows named pipe
  (owner-only, first instance, no remote clients, client pid checked) and
  detect-then-elevate with the reader's hash checked; the `probe` commands
  `read`, `show`, `decode`, `survey`, `group`, `crosstab`, `suit-evidence`, and
  `to-export`.
- A recording the reader made over synthetic memory is kept as a fixture and
  replayed by the daemon's tests.

## Compatibility and migration

Version 1 of the probe protocol is still reserved, so the schema changed in
place. No recording or export of an earlier draft exists. A fact store written
before a blob became a `Reading` holds blobs this build does not read; before
release, such a store is recreated rather than migrated. The `probe` commands
refuse a reading without records instead of skipping it.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Probe schema" and "Probe
sessions, recordings, and exports".
