//! The golden test vectors as a table and as JSON, and the facts each vector exists to show.

use std::fmt::Write as _;

use crate::model::{self, q};
use crate::standard::{self, Arch, anchors, mu};
use crate::{dp, frac};

pub fn vectors_table(vs: &[standard::SoulInput]) -> (String, String) {
    let mut md = String::from(
        "| Id | Soul | Slot, main, level | Best fit | U | Score | Specialized | Tier | Depth | Breadth | Growth | Every candidate |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    let mut json = String::from("[\n");
    for (i, v) in vs.iter().enumerate() {
        v.check()
            .unwrap_or_else(|e| panic!("{} is not a legal soul: {e}", v.id));
        let all = standard::quality(v);
        let lines: Vec<String> = v
            .lines
            .iter()
            .map(|l| format!("{} {} ({})", model::NAMES[l.attr], l.value, l.hits))
            .collect();
        let head = format!(
            "| {} | {} | {}, {}, +{} |",
            v.id,
            lines.join("; "),
            v.slot,
            model::NAMES[v.main],
            v.level
        );
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
        let comma = if i + 1 < vs.len() { "," } else { "" };
        let Some(best) = all.first() else {
            let _ = writeln!(md, "{head} — | — | — | — | unrated | — | — | — | none |");
            let _ = writeln!(
                json,
                "  {{\"id\": \"{}\", \"what\": \"{}\", \"slot\": {}, \"main\": \"{}\", \"level\": {}, \"lines\": [{}], \"best\": null, \"growth\": null, \"archetypes\": []}}{comma}",
                v.id,
                v.what,
                v.slot,
                model::NAMES[v.main],
                v.level,
                lines_json.join(", "),
            );
            continue;
        };
        let growth = standard::growth(v, best.arch);
        if let Some(g) = &growth {
            // The spec's recursion must agree with exact propagation.
            let present = v.lines.iter().fold(0u16, |m, l| m | (1 << l.attr));
            let r = 5 - u32::from(v.level / 3);
            let by_recursion = anchors(best.arch)
                .g(&(&best.utility
                    + mu() * standard::growth_recursion(r, present, best.arch.attrs())));
            assert_eq!(&by_recursion, g, "{}: growth recursion disagrees", v.id);
        }
        let every: Vec<String> = all
            .iter()
            .map(|a| format!("{} {} {}", a.arch.key(), a.tier.name(), dp(&a.score, 2)))
            .collect();
        let _ = writeln!(
            md,
            "{head} {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            best.arch.key(),
            dp(&best.utility, 4),
            dp(&best.score, 2),
            if best.specialized { "yes" } else { "no" },
            best.tier.name(),
            dp(&best.depth, 1),
            best.breadth.as_ref().map_or("n/a".to_owned(), |b| dp(b, 1)),
            growth.as_ref().map_or("—".to_owned(), |g| dp(g, 2)),
            every.join("; "),
        );
        let others: Vec<String> = all
            .iter()
            .map(|a| {
                format!(
                    "{{\"archetype\": \"{}\", \"utility\": \"{}\", \"score\": \"{}\", \"specialized\": {}, \"tier\": \"{}\", \"breadth\": {}}}",
                    a.arch.key(),
                    frac(&a.utility),
                    frac(&a.score),
                    a.specialized,
                    a.tier.name(),
                    a.breadth
                        .as_ref()
                        .map_or("null".to_owned(), |b| format!("\"{}\"", frac(b))),
                )
            })
            .collect();
        let _ = writeln!(
            json,
            "  {{\"id\": \"{}\", \"what\": \"{}\", \"slot\": {}, \"main\": \"{}\", \"level\": {}, \"lines\": [{}], \"best\": \"{}\", \"growth\": {}, \"archetypes\": [{}]}}{comma}",
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
        );
    }
    json += "]\n";
    check_vectors(vs);
    let mut why = String::from("\n| Id | What it shows |\n| --- | --- |\n");
    for v in vs {
        let _ = writeln!(why, "| {} | {} |", v.id, v.what);
    }
    (md + &why, json)
}

/// What the golden vectors exist to show; a change that breaks one fails the program.
fn check_vectors(vs: &[standard::SoulInput]) {
    use standard::Tier;
    let find = |id: &str| vs.iter().find(|v| v.id == id).expect("vector");
    let best = |id: &str| standard::quality(find(id)).remove(0);
    let under = |id: &str, a: Arch| {
        standard::quality(find(id))
            .into_iter()
            .find(|x| x.arch == a)
    };
    // The v1.1 tiers, unchanged on the four-attribute ladder.
    assert_eq!(
        best("V04").tier,
        Tier::Ssr,
        "one perfect line is SSR, not SP"
    );
    assert!(best("V04").specialized, "V04 is specialized");
    assert_eq!(
        best("V05").tier,
        Tier::Sp,
        "specialized above the floor is SP"
    );
    assert_ne!(
        best("V06").tier,
        Tier::Sp,
        "five roll units is not specialized"
    );
    assert_ne!(best("V07").tier, Tier::Sp, "specialization alone is not SP");
    assert_eq!(best("V08").tier, Tier::Ur);
    assert_eq!(best("V09").tier, Tier::Ur);
    assert!(best("V15").score > best("V14").score && best("V15").tier >= best("V14").tier);
    assert_eq!(
        best("V12").score,
        best("V13").score,
        "Crit and CritDmg lines alike"
    );
    // EffectRes is useful to no archetype: V01 scores 0 wherever it is judged.
    assert!(
        standard::quality(find("V01"))
            .iter()
            .all(|x| x.score == q(0, 1))
    );
    // The gate: below, at, and above four roll units, by value and not by increments.
    assert!(
        under("V18", Arch::Speed).is_none(),
        "3.97 roll units is not eligible"
    );
    for id in ["V19", "V20", "V21", "V22"] {
        let b = best(id);
        assert!(b.arch == Arch::Speed && b.tier == Tier::Ssr, "{id}");
    }
    assert_eq!(
        best("V19").score,
        best("V21").score,
        "the gate reads values"
    );
    assert!(best("V20").score > best("V19").score);
    assert_eq!(
        best("V23").tier,
        Tier::Ssr,
        "five roll units is below speed's UR"
    );
    assert_eq!(best("V24").tier, Tier::Ur);
    assert_eq!(best("V24").arch, Arch::Speed);
    assert_eq!(
        best("V25").score,
        q(100, 1),
        "the maximum Speed line scores 100"
    );
    assert_eq!(best("V25").tier, Tier::Ur);
    for id in ["V19", "V22", "V25"] {
        assert!(
            best(id).breadth.is_none(),
            "breadth is not applicable to speed"
        );
    }
    assert!(
        standard::quality(find("V26")).is_empty(),
        "V26 has no candidate"
    );
    // healing is not output: the same soul, far apart.
    let h = under("V27", Arch::Healing).expect("healing");
    let o = under("V27", Arch::Output).expect("output");
    assert!(best("V27").arch == Arch::Healing && h.tier == Tier::Ssr);
    assert!(&h.score - &o.score > q(15, 1) && o.tier < Tier::Sr);
    assert_eq!(best("V28").arch, Arch::Healing);
    assert_eq!(best("V29").arch, Arch::Hit);
    assert_eq!(best("V30").arch, Arch::Speed);
}
