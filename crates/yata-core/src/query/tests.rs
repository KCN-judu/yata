use std::collections::BTreeSet;
use std::num::NonZeroUsize;

use proptest::prelude::*;

use super::eval::{OpenRules, Outcome};
use super::*;
use crate::scheme::code::{SchemeCode, StrengtheningPlan, StrengtheningSchemeSet, encode_code};
use crate::scheme::evaluate::{OpenRule, Verdict};
use crate::scheme::layout::{AccountSegment, Record, SchemeLayout, serialize};
use crate::scheme::selection::{InnateAttribute, SetChoice, SoulSelection};
use crate::scheme::transport::encode_text;
use crate::soul::{Innate, Soul, SoulAttribute, SoulSet, SoulSlot, SubAttribute};

use SoulAttribute::*;

fn soul(set: u8, slot: SoulSlot, star: u8, level: u8, main: SoulAttribute) -> Soul {
    Soul {
        set: SoulSet::from_suit_code(set),
        slot,
        star,
        level,
        main,
        main_value: 57.0,
        subs: Vec::new(),
        // What a reading holds until recordings settle how it carries one.
        innate: Innate::Unknown,
    }
}

fn with_subs(mut s: Soul, subs: &[(SoulAttribute, f64)]) -> Soul {
    s.subs = subs
        .iter()
        .map(|&(attribute, value)| SubAttribute {
            attribute,
            value,
            enhancement_count: None,
        })
        .collect();
    s
}

/// 破势 (30), slot 2, 6★, +15, main Spd; subs Crit 8.1, CritDmg 12, AtkPercent 3.
fn speed_two() -> Soul {
    with_subs(
        soul(30, SoulSlot::Slot2, 6, 15, Spd),
        &[(Crit, 8.1), (CritDmg, 12.0), (AtkPercent, 3.0)],
    )
}

fn filter(expr: Expr) -> SoulQuery {
    SoulQuery {
        filter: Some(expr),
        sort: Vec::new(),
        params: None,
    }
}

fn verdict(expr: Expr, s: &Soul) -> Verdict {
    compile(filter(expr)).expect("valid").verdict(s)
}

fn refusal(query: SoulQuery) -> QueryError {
    compile(query).expect_err("refused")
}

fn pred(field: Field, test: Test) -> Expr {
    Expr::Pred(field, test)
}

fn int(field: Field, min: Option<i64>, max: Option<i64>) -> Expr {
    pred(field, Test::IntRange { min, max })
}

fn num(field: Field, min: Option<f64>, max: Option<f64>) -> Expr {
    pred(field, Test::NumberRange { min, max })
}

fn is(field: Field, b: bool) -> Expr {
    pred(field, Test::Is(b))
}

const YES: Verdict = Verdict::Matches;
const NO: Verdict = Verdict::DoesNotMatch;

/// A selection that `speed_two` passes, with a chosen innate attribute: open for every soul that
/// no decided group rules out.
fn innate_open() -> Expr {
    let mut s = SoulSelection::new(SetChoice::AnySet);
    s.innate = BTreeSet::from([InnateAttribute::ALL[0]]);
    Expr::Matches(s)
}

fn open(rules: &[OpenRule]) -> Verdict {
    Verdict::Undetermined(rules.to_vec())
}

// ---- predicates ----

#[test]
fn enum_predicates_test_membership() {
    let s = speed_two();
    let set = |c| EnumValue::Set(SoulSet::from_suit_code(c));
    assert_eq!(
        verdict(pred(Field::Set, Test::In(vec![set(10), set(30)])), &s),
        YES
    );
    assert_eq!(verdict(pred(Field::Set, Test::In(vec![set(10)])), &s), NO);
    let slots = |k| pred(Field::Slot, Test::In(vec![EnumValue::Slot(k)]));
    assert_eq!(verdict(slots(SoulSlot::Slot2), &s), YES);
    assert_eq!(verdict(slots(SoulSlot::Slot4), &s), NO);
    let main = |a| {
        pred(
            Field::MainAttribute,
            Test::In(vec![EnumValue::Attribute(a)]),
        )
    };
    assert_eq!(verdict(main(Spd), &s), YES);
    assert_eq!(verdict(main(AtkPercent), &s), NO);
}

