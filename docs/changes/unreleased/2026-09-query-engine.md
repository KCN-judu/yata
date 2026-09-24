# Typed queries over souls are checked and evaluated

- Date: 2026-09-25
- Area: domain
- Affected: developers
- Related: ADR-0026, ADR-0002, ADR-0009

## What changed

- `yata-core::query` checks a typed query tree against `query.md` once
  (`compile`) and runs it: `CompiledQuery::verdict` is `evaluate(query, soul)`,
  and `CompiledQuery::page` returns the kept rows in sort-key order, then row
  identity, one page at a time with a cursor.
- Predicates over `set`, `slot`, `star`, `level`, `main_attribute`,
  `main_value`, `sub_value(attr)`, `has_sub(attr)`, `sub_count`, and `pristine`.
  `Matches` and `MatchesScheme` call the scheme evaluator; `MatchesScheme` reads
  an inline scheme code and one of its plans or schemes.
- An open scheme verdict stays open: `And`, `Or`, and `Not` are Kleene logic,
  and each row carries its verdict with the open rules it rests on (ADR-0026).
- Score fields need a parameter set and are then refused with
  `query.field_unavailable` until pass 1 exists. New codes: `query.malformed`
  and `query.field_unavailable`; a new limit of 256 values per `In`.
- `crates/yata-protocol/proto/core.proto` is drafted with the query messages and
  the `EvaluateQuery` endpoint message; core protocol version 1 stays reserved.
- `yata-daemon query` answers `EvaluateQuery` frames on stdin with one
  `EvaluateQueryResult` frame each on stdout, over an inventory each request
  supplies. A malformed request gets its error and the stream goes on.

## Compatibility and migration

Nothing existing changes. The endpoint is headless and for tools and tests; the
application's session `Query` reads the projection once it exists.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Query evaluation".
