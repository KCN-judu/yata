---
id: PRP-0004
status: accepted
date: 2026-09-25
area: scoring
related-issues: []
superseded-by: []
---

# PRP-0004: The pass-2 contract — need profiles, set plans, and three levels of evaluation

## Problem

ADR-0003 fixes the shape of pass 2: `fit` scores one soul against one
`NeedProfile`, and `match` lifts it to the inventory. `scoring.md` sketches the
fields, and PRP-0003 records the direction that set affinity and Shikigami
affinity are both pass-2 fit. None of them is yet a contract an implementation
can be written against, and reading them together against the feature scope
exposes five defects.

1. **Floors and caps are stated per soul, but what they bound is a loadout.**
   `scoring.md` defines `floors` as "a minimum below which the soul is
   disqualified" and `qualifies` as "whether every floor is met". The quantities
   a player puts a floor or a cap on — a Shikigami's Effect HIT, its Crit, its
   Speed — are sums over six souls, their set effects, and the Shikigami's own
   base. One soul cannot meet an Effect HIT floor that six souls meet together,
   so `qualifies` has no meaning for a single soul.
2. **Pruning by `quality_floor` removes the inversions.** `scoring.md` prunes
   `match` by `quality.total` and says pruning "must not change the result". But
   pass 1 is not a bound on pass 2: a soul below its expected quality can be the
   best soul in the inventory for one need (P05 below). A quality threshold on
   candidates is therefore a policy that changes the result, and it removes
   exactly the rows ADR-0003 property 4 calls the most valuable.
3. **Sets are weighted per soul.** `sets` carries "how much they matter (a
   4-piece is a stronger constraint than a 2-piece)". Whether a four-piece
   effect is active is a fact about six souls; no single soul holds it, and a
   weight on one soul's set pretends it does.
4. **There is no hard/soft distinction.** A main attribute a need cannot use and
   a slightly low Speed are both "a mismatch", and nothing says which one
   disqualifies and which one only ranks lower.
5. **The comparison with current equipment has no inputs.** The feature scope
   makes "how much better than what this Shikigami wears now" the usual way into
   matching, but no record names what that comparison needs: the Shikigami's
   current souls, the Shikigami's own base values, and the set effects. The
   probe schema carries none of them yet.

## Goals and non-goals

Goals:

- A typed `NeedProfile` built from a closed vocabulary of constraint forms, with
  no scripting.
- A formal split between hard requirements and soft preferences, where a hard
  failure is never turned into a numeric penalty.
- The smallest `SetEffect` and set-plan model that matching needs.
- The binding from a Shikigami and a build to a profile.
- Three evaluation levels — one soul, one loadout, one swap — each with the
  constraints it can decide and no others.
- Structured, inspectable results in which every number traces to a profile item
  and its provenance.
- Worked examples and golden test vectors, and an implementation handoff.

Non-goals:

- **Any production code.** Pass 2 is not implemented until pass 1 is, and until
  an ADR accepts this contract.
- **Any change to pass 1.** The quality model is untouched. No holder-specific
  need enters `QualityScore`.
- **Combat formulas.** No damage, healing, or effect-hit-versus-resist
  arithmetic, and no conversion between flat and percentage attributes.
- **The assignment optimizer.** `match`'s search and its complexity bound are a
  later proposal; this one defines what `match` optimizes and what it compares
  against.
- **Game data.** No set effect, Shikigami base value, or build is stated as a
  fact here. Every profile below is a test fixture, and says so.

## Proposed design

Sections 1 to 9 are written as the draft of `scoring.md`'s pass-2 sections. On
acceptance they move there, and this proposal keeps only the rationale.

### 1. Claim kinds

Each statement carries the marks of [quality-model.md](../spec/quality-model.md)
and one more:

| Mark | Kind        | Rests on                                                          |
| ---- | ----------- | ----------------------------------------------------------------- |
| (A)  | game rule   | a rule in a spec page with its source                             |
| (B)  | derived     | a proof from (A) and the definitions here                         |
| (C)  | design      | this contract's choice; a change is a new contract version        |
| (D)  | calibration | a computation under stated inputs                                 |
| (F)  | fixture     | a value made up for a test; not game data, not a recommendation   |
| (U)  | unsourced   | a game rule the design needs and no page yet states with a source |

A (U) statement blocks the implementation of the rule that uses it, not the
contract.

### 2. Three levels of evaluation

A constraint is evaluated at the smallest level that can decide it, and at no
smaller one (C):

| Level       | Function   | Input                             | Decides                                                                                  |
| ----------- | ---------- | --------------------------------- | ---------------------------------------------------------------------------------------- |
| one soul    | `fit`      | a need and a soul                 | the slot's main attribute, whether the soul's set has a place, per-soul floors, affinity |
| one loadout | `evaluate` | a need, six slots, a baseline     | set plans, stat floors, ceilings and windows, saturation, the loadout's value            |
| one swap    | `swap`     | a need, a loadout, a slot, a soul | the change one replacement makes to everything `evaluate` decides                        |

```text
fit      : (params, need, soul)                            -> AffinityScore
evaluate : (params, need, loadout, baseline?)              -> LoadoutFit
swap     : (params, need, loadout, slot, soul, baseline?)  -> SwapDelta
match    : (params, inventory, needs)                      -> MatchingResult   // deferred
```

All four are total and pure (ADR-0001). `fit` keeps the signature ADR-0003 gave
it. `swap` is `evaluate` twice and a difference, so it adds no semantics of its
own; it is named because it is what the Shikigami collection asks.

The split is the answer to defects 1 and 3: **no statement about a sum over six
souls is ever made about one soul.** A soul's affinity says whether it is the
right kind of soul for the need. Whether the need's floors, caps, and set plans
hold is said only of a loadout.

### 3. Data shapes

Types are written in Rust syntax because the implementation is Rust; they are a
contract, not code. Identifiers follow the glossary.

