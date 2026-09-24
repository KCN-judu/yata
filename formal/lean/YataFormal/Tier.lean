import Mathlib
import YataFormal.Score

/-!
# Tiers

`N < R < SR < SSR < SP < UR` (`docs/spec/quality-model.md`, "Tiers"). N to SSR are bands of the
score. SP is a specialized soul at or above SP's own quality floor, which lies above the SSR
floor. UR is the region above `g(M − 1)`, one roll unit below the attainable maximum. The tier is
a function of the score and of whether the soul is specialized, and the precedence UR, then SP,
then the band is built into that function. The order the product exposes is tier first, then
score.

Since v1.1 the SSR floor and the SP floor are separate thresholds, ordered
`cSR < cSSR < cSP < tUR`; nothing here assumes they are equal.
-/

namespace Yata

inductive Tier
  | N | R | SR | SSR | SP | UR
  deriving DecidableEq, Repr

/-- The ladder, stated as ranks rather than trusted to constructor order. -/
def Tier.rank : Tier → ℕ
  | .N => 0 | .R => 1 | .SR => 2 | .SSR => 3 | .SP => 4 | .UR => 5

theorem Tier.rank_injective : Function.Injective Tier.rank := by
  intro a b h; cases a <;> cases b <;> simp_all [Tier.rank]

theorem Tier.ladder :
    Tier.N.rank < Tier.R.rank ∧ Tier.R.rank < Tier.SR.rank ∧ Tier.SR.rank < Tier.SSR.rank ∧
    Tier.SSR.rank < Tier.SP.rank ∧ Tier.SP.rank < Tier.UR.rank := by decide

/-- Score thresholds: R, SR and SSR start at `cR`, `cSR`, `cSSR`; SP needs at least `cSP`; UR is
above `tUR`. -/
structure Thresholds where
  cR : ℚ
  cSR : ℚ
  cSSR : ℚ
  cSP : ℚ
  tUR : ℚ
  h0 : 0 < cR
  h1 : cR < cSR
  h2 : cSR < cSSR
  h3 : cSSR < cSP
  h4 : cSP < tUR
  h5 : tUR < 100

namespace Thresholds

variable (T : Thresholds)

/-- The ordinary band of a score. -/
def band (x : ℚ) : Tier :=
  if x < T.cR then .N else if x < T.cSR then .R else if x < T.cSSR then .SR else .SSR

/-- The tier of a soul with score `x`, specialized or not. -/
def tier (x : ℚ) (spec : Bool) : Tier :=
  if T.tUR < x then .UR else if spec ∧ T.cSP ≤ x then .SP else T.band x

theorem band_rank_le (x : ℚ) : (T.band x).rank ≤ 3 := by
  unfold band; split_ifs <;> decide

/-- **Bands never produce SP or UR.** -/
theorem band_ne_SP (x : ℚ) : T.band x ≠ .SP := by
  unfold band; split_ifs <;> decide

theorem band_ne_UR (x : ℚ) : T.band x ≠ .UR := by
  unfold band; split_ifs <;> decide

/-- **UR precedence.** Above `tUR` the tier is UR, whatever else holds. -/
theorem tier_UR_iff (x : ℚ) (spec : Bool) : T.tier x spec = .UR ↔ T.tUR < x := by
  unfold tier
  split_ifs with h1 h2
  · simp [h1]
  · simp [h1]
  · simp [h1, band_ne_UR]

/-- **SP precedence and its floor.** SP is exactly: not UR, specialized, and at or above the SP
floor. -/
theorem tier_SP_iff (x : ℚ) (spec : Bool) :
    T.tier x spec = .SP ↔ ¬ T.tUR < x ∧ spec = true ∧ T.cSP ≤ x := by
  unfold tier
  split_ifs with h1 h2
  · simp [h1]
  · simp only [true_iff]; exact ⟨h1, h2.1, h2.2⟩
  · simp only [band_ne_SP, false_iff]
    intro ⟨_, hs, hc⟩; exact h2 ⟨hs, hc⟩

/-- **Specialization cannot bypass the SP floor:** every SP soul is specialized and at or above
`cSP`. -/
theorem sp_needs_floor (x : ℚ) (spec : Bool) (h : T.tier x spec = .SP) :
    T.cSP ≤ x ∧ spec = true := by
  have := (T.tier_SP_iff x spec).mp h; exact ⟨this.2.2, this.2.1⟩

/-- **Every SP soul is of SSR quality:** its score is above the SSR floor. -/
theorem sp_is_ssr_quality (x : ℚ) (spec : Bool) (h : T.tier x spec = .SP) : T.cSSR < x :=
  lt_of_lt_of_le T.h3 (T.sp_needs_floor x spec h).1

