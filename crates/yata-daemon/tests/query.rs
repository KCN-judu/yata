//! The headless query endpoint (ADR-0026, rule 6): requests in, exactly one result each, and a bad
//! request isolated from the rest.

use prost::Message;
use yata_core::scheme::code::{SchemeCode, StrengtheningPlan, StrengtheningSchemeSet, encode_code};
use yata_core::scheme::layout::{AccountSegment, Record, SchemeLayout, serialize};
use yata_core::scheme::name::SchemeName;
use yata_core::scheme::selection::{SetChoice, SoulSelection};
use yata_core::scheme::transport::encode_text;
use yata_core::soul::SoulSlot as DomainSlot;
use yata_daemon::query::{MAX_ROW_BUDGET, ServeError, handle, respond, serve};
use yata_daemon::wire::Code as _;
use yata_protocol::core::evaluate_query_result::{Outcome, Subject};
use yata_protocol::core::expr::Kind;
use yata_protocol::core::field::Field as FieldKind;
use yata_protocol::core::in_test::Values;
use yata_protocol::core::predicate::Test;
use yata_protocol::core::query_row::Verdict;
use yata_protocol::core::soul_selection::Sets;
use yata_protocol::core::{
    AnySet, BossSoul, Collection, Direction, EvaluateQuery, EvaluateQueryResult, ExactVerdict,
    Expr, ExprList, Field, InTest, IntRange, NumberRange, OpenRule, OpenVerdict, OrdinarySoul,
    PageRequest, ParamSetRef, Predicate, ProtocolVersion, Query, QueryPage, SchemeRef, SimpleField,
    Slots, SortKey, Soul, SoulAttribute, SoulSelection as WireSelection, SoulSlot, SubAttribute,
    SubAttributeChoice, SubAttributeMode, SuitCodes, VERSION, int_range, number_range,
    soul::Kind as SoulKind,
};
use yata_protocol::frame::{FrameError, decode_all, encode};

fn soul(id: &str, star: u32, spd: f64) -> Soul {
    Soul {
        soul_id: id.into(),
        suit_code: 30,
        slot: SoulSlot::SoulSlot2 as i32,
        star,
        level: 15,
        main: SoulAttribute::Spd as i32,
        main_value: 57.0,
        subs: vec![SubAttribute {
            attribute: SoulAttribute::Crit as i32,
            value: spd,
            enhancement_count: None,
        }],
        kind: Some(SoulKind::Ordinary(OrdinarySoul {})),
    }
}

fn inventory() -> Vec<Soul> {
    vec![
        soul("s3", 6, 7.0),
        soul("s1", 6, 9.0),
        soul("s4", 5, 9.0),
        soul("s2", 4, 1.0),
        soul("s5", 6, 3.0),
    ]
}

fn node(kind: Kind) -> Expr {
    Expr { kind: Some(kind) }
}

fn simple(f: SimpleField) -> Option<Field> {
    Some(Field {
        field: Some(FieldKind::Simple(f as i32)),
    })
}

fn pred(field: Option<Field>, test: Test) -> Expr {
    node(Kind::Pred(Predicate {
        field,
        test: Some(test),
    }))
}

fn star_at_least(min: i64) -> Expr {
    pred(
        simple(SimpleField::Star),
        Test::IntRange(IntRange {
            bound: Some(int_range::Bound::AtLeast(min)),
        }),
    )
}

fn query(filter: Option<Expr>) -> Query {
    Query {
        collection: Collection::Souls as i32,
        filter,
        ..Query::default()
    }
}

fn request(id: u64, q: Query) -> EvaluateQuery {
    paged(id, q, None)
}

/// A request for one page: the page belongs to the request, not to the query it pages.
fn paged(id: u64, q: Query, page: Option<PageRequest>) -> EvaluateQuery {
    EvaluateQuery {
        id,
        protocol_version: Some(VERSION),
        inventory: inventory(),
        query: Some(q),
        page,
    }
}

fn ask(r: &EvaluateQuery) -> EvaluateQueryResult {
    handle(&r.encode_to_vec())
}

