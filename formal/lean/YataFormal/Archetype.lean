import Mathlib
import YataFormal.Soul

/-!
# Archetypes: four useful attributes, equal weight

The quality profiles of `docs/spec/quality-model.md` each value four attributes with weight 1.
For them the attainable maximum is nine roll units, reached by the paper's theoretical output
soul, and the two structural facts the tiers rest on hold: a soul whose useful value lies on one
line reaches at most six, and a soul above eight has put every increment on a useful line.
-/

namespace Yata

/-- A profile valuing four attributes, each with weight 1. -/
def archetype (A : Finset Attr) : Profile where
  useful := A
  weight := fun a => if a ∈ A then 1 else 0
  weight_pos := fun a ha => by simp [ha]
  weight_zero := fun a ha => by simp [ha]

namespace Archetype

variable (A : Finset Attr)

theorem utility_eq (x : Features) : (archetype A).utility x = ∑ a ∈ A, x a := by
  unfold Profile.utility archetype
  simp only [ite_mul, one_mul, zero_mul]
  rw [Finset.sum_ite_mem, Finset.univ_inter]

/-- Useful value never exceeds the increments that landed on useful lines. -/
theorem utility_le_useful_hits (s : Soul) :
    (archetype A).utility s.features ≤ ∑ a ∈ A, (s.hits a : ℚ) := by
  rw [utility_eq]
  exact Finset.sum_le_sum (fun a _ => s.features_le_hits a)

theorem useful_hits_le_nine (s : Soul) : ∑ a ∈ A, s.hits a ≤ 9 := by
  have h := s.total_le
  calc ∑ a ∈ A, s.hits a ≤ ∑ a, s.hits a :=
        Finset.sum_le_sum_of_subset_of_nonneg (Finset.subset_univ A) (fun _ _ _ => Nat.zero_le _)
    _ ≤ 9 := h

/-- The attainable maximum: no soul exceeds nine roll units on an archetype. -/
theorem utility_le_nine (s : Soul) : (archetype A).utility s.features ≤ 9 := by
  have h1 := utility_le_useful_hits A s
  have h2 : (∑ a ∈ A, (s.hits a : ℚ)) ≤ 9 := by exact_mod_cast useful_hits_le_nine A s
  linarith

/-- Above eight roll units, all nine increments are on useful lines: the soul is on the
frontier's structure, and only its roll values separate it from the maximum. -/
theorem nine_useful_of_gt_eight (s : Soul) (h : 8 < (archetype A).utility s.features) :
    ∑ a ∈ A, s.hits a = 9 := by
  have h1 := utility_le_useful_hits A s
  have h2 := useful_hits_le_nine A s
  have : (8 : ℚ) < ∑ a ∈ A, (s.hits a : ℚ) := lt_of_lt_of_le h h1
  have : 8 < ∑ a ∈ A, s.hits a := by exact_mod_cast this
  omega

/-- A soul whose useful value lies on one line reaches at most six roll units: one perfect line
alone. This is what the SP quality floor must exceed. -/
theorem single_line_le_six (s : Soul) (b : Attr)
    (h : ∀ a ∈ A, a ≠ b → s.features a = 0) : (archetype A).utility s.features ≤ 6 := by
  rw [utility_eq]
  by_cases hb : b ∈ A
  · rw [← Finset.add_sum_erase A _ hb]
    have : ∑ a ∈ A.erase b, s.features a = 0 :=
      Finset.sum_eq_zero (fun a ha => h a (Finset.mem_of_mem_erase ha) (Finset.ne_of_mem_erase ha))
    rw [this, add_zero]
    have h1 := s.features_le_hits b
    have h2 : (s.hits b : ℚ) ≤ 6 := by exact_mod_cast s.hits_le b
    linarith
  · have : ∑ a ∈ A, s.features a = 0 :=
      Finset.sum_eq_zero (fun a ha => h a ha (fun e => hb (e ▸ ha)))
    rw [this]; norm_num

end Archetype

/-! ## The paper's theoretical output soul

Hu (2026), § 4.4: all four initial sub-attributes useful and at their maximum, and all five
rolls on 暴击 at their maximum: 18% 暴击, 4% 暴击伤害, 3% 攻击加成, 3 速度. -/

def maxRoll : Roll := ⟨1, by norm_num, le_refl 1⟩

def paperSoul : Soul where
  incs := fun a => match a with
    | .crit => List.replicate 6 maxRoll
    | .critDmg | .atkPct | .spd => [maxRoll]
    | _ => []
  lines_le := by decide
  total_le := by decide
  hits_le := by intro a; cases a <;> decide

def outputAttrs : Finset Attr := {.atkPct, .crit, .critDmg, .spd}

theorem paperSoul_utility : (archetype outputAttrs).utility paperSoul.features = 9 := by
  rw [Archetype.utility_eq]
  simp [outputAttrs, paperSoul, Soul.features, maxRoll]
  norm_num

/-- The paper's soul scores exactly 100 under any anchors whose maximum is nine. -/
theorem paperSoul_score (A : Anchors) (hM : A.M = 9) :
    (archetype outputAttrs).score A paperSoul.features = 100 := by
  unfold Profile.score
  rw [paperSoul_utility, ← hM]
  exact A.g_M

end Yata
