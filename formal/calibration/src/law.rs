//! The law of one increment's size, in roll units (`δ / hi(a) ∈ [4/5, 1]`), and of sums of
//! increments. The distribution within the range is open (`soul-mechanics.md`, § Open); the
//! calibration uses a continuous uniform law and reports the sensitivity to discrete ones.

use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};

use crate::model::{Q, q};

#[derive(Clone, Copy)]
pub enum Law {
    /// Continuous uniform on [4/5, 1].
    Uniform,
    /// Uniform over `n` equally spaced values from 4/5 to 1.
    Steps(u32),
}

impl Law {
    pub fn name(self) -> String {
        match self {
            Law::Uniform => "continuous uniform on [0.8, 1]".to_owned(),
            Law::Steps(n) => format!("{n} equal steps on [0.8, 1]"),
        }
    }
}

fn binom(n: u32, k: u32) -> BigInt {
    (0..k).fold(BigInt::one(), |acc, i| {
        acc * BigInt::from(n - i) / BigInt::from(i + 1)
    })
}

fn factorial(n: u32) -> BigInt {
    (1..=n).fold(BigInt::one(), |acc, i| acc * BigInt::from(i))
}

/// The Irwin–Hall CDF of `k` standard uniforms at `x`, exactly.
fn irwin_hall_cdf(k: u32, x: &Q) -> Q {
    if x <= &Q::zero() {
        return Q::zero();
    }
    if x >= &Q::from_integer(BigInt::from(k)) {
        return Q::one();
    }
    let floor = x.floor().to_integer().to_u32().unwrap_or(0);
    let mut s = Q::zero();
    for j in 0..=floor {
        let t = x - Q::from_integer(BigInt::from(j));
        let term = Q::from_integer(binom(k, j)) * num_traits::pow(t, k as usize);
        if j % 2 == 0 {
            s += term;
        } else {
            s -= term;
        }
    }
    s / Q::from_integer(factorial(k))
}

/// P(sum of `k` increments ≥ t).
pub fn tail(law: Law, k: u32, t: &Q) -> Q {
    if k == 0 {
        return if t <= &Q::zero() { Q::one() } else { Q::zero() };
    }
    match law {
        Law::Uniform => {
            // S = 4k/5 + (1/5)·IH_k; continuous, so ≥ and > agree.
            let x = (t - q(4 * i64::from(k), 5)) * q(5, 1);
            Q::one() - irwin_hall_cdf(k, &x)
        }
        Law::Steps(n) => {
            let d = steps_sum(n, k);
            d.iter()
                .filter(|(v, _)| v >= t)
                .map(|(_, p)| p.clone())
                .sum()
        }
    }
}

/// The exact law of a sum of `k` step increments.
pub fn steps_sum(n: u32, k: u32) -> Vec<(Q, Q)> {
    let vals: Vec<Q> = (0..n)
        .map(|j| q(4, 5) + q(i64::from(j), 5 * i64::from(n - 1)))
        .collect();
    let p = q(1, i64::from(n));
    let mut d: std::collections::BTreeMap<Q, Q> =
        std::collections::BTreeMap::from([(Q::zero(), Q::one())]);
    for _ in 0..k {
        let mut next = std::collections::BTreeMap::new();
        for (s, ps) in &d {
            for v in &vals {
                *next.entry(s + v).or_insert_with(Q::zero) += ps * &p;
            }
        }
        d = next;
    }
    d.into_iter().collect()
}

/// The Irwin–Hall density of `k` uniforms at `x`, in floating point, for integration.
fn irwin_hall_pdf(k: u32, x: f64) -> f64 {
    if x <= 0.0 || x >= f64::from(k) {
        return 0.0;
    }
    let mut s = 0.0;
    for j in 0..=(x.floor() as u32) {
        let b = binom(k, j).to_f64().unwrap_or(0.0);
        let term = b * (x - f64::from(j)).powi(k as i32 - 1);
        s += if j % 2 == 0 { term } else { -term };
    }
    s / factorial(k - 1).to_f64().unwrap_or(1.0)
}

/// P(X > 5 ∧ X + Y ≥ c), X the sum of six increments, Y of `m` more, all independent. Exact for
/// step laws; Simpson's rule on 20 000 panels for the uniform law (error far below 1e-9).
pub fn specialized_and_total(law: Law, m: u32, c: &Q) -> Q {
    match law {
        Law::Steps(n) => {
            let x6 = steps_sum(n, 6);
            let five = q(5, 1);
            x6.iter()
                .filter(|(x, _)| x > &five)
                .map(|(x, px)| px * tail(law, m, &(c - x)))
                .sum()
        }
        Law::Uniform => {
            let c = c.to_f64().unwrap_or(0.0);
            let panels = 20_000u32;
            let (a, b) = (5.0f64, 6.0f64);
            let h = (b - a) / f64::from(panels);
            let f = |x: f64| {
                // density of X = 4.8 + IH_6/5 at x
                let dens = 5.0 * irwin_hall_pdf(6, (x - 4.8) * 5.0);
                let t = tail(Law::Uniform, m, &from_f64(c - x));
                dens * t.to_f64().unwrap_or(0.0)
            };
            let mut s = f(a) + f(b);
            for i in 1..panels {
                let x = a + h * f64::from(i);
                s += if i % 2 == 1 { 4.0 * f(x) } else { 2.0 * f(x) };
            }
            from_f64(s * h / 3.0)
        }
    }
}

/// A float as a rational with 1e-12 resolution, for the one numerical path.
fn from_f64(x: f64) -> Q {
    let scale = 1_000_000_000_000i64;
    let n = (x * scale as f64).round() as i64;
    Q::new(BigInt::from(n), BigInt::from(scale))
}

/// `|x|` as f64, for display.
pub fn f(x: &Q) -> f64 {
    x.to_f64().unwrap_or(f64::NAN)
}
