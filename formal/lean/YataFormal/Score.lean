import Mathlib

/-!
# The normalized score

`g A u` maps a utility `u` onto the common 0–100 scale by three anchors (`docs/spec/quality-model.md`,
"Normalization"): no useful value maps to 0, the reference distribution's expected utility `A.E`
to 50, and the attainable maximum `A.M` to 100, linearly in between; below 0 it is 0 and above
`M` it is 100. Everything here is exact rational arithmetic; nothing depends on how the anchors
were computed, only on `0 < E < M`.
-/

namespace Yata

/-- The anchors of one profile: its expected utility under the reference measure, and its
attainable maximum. -/
structure Anchors where
  E : ℚ
  M : ℚ
  E_pos : 0 < E
  E_lt_M : E < M

namespace Anchors

variable (A : Anchors)

lemma M_pos : 0 < A.M := lt_trans A.E_pos A.E_lt_M

lemma gap_pos : 0 < A.M - A.E := sub_pos.mpr A.E_lt_M

/-- The lower segment: 0 at 0, 50 at `E`. -/
def lower (u : ℚ) : ℚ := 50 * u / A.E

/-- The upper segment: 50 at `E`, 100 at `M`. -/
def upper (u : ℚ) : ℚ := 50 + 50 * (u - A.E) / (A.M - A.E)

/-- The normalized score of a utility. -/
def g (u : ℚ) : ℚ := if u ≤ A.E then max 0 (A.lower u) else min 100 (A.upper u)

/-- The same anchors on a scale `c` times larger. -/
def scale (c : ℚ) (hc : 0 < c) : Anchors where
  E := c * A.E
  M := c * A.M
  E_pos := mul_pos hc A.E_pos
  E_lt_M := by nlinarith [A.E_lt_M]

/-! ## The segments -/

lemma lower_mono {u v : ℚ} (h : u ≤ v) : A.lower u ≤ A.lower v := by
  unfold lower
  exact div_le_div_of_nonneg_right (by linarith) (le_of_lt A.E_pos)

lemma lower_strict {u v : ℚ} (h : u < v) : A.lower u < A.lower v := by
  unfold lower
  exact div_lt_div_of_pos_right (by linarith) A.E_pos

lemma upper_mono {u v : ℚ} (h : u ≤ v) : A.upper u ≤ A.upper v := by
  unfold upper
  have := div_le_div_of_nonneg_right (a := 50 * (u - A.E)) (b := 50 * (v - A.E))
    (by linarith) (le_of_lt A.gap_pos)
  linarith

lemma upper_strict {u v : ℚ} (h : u < v) : A.upper u < A.upper v := by
  unfold upper
  have := div_lt_div_of_pos_right (a := 50 * (u - A.E)) (b := 50 * (v - A.E))
    (by linarith) A.gap_pos
  linarith

lemma lower_le_50 {u : ℚ} (h : u ≤ A.E) : A.lower u ≤ 50 := by
  unfold lower
  rw [div_le_iff₀ A.E_pos]
  nlinarith [A.E_pos]

lemma upper_ge_50 {u : ℚ} (h : A.E ≤ u) : 50 ≤ A.upper u := by
  unfold upper
  have : 0 ≤ 50 * (u - A.E) / (A.M - A.E) := div_nonneg (by linarith) (le_of_lt A.gap_pos)
  linarith

lemma upper_le_100 {u : ℚ} (h : u ≤ A.M) : A.upper u ≤ 100 := by
  unfold upper
  have : 50 * (u - A.E) / (A.M - A.E) ≤ 50 := by
    rw [div_le_iff₀ A.gap_pos]; nlinarith [A.gap_pos]
  linarith

lemma lower_nonneg {u : ℚ} (h : 0 ≤ u) : 0 ≤ A.lower u := by
  unfold lower
  exact div_nonneg (by linarith) (le_of_lt A.E_pos)

/-! ## Anchor values -/

theorem g_zero : A.g 0 = 0 := by
  simp [g, le_of_lt A.E_pos, lower]

