//! The tables that simulate: the Monte Carlo check of the exact rates, the quality of a soul over
//! its candidate archetypes, and the comparison of normalizations. Fixed seeds make every run
//! identical.

use std::fmt::Write as _;

use crate::law::{Law, f};
use crate::model::{self, Q, attr};
use crate::standard::{self, ARCHES, Arch, mu};
use crate::{mc, pct, rates};

const MC_N: usize = 4_000_000;
const NORM_N: usize = 1_000_000;

fn se(p: f64) -> f64 {
    (p * (1.0 - p) / MC_N as f64).sqrt()
}

/// A row of the quality table: a label, the legal slots and mains it covers, their archetypes.
type Group = (&'static str, Vec<(u8, usize)>, Vec<Arch>);

pub fn monte_carlo() -> String {
    use Arch::{Healing, Hit, Output, Speed};
    let m = standard::reference();
    let ts = mc::all_thresholds();
    let mut rng = mc::Rng::new(0x5941_5441_7631);
    use attr::{
        ATK_FLAT, ATK_PCT, CRIT, CRIT_DMG, DEF_FLAT, DEF_PCT, EFF_HIT, EFF_RES, HP_FLAT, HP_PCT,
        SPD,
    };
    // The catalogue's distinct acceptance sets, each with every legal slot and main that has it.
    let groups: [Group; 6] = [
        (
            "slots 1, 3, 5; Spd on 2",
            vec![(1, ATK_FLAT), (3, DEF_FLAT), (5, HP_FLAT), (2, SPD)],
            ARCHES.to_vec(),
        ),
        (
            "AtkPercent on 2, 4, 6",
            vec![(2, ATK_PCT), (4, ATK_PCT), (6, ATK_PCT)],
            vec![Output],
        ),
        (
            "Crit, CritDmg on 6",
            vec![(6, CRIT), (6, CRIT_DMG)],
            vec![Output, Healing],
        ),
        (
            "HpPercent on 2, 4, 6",
            vec![(2, HP_PCT), (4, HP_PCT), (6, HP_PCT)],
            vec![Hit, Healing, Speed],
        ),
        (
            "EffectHit on 4; DefPercent on 2, 4, 6",
            vec![(4, EFF_HIT), (2, DEF_PCT), (4, DEF_PCT), (6, DEF_PCT)],
            vec![Hit, Speed],
        ),
        ("EffectRes on 4", vec![(4, EFF_RES)], vec![Speed]),
    ];
    for (name, members, acc) in &groups {
        for (slot, main) in members {
            let expect: Vec<Arch> = ARCHES
                .iter()
                .copied()
                .filter(|a| a.accepts(*slot, *main))
                .collect();
            assert_eq!(&expect, acc, "{name}: the group differs from the catalogue");
        }
    }
    // Every legal main of every slot (glossary `Main(k)`) is in exactly one group.
    let legal: [(u8, &[usize]); 6] = [
        (1, &[ATK_FLAT]),
        (2, &[SPD, ATK_PCT, DEF_PCT, HP_PCT]),
        (3, &[DEF_FLAT]),
        (4, &[EFF_HIT, EFF_RES, ATK_PCT, DEF_PCT, HP_PCT]),
        (5, &[HP_FLAT]),
        (6, &[CRIT, CRIT_DMG, ATK_PCT, DEF_PCT, HP_PCT]),
    ];
    for (slot, mains) in legal {
        for &main in mains {
            let n = groups
                .iter()
                .filter(|(_, ms, _)| ms.contains(&(slot, main)))
                .count();
            assert_eq!(n, 1, "slot {slot} main {}", model::NAMES[main]);
        }
    }
    let mut per_arch = [[0usize; 7]; 4];
    let mut per_group = vec![[0usize; 7]; groups.len()];
    for _ in 0..MC_N {
        let e = mc::draw(&m, &mut rng);
        for (i, a) in ARCHES.iter().enumerate() {
            let t = mc::arch_tier(&ts, &e, *a).map_or(mc::UNRATED, |x| x.0);
            per_arch[i][t] += 1;
        }
        for (i, (_, _, acc)) in groups.iter().enumerate() {
            per_group[i][mc::quality_tier(&ts, &e, acc).0] += 1;
        }
    }
    let mut s = format!(
        "{MC_N} souls per row, seed 0x594154417631, uniform increments, the reference measure. Each cell: rate (standard error), and its z-score against the exact rate. For speed, \"not eligible\" is e_Spd < 4.\n\n"
    );
    s += "| Row | N | R | SR | SSR | SP | UR | not eligible |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n";
    let cell = |c: usize| {
        let p = c as f64 / MC_N as f64;
        format!("{:.4}% ({:.4}%)", 100.0 * p, 100.0 * se(p))
    };
    let zero = Q::from_integer(0.into());
    let mut worst = 0.0f64;
    for (i, a) in ARCHES.iter().enumerate() {
        let ex: [Q; 7] = if *a == Speed {
            let r = rates::speed_rates(&m, Law::Uniform);
            let not = Q::from_integer(1.into()) - &r.eligible;
            let z = || zero.clone();
            [z(), z(), z(), r.ssr, z(), r.ur, not]
        } else {
            let r = rates::tier_rates(&m, Law::Uniform, *a);
            [r.n, r.r, r.sr, r.ssr, r.sp, r.ur, zero.clone()]
        };
        let _ = write!(s, "| exact, {} |", a.key());
        for x in &ex {
            let _ = write!(s, " {} |", pct(x));
        }
        let _ = write!(s, "\n| Monte Carlo, {} |", a.key());
        for (c, x) in per_arch[i].iter().zip(&ex) {
            if x == &zero {
                assert_eq!(*c, 0, "{}: a tier its rule excludes occurred", a.key());
                let _ = write!(s, " {} |", cell(*c));
                continue;
            }
            let p = *c as f64 / MC_N as f64;
            let z = (p - f(x)) / se(f(x));
            worst = worst.max(z.abs());
            let _ = write!(s, " {} z={z:+.2} |", cell(*c));
        }
        s += "\n";
    }
    let _ = writeln!(
        s,
        "\nLargest |z| of a Monte Carlo rate against the exact rate: {worst:.2}.\n"
    );
    s += "| Quality over the candidate archetypes | Accepting | N | R | SR | SSR | SP | UR | unrated |\n";
    s += "| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n";
    for (i, (name, _, acc)) in groups.iter().enumerate() {
        let keys: Vec<&str> = acc.iter().map(|a| a.key()).collect();
        let _ = write!(s, "| {name} | {} |", keys.join(", "));
        for c in per_group[i] {
            let _ = write!(s, " {} |", cell(c));
        }
        s += "\n";
    }
    s
}

pub fn normalization() -> String {
    let m = standard::reference();
    let t = mc::f64_thresholds(Arch::Output);
    let profiles: [(&str, Vec<usize>); 3] = [
        ("speed only (1 attribute)", vec![attr::SPD]),
        ("crit pair (2 attributes)", vec![attr::CRIT, attr::CRIT_DMG]),
        (
            "output archetype (4 attributes)",
            Arch::Output.attrs().to_vec(),
        ),
    ];
    let mut rng = mc::Rng::new(0x4E4F_524D);
    let souls: Vec<[f64; model::ATTRS]> = (0..NORM_N).map(|_| mc::draw(&m, &mut rng)).collect();
    let cuts = [t.c_r, t.c_sr, t.c_ssr, t.t_ur];
    let mut s = format!(
        "{NORM_N} souls, seed 0x4E4F524D, uniform increments; the same souls for every profile. Each cell: share of souls scoring at or above the output archetype's c_R / c_SR / c_SSR / above its t_UR, and the mean score.\n\n| Normalization | Profile | ≥ c_R | ≥ c_SR | ≥ c_SSR | > t_UR | mean |\n| --- | --- | --- | --- | --- | --- | --- |\n"
    );
    for (norm, label) in [
        ("A", "A. bound: 100·U/M"),
        ("C", "C. mean ratio: min(100, 50·U/E)"),
        ("H", "H. anchored (the standard)"),
        ("B", "B. percentile (mid-rank)"),
    ] {
        for (pname, useful) in &profiles {
            let b = useful.len() as f64;
            let max = 5.0 + b;
            let e = f(&(mu() * standard::mean_useful(standard::reference_final(), useful)));
            let us: Vec<f64> = souls
                .iter()
                .map(|x| useful.iter().map(|&a| x[a]).sum())
                .collect();
            let scores: Vec<f64> = match norm {
                "A" => us.iter().map(|u| 100.0 * u / max).collect(),
                "C" => us.iter().map(|u| (50.0 * u / e).min(100.0)).collect(),
                "H" => {
                    let tt = mc::F64Thresholds {
                        e,
                        m: max,
                        ..mc::f64_thresholds(Arch::Output)
                    };
                    us.iter().map(|&u| mc::g(&tt, u)).collect()
                }
                _ => {
                    let mut sorted = us.clone();
                    sorted.sort_by(f64::total_cmp);
                    us.iter()
                        .map(|u| {
                            let lo = sorted.partition_point(|x| x < u);
                            let hi = sorted.partition_point(|x| x <= u);
                            100.0 * (lo as f64 + (hi - lo) as f64 / 2.0) / NORM_N as f64
                        })
                        .collect()
                }
            };
            let share = |c: f64, strict: bool| {
                scores
                    .iter()
                    .filter(|&&x| if strict { x > c } else { x >= c })
                    .count() as f64
                    / NORM_N as f64
            };
            let mean = scores.iter().sum::<f64>() / NORM_N as f64;
            let _ = writeln!(
                s,
                "| {label} | {pname} | {:.3}% | {:.3}% | {:.3}% | {:.4}% | {:.2} |",
                100.0 * share(cuts[0], false),
                100.0 * share(cuts[1], false),
                100.0 * share(cuts[2], false),
                100.0 * share(cuts[3], true),
                mean
            );
        }
    }
    s
}
