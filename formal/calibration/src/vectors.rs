//! The canonical test vectors: future golden tests for `yata-core`'s scorer.

use crate::model::attr::*;
use crate::standard::{Line, SoulInput};

fn l(attr: usize, value: &'static str, hits: u8) -> Line {
    Line { attr, value, hits }
}

pub fn all() -> Vec<SoulInput> {
    vec![
        SoulInput {
            id: "V01",
            what: "obviously poor: flat lines and one EffectRes increment, useful to no v2 archetype (resist, 17.07, in v1.1)",
            slot: 1,
            main: ATK_FLAT,
            level: 15,
            lines: vec![
                l(ATK_FLAT, "72.0", 3),
                l(HP_FLAT, "205.0", 2),
                l(DEF_FLAT, "13.5", 3),
                l(EFF_RES, "3.6", 1),
            ],
        },
        SoulInput {
            id: "V02",
            what: "ordinary useful: three useful increments",
            slot: 3,
            main: DEF_FLAT,
            level: 15,
            lines: vec![
                l(CRIT, "5.4", 2),
                l(ATK_PCT, "2.7", 1),
                l(HP_FLAT, "300.0", 3),
                l(DEF_FLAT, "13.8", 3),
            ],
        },
        SoulInput {
            id: "V03",
            what: "strong balanced SSR, no specialized line",
            slot: 6,
            main: CRIT_DMG,
            level: 15,
            lines: vec![
                l(CRIT, "8.4", 3),
                l(SPD, "5.4", 2),
                l(ATK_PCT, "5.4", 2),
                l(HP_FLAT, "205.0", 2),
            ],
        },
        SoulInput {
            id: "V04",
            what: "one perfect line, nothing useful beside it: SSR, not SP (SR in v1)",
            slot: 1,
            main: ATK_FLAT,
            level: 15,
            lines: vec![
                l(CRIT, "18.0", 6),
                l(HP_FLAT, "114.0", 1),
                l(DEF_FLAT, "5.0", 1),
                l(ATK_FLAT, "27.0", 1),
            ],
        },
        SoulInput {
            id: "V05",
            what: "valid SP",
            slot: 5,
            main: HP_FLAT,
            level: 15,
            lines: vec![
                l(CRIT, "16.5", 6),
                l(CRIT_DMG, "3.6", 1),
                l(ATK_PCT, "2.7", 1),
                l(HP_FLAT, "100.0", 1),
            ],
        },
        SoulInput {
            id: "V06",
            what: "near SP: five roll units exactly is not specialized",
            slot: 5,
            main: HP_FLAT,
            level: 15,
            lines: vec![
                l(CRIT, "15.0", 5),
                l(CRIT_DMG, "3.6", 1),
                l(ATK_PCT, "2.7", 1),
                l(HP_FLAT, "200.0", 2),
            ],
        },
        SoulInput {
            id: "V07",
            what: "specialized, below the SP floor: SSR, not SP (SR in v1)",
            slot: 1,
            main: ATK_FLAT,
            level: 15,
            lines: vec![
                l(CRIT, "15.3", 6),
                l(ATK_PCT, "3.0", 1),
                l(DEF_FLAT, "4.5", 1),
                l(HP_FLAT, "100.0", 1),
            ],
        },
        SoulInput {
            id: "V08",
            what: "valid UR: the paper's theoretical output soul",
            slot: 6,
            main: CRIT_DMG,
            level: 15,
            lines: vec![
                l(CRIT, "18.0", 6),
                l(CRIT_DMG, "4.0", 1),
                l(ATK_PCT, "3.0", 1),
                l(SPD, "3.0", 1),
            ],
        },
        SoulInput {
            id: "V09",
            what: "valid UR without a specialized line",
            slot: 2,
            main: SPD,
            level: 15,
            lines: vec![
                l(CRIT, "8.7", 3),
                l(CRIT_DMG, "7.8", 2),
                l(ATK_PCT, "5.8", 2),
                l(SPD, "5.8", 2),
            ],
        },
        SoulInput {
            id: "V10",
            what: "near UR: eight useful increments at maximum, one wasted",
            slot: 6,
            main: CRIT_DMG,
            level: 15,
            lines: vec![
                l(CRIT, "18.0", 6),
                l(CRIT_DMG, "4.0", 1),
                l(ATK_PCT, "3.0", 1),
                l(HP_FLAT, "114.0", 1),
            ],
        },
        SoulInput {
            id: "V11",
            what: "Speed-focused: UR under speed, SP under hit (hit SP in v1.1)",
            slot: 2,
            main: SPD,
            level: 15,
            lines: vec![
                l(SPD, "17.4", 6),
                l(EFF_HIT, "3.6", 1),
                l(HP_PCT, "2.7", 1),
                l(DEF_FLAT, "4.5", 1),
            ],
        },
        SoulInput {
            id: "V12",
            what: "Crit-focused",
            slot: 6,
            main: CRIT_DMG,
            level: 15,
            lines: vec![
                l(CRIT, "17.4", 6),
                l(ATK_PCT, "2.7", 1),
                l(SPD, "2.7", 1),
                l(DEF_FLAT, "4.5", 1),
            ],
        },
        SoulInput {
            id: "V13",
            what: "CritDmg-focused: the counterpart of V12",
            slot: 6,
            main: CRIT,
            level: 15,
            lines: vec![
                l(CRIT_DMG, "23.2", 6),
                l(ATK_PCT, "2.7", 1),
                l(SPD, "2.7", 1),
                l(HP_FLAT, "110.0", 1),
            ],
        },
        SoulInput {
            id: "V14",
            what: "dominated: V15 improves its Crit",
            slot: 4,
            main: ATK_PCT,
            level: 15,
            lines: vec![
                l(CRIT, "8.1", 3),
                l(SPD, "5.4", 2),
                l(ATK_PCT, "2.7", 1),
                l(HP_FLAT, "300.0", 3),
            ],
        },
        SoulInput {
            id: "V15",
            what: "dominates V14",
            slot: 4,
            main: ATK_PCT,
            level: 15,
            lines: vec![
                l(CRIT, "8.4", 3),
                l(SPD, "5.4", 2),
                l(ATK_PCT, "2.7", 1),
                l(HP_FLAT, "300.0", 3),
            ],
        },
        SoulInput {
            id: "V16",
            what: "rare main, useless lines: a rare main earns nothing",
            slot: 4,
            main: EFF_HIT,
            level: 15,
            lines: vec![
                l(ATK_FLAT, "72.0", 3),
                l(HP_FLAT, "205.0", 2),
                l(DEF_FLAT, "13.5", 3),
                l(CRIT_DMG, "3.6", 1),
            ],
        },
        SoulInput {
            id: "V17",
            what: "growth: +9, two rolls left",
            slot: 2,
            main: SPD,
            level: 9,
            lines: vec![l(SPD, "5.7", 2), l(CRIT, "5.6", 2), l(ATK_PCT, "2.8", 1)],
        },
        speed2(
            "V18",
            "speed gate, just below: 3.97 roll units, speed is not a candidate",
            "11.9",
            4,
        ),
        speed2(
            "V19",
            "speed gate, exactly four roll units: speed is a candidate",
            "12.0",
            5,
        ),
        speed2("V20", "speed gate, just above", "12.1", 5),
        speed2(
            "V21",
            "four roll units from four maximal increments: eligible by value, like V19",
            "12.0",
            4,
        ),
        speed2("V22", "strong Speed: 4.8 roll units", "14.4", 5),
        speed2(
            "V23",
            "speed's UR boundary: exactly five roll units is SSR",
            "15.0",
            6,
        ),
        SoulInput {
            id: "V24",
            what: "extreme Speed on an EffectRes main: UR under speed, its only candidate",
            slot: 4,
            main: EFF_RES,
            level: 15,
            lines: vec![
                l(SPD, "16.2", 6),
                l(HP_FLAT, "100.0", 1),
                l(DEF_FLAT, "4.5", 1),
                l(ATK_FLAT, "25.0", 1),
            ],
        },
        speed2(
            "V25",
            "the theoretical maximum Speed line: 100 under speed",
            "18.0",
            6,
        ),
        SoulInput {
            id: "V26",
            what: "unrated: an EffectRes main with Speed below the gate has no candidate archetype",
            slot: 4,
            main: EFF_RES,
            level: 15,
            lines: vec![
                l(SPD, "8.1", 3),
                l(CRIT, "5.4", 2),
                l(HP_PCT, "2.7", 1),
                l(DEF_FLAT, "5.0", 1),
            ],
        },
        SoulInput {
            id: "V27",
            what: "healing: SSR under healing, R under output on the same soul",
            slot: 6,
            main: CRIT,
            level: 15,
            lines: vec![
                l(HP_PCT, "8.4", 3),
                l(CRIT_DMG, "7.6", 2),
                l(SPD, "5.7", 2),
                l(DEF_FLAT, "5.0", 1),
            ],
        },
        SoulInput {
            id: "V28",
            what: "healing on an HpPercent main: SSR under healing, SR under hit",
            slot: 4,
            main: HP_PCT,
            level: 15,
            lines: vec![
                l(HP_PCT, "11.1", 4),
                l(CRIT, "5.7", 2),
                l(SPD, "2.7", 1),
                l(ATK_FLAT, "27.0", 1),
            ],
        },
        SoulInput {
            id: "V29",
            what: "hit: EffectHit and Speed, equal weights",
            slot: 4,
            main: EFF_HIT,
            level: 15,
            lines: vec![
                l(EFF_HIT, "13.6", 4),
                l(SPD, "5.4", 2),
                l(HP_PCT, "2.7", 1),
                l(DEF_FLAT, "5.0", 1),
            ],
        },
        SoulInput {
            id: "V30",
            what: "growth under speed: +12, eligible now, one roll left",
            slot: 2,
            main: SPD,
            level: 12,
            lines: vec![l(SPD, "13.5", 5), l(HP_PCT, "2.5", 1)],
        },
    ]
}

/// A slot-2 Speed-main soul whose only useful line for `speed` is the given Speed line, beside
/// one HpPercent and one DefPercent increment and a flat line.
fn speed2(id: &'static str, what: &'static str, spd: &'static str, hits: u8) -> SoulInput {
    SoulInput {
        id,
        what,
        slot: 2,
        main: SPD,
        level: 15,
        lines: vec![
            l(SPD, spd, hits),
            l(HP_PCT, "2.7", 1),
            l(DEF_PCT, "2.7", 1),
            l(ATK_FLAT, "27.0", 1),
        ],
    }
}
