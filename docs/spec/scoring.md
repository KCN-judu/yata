---
kind: spec
status: current
area: scoring
---

# Scoring

What the two scoring passes mean, what they take, what they return, and what
makes two results comparable. The architectural shape is ADR-0003; this page is
the definitions.

Nothing on this page is _implemented_. The functions are _designed_: the
signatures and the comparability rules below are what the implementation must
satisfy. Pass 1 is defined exactly by [quality-model.md](quality-model.md), and
pass 2 by § Pass 2 (ADR-0028). No need profile is authored in the public
repository yet.

## The functions

```text
quality  : (params: ParamSet, soul: Soul)                                   -> QualityScore
fit      : (params: ParamSet, need: NeedProfile, soul: Soul)                -> AffinityScore
evaluate : (params: ParamSet, need: NeedProfile, loadout: Loadout, baseline: Option<Baseline>) -> LoadoutFit
swap     : (params: ParamSet, need: NeedProfile, loadout: Loadout, slot: SoulSlot, soul: Soul, baseline: Option<Baseline>) -> SwapDelta
match    : (params: ParamSet, inv: Inventory, needs: [NeedProfile])        -> MatchingResult
```

All of them are total, pure functions of their arguments (ADR-0001). None reads
a clock, a file, a random source, or a global. None has a side effect. Given the
same arguments they return the same value, which is what makes every result on
this page reproducible from a fixture.

`params` is not optional and not implicit. There is no "current weights" global;
a score without its parameter set is not a value this system produces.

## Parameter set

A `ParamSet` carries an identity, a version, and everything the three functions
read:

| Field             | Meaning                                                                                                                                   |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `id`              | a stable identifier for this set (`legacy-compatible`, `speed-farming`, …)                                                                |
| `version`         | bumped whenever any weight or floor in the set changes                                                                                    |
| `quality_model`   | the pass-1 model and its parameters: `yata-quality-v2`, whose catalogue, anchors, and thresholds are [quality-model.md](quality-model.md) |
| `need_profiles`   | the profiles pass 2 scores against: Shikigami builds, roles, and set uses                                                                 |
| `set_catalogue`   | the `SetEffect` table and its version                                                                                                     |
| `inversion_rule`  | which souls are inversions; the default is § Pruning and inversions                                                                       |
| `candidate_floor` | per need, an optional exclusion of candidates by `quality.total`; absent by default                                                       |

Two results are **comparable** only if produced by an equal `(id, version)`. The
system never merges results across parameter versions, and never displays a
stored score without its parameter set identity. A user switching parameter sets
sees a re-computation, not a re-sort of previously computed numbers, because the
previously computed numbers are not the same numbers.

`candidate_floor` exists per need rather than globally because "too weak to
bother" is a property of a need, not of a soul. It is a policy, not a
correctness step: pass 1 does not bound pass 2 (§ Pruning and inversions).

## Pass 1 — `QualityScore`

Answers: _is this Soul good in itself, with no holder in mind?_ The input
contains no Shikigami. The exact definition is
[quality-model.md](quality-model.md) (ADR-0024, ADR-0027); this section states
only what pass 2 relies on.

| Field       | Meaning                                                                                          |
| ----------- | ------------------------------------------------------------------------------------------------ |
| `total`     | 0–100, the single comparable number, for sorting and thresholds                                  |
| `tier`      | N, R, SR, SSR, SP or UR                                                                          |
| `archetype` | the best-fit archetype of the published catalogue, and the score under every candidate archetype |
| `depth`     | the deepest useful line against its attainable maximum; explanatory                              |
| `breadth`   | how many useful lines carry the utility; explanatory                                             |
| `slot_fit`  | which archetypes accept the soul's main attribute, and which of them are eligible                |
| `growth`    | below +15, the score of the expected +15 soul                                                    |

What pass 2 may rely on:

- **Values are relative to a roll, not absolute.** A sub-attribute counts as its
  value over its largest single increment, so HP flat, which rolls in the
  hundreds, does not outweigh Crit. The legacy model had this defect.
- **`depth` and `breadth` explain `total`; they are never added into it.** Which
  shape is better depends on the holder, and that is pass 2's question.
