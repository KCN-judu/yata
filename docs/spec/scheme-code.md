---
kind: spec
status: current
area: domain
---

# Scheme code

The domain model of a scheme code, what it means to evaluate one against a soul,
and the rules the codec must follow. The byte layout and its evidence are kept
in the project's local research record `research/scheme-code-protocol.md` (not
published). The architectural decision is ADR-0009.

There is one wire format, the game's official one (ADR-0009). There is no
project-defined format.

The model follows the wire, not the reverse. The game's editor is a set of
checkbox groups — which souls, which slots, which stars, which main attributes,
include or exclude per sub-attribute, which level bands — and the model is those
groups. A clause/operator/operand model would need a lossy translation in both
directions, because nothing in the game's format has that shape.

## The model

```text
SchemeCode =
    Discard(DiscardScheme)
  | Strengthening(StrengtheningSchemeSet)

DiscardScheme {
  name      : string
  selection : SoulSelection
  preserved : Preserved
}

StrengtheningSchemeSet {
  plans     : [StrengtheningPlan]      // in code order
  preserved : Preserved
}

StrengtheningPlan {
  name      : string
  selection : SoulSelection
  preserved : Preserved
}
```

### The game's panel

The model mirrors the game's soul filter panel (整理 → 筛选). The maintainer's
screenshots from 2026-09-23 establish its groups, their order, and their
dependencies:

| Order | Group (in game) | Choices                                                          | Depends on                                                |
| ----- | --------------- | ---------------------------------------------------------------- | --------------------------------------------------------- |
| 1     | 类型            | 全部, or a set of souls chosen in a picker                       | —                                                         |
| 2     | 位置            | 壹 贰 叁 肆 伍 陆                                                | —                                                         |
| 3     | 星级            | 1星 … 6星                                                        | —                                                         |
| 4     | 等级            | 0–2, 3–5, 6–8, 9–11, 12–14, 15                                   | —                                                         |
| 5     | 主属性          | the main attributes the chosen slots can roll                    | 位置: disabled with 「请选择位置」 until a slot is chosen |
| 6     | 固有属性        | the innate attributes of the chosen boss souls                   | 类型: disabled with 「请选择御魂」 until a soul is chosen |
| 7     | 副属性          | 11 attributes, each ○ (include) or ✕ (exclude), neither = ignore | —                                                         |
| 8     | 数量            | 不足2条, 2条, 3条, 4条                                           | —                                                         |

The panel sits beside the soul grid, in a right-hand pane with the
tabs 筛选 and 方案, and has「存为方案」 and 「重置」 at its foot.

Every group's scheme-code bits are solved (2026-09-24), so every group is in
`SoulSelection`. The game's import preview labels group 6 **固定属性**; the
editor's label is 固有属性.

### SoulSelection

The part of a scheme or plan that picks souls. ADR-0009 names it; its shape is
the game's.

```text
SoulSelection {
  sets             : SetChoice
  slots            : { SoulSlot }
  stars            : { 1..6 }
  main_attributes  : { SoulAttribute }
  sub_attributes   : SoulAttribute → SubAttributeMode    // absent key = Ignore
  levels           : { LevelBand }
  innate           : { InnateAttribute }                  // 固有属性; boss souls only
  sub_counts       : { SubCount }                         // 数量; also called "legs"
}

SetChoice        = AnySet | Sets({ SoulSet })
SubAttributeMode = Ignore | Include | Exclude
LevelBand        = L0to2 | L3to5 | L6to8 | L9to11 | L12to14 | L15
InnateAttribute  = AtkPercent | DefPercent | HpPercent | EffectHit | EffectRes | Crit
SubCount         = FewerThanTwo | Two | Three | Four
```

**`AnySet` and `Sets(every set)` are different values.** The game's
strengthening editor has a distinct "all souls" choice that is not the same as
ticking every set by hand (`research/scheme-code-protocol.md`). Collapsing them
would make a decode–encode round trip change the user's scheme. `AnySet` is
valid only in a strengthening plan; in a discard scheme it is an encode error
until the discard editor is shown to have the same choice.

**`SoulSet` identity is the game's suit code**, the id every soul record
carries. The scheme bit and the UI order are both mappings from it, recorded in
`research/scheme-code-protocol.md`; the codec converts suit code to scheme bit,
and the UI presents sets in ascending suit code.