```rust
/// One need: a Shikigami build's, a role's, or a set's usual use (C).
struct NeedProfile {
    id: NeedId,
    version: u32,
    subject: NeedSubject,
    provenance: Provenance,
    weights: AttributeWeights,        // soft: how much each attribute is worth
    slots: [SlotRule; 6],             // hard: accepted mains; soft: their ranking
    sets: Option<SetRule>,            // plans, hard or soft
    soul_floors: Vec<SoulFloor>,      // hard, per soul; expected to be rare
    targets: Vec<StatTarget>,         // loadout level: floors, ceilings, windows
    saturations: Vec<Saturation>,     // loadout level: value transforms
}

enum NeedSubject {
    Build { shikigami: ShikigamiId, label: String, role: Option<NeedId> },
    Role,                              // a generic need, such as "control"
    SetUse(SoulSet),                   // PRP-0003's set affinity (套装适配度)
}

/// ω_a ≥ 0 per attribute, absent = 0. Only ratios matter (B, § 5).
struct AttributeWeights(BTreeMap<SoulAttribute, Ratio>);

struct SlotRule {
    accepted: BTreeSet<SoulAttribute>, // hard; ⊆ Main(k), non-empty
    preferred: Vec<SoulAttribute>,     // soft; ranked, ⊆ accepted
}

struct SoulFloor {
    attribute: SoulAttribute,
    at_least: Decimal,                 // stored value of that sub-attribute
    basis: ItemBasis,
}

struct StatTarget {
    id: TargetId,
    quantity: Quantity,
    form: TargetForm,
    strength: Strength,
    basis: ItemBasis,
}

enum Quantity {
    /// Σ over the loadout's souls: main values, sub-attribute values, and
    /// two-piece attribute bonuses, in display units.
    Contribution(SoulAttribute),
    /// baseline(s) + Contribution(s); needs a baseline.
    Panel(PanelStat),
}

/// The attributes whose panel value is the base plus the souls' sum (U, § 6).
enum PanelStat { Spd, Crit, CritDmg, EffectHit, EffectRes }

enum TargetForm { AtLeast(Decimal), AtMost(Decimal), Within(Decimal, Decimal) }
enum Strength { Hard, Soft }

/// Value beyond `at` counts with weight `residual` ∈ [0, 1]; 0 is a cap.
struct Saturation { quantity: Quantity, at: Decimal, residual: Ratio, basis: ItemBasis }

enum Provenance {
    Derived { method: String, inputs: Vec<SourceRef> },
    Authored { author: String, date: Date, rationale: String },
}

/// Per item, where a single number's authority differs from the profile's.
enum ItemBasis {
    Profile,                           // as the profile's provenance
    GameRule(RuleRef),                 // must name a rule in a spec page
}
```

```rust
enum SetGroup { Any, OneOf(BTreeSet<SoulSet>) }
struct PlanPart { group: SetGroup, pieces: Pieces }   // Pieces ∈ {Two, Four}
struct SetPlan(Vec<PlanPart>);                        // Σ pieces ≤ 6
struct SetRule { plans: Vec<SetPlan>, strength: Strength } // ranked, non-empty

/// The catalogue entry matching reads (C). Values are game data (U until sourced).
struct SetEffect {
    set: SoulSet,
    two: TwoPiece,
    four: Option<EffectRef>,           // None: the set has no four-piece effect
    source: SourceRef,
}
enum TwoPiece { Attribute(SoulAttribute, Decimal), Other(EffectRef) }
```

```rust
/// Six slots; slot k holds a soul whose slot is k, or nothing.
struct Loadout([Option<Soul>; 6]);

/// What one owned Shikigami wears now, as read from the game.
struct EquippedLoadout { instance: ShikigamiInstanceId, shikigami: ShikigamiId, loadout: Loadout }

/// The Shikigami's own values, for Panel quantities.
struct Baseline { values: BTreeMap<PanelStat, Decimal>, source: BaselineSource }
enum BaselineSource { Read(ShikigamiInstanceId), Authored(Provenance) }
```

`ShikigamiId` is the game's identifier of a Shikigami, and `ShikigamiInstanceId`
the game's identifier of one owned copy, in the way `SoulSet` is the game's suit
code: the probe reads them, and nothing maps a name to them. An account can own
two copies of one Shikigami with different souls, so a profile binds the
Shikigami and a comparison binds the instance.

`ParamSet` gains three fields and relabels one:

| Field             | Meaning                                                                                   |
| ----------------- | ----------------------------------------------------------------------------------------- |
| `need_profiles`   | unchanged: builds, roles, and set uses                                                    |
| `set_catalogue`   | the `SetEffect` table and its version; two results are comparable only on the same one    |
| `inversion_rule`  | § 9; the default is two anchors, not a tuned number                                       |
| `candidate_floor` | was `quality_floor`: an optional per-need exclusion by `quality.total`, absent by default |

### 4. Well-formed profiles

A profile is checked when it is loaded. An ill-formed profile is refused with a
typed error and never scored (C).

```text
 ∀a. ω_a ≥ 0        ∃a. ω_a > 0
 ∀k. ∅ ≠ accepted(k) ⊆ Main(k)       ∀k. preferred(k) ⊆ accepted(k)
 ∀P ∈ plans.  Σ_{p ∈ P} pieces(p) ≤ 6
 ∀P, p ∈ P.  pieces(p) = 4  ⟹  ∀σ ∈ group(p). four(σ) ≠ None
 ∀ Within(lo, hi).  lo ≤ hi          ∀ saturation.  0 ≤ residual ≤ 1
 each quantity has at most one saturation
 ∀ GameRule(r).  r names a rule of a spec page
───────────────────────────────────────────────────────────── (W-Need)
 ⊢ need ok
```

For slots 1, 3 and 5, `Main(k)` has one element, so W-Need forces `accepted(k)`
to be it: a profile cannot reject a fixed main attribute. `GameRule` cannot cite
a rule that does not exist, so an unsourced cap is `Profile`, authored, until
its rule is written (§ 12).

### 5. One soul: gates and affinity

