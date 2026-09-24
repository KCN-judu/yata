---
kind: project
status: current
area: tooling
---

# Continuous integration

What each CI job proves. Every job runs one profile of `scripts/preflight.py`,
and the same profile runs locally with `just run <profile>`. CI runs no check
that preflight does not.

| Job        | Runs on        | Profile       | Proves                                                                                                                                                                                                                                                                                           |
| ---------- | -------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `docs`     | Linux          | `docs-ci`     | records are well-formed; Markdown is formatted and lint-clean; scripts are typed and lint-clean; nothing local-only or key-like is tracked; SQL stays in `yata-store`; icons follow the SVG profile; the release rules                                                                           |
| `rust`     | Linux          | `rust-ci`     | formatting, the crate graph, the toolchain pin equal to `rust-version`, Clippy with `-D warnings`, the workspace tests                                                                                                                                                                           |
| `platform` | Windows, macOS | `platform-ci` | Clippy and the workspace tests on each shipping platform; the daemon builds in release mode and packages, unsigned, into a package that verifies                                                                                                                                                 |
| `flutter`  | Linux          | `flutter-ci`  | Dart formatting, the analyzer, the widget tests; `n/a` until `app/` exists                                                                                                                                                                                                                       |
| `formal`   | Linux          | `formal-ci`   | every Lean theorem of the quality standard checks against the pinned Mathlib; the calibration program is formatted and Clippy-clean, and every number it generates matches the committed documents; every paper's Typst body matches its Markdown source under the pinned Pandoc 3.11 (ADR-0023) |
| `release`  | Windows, macOS | —             | manual only (`workflow_dispatch`): the unsigned package of `release.py`, kept as a workflow artifact for seven days; publishes nothing                                                                                                                                                           |

`just fast` runs the structural checks, the formatters in check mode, and
`cargo check`. `just check` runs everything the Linux jobs prove on the local
host; `just run platform` runs the `platform` job's checks. A check whose tool
is not installed is skipped locally.

## Strict mode

Under GitHub Actions preflight runs with `--strict`: a check whose tool is
missing **fails** instead of being skipped, because each job installs every tool
its profile needs and a skip there means the setup broke. A check with nothing
to check yet, such as the Flutter checks before `app/` exists, reports `n/a` and
passes in either mode. Run `python scripts/preflight.py <profile> --strict` to
reproduce a job exactly.

## Why each platform runs what it runs

| What                          | Where                 | Why                                                                                                                                                           |
| ----------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| formatting, crate graph, pin  | Linux only            | the same on every platform                                                                                                                                    |
| Clippy, tests                 | Linux, Windows, macOS | `cfg`-gated code differs by target; the store, paths, and process spawning differ between platforms (ADR-0010 asks for a Windows test under a non-ASCII path) |
| release package               | Windows, macOS        | the two shipping platforms of ADR-0008; Linux is not packaged                                                                                                 |
| Lean, calibration, docs, Dart | Linux only            | no platform-dependent code                                                                                                                                    |

The macOS job is what shows that nothing the daemon needs is Windows-only: the
workspace builds and its tests pass on macOS arm64. There is no reader code in
this repository (ADR-0006, ADR-0007), and no `cfg(windows)` module yet, so the
Windows job exercises the daemon only. Nothing in CI runs against an installed
game or a running game process; that stays manual and is tested by replay of
recorded bytes (`architecture/overview.md`, "What CI can and cannot see").

## Caching

The Rust jobs use `Swatinem/rust-cache`. The `formal` job caches the Lean
toolchain (`~/.elan`) by `lean-toolchain`, and takes Mathlib's prebuilt files
from Mathlib's own cache (`lake exe cache get`) on every run. They are not kept
in the Actions cache: they would take most of the repository's cache quota to
save about a minute.

## Related

- The checks and their hints: `scripts/preflight.py`
- The release package:
  [../spec/release-manifest.md](../spec/release-manifest.md)
- The requirements:
  [../guides/engineering-requirements.md](../guides/engineering-requirements.md)