#[test]
fn int_predicates_test_inclusive_ranges() {
    let s = speed_two();
    assert_eq!(verdict(int(Field::Star, Some(6), Some(6)), &s), YES);
    assert_eq!(verdict(int(Field::Star, None, Some(5)), &s), NO);
    assert_eq!(verdict(int(Field::Level, Some(15), None), &s), YES);
    assert_eq!(verdict(int(Field::Level, Some(0), Some(14)), &s), NO);
    assert_eq!(verdict(int(Field::SubCount, Some(3), Some(3)), &s), YES);
    assert_eq!(verdict(int(Field::SubCount, Some(4), None), &s), NO);
}

#[test]
fn number_predicates_read_stored_values_and_zero_for_an_absent_sub() {
    let s = speed_two();
    assert_eq!(verdict(num(Field::MainValue, Some(57.0), None), &s), YES);
    assert_eq!(verdict(num(Field::MainValue, None, Some(56.9)), &s), NO);
    assert_eq!(
        verdict(num(Field::SubValue(Crit), Some(8.0), Some(8.2)), &s),
        YES
    );
    assert_eq!(verdict(num(Field::SubValue(Crit), Some(8.2), None), &s), NO);
    // No nulls: `Spd` is not a sub-attribute here, so its value is 0.
    assert_eq!(verdict(num(Field::SubValue(Spd), None, Some(0.0)), &s), YES);
    assert_eq!(verdict(num(Field::SubValue(Spd), Some(0.1), None), &s), NO);
}

#[test]
fn number_bounds_compare_with_the_domain_tolerance() {
    // A stored 17 − 1e-9 is 真 17 速; a stored 16.5 is not (ADR-0026, rule 3).
    let spd = |v| with_subs(soul(30, SoulSlot::Slot1, 6, 15, AtkFlat), &[(Spd, v)]);
    let at_least_17 = || num(Field::SubValue(Spd), Some(17.0), None);
    assert_eq!(verdict(at_least_17(), &spd(17.0 - 1e-9)), YES);
    assert_eq!(verdict(at_least_17(), &spd(16.5)), NO);
    let at_most_17 = num(Field::SubValue(Spd), None, Some(17.0));
    assert_eq!(verdict(at_most_17, &spd(17.0 + 1e-9)), YES);
}

#[test]
fn bool_predicates_test_presence_and_pristine() {
    let s = speed_two();
    assert_eq!(verdict(is(Field::HasSub(Crit), true), &s), YES);
    assert_eq!(verdict(is(Field::HasSub(Spd), true), &s), NO);
    assert_eq!(verdict(is(Field::HasSub(Spd), false), &s), YES);
    let four = [(Crit, 2.4), (CritDmg, 3.0), (Spd, 2.0), (AtkPercent, 2.5)];
    let pristine = with_subs(soul(30, SoulSlot::Slot2, 6, 0, Spd), &four);
    assert_eq!(verdict(is(Field::Pristine, true), &pristine), YES);
    let strengthened = Soul {
        level: 3,
        ..pristine.clone()
    };
    assert_eq!(verdict(is(Field::Pristine, true), &strengthened), NO);
    let three_legs = with_subs(soul(30, SoulSlot::Slot2, 6, 0, Spd), &four[..3]);
    assert_eq!(verdict(is(Field::Pristine, false), &three_legs), YES);
}

// ---- boolean composition ----

#[test]
fn empty_and_is_true_and_empty_or_is_false() {
    let s = speed_two();
    assert_eq!(verdict(Expr::And(vec![]), &s), YES);
    assert_eq!(verdict(Expr::Or(vec![]), &s), NO);
    assert_eq!(
        compile(SoulQuery {
            filter: None,
            ..filter(Expr::And(vec![]))
        })
        .expect("ok")
        .verdict(&s),
        YES
    );
}

#[test]
fn nested_and_or_not_compose() {
    let s = speed_two();
    let six = || int(Field::Star, Some(6), None);
    let five = || int(Field::Star, None, Some(5));
    let crit = || is(Field::HasSub(Crit), true);
    // (6★ ∧ ¬5★) ∨ (5★ ∧ crit)
    let e = Expr::Or(vec![
        Expr::And(vec![six(), Expr::Not(Box::new(five()))]),
        Expr::And(vec![five(), crit()]),
    ]);
    assert_eq!(verdict(e, &s), YES);
    // ¬(6★ ∨ 5★) ∧ crit
    let e = Expr::And(vec![
        Expr::Not(Box::new(Expr::Or(vec![six(), five()]))),
        crit(),
    ]);
    assert_eq!(verdict(e, &s), NO);
}

