---
kind: project
status: current
area: process
---

# Project status

What exists right now. Edited in place when something lands. "Implemented" means
present at HEAD with a test that exercises it — nothing weaker.

The repository tooling, the soul rules, and the store are implemented.
Everything else reads _designed_ or _accepted_.

## Status table

| Area                                      | State       | What exists                                                                                         | What does not                                                                                     | Evidence                                                             |
| ----------------------------------------- | ----------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| crates (ADR-0001, ADR-0005)               | implemented | all four crates of the crate table; effect lints on the pure ones; `crate-graph` check              | —                                                                                                 | `crates/*/clippy.toml`, `scripts/preflight.py`                       |
| fact log + projection (ADR-0002)          | accepted    | ADRs, spec/fact-format.md                                                                           | fact codec and lifting, fold, compaction                                                          | —                                                                    |
| store (ADR-0019)                          | tested      | `yata-store` instructions and plans; the SQLite executor; open, create, refuse; `yata-daemon check` | the fact codec, the fold, compaction, backup                                                      | [evidence/testing.md](../evidence/testing.md)                        |
| two-pass scoring (ADR-0003)               | accepted    | ADR, scoring spec                                                                                   | scoring in yata-core                                                                              | —                                                                    |
| frame codec (ADR-0004, ADR-0006)          | tested      | `yata-protocol::frame`: encode, incremental decode, recordings                                      | the Dart mirror; shared fixture files                                                             | [evidence/testing.md](../evidence/testing.md)                        |
| Flutter↔Rust boundary (ADR-0004)          | accepted    | ADR, spec/core-protocol.md                                                                          | the core schema file, generated bindings, the protocol session                                    | —                                                                    |
| probe wire format (ADR-0006)              | implemented | `probe.proto` draft, generated Rust types, round trip through a frame                               | the soul record confirmed by recordings; the JSON mapping of `ProbeExport`                        | `yata-protocol` tests; [evidence/testing.md](../evidence/testing.md) |
| query vocabulary                          | designed    | spec/query.md                                                                                       | query evaluator, schema messages                                                                  | —                                                                    |
| protocol version ledger                   | designed    | spec/protocol-versions.md                                                                           | any assigned version beyond the reserved 1                                                        | —                                                                    |
| scheme code / SoulSelection (ADR-0009)    | accepted    | ADR-0009, spec/scheme-code.md                                                                       | SoulSelection in yata-core, the bitmask ↔ SoulSelection mapping, encode/decode, QR layer          | —                                                                    |
| domain vocabulary                         | tested      | `yata-core::soul`: attributes, slots, `Soul`                                                        | Shikigami, sets, profiles                                                                         | [evidence/testing.md](../evidence/testing.md)                        |
| soul mechanics                            | tested      | W-Soul, M-Main, I-Hits, predicates, roll distribution                                               | acquisition probabilities; ranges below 6★ (TBD)                                                  | [evidence/testing.md](../evidence/testing.md)                        |
| soul import — desktop memory              | designed    | architecture/overview.md                                                                            | import decode in yata-core, probe protocol                                                        | —                                                                    |
| reader transport and elevation (ADR-0006) | accepted    | ADR, spec/probe-protocol.md                                                                         | named-pipe transport, elevation detection, Windows CI job                                         | —                                                                    |
| data directory and backup (ADR-0010)      | accepted    | ADR                                                                                                 | path root, backup writer, six-step restore, automatic triggers                                    | —                                                                    |
| Flutter app architecture (ADR-0012)       | accepted    | ADR                                                                                                 | app/ package, daemon client, Dart frame codec, providers, ARB files                               | —                                                                    |
| platforms, export-file import (ADR-0008)  | accepted    | ADR, spec/probe-protocol.md                                                                         | probe export mode, ProbeExport schema, import job, macOS CI job                                   | —                                                                    |
| soul import — MuMu over ADB               | designed    | architecture/overview.md                                                                            | import decode in yata-core, MuMu path                                                             | —                                                                    |
| soul scoring pass 1 (quality)             | designed    | spec/scoring.md                                                                                     | pass 1 in yata-core                                                                               | —                                                                    |
| soul scoring pass 2 (affinity/match)      | designed    | spec/scoring.md                                                                                     | pass 2 in yata-core                                                                               | —                                                                    |
| feature scope ledger                      | designed    | spec/feature-scope.md                                                                               | —                                                                                                 | —                                                                    |
| multi-profile (GameProfile)               | designed    | glossary.md, feature-scope.md                                                                       | ProfileId in domain, store                                                                        | —                                                                    |
| Item / GameAsset import                   | designed    | glossary.md, feature-scope.md                                                                       | category list (must be sourced), any code                                                         | —                                                                    |
| official scheme code decode               | designed    | research/scheme-code-protocol.md (local; layout mostly reverse-engineered; open bits listed there)  | decoder, encoder, golden corpus fixtures                                                          | —                                                                    |
| packaging and self-update (ADR-0011)      | accepted    | ADR                                                                                                 | installers, release manifest signing, update jobs, apply-on-exit, rollback, a public release feed | —                                                                    |
| icons (ADR-0017)                          | accepted    | ADR; `icon-profile` and `publication` checks                                                        | icon resolver in `app/`, local pack preparation, release check                                    | —                                                                    |
| repository tooling                        | implemented | preflight profiles, docs validator, Markdown formatting, CI workflow                                | a first green CI run                                                                              | `scripts/preflight.py`, `.github/workflows/ci.yml`                   |

## Claim-strength vocabulary

| Word          | Meaning                                     |
| ------------- | ------------------------------------------- |
| `intended`    | on the roadmap, nothing else                |
| `designed`    | a spec or architecture page exists; no code |
| `accepted`    | an ADR is accepted; the decision is final   |
| `implemented` | code exists at HEAD                         |
| `tested`      | implemented + a test exercises it in CI     |

A row in this table may not say `implemented` without an Evidence cell. That is
the enforcement mechanism.

## Related

- Where each record kind lives: [../README.md](../README.md)
- Feature decisions and cut list:
  [../spec/feature-scope.md](../spec/feature-scope.md)
- Architecture and crate map:
  [../architecture/overview.md](../architecture/overview.md)
