---
id: ADR-0009
status: accepted
date: 2026-09-23
area: domain
supersedes: []
superseded-by: []
related: [ADR-0001, ADR-0004, ADR-0005, PRP-0001, ISS-0001]
---

# ADR-0009: SoulSelection is a domain value mirroring the game's panel, and scheme codes use the game's official format only

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

The game natively provides strengthening schemes and discarding schemes, and
players share them through a _scheme code_. A tool that models neither leaves
the user to rebuild its advice by hand in the in-game panel, and the advice
becomes unusable at exactly the moment it matters most.

The official scheme code is Base64 text over a zlib-compressed payload, and the
payload is a game-defined binary layout (ISS-0001). The game presents the code
as a QR code. Parts of the layout are solved and parts are not: some header
bytes, some framing, and some filter bits have no known meaning yet
(`spec/scheme-code.md`).

Two pressures shape the choice:

1. The condition vocabulary and the domain value are most of the work, and they
   can be modelled from the game's panel whether or not every bit of the layout
   is known.
2. Once released, users generate scheme codes and paste them to each other, in
   the game and in this application. A code the application produces must be one
   the game reads, and a code the game produces must survive a round trip
   through the application unchanged.

## Decision

### The domain value

**`SoulSelection` is a pure domain value** (in the ADR-0001 sense), owned by
`yata-core`. It is the part of a scheme that picks souls, and its shape is the
game's: the checkbox groups of the game's own soul filter panel, not a
vocabulary defined by this project. A scheme code is either a discard scheme or
a strengthening scheme set of plans, each carrying a `SoulSelection`. The
groups, their values, and the full model are recorded in
[spec/scheme-code.md](../spec/scheme-code.md), which is maintained in step with
every game-client update that changes that panel.

A group enters `SoulSelection` only when every value it can take has a solved
bit. Until then it lives in the preserved content below.

### Evaluation

```text
matches(selection: &SoulSelection, soul: &Soul) -> bool
```

`matches` is the only evaluation entry point: a total, pure function in
`yata-core`, with no I/O, covered by unit tests. A query that filters by scheme
calls it. The Flutter layer sends a `SoulSelection` value and the core evaluates
it; the Flutter layer holds no second copy of the condition logic (ADR-0004).

### One format: the game's official one

**The scheme code feature has one wire format, the game's official one.** There
is no format defined by this project.

```text
decode: QR image or Base64 text → Base64 decode → zlib decompress → game layout → SchemeCode
encode: SchemeCode → game layout → zlib compress → Base64 encode → Base64 text and QR matrix
```

- **The payload codec is pure and lives in `yata-core`.** Reading a QR image and
  producing a QR module matrix live in `yata-daemon` (ADR-0005). The Flutter
  layer draws the matrix it is given; it never generates a QR code, because
  generating one is encoding.
- **Decoding dispatches through a `SchemeCodec` registry** with one registered
  codec. If the game introduces a new format, it is added as a new entry,
  changing no existing code path.
- **Rejection is the default.** Input that is not a scheme code is
  `decode.unknown_format`, with a human-readable hint. Accepting silently and
  producing garbage is not an option.

### Open bits are preserved, not awaited

The codec follows the rules in [spec/scheme-code.md](../spec/scheme-code.md):

- It reads and writes only the bits the layout marks as solved. Every other byte
  and bit — header, framing, unsolved filter bits — is carried opaquely in
  `Preserved`: kept on decode, written back unchanged on encode, never cleared
  and never guessed.
- `encode(decode(code))` reproduces the decompressed payload byte for byte, and
  every sample in the corpus is a fixture asserting it.
- `Preserved` exposes one fact, whether any preserved filter bit is set. When it
  is, `matches` evaluates the known groups only, which selects a superset of
  what the game would select, and every result derived from that scheme says so.

Encoding a scheme from scratch waits on the parts of the layout it needs. Until
then, authoring starts from a known-good template code, and only modelled fields
change. The encoder's stages and their gates are in the spec page.

## Alternatives

- **Wait until the whole layout is solved before building the feature.**
  Rejected: the condition vocabulary, the domain value, and `matches` are most
  of the work and are independent of the unsolved bits. Preserving open bits
  opaquely lets decoding and template-based editing proceed without them.
- **An interim format of our own**, a tagged compact text format shipped until
  the official one was fully solved. Rejected: every code a user generated in it
  would have to stay decodable forever, the tag space would need maintaining so
  the two formats could not collide, users would meet two kinds of code, and a
  recipient who uses only the game could not read one. Since open bits can be
  preserved rather than awaited, it would buy nothing the official format does
  not already give.
- **A general clause/operator/operand model defined by this project.** Rejected:
  nothing in the game's format has that shape, so every code would need a lossy
  translation in both directions.
- **One hard-wired decode path instead of a registry.** Rejected: a new format
  from the game would then modify an existing code path rather than add one,
  which raises the risk of a misread.
- **Store the official code as an opaque string and never interpret it.**
  Rejected: it cannot be validated, edited, shown as conditions, or generated.
  Ingestion that cannot interpret is only a paste buffer, not a feature.

## Consequences

**Easier.** Users meet one code format, and a code from anyone using only the
game imports directly. There is no second decoder to keep across versions and no
tag space to keep free of collisions. Decoding and template-based editing are
useful before every bit is solved.

**Harder.** The `SoulSelection` model must follow changes to the game's panel. A
lag means imported codes carry conditions this application does not recognize;
they are preserved and flagged, and evaluation of such a code is partial. Groups
whose bits are unsolved cannot be exported, and from-scratch encoding waits on
the layout; there is no degraded format to ship in the meantime. Three questions
about how the game evaluates a scheme are open, and `matches` cannot be
completed until the game answers them (`spec/scheme-code.md`, "Evaluation").

This record governs [spec/scheme-code.md](../spec/scheme-code.md), which holds
the model, the evaluation rules, the codec rules, and the encoder stages.
