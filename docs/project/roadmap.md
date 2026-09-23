---
kind: project
status: current
area: process
---

# Roadmap

What is planned next, in order, and what gates each item. The roadmap sets
priorities only and is never evidence that anything exists; `status.md` says
what exists. A completed row is deleted, and status and change records take
over.

## Milestone 1 — import an inventory and filter it

Goal: a user imports their souls, sees them in a basic UI, and filters them with
the same logic as the game's own soul filter. Our scoring-based conditions come
on top of that as an advanced filter.

| #   | Item                                                                                                                                                                                           | Gated on                                                                                    |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| 2   | Probe schema: confirm or correct the drafted soul record (`crates/yata-protocol/proto/probe.proto`) against the reader's first recordings; the proto3 JSON mapping of `ProbeExport` (ADR-0008) | recordings from item 3                                                                      |
| 3   | Reader repository (`yata-reader`, open source): external read-only desktop channel and export mode, meeting `reader-security.md`                                                               | 2; ADR-0006, ADR-0007                                                                       |
| 4   | Import in the daemon: export file first, then the live pipe; the SQLite store through `yata-store` and the fact log (`fact-format.md`, ADR-0002, ADR-0019)                                     | 2; the live pipe also on 3 and ISS-0002                                                     |
| 5   | Core protocol schema and Dart bindings: profiles, import job, soul query (`core-protocol.md`, `query.md`)                                                                                      | 1                                                                                           |
| 6   | Basic UI: profile, import, soul list                                                                                                                                                           | 5; ADR-0012; the UI agent, working from PRP-0002 (local research); icons by ADR-0017        |
| 7   | **Official filter**: the soul filter uses the game's own condition model, `SoulSelection` and `matches` (`scheme-code.md`), not a model of our own                                             | the three open evaluation questions in `scheme-code.md`, which must be answered in the game |
| 8   | **Advanced filter**: score-based conditions over `quality.*` and `affinity(...)` (`query.md`), layered on the official filter                                                                  | `scoring.md`: weights, formula, need profiles                                               |

Items 7 and 8 split on purpose. The official filter has to behave exactly like
the game's, so that a filter built in this application and a scheme applied in
the game select the same souls. The advanced filter is where this project adds
what the game lacks, and it is labelled as ours in the UI.

## After milestone 1

In no committed order:

- scheme code import and export through QR (`scheme-code.md`, encoder stage 1)
- MuMu channel over ADB, read-only (ADR-0007)
- the two scoring passes in full, including matching (`scoring.md`)
- packaging and self-update (ADR-0011); a public release feed and release-key
  rotation
- OS code signing and notarization (ISS-0003), before the first public release
- a project website (ISS-0005), no later than the public release feed

## Related

- What exists: [status.md](status.md)
- Scope ledger: [../spec/feature-scope.md](../spec/feature-scope.md)
