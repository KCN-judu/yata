//! Exact tier rates of one archetype under a reference measure and an increment law.

use num_traits::{One, Zero};

use crate::law::{self, Law};
use crate::model::{self, Measure, Q, q};
use crate::standard::{Arch, anchors, thresholds};

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
fn tail_gt(l: Law, k: u32, t: &Q) -> Q {
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

pub fn tier_rates(m: &Measure, l: Law, arch: Arch) -> Rates {
    let d = model::final_hits(m);
    let ul = model::useful_law(&d, &arch.attrs());
    let e = crate::standard::mu() * model::mean_k(&model::k_law(&ul));
    let t = thresholds(arch);
    let a = anchors(arch);
    // The band edges are v1's, whatever measure the rates are computed under.
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
    for (v, p) in &ul {
        let k: u32 = v.iter().map(|&x| u32::from(x)).sum();
        if k == 9 {
            ur += p * tail_gt(l, 9, &t.u_ur);
        }
        if v.first() == Some(&6) {
            let mut s = law::specialized_and_total(l, k - 6, &t.u_ssr);
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
