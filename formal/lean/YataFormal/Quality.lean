import Mathlib
import YataFormal.Archetype
import YataFormal.Fairness
import YataFormal.Tier

/-!
# The quality standard: archetypes, main-attribute fit, eligibility, and tiers on souls

Ties the pieces together for the v2 catalogue (`docs/spec/quality-model.md`): four archetypes of
two sizes, which main attributes each accepts, the soul-level eligibility predicate that gates
`speed`, the four-attribute ladder with its UR and SP conditions stated on souls, and `speed`'s
own tier rule. Cross-archetype comparability comes from each archetype's own anchors, not from
every archetype holding the same number of useful attributes.
-/

namespace Yata

inductive Slot
  | s1 | s2 | s3 | s4 | s5 | s6
  deriving DecidableEq, Repr

instance : Fintype Slot where
  elems := {.s1, .s2, .s3, .s4, .s5, .s6}
  complete := by intro x; cases x <;> simp

/-- The main attributes the game allows on each slot (`glossary.md`, `Main(k)`), with the flat
mains of slots 1, 3 and 5. -/
def legalMain : Slot → Finset Attr
  | .s1 => {.atkFlat}
  | .s2 => {.spd, .atkPct, .defPct, .hpPct}
  | .s3 => {.defFlat}
  | .s4 => {.effHit, .effRes, .atkPct, .defPct, .hpPct}
  | .s5 => {.hpFlat}
  | .s6 => {.crit, .critDmg, .atkPct, .defPct, .hpPct}

/-- The v2 archetypes: three of four useful attributes, and `speed` of one. -/
inductive Arch
  | output | hit | healing | speed
  deriving DecidableEq, Repr

instance : Fintype Arch where
  elems := {.output, .hit, .healing, .speed}
  complete := by intro x; cases x <;> simp

def Arch.attrs : Arch → Finset Attr
  | .output => {.atkPct, .crit, .critDmg, .spd}
  | .hit => {.effHit, .spd, .hpPct, .defPct}
  | .healing => {.hpPct, .crit, .critDmg, .spd}
  | .speed => {.spd}

/-- The main attributes an archetype can use, per slot. -/
def Arch.accepts : Arch → Slot → Finset Attr
  | _, .s1 => {.atkFlat}
  | _, .s3 => {.defFlat}
  | _, .s5 => {.hpFlat}
  | .output, .s2 => {.spd, .atkPct}
  | .output, .s4 => {.atkPct}
  | .output, .s6 => {.crit, .critDmg, .atkPct}
  | .hit, .s2 => {.spd, .hpPct, .defPct}
  | .hit, .s4 => {.effHit, .hpPct, .defPct}
  | .hit, .s6 => {.hpPct, .defPct}
  | .healing, .s2 => {.spd, .hpPct}
  | .healing, .s4 => {.hpPct}
  | .healing, .s6 => {.crit, .critDmg, .hpPct}
  | .speed, .s2 => {.spd, .hpPct, .defPct}
  | .speed, .s4 => {.effHit, .effRes, .hpPct, .defPct}
  | .speed, .s6 => {.hpPct, .defPct}

/-- The four-attribute archetypes value four attributes; `speed` values one. -/
theorem Arch.card_broad (p : Arch) (hp : p ≠ .speed) : p.attrs.card = 4 := by
  cases p <;> first | decide | exact absurd rfl hp

theorem Arch.card_speed : Arch.speed.attrs.card = 1 := by decide

/-- Every archetype accepts only legal main attributes. -/
theorem Arch.accepts_legal (p : Arch) (k : Slot) : p.accepts k ⊆ legalMain k := by
  cases p <;> cases k <;> decide

/-- **Coverage of main attributes.** Every legal main attribute on every slot is accepted by some
archetype. Whether an accepting archetype is also eligible is a property of the soul; see
`candidate_exists_iff`. -/
theorem Arch.coverage (k : Slot) (m : Attr) (h : m ∈ legalMain k) : ∃ p : Arch, m ∈ p.accepts k := by
  cases k <;> cases m <;> simp [legalMain] at h <;> decide

