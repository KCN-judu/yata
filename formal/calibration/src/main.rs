//! Reproduces every number of the Yata Quality Model v1 (`docs/spec/quality-model.md`) and of its
//! paper (`papers/quality-model-v1/paper.md`).
//!
//! `render` writes `out/` and rewrites every `<!-- generated:NAME -->` block in the documents
//! named after it; `check` recomputes the same and fails if anything committed differs. Exact
//! rational arithmetic wherever the model allows it; the rest is marked where it is printed.

mod law;
mod mc;
mod model;
mod rates;
mod standard;
mod vectors;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use law::{Law, f};
use model::{Measure, Q, attr, q};
use standard::{ARCHES, Arch, anchors, mu, thresholds};

fn pct(x: &Q) -> String {
    format!("{:.4}%", 100.0 * f(x))
}

fn frac(x: &Q) -> String {
    if x.denom() == &num_bigint::BigInt::from(1) {
        x.numer().to_string()
    } else {
        format!("{}/{}", x.numer(), x.denom())
    }
}

fn dp(x: &Q, n: usize) -> String {
    format!("{:.*}", n, f(x))
}

fn constants() -> String {
    let mut s = format!(
        "Parameter set `{}`. Reference measure: the official class weights, equal within a class. Exact values: `formal/calibration/out/constants.json`.\n\n",
        standard::MODEL_ID
    );
    // Check of the propagation: under Hu's 1/11 reading every attribute is alike, so four of them
    // expect 8 · 4/11 = 32/11 increments exactly.
    let hu = model::final_hits(&Measure::hu_reading());
    assert_eq!(standard::mean_useful(&hu, &Arch::Output.attrs()), q(32, 11));
    let t0 = thresholds(Arch::Output);
    assert!(
        t0.u_sr < t0.u_ssr && t0.u_ssr < t0.u_sp && t0.u_sp < t0.u_ur,
        "the milestones must be ordered: SR < SSR < SP floor < UR"
    );
    assert_eq!(t0.u_ssr, q(6, 1), "SSR begins at one perfect line");
    s += "| Milestone | Roll units | Meaning |\n| --- | --- | --- |\n";
    let row = |s: &mut String, n: &str, x: &Q, m: &str| {
        let _ = writeln!(s, "| {n} | {} | {m} |", frac(x));
    };
    row(&mut s, "μ", &mu(), "mean increment, roll units");
    row(
        &mut s,
        "M",
        &standard::max_u(),
        "attainable maximum, every archetype",
    );
    row(
        &mut s,
        "u_SR",
        &t0.u_sr,
        "SR: five useful increments at mean value, 5μ",
    );
    row(
        &mut s,
        "u_SSR",
        &t0.u_ssr,
        "SSR: the most one useful line can hold, six increments at maximum",
    );
    row(
        &mut s,
        "u_SP_floor",
        &t0.u_sp,
        "SP's quality floor: seven useful increments at mean value, 7μ",
    );
    row(&mut s, "u_UR", &t0.u_ur, "UR: above M − 1");
    s += "\n| Archetype | E[K] | E = μ · E[K] | c_R | c_SR | c_SSR = g(6) | c_SP | t_UR |\n";
    s += "| --- | --- | --- | --- | --- | --- | --- | --- |\n";
    for a in ARCHES {
        let an = anchors(a);
        let t = thresholds(a);
        let k = standard::mean_useful(standard::reference_final(), &a.attrs());
        assert!(
            an.g(&q(6, 1)) < t.c_sp,
            "one line alone must stay below the SP floor"
        );
        assert_eq!(t.c_ssr, an.g(&q(6, 1)));
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            a.key(),
            dp(&k, 9),
            dp(&an.e, 9),
            frac(&t.c_r),
            dp(&t.c_sr, 6),
            dp(&t.c_ssr, 6),
            dp(&t.c_sp, 6),
            dp(&t.t_ur, 6),
        );
    }
    s
}

