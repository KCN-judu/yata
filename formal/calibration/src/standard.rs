//! The Yata Quality Model v1, as numbers: the reference measure, per-archetype anchors and
//! thresholds, and the scoring of one soul (`docs/spec/quality-model.md`). Exact rational
//! arithmetic throughout.

use std::sync::OnceLock;

use num_bigint::BigInt;
use num_traits::{One, Zero};

use crate::model::{self, ATTRS, Dist, HI_TENTHS, Measure, Q, attr, q};

pub const MODEL_ID: &str = "yata-quality-v1";

/// The reference measure of v1: the official notice's class weights, shared equally within a
/// class, and the initial count 2, 3, 4 with 1/3 each.
pub fn reference() -> Measure {
    Measure::official()
}

/// The +15 hit law under the reference measure, computed once.
pub fn reference_final() -> &'static Dist {
    static D: OnceLock<Dist> = OnceLock::new();
    D.get_or_init(|| model::final_hits(&reference()))
}

/// μ: the mean of one increment in roll units. The midpoint of [4/5, 1]; see the spec.
pub fn mu() -> Q {
    q(9, 10)
}

/// The attainable maximum of a four-attribute archetype, in roll units.
pub fn max_u() -> Q {
    q(9, 1)
}

/// E[K]: the expected increments on `useful` in a +15 soul under a measure's hit law.
pub fn mean_useful(d: &Dist, useful: &[usize]) -> Q {
    model::mean_k(&model::k_law(&model::useful_law(d, useful)))
}

#[derive(Clone)]
pub struct Anchors {
    pub e: Q,
    pub m: Q,
}

impl Anchors {
    pub fn g(&self, u: &Q) -> Q {
        let fifty = q(50, 1);
        if u <= &self.e {
            let v = &fifty * u / &self.e;
            if v < Q::zero() { Q::zero() } else { v }
        } else {
            let v = &fifty + &fifty * (u - &self.e) / (&self.m - &self.e);
            if v > q(100, 1) { q(100, 1) } else { v }
        }
    }
}

/// An archetype's anchors: E = μ · E[K] under the reference measure, M = 9.
pub fn anchors(arch: Arch) -> Anchors {
    Anchors {
        e: mu() * mean_useful(reference_final(), &arch.attrs()),
        m: max_u(),
    }
}

/// The band edges are utility milestones, the same for every archetype: R at the archetype's E,
/// SR at μ·5, SSR at μ·7, UR above M − 1. Their scores follow from the archetype's anchors.
pub struct Thresholds {
    pub u_sr: Q,
    pub u_ssr: Q,
    pub u_ur: Q,
    pub c_r: Q,
    pub c_sr: Q,
    pub c_ssr: Q,
    pub t_ur: Q,
}

