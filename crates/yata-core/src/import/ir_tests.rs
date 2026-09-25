use super::super::fixture::*;
use super::*;

#[test]
fn absent_and_empty_are_different_sections() {
    let absent = empty();
    let explicit = YataSnapshot {
        guild: complete(Guild {
            level: 1,
            member_count: 0,
        }),
        souls: complete(Souls::default()),
        ..empty()
    };
    assert_ne!(absent, explicit);
    assert!(absent.sections().is_empty());
    assert_eq!(
        explicit.sections(),
        vec![
            (SectionKind::Souls, Completeness::Complete),
            (SectionKind::Guild, Completeness::Complete)
        ]
    );
    assert_eq!(check(&explicit), Ok(()));
}

#[test]
fn a_text_is_non_empty_and_bounded() {
    assert_eq!(SourceId::new(""), Err(TextError::Empty));
    assert_eq!(
        SourceId::new(&"x".repeat(65)),
        Err(TextError::TooLong {
            chars: 65,
            limit: MAX_ID_CHARS
        })
    );
    assert!(SetName::new(&"魂".repeat(256)).is_ok());
}

#[test]
fn ids_are_unique_within_a_section() {
    assert_eq!(
        check(&with_souls(vec![soul("a", 1), soul("a", 2)])),
        Err(IrError::DuplicateId {
            section: SectionKind::Souls,
            id: id("a")
        })
    );
}

#[test]
fn a_preset_names_souls_of_its_own_snapshot() {
    let ok = YataSnapshot {
        presets: complete(GamePresets {
            presets: vec![preset([Some("a"), None, None, None, None, None])],
        }),
        ..with_souls(vec![soul("a", 1)])
    };
    assert_eq!(check(&ok), Ok(()));
    let broken = YataSnapshot {
        presets: complete(GamePresets {
            presets: vec![preset([None, Some("zz"), None, None, None, None])],
        }),
        ..with_souls(vec![soul("a", 1)])
    };
    assert_eq!(
        check(&broken),
        Err(IrError::InconsistentReference {
            preset: 0,
            position: 1,
            soul: id("zz")
        })
    );
    let alone = YataSnapshot {
        presets: complete(GamePresets::default()),
        ..empty()
    };
    assert_eq!(check(&alone), Err(IrError::PresetsWithoutSouls));
}

#[test]
fn each_currency_appears_once_and_limits_hold() {
    let twice = YataSnapshot {
        assets: complete(Assets {
            currencies: vec![(Currency::Jade, 1), (Currency::Jade, 2)],
            realm_cards: vec![],
        }),
        ..empty()
    };
    assert_eq!(
        check(&twice),
        Err(IrError::DuplicateCurrency(Currency::Jade))
    );
    let many = with_souls(
        (0..=limits::SOULS)
            .map(|i| soul(&format!("s{i}"), 1))
            .collect(),
    );
    assert_eq!(
        check(&many),
        Err(IrError::TooMany {
            section: SectionKind::Souls,
            count: limits::SOULS + 1,
            limit: limits::SOULS
        })
    );
}