/// Every v1 constant as an exact fraction, for the implementation.
fn constants_json() -> String {
    let mut s = format!(
        "{{\n  \"model\": \"{}\",\n  \"mu\": \"{}\",\n  \"M\": \"{}\",\n  \"u_SR\": \"{}\",\n  \"u_SSR\": \"{}\",\n  \"u_SP_floor\": \"{}\",\n  \"u_UR\": \"{}\",\n  \"archetypes\": {{\n",
        standard::MODEL_ID,
        frac(&mu()),
        frac(&standard::max_u()),
        frac(&standard::u_sr()),
        frac(&standard::u_ssr()),
        frac(&standard::u_sp_floor()),
        frac(&standard::u_ur())
    );
    for (i, a) in ARCHES.iter().enumerate() {
        let an = anchors(*a);
        let t = thresholds(*a);
        let _ = writeln!(
            s,
            "    \"{}\": {{\"E\": \"{}\", \"c_R\": \"{}\", \"c_SR\": \"{}\", \"c_SSR\": \"{}\", \"c_SP\": \"{}\", \"t_UR\": \"{}\"}}{}",
            a.key(),
            frac(&an.e),
            frac(&t.c_r),
            frac(&t.c_sr),
            frac(&t.c_ssr),
            frac(&t.c_sp),
            frac(&t.t_ur),
            if i + 1 < ARCHES.len() { "," } else { "" }
        );
    }
    s += "  }\n}\n";
    s
}

fn k_law() -> String {
    let d = standard::reference_final();
    let laws: Vec<_> = [Arch::Output, Arch::Hit]
        .iter()
        .map(|a| model::k_law(&model::useful_law(d, &a.attrs())))
        .collect();
    let mut s = String::from("| K | P(K), output | P(K), hit and resist |\n| --- | --- | --- |\n");
    for k in 0..=9u32 {
        let cell = |l: &BTreeMap<u32, Q>| l.get(&k).map_or("0".to_owned(), pct);
        let _ = writeln!(s, "| {k} | {} | {} |", cell(&laws[0]), cell(&laws[1]));
    }
    let _ = writeln!(
        s,
        "| E[K] | {} | {} |",
        dp(&model::mean_k(&laws[0]), 9),
        dp(&model::mean_k(&laws[1]), 9)
    );
    s
}

fn tier_rates() -> String {
    let m = standard::reference();
    let mut s = String::from(
        "| Increment law | Archetype | N | R | SR | SSR | SP | UR |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for l in [Law::Uniform, Law::Steps(7), Law::Steps(2)] {
        let hit = rates::tier_rates(&m, l, Arch::Hit);
        let resist = rates::tier_rates(&m, l, Arch::Resist);
        assert!(
            hit.n == resist.n && hit.sp == resist.sp && hit.ur == resist.ur && hit.e == resist.e,
            "hit and resist must agree: EffectHit and EffectRes share a class"
        );
        for (name, r) in [
            ("output", rates::tier_rates(&m, l, Arch::Output)),
            ("hit, resist", hit),
        ] {
            let _ = writeln!(
                s,
                "| {} | {name} | {} | {} | {} | {} | {} | {} |",
                l.name(),
                pct(&r.n),
                pct(&r.r),
                pct(&r.sr),
                pct(&r.ssr),
                pct(&r.sp),
                pct(&r.ur)
            );
        }
    }
    s
}

fn measure_sensitivity() -> String {
    let mut s = String::from(
        "| Measure | Archetype | E | N | R | SR | SSR | SP | UR |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    let skewed = Measure::official().with_initial(
        [q(1, 2), q(1, 3), q(1, 6)],
        "official weights; initial count 1/2, 1/3, 1/6",
    );
    for m in [Measure::official(), Measure::hu_reading(), skewed] {
        for a in ARCHES {
            let r = rates::tier_rates(&m, Law::Uniform, a);
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                m.name,
                a.key(),
                dp(&r.e, 4),
                pct(&r.n),
                pct(&r.r),
                pct(&r.sr),
                pct(&r.ssr),
                pct(&r.sp),
                pct(&r.ur)
            );
        }
    }
    s
}

fn mu_sensitivity(vs: &[standard::SoulInput]) -> String {
    let picks = ["V03", "V05", "V11", "V12"];
    let mut s = String::from("| μ | E, output | E, hit and resist |");
    for p in picks {
        let _ = write!(s, " score {p} |");
    }
    s += "\n| --- | --- | --- | --- | --- | --- | --- |\n";
    let d = standard::reference_final();
    for (m, label) in [
        (q(8, 10), "0.8 (all rolls minimal)"),
        (q(9, 10), "0.9 (v1: midpoint)"),
        (q(2720, 3000), "0.9067 (官网 2.72/3.0)"),
        (q(365, 400), "0.9125 (官网 3.65/4.0)"),
        (q(1, 1), "1.0 (all rolls maximal)"),
    ] {
        let anchors_at = |a: Arch| standard::Anchors {
            e: &m * standard::mean_useful(d, &a.attrs()),
            m: standard::max_u(),
        };
        let _ = write!(
            s,
            "| {label} | {} | {} |",
            dp(&anchors_at(Arch::Output).e, 4),
            dp(&anchors_at(Arch::Hit).e, 4)
        );
        for p in picks {
            let v = vs.iter().find(|v| v.id == p).expect("vector");
            let best = standard::quality(v).remove(0);
            let _ = write!(s, " {} |", dp(&anchors_at(best.arch).g(&best.utility), 2));
        }
        s += "\n";
    }
    s
}

