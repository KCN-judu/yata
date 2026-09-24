---
id: ADR-0022
status: accepted
date: 2026-09-24
area: domain
supersedes: []
superseded-by: []
related: [ADR-0009, ISS-0007]
---

# ADR-0022: A generated scheme code carries a constant header segment by default

## Status

Accepted, 2026-09-24. Resolves ISS-0007.

## Context

Every scheme code's header carries a 14-byte segment, and a code whose segment
is the importing account's own cannot be imported by that account
(`spec/scheme-code.md`, "The header and the user's account"). A code the
application generates for the user must therefore carry a segment that is not
the user's. A segment cannot be constructed; it can only be copied from a code
the game exported. ISS-0007 set out where it could come from.

## Decision

1. **A constant segment.** The codec holds one constant,
   `layout::CONST_SEGMENT`, and a generated code carries it unless the caller
   gives another. It is no user's own segment, so every user can import the
   code.
2. **A required constant.** `CONST_SEGMENT` is not to be modified or removed.
   Its source says so, and a change to it is a change to this record.
3. **An override.** A caller may give another segment, taken from the header of
   any code. The research commands take either a code or `const`.
4. **Unchanged: a user's own segment stays private.** The segment learned from a
   user's own code (`SchemeAccountLearned`, `spec/fact-format.md`) is still kept
   on the user's machine, never logged, and never shown.

## Alternatives

- **Only a segment the user supplies.** Rejected: the user must first find a
  code exported by another player, and the application then holds that player's
  account data.
- **Only the constant segment.** Rejected: research needs codes for chosen
  accounts, and the override costs one argument.

## Consequences

**Easier.** Any user can import a generated code with nothing to supply.

**Harder.** Generated codes depend on one constant the game keeps accepting. A
code the user imports does not show the user's own account.
