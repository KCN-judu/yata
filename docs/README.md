---
kind: project
status: current
area: process
---

# docs/

Front door. Every page in this directory is listed here. If a page is not listed
here, it is a loose page — a defect in the record system.

One authority per fact. Before adding a sentence to a page, check whether it
belongs somewhere else. The requirements every change must meet are
[guides/engineering-requirements.md](guides/engineering-requirements.md).

## Specifications — `spec/`

Current truth about what the system means. Edited in place.

- [spec/glossary.md](spec/glossary.md) — canonical English ↔ CN term table; Soul
  qualifier rules; attribute enums
- [spec/soul-mechanics.md](spec/soul-mechanics.md) — soul rules as inference
  rules: legality, rolls, the official roll and acquisition probabilities, value
  ranges, roll-count inference, display, community predicates; each rule with
  its source
- [spec/scoring.md](spec/scoring.md) — the three scoring functions, parameter
  set, QualityScore, AffinityScore, MatchingResult
- [spec/quality-model.md](spec/quality-model.md) — the quality score of pass 1
  exactly: archetypes, roll units, anchored normalization, tiers N to UR, test
  vectors (`yata-quality-v2`)
- [spec/feature-scope.md](spec/feature-scope.md) — what is in scope, what is
  cut, what is deferred, and why
- [spec/core-protocol.md](spec/core-protocol.md) — the daemon ↔ Flutter wire:
  frames, call categories, pages, revisions, the error envelope
- [spec/probe-protocol.md](spec/probe-protocol.md) — the daemon ↔ probe wire:
  handshake, read requests, cancellation, and the recording format
- [spec/protocol-versions.md](spec/protocol-versions.md) — append-only ledger of
  assigned wire versions and retired error codes
- [spec/scheme-code.md](spec/scheme-code.md) — the SchemeCode / SoulSelection
  model, evaluation semantics, the QR-to-payload codec and its rules, encoder
  stages
- [spec/reader-security.md](spec/reader-security.md) — what the reader may and
  may not do to the user's system, R1–R11, each with its check
- [spec/fact-format.md](spec/fact-format.md) — commits, the fact envelope and
  kinds, blob content addressing, lifting old facts, the projection cache,
  compaction
- [spec/query.md](spec/query.md) — collections, fields, and the filter, sort,
  group, and aggregate vocabulary a core-protocol query accepts
- [spec/release-manifest.md](spec/release-manifest.md) — the draft release
  package names, manifest, checksums, and signing slot of ADR-0011

## Architecture — `architecture/`

How the layers fit together. Edited in place.

- [architecture/overview.md](architecture/overview.md) — the shape, crate graph,
  layer responsibilities, data-exchange paths, what CI can and cannot see

## Decisions — `decisions/`

Why things are the way they are. Append-only once published; supersede, never
rewrite.

- [decisions/0001-pure-core-effects-at-the-boundary.md](decisions/0001-pure-core-effects-at-the-boundary.md)
  — semantic truth in a pure Rust core; effects only at the boundary
- [decisions/0002-fact-log-projection-and-fact-schema.md](decisions/0002-fact-log-projection-and-fact-schema.md)
  — append-only fact log of protobuf records, in-memory projection fold, no
  query language; the store engine is amended by ADR-0019
- [decisions/0003-two-pass-scoring.md](decisions/0003-two-pass-scoring.md) —
  quality pass then affinity/matching pass; why one pass cannot work
- [decisions/0004-core-process-boundary.md](decisions/0004-core-process-boundary.md)
  — the core is a child process; protobuf over length-prefixed stdio frames;
  revision-only subscriptions
- [decisions/0005-crate-boundaries-by-rule.md](decisions/0005-crate-boundaries-by-rule.md)
  — crates exist by split trigger, not by list; three crates to start
- [decisions/0006-reader-channel.md](decisions/0006-reader-channel.md) — the
  reader speaks the core's wire over a user-restricted named pipe; detect, then
  elevate through UAC; recordings
- [decisions/0007-open-source-read-only-reader.md](decisions/0007-open-source-read-only-reader.md)
  — the reader is open source (MIT OR Apache-2.0), reads without writing to the
  game, meets the security baseline
- [decisions/0008-platforms-and-export-file-import.md](decisions/0008-platforms-and-export-file-import.md)
  — Windows and macOS; macOS reads inventory from a reader JSON export file
- [decisions/0009-scheme-code-model-and-format.md](decisions/0009-scheme-code-model-and-format.md)
  — SoulSelection mirrors the game's panel; the game's official format only;
  open bits preserved
- [decisions/0010-data-directory-and-backup.md](decisions/0010-data-directory-and-backup.md)
  — per-user local data directory; self-contained checked backup files;
  automatic backup before format upgrade and compaction
- [decisions/0011-packaging-and-self-update.md](decisions/0011-packaging-and-self-update.md)
  — per-user installer, standalone reader zip, automatic updates checked against
  a signed release manifest
