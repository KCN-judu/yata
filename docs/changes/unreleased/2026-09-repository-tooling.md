# The repository has its tooling: preflight, formatting, and CI

- Date: 2026-09-24
- Area: tooling
- Affected: developers
- Related: ADR-0013, ADR-0016, ADR-0017, ADR-0018, ADR-0019

## What changed

- `just fast` (before a commit), `just check` (before a push), and `just fmt`
  (rewrites files) run the named checks of `scripts/preflight.py`. CI runs the
  same checks by profile (`docs/project/ci.md`).
- The toolchains are pinned: Rust by `rust-toolchain.toml`, Python tooling by
  `.python-version` and `requirements-dev.txt`, Prettier and markdownlint by
  version in `scripts/preflight.py`.
- The repository is licensed MIT OR Apache-2.0 (ADR-0018).

## Compatibility and migration

Install the Python tooling with `pip install -r requirements-dev.txt`. Prettier
and markdownlint run through `npx`, which needs Node.

## Evidence

`just fast` passes on the maintainer's Windows machine.
