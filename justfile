# The documented entry point: every command a contributor needs, and nothing CI does differently
# (docs/project/ci.md). Recipes that rewrite files say so.

set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

python := if os_family() == "windows" { "python" } else { "python3" }

default:
    @just --list

# structural checks, formatting, cargo check: before a commit
fast:
    {{python}} scripts/preflight.py fast

# everything CI proves: before a push
check:
    {{python}} scripts/preflight.py full

# REWRITES FILES: cargo fmt, dart format, prettier, markdownlint --fix, ruff format
fmt:
    {{python}} scripts/preflight.py fix

# REWRITES FILES: the papers' Typst bodies and PDFs, with Pandoc and Typst (papers/README.md)
papers:
    {{python}} scripts/build_papers.py

# one named check or profile
run +names:
    {{python}} scripts/preflight.py {{names}}

# REWRITES FILES: the committed Dart bindings of the core protocol schema
proto:
    {{python}} scripts/gen_dart_protocol.py

# the engineering records alone
docs:
    {{python}} scripts/validate_docs.py

test:
    cargo test --workspace
