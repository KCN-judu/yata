# Scheme evaluation decides empty groups, includes, and counts

- Date: 2026-09-24
- Area: domain
- Affected: developers
- Related: ADR-0009

## What changed

- `yata-core::scheme::evaluate::matches` now decides the rules the game's filter
  was observed to follow: an empty group is no constraint, several included
  sub-attributes must all be present, and 数量 is the soul's number of
  sub-attributes. Verdicts that were `Undetermined` on those rules are now
  `Matches` or `DoesNotMatch`.
- `OpenRule` keeps only `Innate` (a chosen innate attribute) and
  `UnknownConditions`. `EmptyGroup`, `SeveralIncludes`, `SubCount` and `Group`
  are removed.
- `SubCount::of` places a number of sub-attributes in the editor's choices.
- The empty-group rule is observed for 等级 and 数量 and extrapolated to the
  other groups (`scheme-code.md`, "Evaluation").

## Compatibility and migration

Code that matched on the removed `OpenRule` variants drops those arms.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Scheme selection and
evaluation".
