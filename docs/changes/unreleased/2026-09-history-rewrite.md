# The history of `main` was rewritten to remove agent attribution

- Date: 2026-09-24
- Area: process
- Affected: developers
- Related: ADR-0020

## What changed

- Every commit on `main` was rewritten to drop its `Co-Authored-By: Claude …`
  trailer, and force-pushed. `main` moved from `58c0055` to `63cc16e`. Trees,
  authors, and dates are unchanged.
- Commits carry no agent attribution from now on (ADR-0020).

## Compatibility and migration

A clone made before 2026-09-24 has the old history. Do not merge or pull it,
because that brings the old commits back. Reset it instead:
`git fetch origin && git reset --hard origin/main`. Local work goes on top of
`origin/main` by `git rebase --onto origin/main 58c0055`.

## Evidence

Checked per commit before the push: the old and new histories have identical
trees, authors, and dates. `git log --format=%B` on the new history has no
`Co-Authored-By` line.
