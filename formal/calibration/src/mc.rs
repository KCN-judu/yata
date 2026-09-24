//! Monte Carlo, for what exact propagation does not cover: the quality maximum over
//! overlapping archetypes, and the comparison of normalizations. A fixed seed makes every run
//! identical; each rate is reported with its standard error.

use crate::law::f;
use crate::model::{ATTRS, Measure};
use crate::standard::{ARCHES, Arch, anchors, speed_gate, thresholds, u_speed_ur};

/// SplitMix64: small, fixed, and enough for this.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform on [0, 1).
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// One +15 soul's sub-attribute values in roll units, drawn from the model with uniform
/// increments.
pub fn draw(m: &Measure, rng: &mut Rng) -> [f64; ATTRS] {
    let w: Vec<f64> = m.weight.iter().map(f).collect();
    let init: Vec<f64> = m.initial.iter().map(f).collect();
    let r = rng.unit();
    let n0 = if r < init[0] {
        2
    } else if r < init[0] + init[1] {
        3
    } else {
        4
    };
    let mut e = [0.0f64; ATTRS];
    let mut present = [false; ATTRS];
    let inc = |rng: &mut Rng| 0.8 + 0.2 * rng.unit();
    for _ in 0..n0 {
        let free: f64 = (0..ATTRS).filter(|&a| !present[a]).map(|a| w[a]).sum();
        let mut x = rng.unit() * free;
        let mut pick = 0;
        for a in (0..ATTRS).filter(|&a| !present[a]) {
            pick = a;
            if x < w[a] {
                break;
            }
            x -= w[a];
        }
        present[pick] = true;
        e[pick] += inc(rng);
    }
    for _ in 0..5 {
        let count = present.iter().filter(|&&p| p).count();
        let pick = if count < 4 {
            let mut x = rng.unit();
            let mut pick = 0;
            for (a, wa) in w.iter().enumerate() {
                pick = a;
                if x < *wa {
                    break;
                }
                x -= wa;
            }
            pick
        } else {
            let k = (rng.unit() * 4.0) as usize;
            (0..ATTRS)
                .filter(|&a| present[a])
                .nth(k.min(3))
                .unwrap_or(0)
        };
        present[pick] = true;
        e[pick] += inc(rng);
    }
    e
}

pub struct F64Thresholds {
    pub e: f64,
    pub m: f64,
    pub c_r: f64,
    pub c_sr: f64,
    pub c_ssr: f64,
    pub c_sp: f64,
    pub t_ur: f64,
    /// `speed`'s gate and UR boundary in roll units, kept here so that the inner loop does no
    /// rational arithmetic.
    pub gate: f64,
    pub u_speed_ur: f64,
}

/// The four-attribute ladder of an archetype in floating point; for `speed`, only its anchors
/// are meaningful, and its tier is `speed_tier`'s rule.
pub fn f64_thresholds(arch: Arch) -> F64Thresholds {
    let a = anchors(arch);
    let t = thresholds(if arch == Arch::Speed {
        Arch::Output
    } else {
        arch
    });
    F64Thresholds {
        e: f(&a.e),
        m: f(&a.m),
        c_r: f(&t.c_r),
        c_sr: f(&t.c_sr),
        c_ssr: f(&t.c_ssr),
        c_sp: f(&t.c_sp),
        t_ur: f(&t.t_ur),
        gate: f(&speed_gate()),
        u_speed_ur: f(&u_speed_ur()),
    }
}

/// The thresholds of every archetype, in catalogue order.
pub fn all_thresholds() -> [F64Thresholds; 4] {
    ARCHES.map(f64_thresholds)
}

pub fn g(t: &F64Thresholds, u: f64) -> f64 {
    if u <= t.e {
        (50.0 * u / t.e).max(0.0)
    } else {
        (50.0 + 50.0 * (u - t.e) / (t.m - t.e)).min(100.0)
    }
}

/// Tier index 0..=5 (N..UR) on the four-attribute ladder.
pub fn tier(t: &F64Thresholds, score: f64, spec: bool) -> usize {
    if score > t.t_ur {
        5
    } else if spec && score >= t.c_sp {
        4
    } else if score < t.c_r {
        0
    } else if score < t.c_sr {
        1
    } else if score < t.c_ssr {
        2
    } else {
        3
    }
}

/// The tier index and score under an archetype, or `None` when the soul is not eligible for it.
pub fn arch_tier(ts: &[F64Thresholds; 4], e: &[f64; ATTRS], a: Arch) -> Option<(usize, f64)> {
    let t = &ts[a.index()];
    let vals: Vec<f64> = a.attrs().iter().map(|&i| e[i]).collect();
    let u: f64 = vals.iter().sum();
    let s = g(t, u);
    if a == Arch::Speed {
        let gate = t.gate;
        if u < gate {
            return None;
        }
        return Some((if u > t.u_speed_ur { 5 } else { 3 }, s));
    }
    Some((tier(t, s, vals.iter().any(|&v| v > 5.0)), s))
}

/// Index of the "unrated" column: no candidate archetype.
pub const UNRATED: usize = 6;

/// The quality tier of a soul whose main is accepted by `accepted`: the best over the eligible
/// ones, or `UNRATED` when none is eligible.
pub fn quality_tier(ts: &[F64Thresholds; 4], e: &[f64; ATTRS], accepted: &[Arch]) -> (usize, f64) {
    accepted
        .iter()
        .filter_map(|&a| arch_tier(ts, e, a))
        .fold((UNRATED, -1.0), |best, x| {
            if best.0 == UNRATED || x.0 > best.0 || (x.0 == best.0 && x.1 > best.1) {
                x
            } else {
                best
            }
        })
}