fn page(r: &EvaluateQuery) -> QueryPage {
    match ask(r).outcome {
        Some(Outcome::Page(p)) => p,
        other => panic!("expected a page, got {other:?}"),
    }
}

fn code(r: &EvaluateQuery) -> String {
    error_code(&ask(r))
}

fn error_code(result: &EvaluateQueryResult) -> String {
    match &result.outcome {
        Some(Outcome::Error(e)) => e.kind.as_ref().map_or("no kind", |k| k.code()).to_owned(),
        other => panic!("expected an error, got {other:?}"),
    }
}

fn ids(p: &QueryPage) -> Vec<&str> {
    p.rows.iter().map(|r| r.soul_id.as_str()).collect()
}

fn exact() -> Option<Verdict> {
    Some(Verdict::Exact(ExactVerdict {}))
}

#[test]
fn a_query_over_a_supplied_inventory_returns_its_rows_in_identity_order() {
    let r = request(7, query(Some(star_at_least(5))));
    let result = ask(&r);
    assert_eq!(result.subject, Some(Subject::Id(7)));
    let Some(Outcome::Page(p)) = result.outcome else {
        panic!("a page")
    };
    assert_eq!(ids(&p), ["s1", "s3", "s4", "s5"]);
    assert_eq!(p.next_cursor, None);
    assert!(p.rows.iter().all(|r| r.verdict == exact()));
}

#[test]
fn the_same_rows_in_any_order_give_the_same_bytes() {
    let mut q = query(None);
    q.sort = vec![SortKey {
        field: Some(Field {
            field: Some(FieldKind::SubValue(SoulAttribute::Crit as i32)),
        }),
        direction: Direction::Desc as i32,
    }];
    let forward = request(1, q);
    let mut backward = forward.clone();
    backward.inventory.reverse();
    let a = respond(&forward.encode_to_vec());
    assert_eq!(a, respond(&backward.encode_to_vec()));
    assert_eq!(ids(&page(&forward)), ["s1", "s4", "s3", "s5", "s2"]);
}

#[test]
fn pages_follow_their_cursor_across_requests() {
    let mut q = query(None);
    q.sort = vec![SortKey {
        field: simple(SimpleField::Star),
        direction: Direction::Desc as i32,
    }];
    let mut seen = Vec::new();
    let mut cursor = None;
    loop {
        let p = page(&paged(
            1,
            q.clone(),
            Some(PageRequest {
                row_budget: Some(2),
                cursor: cursor.clone(),
            }),
        ));
        seen.extend(ids(&p).into_iter().map(str::to_owned));
        match p.next_cursor {
            None => break,
            Some(next) => cursor = Some(next),
        }
    }
    assert_eq!(seen, ["s1", "s3", "s5", "s4", "s2"]);
}

fn account() -> AccountSegment {
    AccountSegment::from_bytes([7; 14])
}

#[allow(clippy::expect_used, reason = "a failure here is the test failing")]
fn text(layout: &SchemeLayout) -> String {
    encode_text(&serialize(layout).expect("serializable"))
        .expect("encodable")
        .into_string()
}

fn scheme(code: String, entry: Option<u32>) -> Expr {
    node(Kind::MatchesScheme(SchemeRef { code, entry }))
}

