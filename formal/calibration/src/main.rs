//! Reproduces every number of the Yata Quality Model v2 (`docs/spec/quality-model.md`) and of its
//! paper (`papers/quality-model-v2/paper.md`).
//!
//! `render` writes `out/` and rewrites every `<!-- generated:NAME -->` block in the documents
//! named after it; `check` recomputes the same and fails if anything committed differs. Exact
//! rational arithmetic wherever the model allows it; the rest is marked where it is printed.

mod golden;
mod law;
mod mc;
mod model;
mod rates;
mod simulate;
mod standard;
mod vectors;
mod zh;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use law::{Law, f};
use model::{Measure, Q, q};
use standard::{Arch, BROAD, anchors, mu, thresholds};

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
    // expect 8 · 4/11 = 32/11 increments exactly, and one of them 8/11.
    let hu = model::final_hits(&Measure::hu_reading());
    assert_eq!(standard::mean_useful(hu, Arch::Output.attrs()), q(32, 11));
    assert_eq!(standard::mean_useful(hu, Arch::Speed.attrs()), q(8, 11));
    // output and healing differ by AtkPercent ↔ HpPercent, both 9%: the same scale exactly.
    assert!(
        anchors(Arch::Output).e == anchors(Arch::Healing).e,
        "output and healing must share E: AtkPercent and HpPercent have the same weight"
    );
    let t0 = thresholds(Arch::Output);
    assert!(
        t0.u_sr < t0.u_ssr && t0.u_ssr < t0.u_sp && t0.u_sp < t0.u_ur,
        "the milestones must be ordered: SR < SSR < SP floor < UR"
    );
    assert_eq!(t0.u_ssr, q(6, 1), "SSR begins at one perfect line");
    assert!(
        standard::speed_gate() < standard::u_speed_ur()
            && standard::u_speed_ur() < standard::max_u(Arch::Speed),
        "speed's gate, UR boundary and maximum must be ordered"
    );
    s += "| Milestone | Roll units | Meaning |\n| --- | --- | --- |\n";
    let row = |s: &mut String, n: &str, x: &Q, m: &str| {
        let _ = writeln!(s, "| {n} | {} | {m} |", frac(x));
    };
    row(&mut s, "μ", &mu(), "mean increment, roll units");
    row(
        &mut s,
        "M",
        &standard::max_u(Arch::Output),
        "attainable maximum of a four-attribute archetype",
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
    row(
        &mut s,
        "M_speed",
        &standard::max_u(Arch::Speed),
        "attainable maximum of speed: one line, six increments at maximum",
    );
    row(
        &mut s,
        "u_gate",
        &standard::speed_gate(),
        "speed is a candidate only when e_Spd ≥ 4",
    );
    row(
        &mut s,
        "u_UR,speed",
        &standard::u_speed_ur(),
        "speed's UR: above M_speed − 1",
    );
    s += "\n| Archetype | E[K] | E = μ · E[K] | M | c_R | c_SR | c_SSR = g(6) | c_SP | t_UR |\n";
    s += "| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n";
    for a in BROAD {
        let an = anchors(a);
        let t = thresholds(a);
        let k = standard::mean_useful(standard::reference_final(), a.attrs());
        assert!(
            an.g(&q(6, 1)) < t.c_sp,
            "one line alone must stay below the SP floor"
        );
        assert_eq!(t.c_ssr, an.g(&q(6, 1)));
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            a.key(),
            dp(&k, 9),
            dp(&an.e, 9),
            frac(&an.m),
            frac(&t.c_r),
            dp(&t.c_sr, 6),
            dp(&t.c_ssr, 6),
            dp(&t.c_sp, 6),
            dp(&t.t_ur, 6),
        );
    }
    let sp = anchors(Arch::Speed);
    let k = standard::mean_useful(standard::reference_final(), Arch::Speed.attrs());
    s += "\n| Archetype | E[K] | E = μ · E[K] | M | score at the gate, g(4) | t_UR = g(5) |\n";
    s += "| --- | --- | --- | --- | --- | --- |\n";
    let _ = writeln!(
        s,
        "| speed | {} | {} | {} | {} | {} |",
        dp(&k, 9),
        dp(&sp.e, 9),
        frac(&sp.m),
        dp(&sp.g(&standard::speed_gate()), 6),
        dp(&sp.g(&standard::u_speed_ur()), 6),
    );
    s
}