pub fn thresholds(arch: Arch) -> Thresholds {
    let a = anchors(arch);
    let u_sr = mu() * q(5, 1);
    let u_ssr = mu() * q(7, 1);
    let u_ur = max_u() - Q::one();
    Thresholds {
        c_r: a.g(&a.e),
        c_sr: a.g(&u_sr),
        c_ssr: a.g(&u_ssr),
        t_ur: a.g(&u_ur),
        u_sr,
        u_ssr,
        u_ur,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tier {
    N,
    R,
    Sr,
    Ssr,
    Sp,
    Ur,
}

impl Tier {
    pub fn name(self) -> &'static str {
        match self {
            Tier::N => "N",
            Tier::R => "R",
            Tier::Sr => "SR",
            Tier::Ssr => "SSR",
            Tier::Sp => "SP",
            Tier::Ur => "UR",
        }
    }
}

pub fn tier(score: &Q, specialized: bool, t: &Thresholds) -> Tier {
    if score > &t.t_ur {
        Tier::Ur
    } else if specialized && score >= &t.c_ssr {
        Tier::Sp
    } else if score < &t.c_r {
        Tier::N
    } else if score < &t.c_sr {
        Tier::R
    } else if score < &t.c_ssr {
        Tier::Sr
    } else {
        Tier::Ssr
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arch {
    Output,
    Hit,
    Resist,
}

pub const ARCHES: [Arch; 3] = [Arch::Output, Arch::Hit, Arch::Resist];

impl Arch {
    pub fn key(self) -> &'static str {
        match self {
            Arch::Output => "output",
            Arch::Hit => "hit",
            Arch::Resist => "resist",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Arch::Output => 0,
            Arch::Hit => 1,
            Arch::Resist => 2,
        }
    }

    pub fn attrs(self) -> [usize; 4] {
        use attr::*;
        match self {
            Arch::Output => [ATK_PCT, CRIT, CRIT_DMG, SPD],
            Arch::Hit => [SPD, EFF_HIT, HP_PCT, DEF_PCT],
            Arch::Resist => [SPD, EFF_RES, HP_PCT, DEF_PCT],
        }
    }

    /// The main attributes this archetype can use on a slot (1–6).
    pub fn accepts(self, slot: u8, main: usize) -> bool {
        use attr::*;
        match slot {
            1 | 3 | 5 => true,
            2 => match self {
                Arch::Output => [SPD, ATK_PCT].contains(&main),
                Arch::Hit | Arch::Resist => [SPD, HP_PCT, DEF_PCT].contains(&main),
            },
            4 => match self {
                Arch::Output => main == ATK_PCT,
                Arch::Hit => [EFF_HIT, HP_PCT, DEF_PCT].contains(&main),
                Arch::Resist => [EFF_RES, HP_PCT, DEF_PCT].contains(&main),
            },
            6 => match self {
                Arch::Output => [CRIT, CRIT_DMG, ATK_PCT].contains(&main),
                Arch::Hit | Arch::Resist => [HP_PCT, DEF_PCT].contains(&main),
            },
            _ => false,
        }
    }
}

/// Parse a decimal such as "17.4" exactly.
pub fn dec(s: &str) -> Q {
    let (int, frac) = s.split_once('.').unwrap_or((s, ""));
    let digits = format!("{int}{frac}");
    let n: BigInt = digits.parse().unwrap_or_default();
    let d: BigInt = num_traits::pow(BigInt::from(10), frac.len());
    Q::new(n, d)
}

/// One sub-attribute line: attribute, stored value (display units, decimal), increments.
#[derive(Clone)]
pub struct Line {
    pub attr: usize,
    pub value: &'static str,
    pub hits: u8,
}

#[derive(Clone)]
pub struct SoulInput {
    pub id: &'static str,
    pub what: &'static str,
    pub slot: u8,
    pub main: usize,
    pub level: u8,
    pub lines: Vec<Line>,
}

impl SoulInput {
    /// `e_a = S(a) / hi(a)`.
    pub fn features(&self) -> [Q; ATTRS] {
        let mut e: [Q; ATTRS] = std::array::from_fn(|_| Q::zero());
        for l in &self.lines {
            e[l.attr] = dec(l.value) / q(HI_TENTHS[l.attr], 10);
        }
        e
    }

    /// W-Soul at six stars, and the increment count the level allows.
    pub fn check(&self) -> Result<(), String> {
        if self.lines.len() > 4 {
            return Err("more than four lines".into());
        }
        let total: u32 = self.lines.iter().map(|l| u32::from(l.hits)).sum();
        let rolls = u32::from(self.level / 3);
        if !(2 + rolls..=4 + rolls).contains(&total) || total > 9 {
            return Err(format!("{total} increments at +{}", self.level));
        }
        for l in &self.lines {
            let v = dec(l.value);
            let hi = q(HI_TENTHS[l.attr], 10);
            let lo = &hi * q(4, 5);
            let h = Q::from_integer(BigInt::from(l.hits));
            if l.hits == 0 || l.hits > 6 || v < &h * &lo || v > &h * &hi {
                return Err(format!(
                    "{} {} with {} increments",
                    model::NAMES[l.attr],
                    l.value,
                    l.hits
                ));
            }
        }
        Ok(())
    }
}

pub struct ArchScore {
    pub arch: Arch,
    pub utility: Q,
    pub score: Q,
    pub specialized: bool,
    pub tier: Tier,
    pub depth: Q,
    pub breadth: Q,
}

pub fn score_arch(s: &SoulInput, arch: Arch) -> ArchScore {
    let e = s.features();
    let a = anchors(arch);
    let t = thresholds(arch);
    let vals: Vec<Q> = arch.attrs().iter().map(|&i| e[i].clone()).collect();
    let utility: Q = vals.iter().cloned().sum();
    let score = a.g(&utility);
    let specialized = vals.iter().any(|v| v > &q(5, 1));
    let depth = vals
        .iter()
        .cloned()
        .fold(Q::zero(), |m, v| if v > m { v } else { m })
        / q(6, 1)
        * q(100, 1);
    let sq: Q = vals.iter().map(|v| v * v).sum();
    let breadth = if utility.is_zero() {
        Q::zero()
    } else {
        let ed = &utility * &utility / sq;
        (ed - Q::one()) / q(3, 1) * q(100, 1)
    };
    ArchScore {
        arch,
        tier: tier(&score, specialized, &t),
        utility,
        score,
        specialized,
        depth,
        breadth,
    }
}

/// Quality: the best accepted archetype, tier first, then score; ties keep catalogue order.
pub fn quality(s: &SoulInput) -> Vec<ArchScore> {
    let mut all: Vec<ArchScore> = ARCHES
        .iter()
        .filter(|a| a.accepts(s.slot, s.main))
        .map(|&a| score_arch(s, a))
        .collect();
    all.sort_by(|x, y| (y.tier, &y.score).cmp(&(x.tier, &x.score)));
    all
}

/// Growth: the score of the expected +15 utility under an archetype, for a soul below +15, by
/// exact propagation of the reference measure.
pub fn growth(s: &SoulInput, arch: Arch) -> Option<Q> {
    if s.level >= 15 {
        return None;
    }
    let rolls = 5 - usize::from(s.level / 3);
    let mut from = [0u8; ATTRS];
    for l in &s.lines {
        from[l.attr] = l.hits;
    }
    let useful = arch.attrs();
    let now: u32 = useful.iter().map(|&a| u32::from(from[a])).sum();
    let d = model::evolve(&reference(), from, rolls);
    let mean_after: Q = d
        .iter()
        .map(|(h, p)| {
            let k: u32 = useful.iter().map(|&a| u32::from(h[a])).sum();
            Q::from_integer(BigInt::from(k - now)) * p
        })
        .sum();
    let e = s.features();
    let u_now: Q = useful.iter().map(|&i| e[i].clone()).sum();
    Some(anchors(arch).g(&(u_now + mu() * mean_after)))
}

/// The spec's recursion for the expected further useful increments, `f(r, P)`, over the set `P`
/// of present attributes (a bitmask) and the reference weights.
pub fn growth_recursion(r: u32, present: u16, useful: &[usize]) -> Q {
    if r == 0 {
        return Q::zero();
    }
    let is_useful = |a: usize| useful.contains(&a);
    if present.count_ones() == 4 {
        let u = (0..ATTRS)
            .filter(|&a| present >> a & 1 == 1 && is_useful(a))
            .count();
        return Q::from_integer(BigInt::from(u64::from(r) * u as u64)) / q(4, 1);
    }
    let m = reference();
    (0..ATTRS)
        .map(|a| {
            let hit = if is_useful(a) { Q::one() } else { Q::zero() };
            &m.weight[a] * (hit + growth_recursion(r - 1, present | (1 << a), useful))
        })
        .sum()
}