#[test]
fn a_scheme_filter_reports_each_rows_verdict() {
    let mut slot2 = SoulSelection::new(SetChoice::AnySet);
    slot2.slots = [DomainSlot::Slot2].into();
    let mut slot4 = SoulSelection::new(SetChoice::AnySet);
    slot4.slots = [DomainSlot::Slot4].into();
    let set = SchemeCode::Strengthening(StrengtheningSchemeSet {
        plans: vec![
            StrengtheningPlan::new(SchemeName::new("a").expect("short"), slot2),
            StrengtheningPlan::new(SchemeName::new("b").expect("short"), slot4),
        ],
    });
    let code = text(&encode_code(&set, account()).expect("valid"));
    // Matches: every soul is in slot 2.
    let p = page(&request(1, query(Some(scheme(code.clone(), Some(0))))));
    assert_eq!(p.rows.len(), 5);
    assert!(p.rows.iter().all(|r| r.verdict == exact()));
    // DoesNotMatch: none is in slot 4.
    assert!(
        page(&request(1, query(Some(scheme(code, Some(1))))))
            .rows
            .is_empty()
    );
    // Undetermined: 破势, slot 2, 6★, +15, Spd main, and filter bit 61, which is unmapped.
    let mut filter = vec![0u8; 8];
    for bit in [1usize, 11, 18, 54, 61] {
        filter[bit / 8] |= 1 << (bit % 8);
    }
    let record = Record::new("x", vec![0, 0, 0x20], filter).expect("short");
    let unknown = text(&SchemeLayout::discard(account(), vec![record]).expect("valid"));
    let p = page(&request(1, query(Some(scheme(unknown, None)))));
    assert_eq!(ids(&p), ["s1", "s3", "s5"]);
    let open = Some(Verdict::Open(OpenVerdict {
        rules: vec![OpenRule::UnknownConditions as i32],
    }));
    assert!(p.rows.iter().all(|r| r.verdict == open));
}

fn choice(a: SoulAttribute, mode: SubAttributeMode) -> SubAttributeChoice {
    SubAttributeChoice {
        attribute: a as i32,
        mode: mode as i32,
    }
}

#[test]
fn an_inline_selection_is_converted_and_evaluated() {
    let selection = WireSelection {
        sets: Some(Sets::Chosen(SuitCodes { codes: vec![30] })),
        stars: vec![6],
        sub_attributes: vec![choice(SoulAttribute::Crit, SubAttributeMode::Include)],
        ..WireSelection::default()
    };
    let p = page(&request(1, query(Some(node(Kind::Matches(selection))))));
    assert_eq!(ids(&p), ["s1", "s3", "s5"]);
    let malformed = |selection| code(&request(1, query(Some(node(Kind::Matches(selection))))));
    // One attribute given two choices.
    assert_eq!(
        malformed(WireSelection {
            sets: Some(Sets::All(AnySet {})),
            sub_attributes: vec![
                choice(SoulAttribute::Crit, SubAttributeMode::Include),
                choice(SoulAttribute::Crit, SubAttributeMode::Exclude),
            ],
            ..WireSelection::default()
        }),
        "query.malformed"
    );
    // "Every set" is `all`; an empty chosen list is not a second encoding of it, and a selection
    // must say which it is.
    assert_eq!(
        malformed(WireSelection {
            sets: Some(Sets::Chosen(SuitCodes { codes: vec![] })),
            ..WireSelection::default()
        }),
        "query.malformed"
    );
    assert_eq!(malformed(WireSelection::default()), "query.malformed");
}

fn nest(depth: usize) -> Expr {
    (1..depth).fold(node(Kind::And(ExprList::default())), |e, _| {
        node(Kind::Not(Box::new(e)))
    })
}