/-! ## The milestones, in roll units

The calibration program reads these definitions and fails if its own values differ. -/

/-- SR: five useful increments at mean value, `5μ`. -/
def uSR : ℚ := 9 / 2
/-- SSR: the most one useful line can hold, six increments at maximum. -/
def uSSR : ℚ := 6
/-- SP's quality floor: seven useful increments at mean value, `7μ`. -/
def uSPFloor : ℚ := 63 / 10
/-- UR: one roll unit below the attainable maximum of a four-attribute archetype. -/
def uUR : ℚ := 8
/-- `speed`'s eligibility gate: four Speed roll units. -/
def speedGate : ℚ := 4
/-- `speed`'s UR boundary: one roll unit below its own maximum. -/
def uSpeedUR : ℚ := 5
/-- `speed`'s attainable maximum: one line, six increments at maximum. -/
def speedMax : ℚ := 6

theorem milestones_ordered : uSR < uSSR ∧ uSSR < uSPFloor ∧ uSPFloor < uUR := by
  norm_num [uSR, uSSR, uSPFloor, uUR]

theorem speed_milestones : speedGate < uSpeedUR ∧ uSpeedUR = speedMax - 1 := by
  norm_num [speedGate, uSpeedUR, speedMax]

/-! ## Eligibility and candidates -/

/-- The soul-level eligibility predicate: `speed` needs at least four Speed roll units; the other
archetypes are always eligible. It reads the Speed line's value, never how many increments it
took. -/
def Arch.gate : Arch → Soul → Prop
  | .speed, s => speedGate ≤ s.features .spd
  | _, _ => True

/-- An archetype is a candidate for a soul in slot `k` with main `m` when it accepts the main and
its eligibility predicate holds. The quality of the soul is the best result over its candidates. -/
def Candidate (k : Slot) (m : Attr) (s : Soul) (p : Arch) : Prop := m ∈ p.accepts k ∧ p.gate s

theorem gate_broad (p : Arch) (hp : p ≠ .speed) (s : Soul) : p.gate s := by
  cases p <;> trivial

/-- **Below the gate, `speed` does not take part in quality selection** — it is not scored low;
it is not a candidate. -/
theorem speed_not_candidate (k : Slot) (m : Attr) (s : Soul) (h : s.features .spd < speedGate) :
    ¬ Candidate k m s .speed := by
  intro ⟨_, hg⟩
  exact absurd hg (not_le.mpr h)

theorem speed_candidate_iff (k : Slot) (m : Attr) (s : Soul) :
    Candidate k m s .speed ↔ m ∈ Arch.speed.accepts k ∧ 4 ≤ s.features .spd := by
  simp [Candidate, Arch.gate, speedGate]

/-- Raising the Speed line never removes `speed` from the candidates. -/
theorem speed_gate_mono (s t : Soul) (h : s.features .spd ≤ t.features .spd)
    (hs : Arch.speed.gate s) : Arch.speed.gate t :=
  le_trans hs h

theorem accepts_broad_or_effRes (k : Slot) (m : Attr) (h : m ∈ legalMain k)
    (hne : ¬ (k = .s4 ∧ m = .effRes)) : ∃ p : Arch, p ≠ .speed ∧ m ∈ p.accepts k := by
  cases k <;> cases m <;> simp [legalMain] at h hne <;> decide

theorem effRes_only_speed (p : Arch) (h : Attr.effRes ∈ p.accepts .s4) : p = .speed := by
  cases p <;> simp_all [Arch.accepts]

