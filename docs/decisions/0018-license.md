---
id: ADR-0018
status: accepted
date: 2026-09-24
area: process
supersedes: []
superseded-by: []
related: [ADR-0007, ADR-0014, ADR-0016]
---

# ADR-0018: The main repository is licensed MIT OR Apache-2.0

## Status

Accepted, 2026-09-24.

## Context

The main repository is public from its first push (ADR-0016). A public
repository with no license grants no rights to use, modify, or redistribute its
code, so a license has to be chosen before that push. The reader is already
licensed `MIT OR Apache-2.0` (ADR-0007). The main repository ports nothing from
the prior tool (ADR-0014), so no inherited license constrains the choice.

## Decision

**`yata` is licensed `MIT OR Apache-2.0`, at the user's option**, the same as
`yata-reader`.

- The repository root holds `LICENSE-MIT` and `LICENSE-APACHE`, and every crate
  manifest says `license = "MIT OR Apache-2.0"`.
- A contribution is accepted under the same dual license, as the Apache
  License's section 5 provides, unless the contributor states otherwise.
- The license covers the repository's code and the project's own artwork. It
  does not cover the game's names, data, or artwork, which belong to NetEase.
  The local icon pack (ADR-0017) and the local research (ADR-0016) are not in
  the repository.

## Alternatives

- **GPL-3.0-or-later or AGPL-3.0-or-later.** Offered: they would require
  modified copies to stay open source. Not chosen by the maintainer.
- **MPL-2.0.** Offered: file-level copyleft. Not chosen.
- **The same license as the reader** was chosen, so that code and fixtures can
  move between the two repositories with no license question.

## Consequences

**Easier.** Code can move between the two repositories freely. The license
matches the Rust ecosystem convention, so dependency license checks are simple.

**Harder.** Anyone may ship a closed modified copy of the application. The
maintainer accepts this.
