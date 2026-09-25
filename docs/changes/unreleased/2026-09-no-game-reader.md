# Yata no longer reads the game; the inventory comes from a file the user supplies

- Date: 2026-09-25
- Area: import
- Affected: users, developers
- Related: ADR-0030, PRP-0008

## What changed

- The project maintains no reader. `yata-reader` is deleted, and nothing in this
  repository builds, starts, downloads, or links to one (ADR-0030).
- `yata-daemon` loses `probe read`, the named pipe, detect-then-elevate, and its
  Windows-only dependencies. The `probe` research commands over recordings and
  export files remain.
- The MuMu channel is removed: `CHANNEL_MUMU_ADB` is reserved in `probe.proto`
  and `fact.proto`, and `Channel` has one case.
- `spec/reader-security.md` is deleted. `spec/import-format.md` describes the
  community snapshot format and a CSV template a user can fill in by hand.
- The importer for that format is not built yet (PRP-0008).

## Compatibility and migration

A fact store that recorded a MuMu reading no longer opens: the channel's value
is reserved, and an unknown channel is refused. Before release, such a store is
recreated rather than migrated. A probe export file of channel MuMu is refused
in the same way.

## Evidence

`cargo clippy --workspace --all-targets` and the workspace tests at the commit
that made the change.
