# Scheme and soul types hold only states the game has

- Date: 2026-09-25
- Area: domain
- Affected: developers
- Related: ADR-0009, ADR-0029

## What changed

- **All souls has one encoding.** `SoulChoice = All | Souls(non-empty)` at the
  bit level and `SetChoice = AnySet | Sets(non-empty SetBit)` in a selection. A
  soul mask with no bit set is all souls, however long; clearing the last soul
  makes a record all souls again.
- **An unmapped soul bit widens 类型.** A set soul bit is a chosen soul:
  `Mapped` for a set the bit table holds (`SchemeSet`), `Unmapped` beyond it. A
  soul whose set has no bit is `Undetermined(UnknownSet)` where an unmapped bit
  is chosen; it was a decided `DoesNotMatch`. `OpenRule::UnknownConditions`
  splits into `UnknownSet` and `UnknownFilter`; the wire keeps one rule for
  both.
- **Layouts cannot disagree with their kind.** `SchemeLayout` is
  `Strengthening { account, plans }` or `Discard { account, schemes }`, the
  schemes a non-empty list of `DiscardRecord`. A discard scheme that chooses all
  souls is refused with `DiscardCannotSelectAll`, at the layout and on
  `DiscardScheme`, whose selection is read through `selection()`.
  `SchemeCode::Discard` is non-empty.
- **Names are checked once.** `SchemeName` holds a name the game imports; plans,
  schemes, and `Record::from_bits` take one.
- **Every bit edit is total.** `set_soul`, `set_filter`, and `from_bits` take a
  `BitState` and cannot fail; `EditError` is gone.
- **Soul values are typed.** `Star` is one to six, `Level` zero to fifteen;
  `StoredValue` is finite and not negative, for main and sub-attribute values;
  `RollCount` is `c(a)`, and `HitCount` and `HitRange` replace the raw `Hits`
  fields.
- **Verdicts name non-empty open rules.** `Verdict::Undetermined` holds
  `OpenRules`, shared with the query engine, and groups combine by written
  rules.
- **Inspection and QR errors are precise.** `ByteDifference` says whether a byte
  changed or only one payload has it; `BitSpan` is 1 to 64 bits by construction;
  `QrError` gains `SegmentTooLong` and `EmptyImage`.
- Removed errors: `LayoutError::DiscardAllSouls`,
  `SelectionError::{EmptyDiscardSouls, AnySetInDiscard, NoSets, InvalidStar, FieldTooLong, UnknownSet}`,
  `EditError`, `CodeError::NoDiscardScheme`,
  `Violation::{StarOutOfRange, LevelOutOfRange, InvalidValue}`,
  `BitSpanError::BadLength`.

## Compatibility and migration

No wire or storage format changes. A query whose inline selection chooses a suit
code no scheme can choose, or whose inventory holds a soul with a star outside 1
to 6, a level above 15, or a value that is NaN, infinite, or negative, is now
`query.malformed`. Until the core protocol carries soul bits beyond the mapped
sets, a scheme entry that chooses one is sent without a selection, with
`has_unknown_conditions` set. `yata-daemon scheme diff` lists every bit of a
byte only one payload has.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Soul mechanics", "Scheme
selection and evaluation", "Scheme layout and edits", and "QR codes and research
commands".
