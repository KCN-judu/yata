import Mathlib
import YataFormal.Archetype
import YataFormal.Tier

/-!
# The quality standard: archetypes, main-attribute fit, and tiers on souls

Ties the pieces together for the v1 catalogue (`docs/spec/quality-model.md`): the three
archetypes, which main attributes each accepts, the UR and SP conditions stated on souls, and
the facts that make them mean what their names say.
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

/-- The v1 archetypes. -/
inductive Arch
  | output | hit | resist
  deriving DecidableEq, Repr

instance : Fintype Arch where
  elems := {.output, .hit, .resist}
  complete := by intro x; cases x <;> simp

def Arch.attrs : Arch → Finset Attr
  | .output => {.atkPct, .crit, .critDmg, .spd}
  | .hit => {.spd, .effHit, .hpPct, .defPct}
  | .resist => {.spd, .effRes, .hpPct, .defPct}

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
  | .resist, .s2 => {.spd, .hpPct, .defPct}
  | .resist, .s4 => {.effRes, .hpPct, .defPct}
  | .resist, .s6 => {.hpPct, .defPct}

theorem Arch.card_four (p : Arch) : p.attrs.card = 4 := by cases p <;> decide

/-- Every archetype accepts only legal main attributes. -/
theorem Arch.accepts_legal (p : Arch) (k : Slot) : p.accepts k ⊆ legalMain k := by
  cases p <;> cases k <;> decide

/-- **Coverage.** Every legal main attribute on every slot is accepted by some archetype, so
every six-star soul has a quality score. -/
theorem Arch.coverage (k : Slot) (m : Attr) (h : m ∈ legalMain k) : ∃ p : Arch, m ∈ p.accepts k := by
  cases k <;> cases m <;> simp [legalMain] at h <;> decide

/-! ## The v1.1 milestones, in roll units

The calibration program reads these four definitions and fails if its own values differ. -/

/-- SR: five useful increments at mean value, `5μ`. -/
def uSR : ℚ := 9 / 2
/-- SSR: the most one useful line can hold, six increments at maximum. -/
def uSSR : ℚ := 6
/-- SP's quality floor: seven useful increments at mean value, `7μ`. -/
def uSPFloor : ℚ := 63 / 10
/-- UR: one roll unit below the attainable maximum. -/
def uUR : ℚ := 8

theorem milestones_ordered : uSR < uSSR ∧ uSSR < uSPFloor ∧ uSPFloor < uUR := by
  norm_num [uSR, uSSR, uSPFloor, uUR]

/-! ## Tiers on souls

Each archetype has its own anchors and thresholds: the expected utility `E` is computed under
the official class weights, which differ between archetypes. The attainable maximum is nine for
every archetype. -/

variable (A : Arch → Anchors) (T : Arch → Thresholds)

/-- `specialized`: some useful line holds more than five roll units, which forces all six
increments onto it (`Soul.six_hits_of_gt_five`). -/
def specialized (p : Arch) (s : Soul) : Prop := ∃ a ∈ p.attrs, 5 < s.features a

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

end Yata

namespace Yata

variable (A : Arch → Anchors) (T : Arch → Thresholds)

/-- **Dominance never lowers a soul in the exposed order** under an archetype: its tier is no
lower, and at an equal tier its score is no lower. The quality is the best of these pairs over
the accepted archetypes, and the best of pointwise larger pairs is larger. -/
theorem exposed_mono_dominance (p : Arch) (s t : Soul)
    (h : ∀ a ∈ p.attrs, s.features a ≤ t.features a) :
    exposedLE (tierOf A T p s, (archetype p.attrs).score (A p) s.features)
      (tierOf A T p t, (archetype p.attrs).score (A p) t.features) := by
  have hr := tier_mono_dominance A T p s t h
  have hs := (archetype p.attrs).score_mono (A p) h
  unfold exposedLE
  rcases Nat.lt_or_eq_of_le hr with hlt | heq
  · exact Or.inl hlt
  · exact Or.inr ⟨heq, hs⟩

end Yata