/// The percentile of a utility under the output archetype and a law: P(U < u) + ½ P(U = u), exact.
fn percentile(law: Law, u: &Q) -> Q {
    let ul = model::useful_law(standard::reference_final(), &Arch::Output.attrs());
    let mut below = Q::from_integer(0.into());
    let mut at = Q::from_integer(0.into());
    for (v, p) in &ul {
        let k: u32 = v.iter().map(|&x| u32::from(x)).sum();
        let ge = law::tail(law, k, u);
        let gt = match law {
            Law::Uniform => ge.clone(),
            Law::Steps(n) => law::steps_sum(n, k)
                .iter()
                .filter(|(x, _)| x > u)
                .map(|(_, p)| p.clone())
                .sum(),
        };
        below += p * (Q::from_integer(1.into()) - &ge);
        at += p * (&ge - &gt);
    }
    below + at / q(2, 1)
}

/// P(U ≥ u) for the output archetype under the reference measure and uniform increments, exact:
/// how likely a +15 soul is to be at least this good.
fn equal_or_better(u: &Q) -> String {
    let ul = model::useful_law(standard::reference_final(), &Arch::Output.attrs());
    let p: Q = ul
        .iter()
        .map(|(v, p)| {
            let k: u32 = v.iter().map(|&x| u32::from(x)).sum();
            p * law::tail(Law::Uniform, k, u)
        })
        .sum();
    format!("{} (1 in {:.0})", pct(&p), 1.0 / f(&p))
}

fn law_robustness(vs: &[standard::SoulInput]) -> String {
    let mut s = String::from(
        "| Soul | U | anchored score (any law with mean 0.9) | percentile, uniform | percentile, 7 steps | percentile, 2 steps | P(equal or better), uniform |\n| --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for id in ["V02", "V03", "V05", "V10"] {
        let v = vs.iter().find(|v| v.id == id).expect("vector");
        let b = standard::score_arch(v, Arch::Output);
        let _ = writeln!(
            s,
            "| {id} | {} | {} | {} | {} | {} | {} |",
            dp(&b.utility, 4),
            dp(&b.score, 2),
            dp(&(percentile(Law::Uniform, &b.utility) * q(100, 1)), 2),
            dp(&(percentile(Law::Steps(7), &b.utility) * q(100, 1)), 2),
            dp(&(percentile(Law::Steps(2), &b.utility) * q(100, 1)), 2),
            equal_or_better(&b.utility),
        );
    }
    s
}

const MC_N: usize = 4_000_000;
const NORM_N: usize = 1_000_000;

fn se(p: f64) -> f64 {
    (p * (1.0 - p) / MC_N as f64).sqrt()
}