/-- **Exclusivity.** Exactly one of three cases holds, and it fixes the tier. -/
theorem tier_cases (x : ℚ) (spec : Bool) :
    (T.tier x spec = .UR ∧ T.tUR < x) ∨
    (T.tier x spec = .SP ∧ ¬ T.tUR < x ∧ spec = true ∧ T.cSP ≤ x) ∨
    (T.tier x spec = T.band x ∧ ¬ T.tUR < x ∧ ¬ (spec = true ∧ T.cSP ≤ x)) := by
  unfold tier
  split_ifs with h1 h2
  · exact Or.inl ⟨rfl, h1⟩
  · exact Or.inr (Or.inl ⟨rfl, h1, h2.1, h2.2⟩)
  · exact Or.inr (Or.inr ⟨rfl, h1, h2⟩)

theorem band_high (x : ℚ) (h : T.cSSR ≤ x) : T.band x = .SSR := by
  have hR := T.h1; have hS := T.h2
  have a : ¬ x < T.cR := by linarith
  have b : ¬ x < T.cSR := by linarith
  have c : ¬ x < T.cSSR := by linarith
  simp only [band, a, b, c, ↓reduceIte]

/-- **No lower band at or above the SSR floor.** There the tier is SSR, SP, or UR. -/
theorem at_ssr_floor_high_tier (x : ℚ) (spec : Bool) (h : T.cSSR ≤ x) :
    T.tier x spec = .SSR ∨ T.tier x spec = .SP ∨ T.tier x spec = .UR := by
  rcases T.tier_cases x spec with ⟨e, _⟩ | ⟨e, _⟩ | ⟨e, _⟩
  · exact Or.inr (Or.inr e)
  · exact Or.inr (Or.inl e)
  · exact Or.inl (e.trans (T.band_high x h))

/-- At or above the SSR floor the rank is at least SSR's. -/
theorem ssr_floor_rank (x : ℚ) (spec : Bool) (h : T.cSSR ≤ x) : 3 ≤ (T.tier x spec).rank := by
  rcases T.at_ssr_floor_high_tier x spec h with e | e | e <;> (rw [e]; decide)

/-- **UR outranks everything in score, not only in tier.** Every non-UR soul scores below every
UR soul. -/
theorem UR_score_gt (x y : ℚ) (sx sy : Bool) (hx : T.tier x sx = .UR) (hy : T.tier y sy ≠ .UR) :
    y < x := by
  have h1 := (T.tier_UR_iff x sx).mp hx
  have h2 : ¬ T.tUR < y := fun h => hy ((T.tier_UR_iff y sy).mpr h)
  linarith [not_lt.mp h2]

/-- **Tier is monotone.** A soul at least as good in score, and specialized whenever the other
is, is at least as high on the ladder. -/
theorem tier_mono (x y : ℚ) (sx sy : Bool) (hxy : x ≤ y) (hs : sx = true → sy = true) :
    (T.tier x sx).rank ≤ (T.tier y sy).rank := by
  have hR := T.h1; have hS := T.h2; have hP := T.h3; have hU := T.h4; have h0 := T.h0
  unfold tier band
  split_ifs <;> simp_all [Tier.rank] <;> linarith

end Thresholds

/-- **Lowering the SSR floor never lowers a tier.** Two threshold sets that agree everywhere but
at SSR, the second with a floor at most the first's, rank every soul at least as high under the
second. -/
theorem lower_ssr_floor_never_lowers_tier (T T' : Thresholds) (hR : T'.cR = T.cR)
    (hSR : T'.cSR = T.cSR) (hSP : T'.cSP = T.cSP) (hUR : T'.tUR = T.tUR)
    (hSSR : T'.cSSR ≤ T.cSSR) (x : ℚ) (spec : Bool) :
    (T.tier x spec).rank ≤ (T'.tier x spec).rank := by
  unfold Thresholds.tier Thresholds.band
  rw [hR, hSR, hSP, hUR]
  split_ifs <;> (simp_all [Tier.rank]; try linarith)

/-- The exposed order: tier first, then score. -/
def exposedLE (a b : Tier × ℚ) : Prop :=
  a.1.rank < b.1.rank ∨ (a.1.rank = b.1.rank ∧ a.2 ≤ b.2)

theorem Tier.rank_le_four {t : Tier} (h : t ≠ .UR) : t.rank ≤ 4 := by
  cases t <;> simp_all [Tier.rank]

/-- A UR soul is above every non-UR soul in the exposed order, strictly. -/
theorem UR_top (T : Thresholds) (x y : ℚ) (sx sy : Bool) (hx : T.tier x sx = .UR)
    (hy : T.tier y sy ≠ .UR) :
    exposedLE (T.tier y sy, y) (T.tier x sx, x) ∧ ¬ exposedLE (T.tier x sx, x) (T.tier y sy, y) := by
  have h4 := Tier.rank_le_four hy
  have h5 : (T.tier x sx).rank = 5 := by rw [hx]; rfl
  unfold exposedLE
  simp only
  omega

end Yata
