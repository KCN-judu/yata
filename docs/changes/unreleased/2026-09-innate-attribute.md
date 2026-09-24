# A soul carries its innate attribute, and 固有属性 is evaluated

- Date: 2026-09-25
- Area: domain
- Affected: developers
- Related: ADR-0009

## What changed

- `Soul` has a field `innate: Innate`, which is `Absent`, `Present(attribute)`,
  or `Unknown`. The innate attribute is held apart from `subs`.
- `yata-core::scheme::evaluate::matches` decides a chosen 固有属性 as the
  maintainer observed it on 2026-09-25: a soul without an innate attribute
  passes, and a boss soul passes exactly when its innate attribute is chosen.
- `OpenRule::Innate` remains, only for an `Unknown` innate attribute and for a
  boss soul under `AnySet` whose innate attribute is not chosen.
- Several innate choices together are membership, extrapolated
  (`scheme-code.md`, "Evaluation").

## Compatibility and migration

Code that builds a `Soul` sets `innate`. A decode that cannot tell whether a
soul carries one sets `Unknown`, not `Absent`.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Scheme selection and
evaluation".