- [decisions/0012-flutter-app-architecture.md](decisions/0012-flutter-app-architecture.md)
  — Flutter engineering: one package, three layers, Riverpod, revision-driven
  invalidation, gen-l10n, errors by code
- [decisions/0013-languages-and-the-python-boundary.md](decisions/0013-languages-and-the-python-boundary.md)
  — shipped code is Rust and Dart; Python only as typed glue in `scripts/`
- [decisions/0014-clean-room-rewrite.md](decisions/0014-clean-room-rewrite.md) —
  nothing is ported from the prior tool; it supplies facts to test, not code
- [decisions/0015-product-name-yata.md](decisions/0015-product-name-yata.md) —
  the product is Yata (八咫镜); crates and repositories take the `yata-` prefix;
  `<app-id>` is `io.github.kcn-judu.yata`
- [decisions/0016-public-repository-local-research.md](decisions/0016-public-repository-local-research.md)
  — the main repository is public from its creation; research records stay local
  in `research/`
- [decisions/0017-icon-source-and-svg-slot.md](decisions/0017-icon-source-and-svg-slot.md)
  — icons keyed by game id, two roles; project SVG, then an untracked local
  pack, then a text mark
- [decisions/0018-license.md](decisions/0018-license.md) — the main repository
  is licensed MIT OR Apache-2.0
- [decisions/0019-sqlite-store-behind-an-instruction-crate.md](decisions/0019-sqlite-store-behind-an-instruction-crate.md)
  — the store is SQLite, reached only through `yata-store`, a pure crate
  translating store instructions into SQL
- [decisions/0020-no-agent-attribution.md](decisions/0020-no-agent-attribution.md)
  — commits and published text carry no agent attribution, in every Yata
  repository
- [decisions/0021-conventional-commits.md](decisions/0021-conventional-commits.md)
  — commit messages follow Conventional Commits 1.0.0, in every Yata repository
- [decisions/0022-constant-scheme-code-segment.md](decisions/0022-constant-scheme-code-segment.md)
  — a generated scheme code carries a constant header segment by default; a
  caller may give another
- [decisions/0023-formal-verification-and-papers.md](decisions/0023-formal-verification-and-papers.md)
  — a public scoring standard is proved in Lean (`formal/lean`), reproduced by a
  calibration program (`formal/calibration`), and explained in a paper
  (`papers/`), all outside the product
- [decisions/0024-quality-model-v1.md](decisions/0024-quality-model-v1.md) —
  Quality Model v1: six-star souls against equal-weight archetypes, anchored at
  zero, the expected soul, and the attainable maximum
- [decisions/0025-ssr-at-one-perfect-line.md](decisions/0025-ssr-at-one-perfect-line.md)
  — Quality Model v1.1: SSR begins at six roll units, one perfect line; SP keeps
  its own floor at `7μ`
- [decisions/0026-query-evaluation.md](decisions/0026-query-evaluation.md) — a
  query keeps a scheme's open verdict by Kleene logic, compares stored values
  with the domain's tolerance, and refuses what it cannot evaluate
- [decisions/0027-quality-model-v2-catalogue.md](decisions/0027-quality-model-v2-catalogue.md)
  — Quality Model v2: output, hit, healing, and a speed archetype gated at four
  roll units; `resist` removed; archetypes of different sizes
- [decisions/0028-pass-2-affinity-contract.md](decisions/0028-pass-2-affinity-contract.md)
  — pass 2 evaluates one soul, one loadout, and one swap; hard requirements gate
  and never score; set use only adds keep suggestions

## Proposals — `proposals/`

Design questions under active consideration. Close with `accepted` (→ ADR) or
`rejected`.

- [proposals/0003-set-and-shikigami-affinity.md](proposals/0003-set-and-shikigami-affinity.md)
  — set affinity and Shikigami affinity as two kinds of pass-2 fit; **accepted**
  as the direction → ADR-0028
- [proposals/0004-pass-2-affinity-contract.md](proposals/0004-pass-2-affinity-contract.md)
  — the pass-2 contract: need profiles, hard gates and soft preferences, set
  plans, and evaluation per soul, per loadout, and per swap; **accepted** →
  ADR-0028
- PRP-0001 and PRP-0002 are kept in local research (ADR-0016).

## Issues — `issues/`

Design questions with unknown answers. Close with `resolved` or `deferred`.

- [issues/0001-official-scheme-code-encoding.md](issues/0001-official-scheme-code-encoding.md)
  — the game's scheme-code encoding; **resolved** by ADR-0009: QR → Base64 →
  zlib → game-defined binary layout
- [issues/0002-probe-elevation.md](issues/0002-probe-elevation.md) — the reader
  needs elevation when the game has it; **resolved** by ADR-0006
- [issues/0003-packaging-and-code-signing.md](issues/0003-packaging-and-code-signing.md)
  — packaging and updates decided by ADR-0011; OS signing and notarization
  **deferred**