**Every field of the editor is in the model.** A "rescue count" (救几次) seen in
some plan names is not an editor option: the maintainer confirmed on 2026-09-24
that it is only part of the name. A filter bit the editor never sets (61 and
above) is preserved, never modelled.

### Preserved

`Preserved` is everything the codec read and does not understand: header bytes,
framing, and every filter bit marked ◇ or ?. It is opaque to every layer but the
codec. Two facts about it are exposed:

- `has_unknown_conditions: bool` — whether any preserved filter bit is set, i.e.
  whether the scheme selects on something the model cannot see
- nothing else; the UI never renders preserved content

## Evaluation

```text
matches : (SoulSelection, Soul) -> bool
```

A total, pure function in `yata-core` (ADR-0001), and the only place a scheme's
meaning is computed. The Flutter layer never evaluates a scheme; a query that
filters by scheme calls this function (`query.md`, `MatchesScheme`).

A soul matches when every group matches. The per-group rules:

| Group                                         | A soul matches when                        | Mark |
| --------------------------------------------- | ------------------------------------------ | ---- |
| `sets`                                        | `AnySet`, or its set is in the chosen sets | ✓    |
| `slots`, `stars`, `main_attributes`, `levels` | its value is in the chosen set             | ✓    |
| a group with nothing chosen                   | ? — see below                              | ?    |
| `sub_attributes`, `Include`                   | ? — see below                              | ?    |
| `sub_attributes`, `Exclude`                   | it does not have that sub-attribute        | ◎    |

Three semantic questions are open, and `matches` cannot be written until the
game answers them:

1. **An empty group.** Whether ticking nothing in a group means "no constraint"
   or "match nothing".
2. **Several includes.** Whether `Include` on two sub-attributes requires both
   (AND) or either (OR).
3. **Include and a count condition together**, once the sub-attribute count
   field is solved.

These are questions about the game's behaviour, not about bytes; they are
settled by applying a scheme in the game and observing which souls it picks.

**A scheme with unknown conditions is evaluated partially.** When
`has_unknown_conditions` is true, `matches` evaluates the known groups only,
which selects a superset of what the game would select. Every result derived
from such a scheme carries that fact, and the UI says so; it is never presented
as the game's own selection.

## Codec

```text
decode : (QR image | Base64 text) -> Result<SchemeCode, DecodeError>
encode : (SchemeCode)             -> Result<EncodedScheme, EncodeError>

EncodedScheme { text: string, qr: QrMatrix }
```

The game presents a scheme code as a QR code
(`research/scheme-code-protocol.md`, "Encoding layers"). Both ends of the QR
layer are in Rust, in `yata-daemon` (ADR-0005); the payload codec underneath is
pure and lives in `yata-core`:

- **Decode** accepts an image (a file, a clipboard image, a screenshot region)
  or the Base64 text directly. Locating and reading the QR code is a pure
  function over the image bytes.
- **Encode** returns the Base64 text and the QR module matrix. The Flutter layer
  draws the matrix; it does not generate a QR code, because generating one is
  encoding and encoding is not presentation.

### Transport

The layers under the payload, as `yata-core::scheme::transport` implements them.
Each rule rests on the observed codes, and nothing outside them is accepted:

- **Base64** is the standard alphabet with `+` and `/`. The game writes it
  **without `=` padding** and accepts it with or without (both observed
  2026-09-24), so decoding accepts both and encoding writes none. Otherwise
  decoding is strict: no whitespace and no stray trailing bits.
- **zlib**: the text carries exactly one zlib stream, whose checksum must match
  and after which no byte may follow.
- **Limits**, checked before anything is allocated for them:

  | Limit                 | Value        | Why                                                                           |
  | --------------------- | ------------ | ----------------------------------------------------------------------------- |
  | `MAX_SCHEME_TEXT_LEN` | 4 096 chars  | a QR code carries at most 2 953 bytes; the rest is room for pasted text       |
  | `MAX_COMPRESSED_LEN`  | 3 072 bytes  | what the text limit can carry                                                 |
  | `MAX_PAYLOAD_LEN`     | 65 536 bytes | the largest observed payload is 1 102 bytes; decompression stops at the limit |

- **An empty payload is refused.** No scheme is empty.
- **Errors name their stage**: the text length, the Base64 layer, the zlib
  layer, trailing bytes, the payload size, an empty payload.

What a round trip guarantees, and what it does not:

| Identity                                  | Guaranteed  | Why                                                                                   |
| ----------------------------------------- | ----------- | ------------------------------------------------------------------------------------- |
| payload: `decode(encode(p)) = p`          | yes         | tested for every payload up to 2 KiB by property, and for every corpus sample         |
| compressed stream and text of our encoder | yes         | the compression level is fixed, so the same payload always gives the same text        |
| compressed stream and text of a game code | no          | the game's compressor and settings are unknown; a re-encoded code may differ in bytes |
| meaning                                   | not claimed | the payload is not interpreted at this layer                                          |

### QR codes

`yata-daemon::qr` makes and reads the QR layer. A code it makes is plain and
standard, for testing in the game: no logo, colour, or styling. Its parameters
are fixed, so the same text always gives the same matrix:

| Parameter        | Value                                                                 | Why                                                                |
| ---------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------ |
| mode             | byte, one segment                                                     | the text is carried exactly as it is                               |
| error correction | M                                                                     | tolerates blur from a photographed screen; the game's is not known |
| version          | the smallest that holds it                                            | the smallest code                                                  |
| mask             | the standard's penalty rule                                           | deterministic                                                      |
| rendering        | quiet zone of 4 modules, black on white, at most 16 pixels per module | the standard's quiet zone                                          |

Reading accepts one PNG image of at most 32 MiB, 8 192 pixels per side, and 32
Mi pixels in all. An image with no code, or with more than one, is refused
rather than guessed at.

### Codec rules

- **Marked bits only.** The codec reads and writes only what
  `research/scheme-code-protocol.md` marks ✓ or ◎. Every other bit is carried in
  `Preserved`: kept on decode, written back unchanged on encode. Never cleared,
  never guessed.
- **Mapped bits only.** A soul mask is built through the suit code → scheme bit
  table, never from suit-code order or UI index.
- **Round-trip is lossless.** `encode(decode(code))` reproduces the decompressed
  payload byte for byte. Every sample in the corpus is a fixture asserting it.
- **Unknown format is an error.** Input that is not a scheme code is
  `decode.unknown_format`, never a best-effort parse.

Codecs are reached through the `SchemeCodec` registry interface (ADR-0009),
which carries a `SchemeCode`. There is one registered codec.

### Encoder stages

| Stage                       | Produces                                                              | Gated on                           |
| --------------------------- | --------------------------------------------------------------------- | ---------------------------------- |
| 1. template edit            | a code derived from a decoded code, with only modelled fields changed | nothing: the framing is solved     |
| 2. single plan from scratch | a strengthening plan with no template                                 | nothing: the plan record is solved |
| 3. whole set from scratch   | a complete strengthening scheme set, or a discard scheme              | nothing; see the header rule       |

**The framing is solved for both kinds.** On 2026-09-24 a strengthening set
built from nothing by this project, with 14 plans, was imported into the game,
which listed every plan by name and showed the soul each one selected; two
discard schemes built the same way were imported and the game's preview showed
every group as encoded (`research/scheme-code-protocol.md`, "Confirmed by
import" and "Discard schemes confirmed by import"). The filter layout is the
same in both kinds, and every filter group of the game's editor is solved.

A plan's name must have at most 10 characters and 26 bytes: longer names were
refused on import.

A plan built from scratch writes only solved fields; bits 61 and above, never
seen set, are left clear, and bits already present in a template are preserved
as read.

### The header and the user's account

Every code carries a header that identifies **the account that exported it** and
the scheme's kind. The game shows that account on import. The header is
account-derived data: it stays on the user's machine, is never logged, and is
never shown except as the account it names.

**A code carrying an account's own header cannot be imported by that account.**
Another account can import it (the maintainer, 2026-09-24). So a code this
application encodes for the user to import must carry the header of a different
account. It carries the codec's constant header segment,
`layout::CONST_SEGMENT`, a required constant that is not to be modified or
removed, unless the caller gives another taken from any code's header
(ADR-0022). The research commands take a code, or `const` for the constant
segment.

The user's own account segment is still worth knowing: a code the user shares
identifies their profile, and the application can refuse to encode a code the
user could not import.

## Open questions

- whether the game's import accepts Base64 text as well as a QR code
- the three evaluation questions above

## Related

- The wire format: `research/scheme-code-protocol.md` (local research, not
  published)
- Decision: [ADR-0009](../decisions/0009-scheme-code-model-and-format.md)
- Filtering souls by scheme: [query.md](query.md)
- Saved schemes: [fact-format.md](fact-format.md)
- Attribute and slot vocabulary: [glossary.md](glossary.md)
