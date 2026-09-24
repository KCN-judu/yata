# The probe channel reads, records, and replays; readings carry their evidence

- Date: 2026-09-25
- Area: import
- Affected: developers, the reader repository
- Related: ADR-0006, ADR-0007, ADR-0008, ADR-0014

## What changed

- `probe.proto`: every typed field of `SoulRecord` is optional, and a result
  states the evidence behind each field it fills (`FieldEvidence`: inherited or
  established), and behind the rule that recognised the records. Each record
  carries what the reader saw, verbatim, as an `ObservedRecord` of raw values.
  `Handshake` can name the process; `HandshakeAck` and `ProbeExport` name the
  process read; `ProbeError` carries discovery's candidates and the system's
  error number; `ProbeExport` has its capture time.
- `yata-protocol` reads and writes the export file as proto3 JSON, checking the
  version before the body; holds the probe error and exit codes; and runs the
  request discipline as one state machine for both peers.
- `yata-core::import`: a reading as typed observations with their evidence, and
  the research analyses — survey, grouping, cross-tabulation, and the suit-code
  ledger against maintainer attestations.
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
place. No recording or export of an earlier draft exists.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Probe schema" and "Probe
sessions, recordings, and exports".