#[test]
fn an_open_verdict_stays_open_through_not() {
    let s = speed_two();
    let innate = [OpenRule::Innate];
    assert_eq!(verdict(innate_open(), &s), open(&innate));
    assert_eq!(
        verdict(Expr::Not(Box::new(innate_open())), &s),
        open(&innate)
    );
}

#[test]
fn a_decided_operand_settles_an_open_one() {
    let s = speed_two();
    let yes = || int(Field::Star, Some(6), None);
    let no = || int(Field::Star, None, Some(5));
    assert_eq!(verdict(Expr::And(vec![innate_open(), no()]), &s), NO);
    assert_eq!(verdict(Expr::Or(vec![innate_open(), yes()]), &s), YES);
    assert_eq!(
        verdict(Expr::And(vec![innate_open(), yes()]), &s),
        open(&[OpenRule::Innate])
    );
    assert_eq!(
        verdict(Expr::Or(vec![no(), innate_open()]), &s),
        open(&[OpenRule::Innate])
    );
}

#[test]
fn an_open_result_names_every_open_rule_beneath_it() {
    let s = speed_two();
    let code = unknown_condition_code();
    let scheme = Expr::MatchesScheme(SchemeRef { code, entry: None });
    let both = Expr::And(vec![innate_open(), scheme]);
    assert_eq!(
        verdict(both, &s),
        open(&[OpenRule::Innate, OpenRule::UnknownConditions])
    );
}

fn outcome() -> impl Strategy<Value = Outcome> {
    prop_oneof![
        Just(Outcome::Yes),
        Just(Outcome::No),
        (1u8..4).prop_map(|m| Outcome::Open(OpenRules::of(
            &[OpenRule::Innate, OpenRule::UnknownConditions]
                .into_iter()
                .enumerate()
                .filter(|(i, _)| m & (1 << i) != 0)
                .map(|(_, r)| r)
                .collect::<Vec<_>>()
        ))),
    ]
}

/// The classical readings of an outcome: both booleans when it is open.
fn readings(o: Outcome) -> Vec<bool> {
    match o {
        Outcome::Yes => vec![true],
        Outcome::No => vec![false],
        Outcome::Open(_) => vec![true, false],
    }
}

proptest! {
    #[test]
    fn kleene_logic_is_sound_for_every_reading(a in outcome(), b in outcome()) {
        // A decided result holds under every reading of its open operands (ADR-0026, rule 2).
        for (combined, op) in [(a.and(b), (|x, y| x && y) as fn(bool, bool) -> bool), (a.or(b), |x, y| x || y)] {
            let results: BTreeSet<bool> = readings(a)
                .into_iter()
                .flat_map(|x| readings(b).into_iter().map(move |y| op(x, y)))
                .collect();
            match combined {
                Outcome::Yes => prop_assert_eq!(results, BTreeSet::from([true])),
                Outcome::No => prop_assert_eq!(results, BTreeSet::from([false])),
                Outcome::Open(_) => prop_assert!(!results.is_empty()),
            }
        }
    }

    #[test]
    fn not_is_an_involution_and_de_morgan_holds(a in outcome(), b in outcome()) {
        prop_assert_eq!(a.not().not(), a);
        prop_assert_eq!(a.and(b).not(), a.not().or(b.not()));
        prop_assert_eq!(a.and(b), b.and(a));
    }
}

// ---- scheme filtering ----

fn account() -> AccountSegment {
    AccountSegment::from_bytes([7; 14])
}

fn text_of(layout: &SchemeLayout) -> String {
    encode_text(&serialize(layout).expect("serializable"))
        .expect("encodable")
        .into_string()
}

/// A strengthening set of the given plans.
fn code_of(selections: Vec<SoulSelection>) -> String {
    let plans = selections
        .into_iter()
        .enumerate()
        .map(|(i, s)| StrengtheningPlan::new(format!("p{i}"), s))
        .collect();
    let code = SchemeCode::Strengthening(StrengtheningSchemeSet { plans });
    text_of(&encode_code(&code, account()).expect("valid"))
}

