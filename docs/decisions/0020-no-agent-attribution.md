---
id: ADR-0020
status: accepted
date: 2026-09-24
area: process
supersedes: []
superseded-by: []
related: [ADR-0015, ADR-0016]
---

# ADR-0020: Commits and published text carry no agent attribution, in every Yata repository

## Status

Accepted, 2026-09-24.

## Context

The maintainer writes much of this project with coding agents. By default an
agent signs its work: a `Co-Authored-By:` trailer naming the model, or a
"Generated with" line in a commit message or pull request description.

The first pushes did this. Every commit in `yata` (6) and `yata-reader` (1)
carried `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. On 2026-09-24
both histories were rewritten to remove the trailer and force-pushed. Trees,
authors, and dates were unchanged:

| Repository    | Old `main` | New `main` |
| ------------- | ---------- | ---------- |
| `yata`        | `58c0055`  | `63cc16e`  |
| `yata-reader` | `31b01e1`  | `98e915e`  |

The maintainer is the author of record and answers for every change. A trailer
naming a tool is not a statement of authorship. GitHub also reads the trailer
and lists the tool's account as a contributor to a public repository.

## Decision

1. **No agent attribution.** Commit messages, pull request titles and
   descriptions, issues, release notes, and files carry no attribution to an
   agent or model. That means no `Co-Authored-By:` trailer naming one, no
   "Generated with" line, and no signature. A `Co-Authored-By:` trailer is kept
   only for a human co-author.
2. **Every Yata repository.** The rule holds in `yata`, `yata-reader`, and any
   repository added under the `yata-` prefix (ADR-0015). The other repositories
   cite this record and do not restate it.
3. **Agents are configured not to add it.** The configuration lives on the
   maintainer's machines, not in the repositories, because agent tooling is
   never committed (ADR-0016). An agent working on a Yata repository follows
   this record over its own default.
4. **A violation is removed.** If it has not been pushed, amend it. If it has
   been pushed, rewrite the history and force-push, as was done on 2026-09-24,
   and say so in a change fragment.

## Alternatives

- **Keep the trailer as disclosure.** Rejected by the maintainer, who is the
  author of record.
- **Enforce it in CI.** Deferred. CI checks out one commit, so it cannot see a
  pushed range without a deeper checkout and a base to compare against. If
  violations recur, the check is added to `scripts/preflight.py`.

## Consequences

**Easier.** The contributor list and history show only people.

**Harder.** Each machine an agent runs on has to be configured, and a machine
that is not configured shows up only in review. Rewriting a pushed history
invalidates every clone, open branch, and SHA cited anywhere. Every rewrite
therefore needs a note telling clones to reset.
