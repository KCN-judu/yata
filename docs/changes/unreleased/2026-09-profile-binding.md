# An import of another account is refused; store format 3

- Date: 2026-09-26
- Area: persistence
- Affected: developers
- Related: ADR-0033, ADR-0032

## What changed

- `SnapshotImported` records the account a snapshot states, if it states one.
- A profile's known account is the one named at its creation, else the one of
  its earliest current import that states one. An import stating another account
  is refused as `ProfileMismatch`, and nothing is written. An import stating no
  account is not checked.
- Withdrawing the import that bound a profile frees it to be bound again.
- The store format is 3.

## Compatibility and migration

A format-2 store is refused as retired, like a format-1 store, and a developer's
store is recreated. No released build wrote one.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Fact model and fold" and "The
fact log on SQLite".