- **The main attribute gates, it does not score.** `slot_fit` states which
  archetypes can use the main attribute. A main attribute illegal for its slot
  is a decode error, never a low score.
- **A soul may be unrated.** A slot 4 EffectRes soul whose Speed is below the
  `speed` gate has no candidate archetype; its `QualityScore` is absent, not 0.
  `candidate_floor` never excludes it, and it is an inversion for any need it
  fits above 50 (§ Pruning and inversions).
- **`total` is monotone.** A soul no worse on every useful sub-attribute never
  scores lower, and never takes a lower tier.

## Pass 2 — affinity

Answers: _for this Shikigami, build, or set, how useful is this soul?_ The
contract is ADR-0028; the rationale, the worked cases, and the golden vectors
are in PRP-0004. It never redefines quality: a soul of low quality may fit one
need very well, and a soul of high quality may fit one need badly, and both are
results.

### Claim kinds

Each statement carries the marks of [quality-model.md](quality-model.md) and one
more:

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

### Three levels of evaluation

A constraint is evaluated at the smallest level that can decide it, and at no
smaller one (C):

| Level       | Function   | Input                             | Decides                                                                                  |
| ----------- | ---------- | --------------------------------- | ---------------------------------------------------------------------------------------- |
| one soul    | `fit`      | a need and a soul                 | the slot's main attribute, whether the soul's set has a place, per-soul floors, affinity |
| one loadout | `evaluate` | a need, six slots, a baseline     | set plans, stat floors, ceilings and windows, saturation, the loadout's value            |
| one swap    | `swap`     | a need, a loadout, a slot, a soul | the change one replacement makes to everything `evaluate` decides                        |

The signatures are in § The functions. `fit` keeps the one ADR-0003 gave it.
`swap` is `evaluate` twice and a difference, so it adds no semantics of its own;
it is named because it is what the Shikigami collection asks.

The split is the rule that **no statement about a sum over six souls is ever
made about one soul.** A soul's affinity says whether it is the right kind of
soul for the need. Whether the need's floors, caps, and set plans hold is said
only of a loadout.

### Data shapes

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

/// ω_a ≥ 0 per attribute, absent = 0. Only ratios matter (B, § One soul).
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

/// The attributes whose panel value is the base plus the souls' sum (U, § One loadout).
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

The parameter set's pass-2 fields are in § Parameter set.

### Well-formed profiles

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
its rule is written (§ Provenance).

### One soul: gates and affinity

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
  value does enter a loadout's totals (§ One loadout), where it matters.
- **The set adds nothing to a soul's affinity** (C), § Sets.
- Below +15, `growth` is `g_n(U_n + μ·E[ΔU_n])`, with `f` of quality-model.md
  weighted by `ω` in place of the indicator of `A_p`.

The value is **always computed**, including for a soul that fails a gate.
Admission is a separate field (§ Hard and soft); it never changes the value.

### One loadout: totals, sets, targets, value

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
already bring (PRP-0004, P08).

If any term needs a missing baseline, `V` is
`Undetermined { partial, missing }`: the determined terms are reported, and no
total is.

`V` is not normalized to 0–100. A loadout's value is only ever compared with
another loadout's for the same need and baseline, as a difference.

### Sets

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

**Set use only adds a keep suggestion** (ADR-0028). A `SetUse(σ)` result is
never shown as a score beside quality, never changes a soul's quality, tier, or
sort position, never enters a Shikigami's affinity, and a low set-use fit never
produces a discard suggestion. Its one output is:

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
no keep suggestion. Two-piece-only sets get none. The profiles are kept in local
research until the parameter set has a profile file format.

### Hard and soft

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

Whether a soft target should weigh more than a tie-breaker is open (§ Not
decided here); the contract keeps it a tie-breaker until a case needs more.

### Pruning and inversions

**Pruning.** The only pruning that must not change a result is by an admissible
bound derived from the need: a soul that fails a soul-level gate, or whose best
attainable swap cannot beat the current best, can be skipped. That is ADR-0003
property 3's "cannot reach the need's floors", read at the level where it is
decidable. Pruning by `quality.total` is not admissible: PRP-0004's P05 has
quality 34.1 and affinity 81.5 for one need. It stays available as
`candidate_floor`, a user's policy, absent by default, and a result produced
under it says so.

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

