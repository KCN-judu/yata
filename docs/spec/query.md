---
kind: spec
status: current
area: domain
---

# Query vocabulary

What a core-protocol `Query` or `Aggregate` may ask for: the collections, the
fields of each, and the filter, sort, and group expressions over them.
`core-protocol.md` defines pages, cursors, and revisions; this page defines what
goes inside a query.

Everything here is _designed_. Nothing is _implemented_.

## This is not a query language

ADR-0002 rules out a query language, and this page does not introduce one. A
query is a **closed, typed expression tree** carried as a protobuf message.
There is no text syntax, no parser, and no field the user can type by name. The
UI builds the tree from its controls; the daemon evaluates it as iterator
composition over the in-memory projection.

Closed means the vocabulary is exactly the fields and operators on this page. A
field is added by editing this page and the schema together, as an additive
minor version bump.

## Query shape

```text
Query {
  profile    : ProfileId
  collection : Collection          // what the rows are
  filter     : Expr                // absent = every row
  sort       : [SortKey]           // absent = the collection's default order
  params     : ParamSetRef?        // required if any score field is used
  page       : { row_budget, cursor }   // core-protocol.md
}

Aggregate {
  profile, collection, filter, params   // as above
  group_by   : GroupKey?           // absent = one group, the whole selection
  measures   : [Measure]
}
```

A query reads one profile. Comparing profiles is two queries; the core never
mixes rows from two profiles in one page.

## Collections

| Collection   | One row is                      | Row identity  |
| ------------ | ------------------------------- | ------------- |
| `Souls`      | one soul in the live projection | game soul id  |
| `GameAssets` | one item or realm card stack    | game asset id |

A soul that failed to decode is not a row (`scoring.md`: no score for an
undecodable soul). It is reported by the import that produced it, not hidden in
a query result.

## No nulls

**Every field is defined for every row of its collection.** There is no `NULL`,
no missing value, and no three-valued logic. Where the domain has an "absent"
case, the field's type names it:

- a soul not equipped has `equipped_by = Unequipped`, not a null Shikigami
- a sub-attribute a soul does not have has value `0` under `sub_value(attr)` and
  `false` under `has_sub(attr)`; "has it at zero" cannot occur, because a
  sub-attribute that exists has rolled at least once

A predicate is therefore always true or false, and `Not` is exact complement.

## Soul fields

| Field                                               | Type                                       | Source                                    |
| --------------------------------------------------- | ------------------------------------------ | ----------------------------------------- |
| `set`                                               | `SoulSet`                                  | snapshot                                  |
| `slot`                                              | `SoulSlot`                                 | snapshot                                  |
| `star`                                              | int 1–6                                    | snapshot                                  |
| `level`                                             | int 0–15                                   | snapshot                                  |
| `main_attribute`                                    | `SoulAttribute`                            | snapshot                                  |
| `main_value`                                        | number                                     | snapshot                                  |
| `sub_value(attr)`                                   | number                                     | snapshot                                  |
| `has_sub(attr)`                                     | bool                                       | snapshot                                  |
| `sub_count`                                         | int 0–4                                    | snapshot                                  |
| `initial_sub_count`                                 | int 0–4                                    | snapshot                                  |
| `pristine`                                          | bool                                       | derived: `PristineSoul` per `glossary.md` |
| `equipped_by`                                       | `Shikigami` id or `Unequipped`             | snapshot                                  |
| `game_locked`                                       | bool                                       | snapshot                                  |
| `mark`                                              | `Keep` · `Discard` · `Strengthen` · `None` | user fact                                 |
| `has_note`                                          | bool                                       | user fact                                 |
| `quality.total`, `quality.depth`, `quality.breadth` | score                                      | pass 1                                    |
| `affinity(need).value`                              | score                                      | pass 2                                    |
| `affinity(need).qualifies`                          | bool                                       | pass 2                                    |

Values are in the units the domain uses; this page does not restate them.

A field whose source is the snapshot exists in this vocabulary only once the
probe schema carries it. `equipped_by`, `game_locked`, and `initial_sub_count`
are listed because the legacy import read them; if the probe cannot, they are
removed from this table rather than filled with a default.

**Score fields require `params`.** A query that names a score field without a
`ParamSetRef` is refused with `query.param_set_required`. There is no implicit
current parameter set (`scoring.md`), and a score without its parameter set is
not a value this system produces.

## GameAsset fields

| Field      | Type                |
| ---------- | ------------------- |
| `category` | `GameAssetCategory` |
| `asset`    | the asset's id      |
| `count`    | int                 |

`GameAssetCategory` waits on the authoritative item category list
(`glossary.md`, `Item`). Until then this collection is designed but has no
filterable category.