/// Every v2 constant as an exact fraction, for the implementation.
fn constants_json() -> String {
    let mut s = format!(
        "{{\n  \"model\": \"{}\",\n  \"mu\": \"{}\",\n  \"u_SR\": \"{}\",\n  \"u_SSR\": \"{}\",\n  \"u_SP_floor\": \"{}\",\n  \"u_UR\": \"{}\",\n  \"speed_gate\": \"{}\",\n  \"u_UR_speed\": \"{}\",\n  \"archetypes\": {{\n",
        standard::MODEL_ID,
        frac(&mu()),
        frac(&standard::u_sr()),
        frac(&standard::u_ssr()),
        frac(&standard::u_sp_floor()),
        frac(&standard::u_ur()),
        frac(&standard::speed_gate()),
        frac(&standard::u_speed_ur()),
    );
    for a in BROAD {
        let an = anchors(a);
        let t = thresholds(a);
        let _ = writeln!(
            s,
            "    \"{}\": {{\"E\": \"{}\", \"M\": \"{}\", \"c_R\": \"{}\", \"c_SR\": \"{}\", \"c_SSR\": \"{}\", \"c_SP\": \"{}\", \"t_UR\": \"{}\"}},",
            a.key(),
            frac(&an.e),
            frac(&an.m),
            frac(&t.c_r),
            frac(&t.c_sr),
            frac(&t.c_ssr),
            frac(&t.c_sp),
            frac(&t.t_ur),
        );
    }
    let sp = anchors(Arch::Speed);
    let _ = writeln!(
        s,
        "    \"speed\": {{\"E\": \"{}\", \"M\": \"{}\", \"c_gate\": \"{}\", \"t_UR\": \"{}\"}}",
        frac(&sp.e),
        frac(&sp.m),
        frac(&sp.g(&standard::speed_gate())),
        frac(&sp.g(&standard::u_speed_ur())),
    );
    s += "  }\n}\n";
    s
}

fn k_law() -> String {
    let d = standard::reference_final();
    let laws: Vec<_> = [Arch::Output, Arch::Hit, Arch::Speed]
        .iter()
        .map(|a| model::k_law(&model::useful_law(d, a.attrs())))
        .collect();
    let mut s = String::from(
        "| K | P(K), output and healing | P(K), hit | P(K), speed |\n| --- | --- | --- | --- |\n",
    );
    for k in 0..=9u32 {
        let cell = |l: &BTreeMap<u32, Q>| l.get(&k).map_or("0".to_owned(), pct);
        let _ = writeln!(
            s,
            "| {k} | {} | {} | {} |",
            cell(&laws[0]),
            cell(&laws[1]),
            cell(&laws[2])
        );
    }
    let _ = writeln!(
        s,
        "| E[K] | {} | {} | {} |",
        dp(&model::mean_k(&laws[0]), 9),
        dp(&model::mean_k(&laws[1]), 9),
        dp(&model::mean_k(&laws[2]), 9)
    );
    s
}

