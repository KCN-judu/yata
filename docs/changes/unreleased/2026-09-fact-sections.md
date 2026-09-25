# Facts are typed in store format 2, and an import is recorded by its sections

- Date: 2026-09-25
- Area: persistence
- Affected: developers
- Related: ADR-0032, ADR-0031, PRP-0006

## What changed

- `fact.proto` is store format 2: a fact's kind and version are its `oneof`
  case, and its subject is a `oneof` with one case, the profile.
- `SnapshotImported` replaces `SnapshotAcquired`. It names the snapshot's
  canonical encoding and the imported file, both stored as blobs, the source
  format, and each section it holds with its completeness. `SnapshotRetracted`
  withdraws every section of a snapshot.
- The fold keeps imports per profile and per section. An import without a
  section leaves that section as it was. `held` gives what a profile holds of
  each section, for the capability table.
- The soul inventory is derived from the live snapshots through admission.
- The probe module, its commands, its tests and fixture, and the reader's
  observations and evidence analyses are deleted from `yata-daemon` and
  `yata-core`.

## Compatibility and migration

A format-1 store is refused as retired, and a developer's store is recreated. No
released build wrote one. A snapshot imported twice is recorded twice and stored
once.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Fact model and fold", "Fact
codec and blobs", and "The fact log on SQLite".
