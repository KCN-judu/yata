---
id: ISS-0001
state: resolved
area: domain
opened: 2026-09-23
resolved-by: [ADR-0009]
related: []
---

# ISS-0001: The game's strengthening and discard scheme-code encoding is undocumented

## Problem

The game turns a strengthening scheme or a discard scheme into a code and
accepts one back, which is how players exchange filter setups. The encoding is
not documented anywhere this project can consult.

## Why it matters

Scheme codes are how players share filters, including the ones streamers
publish. A scheme feature that cannot read and write the game's own codes cannot
exchange a filter with anyone, so the feature depends on this encoding entirely.

## Current evidence

The encoding is layered, and each layer is identifiable from real codes:

1. the game presents a code as a QR code
2. the QR code carries a Base64 string
3. Base64 decodes to a zlib stream
4. zlib decompresses to a game-defined binary layout carrying the scheme's name
   as UTF-8 and its conditions as bitmasks

The layout is known well enough to decode and encode the conditions that matter;
a few bits are not yet understood.

## Dependencies

- Real codes with known source schemes, to isolate each condition's
  contribution.
- The `SoulSelection` model (`spec/scheme-code.md`), in which a decoded code is
  expressed.

## Resolution

**Resolved 2026-09-23 by
[ADR-0009](../decisions/0009-scheme-code-model-and-format.md).** The application
uses the game's own format and no other. The model and the codec rules are in
[`spec/scheme-code.md`](../spec/scheme-code.md). Bits that are not yet
understood are preserved opaquely, so a decode–encode round trip never changes a
code.