/-- **Which souls have no candidate.** A legal soul has a candidate archetype unless its main is
`EffectRes` on slot 4 and its Speed line is below the gate. No other main attribute is left
without an archetype; such a soul is unrated in pass 1, and its value is pass 2's question. -/
theorem candidate_exists_iff (k : Slot) (m : Attr) (h : m ∈ legalMain k) (s : Soul) :
    (∃ p, Candidate k m s p) ↔ ¬ (k = .s4 ∧ m = .effRes ∧ s.features .spd < speedGate) := by
  constructor
  · rintro ⟨p, hm, hg⟩ ⟨rfl, rfl, hlt⟩
    have := effRes_only_speed p hm
    subst this
    exact absurd hg (not_le.mpr hlt)
  · intro hn
    by_cases he : k = .s4 ∧ m = .effRes
    · obtain ⟨rfl, rfl⟩ := he
      have hge : speedGate ≤ s.features .spd := by
        by_contra hc; exact hn ⟨rfl, rfl, not_le.mp hc⟩
      exact ⟨.speed, by decide, hge⟩
    · obtain ⟨p, hp, hm⟩ := accepts_broad_or_effRes k m h he
      exact ⟨p, hm, gate_broad p hp s⟩

/-! ## The four-attribute ladder on souls

Each archetype has its own anchors and thresholds: the expected utility `E` is computed under
the official class weights. The ladder below is that of `output`, `hit` and `healing`, whose
attainable maximum is nine; `speed` is tiered by `speedTier`, and the exposed tier `tierOn`
chooses between them. -/

variable (A : Arch → Anchors) (T : Arch → Thresholds)

/-- `specialized`: some useful line holds more than five roll units, which forces all six
increments onto it (`Soul.six_hits_of_gt_five`). -/
def specialized (p : Arch) (s : Soul) : Prop := ∃ a ∈ p.attrs, 5 < s.features a

/-- The four-attribute ladder under an archetype. -/
noncomputable def tierOf (p : Arch) (s : Soul) : Tier := by
  classical
  exact (T p).tier ((archetype p.attrs).score (A p) s.features) (decide (specialized p s))

/-- **UR is the near-frontier region.** With `tUR = g(M − 1)` and `M = 9`, a soul is UR under an
archetype exactly when its utility exceeds eight roll units. -/
theorem UR_iff_gt_eight (p : Arch) (s : Soul) (hM : (A p).M = 9)
    (hT : (T p).tUR = (A p).g 8) :
    tierOf A T p s = .UR ↔ 8 < (archetype p.attrs).utility s.features := by
  unfold tierOf
  rw [(T p).tier_UR_iff, hT]
  unfold Profile.score
  have hu0 := (archetype p.attrs).utility_nonneg s
  have hu9 := Archetype.utility_le_nine p.attrs s
  constructor
  · intro h
    by_contra hc
    have := (A p).g_mono (not_lt.mp hc)
    linarith
  · intro h
    exact (A p).g_strict (by norm_num) h (by rw [hM]; exact hu9)

/-- A UR soul has put all nine increments on useful lines. -/
theorem UR_structure (p : Arch) (s : Soul) (hM : (A p).M = 9)
    (hT : (T p).tUR = (A p).g 8) (h : tierOf A T p s = .UR) : ∑ a ∈ p.attrs, s.hits a = 9 :=
  Archetype.nine_useful_of_gt_eight p.attrs s ((UR_iff_gt_eight A T p s hM hT).mp h)

/-- **One perfect line and nothing useful beside it is never SP**, provided the SP floor lies
above the score of six roll units. -/
theorem single_line_not_SP (p : Arch) (s : Soul) (b : Attr)
    (hb : ∀ a ∈ p.attrs, a ≠ b → s.features a = 0) (hfloor : (A p).g 6 < (T p).cSP) :
    tierOf A T p s ≠ .SP := by
  intro h
  unfold tierOf at h
  have := ((T p).sp_needs_floor _ _ h).1
  unfold Profile.score at this
  have h6 := (A p).g_mono (Archetype.single_line_le_six p.attrs s b hb)
  linarith