#[test]
fn malformed_trees_are_refused_with_their_code() {
    let cases: Vec<(Query, &str)> = vec![
        (query(Some(Expr { kind: None })), "query.malformed"),
        (
            query(Some(pred(simple(SimpleField::Star), Test::Is(true)))),
            "query.type_mismatch",
        ),
        (
            query(Some(pred(
                Some(Field {
                    field: Some(FieldKind::Simple(99)),
                }),
                Test::Is(true),
            ))),
            "query.unknown_field",
        ),
        (
            query(Some(pred(Some(Field { field: None }), Test::Is(true)))),
            "query.malformed",
        ),
        (
            query(Some(node(Kind::Pred(Predicate {
                field: simple(SimpleField::Star),
                test: None,
            })))),
            "query.malformed",
        ),
        (
            query(Some(pred(
                simple(SimpleField::Slot),
                Test::In(InTest {
                    values: Some(Values::SlotValues(Slots { values: vec![42] })),
                }),
            ))),
            "query.malformed",
        ),
        (
            query(Some(pred(
                simple(SimpleField::Slot),
                Test::In(InTest { values: None }),
            ))),
            "query.malformed",
        ),
        (
            query(Some(pred(
                simple(SimpleField::Set),
                Test::In(InTest {
                    values: Some(Values::SetValues(SuitCodes { codes: vec![] })),
                }),
            ))),
            "query.malformed",
        ),
        (
            query(Some(pred(
                simple(SimpleField::MainValue),
                Test::NumberRange(NumberRange { bound: None }),
            ))),
            "query.malformed",
        ),
        (
            query(Some(pred(
                simple(SimpleField::Star),
                Test::IntRange(IntRange {
                    bound: Some(int_range::Bound::Between(yata_protocol::core::IntBetween {
                        min: 6,
                        max: 5,
                    })),
                }),
            ))),
            "query.malformed",
        ),
        (query(Some(nest(17))), "query.too_complex"),
        (
            query(Some(node(Kind::Or(ExprList {
                items: vec![star_at_least(1); 300],
            })))),
            "query.too_complex",
        ),
        (
            query(Some(scheme("not a scheme".into(), None))),
            "query.unknown_scheme",
        ),
        (
            Query {
                collection: Collection::Unspecified as i32,
                ..query(None)
            },
            "query.malformed",
        ),
    ];
    for (q, expected) in cases {
        assert_eq!(code(&request(3, q.clone())), expected, "{q:?}");
    }
    assert!(
        ask(&request(3, query(Some(nest(16)))))
            .outcome
            .is_some_and(|o| matches!(o, Outcome::Page(_)))
    );
}

#[test]
fn a_tree_past_the_decoders_recursion_limit_is_undecodable_not_a_crash() {
    let result = handle(&request(9, query(Some(nest(500)))).encode_to_vec());
    assert_eq!(
        result.subject,
        Some(Subject::Undecodable(yata_protocol::core::Undecodable {})),
        "the request did not decode, so it has no id to answer"
    );
    assert_eq!(error_code(&result), "query.malformed");
}

#[test]
fn bytes_that_are_not_a_request_are_malformed() {
    let result = handle(&[0xff, 0xff, 0xff]);
    assert!(matches!(result.subject, Some(Subject::Undecodable(_))));
    assert_eq!(error_code(&result), "query.malformed");
}

#[test]
fn score_fields_are_refused_until_pass_one_exists() {
    let q = query(Some(pred(
        Some(Field {
            field: Some(FieldKind::Quality(
                yata_protocol::core::QualityComponent::Total as i32,
            )),
        }),
        Test::NumberRange(NumberRange {
            bound: Some(number_range::Bound::AtLeast(50.0)),
        }),
    )));
    assert_eq!(code(&request(1, q.clone())), "query.param_set_required");
    let with_params = |id: &str| Query {
        params: Some(ParamSetRef {
            id: id.into(),
            version: 1,
        }),
        ..q.clone()
    };
    assert_eq!(
        code(&request(1, with_params("yata-quality"))),
        "query.field_unavailable"
    );
    assert_eq!(code(&request(1, with_params(""))), "query.malformed");
}

#[test]
fn request_level_refusals_carry_their_codes() {
    let mut newer = request(1, query(None));
    newer.protocol_version = Some(ProtocolVersion { major: 2, minor: 0 });
    assert_eq!(code(&newer), "session.protocol_unsupported");
    let mut unversioned = request(1, query(None));
    unversioned.protocol_version = None;
    assert_eq!(code(&unversioned), "session.protocol_unsupported");
    let mut repeated = request(1, query(None));
    repeated.inventory.push(soul("s1", 6, 1.0));
    assert_eq!(code(&repeated), "query.malformed");
    let mut unnamed = request(1, query(None));
    unnamed.inventory[0].soul_id = String::new();
    assert_eq!(code(&unnamed), "query.malformed");
    let with_page = |row_budget, cursor| {
        code(&paged(
            1,
            query(None),
            Some(PageRequest { row_budget, cursor }),
        ))
    };
    assert_eq!(with_page(Some(0), None), "query.malformed");
    assert_eq!(
        with_page(Some(MAX_ROW_BUDGET + 1), None),
        "query.too_complex"
    );
    assert_eq!(
        with_page(None, Some(vec![0xff, 0x01])),
        "query.malformed_cursor"
    );
    // A present, empty cursor is not the first page: it names no row.
    assert_eq!(with_page(None, Some(vec![])), "query.malformed_cursor");
}

