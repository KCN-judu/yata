# The history of `main` was rewritten twice

- Date: 2026-09-24
- Area: process
- Affected: developers
- Related: ADR-0020, ADR-0021

## What changed

- Every commit on `main` was rewritten to drop its `Co-Authored-By: Claude …`
  trailer, and force-pushed. `main` moved from `58c0055` to `63cc16e`. Trees,
  authors, and dates are unchanged.
- Commits carry no agent attribution from now on (ADR-0020).
- Later on 2026-09-24, the commits `b6d0d36` to `6dcf34c` were rewritten again
  to remove text that must not be published (ADR-0021, rule 7): authors and
  dates are unchanged, and the trees differ only in
  `docs/issues/0007-scheme-code-encoding-header.md`. The two commits after them,
  up to `4814b73`, were replaced by the commits for ADR-0022. `main` was
  force-pushed.

## Compatibility and migration

A clone made before 2026-09-24 has the old history. Do not merge or pull it,
because that brings the old commits back. Reset it instead:
`git fetch origin && git reset --hard origin/main`. Local work goes on top of
`origin/main` by `git rebase --onto origin/main 58c0055`, or, for a clone made
between the two rewrites, `git rebase --onto origin/main 4814b73`.

## Evidence

Checked per commit before the push: the old and new histories have identical
trees, authors, and dates. `git log --format=%B` on the new history has no
`Co-Authored-By` line. For the second rewrite, each rewritten commit was
compared with its original: same author and dates, and a tree differing only in
the ISS-0007 page.
