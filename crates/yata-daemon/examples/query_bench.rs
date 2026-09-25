//! Query evaluation over synthetic inventories of realistic size, for deciding whether an index is
//! needed (ADR-0026). Run with `cargo run --release -p yata-daemon --example query_bench`.
//!
//! The inventory is generated from a fixed seed, so two runs time the same work. Its shape is
//! rough, not sourced: most souls 6★, levels bunched at +0 and +15, two to four distinct
//! sub-attributes. What is measured is the evaluator, whose cost depends on the tree and the row
//! count, not on the exact distribution.

#![allow(
    clippy::expect_used,
    reason = "every input is fixed and valid; a failure is a bug in the benchmark"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::hint::black_box;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

use prost::Message;
use yata_core::query::{
    Bound, CompiledQuery, Direction, EnumValue, Expr, Field, PageRequest, SchemeCodeText,
    SchemeRef, SortKey, SoulQuery, Test, compile,
};
use yata_core::scheme::code::{SchemeCode, StrengtheningPlan, StrengtheningSchemeSet, encode_code};
use yata_core::scheme::layout::{AccountSegment, serialize};
use yata_core::scheme::selection::{LevelBand, SetChoice, SoulSelection, SubAttributeMode};
use yata_core::scheme::transport::encode_text;
use yata_core::soul::{Soul, SoulAttribute, SoulKind, SoulSet, SoulSlot, SubAttribute};
use yata_protocol::core as wire;

/// The first forty suit codes of the scheme mapping, in soul-bit order: sets a scheme can name.
const SUIT_CODES: [u8; 40] = [
    2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 18, 19, 20, 22, 24, 26, 27, 29, 30, 31, 32, 33,
    34, 35, 36, 23, 21, 39, 48, 49, 50, 51, 52, 53, 54, 73, 74,
];

/// A 64-bit linear congruential generator (Knuth's MMIX constants): deterministic, no dependency.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn inventory(n: usize, seed: u64) -> BTreeMap<String, Soul> {
    let mut r = Lcg(seed);
    (0..n)
        .map(|i| {
            let slot = SoulSlot::ALL[r.below(6) as usize];
            let options = slot.main_options();
            let main = options[r.below(options.len() as u64) as usize];
            let star = match r.below(10) {
                0 => 4,
                1 | 2 => 5,
                _ => 6,
            };
            let level = match r.below(10) {
                0..=3 => 0,
                4..=7 => 15,
                _ => r.below(15) as u8,
            };
            let count = 2 + r.below(3) as usize;
            let mut attributes = BTreeSet::new();
            while attributes.len() < count {
                attributes.insert(SoulAttribute::ALL[r.below(11) as usize]);
            }
            let subs = attributes
                .into_iter()
                .map(|attribute| SubAttribute {
                    attribute,
                    value: 1.0 + r.below(200) as f64 / 10.0,
                    enhancement_count: None,
                })
                .collect();
            let soul = Soul {
                set: SoulSet::from_suit_code(SUIT_CODES[r.below(40) as usize]),
                slot,
                star,
                level,
                main,
                main_value: 10.0 + r.below(500) as f64 / 10.0,
                subs,
                kind: SoulKind::Ordinary,
            };
            (format!("soul-{i:06}"), soul)
        })
        .collect()
}

fn scheme_code() -> String {
    let mut s = SoulSelection::new(SetChoice::Sets(
        SUIT_CODES[..20]
            .iter()
            .copied()
            .map(SoulSet::from_suit_code)
            .collect(),
    ));
    s.slots = [SoulSlot::Slot2, SoulSlot::Slot4, SoulSlot::Slot6].into();
    s.stars = [6].into();
    s.levels = [LevelBand::L0to2, LevelBand::L15].into();
    s.sub_attributes
        .set(SoulAttribute::Spd, SubAttributeMode::Include);
    let code = SchemeCode::Strengthening(StrengtheningSchemeSet {
        plans: vec![StrengtheningPlan::new("bench", s)],
    });
    let layout = encode_code(&code, AccountSegment::from_bytes([7; 14])).expect("valid");
    encode_text(&serialize(&layout).expect("serializable"))
        .expect("encodable")
        .into_string()
}

fn nested() -> Expr {
    let slots = [SoulSlot::Slot2, SoulSlot::Slot4, SoulSlot::Slot6]
        .map(EnumValue::Slot)
        .to_vec();
    let at_least = |a, v| Expr::Pred(Field::SubValue(a), Test::NumberRange(Bound::AtLeast(v)));
    Expr::And(vec![
        Expr::Pred(Field::Slot, Test::In(slots)),
        Expr::Or(vec![
            at_least(SoulAttribute::Spd, 15.0),
            Expr::And(vec![
                Expr::Pred(Field::HasSub(SoulAttribute::Crit), Test::Is(true)),
                at_least(SoulAttribute::CritDmg, 15.0),
            ]),
        ]),
        Expr::Not(Box::new(Expr::Pred(
            Field::Level,
            Test::IntRange(Bound::AtMost(3)),
        ))),
    ])
}

