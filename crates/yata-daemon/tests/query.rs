//! The headless query endpoint (ADR-0026, rule 6): requests in, exactly one result each, and a bad
//! request isolated from the rest.

use prost::Message;
use yata_core::scheme::code::{SchemeCode, StrengtheningPlan, StrengtheningSchemeSet, encode_code};
use yata_core::scheme::layout::{AccountSegment, Record, SchemeLayout, serialize};
use yata_core::scheme::selection::{SetChoice, SoulSelection};
use yata_core::scheme::transport::encode_text;
use yata_core::soul::SoulSlot as DomainSlot;
use yata_daemon::query::{MAX_ROW_BUDGET, ServeError, handle, respond, serve};
use yata_protocol::core::evaluate_query_result::Outcome;
use yata_protocol::core::expr::Kind;
use yata_protocol::core::predicate::Test;
use yata_protocol::core::{
    Collection, Direction, EvaluateQuery, EvaluateQueryResult, Expr, ExprList, Field, FieldName,
    InTest, Innate, IntRange, NumberRange, PageRequest, ParamSetRef, Predicate, ProtocolVersion,
    Query, QueryPage, SchemeRef, SortKey, Soul, SoulAttribute, SoulSelection as WireSelection,
    SoulSlot, SubAttribute, VERSION, innate,
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
        innate: Some(Innate {
            state: Some(innate::State::Absent(true)),
        }),
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

fn field(name: FieldName) -> Option<Field> {
    Some(Field {
        name: name as i32,
        attribute: 0,
    })
}

fn star_at_least(min: i64) -> Expr {
    node(Kind::Pred(Predicate {
        field: field(FieldName::Star),
        test: Some(Test::IntRange(IntRange {
            min: Some(min),
            max: None,
        })),
    }))
}

fn query(filter: Option<Expr>) -> Query {
    Query {
        collection: Collection::Souls as i32,
        filter,
        sort: Vec::new(),
        params: None,
        page: None,
        ..Query::default()
    }
}

fn request(id: u64, q: Query) -> EvaluateQuery {
    EvaluateQuery {
        id,
        protocol_version: Some(VERSION),
        inventory: inventory(),
        query: Some(q),
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
        Some(Outcome::Error(e)) => e.code.clone(),
        other => panic!("expected an error, got {other:?}"),
    }
}

fn ids(p: &QueryPage) -> Vec<&str> {
    p.rows.iter().map(|r| r.soul_id.as_str()).collect()
}

#[test]
fn a_query_over_a_supplied_inventory_returns_its_rows_in_identity_order() {
    let r = request(7, query(Some(star_at_least(5))));
    let result = ask(&r);
    assert_eq!(result.id, 7);
    let Some(Outcome::Page(p)) = result.outcome else {
        panic!("a page")
    };
    assert_eq!(ids(&p), ["s1", "s3", "s4", "s5"]);
    assert!(!p.has_more && p.cursor.is_empty());
    assert!(p.rows.iter().all(|r| r.open_rules.is_empty()));
}

#[test]
fn the_same_rows_in_any_order_give_the_same_bytes() {
    let mut q = query(None);
    q.sort = vec![SortKey {
        field: Some(Field {
            name: FieldName::SubValue as i32,
            attribute: SoulAttribute::Crit as i32,
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
        field: field(FieldName::Star),
        direction: Direction::Desc as i32,
    }];
    let mut seen = Vec::new();
    let mut cursor = Vec::new();
    loop {
        q.page = Some(PageRequest {
            row_budget: Some(2),
            cursor: cursor.clone(),
        });
        let p = page(&request(1, q.clone()));
        seen.extend(ids(&p).into_iter().map(str::to_owned));
        if !p.has_more {
            break;
        }
        cursor = p.cursor;
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
            StrengtheningPlan::new("a", slot2),
            StrengtheningPlan::new("b", slot4),
        ],
    });
    let code = text(&encode_code(&set, account()).expect("valid"));
    // Matches: every soul is in slot 2.
    let p = page(&request(1, query(Some(scheme(code.clone(), Some(0))))));
    assert_eq!(p.rows.len(), 5);
    assert!(p.rows.iter().all(|r| r.open_rules.is_empty()));
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
    let unknown_conditions = yata_protocol::core::OpenRule::UnknownConditions as i32;
    assert!(p.rows.iter().all(|r| r.open_rules == [unknown_conditions]));
}

#[test]
fn an_inline_selection_is_converted_and_evaluated() {
    let selection = WireSelection {
        suit_codes: vec![30],
        stars: vec![6],
        sub_included: vec![SoulAttribute::Crit as i32],
        ..WireSelection::default()
    };
    let p = page(&request(1, query(Some(node(Kind::Matches(selection))))));
    assert_eq!(ids(&p), ["s1", "s3", "s5"]);
    let both = WireSelection {
        any_set: true,
        sub_included: vec![SoulAttribute::Crit as i32],
        sub_excluded: vec![SoulAttribute::Crit as i32],
        ..WireSelection::default()
    };
    assert_eq!(
        code(&request(1, query(Some(node(Kind::Matches(both)))))),
        "query.malformed"
    );
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
            query(Some(node(Kind::Pred(Predicate {
                field: Some(Field {
                    name: 99,
                    attribute: 0,
                }),
                test: Some(Test::Is(true)),
            })))),
            "query.unknown_field",
        ),
        (
            query(Some(node(Kind::Pred(Predicate {
                field: field(FieldName::Star),
                test: None,
            })))),
            "query.malformed",
        ),
        (
            query(Some(node(Kind::Pred(Predicate {
                field: field(FieldName::Slot),
                test: Some(Test::In(InTest {
                    slots: vec![42],
                    ..InTest::default()
                })),
            })))),
            "query.malformed",
        ),
        (
            query(Some(node(Kind::Pred(Predicate {
                field: field(FieldName::SubValue),
                test: Some(Test::NumberRange(NumberRange {
                    min: Some(1.0),
                    max: None,
                })),
            })))),
            "query.malformed",
        ),
        (
            query(Some(node(Kind::Pred(Predicate {
                field: field(FieldName::Star),
                test: Some(Test::Is(true)),
            })))),
            "query.type_mismatch",
        ),
        (
            query(Some(node(Kind::Pred(Predicate {
                field: field(FieldName::Set),
                test: Some(Test::In(InTest::default())),
            })))),
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
fn a_tree_past_the_decoders_recursion_limit_is_malformed_not_a_crash() {
    let result = handle(&request(9, query(Some(nest(500)))).encode_to_vec());
    assert_eq!(
        result.id, 0,
        "the request did not decode, so its id is unknown"
    );
    assert_eq!(error_code(&result), "query.malformed");
}

#[test]
fn bytes_that_are_not_a_request_are_malformed() {
    assert_eq!(error_code(&handle(&[0xff, 0xff, 0xff])), "query.malformed");
}

#[test]
fn score_fields_are_refused_until_pass_one_exists() {
    let q = query(Some(node(Kind::Pred(Predicate {
        field: field(FieldName::QualityTotal),
        test: Some(Test::NumberRange(NumberRange {
            min: Some(50.0),
            max: None,
        })),
    }))));
    assert_eq!(code(&request(1, q.clone())), "query.param_set_required");
    let with_params = Query {
        params: Some(ParamSetRef {
            id: "yata-quality".into(),
            version: 1,
        }),
        ..q
    };
    assert_eq!(code(&request(1, with_params)), "query.field_unavailable");
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
    let budget = |b| {
        let mut q = query(None);
        q.page = Some(PageRequest {
            row_budget: Some(b),
            cursor: Vec::new(),
        });
        code(&request(1, q))
    };
    assert_eq!(budget(0), "query.malformed");
    assert_eq!(budget(MAX_ROW_BUDGET + 1), "query.too_complex");
    let mut foreign = query(None);
    foreign.page = Some(PageRequest {
        row_budget: None,
        cursor: vec![0xff, 0x01],
    });
    assert_eq!(code(&request(1, foreign)), "query.malformed_cursor");
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
