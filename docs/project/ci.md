---
kind: project
status: current
area: tooling
---

# Continuous integration

What each CI job proves. Every job runs one profile of `scripts/preflight.py`,
and the same profile runs locally with `just run <profile>`. CI runs no check
that preflight does not.

| Job       | Runs on               | Profile      | Proves                                                                                                                                                                                                        |
| --------- | --------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `docs`    | Linux                 | `docs-ci`    | records are well-formed; Markdown is formatted and lint-clean; scripts are typed and lint-clean; nothing local-only is tracked; SQL stays in `yata-store`; committed icons follow the SVG profile             |
| `rust`    | Linux, Windows, macOS | `rust-ci`    | formatting, the crate graph, Clippy with `-D warnings`, the workspace tests                                                                                                                                   |
| `flutter` | Linux                 | `flutter-ci` | Dart formatting, the analyzer, the widget tests; passes with nothing to check until `app/` exists                                                                                                             |
| `formal`  | Linux                 | `formal-ci`  | every Lean theorem of the quality standard checks against the pinned Mathlib; the calibration program is formatted and Clippy-clean, and every number it generates matches the committed documents (ADR-0023) |

`just fast` runs the structural checks, the formatters in check mode, and
`cargo check`. `just check` runs everything the CI jobs prove on the local host;
the `formal` checks are skipped where Lean (`elan`) is not installed.

The Rust job runs on all three platforms because the store, paths, and process
spawning differ between them (ADR-0010 asks for a Windows test under a non-ASCII
path).

## Related

- The checks and their hints: `scripts/preflight.py`
- The requirements:
  [../guides/engineering-requirements.md](../guides/engineering-requirements.md)