The domain is that of the quality model: six-star souls (C). Below six stars
`fit` returns `OutOfDomain`, because `hi(a)` is unsourced there.

**Gates.** Each is a hard requirement decidable on the soul alone:

```text
 m ∈ accepted(k)
─────────────────────── (H-Main)
 need ⊢ s main-ok

 sets = None  ∨  strength(sets) = Soft  ∨  ∃P ∈ plans. admits(P, σ)
─────────────────────────────────────────────────────── (H-Set)
 need ⊢ s set-ok

 ∀f ∈ soul_floors.  attribute(f) ∈ dom S  ∧  S(attribute(f)) ≥ at_least(f)
─────────────────────────────────────────────────────── (H-Floor)
 need ⊢ s floor-ok

admits(P, σ)  ⟺  ∃p ∈ P. σ ∈ group(p)  ∨  ∃p ∈ P. group(p) = Any  ∨  Σ_{p∈P} pieces(p) < 6
```

`admits` asks only whether the soul's set has a place in some plan: as a member
of a named group, in an `Any` part, or in a free slot. It never asks whether the
plan will be completed; that is a loadout fact (S-Plan).

**Affinity.** With the roll units `e_a = S(a)/hi(a)` of pass 1 and the need's
weights `ω`:

```text
U_n(s) = Σ_{a ∈ dom S} ω_a · e_a
M_n    = 6·ω₍₁₎ + ω₍₂₎ + ω₍₃₎ + ω₍₄₎        ω₍ᵢ₎ the i-th largest weight, 0 if fewer
E_n    = μ · Σ_a ω_a · E[K_a]
value  = g_n(U_n(s))                       g_n as g_p of quality-model.md, with E_n, M_n
```

- **0, 50, 100 mean what they mean in pass 1** (C, PRP-0003 item 4): nothing
  useful to this need, the expected +15 soul for this need, and the most this
  need can get from one soul.
- `M_n` is the largest `U_n` of any legal soul, and some soul attains it. The
  argument, to be proved in Lean before it is marked (B): a +15 soul has at most
  nine increments, at most six on one line, and at most four lines, so the best
  soul puts six on the heaviest attribute and one on each of the next three. The
  same count gives `0 < E_n < M_n` whenever some `ω_a > 0`, so `g_n` is well
  defined.
- `E[K_a]` is the expected number of increments a +15 soul puts on `a` under the
  reference measure, so `E_n` is linear in `ω` (B). Only `μ` of the increment's
  law enters (B, as in pass 1). The per-attribute values, derived from the
  published exact `E_output` and `E_hit` by linearity and symmetry (D):
  `μ·E[K_a] = 0.648926675` for 攻击类 and 防御类 attributes, `0.669528867`
  for 功能类 attributes. Their sum over all eleven is `μ · 8`, as it must be: a
  +15 soul whose initial count is 2, 3 or 4 with equal probability has eight
  increments on average.
- **Scaling every weight leaves every value unchanged** (B, from `g_scale`).
  Authored weights mean only their ratios.
- **The main attribute gates and does not add** (C). At +15 every soul with a
  given main has the same main value, so within a slot and main it cannot rank
  souls; between mains, `accepted` decides, and `preferred` ranks. The main
  value does enter a loadout's totals (§ 6), where it matters.
- **The set adds nothing to a soul's affinity** (C). § 7.
- Below +15, `growth` is `g_n(U_n + μ·E[ΔU_n])`, with `f` of quality-model.md
  weighted by `ω` in place of the indicator of `A_p`.

The value is **always computed**, including for a soul that fails a gate.
Admission is a separate field (§ 8); it never changes the value.

### 6. One loadout: totals, sets, targets, value

**Totals** (A for M-Main; C for the sum):

```text
T_a(L) = Σ_{s ∈ L} ( [m_s = a] · main(a, ℓ_s) + S_s(a) )  +  Σ_{σ active 2} bonus₂(σ, a)
```

`bonus₂(σ, a)` is the value of `σ`'s two-piece effect when it is
`Attribute(a, v)`, else 0. An innate attribute (固有属性) is not read yet and
does not enter until it is.

**Set activation and plans:**

```text
 count_L(σ) ≥ 2                           count_L(σ) ≥ 4     four(σ) ≠ None
──────────────── (S-Two, U)               ─────────────────────────────── (S-Four, U)
 active₂(L, σ)                             active₄(L, σ)

 ∃ injective π : P → SoulSet.  ∀p ∈ P.  π(p) ∈ group(p)  ∧  count_L(π(p)) ≥ pieces(p)
─────────────────────────────────────────────────────── (S-Plan, C)
 L ⊨ P
```

- S-Two and S-Four are the game's rule as the glossary names it (2-piece and
  4-piece variants); the thresholds and whether four pieces also activate the
  two-piece effect need a source before implementation (U).
- `π` is injective: one set realizes one part. A loadout of six souls of one set
  satisfies `[4 × {A}]` but not `[4 × {A}, 2 × Any]`. This is a choice (C), made
  so that no rule depends on how six pieces of one set behave, which is not
  sourced.
- `SetRule` is satisfied at rank `i` when `plans[i]` is the first plan `L`
  satisfies. A hard rule is `Unmet` when no plan is satisfied.

**Targets:**

```text
 q(L) = T_a(L)                                  for Contribution(a)
 q(L) = base(t) + T_t(L)                        for Panel(t), base present
 q(L) undetermined                              for Panel(t), base absent

 AtLeast(x): Met iff q ≥ x, else Unmet(x − q)
 AtMost(x):  Met iff q ≤ x, else Unmet(q − x)
 Within(lo, hi): Met iff lo ≤ q ≤ hi, else Unmet(distance to the interval)
```

A `Panel` target without a baseline is `Undetermined { missing: Baseline(t) }`.
It is never evaluated with a guessed or zero base (C), for the same reason a
soul that fails to decode is never scored.

