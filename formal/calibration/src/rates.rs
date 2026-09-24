//! Exact tier rates of one archetype under a reference measure and an increment law.

use num_traits::{One, Zero};

use crate::law::{self, Law};
use crate::model::{self, Measure, Q, attr, q};
use crate::standard::{Arch, Thresholds, anchors, speed_gate, thresholds, u_speed_ur};

pub struct Rates {
    pub n: Q,
    pub r: Q,
    pub sr: Q,
    pub ssr: Q,
    pub sp: Q,
    pub ur: Q,
    pub e: Q,
}

/// P(sum of `k` increments > t).
pub fn tail_gt(l: Law, k: u32, t: &Q) -> Q {
    match l {
        Law::Uniform => law::tail(l, k, t),
        Law::Steps(n) => {
            if k == 0 {
                return if t < &Q::zero() { Q::one() } else { Q::zero() };
            }
            law::steps_sum(n, k)
                .iter()
                .filter(|(v, _)| v > t)
                .map(|(_, p)| p.clone())
                .sum()
        }
    }
}

/// P(X₆ > 5 ∧ X₆ + Y₃ > 8), the specialized part of UR.
fn spec_and_gt8(l: Law) -> Q {
    match l {
        Law::Uniform => law::specialized_and_total(l, 3, &q(8, 1)),
        Law::Steps(n) => {
            let five = q(5, 1);
            let eight = q(8, 1);
            law::steps_sum(n, 6)
                .iter()
                .filter(|(x, _)| x > &five)
                .map(|(x, px)| px * tail_gt(l, 3, &(&eight - x)))
                .sum()
        }
    }
}

/// The rates of a four-attribute archetype.
pub fn tier_rates(m: &Measure, l: Law, arch: Arch) -> Rates {
    tier_rates_with(m, l, arch, &thresholds(arch))
}

/// The rates under given thresholds. SP and UR lie inside `U ≥ u_ssr` as long as the SSR
/// milestone is at most the SP floor, which every calibration compared here satisfies.
pub fn tier_rates_with(m: &Measure, l: Law, arch: Arch, t: &Thresholds) -> Rates {
    assert!(
        t.u_ssr <= t.u_sp,
        "SSR's edge must not lie above the SP floor"
    );
    let ul = model::useful_law_of(m, arch.attrs());
    let e = crate::standard::mu() * model::mean_k(&model::k_law(ul));
    let a = anchors(arch);
    // The band edges are the reference measure's, whatever measure the rates are computed under.
    let ge = |u: &Q| -> Q {
        ul.iter()
            .map(|(v, p)| {
                let k: u32 = v.iter().map(|&x| u32::from(x)).sum();
                p * law::tail(l, k, u)
            })
            .sum()
    };
    let p_r = ge(&a.e);
    let p_sr = ge(&t.u_sr);
    let p_top = ge(&t.u_ssr);
    let mut ur = Q::zero();
    let mut sp = Q::zero();
    for (v, p) in ul {
        let k: u32 = v.iter().map(|&x| u32::from(x)).sum();
        if k == 9 {
            ur += p * tail_gt(l, 9, &t.u_ur);
        }
        if v.first() == Some(&6) {
            let mut s = law::specialized_and_total(l, k - 6, &t.u_sp);
            if k == 9 {
                s -= spec_and_gt8(l);
            }
            sp += p * s;
        }
    }
    Rates {
        n: Q::one() - &p_r,
        r: &p_r - &p_sr,
        sr: &p_sr - &p_top,
        ssr: &p_top - &sp - &ur,
        sp,
        ur,
        e,
    }
}

/// `speed` under a measure and a law: how often a +15 soul is eligible, and its tiers. Every
/// probability is of all +15 souls unless it says "of eligible".
pub struct SpeedRates {
    /// E_speed = μ · E[K_Spd], unconditional.
    pub e: Q,
    pub eligible: Q,
    pub ssr: Q,
    pub ur: Q,
    /// Of eligible souls: e_Spd in [4, 4.5), [4.5, 5], (5, 5.5], (5.5, 6].
    pub bins: [Q; 4],
    /// Of eligible souls: the share with 4, 5 and 6 increments on Speed.
    pub by_hits: [Q; 3],
    /// E[e_Spd | e_Spd ≥ 4]: the expectation a reference conditioned on the gate would use.
    pub e_given_eligible: Q,
    /// Of eligible souls: the share below that conditional expectation, which a conditioned
    /// reference would score under 50.
    pub below_given: Q,
}

pub fn speed_rates(m: &Measure, l: Law) -> SpeedRates {
    let kl = model::k_law(model::useful_law_of(m, &[attr::SPD]));
    let e = crate::standard::mu() * model::mean_k(&kl);
    let ge = |u: &Q| -> Q { kl.iter().map(|(k, p)| p * law::tail(l, *k, u)).sum() };
    let gt = |u: &Q| -> Q { kl.iter().map(|(k, p)| p * tail_gt(l, *k, u)).sum() };
    let gate = speed_gate();
    let eligible = ge(&gate);
    let ur = gt(&u_speed_ur());
    let (x45, x5, x55) = (q(9, 2), q(5, 1), q(11, 2));
    let of = |x: Q| x / &eligible;
    let bins = [
        of(&eligible - ge(&x45)),
        of(ge(&x45) - gt(&x5)),
        of(gt(&x5) - gt(&x55)),
        of(gt(&x55)),
    ];
    let hits = |k: u32| of(kl.get(&k).map_or(Q::zero(), |p| p * law::tail(l, k, &gate)));
    let e_given_eligible = of(kl
        .iter()
        .map(|(k, p)| p * law::partial_mean(l, *k, &gate))
        .sum());
    let below_given = of(&eligible - ge(&e_given_eligible));
    SpeedRates {
        below_given,
        e_given_eligible,
        e,
        ssr: &eligible - &ur,
        ur,
        bins,
        by_hits: [hits(4), hits(5), hits(6)],
        eligible,
    }
}
