# The daemon reads a snapshot file into the snapshot IR

- Date: 2026-09-25
- Area: import
- Affected: developers
- Related: ADR-0030, PRP-0008

## What changed

- `yata-daemon import check <snapshot.json>` imports a file without storing it,
  and reports its souls, the records left out with their reasons, and how many
  souls the game's rules find legal.
- The format is recognised from the file's header, and each format module
  normalizes into the snapshot IR (ADR-0031): souls, Shikigami, the game's
  presets, assets, and the guild, each absent or present with its completeness.
  `mumu-snapshot-v1` is the first format.
- Admission turns the IR into domain values and names each record it refuses.
  Every sub-attribute's strengthening count is required, a file must say it is
  complete, and a boss soul keeps its innate attribute without its value.
- `SoulSet::from_name` resolves a set by the name the game shows, and owned
  Shikigami are a domain type (`yata-core::shikigami`).

## Compatibility and migration

None: nothing was imported from a file before.

## Evidence

[../../evidence/testing.md](../../evidence/testing.md), "Import".