That a panel value is the base plus the souls' sum is stated for the five
`PanelStat`s only, and is itself (U). Attack, Defense and HP combine a flat and
a percentage part with the base, which is a combat formula; they appear only as
`Contribution` quantities, per attribute.

**Value** (C):

```text
V(L)     = Σ_a ω_a · φ_a(q_a(L)) / hi(a)
φ_a(t)   = t                                         no saturation on a
φ_a(t)   = min(t, c) + r · max(0, t − c)             saturation (at, r); c = at, or at − base for Panel
```

`V` is in weighted roll units, the unit of `U_n`, so a soul's affinity and a
loadout's value speak about the same thing. A saturation makes value concave in
the total (B for `r ≤ 1`): past the knee, each further point is worth `r` of
what it was worth before. That is the reason pass 2 cannot be one more linear
pass-1 score — the worth of a soul's Crit depends on the Crit the other five
already bring (P08).

If any term needs a missing baseline, `V` is
`Undetermined { partial, missing }`: the determined terms are reported, and no
total is.

`V` is not normalized to 0–100. A loadout's value is only ever compared with
another loadout's for the same need and baseline, as a difference.

### 7. Sets: four answers, four places

| Question                                                    | Where it is answered             | How                                                                      |
| ----------------------------------------------------------- | -------------------------------- | ------------------------------------------------------------------------ |
| Can this soul's set have a place in this need's plans?      | gate, per soul (H-Set)           | hard only when the `SetRule` is hard                                     |
| Which role does the set play for this soul?                 | returned separately, per soul    | `SetStanding`: `Member { plan, part }`, `Filler { plan }`, `NotAdmitted` |
| Is the plan complete; is a four-piece effect active?        | loadout only (S-Plan)            | never on one soul                                                        |
| How well do this soul's rolls suit its own set's usual use? | `fit` against a `SetUse(σ)` need | PRP-0003's set affinity, an ordinary profile                             |

Set compatibility never adds to or multiplies a soul's affinity. Doing so would
put a number on a four-piece outcome that one soul cannot produce, and it would
hide a hard exclusion inside a scale. PRP-0003's set affinity needs no new
function: a set's usual use is a `NeedProfile` whose subject is `SetUse(σ)`, and
the maintainer's example — a 心眼 with Effect HIT maxed scores low as a 心眼 and
high for 丑时之女 — is two `fit` calls against two authored profiles.

**Set use only adds a keep suggestion** (the maintainer, 2026-09-25). A
`SetUse(σ)` result is never shown as a score beside quality, never changes a
soul's quality, tier, or sort position, never enters a Shikigami's affinity, and
a low set-use fit never produces a discard suggestion. Its one output is:

```text
 subject(n) = SetUse(σ)     set(s) = σ     fit(n, s) admissible
 U_n(s) ≥ 5μ · ω₍₁₎
─────────────────────────────────────────────────────── (K-SetUse)
 keep(s, because: SetUse(σ), carried by: the contributions of fit(n, s))
```

The bar is pass 1's SR milestone, five increments at mean value, in units of the
profile's heaviest weight: high enough that an ordinary soul of the set earns
nothing. A soul that quality already keeps gets one keep with both reasons, not
two.

**Where set-use profiles come from.** One profile per four-piece set, read from
the set's effect text. Each is authored: the reading of what an effect is
triggered by and scales with is a player's reading, not a game rule. A set whose
effect produces a heal or a shield is read for Crit and Crit DMG, since heals
and shields can crit, and for the stat the amount scales with. A set whose
effect scales with nothing of the equipper's (onibi, allies) gets no profile and
no keep suggestion. Two-piece-only sets get none. The first draft, with its
sources and the reading rules, is kept in local research, for the maintainer's
review.

### 8. Hard and soft

Each requirement evaluates to one of three outcomes (C):

```text
Outcome = Met | Unmet { shortfall } | Undetermined { missing }
```

**Hard requirements** combine into an admission, by three-valued conjunction:

```text
 ∃h. h Unmet                         → Inadmissible(the unmet ones)
 else ∃h. h Undetermined             → Undetermined(the missing inputs)
 else                                → Admissible
```

**Soft preferences** — the weights, `preferred` mains, the rank of the plan a
loadout satisfies, and soft targets — are reported as satisfied or violated,
each with its measure.

The order a result list takes is a rule, not a score (C):

1. Only `Admissible` results are ranked. `Inadmissible` results are listed as
   explanations, never as candidates. `Undetermined` results are listed apart,
   with the estimate marker `~` of soul-mechanics.md.
2. Among admissible results: value, then the number of soft preferences met,
   then `quality.total` (an unrated soul after every rated one), then the soul
   id, so the order is total and reproducible.

A hard failure therefore removes a result from ranking without changing its
value, and a soft preference reorders only among equal values. **No hard
requirement is ever expressed as a penalty,** because a penalty has to be given
a size, and every size is arbitrary: too small and an unusable soul outranks a
usable one, too large and it is a gate written in the wrong unit.

Whether a soft target should weigh more than a tie-breaker is open (§ Open
questions); the contract keeps it a tie-breaker until a case needs more.

### 9. Pruning and inversions

**Pruning.** The only pruning that must not change a result is by an admissible
bound derived from the need: a soul that fails a soul-level gate, or whose best
attainable swap cannot beat the current best, can be skipped. That is ADR-0003
property 3's "cannot reach the need's floors", read at the level where it is
decidable. Pruning by `quality.total` is not admissible: P05 has quality 34.1
and affinity 81.5 for one need. It stays available as `candidate_floor`, a
user's policy, absent by default, and a result produced under it says so.

**Unrated souls** (ADR-0027: no candidate archetype, so no `QualityScore`) are
never excluded by `candidate_floor`: there is no score to hold against the
floor, and pass 2 is the only reading such a soul has.

**Inversions.** `inversions` is computed from `fit` over every soul, never from
the pruned candidates. The default rule uses the two anchors the passes share:

