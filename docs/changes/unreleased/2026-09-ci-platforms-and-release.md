# CI fails on a missing tool, splits by platform, and packages the daemon

- Date: 2026-09-25
- Area: tooling
- Affected: developers
- Related: ADR-0008, ADR-0011, ADR-0023

## What changed

- Under GitHub Actions `scripts/preflight.py` runs `--strict`: a check whose
  tool is missing fails the job instead of passing as a skip. Checks with
  nothing to check yet report `n/a`.
- The Rust job runs once on Linux with formatting, the crate graph, and the new
  `rust-version` check; a `platform` job runs Clippy, the tests, and the new
  `release-package` check on Windows and macOS.
- `scripts/release.py` stages an unsigned package of the daemon with a
  deterministic name, a draft manifest, and `SHA256SUMS`
  (`docs/spec/release-manifest.md`); a manual `release` workflow keeps it as an
  artifact. Signing is refused until it is wired.
- The `publication` check fails on tracked private keys and certificates.

## Compatibility and migration

`just run platform` reproduces the `platform` job on a Windows or macOS machine.
Add `--strict` to any profile to reproduce a CI job exactly.

## Evidence

`python scripts/preflight.py fast --strict` and
`python scripts/preflight.py platform --strict` pass on the maintainer's Windows
machine; the CI run is recorded in `docs/project/status.md`.
