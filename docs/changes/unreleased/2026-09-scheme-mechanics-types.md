# Scheme and soul types hold only states the game has

- Date: 2026-09-25
- Area: domain
- Affected: developers
- Related: ADR-0009, ADR-0029

## What changed

- **All souls has one encoding.** `SoulChoice = All | Souls(non-empty)` at the
  bit level and `SetChoice = AnySet | Sets(non-empty) | OnlyUnmapped` in a
  selection. A soul mask with no bit set is all souls, however long; clearing
  the last soul makes a record all souls again. A mask whose bits all lie beyond
  the mapped sets is `OnlyUnmapped` and picks no mapped soul.
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
- **Soul values are typed.** `Star` is one to six; `StoredValue` is finite and
  not negative, for main and sub-attribute values; `RollCount` is `c(a)`, and
  `HitCount` and `HitRange` replace the raw `Hits` fields.
- **Verdicts name non-empty open rules.** `Verdict::Undetermined` holds
  `OpenRules`, shared with the query engine.
- **Inspection and QR errors are precise.** `ByteDifference` says whether a byte
  changed or only one payload has it; `BitSpan` is 1 to 64 bits by construction;
  `QrError` gains `SegmentTooLong` and `EmptyImage`.
- Removed errors: `LayoutError::DiscardAllSouls`,
  `SelectionError::{EmptyDiscardSouls, AnySetInDiscard, NoSets, InvalidStar, FieldTooLong}`,
  `EditError`, `CodeError::NoDiscardScheme`,
  `Violation::{StarOutOfRange, InvalidValue}`, `BitSpanError::BadLength`.

## Compatibility and migration

No wire or storage format changes. A query whose inline selection has neither
`any_set` nor a suit code, a soul with a star outside 1 to 6, or a value that is
NaN, infinite, or negative is now `query.malformed`. `yata-daemon scheme diff`
lists every bit of a byte only one payload has.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Soul mechanics", "Scheme
selection and evaluation", "Scheme layout and edits", and "QR codes and research
commands".
