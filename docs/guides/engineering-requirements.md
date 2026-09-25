---
kind: guides
status: current
area: process
---

# Engineering requirements

What every change to this repository must satisfy. This page states requirements
only. The detailed conventions behind them are maintained separately and will be
opened to contributors when the project accepts outside contributions.

## Records

- Each kind of fact has one home: what the system means in `docs/spec/`, how it
  is built in `docs/architecture/`, why a choice was made in `docs/decisions/`,
  what exists in `docs/project/status.md`, what changed for someone in
  `docs/changes/`. A fact is written once.
- A decision record is append-only. A changed decision gets a new record that
  supersedes the old one.
- Claims use exactly these words: _intended_, _designed_, _accepted_,
  _implemented_ (present at HEAD), _tested_ (a named test exercises it).
  "Implemented" without a test is not claimed.
- Every page in `docs/` carries its header and is listed in `docs/README.md`.
  Relative links resolve.
- Game rules are specified as inference rules in `docs/spec/`, with every
  unknown marked TBD. Game data is never guessed.

## Language and naming

- Records, identifiers, commit messages, file names, and log targets are
  English. User-facing text is Chinese.
- File names are English kebab-case; no pinyin.
- Terms follow the game, through `docs/spec/glossary.md`. A term that collides
  with a language or framework name takes the `Soul` qualifier.
- Product identifiers take the `yata` prefix (ADR-0015).

## Architecture

- Domain truth lives in the pure Rust core. The core performs no I/O and reads
  no clock; every effect enters at the boundary (ADR-0001).
- The Flutter application holds no domain rule and computes no score (ADR-0004,
  ADR-0012).
- A crate exists only when a split trigger holds (ADR-0005). Workspace lints are
  inherited and never overridden; `unsafe` is forbidden in this workspace.
- Shipped code is Rust and Dart. Python is typed tooling in `scripts/` only
  (ADR-0013).
- Nothing from the prior tool is ported (ADR-0014).

## Modelling

Correctness by construction is the first means of modelling. A requirement that
a type can express is expressed in the type, before any runtime check, test, or
comment.

- Alternatives are sum types whose cases carry their own data, not flags beside
  optional fields. A value the domain does not admit cannot be built.
- Every meaning has exactly one encoding. No sentinel values: absence is
  explicit, and "unknown", "absent", and "present" are distinct cases.
- Primitives that admit values the domain does not are wrapped in types with a
  checked constructor.
- Input is parsed into domain types once, at the boundary where it enters.
  Unstated input is refused, never given a default.
- Functions are total. A failure is returned in the type and never turned into a
  plausible default.
- Failures that are handled differently have different types.
- Lifecycle state is a sum type. State that must agree is kept in one place.
- Side effects are isolated at the edge. Domain logic takes values and returns
  values: it performs no I/O, reads no clock or environment, draws no
  randomness, and does not log. Effects enter as parameters and leave as data
  that the boundary carries out.
- Wire schemas follow the same rules: `oneof` for alternatives, closed enums for
  codes with the unspecified value refused, explicit presence where absence has
  meaning.
- Proposals and specs describe every data shape, state, error, and function they
  introduce as a type or signature, from the proposal stage on. Prose explains
  the types. A shape left undecided is listed as an open question.

## Code

- Errors are structured values carrying typed fields, returned in the type. A
  formatted string is produced only where an error is shown.
- In a batch, one bad item does not stop the rest, unless skipping it would
  silently corrupt the result.
- Externally sourced input is checked against named limits before allocation.
- Hand-written source files stay at or under 1 000 lines, and are split by
  responsibility.
- Comments say why, not what.

## Verification

- The toolchains are pinned.
- Formatting is done by tools, never by hand.
- Every check CI runs can be run locally with one command, and CI runs nothing
  else.

## Commits

- Commit messages follow Conventional Commits 1.0.0:
  `<type>(<scope>): <subject>`, with the types and scopes of ADR-0021.
- One logical change per commit, at a point where the tree builds and the fast
  checks pass.
- A move or split is its own commit and changes nothing else.
- A change to a record goes in the same commit as the change it records.
- Commits and pull requests carry no agent attribution: no `Co-Authored-By`
  trailer or "Generated with" line naming an agent or model (ADR-0020).

## Publication

- The local `research/` folder and agent tooling are never committed (ADR-0016).
  A file copied out of `research/` is published by that copy.
