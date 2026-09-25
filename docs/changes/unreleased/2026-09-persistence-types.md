# The store and the fact log answer in their own types

- Date: 2026-09-25
- Area: persistence
- Affected: developers
- Related: ADR-0002, ADR-0019, PRP-0006

## What changed

- Each `yata-store` instruction is a type with its own output; instructions that
  land together are a tuple or a list, answered in the same shape. `Digest`,
  `StoreId`, and `Seq` are types, and a `seq` of 0 cannot be built.
- Opening an empty file without asking to create a store is
  `store.uninitialized`, no longer reported as a foreign file.
- Admission parses a reading into typed rows and defects by position. A defect
  names whether a field is unmapped or inherited, and an empty soul id or
  account id is refused by name.
- A command carries the revision it was formed against, and is stale, unchanged,
  or applied. An import always lands.
- `SoulNoted` is written at version 2, where a note is a text or a clearing.
  Version 1 is lifted on load, its empty text becoming a clearing.
- PRP-0006 proposes typing a fact's payload by its envelope.

## Compatibility and migration

The store format version stays 1. A store written before this change opens
unchanged: its `SoulNoted` facts are lifted when they are read.

## Evidence

[../../evidence/testing.md](../../evidence/testing.md), § Store instructions, §
Fact model and fold, § Fact codec and blobs, and § The fact log on SQLite.