/// A discard scheme for 破势, slot 2, 6★, +15, main Spd, and filter bit 61, which the model does
/// not map (the fixture of `scheme::evaluate`'s tests).
fn unknown_condition_code() -> String {
    let mut filter = vec![0u8; 8];
    for bit in [1usize, 11, 18, 54, 61] {
        filter[bit / 8] |= 1 << (bit % 8);
    }
    let record = Record::new("x", vec![0, 0, 0x20], filter).expect("short");
    text_of(&SchemeLayout::discard(account(), vec![record]).expect("valid"))
}

fn slots(k: SoulSlot) -> SoulSelection {
    let mut s = SoulSelection::new(SetChoice::AnySet);
    s.slots = BTreeSet::from([k]);
    s
}

fn scheme(code: &str, entry: Option<usize>) -> Expr {
    Expr::MatchesScheme(SchemeRef {
        code: code.to_owned(),
        entry,
    })
}

#[test]
fn a_scheme_filter_takes_the_scheme_evaluators_verdict() {
    let s = speed_two();
    let code = code_of(vec![slots(SoulSlot::Slot2), slots(SoulSlot::Slot4)]);
    assert_eq!(verdict(scheme(&code, Some(0)), &s), YES);
    assert_eq!(verdict(scheme(&code, Some(1)), &s), NO);
    let unknown = unknown_condition_code();
    assert_eq!(
        verdict(scheme(&unknown, None), &s),
        open(&[OpenRule::UnknownConditions])
    );
    let five_star = Soul { star: 5, ..s };
    assert_eq!(verdict(scheme(&unknown, None), &five_star), NO);
}

#[test]
fn an_inline_selection_is_the_official_filter() {
    let s = speed_two();
    assert_eq!(verdict(Expr::Matches(slots(SoulSlot::Slot2)), &s), YES);
    assert_eq!(verdict(Expr::Matches(slots(SoulSlot::Slot6)), &s), NO);
}

#[test]
fn a_scheme_reference_names_exactly_one_entry() {
    let two = code_of(vec![slots(SoulSlot::Slot2), slots(SoulSlot::Slot4)]);
    let unknown = |e| QueryError::UnknownScheme(e);
    assert_eq!(
        refusal(filter(scheme(&two, None))),
        unknown(SchemeProblem::EntryRequired { entries: 2 })
    );
    assert_eq!(
        refusal(filter(scheme(&two, Some(2)))),
        unknown(SchemeProblem::NoSuchEntry {
            entry: 2,
            entries: 2
        })
    );
    assert!(matches!(
        refusal(filter(scheme("not a scheme code", None))),
        QueryError::UnknownScheme(SchemeProblem::Transport(_))
    ));
    let empty = code_of(vec![]);
    assert_eq!(
        refusal(filter(scheme(&empty, None))),
        unknown(SchemeProblem::NoSuchEntry {
            entry: 0,
            entries: 0
        })
    );
}

// ---- refusals ----

fn not_chain(depth: usize) -> Expr {
    (1..depth).fold(Expr::And(vec![]), |e, _| Expr::Not(Box::new(e)))
}

#[test]
fn depth_is_limited() {
    assert!(compile(filter(not_chain(MAX_EXPR_DEPTH))).is_ok());
    assert_eq!(
        refusal(filter(not_chain(MAX_EXPR_DEPTH + 1))),
        QueryError::TooComplex {
            limit: Limit::ExprDepth,
            actual: MAX_EXPR_DEPTH + 1
        }
    );
}

#[test]
fn node_count_is_limited() {
    let flat = |n: usize| Expr::And(vec![Expr::Or(vec![]); n - 1]);
    assert!(compile(filter(flat(MAX_EXPR_NODES))).is_ok());
    assert_eq!(
        refusal(filter(flat(MAX_EXPR_NODES + 1))),
        QueryError::TooComplex {
            limit: Limit::ExprNodes,
            actual: MAX_EXPR_NODES + 1
        }
    );
}