- [issues/0004-flutter-app-architecture.md](issues/0004-flutter-app-architecture.md)
  — Flutter architecture and who decides; **resolved** by ADR-0012
- [issues/0005-project-website.md](issues/0005-project-website.md) — project
  website: first a concept showcase (layered paper-cut, 和风/国风, gilded
  accents), design by the UI agent; own download page and user guide entry;
  GitHub Pages from the main repository; timing and build **deferred**
- [issues/0006-soul-and-shikigami-icon-source.md](issues/0006-soul-and-shikigami-icon-source.md)
  — where soul and Shikigami icons come from; **resolved** by ADR-0017
- [issues/0007-scheme-code-encoding-header.md](issues/0007-scheme-code-encoding-header.md)
  — which account's header a generated scheme code carries, since a code with
  the importer's own header is refused; **resolved** by ADR-0022

## Evidence — `evidence/`

Which test backs which claim.

- [evidence/testing.md](evidence/testing.md) — the test suites, and the tests
  behind each implemented rule

## Guides — `guides/`

How to work in this repository. Edited in place.

- [guides/engineering-requirements.md](guides/engineering-requirements.md) — the
  requirements every change must meet: records, naming, architecture, code,
  verification, commits, publication

## Project — `project/`

Current state. Edited in place.

- [project/status.md](project/status.md) — what exists, what does not,
  claim-strength vocabulary
- [project/roadmap.md](project/roadmap.md) — milestone 1 (import, basic UI,
  official filter, advanced filter) and what gates each item
- [project/ci.md](project/ci.md) — what each CI job proves; every job is one
  preflight profile

## Changes — `changes/`

What changed for someone. Append-only; one fragment per change.

- `changes/unreleased/` — fragments not yet tied to a release
- [changes/unreleased/2026-09-scheme-transport-and-qr.md](changes/unreleased/2026-09-scheme-transport-and-qr.md)
  — scheme codes decoded, compared, listed, built for an account, and made into
  QR codes
- [changes/unreleased/2026-09-scheme-selection.md](changes/unreleased/2026-09-scheme-selection.md)
  — scheme codes read as selections of souls; the game filter's verdict
- [changes/unreleased/2026-09-filter-semantics.md](changes/unreleased/2026-09-filter-semantics.md)
  — empty groups, several includes, and 数量 decided from the game's filter
- [changes/unreleased/2026-09-repository-tooling.md](changes/unreleased/2026-09-repository-tooling.md)
  — preflight, formatting, and CI; the license
- [changes/unreleased/2026-09-history-rewrite.md](changes/unreleased/2026-09-history-rewrite.md)
  — `main` rewritten to remove agent attribution; reset old clones
- [changes/unreleased/2026-09-quality-model.md](changes/unreleased/2026-09-quality-model.md)
  — Quality Model v1 defined, proved, and calibrated; not yet implemented
- [changes/unreleased/2026-09-quality-model-v1-1.md](changes/unreleased/2026-09-quality-model-v1-1.md)
  — Quality Model v1.1: the SSR edge moves to one perfect line
- [changes/unreleased/2026-09-quality-model-v2.md](changes/unreleased/2026-09-quality-model-v2.md)
  — Quality Model v2: the catalogue becomes output, hit, healing and speed
- [changes/unreleased/2026-09-query-engine.md](changes/unreleased/2026-09-query-engine.md)
  — typed queries over souls are checked and evaluated; a headless endpoint
- [changes/unreleased/2026-09-innate-attribute.md](changes/unreleased/2026-09-innate-attribute.md)
  — a soul carries its innate attribute, and 固有属性 is evaluated
- [changes/unreleased/2026-09-ci-platforms-and-release.md](changes/unreleased/2026-09-ci-platforms-and-release.md)
  — CI fails on a missing tool, runs per shipping platform, and packages the
  daemon unsigned
- [changes/unreleased/2026-09-reader-acquisition.md](changes/unreleased/2026-09-reader-acquisition.md)
  — the probe channel reads, records, and replays; readings carry their evidence
- [changes/unreleased/2026-09-fact-log.md](changes/unreleased/2026-09-fact-log.md)
  — the store records facts; the soul inventory is derived from them

## Outside `docs/`

- `papers/quality-model-v2/paper.md` and `paper.zh.md` — the public papers
  explaining Quality Model v2, in English and Chinese; explanatory, not
  normative (ADR-0023). `papers/archive/quality-model-v1/paper.md` is the frozen
  record of v1.1
- `formal/lean/` and `formal/calibration/` — the proofs and the program that
  reproduces every number of the quality standard (ADR-0023)

## Local research — not published

Research records live in `research/` beside `docs/` in the working copy and are
excluded from the repository (ADR-0016): notes on the prior tool, the
scheme-code reverse-engineering evidence, the sources behind
`spec/soul-mechanics.md`, and the UI research including PRP-0001 and PRP-0002.
Public records cite them by path in a code span, never by link.
