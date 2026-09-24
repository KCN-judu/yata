# A soul is ordinary or a boss soul, and 固有属性 is evaluated

- Date: 2026-09-25
- Area: domain
- Affected: developers
- Related: ADR-0009, ADR-0029

## What changed

- `Soul` has a field `kind: SoulKind`, which is `Ordinary` or
  `Boss(InnateAttribute)`. The innate attribute is held apart from `subs`, and
  only the six innate attributes can be held. `InnateAttribute` moves to
  `yata-core::soul`; `scheme::selection` re-exports it.
- There is no unknown kind (ADR-0029). A record whose innate attribute the
  reading does not establish is not an inventory row: `innate` joins the fact
  log's row fields.
- `yata-core::scheme::evaluate::matches` decides a chosen 固有属性 as the
  maintainer observed it on 2026-09-25: an ordinary soul passes, and a boss soul
  passes exactly when its innate attribute is chosen.
- `OpenRule::Innate` remains only for a boss soul whose own set is not chosen
  and whose innate attribute is not chosen.
- Several innate choices together pick a boss soul carrying any of them,
  and 数量 does not count the innate attribute; both observed (`scheme-code.md`,
  "Evaluation").
- On the core wire, `Soul.kind` is a oneof of `OrdinarySoul` and `BossSoul`;
  field 9 and the `Innate` message are retired, so `absent: false` cannot be
  written. A soul with no kind is `query.malformed`.

## Compatibility and migration

Code that builds a `Soul` sets `kind`. A decode that cannot tell whether a soul
is a boss soul refuses it; it does not guess `Ordinary`. Dart code reads
`Soul.whichKind()` in place of `Soul.innate`.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Scheme selection and
evaluation" and "Query evaluation".
