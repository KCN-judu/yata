# The store records facts, and the inventory is derived from them

- Date: 2026-09-25
- Area: persistence
- Affected: developers
- Related: ADR-0002, ADR-0019

## What changed

- The daemon's store writes commits of typed facts, encoded with
  `crates/yata-daemon/proto/fact.proto`, and replays the whole log into the
  projection when it opens a store.
- A probe `ReadResult` is imported as a zstd-compressed blob named by its
  SHA-256 and one `SnapshotAcquired` commit. A profile's soul inventory is
  derived from its live readings and the user's marks and notes.
- A reading whose soul id is not established is refused with
  `import.unestablished_identity`. The reader establishes no field yet, so every
  real reading is refused for now.
- `yata-daemon log <store.sqlite3>` prints every commit of a store's log in
  readable form. Account ids are shown only as present.

## Compatibility and migration

The store format version stays 1. A store written before this change has an
empty log and opens unchanged. Its projection cache table is still unused.

## Evidence

[../../evidence/testing.md](../../evidence/testing.md), § Fact model and fold, §
Fact codec and blobs, and § The fact log on SQLite.