fn tier_rates() -> String {
    let m = standard::reference();
    let mut s = String::from(
        "| Increment law | Archetype | N | R | SR | SSR | SP | UR |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for l in [Law::Uniform, Law::Steps(7), Law::Steps(2)] {
        let output = rates::tier_rates(&m, l, Arch::Output);
        let healing = rates::tier_rates(&m, l, Arch::Healing);
        assert!(
            output.n == healing.n
                && output.sr == healing.sr
                && output.sp == healing.sp
                && output.ur == healing.ur
                && output.e == healing.e,
            "output and healing must agree: AtkPercent and HpPercent share a weight"
        );
        for (name, r) in [
            ("output, healing", output),
            ("hit", rates::tier_rates(&m, l, Arch::Hit)),
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
    let measures = [Measure::official(), Measure::hu_reading(), skewed];
    for m in &measures {
        for a in BROAD {
            let r = rates::tier_rates(m, Law::Uniform, a);
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
    s += "\n| Measure | E, speed | P(eligible) | SSR | UR |\n| --- | --- | --- | --- | --- |\n";
    for m in &measures {
        let r = rates::speed_rates(m, Law::Uniform);
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} |",
            m.name,
            dp(&r.e, 4),
            pct(&r.eligible),
            pct(&r.ssr),
            pct(&r.ur)
        );
    }
    s
}

/// `speed`: how often a +15 soul is eligible, its tiers, and what the eligible souls look like.
fn speed_rates() -> String {
    let m = standard::reference();
    let a = anchors(Arch::Speed);
    let mut s = String::from(
        "Of all +15 souls, under the reference measure; speed has no N, R, SR or SP.\n\n| Increment law | P(eligible) | SSR | UR | eligible: 1 in |\n| --- | --- | --- | --- | --- |\n",
    );
    let all: Vec<_> = [Law::Uniform, Law::Steps(7), Law::Steps(2)]
        .iter()
        .map(|&l| (l, rates::speed_rates(&m, l)))
        .collect();
    for (l, r) in &all {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {:.0} |",
            l.name(),
            pct(&r.eligible),
            pct(&r.ssr),
            pct(&r.ur),
            1.0 / f(&r.eligible)
        );
    }
    s += "\nOf eligible souls: where e_Spd lies, the speed score and tier there, and how many increments the Speed line has.\n\n| Increment law | [4, 4.5) | [4.5, 5] | (5, 5.5] | (5.5, 6] | 4 increments | 5 increments | 6 increments |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n";
    for (l, r) in &all {
        let _ = write!(s, "| {} |", l.name());
        for b in r.bins.iter().chain(&r.by_hits) {
            let _ = write!(s, " {} |", pct(b));
        }
        s += "\n";
    }
    let edge = |x: Q| dp(&a.g(&x), 2);
    let _ = writeln!(
        s,
        "| speed score | {} to {} | {} to {} | {} to {} | {} to {} | | | |",
        edge(q(4, 1)),
        edge(q(9, 2)),
        edge(q(9, 2)),
        edge(q(5, 1)),
        edge(q(5, 1)),
        edge(q(11, 2)),
        edge(q(11, 2)),
        edge(q(6, 1)),
    );
    s += "| tier | SSR | SSR | UR | UR | | | |\n";
    // The gate is not a condition on the reference: compare the two references at the gate.
    let r = &all[0].1;
    let b = standard::Anchors {
        e: r.e_given_eligible.clone(),
        m: a.m.clone(),
    };
    s += "\nThe reference of speed's normalization, uniform increments: A, the standard, is unconditional; B would condition it on the gate.\n\n| Reference | E | score at e_Spd = 4 | 4.5 | 5 | 6 | eligible souls scored below 50 |\n| --- | --- | --- | --- | --- | --- | --- |\n";
    for (name, an, below) in [
        (
            "A. unconditional (the standard)",
            &a,
            pct(&Q::from_integer(0.into())),
        ),
        ("B. conditioned on e_Spd ≥ 4", &b, pct(&r.below_given)),
    ] {
        let _ = writeln!(
            s,
            "| {name} | {} | {} | {} | {} | {} | {below} |",
            dp(&an.e, 4),
            dp(&an.g(&q(4, 1)), 2),
            dp(&an.g(&q(9, 2)), 2),
            dp(&an.g(&q(5, 1)), 2),
            dp(&an.g(&q(6, 1)), 2),
        );
    }
    s
}

fn mu_sensitivity(vs: &[standard::SoulInput]) -> String {
    let picks = ["V03", "V05", "V11", "V22"];
    let mut s = String::from("| μ | E, output and healing | E, hit | E, speed |");
    for p in picks {
        let _ = write!(s, " score {p} |");
    }
    s += "\n| --- | --- | --- | --- | --- | --- | --- | --- |\n";
    let reference = standard::reference();
    let k = |a: Arch| model::mean_k(&model::k_law(model::useful_law_of(&reference, a.attrs())));
    for (m, label) in [
        (q(8, 10), "0.8 (all rolls minimal)"),
        (q(9, 10), "0.9 (the standard: midpoint)"),
        (q(2720, 3000), "0.9067 (官网 2.72/3.0)"),
        (q(365, 400), "0.9125 (官网 3.65/4.0)"),
        (q(1, 1), "1.0 (all rolls maximal)"),
    ] {
        let anchors_at = |a: Arch| standard::Anchors {
            e: &m * k(a),
            m: standard::max_u(a),
        };
        let _ = write!(
            s,
            "| {label} | {} | {} | {} |",
            dp(&anchors_at(Arch::Output).e, 4),
            dp(&anchors_at(Arch::Hit).e, 4),
            dp(&anchors_at(Arch::Speed).e, 4)
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
    let ul = model::useful_law(standard::reference_final(), Arch::Output.attrs());
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
    let ul = model::useful_law(standard::reference_final(), Arch::Output.attrs());
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

/// The utility at which "SSR or better" is `share` of +15 souls under the uniform law, for
/// `output`: a distribution-calibrated alternative, found by bisection on the exact tail.
fn tail_quantile(share: &Q) -> Q {
    let ul = model::useful_law(standard::reference_final(), Arch::Output.attrs());
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

/// Tier rates under four SSR milestones, every increment law and four-attribute archetype.
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
        "SP's floor stays 7μ = 6.3 and UR's boundary 8 in every row; only SSR's edge moves. The four-attribute archetypes of v2; speed has no SSR edge of this kind.\n\n| Candidate | u_SSR | Increment law | Archetype | N | R | SR | SSR | SP | UR | SSR or better |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for (name, u) in &cands {
        for l in [Law::Uniform, Law::Steps(7), Law::Steps(2)] {
            for (label, a) in [("output, healing", Arch::Output), ("hit", Arch::Hit)] {
                let t = standard::thresholds_with(a, u.clone());
                let r = rates::tier_rates_with(&m, l, a, &t);
                if a == Arch::Output {
                    let h = rates::tier_rates_with(
                        &m,
                        l,
                        Arch::Healing,
                        &standard::thresholds_with(Arch::Healing, u.clone()),
                    );
                    assert!(
                        h.ssr == r.ssr && h.sp == r.sp,
                        "output and healing must agree"
                    );
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
        ("speedGate", standard::speed_gate()),
        ("uSpeedUR", standard::u_speed_ur()),
        ("speedMax", standard::max_u(Arch::Speed)),
    ] {
        assert_eq!(
            value(name).as_ref(),
            Some(&ours),
            "{name} in {} differs from the calibration",
            path.display()
        );
    }
}

fn blocks() -> BTreeMap<&'static str, String> {
    let vs = vectors::all();
    let (vmd, _) = golden::vectors_table(&vs);
    BTreeMap::from([
        ("constants", constants()),
        ("k-law", k_law()),
        ("tier-rates", tier_rates()),
        ("measure-sensitivity", measure_sensitivity()),
        ("mu-sensitivity", mu_sensitivity(&vs)),
        ("law-robustness", law_robustness(&vs)),
        ("monte-carlo", simulate::monte_carlo()),
        ("normalization", simulate::normalization()),
        ("vectors", vmd),
        ("ssr-candidates", ssr_candidates()),
        ("speed-rates", speed_rates()),
    ])
}

/// Every block and its Chinese edition, `NAME-zh`, for the Chinese paper. A Chinese block that
/// still holds an English word fails the program: a new heading must be translated in `zh`.
fn with_chinese(blocks: &BTreeMap<&'static str, String>) -> BTreeMap<String, String> {
    let mut all = BTreeMap::new();
    let mut missing = Vec::new();
    for (name, body) in blocks {
        let chinese = zh::zh(body);
        let left = zh::untranslated(&chinese);
        if !left.is_empty() {
            missing.push(format!("{name}-zh: {left:?}"));
        }
        all.insert(format!("{name}-zh"), chinese);
        all.insert((*name).to_owned(), body.clone());
    }
    assert!(missing.is_empty(), "untranslated: {}", missing.join("; "));
    all
}

/// Replace every generated block of `text` with the current content.
fn fill(text: &str, blocks: &BTreeMap<String, String>) -> String {
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
    let all = with_chinese(&b);
    let (_, json) = golden::vectors_table(&vectors::all());
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
        files.push((p, fill(&text, &all)));
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