/-- **A perfect line is SSR quality.** A soul with six roll units on one useful line ranks at
least SSR, when the SSR floor is the score of six roll units. -/
theorem perfect_line_at_least_SSR (p : Arch) (s : Soul) {b : Attr} (hb : b ∈ p.attrs)
    (h6 : 6 ≤ s.features b) (hSSR : (T p).cSSR = (A p).g 6) : 3 ≤ (tierOf A T p s).rank := by
  unfold tierOf
  apply (T p).ssr_floor_rank
  rw [hSSR]
  exact (A p).g_mono (le_trans h6 (Archetype.line_le_utility p.attrs s hb))

/-- **One perfect line alone is exactly SSR:** at least SSR by the line, not SP by the floor, not
UR by the maximum. -/
theorem perfect_single_line_is_SSR (p : Arch) (s : Soul) {b : Attr} (hb : b ∈ p.attrs)
    (h6 : 6 ≤ s.features b) (halone : ∀ a ∈ p.attrs, a ≠ b → s.features a = 0)
    (hM : (A p).M = 9) (hSSR : (T p).cSSR = (A p).g 6) (hSP : (A p).g 6 < (T p).cSP)
    (hUR : (T p).tUR = (A p).g 8) : tierOf A T p s = .SSR := by
  have hge := perfect_line_at_least_SSR A T p s hb h6 hSSR
  have hnsp := single_line_not_SP A T p s b halone hSP
  have hnur : tierOf A T p s ≠ .UR := by
    rw [ne_eq, UR_iff_gt_eight A T p s hM hUR]
    have := Archetype.single_line_le_six p.attrs s b halone
    linarith
  revert hge hnsp hnur
  cases tierOf A T p s <;> simp [Tier.rank]

/-- The paper's theoretical output soul is UR under the output archetype. -/
theorem paperSoul_UR (hM : (A .output).M = 9) (hT : (T .output).tUR = (A .output).g 8) :
    tierOf A T .output paperSoul = .UR := by
  rw [UR_iff_gt_eight A T _ _ hM hT]
  have : Arch.attrs .output = outputAttrs := rfl
  rw [this, paperSoul_utility]; norm_num

/-- **Dominance never lowers the tier** under an archetype. -/
theorem tier_mono_dominance (p : Arch) (s t : Soul)
    (h : ∀ a ∈ p.attrs, s.features a ≤ t.features a) :
    (tierOf A T p s).rank ≤ (tierOf A T p t).rank := by
  unfold tierOf
  apply (T p).tier_mono
  · exact (archetype p.attrs).score_mono (A p) h
  · intro hs
    simp only [decide_eq_true_eq] at hs ⊢
    obtain ⟨a, ha, h5⟩ := hs
    exact ⟨a, ha, lt_of_lt_of_le h5 (h a ha)⟩

/-! ## `speed`: one attribute, its own maximum, its own tier rule -/

/-- `speed`'s utility is the Speed line's value in roll units, nothing else. -/
theorem speed_utility (x : Features) : (archetype Arch.speed.attrs).utility x = x .spd := by
  rw [Archetype.utility_eq]; simp [Arch.attrs]

/-- **`0 ≤ U_speed ≤ 6`:** one line holds at most six increments of at most one roll unit. -/
theorem speed_utility_bounds (s : Soul) :
    0 ≤ (archetype Arch.speed.attrs).utility s.features ∧
      (archetype Arch.speed.attrs).utility s.features ≤ speedMax := by
  rw [speed_utility]
  refine ⟨s.features_nonneg _, le_trans (s.features_le_hits _) ?_⟩
  have : (s.hits .spd : ℚ) ≤ 6 := by exact_mod_cast s.hits_le .spd
  simpa [speedMax] using this

/-- Raising Speed, with nothing else changed or not, never lowers the speed score. -/
theorem speed_score_mono (B : Anchors) {x y : Features} (h : x .spd ≤ y .spd) :
    (archetype Arch.speed.attrs).score B x ≤ (archetype Arch.speed.attrs).score B y :=
  (archetype Arch.speed.attrs).score_mono B (fun a ha => by
    have : a = .spd := by simpa [archetype, Arch.attrs] using ha
    subst this; exact h)

