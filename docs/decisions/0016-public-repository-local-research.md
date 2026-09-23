---
id: ADR-0016
status: accepted
date: 2026-09-23
area: process
supersedes: []
superseded-by: []
related: [ADR-0007, ADR-0011, ADR-0014, ADR-0015, ISS-0005]
---

# ADR-0016: The main repository is public from its creation; research records stay local

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

ADR-0015 names the repositories `yata` and `yata-reader` under the `KCN-judu`
GitHub account. The reader is public (ADR-0007). The main repository's
visibility has to be chosen before its first push.

Publishing is not reversible. Once pushed, content can be cached, forked, and
indexed. Some of what the project has written is research rather than
engineering record:

- the notes on an earlier tool for the same game (ADR-0014)
- the byte-level reverse-engineering evidence for the game's scheme code
- the sources behind the soul-mechanics rules
- the UI research: PRP-0002, its mockups, and the colour derivations

The agent skills under `.claude/` are tooling for the maintainer's agents, not
part of the product.

## Decision

1. **`yata` is public from its first push.** There is no private phase.
2. **Research records are kept out of the repository.** They live in `research/`
   beside `docs/` in the maintainer's working copy, and `.gitignore` excludes
   `research/`. The folder holds:
   - `research/legacy/`: the notes on the earlier tool
   - `research/scheme-code-protocol.md`: the scheme-code byte layout with its
     evidence marks, and the check output that comes with it
   - `research/soul-mechanics-sources.md`: the source and evidence level of each
     rule in [spec/soul-mechanics.md](../spec/soul-mechanics.md)
   - `research/proposals/`: PRP-0001 (the scheme-code proposal) and PRP-0002 (UI
     layout and theme)
   - `research/pre-publication-docs/`: earlier drafts of the records in `docs/`
3. **Research is kept on the maintainer's machine only.** It is not placed in a
   second repository, private or otherwise.
4. **The agent skills stay out.** `.claude/` is excluded.
5. **Public records cite research by path, never by link.** A citation is the
   path in a code span, marked "local research, not published". A relative link
   to `research/` would be a dead link in the published repository. Record IDs
   held in research, such as PRP-0001 and PRP-0002, are valid IDs and are never
   reused.
6. **What the code needs stays public.** Rules the code implements are specified
   in `docs/`: the soul-mechanics rules, the scheme-code model in
   [spec/scheme-code.md](../spec/scheme-code.md). Only the evidence trail is
   local. The codec's constants and its golden tests are code, and are public
   with it.

## Alternatives

- **Private until the first release.** Not chosen by the maintainer: publishing
  from the first push makes the application and the reader it drives inspectable
  from the first commit.
- **Research in a separate private repository.** Offered: history and a backup
  for research. Not chosen; the maintainer keeps research local only.
- **Publish everything.** Rejected by the maintainer for the four groups above.

## Consequences

**Easier.** The main repository's own GitHub Releases are a candidate release
feed (ADR-0011). The website can be built from the main repository (ISS-0005).
The reader and the application it serves are both inspectable from the first
commit.

**Harder.**

- **Research has no history and no backup.** A lost or damaged working copy
  loses it. Git in the public repository cannot help, by construction.
- **Public readers see rules without their evidence.** `spec/soul-mechanics.md`
  states where a rule is unsourced or contested, but not the citations. The
  scheme-code spec describes the model without the byte-level evidence.
- **Every push must be checked for research.** A file copied out of `research/`
  into `docs/` or a code comment publishes it. The preflight checks that no
  tracked path is under `research/` and that no tracked Markdown links into it.