fn monte_carlo() -> String {
    let m = standard::reference();
    let ts = mc::all_thresholds();
    let mut rng = mc::Rng::new(0x5941_5441_7631);
    let groups: [(&str, Vec<Arch>); 6] = [
        ("slot 1, 3, 5 (all archetypes)", mc::all_arches()),
        ("slot 2 Spd (all archetypes)", mc::all_arches()),
        ("slot 2 AtkPercent (output)", vec![Arch::Output]),
        ("slot 4 EffectHit (hit)", vec![Arch::Hit]),
        ("slot 6 CritDmg (output)", vec![Arch::Output]),
        (
            "slot 6 HpPercent (hit, resist)",
            vec![Arch::Hit, Arch::Resist],
        ),
    ];
    let mut per_arch = [[0usize; 6]; 3];
    let mut per_group = vec![[0usize; 6]; groups.len()];
    for _ in 0..MC_N {
        let e = mc::draw(&m, &mut rng);
        for (i, a) in ARCHES.iter().enumerate() {
            per_arch[i][mc::arch_tier(&ts, &e, *a).0] += 1;
        }
        for (i, (_, acc)) in groups.iter().enumerate() {
            per_group[i][mc::quality_tier(&ts, &e, acc).0] += 1;
        }
    }
    let mut s = format!(
        "{MC_N} souls per row, seed 0x594154417631, uniform increments, the reference measure. Each cell: rate (standard error), and its z-score against the exact rate.\n\n"
    );
    s += "| Row | N | R | SR | SSR | SP | UR |\n| --- | --- | --- | --- | --- | --- | --- |\n";
    let cell = |c: usize| {
        let p = c as f64 / MC_N as f64;
        format!("{:.4}% ({:.4}%)", 100.0 * p, 100.0 * se(p))
    };
    let mut worst = 0.0f64;
    for (i, a) in ARCHES.iter().enumerate() {
        let exact = rates::tier_rates(&m, Law::Uniform, *a);
        let ex = [
            &exact.n, &exact.r, &exact.sr, &exact.ssr, &exact.sp, &exact.ur,
        ];
        let _ = write!(s, "| exact, {} |", a.key());
        for x in ex {
            let _ = write!(s, " {} |", pct(x));
        }
        let _ = write!(s, "\n| Monte Carlo, {} |", a.key());
        for (c, x) in per_arch[i].iter().zip(ex) {
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
    s += "| Quality over accepted archetypes | N | R | SR | SSR | SP | UR |\n";
    s += "| --- | --- | --- | --- | --- | --- | --- |\n";
    for (i, (name, _)) in groups.iter().enumerate() {
        let _ = write!(s, "| {name} |");
        for c in per_group[i] {
            let _ = write!(s, " {} |", cell(c));
        }
        s += "\n";
    }
    s
}

fn normalization() -> String {
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
        ("H", "H. anchored (v1)"),
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

/// The utility at which "SSR or better" is `share` of +15 souls under the uniform law, for
/// `output`: a distribution-calibrated alternative, found by bisection on the exact tail.
fn tail_quantile(share: &Q) -> Q {
    let ul = model::useful_law(standard::reference_final(), &Arch::Output.attrs());
    let ge = |u: &Q| -> Q {
        ul.iter()
            .map(|(v, p)| {
                let k: u32 = v.iter().map(|&x| u32::from(x)).sum();
                p * law::tail(Law::Uniform, k, u)
            })
            .sum()
    };
    let (mut lo, mut hi) = (standard::u_sr(), standard::u_sp_floor());
    for _ in 0..30 {
        let mid = (&lo + &hi) / q(2, 1);
        if ge(&mid) > *share {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    // Rounded to 1/1000 of a roll unit, so the value can be written down.
    (hi * q(1000, 1)).round() / q(1000, 1)
}

/// Tier rates under four SSR milestones, every increment law and archetype group.
fn ssr_candidates() -> String {
    let m = standard::reference();
    let d = tail_quantile(&q(5, 100));
    let cands: [(&str, Q); 4] = [
        ("A. v1: 7μ", standard::u_ssr_v1()),
        ("B. v1.1: one perfect line", standard::u_ssr()),
        ("C. 6μ", mu() * q(6, 1)),
        ("D. SSR or better at 5% (uniform, output)", d),
    ];
    let mut s = String::from(
        "SP's floor stays 7μ = 6.3 and UR's boundary 8 in every row; only SSR's edge moves.\n\n| Candidate | u_SSR | Increment law | Archetype | N | R | SR | SSR | SP | UR | SSR or better |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for (name, u) in &cands {
        for l in [Law::Uniform, Law::Steps(7), Law::Steps(2)] {
            for (label, a) in [("output", Arch::Output), ("hit, resist", Arch::Hit)] {
                let t = standard::thresholds_with(a, u.clone());
                let r = rates::tier_rates_with(&m, l, a, &t);
                if label != "output" {
                    let o = rates::tier_rates_with(
                        &m,
                        l,
                        Arch::Resist,
                        &standard::thresholds_with(Arch::Resist, u.clone()),
                    );
                    assert!(o.ssr == r.ssr && o.sp == r.sp, "hit and resist must agree");
                }
                let top = &r.ssr + &r.sp + &r.ur;
                let _ = writeln!(
                    s,
                    "| {name} | {} | {} | {label} | {} | {} | {} | {} | {} | {} | {} |",
                    dp(u, 3),
                    l.name(),
                    pct(&r.n),
                    pct(&r.r),
                    pct(&r.sr),
                    pct(&r.ssr),
                    pct(&r.sp),
                    pct(&r.ur),
                    pct(&top)
                );
            }
        }
    }
    s
}

/// The Lean files name the same milestones; a divergence fails `render` and `check` alike.
fn lean_milestones_agree(root: &Path) {
    let path = root.join("../lean/YataFormal/Quality.lean");
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let value = |name: &str| -> Option<Q> {
        let line = text
            .lines()
            .find(|l| l.starts_with(&format!("def {name} : ℚ := ")))?;
        let expr = line.split(":= ").nth(1)?.trim();
        let (n, d) = expr.split_once('/').unwrap_or((expr, "1"));
        Some(q(n.trim().parse().ok()?, d.trim().parse().ok()?))
    };
    for (name, ours) in [
        ("uSR", standard::u_sr()),
        ("uSSR", standard::u_ssr()),
        ("uSPFloor", standard::u_sp_floor()),
        ("uUR", standard::u_ur()),
    ] {
        assert_eq!(
            value(name).as_ref(),
            Some(&ours),
            "{name} in {} differs from the calibration",
            path.display()
        );
    }
}

fn vectors_table(vs: &[standard::SoulInput]) -> (String, String) {
    let mut md = String::from(
        "| Id | Soul | Slot, main, level | Archetype | U | Score | Specialized | Tier | Tier in v1 | Depth | Breadth | Growth |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    let mut json = String::from("[\n");
    for (i, v) in vs.iter().enumerate() {
        v.check()
            .unwrap_or_else(|e| panic!("{} is not a legal soul: {e}", v.id));
        let all = standard::quality(v);
        let best = &all[0];
        let lines: Vec<String> = v
            .lines
            .iter()
            .map(|l| format!("{} {} ({})", model::NAMES[l.attr], l.value, l.hits))
            .collect();
        let growth = standard::growth(v, best.arch);
        if let Some(g) = &growth {
            // The spec's recursion must agree with exact propagation.
            let present = v.lines.iter().fold(0u16, |m, l| m | (1 << l.attr));
            let r = 5 - u32::from(v.level / 3);
            let by_recursion = anchors(best.arch).g(&(&best.utility
                + mu() * standard::growth_recursion(r, present, &best.arch.attrs())));
            assert_eq!(&by_recursion, g, "{}: growth recursion disagrees", v.id);
        }
        let _ = writeln!(
            md,
            "| {} | {} | {}, {}, +{} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            v.id,
            lines.join("; "),
            v.slot,
            model::NAMES[v.main],
            v.level,
            best.arch.key(),
            dp(&best.utility, 4),
            dp(&best.score, 2),
            if best.specialized { "yes" } else { "no" },
            best.tier.name(),
            standard::quality_with(v, &standard::u_ssr_v1())[0]
                .tier
                .name(),
            dp(&best.depth, 1),
            dp(&best.breadth, 1),
            growth.as_ref().map_or("—".to_owned(), |g| dp(g, 2)),
        );
        let others: Vec<String> = all
            .iter()
            .map(|a| {
                format!(
                    "{{\"archetype\": \"{}\", \"utility\": \"{}\", \"score\": \"{}\", \"specialized\": {}, \"tier\": \"{}\"}}",
                    a.arch.key(),
                    frac(&a.utility),
                    frac(&a.score),
                    a.specialized,
                    a.tier.name()
                )
            })
            .collect();
        let lines_json: Vec<String> = v
            .lines
            .iter()
            .map(|l| {
                format!(
                    "{{\"attribute\": \"{}\", \"value\": \"{}\", \"increments\": {}}}",
                    model::NAMES[l.attr],
                    l.value,
                    l.hits
                )
            })
            .collect();
        let _ = writeln!(
            json,
            "  {{\"id\": \"{}\", \"what\": \"{}\", \"slot\": {}, \"main\": \"{}\", \"level\": {}, \"lines\": [{}], \"best\": \"{}\", \"growth\": {}, \"archetypes\": [{}]}}{}",
            v.id,
            v.what,
            v.slot,
            model::NAMES[v.main],
            v.level,
            lines_json.join(", "),
            best.arch.key(),
            growth
                .as_ref()
                .map_or("null".to_owned(), |g| format!("\"{}\"", frac(g))),
            others.join(", "),
            if i + 1 < vs.len() { "," } else { "" }
        );
    }
    json += "]\n";
    // The regressions the v1.1 recalibration exists for, and what it must not change.
    let tier_of = |id: &str| {
        let v = vs.iter().find(|v| v.id == id).expect("vector");
        let b = standard::quality(v).remove(0);
        (b.tier, b.specialized, b.score)
    };
    use standard::Tier;
    assert_eq!(
        tier_of("V04").0,
        Tier::Ssr,
        "one perfect line is SSR, not SP"
    );
    assert!(tier_of("V04").1, "V04 is specialized");
    assert_eq!(
        tier_of("V05").0,
        Tier::Sp,
        "specialized and above the SP floor is SP"
    );
    assert_ne!(
        tier_of("V06").0,
        Tier::Sp,
        "five roll units exactly is not specialized"
    );
    assert_ne!(
        tier_of("V07").0,
        Tier::Sp,
        "specialization alone does not reach SP"
    );
    assert_eq!(tier_of("V08").0, Tier::Ur);
    assert_eq!(tier_of("V09").0, Tier::Ur);
    assert!(tier_of("V15").2 > tier_of("V14").2 && tier_of("V15").0 >= tier_of("V14").0);
    let mut why = String::from("\n| Id | What it shows |\n| --- | --- |\n");
    for v in vs {
        let _ = writeln!(why, "| {} | {} |", v.id, v.what);
    }
    (md + &why, json)
}

fn blocks() -> BTreeMap<&'static str, String> {
    let vs = vectors::all();
    let (vmd, _) = vectors_table(&vs);
    BTreeMap::from([
        ("constants", constants()),
        ("k-law", k_law()),
        ("tier-rates", tier_rates()),
        ("measure-sensitivity", measure_sensitivity()),
        ("mu-sensitivity", mu_sensitivity(&vs)),
        ("law-robustness", law_robustness(&vs)),
        ("monte-carlo", monte_carlo()),
        ("normalization", normalization()),
        ("vectors", vmd),
        ("ssr-candidates", ssr_candidates()),
    ])
}

/// Replace every generated block of `text` with the current content.
fn fill(text: &str, blocks: &BTreeMap<&str, String>) -> String {
    let mut out = text.to_owned();
    for (name, body) in blocks {
        let open = format!("<!-- generated:{name} -->");
        let close = format!("<!-- /generated:{name} -->");
        if let (Some(a), Some(b)) = (out.find(&open), out.find(&close)) {
            let start = a + open.len();
            out = format!("{}\n\n{}\n{}", &out[..start], body.trim_end(), &out[b..]);
        }
    }
    out
}

/// Content as compared: Prettier re-pads tables and re-wraps prose in committed Markdown, so
/// whitespace and the length of a table's dash rule are not content. Every number and word is.
fn normalize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last_dash = false;
    for c in text.chars().filter(|c| !c.is_whitespace()) {
        if c == '-' && last_dash {
            continue;
        }
        last_dash = c == '-';
        out.push(c);
    }
    out
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (mode, docs) = match args.split_first() {
        Some((m, rest)) if m == "render" || m == "check" => (m.clone(), rest.to_vec()),
        _ => {
            eprintln!("usage: yata-quality-calibration (render | check) [document.md …]");
            return ExitCode::from(2);
        }
    };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    lean_milestones_agree(&root);
    let b = blocks();
    let (_, json) = vectors_table(&vectors::all());
    let mut files: Vec<(PathBuf, String)> = b
        .iter()
        .map(|(n, body)| {
            let page = format!(
                "# {n}\n\nGenerated by `formal/calibration` (`render`); do not edit.\n\n{body}"
            );
            (root.join("out").join(format!("{n}.md")), page)
        })
        .collect();
    files.push((root.join("out").join("test-vectors.json"), json));
    files.push((root.join("out").join("constants.json"), constants_json()));
    for d in &docs {
        let p = Path::new(d).to_path_buf();
        let text = std::fs::read_to_string(&p).unwrap_or_default();
        files.push((p, fill(&text, &b)));
    }
    let mut stale = Vec::new();
    for (p, content) in &files {
        let current = std::fs::read_to_string(p).unwrap_or_default();
        if normalize(&current) != normalize(content) {
            if mode == "render" {
                if let Some(dir) = p.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                if let Err(e) = std::fs::write(p, content) {
                    eprintln!("{}: {e}", p.display());
                    return ExitCode::FAILURE;
                }
            }
            stale.push(p.display().to_string());
        }
    }
    if mode == "check" && !stale.is_empty() {
        eprintln!(
            "stale generated content; run `render`:\n  {}",
            stale.join("\n  ")
        );
        return ExitCode::FAILURE;
    }
    println!(
        "{mode}: {} file(s) {}",
        stale.len(),
        if mode == "render" { "written" } else { "stale" }
    );
    ExitCode::SUCCESS
}