/-- The speed score is strictly increasing over the Speed values a soul can have. -/
theorem speed_score_strict (B : Anchors) (hM : B.M = speedMax) {x y : Features}
    (hx : 0 ≤ x .spd) (h : x .spd < y .spd) (hy : y .spd ≤ speedMax) :
    (archetype Arch.speed.attrs).score B x < (archetype Arch.speed.attrs).score B y := by
  unfold Profile.score
  rw [speed_utility, speed_utility]
  exact B.g_strict hx h (by rw [hM]; exact hy)

/-- The theoretical maximum Speed line: six increments at maximum. -/
def maxSpeedSoul : Soul where
  incs := fun a => match a with
    | .spd => List.replicate 6 maxRoll
    | _ => []
  lines_le := by decide
  total_le := by decide
  hits_le := by intro a; cases a <;> decide

theorem maxSpeedSoul_speed : maxSpeedSoul.features .spd = 6 := by
  simp [maxSpeedSoul, Soul.features, maxRoll]; norm_num

/-- **The maximum Speed line maps to speed's upper anchor, 100.** -/
theorem maxSpeedSoul_score (B : Anchors) (hM : B.M = speedMax) :
    (archetype Arch.speed.attrs).score B maxSpeedSoul.features = 100 := by
  unfold Profile.score
  rw [speed_utility, maxSpeedSoul_speed, show (6 : ℚ) = B.M by rw [hM, speedMax]]
  exact B.g_M

/-- `speed`'s tier on an eligible soul: UR above `M_speed − 1`, SSR otherwise. -/
def speedTier (u : ℚ) : Tier := if uSpeedUR < u then .UR else .SSR

/-- **Deterministic and exclusive:** exactly one of UR and SSR, fixed by the utility. -/
theorem speedTier_cases (u : ℚ) :
    (speedTier u = .UR ∧ uSpeedUR < u) ∨ (speedTier u = .SSR ∧ u ≤ uSpeedUR) := by
  unfold speedTier
  split_ifs with h
  · exact Or.inl ⟨rfl, h⟩
  · exact Or.inr ⟨rfl, not_lt.mp h⟩

/-- `speed` never emits N, R, SR, or SP. -/
theorem speedTier_high (u : ℚ) : speedTier u = .SSR ∨ speedTier u = .UR := by
  rcases speedTier_cases u with ⟨h, _⟩ | ⟨h, _⟩
  · exact Or.inr h
  · exact Or.inl h

theorem speedTier_ne_SP (u : ℚ) : speedTier u ≠ .SP := by
  rcases speedTier_high u with h | h <;> rw [h] <;> decide

theorem speedTier_mono {u v : ℚ} (h : u ≤ v) : (speedTier u).rank ≤ (speedTier v).rank := by
  unfold speedTier
  split_ifs with h1 h2 h2 <;> simp [Tier.rank]
  linarith

/-- **The generic specialization rule is not a separate condition for `speed`:** its only useful
line is above five roll units exactly when the soul is in `speed`'s UR region. An SP rule built
on it would never fire below UR, so `speed` has no SP. -/
theorem speed_specialized_iff_UR (s : Soul) :
    specialized .speed s ↔
      speedTier ((archetype Arch.speed.attrs).utility s.features) = .UR := by
  rw [speed_utility]
  by_cases h : 5 < s.features .spd <;> simp [specialized, Arch.attrs, speedTier, uSpeedUR, h]

/-- A `speed` UR soul put all six increments on Speed. -/
theorem speed_UR_six_hits (s : Soul)
    (h : speedTier ((archetype Arch.speed.attrs).utility s.features) = .UR) :
    s.hits .spd = 6 := by
  rw [speed_utility] at h
  unfold speedTier at h
  split_ifs at h with hu
  exact s.six_hits_of_gt_five .spd (by simpa [uSpeedUR] using hu)