#[test]
fn test_values_and_sort_keys_are_limited() {
    let values = |n| {
        pred(
            Field::Slot,
            Test::In(vec![EnumValue::Slot(SoulSlot::Slot1); n]),
        )
    };
    assert!(compile(filter(values(MAX_TEST_VALUES))).is_ok());
    assert_eq!(
        refusal(filter(values(MAX_TEST_VALUES + 1))),
        QueryError::TooComplex {
            limit: Limit::TestValues,
            actual: MAX_TEST_VALUES + 1
        }
    );
    let keys = |n| SoulQuery {
        filter: None,
        sort: vec![
            SortKey {
                field: Field::Star,
                direction: Direction::Asc
            };
            n
        ],
        params: None,
    };
    assert!(compile(keys(MAX_SORT_KEYS)).is_ok());
    assert!(matches!(
        refusal(keys(MAX_SORT_KEYS + 1)),
        QueryError::TooComplex {
            limit: Limit::SortKeys,
            ..
        }
    ));
}

#[test]
fn a_test_must_fit_its_field() {
    let wrong = [
        (Field::Star, Test::Is(true)),
        (
            Field::Set,
            Test::IntRange {
                min: Some(1),
                max: None,
            },
        ),
        (
            Field::Level,
            Test::NumberRange {
                min: Some(1.0),
                max: None,
            },
        ),
        (
            Field::MainValue,
            Test::IntRange {
                min: Some(1),
                max: None,
            },
        ),
        (
            Field::HasSub(Spd),
            Test::In(vec![EnumValue::Attribute(Spd)]),
        ),
        (
            Field::Slot,
            Test::In(vec![
                EnumValue::Slot(SoulSlot::Slot1),
                EnumValue::Attribute(Spd),
            ]),
        ),
    ];
    for (field, test) in wrong {
        let kind = test.kind();
        assert_eq!(
            refusal(filter(pred(field, test))),
            QueryError::TypeMismatch { field, test: kind },
            "{field:?}"
        );
    }
}

#[test]
fn a_malformed_test_is_refused_not_read_as_false() {
    let malformed = |m| QueryError::Malformed(m);
    assert_eq!(
        refusal(filter(pred(Field::Set, Test::In(vec![])))),
        malformed(Malformation::EmptyIn)
    );
    assert_eq!(
        refusal(filter(int(Field::Star, None, None))),
        malformed(Malformation::RangeWithoutBound)
    );
    assert_eq!(
        refusal(filter(int(Field::Star, Some(6), Some(5)))),
        malformed(Malformation::RangeInverted)
    );
    assert_eq!(
        refusal(filter(num(Field::MainValue, Some(f64::NAN), None))),
        malformed(Malformation::NonFiniteBound)
    );
    assert_eq!(
        refusal(filter(num(Field::MainValue, None, Some(f64::INFINITY)))),
        malformed(Malformation::NonFiniteBound)
    );
}

#[test]
fn a_score_field_needs_a_parameter_set_and_is_not_evaluated_yet() {
    let total = Field::Quality(QualityComponent::Total);
    let q = filter(num(total, Some(50.0), None));
    assert_eq!(
        refusal(q.clone()),
        QueryError::ParamSetRequired { field: total }
    );
    let with_params = SoulQuery {
        params: Some(ParamSetRef {
            id: "yata-quality".into(),
            version: 1,
        }),
        ..q
    };
    assert_eq!(
        refusal(with_params),
        QueryError::FieldUnavailable { field: total }
    );
    let sorted = SoulQuery {
        filter: None,
        sort: vec![SortKey {
            field: total,
            direction: Direction::Desc,
        }],
        params: None,
    };
    assert_eq!(
        refusal(sorted),
        QueryError::ParamSetRequired { field: total }
    );
}

#[test]
fn has_sub_is_not_a_sort_key() {
    let q = SoulQuery {
        filter: None,
        sort: vec![SortKey {
            field: Field::HasSub(Spd),
            direction: Direction::Asc,
        }],
        params: None,
    };
    assert_eq!(
        refusal(q),
        QueryError::NotSortable {
            field: Field::HasSub(Spd)
        }
    );
}

// ---- order and pages ----

fn inventory() -> Vec<(String, Soul)> {
    let s = |id: &str, star, spd| {
        (
            id.to_owned(),
            with_subs(soul(30, SoulSlot::Slot1, star, 15, AtkFlat), &[(Spd, spd)]),
        )
    };
    vec![
        s("e", 6, 10.0),
        s("a", 6, 17.0),
        s("d", 5, 17.0),
        s("b", 6, 12.0),
        s("c", 6, 17.0),
        s("f", 4, 3.0),
    ]
}