### Results

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
`MatchingResult` keeps the shape of § Matching; its `candidates` are
`SoulMatch`es, and `best` and `delta_vs_current` are the optimizer's.

### Explainability

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

### Provenance

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

## Matching — `match`

`match` takes the whole inventory and every need, and produces candidate
assignments. Its search and complexity bound are not decided (§ Not decided
here); what it returns is:

```text
MatchingResult {
  per_need: [
    { need_id
    , candidates: [ SoulMatch ]                      // admissible, ranked by § Hard and soft
    , current:    Option<EquippedLoadout>            // what the Shikigami wears now
    , best:       Option<Loadout>                    // the best admissible assignment under the constraint
    , delta_vs_current: Option<SwapDelta or its loadout-wide form>
    }
  ]
  inversions: [ { soul_id, quality, need_id, affinity } ]   // § Pruning and inversions
}
```

**The assignment constraint is real.** A soul is equipped once. `best` is the
best admissible assignment of distinct souls to the six slots, not the six
highest individual affinities — a soul cannot be its own substitute, and the
distinction is exactly where a naive implementation produces a loadout the user
cannot build.

**`inversions` is a first-class output, not a report.** It answers "which of my
bad souls are actually good". It is computed from `fit` over every soul, never
stored, and never taken from pruned candidates.

**`delta_vs_current` says what would change, not what is good.** It is `swap`,
and its loadout-wide form, against the Shikigami's current loadout. This is what
makes `Loadout` (a domain value) distinct from `Inventory` (a fact).

## Worked example

Two cases from PRP-0004's vectors, whose needs are test fixtures:

- **An inversion** (P05). A slot-5 soul with Effect RES 16.0, DEF% 5.4, HP 228
  and ATK 27 has quality 34.13, tier N, under `yata-quality-v2`: no archetype
  values Effect RES. For a need valuing Effect RES at 1 and HP% and DEF% at 1/2
  it scores 81.52, and it is in `inversions`.
- **A better soul, a worse loadout** (P08). For an output need whose panel Crit
  saturates at 100, a slot-3 soul of affinity 91.96 replacing one of 83.92
  lowers the loadout's value from 86.500 to 84.167: the panel was at 98, so 10
  points of the new Crit count for nothing, and the Crit DMG and ATK% it gives
  up do. The result says so: `Saturated { Panel(Crit), beyond: 10.0 }`.

## Provenance and honesty rules

These are presentation constraints with the same weight as the arithmetic,
because a score the user cannot interpret is worse than no score:

- A score is always shown with its `ParamSet` identity and version.
- An `authored` need profile is labeled as authored. A recommendation resting on
  a judgment call must not look like one resting on the game's arithmetic.
- Inadmissible results are never displayed as recommendations, only as
  explanations, with the hard checks that failed.
- No score is ever displayed for a soul that failed to decode. A decode failure
  is reported as a decode failure; inventing a low score for unreadable data is
  how the legacy application produced confidently wrong discard advice.

## Not decided here

- the minimum evidence for changing the quality model's parameters
- how `derived` need profiles are computed from the game's damage model
- how `authored` profiles are created, reviewed, and versioned
- the matching algorithm and its complexity bound
- the profile file format, and the review of authored profiles
- the rules marked (U): set activation, and the panel value of the five
  `PanelStat`s
- an order between two Shikigami, which constrains two loadouts at once
- whether a soft target should weigh more than a tie-breaker

Each is a future entry on this page or a decision record, not an architectural
gap.

## Related

- The premise:
  [ADR-0001](../decisions/0001-pure-core-effects-at-the-boundary.md)
- Two passes, and why one cannot work:
  [ADR-0003](../decisions/0003-two-pass-scoring.md)
- The pass-2 contract:
  [ADR-0028](../decisions/0028-pass-2-affinity-contract.md), from PRP-0004
- Where these crates sit:
  [../architecture/overview.md](../architecture/overview.md)
- Slot and attribute facts: [glossary.md](glossary.md)