```text
 admissible(fit(n, s))     quality(s) unrated  ∨  quality(s).total < 50     fit(n, s).value > 50
─────────────────────────────────────────────────────── (Inv)
 inversion(s, n),  ordered by fit(n, s).value − quality(s).total,  an unrated soul's quality taken as 0
```

That is: below the expected soul in itself, above the expected soul for this
need. No tuned number is involved; another rule is another `inversion_rule` in
the parameter set.

### 10. Results

The single-soul result, the evolved `AffinityScore`:

```rust
struct AffinityScore {
    need: NeedRef,                     // id and version
    params: ParamRef,                  // id and version
    admission: Admission,              // Admissible | Inadmissible(..) | Undetermined(..)
    hard: Vec<Check>,                  // every soul-level hard requirement, exactly once
    soft: Vec<Check>,                  // every soul-level soft preference, exactly once
    value: Score,                      // g_n(U_n), 0–100, comparable within (need, params)
    utility: RollUnits,                // U_n
    anchors: (RollUnits, RollUnits),   // (E_n, M_n)
    contributions: Vec<Contribution>,  // per sub-attribute: e_a, ω_a, ω_a·e_a, share of U_n
    wasted: Vec<(SoulAttribute, RollUnits)>, // e_a > 0 with ω_a = 0
    set_standing: SetStanding,
    growth: Option<Score>,             // below +15
    reasons: Vec<Reason>,
}

struct Check { item: ItemRef, strength: Strength, outcome: Outcome, basis: ItemBasis }
```

The loadout result and the swap:

```rust
struct LoadoutFit {
    admission: Admission,
    value: Determined<RollUnits>,      // Value(v) | Undetermined { partial, missing }
    terms: Vec<Term>,                  // per weighted attribute: total, counted, beyond the knee, ω·φ/hi
    plans: Vec<Check>,                 // the SetRule, with the rank satisfied
    targets: Vec<Check>,               // every target, hard and soft
    active_sets: Vec<(SoulSet, Pieces)>,
    reasons: Vec<Reason>,
}

struct SwapDelta {
    slot: SoulSlot,
    outgoing: Option<SoulId>,
    incoming: SoulId,
    incoming_worn_by: Option<ShikigamiInstanceId>, // a conflict to show, not to resolve
    before: LoadoutFit,
    after: LoadoutFit,
    delta: Determined<RollUnits>,      // V(after) − V(before)
    per_attribute: Vec<TermDelta>,
    newly_met: Vec<ItemRef>,
    newly_unmet: Vec<ItemRef>,
}
```

What the Shikigami collection shows for one soul and one Shikigami is the pair:

```rust
struct SoulMatch { affinity: AffinityScore, vs_current: Option<SwapDelta> }
```

`vs_current` is present when the Shikigami's current loadout is known, and
`SwapDelta.after.admission` decides whether the swap is a candidate. `match`'s
`MatchingResult` keeps its shape in `scoring.md`; its `candidates` become
`SoulMatch`es, and `best` and `delta_vs_current` are the optimizer's, later.

### 11. Explainability

Every result is a value and its reasons, and the reasons are structured (C):

```rust
enum Reason {
    MainNotAccepted { slot: SoulSlot, main: SoulAttribute, accepted: Vec<SoulAttribute> },
    SetNotAdmitted { set: SoulSet },
    SoulFloorShort { attribute: SoulAttribute, at_least: Decimal, have: Decimal },
    CarriedBy { attribute: SoulAttribute, share: Ratio },
    Wasted { attribute: SoulAttribute, roll_units: RollUnits },
    TargetShort { target: TargetId, shortfall: Decimal },
    TargetExceeded { target: TargetId, excess: Decimal },
    Saturated { quantity: Quantity, beyond: Decimal },
    PlanSatisfied { rank: usize },
    PlanBroken { plan: usize, set: SoulSet, have: u8, need: u8 },
    Undetermined { missing: Missing },
    Inversion { quality: Score, value: Score },
}
```

The UI renders a `Reason` in Chinese; the core never produces text. The
invariants, each a property test:

- **E1.** `utility = Σ contributions.weighted`, and `value = g_n(utility)` with
  the reported anchors.
- **E2.** Every soul-level hard requirement of the profile appears in `hard`
  exactly once, every soft preference in `soft` exactly once, and `admission` is
  the three-valued conjunction of `hard`.
- **E3.** `V = Σ terms`, and `delta = Σ per_attribute`.
- **E4.** Reasons are a pure function of the checks and terms, so they cannot
  disagree with them.
- **E5.** Every number that a result reports and that is not computed from the
  soul traces to a profile item, and every item carries its basis: `Profile`
  under an `authored` or `derived` provenance, or a `GameRule` naming its rule.

### 12. Provenance

- A profile is `derived` or `authored` (ADR-0003 property 2). Every profile this
  project can write today is `authored`: derived profiles need the damage model,
  deferred in the feature scope.
- A number inside an authored profile can still rest on a game rule, and then it
  says so per item. A Crit saturation at 100 is a player's reading of the game
  until a spec page states the rule with a source; until then it is
  `ItemBasis::Profile` under the author's name, and the UI shows it as a
  judgment.
- Weights are always the profile's basis. No weight is a game rule.
- A player heuristic ("a control build wants Effect HIT first") is recorded as
  an authored profile's rationale. It is never written into a spec page as a
  rule.

### 13. Worked examples

**The maintainer's example, in this model.** PRP-0003's 心眼 soul with Effect
HIT maxed is scored twice: against `SetUse(心眼)`, an authored profile of the
set's usual use, and against `Build { shikigami: 丑时之女, .. }`, an authored
profile whose weights include Effect HIT and whose `SetRule` names 心眼. The
first value is low and the second high, and both are `fit`; no set knowledge
enters pass 1, and nothing new is needed. The two profiles' weights are the
maintainer's to write; none is stated here.

