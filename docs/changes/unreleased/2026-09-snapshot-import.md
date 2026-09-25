# The daemon reads a snapshot file of the first community format

- Date: 2026-09-25
- Area: import
- Affected: developers
- Related: ADR-0030, PRP-0008

## What changed

- `yata-daemon import check <snapshot.json>` imports a file without storing it,
  and reports its souls, the records left out with their reasons, and how many
  souls the game's rules find legal.
- The format is recognised from the file's header. `mumu-snapshot-v1` is the
  first format; a file that no format, or several, recognise is refused.
- Every sub-attribute's strengthening count is required, a file must say it is
  complete, and a boss soul keeps its innate attribute without its value
  (`spec/import-format.md`, "Import").
- `SoulSet::from_name` resolves a set by the name the game shows.

## Compatibility and migration

None: nothing was imported from a file before.

## Evidence

[../../evidence/testing.md](../../evidence/testing.md), "Import".
