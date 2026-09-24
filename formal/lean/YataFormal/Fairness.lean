import Mathlib
import YataFormal.Soul

/-!
# Cross-profile fairness

The anchors of a profile come from a reference measure: a finite distribution over outcomes, each
with a feature vector. `E_p` is the expected utility of `p` under it. If the reference measure
treats attributes symmetrically — relabelling attributes by a permutation `π` maps it onto itself,
as the 1/11 draw model does, since every six-star attribute has the same range in roll units —
then a profile and its relabelling have the same expected utility, hence the same anchors, and a
relabelled soul scores exactly what the original scores. A Speed profile that is the image of a
Crit profile is judged on the same scale: neither gains from how often its attributes occur.
-/

namespace Yata

/-- A finite reference distribution over outcomes `Ω`, each with the features it produces. -/
structure RefMeasure (Ω : Type) [Fintype Ω] where
  prob : Ω → ℚ
  prob_nonneg : ∀ ω, 0 ≤ prob ω
  prob_sum : ∑ ω, prob ω = 1
  feat : Ω → Features

variable {Ω : Type} [Fintype Ω]

/-- `E_p`: the expected utility of a profile under the reference measure. -/
def RefMeasure.expected (μ : RefMeasure Ω) (p : Profile) : ℚ :=
  ∑ ω, μ.prob ω * p.utility (μ.feat ω)

/-- Features with attributes relabelled by `π`: attribute `π a` now holds what `a` held. -/
def relabelF (π : Attr ≃ Attr) (x : Features) : Features := fun a => x (π.symm a)

/-- The profile valuing `π a` as `p` valued `a`. -/
def Profile.relabel (p : Profile) (π : Attr ≃ Attr) : Profile where
  useful := p.useful.map π.toEmbedding
  weight := fun a => p.weight (π.symm a)
  weight_pos := fun a ha => by
    obtain ⟨b, hb, rfl⟩ := Finset.mem_map.mp ha
    simpa using p.weight_pos b hb
  weight_zero := fun a ha => by
    apply p.weight_zero
    intro hb
    exact ha (Finset.mem_map.mpr ⟨π.symm a, hb, by simp⟩)

/-- Relabelling both the profile and the soul leaves the utility unchanged. -/
theorem utility_relabel (p : Profile) (π : Attr ≃ Attr) (x : Features) :
    (p.relabel π).utility (relabelF π x) = p.utility x := by
  unfold Profile.utility Profile.relabel relabelF
  exact Equiv.sum_comp π.symm (fun a => p.weight a * x a)

/-- A reference measure is symmetric under `π` when some bijection of outcomes preserves every
probability and relabels every outcome's features by `π`. -/
def RefMeasure.SymmetricUnder (μ : RefMeasure Ω) (π : Attr ≃ Attr) : Prop :=
  ∃ σ : Ω ≃ Ω, ∀ ω, μ.prob (σ ω) = μ.prob ω ∧ μ.feat (σ ω) = relabelF π (μ.feat ω)

/-- Under a symmetric reference measure, a profile and its relabelling expect the same utility. -/
theorem expected_relabel (μ : RefMeasure Ω) (p : Profile) (π : Attr ≃ Attr)
    (hμ : μ.SymmetricUnder π) : μ.expected (p.relabel π) = μ.expected p := by
  obtain ⟨σ, hσ⟩ := hμ
  unfold RefMeasure.expected
  rw [← Equiv.sum_comp σ]
  apply Finset.sum_congr rfl
  intro ω _
  rw [(hσ ω).1, (hσ ω).2, utility_relabel]

/-- Anchors of a profile: its expected utility and a given attainable maximum. -/
def anchorsOf (μ : RefMeasure Ω) (p : Profile) (M : ℚ) (h0 : 0 < μ.expected p)
    (h1 : μ.expected p < M) : Anchors := ⟨μ.expected p, M, h0, h1⟩

/-- **Fairness under relabelling.** With a symmetric reference measure and equal attainable
maxima, the relabelled profile scores the relabelled soul exactly as the original profile scores
the original soul. The score of a Speed-focused soul under a Speed profile equals the score of
its Crit-focused counterpart under the Crit profile. -/
theorem score_relabel (μ : RefMeasure Ω) (p : Profile) (π : Attr ≃ Attr)
    (hμ : μ.SymmetricUnder π) (M : ℚ) (x : Features)
    (h0 : 0 < μ.expected p) (h1 : μ.expected p < M)
    (h0' : 0 < μ.expected (p.relabel π)) (h1' : μ.expected (p.relabel π) < M) :
    (p.relabel π).score (anchorsOf μ (p.relabel π) M h0' h1') (relabelF π x)
      = p.score (anchorsOf μ p M h0 h1) x := by
  unfold Profile.score
  rw [utility_relabel]
  have hE := expected_relabel μ p π hμ
  unfold anchorsOf Anchors.g
  simp only [hE]

end Yata