/-- The tier the product exposes under an archetype: the four-attribute ladder, or `speed`'s
rule. `speed`'s tier is meaningful only when it is a candidate, at or above the gate. -/
noncomputable def tierOn (p : Arch) (s : Soul) : Tier :=
  match p with
  | .speed => speedTier ((archetype Arch.speed.attrs).utility s.features)
  | .output => tierOf A T .output s
  | .hit => tierOf A T .hit s
  | .healing => tierOf A T .healing s

theorem tierOn_broad (p : Arch) (hp : p ≠ .speed) (s : Soul) : tierOn A T p s = tierOf A T p s := by
  cases p <;> first | rfl | exact absurd rfl hp

/-- An eligible `speed` soul is at least SSR. -/
theorem speed_candidate_rank (k : Slot) (m : Attr) (s : Soul) (_h : Candidate k m s .speed) :
    3 ≤ (tierOn A T .speed s).rank := by
  show 3 ≤ (speedTier ((archetype Arch.speed.attrs).utility s.features)).rank
  rcases speedTier_high ((archetype Arch.speed.attrs).utility s.features) with e | e <;>
    rw [e] <;> decide

/-- **Dominance never lowers the exposed tier**, for every archetype. -/
theorem tierOn_mono_dominance (p : Arch) (s t : Soul)
    (h : ∀ a ∈ p.attrs, s.features a ≤ t.features a) :
    (tierOn A T p s).rank ≤ (tierOn A T p t).rank := by
  cases p
  case speed =>
    unfold tierOn
    apply speedTier_mono
    rw [speed_utility, speed_utility]
    exact h .spd (by simp [Arch.attrs])
  all_goals exact tier_mono_dominance A T _ s t h

/-! ## `output` and `healing` are one archetype under a relabelling -/

/-- The profile of a relabelled archetype is the archetype of the relabelled set. -/
theorem archetype_relabel (B : Finset Attr) (π : Attr ≃ Attr) :
    (archetype B).relabel π = archetype (B.map π.toEmbedding) := by
  simp only [Profile.relabel, archetype, Profile.mk.injEq, true_and]
  funext a
  simp [Finset.mem_map_equiv]

/-- **`healing` is `output` with `AtkPercent` and `HpPercent` exchanged.** Under a reference
measure symmetric under that exchange, the two have the same expected utility, hence the same
anchors and scales (`healing_expected_eq_output`, `score_relabel`). The official class weights
give both attributes 9%; the calibration asserts the equality of the anchors and every tier
rate. -/
theorem healing_is_output_relabelled :
    archetype Arch.healing.attrs =
      (archetype Arch.output.attrs).relabel (Equiv.swap Attr.atkPct Attr.hpPct) := by
  rw [archetype_relabel]
  congr 1

theorem healing_expected_eq_output {Ω : Type} [Fintype Ω] (μ : RefMeasure Ω)
    (hμ : μ.SymmetricUnder (Equiv.swap Attr.atkPct Attr.hpPct)) :
    μ.expected (archetype Arch.healing.attrs) = μ.expected (archetype Arch.output.attrs) := by
  rw [healing_is_output_relabelled]
  exact expected_relabel μ _ _ hμ

end Yata

namespace Yata

variable (A : Arch → Anchors) (T : Arch → Thresholds)

/-- **Dominance never lowers a soul in the exposed order** under an archetype: its tier is no
lower, and at an equal tier its score is no lower. The quality is the best of these pairs over
the candidate archetypes, and the best of pointwise larger pairs is larger. -/
theorem exposed_mono_dominance (p : Arch) (s t : Soul)
    (h : ∀ a ∈ p.attrs, s.features a ≤ t.features a) :
    exposedLE (tierOn A T p s, (archetype p.attrs).score (A p) s.features)
      (tierOn A T p t, (archetype p.attrs).score (A p) t.features) := by
  have hr := tierOn_mono_dominance A T p s t h
  have hs := (archetype p.attrs).score_mono (A p) h
  unfold exposedLE
  rcases Nat.lt_or_eq_of_le hr with hlt | heq
  · exact Or.inl hlt
  · exact Or.inr ⟨heq, hs⟩

end Yata