**Why a better soul can make a worse loadout** (P08). A fixture output need
values Crit, Crit DMG and ATK% equally and Speed at half, and caps Crit on the
panel at 100 against a fixture base of 10. Its Shikigami wears six souls whose
Crit sums to 88, a panel of 98. A slot-3 soul with Crit 15.0, Crit DMG 4.0, ATK%
3.0 and Speed 3.0 has a higher affinity (91.96) than the slot-3 soul it would
replace (83.92), whose lines are Crit 3.0, Crit DMG 12.0, ATK% 6.0 and Speed
3.0. The swap raises the panel to 110, so 10 points of the new Crit are past the
cap: the loadout gains 2 counted points of Crit (0.667 roll units) and loses 8
of Crit DMG (−2) and 3 of ATK% (−1), a net −2.333. The soul is the better soul
for the need and the worse soul for this Shikigami now, and the result says why:
`Saturated { Panel(Crit), beyond: 10.0 }`.

**The inversion** (P05). A slot-5 soul with Effect RES 16.0, DEF% 5.4, HP 228
and ATK 27 is well below its expected soul in itself: quality 34.13, tier N,
because no archetype of `yata-quality-v2` values Effect RES, and its one useful
line is DEF% under `hit`. For a fixture need valuing Effect RES at 1 and HP% and
DEF% at 1/2, it scores 81.52, far above that need's expected soul, and appears
in `inversions`. The same need takes a slot-4 Effect RES soul that pass 1 leaves
unrated (P19).

### 14. Test vectors

All profiles, bases and sets below are fixtures (F). The numbers are computed
with the exact constants of `yata-quality-v2`; the values of `E_n` use the
per-attribute expectations of § 5 (D). Souls are six-star and +15. Values are
stored values in display units, with each line's increment count in parentheses.

Fixture needs:

| Need   | Weights ω                                         | Accepted mains, slots 2 / 4 / 6                             | `E_n`    | `M_n` |
| ------ | ------------------------------------------------- | ----------------------------------------------------------- | -------- | ----- |
| N-OUT  | Crit 1, CritDmg 1, AtkPercent 1, Spd 1/2          | Spd AtkPercent / AtkPercent / Crit CritDmg                  | 2.281544 | 8.5   |
| N-CTL  | EffectHit 1, Spd 1, HpPercent 1/4, DefPercent 1/4 | Spd / EffectHit / HpPercent DefPercent                      | 1.663521 | 7.5   |
| N-HEAL | HpPercent 1, Crit 3/4, Spd 3/4                    | Spd HpPercent / HpPercent / Crit HpPercent                  | 1.637768 | 7.5   |
| N-SPD  | Spd 1                                             | Spd / any / any                                             | 0.669529 | 6     |
| N-RES  | EffectRes 1, HpPercent 1/2, DefPercent 1/2        | any / EffectRes HpPercent DefPercent / HpPercent DefPercent | 1.318456 | 7     |

Fixture souls:

| Soul | Slot, main    | Sub-attributes                                                              | Quality (v2)      |
| ---- | ------------- | --------------------------------------------------------------------------- | ----------------- |
| A1   | 6, CritDmg    | Crit 8.4 (3); Spd 5.4 (2); AtkPercent 5.4 (2); HpFlat 205.0 (2)             | output 79.64 SSR  |
| A2   | 4, EffectHit  | EffectHit 16.0 (4); Spd 5.4 (2); HpPercent 2.7 (1); AtkFlat 54.0 (2)        | hit 81.93 SSR     |
| A3   | 6, Crit       | HpPercent 11.4 (4); Crit 5.4 (2); Spd 2.7 (1); DefFlat 10.0 (2)             | healing 80.42 SSR |
| A4   | 2, Spd        | Spd 16.2 (6); AtkFlat 27.0 (1); DefFlat 5.0 (1); HpFlat 114.0 (1)           | speed 94.37 UR    |
| A6   | 5, HpFlat     | Crit 16.5 (6); CritDmg 3.6 (1); AtkPercent 2.7 (1); HpFlat 100.0 (1)        | output 86.68 SP   |
| A7   | 4, AtkPercent | EffectHit 16.0 (4); Spd 8.1 (3); HpPercent 2.7 (1); DefFlat 5.0 (1)         | output 50.66 R    |
| A8   | 5, HpFlat     | EffectRes 16.0 (4); DefPercent 5.4 (2); HpFlat 228.0 (2); AtkFlat 27.0 (1)  | hit 34.13 N       |
| A9   | 4, EffectRes  | EffectRes 16.0 (4); HpPercent 5.4 (2); DefPercent 5.4 (2); AtkFlat 27.0 (1) | unrated           |

Single-soul vectors:

| Id  | Case                        | Soul, need  | `U_n`  | Value | Admission                                                       | Also asserted                                                                                             |
| --- | --------------------------- | ----------- | ------ | ----- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| P01 | output soul, output need    | A1, N-OUT   | 5.5000 | 75.88 | admissible                                                      | `CarriedBy(Crit)` first, share 2.8/5.5                                                                    |
| P02 | high-hit soul, control need | A2, N-CTL   | 6.0250 | 87.36 | admissible                                                      | against N-OUT: 19.72, `MainNotAccepted(Slot4, EffectHit)`                                                 |
| P03 | healing soul, healing need  | A3, N-HEAL  | 5.8250 | 85.71 | admissible                                                      | quality healing SSR 80.42: not an inversion                                                               |
| P04 | pure-speed soul, speed need | A4, N-SPD   | 5.4000 | 94.37 | admissible                                                      | wasted: AtkFlat 1.0, DefFlat 1.0, HpFlat 1.0; quality speed UR 94.37, the same number by the same anchors |
| P05 | low quality, high affinity  | A8, N-RES   | 4.9000 | 81.52 | admissible                                                      | `inversion(A8, N-RES)`, gap 47.39                                                                         |
| P06 | high quality, low affinity  | A6, N-CTL   | 0.0000 | 0.00  | admissible                                                      | quality SP 86.68; wasted Crit 5.5, CritDmg 0.9, AtkPercent 0.9, HpFlat 0.877                              |
| P07 | hard-main mismatch          | A7, N-CTL   | 6.9250 | 95.07 | inadmissible: `MainNotAccepted(Slot4, AtkPercent, [EffectHit])` | value reported unchanged; absent from ranking and from `inversions`                                       |
| P13 | weights scaled              | A1, 2·N-OUT | 11.000 | 75.88 | admissible                                                      | every value equal to P01's                                                                                |
| P19 | unrated in pass 1           | A9, N-RES   | 5.8000 | 89.44 | admissible                                                      | `inversion(A9, N-RES)`, ordered with quality 0; never excluded by `candidate_floor`                       |

