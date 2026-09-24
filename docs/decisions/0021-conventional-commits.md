---
id: ADR-0021
status: accepted
date: 2026-09-24
area: process
supersedes: []
superseded-by: []
related: [ADR-0015, ADR-0020]
---

# ADR-0021: Commit messages follow Conventional Commits 1.0.0, in every Yata repository

## Status

Accepted, 2026-09-24.

## Context

The first commits in `yata` and `yata-reader` used free-form imperative
subjects, such as "Add yata-store: store instructions and…". Nothing in the
subject said what kind of change a commit was or which part of the project it
touched, so it could not be filtered by either. The maintainer uses
[Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) in
their other projects.

## Decision

1. **Format.** Every commit subject is `<type>(<scope>): <subject>`, following
   Conventional Commits 1.0.0.
2. **Types.** `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`, `ci`,
   `perf`, `style`.
3. **Scope.** The scope names the part the commit touches:
   - a crate, without its `yata-` prefix: `core`, `store`, `daemon`, `protocol`
   - `app` for the Flutter application
   - `scripts` or `ci` for tooling
   - for documentation, the folder under `docs/`: `decisions`, `spec`,
     `architecture`, `project`, `guides`, `changes`, `issues`

   A commit that touches several parts takes the one it is mainly about. A
   commit that cannot name a main part is split, as the engineering requirements
   already ask (one logical change per commit).

4. **Subject.** The subject is imperative, in English, and at most 72
   characters. Details go in the body.
5. **Breaking changes** are marked with `!` after the type or scope, or with a
   `BREAKING CHANGE:` footer. For this project, a breaking change is one that
   breaks a wire, a file format, or a stored record that another component or a
   user's data depends on.
6. **Every Yata repository.** The rule holds in `yata`, `yata-reader`, and any
   repository added under the `yata-` prefix (ADR-0015). A repository's own
   records may add scopes for its own parts, and cite this record for the rest.
7. **No rewrite.** Commits made before this record keep their messages. History
   is rewritten only to remove what must not be published (ADR-0020), never for
   style.

## Alternatives

- **Free-form subjects.** Rejected: the history could not be filtered by kind or
  by part.
- **Rewrite the existing commits into the new form.** Rejected: rewriting a
  public history invalidates every clone, and the old subjects are only
  unstructured, not wrong.
- **Enforce it in CI.** Deferred, for the reason given in ADR-0020: CI checks
  out one commit. If violations recur, the check is added to
  `scripts/preflight.py` together with the attribution check.

## Consequences

**Easier.** The history can be filtered by kind and by part. A release note can
be drafted from `feat`, `fix`, and breaking commits.

**Harder.** Every commit needs a type and a scope, and a commit that needs two
scopes has to be split. The format is checked only in review until a check is
added.