## Filter expressions

```text
Expr =
    And [Expr]                     // empty And = true
  | Or  [Expr]                     // empty Or  = false
  | Not Expr
  | Pred { field, test }
  | Matches SoulSelection          // Souls only: the official filter, inline
  | MatchesScheme SchemeRef        // Souls only: a saved or pasted scheme
```

The test a predicate may apply is fixed by the field's type:

| Field type                                             | Tests                                                        |
| ------------------------------------------------------ | ------------------------------------------------------------ |
| enum (`SoulSet`, `SoulSlot`, `SoulAttribute`, mark, …) | `In [value]`                                                 |
| int, number, score                                     | `Range { min?, max? }`, both inclusive, at least one present |
| bool                                                   | `Is bool`                                                    |

A test that does not fit its field's type is `query.type_mismatch`. `In []` is
refused, not evaluated as false, because it is always a UI bug.

**`Matches`** is the official filter (roadmap milestone 1): the soul filter UI
builds a `SoulSelection`, the game's own condition model, and the daemon
evaluates it with `matches`. Every other `Expr` form is this project's extension
and is presented as the advanced filter.

**`MatchesScheme`** evaluates a scheme with `matches(selection, soul)`
(`scheme-code.md`). `SchemeRef` is a saved scheme id (`SchemeSaved` in
`fact-format.md`) or an inline scheme code string, plus a plan index when the
scheme is a strengthening scheme set. When the scheme has unknown conditions,
the result is a superset of the game's selection, and the page says so. A query
never re-implements a scheme's condition logic as an `Expr`; it asks the one
evaluator that exists.

## Sort

```text
SortKey { field, direction: Asc | Desc }
```

Any field except `has_sub(attr)` and `has_note` may be a sort key; sorting by a
bool is a filter in disguise. Enum fields sort by the order their type defines
in `glossary.md`, never by display text, because display text is Chinese UI copy
and not a domain order.

**Row identity is always the final key.** The daemon appends the collection's
row identity, ascending, after the client's keys. Two rows therefore never
compare equal, which is what makes the cursor well defined: it is the full
sort-key tuple of the last row returned.

The default order, when `sort` is absent, is row identity alone.

## Group and aggregate

**Grouping is an aggregate, not a paging mode.** A grouped view is two calls: an
`Aggregate` with `group_by` returns one row per group, and the rows of a group
are a `Query` with the group's key added to the filter. There is no page of
groups each holding a page of rows, because a cursor over two nested orders is a
second pagination protocol.

`GroupKey` is one enum field or one int field. Measures:

| Measure                     | Over                           |
| --------------------------- | ------------------------------ |
| `Count`                     | rows                           |
| `Sum`, `Min`, `Max`, `Mean` | an int, number, or score field |

A group with no rows is not returned. `Mean` over a group is computed in the
core; the client does not divide.

## Limits

| Limit                  | Value | Error               |
| ---------------------- | ----- | ------------------- |
| expression nodes       | 256   | `query.too_complex` |
| expression depth       | 16    | `query.too_complex` |
| sort keys              | 8     | `query.too_complex` |
| measures per aggregate | 16    | `query.too_complex` |

The limits exist to make a malformed or runaway tree a refusal instead of a slow
query. No UI control should come near them.

## Errors

All in the `query.*` namespace (`core-protocol.md`, "Errors"):

| Code                       | When                                                                             |
| -------------------------- | -------------------------------------------------------------------------------- |
| `query.unknown_field`      | a field this collection does not have                                            |
| `query.type_mismatch`      | a test that does not fit the field's type                                        |
| `query.param_set_required` | a score field with no `params`                                                   |
| `query.unknown_scheme`     | a `SchemeRef` that names no saved scheme, or an inline code that does not decode |
| `query.too_complex`        | a limit above is exceeded                                                        |
| `query.malformed_cursor`   | defined in `core-protocol.md`                                                    |
| `query.stale_revision`     | defined in `core-protocol.md`                                                    |

## Not decided here

- the `Match` call's inputs and result shape — it is a query, but its vocabulary
  is the matching result's, and belongs with `scoring.md` when the matching
  algorithm is decided
- whether `affinity(need)` fields may name more than one need in one query

## Related

- Why there is no query language:
  [ADR-0002](../decisions/0002-fact-log-projection-and-fact-schema.md)
- Pages, cursors, revisions: [core-protocol.md](core-protocol.md)
- Score definitions and the parameter set rule: [scoring.md](scoring.md)
- Scheme evaluation: [scheme-code.md](scheme-code.md)
- What the rows are built from: [fact-format.md](fact-format.md)
- Field vocabulary: [glossary.md](glossary.md)
