import Mathlib
import YataFormal.Score

/-!
# Souls, profiles, and utility

The part of the game model the theorems need, and nothing more (`docs/spec/quality-model.md`).
A six-star soul at +15 is described by the increments each sub-attribute received, each measured
in roll units: an increment of attribute `a` is `δ / hi(a)`, which the game's table places in
`[4/5, 1]` for every six-star attribute (`docs/spec/soul-mechanics.md`, "Values"). A soul has at
most four sub-attributes, at most nine increments in all (four initial and five rolls), and at
most six on one attribute. How likely each soul is, is not modelled here: the theorems hold for
every soul the rules allow.
-/

namespace Yata

/-- The eleven sub-attributes. -/
inductive Attr
  | atkFlat | atkPct | defFlat | defPct | hpFlat | hpPct | spd | effHit | effRes | crit | critDmg
  deriving DecidableEq, Repr

instance : Fintype Attr where
  elems := {.atkFlat, .atkPct, .defFlat, .defPct, .hpFlat, .hpPct, .spd, .effHit, .effRes, .crit,
    .critDmg}
  complete := by intro x; cases x <;> simp

/-- One increment, in roll units: `δ / hi(a) ∈ [4/5, 1]`. -/
structure Roll where
  val : ℚ
  lo : 4 / 5 ≤ val
  hi : val ≤ 1

/-- A six-star soul's sub-attributes, as the increments each received. -/
structure Soul where
  incs : Attr → List Roll
  lines_le : (Finset.univ.filter (fun a => incs a ≠ [])).card ≤ 4
  total_le : ∑ a, (incs a).length ≤ 9
  hits_le : ∀ a, (incs a).length ≤ 6

/-- `e_a`: an attribute's value in roll units; 0 when the soul lacks it. -/
abbrev Features := Attr → ℚ

namespace Soul

def hits (s : Soul) (a : Attr) : ℕ := (s.incs a).length

def features (s : Soul) : Features := fun a => ((s.incs a).map Roll.val).sum

lemma list_sum_le_length (l : List Roll) : (l.map Roll.val).sum ≤ l.length := by
  induction l with
  | nil => simp
  | cons r t ih => simp only [List.map_cons, List.sum_cons, List.length_cons, Nat.cast_succ]; linarith [r.hi]

lemma list_sum_ge (l : List Roll) : (4 / 5 : ℚ) * l.length ≤ (l.map Roll.val).sum := by
  induction l with
  | nil => simp
  | cons r t ih => simp only [List.map_cons, List.sum_cons, List.length_cons, Nat.cast_succ]; linarith [r.lo]

/-- A line's value never exceeds its increment count: every increment is at most one roll unit. -/
theorem features_le_hits (s : Soul) (a : Attr) : s.features a ≤ s.hits a :=
  list_sum_le_length _

theorem features_nonneg (s : Soul) (a : Attr) : 0 ≤ s.features a := by
  have := list_sum_ge (s.incs a)
  have : (0 : ℚ) ≤ (4 / 5) * (s.incs a).length := by positivity
  unfold features; linarith

/-- A value above five roll units needs six increments: all five rolls went to this attribute.
This is I-Hits of `soul-mechanics.md`, and it makes specialization decidable from values. -/
theorem six_hits_of_gt_five (s : Soul) (a : Attr) (h : 5 < s.features a) : s.hits a = 6 := by
  have h1 := s.features_le_hits a
  have h2 := s.hits_le a
  have : (5 : ℚ) < s.hits a := lt_of_lt_of_le h h1
  have : 5 < s.hits a := by exact_mod_cast this
  unfold hits at *; omega

end Soul

/-- An evaluation profile: which attributes are useful, and how much. -/
structure Profile where
  useful : Finset Attr
  weight : Attr → ℚ
  weight_pos : ∀ a ∈ useful, 0 < weight a
  weight_zero : ∀ a ∉ useful, weight a = 0

namespace Profile

variable (p : Profile)

lemma weight_nonneg (a : Attr) : 0 ≤ p.weight a := by
  by_cases h : a ∈ p.useful
  · exact le_of_lt (p.weight_pos a h)
  · rw [p.weight_zero a h]

/-- `U_p(x) = Σ_a w_p(a) · x_a`. -/
def utility (x : Features) : ℚ := ∑ a, p.weight a * x a

/-- The score of a feature vector under a profile and its anchors. -/
def score (A : Anchors) (x : Features) : ℚ := A.g (p.utility x)

/-- `y` dominates `x` under `p`: no worse on any useful attribute, better on one. -/
def Dominates (y x : Features) : Prop :=
  (∀ a ∈ p.useful, x a ≤ y a) ∧ ∃ a ∈ p.useful, x a < y a

/-- No worse on any useful attribute. -/
def WeaklyDominates (y x : Features) : Prop := ∀ a ∈ p.useful, x a ≤ y a

theorem utility_mono {x y : Features} (h : p.WeaklyDominates y x) : p.utility x ≤ p.utility y := by
  unfold utility
  apply Finset.sum_le_sum
  intro a _
  by_cases ha : a ∈ p.useful
  · exact mul_le_mul_of_nonneg_left (h a ha) (p.weight_nonneg a)
  · simp [p.weight_zero a ha]

theorem utility_strict {x y : Features} (h : p.Dominates y x) : p.utility x < p.utility y := by
  obtain ⟨hle, a, ha, hlt⟩ := h
  unfold utility
  apply Finset.sum_lt_sum
  · intro b _
    by_cases hb : b ∈ p.useful
    · exact mul_le_mul_of_nonneg_left (hle b hb) (p.weight_nonneg b)
    · simp [p.weight_zero b hb]
  · exact ⟨a, Finset.mem_univ a, mul_lt_mul_of_pos_left hlt (p.weight_pos a ha)⟩

/-- Improving useful attributes never lowers the score. -/
theorem score_mono (A : Anchors) {x y : Features} (h : p.WeaklyDominates y x) :
    p.score A x ≤ p.score A y :=
  A.g_mono (p.utility_mono h)

/-- A dominating soul scores strictly higher, while it is within the attainable maximum. -/
theorem score_strict (A : Anchors) {x y : Features} (h : p.Dominates y x)
    (hx : 0 ≤ p.utility x) (hy : p.utility y ≤ A.M) : p.score A x < p.score A y :=
  A.g_strict hx (p.utility_strict h) hy

/-- The score sees a soul only through its utility: two souls of equal utility score equally,
however probable or improbable either one is. There is no term for how rare a soul is. -/
theorem score_factors (A : Anchors) {x y : Features} (h : p.utility x = p.utility y) :
    p.score A x = p.score A y := by
  unfold score; rw [h]

/-- Utility of a soul's features is nonnegative. -/
theorem utility_nonneg (s : Soul) : 0 ≤ p.utility s.features := by
  unfold utility
  apply Finset.sum_nonneg
  intro a _
  exact mul_nonneg (p.weight_nonneg a) (s.features_nonneg a)

end Profile

end Yata
