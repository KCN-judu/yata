# Scheme codes read as selections of souls

- Date: 2026-09-24
- Area: domain
- Affected: developers
- Related: ADR-0009

## What changed

- `yata-core::scheme::selection` reads a scheme record as a `SoulSelection`, the
  game's filter-panel groups, and writes one back over the record. Every bit it
  does not map, and each field's length, is kept, so an unedited record is
  written back byte for byte.
- `yata-core::scheme::code` reads a whole code as a strengthening scheme set of
  plans or as one or more discard schemes, each a name and a selection.
- `yata-core::scheme::evaluate::matches` says whether the game's filter picks a
  soul: `Matches`, `DoesNotMatch`, or `Undetermined` with the open rules it
  rests on. The empty-group, several-include, and count rules, and the innate
  attribute, are open, so most verdicts are `Undetermined` for now.
- Every bit position is in one table, `yata-core::scheme::mapping`; the research
  tool's bit editor reads it too.
- `Soul` has a `set`, a `SoulSet` identified by the game's suit code.

## Compatibility and migration

Code that builds a `Soul` must give its set. The research commands are
unchanged.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Scheme selection and
evaluation".