fn run(
    q: &CompiledQuery,
    souls: &[(String, Soul)],
    budget: usize,
    cursor: Option<Cursor<String>>,
) -> Page<String> {
    let request = PageRequest {
        row_budget: NonZeroUsize::new(budget).expect("positive"),
        cursor,
    };
    q.page(souls.iter().map(|(id, s)| (id, s)), &request)
        .expect("valid")
}

fn ids(page: &Page<String>) -> Vec<&str> {
    page.rows.iter().map(|r| r.id.as_str()).collect()
}

#[test]
fn the_default_order_is_row_identity() {
    let q = compile(filter(int(Field::Star, Some(5), None))).expect("ok");
    let page = run(&q, &inventory(), 100, None);
    assert_eq!(ids(&page), ["a", "b", "c", "d", "e"]);
    assert!(page.next.is_none());
}

#[test]
fn sort_keys_order_rows_and_identity_breaks_ties() {
    let q = compile(SoulQuery {
        filter: None,
        sort: vec![
            SortKey {
                field: Field::SubValue(Spd),
                direction: Direction::Desc,
            },
            SortKey {
                field: Field::Star,
                direction: Direction::Asc,
            },
        ],
        params: None,
    })
    .expect("ok");
    let all = run(&q, &inventory(), 100, None);
    assert_eq!(ids(&all), ["d", "a", "c", "b", "e", "f"]);
    // The same rows in any input order give the same page.
    let mut reversed = inventory();
    reversed.reverse();
    assert_eq!(run(&q, &reversed, 100, None), all);
}

#[test]
fn pages_continue_from_their_cursor_without_gaps_or_repeats() {
    let q = compile(SoulQuery {
        filter: None,
        sort: vec![SortKey {
            field: Field::SubValue(Spd),
            direction: Direction::Desc,
        }],
        params: None,
    })
    .expect("ok");
    let souls = inventory();
    let whole = ids(&run(&q, &souls, 100, None)).join("");
    for budget in 1..=6 {
        let mut seen = String::new();
        let mut cursor = None;
        loop {
            let page = run(&q, &souls, budget, cursor);
            assert!(page.rows.len() <= budget);
            seen += &ids(&page).join("");
            match page.next {
                Some(c) => cursor = Some(c),
                None => break,
            }
        }
        assert_eq!(seen, whole, "budget {budget}");
    }
}

#[test]
fn rows_carry_their_verdict() {
    let q = compile(filter(Expr::Or(vec![
        int(Field::Star, Some(6), None),
        innate_open(),
    ])))
    .expect("ok");
    let page = run(&q, &inventory(), 100, None);
    let by_id: Vec<(&str, &Verdict)> = page
        .rows
        .iter()
        .map(|r| (r.id.as_str(), &r.verdict))
        .collect();
    let innate = open(&[OpenRule::Innate]);
    assert_eq!(
        by_id,
        [
            ("a", &YES),
            ("b", &YES),
            ("c", &YES),
            ("d", &innate),
            ("e", &YES),
            ("f", &innate)
        ]
    );
}

#[test]
fn a_cursor_from_another_order_is_refused() {
    let by_star = compile(SoulQuery {
        filter: None,
        sort: vec![SortKey {
            field: Field::Star,
            direction: Direction::Asc,
        }],
        params: None,
    })
    .expect("ok");
    let souls = inventory();
    let request = |keys| PageRequest {
        row_budget: NonZeroUsize::MIN,
        cursor: Some(Cursor {
            keys,
            id: "a".to_owned(),
        }),
    };
    let page = |keys| by_star.page(souls.iter().map(|(id, s)| (id, s)), &request(keys));
    assert_eq!(page(vec![]), Err(QueryError::MalformedCursor));
    assert_eq!(
        page(vec![SortValue::Number(6.0)]),
        Err(QueryError::MalformedCursor)
    );
    assert_eq!(
        page(vec![SortValue::Int(6), SortValue::Int(6)]),
        Err(QueryError::MalformedCursor)
    );
    assert!(page(vec![SortValue::Int(6)]).is_ok());
}
