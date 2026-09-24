---
id: ISS-0007
state: resolved
area: domain
opened: 2026-09-24
resolved-by: [ADR-0022]
related: [ADR-0009, ADR-0016]
---

# ISS-0007: Which account's header a generated scheme code carries

## Problem

Every scheme code carries a header naming the account that exported it. A code
whose header names the importing account cannot be imported by that account;
another account can import it (the maintainer, 2026-09-24). A code this
application generates for a user to import must therefore carry the header of an
account other than the user's own, and the application has to get that header
from somewhere.

## Why it matters

Without an answer, the application cannot generate a scheme code the user can
import, which is the point of encoding. The header is account-derived data of
whoever owns it, so its source also decides whose data the application holds
and, if bundled, publishes.

## Current evidence

- Codes carrying another account's header were imported successfully on
  2026-09-24.
- The account segment is 14 bytes that do not visibly encode the numeric game
  id; it cannot be constructed, only copied from a real code.
- `research/scheme-code-protocol.md` (local research, not published), "Header".

## Candidate answers

1. **A constant segment.** The application ships one fixed header segment. Every
   user can import the codes.
2. **A header the user supplies.** The user shares any code exported by another
   account (a friend's, a community code), and the application keeps its header
   for encoding. Nothing is bundled, but the user holds another player's account
   data and must find such a code first.
3. **Both**: the constant segment by default, a user-supplied header as an
   override.

## Dependencies

The maintainer's choice among the candidates. For candidate 1, a segment to
ship.

## Resolution

Resolved 2026-09-24 by
[ADR-0022](../decisions/0022-constant-scheme-code-segment.md): candidate 3. A
generated code carries the constant segment unless the caller gives another.