theorem g_E : A.g A.E = 50 := by
  have h := A.E_pos
  simp only [g, le_refl, ↓reduceIte, lower]
  rw [mul_div_assoc, div_self (ne_of_gt h)]
  norm_num

theorem g_M : A.g A.M = 100 := by
  have h3 := A.gap_pos
  simp only [g, not_le.mpr A.E_lt_M, ↓reduceIte, upper]
  rw [mul_div_assoc, div_self (ne_of_gt h3)]
  norm_num

/-! ## Range -/

theorem g_nonneg (u : ℚ) : 0 ≤ A.g u := by
  unfold g
  split_ifs with h
  · exact le_max_left _ _
  · exact le_min (by norm_num) (le_trans (by norm_num) (A.upper_ge_50 (le_of_lt (not_le.mp h))))

theorem g_le_100 (u : ℚ) : A.g u ≤ 100 := by
  unfold g
  split_ifs with h
  · exact max_le (by norm_num) (le_trans (A.lower_le_50 h) (by norm_num))
  · exact min_le_left _ _

/-! ## Monotonicity -/

theorem g_mono {u v : ℚ} (h : u ≤ v) : A.g u ≤ A.g v := by
  unfold g
  split_ifs with hu hv hv
  · exact max_le_max (le_refl 0) (A.lower_mono h)
  · have hv' : A.E ≤ v := le_of_lt (not_le.mp hv)
    exact le_trans (max_le (by norm_num) (A.lower_le_50 hu))
      (le_min (by norm_num) (A.upper_ge_50 hv'))
  · exact absurd (le_trans h hv) hu
  · exact min_le_min (le_refl 100) (A.upper_mono h)

/-- Strictly increasing on `[0, M]`: an improvement in utility is never lost in the score. -/
theorem g_strict {u v : ℚ} (hu : 0 ≤ u) (h : u < v) (hv : v ≤ A.M) : A.g u < A.g v := by
  unfold g
  split_ifs with h1 h2 h2
  · rw [max_eq_right (A.lower_nonneg hu), max_eq_right (A.lower_nonneg (by linarith))]
    exact A.lower_strict h
  · have hv' : A.E < v := not_le.mp h2
    rw [max_eq_right (A.lower_nonneg hu), min_eq_right (A.upper_le_100 hv)]
    have := A.upper_strict hv'
    have := A.upper_ge_50 (le_refl A.E)
    have := A.lower_le_50 h1
    linarith
  · exact absurd (le_trans (le_of_lt h) h2) h1
  · have hu' : u ≤ A.M := by linarith
    rw [min_eq_right (A.upper_le_100 hu'), min_eq_right (A.upper_le_100 hv)]
    exact A.upper_strict h

/-- The score does not change when the utility and both anchors are measured on another scale:
only a utility's position between the anchors matters, never its raw magnitude. -/
theorem g_scale (c : ℚ) (hc : 0 < c) (u : ℚ) : (A.scale c hc).g (c * u) = A.g u := by
  have hE := A.E_pos
  have h3 := A.gap_pos
  have hc0 : c ≠ 0 := ne_of_gt hc
  have hlo : (A.scale c hc).lower (c * u) = A.lower u := by
    simp only [lower, scale]
    rw [mul_left_comm, mul_div_mul_left _ _ hc0]
  have hup : (A.scale c hc).upper (c * u) = A.upper u := by
    simp only [upper, scale]
    rw [← mul_sub, ← mul_sub, mul_left_comm, mul_div_mul_left _ _ hc0]
  have hcmp : (c * u ≤ (A.scale c hc).E) ↔ (u ≤ A.E) := by
    simp only [scale]; exact mul_le_mul_iff_of_pos_left hc
  unfold g
  by_cases h : u ≤ A.E
  · simp only [hcmp, h, ↓reduceIte, hlo]
  · simp only [hcmp, h, ↓reduceIte, hup]

end Anchors

end Yata
