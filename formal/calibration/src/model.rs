//! The generative model of a six-star soul's sub-attributes, with exact rational probabilities
//! (`docs/spec/quality-model.md`, "The generative model").
//!
//! Only which attribute each increment lands on is modelled here; how large an increment is, is
//! `law`'s. A state is the hit count of each of the eleven attributes.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Mutex, OnceLock};

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

pub type Q = BigRational;

pub fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

pub const ATTRS: usize = 11;

/// Attribute indices, in the game's attribute order.
pub mod attr {
    pub const ATK_FLAT: usize = 0;
    pub const ATK_PCT: usize = 1;
    pub const DEF_FLAT: usize = 2;
    pub const DEF_PCT: usize = 3;
    pub const HP_FLAT: usize = 4;
    pub const HP_PCT: usize = 5;
    pub const SPD: usize = 6;
    pub const EFF_HIT: usize = 7;
    pub const EFF_RES: usize = 8;
    pub const CRIT: usize = 9;
    pub const CRIT_DMG: usize = 10;
}

pub const NAMES: [&str; ATTRS] = [
    "AtkFlat",
    "AtkPercent",
    "DefFlat",
    "DefPercent",
    "HpFlat",
    "HpPercent",
    "Spd",
    "EffectHit",
    "EffectRes",
    "Crit",
    "CritDmg",
];

/// One increment's largest value, `hi(a)`, at six stars, in display units, as (numerator, 10).
pub const HI_TENTHS: [i64; ATTRS] = [270, 30, 50, 30, 1140, 30, 30, 40, 40, 30, 40];

/// A reference measure: the weight of each attribute in a draw, and the law of the initial count.
#[derive(Clone)]
pub struct Measure {
    pub name: &'static str,
    pub weight: [Q; ATTRS],
    /// P(initial count = 2, 3, 4).
    pub initial: [Q; 3],
}

impl Measure {
    /// The analysis's reading (Hu 2026, § 3.2): every attribute 1/11, the notice's figures as
    /// rounding. Not adopted: the official figures rank first. Kept for sensitivity.
    pub fn hu_reading() -> Measure {
        Measure {
            name: "1/11 per attribute (Hu 2026 reading)",
            weight: std::array::from_fn(|_| q(1, 11)),
            initial: [q(1, 3), q(1, 3), q(1, 3)],
        }
    }

    /// The official notice: 攻击类 36%, 防御类 36%, 功能类 28%, shared equally within a class;
    /// initial count 2, 3, 4 with 1/3 each (Hu 2026, § 3.1). The v1 reference measure.
    pub fn official() -> Measure {
        let w = std::array::from_fn(|a| match a {
            attr::SPD | attr::EFF_HIT | attr::EFF_RES => q(28, 300),
            _ => q(9, 100),
        });
        Measure {
            name: "36/36/28 per class (official notice; the reference)",
            weight: w,
            initial: [q(1, 3), q(1, 3), q(1, 3)],
        }
    }

    pub fn with_initial(mut self, initial: [Q; 3], name: &'static str) -> Measure {
        self.initial = initial;
        self.name = name;
        self
    }
}

/// Hit counts per attribute.
pub type Hits = [u8; ATTRS];

pub type Dist = BTreeMap<Hits, Q>;

fn present(h: &Hits) -> usize {
    h.iter().filter(|&&x| x > 0).count()
}

/// The initial sub-attributes: `n` distinct attributes drawn one by one by weight, without
/// replacement (the draws cannot repeat: an assumption, see the spec).
fn initial(m: &Measure) -> Dist {
    let mut out = Dist::new();
    for (i, pn) in m.initial.iter().enumerate() {
        let n = i + 2;
        let mut layer: Dist = BTreeMap::from([([0u8; ATTRS], pn.clone())]);
        for _ in 0..n {
            let mut next = Dist::new();
            for (h, p) in &layer {
                let free: Q = (0..ATTRS)
                    .filter(|&a| h[a] == 0)
                    .map(|a| m.weight[a].clone())
                    .sum();
                for a in (0..ATTRS).filter(|&a| h[a] == 0) {
                    let mut g = *h;
                    g[a] = 1;
                    *next.entry(g).or_insert_with(Q::zero) += p * &m.weight[a] / &free;
                }
            }
            layer = next;
        }
        for (h, p) in layer {
            *out.entry(h).or_insert_with(Q::zero) += p;
        }
    }
    out
}

/// One roll: below four sub-attributes a weighted draw over all eleven, which adds or
/// strengthens (P-Draw); at four, each present attribute with 1/4 (P-Four).
pub fn roll(m: &Measure, d: &Dist) -> Dist {
    let mut next = Dist::new();
    let quarter = q(1, 4);
    for (h, p) in d {
        if present(h) < 4 {
            for a in 0..ATTRS {
                let mut g = *h;
                g[a] += 1;
                *next.entry(g).or_insert_with(Q::zero) += p * &m.weight[a];
            }
        } else {
            for a in (0..ATTRS).filter(|&a| h[a] > 0) {
                let mut g = *h;
                g[a] += 1;
                *next.entry(g).or_insert_with(Q::zero) += p * &quarter;
            }
        }
    }
    next
}

/// The hit counts of a soul at +15, computed once per measure: the propagation is the costliest
/// exact step, and every table asks for it again.
pub fn final_hits(m: &Measure) -> &'static Dist {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, &'static Dist>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    map.entry(m.name).or_insert_with(|| {
        let mut d = initial(m);
        for _ in 0..5 {
            d = roll(m, &d);
        }
        Box::leak(Box::new(d))
    })
}

/// The hit counts after `rolls` more rolls from one state.
pub fn evolve(m: &Measure, from: Hits, rolls: usize) -> Dist {
    let mut d: Dist = BTreeMap::from([(from, Q::one())]);
    for _ in 0..rolls {
        d = roll(m, &d);
    }
    d
}

/// The law of the useful hit vector: for each final state, the hits on `useful`, sorted
/// descending.
pub fn useful_law(d: &Dist, useful: &[usize]) -> BTreeMap<Vec<u8>, Q> {
    let mut out = BTreeMap::new();
    for (h, p) in d {
        let mut v: Vec<u8> = useful.iter().map(|&a| h[a]).collect();
        v.sort_unstable_by(|a, b| b.cmp(a));
        *out.entry(v).or_insert_with(Q::zero) += p;
    }
    out
}

/// The useful law of a measure's +15 hit law, computed once per measure and attribute set.
pub fn useful_law_of(m: &Measure, useful: &[usize]) -> &'static BTreeMap<Vec<u8>, Q> {
    type Law = BTreeMap<Vec<u8>, Q>;
    type Memo = HashMap<(&'static str, Vec<usize>), &'static Law>;
    static CACHE: OnceLock<Mutex<Memo>> = OnceLock::new();
    let d = final_hits(m);
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    map.entry((m.name, useful.to_vec()))
        .or_insert_with(|| Box::leak(Box::new(useful_law(d, useful))))
}

/// The law of `K`, the increments on useful attributes.
pub fn k_law(law: &BTreeMap<Vec<u8>, Q>) -> BTreeMap<u32, Q> {
    let mut out = BTreeMap::new();
    for (v, p) in law {
        let k: u32 = v.iter().map(|&x| u32::from(x)).sum();
        *out.entry(k).or_insert_with(Q::zero) += p;
    }
    out
}

pub fn mean_k(k: &BTreeMap<u32, Q>) -> Q {
    k.iter()
        .map(|(n, p)| Q::from_integer(BigInt::from(*n)) * p)
        .sum()
}
