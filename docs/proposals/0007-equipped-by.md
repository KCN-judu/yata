---
id: PRP-0007
status: draft
date: 2026-09-25
area: import
related-issues: []
superseded-by: []
---

# PRP-0007: Which Shikigami wears a soul

## Problem

`query.md` lists `equipped_by` as a field a soul row can be filtered and sorted
by, because the prior tool read it. No reading of this project carries it: the
probe schema has no field for the Shikigami a soul is equipped on, and the
reader maps none. Until one does, the field cannot be evaluated, and `query.md`
already says that such a field is removed rather than filled with a default.

The maintainer decided on 2026-09-25 that the first version does not use this
field at all: no filter, no sort key, and no rule for where unequipped souls
sort. This proposal records the shape the field would have and the questions it
leaves, so a later version starts from them rather than from the prior tool.

## Goals and non-goals

- Goal: a typed description of "who wears this soul", ready for when a reading
  carries it.
- Goal: the questions a decision needs, listed.
- Non-goal: anything in the first version. The field stays out of the query
  schema until it can be evaluated (ADR-0026, rule 4).
- Non-goal: Yata's own loadout presets (the maintainer's answer to A11 of the
  2026-09-25 audit). A preset is a user's plan; `equipped_by` is what the game
  shows. They are different facts and are not merged.

## Proposed design

The types first. Everything marked `TBD` waits on a recording of the game.

```text
type ShikigamiInstanceRef = TBD     -- how the game identifies one Shikigami instance;
                                    -- two copies of one Shikigami are two instances
type Wearer = Unequipped
            | WornBy(ShikigamiInstanceRef)

-- on the wire: a field of SoulRecord with a Mapping, like every typed field
-- (probe-protocol.md, "Evidence"), so "the reader does not map it" and "the
-- soul is unequipped" are different values
equipped_by : Field<Wearer>         -- Unmapped | Mapped { evidence, value: Option<Wearer> }
```

`Unequipped` is a case, never a missing Shikigami or an id of 0. A soul whose
record lacks the field under a mapped `equipped_by` is `Missing`, as for any row
field.

As a query field and sort key, once it exists:

```text
sort key  equipped_by : SortKey { direction: Asc | Desc }
order     TBD: where Unequipped sorts relative to WornBy, and how instances order
```

## Compatibility and migration

Adding the field to `SoulRecord` is a minor version of the probe protocol (a new
optional field). Readings made before it carry no mapping for it, which reads as
unmapped. The query field appears only in a build that evaluates it.

## Alternatives

- Keep the prior tool's field with a default for "not read": rejected, since a
  default makes "not read" and "unequipped" the same value.
- Derive it from Yata's presets: rejected, since a preset is what the user
  plans, not what the game shows.

## Implementation and evidence

1. A recording of the desktop game that shows where the game keeps the wearer of
   a soul, and how it identifies a Shikigami instance.
2. The probe schema field, the reader's mapping with its evidence, and the
   domain type.
3. The query field and the sort key, with tests.

## Open questions

- How the game identifies a Shikigami instance (`ShikigamiInstanceRef`).
- Where `Unequipped` sorts, and whether it follows the key's direction.
- Whether the query also filters by the Shikigami (all copies) or only by the
  instance.

## Outcome

Open. Deferred past the first version by the maintainer, 2026-09-25.