Loadout vectors use one fixture loadout `L0` for N-OUT, all souls +15, with
N-OUT's saturation `Panel(Crit)` at 100, residual 0, and fixture base Crit 10:

| Slot | Main (value)    | Sub-attributes                                                     | Set  |
| ---- | --------------- | ------------------------------------------------------------------ | ---- |
| 1    | AtkFlat (486)   | Crit 9.0 (3); CritDmg 8.0 (2); AtkPercent 3.0 (1); Spd 3.0 (1)     | SetA |
| 2    | AtkPercent (55) | Crit 6.0 (2); CritDmg 4.0 (1); Spd 6.0 (2); HpFlat 114.0 (1)       | SetA |
| 3    | DefFlat (104)   | Crit 3.0 (1); CritDmg 12.0 (3); AtkPercent 6.0 (2); Spd 3.0 (1)    | SetA |
| 4    | AtkPercent (55) | Crit 9.0 (3); CritDmg 4.0 (1); Spd 3.0 (1); HpFlat 114.0 (1)       | SetA |
| 5    | HpFlat (2052)   | Crit 6.0 (2); CritDmg 8.0 (2); AtkPercent 3.0 (1); Spd 3.0 (1)     | SetB |
| 6    | Crit (55)       | CritDmg 8.0 (2); AtkPercent 6.0 (2); Spd 3.0 (1); HpFlat 114.0 (1) | SetB |

Totals: Crit 88 (panel 98), CritDmg 44, AtkPercent 128, Spd 21; `V(L0)` =
29.333 + 11.000 + 42.667 + 3.500 = **86.500**.

Candidates: `C3`, slot 3, DefFlat main, set SetC: Crit 15.0 (5); CritDmg 4.0
(1); AtkPercent 3.0 (1); Spd 3.0 (1). `C5`, slot 5, HpFlat main, set SetC: Crit
3.0 (1); CritDmg 16.0 (4); AtkPercent 3.0 (1); Spd 3.0 (1).

| Id  | Case                            | Setup                                                                 | Expected                                                                                                                    |
| --- | ------------------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| P08 | cap saturation                  | `swap(N-OUT, L0, Slot3, C3)`                                          | standalone: C3 91.96 > outgoing 83.92; `V` 86.500 → 84.167, delta −2.333; Crit counted +2.0, `Saturated(Panel(Crit), 10.0)` |
| P09 | a swap breaks a hard four-piece | N-OUT with hard `SetRule [[4 × {SetA}]]`; `swap(.., L0, Slot3, C3)`   | before admissible; after inadmissible, `PlanBroken { set: SetA, have: 3, need: 4 }`; delta still −2.333, reported           |
| P10 | a filler keeps the plan         | same rule; `swap(.., L0, Slot5, C5)`                                  | after admissible, plan satisfied at rank 0; Crit 85, CritDmg 52; delta +1.000                                               |
| P11 | set standing on one soul        | C3 under `[[4 × {SetA}]]`, then under `[[4 × {SetA}, 2 × {SetB}]]`    | first `Filler`, admitted; second `NotAdmitted`, and with a hard rule `SetNotAdmitted(SetC)`; value 91.96 both times         |
| P12 | a panel target without a base   | P08 with no baseline                                                  | Crit term and `V` `Undetermined { missing: Baseline(Crit) }`; the other three terms reported; no total, no delta            |
| P14 | the same cap on a contribution  | P08 with `Contribution(Crit)` saturating at 90, authored, no baseline | as P08 exactly; the item's basis is `Profile`, not `GameRule`                                                               |

Speed vectors use N-SPD with a hard `Panel(Spd) Within(180, 186)` and a fixture
base Speed 100. Loadout `L1`: A4 in slot 2 (main 57, sub 16.2) and five souls
whose Speed lines sum to 12.0.

| Id  | Case                  | Setup                                             | Expected                                             |
| --- | --------------------- | ------------------------------------------------- | ---------------------------------------------------- |
| P15 | window met            | `evaluate(N-SPD, L1)`                             | panel 185.2, target met, admissible                  |
| P16 | too slow              | swap slot 2 for a Spd-main soul with Spd 2.7 (1)  | panel 171.7, `TargetShort(8.3)`, inadmissible        |
| P17 | too fast              | swap slot 2 for a Spd-main soul with Spd 18.0 (6) | panel 187.0, `TargetExceeded(1.0)`, inadmissible     |
| P18 | window without a base | P15 with no baseline                              | target `Undetermined`, admission `Undetermined`, `~` |

P16 and P17 are the case a scalar cannot express: the faster soul fails the same
need as the slower one, because the window stands for an order in the team.

## Compatibility and migration

Nothing changes until an ADR accepts a contract. On acceptance:

- **ADR.** A new ADR records the contract: the three levels, hard gates never
  scored, sets never added to a soul's affinity, and the inversion rule. It
  appends a dated amendment to ADR-0003 property 3, which this proposal reads as
  "prune by what the need makes decidable", and records that pruning by
  `quality.total` is a policy, not a correctness-preserving step. ADR-0003's
  other properties are unchanged.
- **`scoring.md`.** § Pass 2 and § Matching are replaced by sections 2–12 here.
  `qualifies` becomes `admission`; `blocking` and `missing` become the `hard`
  checks and their outcomes; `wasted` keeps its name. `quality_floor` becomes
  `candidate_floor`, which never excludes an unrated soul. The worked example is
  replaced by P05 and P08.