#[test]
fn a_star_outside_one_to_six_is_malformed() {
    for bad in [0, 7] {
        let mut r = request(1, query(None));
        r.inventory.push(soul("s9", bad, 1.0));
        assert_eq!(code(&r), "query.malformed", "soul star {bad}");
        let selection = WireSelection {
            sets: Some(Sets::All(AnySet {})),
            stars: vec![bad],
            ..WireSelection::default()
        };
        assert_eq!(
            code(&request(1, query(Some(node(Kind::Matches(selection)))))),
            "query.malformed",
            "selection star {bad}"
        );
    }
}

#[test]
fn a_level_above_fifteen_is_malformed() {
    let mut r = request(1, query(None));
    r.inventory[0].level = 16;
    assert_eq!(code(&r), "query.malformed");
}

#[test]
fn a_value_that_is_not_a_stored_value_is_malformed() {
    for bad in [f64::NAN, f64::INFINITY, -1.0] {
        let mut sub = request(1, query(None));
        sub.inventory.push(soul("s9", 6, bad));
        assert_eq!(code(&sub), "query.malformed", "sub value {bad}");
        let mut main = request(1, query(None));
        main.inventory[0].main_value = bad;
        assert_eq!(code(&main), "query.malformed", "main value {bad}");
    }
}

#[test]
fn a_soul_must_state_its_kind() {
    // ADR-0029: there is no unknown kind, and a boss soul's innate attribute is one of six.
    let with = |kind| {
        let mut r = request(1, query(None));
        r.inventory[0].kind = kind;
        code(&r)
    };
    assert_eq!(with(None), "query.malformed");
    let boss = |a: SoulAttribute| Some(SoulKind::Boss(BossSoul { innate: a as i32 }));
    assert_eq!(with(boss(SoulAttribute::Spd)), "query.malformed");
    let mut r = request(1, query(None));
    r.inventory[0].kind = boss(SoulAttribute::Crit);
    assert_eq!(page(&r).rows.len(), 5);
}

#[test]
fn a_bad_request_in_a_stream_does_not_affect_the_next() {
    let good = request(1, query(Some(star_at_least(6))));
    let bad = request(2, query(Some(nest(500))));
    let alone = respond(&good.encode_to_vec());
    let mut stream = Vec::new();
    for payload in [
        bad.encode_to_vec(),
        good.encode_to_vec(),
        vec![0xff; 3],
        good.encode_to_vec(),
    ] {
        stream.extend(encode(&payload).expect("framable"));
    }
    let mut out = Vec::new();
    serve(&mut stream.as_slice(), &mut out).expect("clean end");
    let results: Vec<EvaluateQueryResult> = decode_all(&out)
        .expect("frames")
        .iter()
        .map(|p| EvaluateQueryResult::decode(p.as_slice()).expect("result"))
        .collect();
    assert_eq!(results.len(), 4, "exactly one result per request");
    assert_eq!(error_code(&results[0]), "query.malformed");
    assert_eq!(error_code(&results[2]), "query.malformed");
    let alone =
        EvaluateQueryResult::decode(&decode_all(&alone).expect("frame")[0][..]).expect("ok");
    assert_eq!(results[1], alone);
    assert_eq!(results[3], alone);
}

#[test]
fn a_broken_frame_ends_the_stream_after_answering_what_came_before() {
    let good = request(1, query(None));
    let mut stream = encode(&good.encode_to_vec()).expect("framable");
    stream.extend([0, 0, 0, 0]);
    let mut out = Vec::new();
    let end = serve(&mut stream.as_slice(), &mut out);
    assert!(matches!(end, Err(ServeError::Frame(FrameError::Empty))));
    assert_eq!(decode_all(&out).expect("frames").len(), 1);
}
