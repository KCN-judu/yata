---
id: ADR-0026
status: accepted
date: 2026-09-25
area: domain
supersedes: []
superseded-by: []
related: [ADR-0002, ADR-0004, ADR-0009]
---

# ADR-0026: A query keeps a scheme's open verdict, compares stored values with the domain's tolerance, and refuses what it cannot evaluate

## Status

Accepted, 2026-09-25.

## Context

`query.md` designs a closed, typed expression tree and leaves one semantic
question to the evaluator: how a query treats a soul for which `matches` answers
`Undetermined` (`scheme-code.md`, "Evaluation"). The question is not rare. A
scheme with unknown conditions answers `Undetermined` for each soul that no
decided group rules out, and so does a chosen innate attribute for each soul
whose innate attribute the reading does not carry.

The rest of the vocabulary is two-valued. Every field is defined for every row,
so a predicate is true or false and `Not` is its exact complement. A verdict is
not a predicate: `Undetermined` says that the game's answer rests on rules the
evidence does not settle. Reading it as either boolean presents a guess as the
game's selection, which `scheme-code.md` forbids.

Three smaller questions come with the evaluator. Number fields hold stored
values, and the domain compares stored values with a tolerance (`true_n`,
`VALUE_TOLERANCE`). The vocabulary lists fields that this build cannot evaluate:
scores before pass 1 exists, user facts before the fold exists. And no
projection exists yet for a query to read.

## Decision

1. **The filter has three outcomes, combined by strong Kleene logic.** A
   predicate is decided. `Matches` and `MatchesScheme` take the verdict of
   `matches`. `And` is decided false if any operand is false, true if all are
   true, and open otherwise. `Or` is the dual. `Not` swaps true and false and
   leaves open as open. An open outcome carries the union of the open rules of
   the verdicts that made it open. An empty `And` is true and an empty `Or` is
   false, as `query.md` states.
2. **A result holds every row that is true or open, and each row carries its
   verdict.** A row is exact (`Matches`) or open (`Undetermined` with its open
   rules). An exact row is in the game's selection however the open rules are
   settled, and every row the game could select is in the result. So the result
   is a superset of the game's selection, and it marks the rows that may be the
   difference. No row is dropped for being open, and no open row is shown as
   exact. Kleene logic is sound, not complete: `X Or Not X` over an open `X` is
   open, although it holds either way. The rule accepts that, because an open
   row is never wrong, only less precise.
3. **Number tests compare stored values with the domain's tolerance.**
   `Range { min, max }` over a number holds when `value + VALUE_TOLERANCE ≥ min`
   and `value − VALUE_TOLERANCE ≤ max`. So `sub_value(Spd) ≥ 17` agrees
   with 真 17 速 on every soul that has `Spd`.
4. **A field that this build cannot evaluate is refused.** A score field without
   a parameter set is `query.param_set_required`, as before. With one, until the
   pass that produces it exists, it is `query.field_unavailable`. A field that
   waits on the probe schema or the fold is not in the schema until it can be
   evaluated. Nothing is filled with a default.
5. **A scheme reference names one plan or scheme.** A code with more than one
   entry needs an index. The query does not choose between "any of" and "all of"
   for a discard code with several schemes, because the game's own meaning of
   that is not observed.
6. **Until the projection exists, the daemon evaluates a query over an inventory
   the request supplies.** The endpoint is headless and stateless. Each request
   frame gets exactly one response frame, and a refused request changes nothing.
   When the fold lands, the session's `Query` reads the projection instead, and
   this endpoint is either kept as a headless tool or removed with a minor
   version bump.

## Alternatives

- **Refuse a query that meets an open verdict.** Rejected: most selections with
  a chosen innate attribute would be unusable, and the decided rows are still
  exact.
- **Treat an open verdict as a match.** Rejected: under `Not` the superset turns
  into a subset. The result would then drop souls the game picks, and nothing
  would say so.
- **Evaluate the tree twice, once with every open rule read as true and once as
  false.** Rejected: an open rule is not one boolean shared by the whole tree,
  so two passes would assume a correlation the evidence does not give. Kleene
  logic assumes nothing about how open verdicts relate.
- **Exact comparison of number bounds.** Rejected: a stored 16.9999999 would
  fail `≥ 17` while the domain's own predicate calls it 17.

## Consequences

**Easier.** The filter is total, and so is evaluation after the tree is checked.
A result can always be produced, and it never overstates what the evidence
decides. When a rule is settled later (a soul's innate attribute, a newly mapped
scheme bit), open rows become exact or drop out, and no query changes.

**Harder.** A client shows two kinds of rows. `Not` over a scheme is no longer
an exact complement: an open row is in both results. `query.md` states this next
to the rule it qualifies.
