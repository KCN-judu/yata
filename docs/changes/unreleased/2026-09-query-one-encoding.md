# The query wire gives every shape one encoding

- Date: 2026-09-25
- Area: process
- Affected: developers, the application
- Related: ADR-0026, ADR-0004

## What changed

- `core.proto`, core protocol version 1, still reserved:
  - `SoulSelection` 类型 is `oneof sets { AnySet all; SuitCodes chosen }`; an
    empty `chosen` list is `query.malformed`, because "every set" is
    `all`. 副属性 is `repeated SubAttributeChoice { attribute, mode }`.
  - `Field` is `oneof { SimpleField simple; sub_value; has_sub; quality }`;
    `FieldName` is removed.
  - `InTest` holds one list: `oneof { SuitCodes; Slots; Attributes }`.
  - `IntRange` and `NumberRange` are
    `oneof bound { at_least; at_most; between }`.
  - `QueryRow.verdict` is `oneof { ExactVerdict exact; OpenVerdict open }`, in
    place of `open_rules` empty for exact.
  - `QueryPage.next_cursor` is optional, in place of `has_more` and `cursor`;
    `PageRequest.cursor` is optional.
  - `EvaluateQueryResult.subject` is `oneof { id; Undecodable undecodable }`, in
    place of id 0 for a request that did not decode.
  - Removed tags and names are reserved.
- `yata-core::query`: a kept row is `RowVerdict::Exact` or `Open(OpenRules)`,
  and `OpenRules` is never empty. Ranges hold a `Bound`. A code with no plan is
  `SchemeProblem::NoEntries`. Scheme code text and parameter set ids are
  newtypes.
- `yata-daemon::query`: `run` returns the domain page and `render_headless`
  writes it; soul ids are `GameSoulId`; a zero and a too-large row budget are
  separate errors; the fallback frame for a result too large to frame cannot
  fail.

## Compatibility and migration

The application's generated Dart types change: `QueryPage.hasNextCursor()` and
`nextCursor`, `QueryRow.whichVerdict()`, `SoulSelection.whichSets()` with
`chosen.codes`, `SoulSelection.subAttributes`, `Field.whichField()`, and the
`bound` oneofs. A client sends `PageRequest.cursor` only when it has one.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Query evaluation" and "Core
protocol session".