- **`query.md`.** `affinity(need).qualifies` becomes `affinity(need).admitted`.
- **`glossary.md`.** New rows: `NeedProfile`, `Loadout`, `SetPlan`, `Baseline`,
  `ShikigamiInstanceId`. The CN terms need sourcing where they are game terms.
- **`soul-mechanics.md`, or a new set-mechanics page.** S-Two, S-Four, and the
  panel rule of the five `PanelStat`s, each with its source, before the loadout
  level is implemented.
- **Probe schema.** Shikigami records — the Shikigami id, the instance id,
  level, star, evolution, the six equipped soul ids, and the base values if the
  game stores them — before `swap` against current equipment can run on real
  data.
- **The quality model.** Unchanged.

## Alternatives

- **One scalar, hard mismatches as penalties.** Rejected (§ 8): the penalty's
  size is arbitrary, and the result can no longer say that a soul is unusable.
- **Floors and caps on single souls**, as `scoring.md` states them now.
  Rejected: undefined for loadout sums (defect 1). Per-soul floors survive as
  `soul_floors`, for the rare need that constrains one soul.
- **Set as an affinity bonus or multiplier.** Rejected (§ 7): it scores a
  four-piece outcome one soul cannot produce.
- **Set affinity as a separate function over a separate per-set profile type**
  (PRP-0003, item 1, read literally). Superseded by `SetUse(σ)`: the same
  function over an ordinary profile, so set and Shikigami affinity cannot drift
  apart.
- **An expression language for needs.** Rejected: an open language cannot be
  checked by W-Need, cannot be explained reason by reason, and turns every
  profile into code. The closed vocabulary covers every case in § 14.
- **Profile inheritance** (a build overrides a role). Deferred: a build names
  its role for explanation, and holds a complete profile. Inheritance can be
  added when duplicated profiles become a real cost.
- **General value curves.** Rejected for now: one knee with a residual weight
  covers caps and diminishing value, and keeps value concave and explainable as
  "counted" and "beyond".
- **Normalizing a loadout's value to 0–100.** Deferred: loadouts are compared by
  difference only, and no anchor for "the expected loadout" is defined.
- **The main attribute's value in a soul's affinity.** Rejected for the same
  reason as in pass 1, and redundant: at +15 it is equal for every soul with
  that main, and it enters the loadout totals.

## Implementation and evidence

The handoff, after pass 1 is _implemented_ and _tested_ in `yata-core` and an
ADR accepts this contract:

1. **Types and W-Need.** `NeedProfile` and its parts, profile loading and every
   W-Need refusal as a typed error, in `yata-core` (a `scoring` module, split by
   ADR-0005's triggers only). Tests: one refusal per W-Need premise.
2. **Constants.** `formal/calibration` emits `μ·E[K_a]` per attribute as exact
   fractions, and `E_n` for the fixture needs; § 5's two decimals are then
   generated, not typed. Lean: `M_n` is the attainable maximum; scale
   invariance; `U_n` monotone under dominance on the weighted attributes;
   admission independent of value.
3. **`fit`.** Gates, value, contributions, `set_standing`, `growth`. Tests: P01
   to P07, P11, P13, P19; properties E1, E2, E4.
4. **`evaluate`.** Totals, S-Plan, targets, saturation, `Determined`. Blocked on
   the (U) rules of § 6 being written with sources, and on the set catalogue.
   Tests: P12, P14, P15, P18; E3; concavity of `φ`.
5. **`swap`.** Tests: P08 to P10, P16, P17; `swap(L, k, L[k])` has delta 0;
   `delta(L → L′) = −delta(L′ → L)`.
6. **Inversions.** Over `fit`, not over candidates. Test: P05 and P19 in, P03
   and P07 out; a property test that `candidate_floor` never removes an
   inversion.
7. **Profiles.** The profile file format, its review, and its versioning; then
   the first authored profiles, each reviewed by the maintainer.
8. **`match` and the optimizer.** A separate proposal with the search and its
   complexity bound.

Status on acceptance: the pass-2 row of `status.md` stays _designed_ until step
3 lands with its tests.

## Open questions

- **Set rules.** Whether four pieces also activate the two-piece effect, and how
  six pieces of one set behave, need a source (U). Which sets have no four-piece
  effect, and each two-piece bonus, have a community source in the Chinese
  client's text: the 灰机 wiki's soul data marks 13 of the 70 sets as
  boss-style, with a random base-stat bonus and a two-piece effect only, and
  gives the two-piece bonus of the other 57. It is a community wiki, not a
  NetEase publication, and ranks accordingly.
- **Panel rules.** Whether Speed, Crit, Crit DMG, Effect HIT and Effect RES are
  the base plus the souls' sum, and whether Crit above 100 has any effect (U).
- **Team order.** An order between two Shikigami ("A acts before B") is a
  constraint on two loadouts at once. The shape would be a `TeamNeed` holding
  needs and `Order { faster, slower, margin }`; it needs the optimizer, and a
  window on one need is its authored stand-in until then.
- **Soft targets.** Whether a violated soft target should rank below a smaller
  value, not only break ties.
- **Innate attributes.** How a boss soul's 固有属性 enters totals, once the
  reader reads it.
- **Baselines.** Whether the game stores a Shikigami's base values in a form the
  reader can read, or whether baselines are always authored.
- **Where a profile lives.** A data file in the parameter set, its format, and
  who reviews an authored profile.
- **The inversion rule.** Whether "below 50, above 50" is the rule players want,
  or whether a gap is.
- **Several builds per Shikigami.** Which build a comparison uses by default;
  that choice is a user fact, and would be a new fact kind.

## Outcome

Accepted by the maintainer, 2026-09-25 (→ ADR-0028). Sections 2–12 moved to
`scoring.md`. Before acceptance the maintainer decided that set use only adds
keep suggestions (§ 7, K-SetUse), and the vectors were recomputed under
`yata-quality-v2` (ADR-0027), which also settled how an unrated soul is treated
(§ 9).