fn sorted(filter: Option<Expr>) -> SoulQuery {
    SoulQuery {
        filter,
        sort: vec![
            SortKey {
                field: Field::SubValue(SoulAttribute::Spd),
                direction: Direction::Desc,
            },
            SortKey {
                field: Field::Star,
                direction: Direction::Asc,
            },
        ],
        params: None,
    }
}

fn unsorted(filter: Expr) -> SoulQuery {
    SoulQuery {
        filter: Some(filter),
        sort: Vec::new(),
        params: None,
    }
}

/// The median of `runs` timings of `f`.
fn median(runs: usize, mut f: impl FnMut()) -> Duration {
    let mut times: Vec<Duration> = (0..runs)
        .map(|_| {
            let start = Instant::now();
            f();
            start.elapsed()
        })
        .collect();
    times.sort_unstable();
    times[runs / 2]
}

fn page_of(q: &CompiledQuery, souls: &BTreeMap<String, Soul>, budget: usize) -> usize {
    let request = PageRequest {
        row_budget: NonZeroUsize::new(budget).expect("positive"),
        cursor: None,
    };
    q.page(souls, &request).expect("valid").rows.len()
}

fn wire_soul(id: &str, s: &Soul) -> wire::Soul {
    let slot = SoulSlot::ALL.iter().position(|&k| k == s.slot).unwrap_or(0) as i32 + 1;
    let attribute = |a| SoulAttribute::ALL.iter().position(|&x| x == a).unwrap_or(0) as i32 + 1;
    wire::Soul {
        soul_id: id.to_owned(),
        suit_code: u32::from(s.set.suit_code()),
        slot,
        star: u32::from(s.star),
        level: u32::from(s.level),
        main: attribute(s.main),
        main_value: s.main_value,
        subs: s
            .subs
            .iter()
            .map(|x| wire::SubAttribute {
                attribute: attribute(x.attribute),
                value: x.value,
                enhancement_count: None,
            })
            .collect(),
        kind: Some(wire::soul::Kind::Ordinary(wire::OrdinarySoul {})),
    }
}

fn main() {
    let code = scheme_code();
    let cases: Vec<(&str, SoulQuery, usize)> = vec![
        (
            "one predicate, all rows",
            unsorted(Expr::Pred(Field::Star, Test::IntRange(Bound::AtLeast(6)))),
            usize::MAX,
        ),
        (
            "nested and/or/not, all rows",
            unsorted(nested()),
            usize::MAX,
        ),
        (
            "matches_scheme, all rows",
            unsorted(Expr::MatchesScheme(SchemeRef {
                code: SchemeCodeText(code.clone()),
                entry: None,
            })),
            usize::MAX,
        ),
        (
            "nested, two sort keys, page 256",
            sorted(Some(nested())),
            256,
        ),
        ("no filter, two sort keys, page 256", sorted(None), 256),
    ];
    println!(
        "{:>7}  {:<36} {:>8} {:>10}",
        "souls", "query", "rows", "median"
    );
    for n in [1_000, 10_000, 100_000] {
        let souls = inventory(n, 0x5eed);
        let runs = if n >= 100_000 { 11 } else { 31 };
        for (name, q, budget) in &cases {
            let compiled = compile(q.clone()).expect("valid");
            let budget = (*budget).min(n.max(1));
            let rows = page_of(&compiled, &souls, budget);
            let t = median(runs, || {
                black_box(page_of(&compiled, black_box(&souls), budget));
            });
            println!("{n:>7}  {name:<36} {rows:>8} {:>10.3?}", t);
        }
    }
    // The endpoint end to end: decode a request carrying the inventory, convert, evaluate, encode.
    for n in [10_000, 100_000] {
        let souls = inventory(n, 0x5eed);
        let request = wire::EvaluateQuery {
            id: 1,
            protocol_version: Some(wire::VERSION),
            inventory: souls.iter().map(|(id, s)| wire_soul(id, s)).collect(),
            query: Some(wire::Query {
                collection: wire::Collection::Souls as i32,
                filter: None,
                sort: Vec::new(),
                params: None,
            }),
            page: None,
        }
        .encode_to_vec();
        let t = median(11, || {
            black_box(yata_daemon::query::respond(black_box(&request)));
        });
        println!(
            "{n:>7}  {:<36} {:>8} {:>10.3?}",
            "endpoint: decode + page 256",
            request.len(),
            t
        );
    }
}
