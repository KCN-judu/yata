//! The Chinese edition of the generated blocks, for `paper.zh.md`: the same numbers, with the
//! table headings and fixed phrases translated. Identifiers — archetype keys, attribute names,
//! symbols — stay as they are, as the Chinese paper writes them.

/// Phrases and their Chinese, applied longest first so that a phrase inside a longer one is
/// never translated on its own.
const PHRASES: &[(&str, &str)] = &[
    // constants
    ("Parameter set", "参数集"),
    (
        "`. Reference measure: the official class weights, equal within a class. Exact values: ",
        "`；参考测度：官方类别权重，类内均分；精确值见 ",
    ),
    ("constants.json`.", "constants.json`。"),
    (
        "| Milestone | Roll units | Meaning |",
        "| 里程碑 | 强化单位 | 含义 |",
    ),
    ("mean increment, roll units", "一次增量的均值，以强化单位计"),
    (
        "attainable maximum of a four-attribute archetype",
        "四属性原型的可达上限",
    ),
    (
        "SR: five useful increments at mean value, 5μ",
        "SR：五次平均值的有用增量，5μ",
    ),
    (
        "SSR: the most one useful line can hold, six increments at maximum",
        "SSR：一条有用副属性所能达到的最多，六次最大增量",
    ),
    (
        "SP's quality floor: seven useful increments at mean value, 7μ",
        "SP 的品质下限：七次平均值的有用增量，7μ",
    ),
    ("UR: above M − 1", "UR：高于 M − 1"),
    (
        "attainable maximum of speed: one line, six increments at maximum",
        "speed 的可达上限：一条副属性，六次最大增量",
    ),
    (
        "speed is a candidate only when e_Spd ≥ 4",
        "仅当 e_Spd ≥ 4 时 speed 为候选",
    ),
    (
        "speed's UR: above M_speed − 1",
        "speed 的 UR：高于 M_speed − 1",
    ),
    ("score at the gate, g(4)", "门槛处分数 g(4)"),
    ("| Archetype |", "| 原型 |"),
    // k-law, tier rates, sensitivity
    ("P(K), output and healing", "P(K)，output 与 healing"),
    ("P(K), hit", "P(K)，hit"),
    ("P(K), speed", "P(K)，speed"),
    ("| Increment law |", "| 增量分布律 |"),
    ("continuous uniform on [0.8, 1]", "[0.8, 1] 上连续均匀"),
    ("7 equal steps on [0.8, 1]", "[0.8, 1] 上 7 个等距值"),
    ("2 equal steps on [0.8, 1]", "[0.8, 1] 上 2 个等距值"),
    ("output, hit, healing, speed", "output、hit、healing、speed"),
    ("hit, healing, speed", "hit、healing、speed"),
    ("output, healing", "output、healing"),
    ("hit, speed", "hit、speed"),
    ("| Measure |", "| 测度 |"),
    (
        "36/36/28 per class (official notice; the reference)",
        "类别 36/36/28（官方公告；参考测度）",
    ),
    (
        "1/11 per attribute (Hu 2026 reading)",
        "每属性 1/11（Hu 2026 的读法）",
    ),
    (
        "official weights; initial count 1/2, 1/3, 1/6",
        "官方权重；初始条数 1/2、1/3、1/6",
    ),
    ("E, output and healing", "E，output 与 healing"),
    ("E, hit", "E，hit"),
    ("E, speed", "E，speed"),
    ("P(eligible)", "P(合格)"),
    ("| score V", "| 分数 V"),
    ("0.8 (all rolls minimal)", "0.8（每次均最小）"),
    ("0.9 (the standard: midpoint)", "0.9（本标准：中点）"),
    ("1.0 (all rolls maximal)", "1.0（每次均最大）"),
    // speed rates
    (
        "Of all +15 souls, under the reference measure; speed has no N, R, SR or SP.",
        "占全部 +15 御魂的比例，参考测度下；speed 没有 N、R、SR 或 SP。",
    ),
    ("eligible: 1 in", "合格：约每 n 个中 1 个"),
    (
        "Of eligible souls: where e_Spd lies, the speed score and tier there, and how many increments the Speed line has.",
        "合格御魂中：e_Spd 所在的区间、该处的 speed 分数与档位，以及速度副属性的增量次数。",
    ),
    ("4 increments", "4 次增量"),
    ("5 increments", "5 次增量"),
    ("6 increments", "6 次增量"),
    ("| speed score |", "| speed 分数 |"),
    ("| tier |", "| 档位 |"),
    (
        "The reference of speed's normalization, uniform increments: A, the standard, is unconditional; B would condition it on the gate.",
        "speed 归一化的参考，均匀增量：A 为本标准，无条件；B 以门槛为条件。",
    ),
    ("| Reference |", "| 参考 |"),
    ("score at e_Spd = 4", "e_Spd = 4 处分数"),
    ("eligible souls scored below 50", "合格御魂中低于 50 分者"),
    ("A. unconditional (the standard)", "A. 无条件（本标准）"),
    ("B. conditioned on e_Spd ≥ 4", "B. 以 e_Spd ≥ 4 为条件"),
    // law robustness
    ("| Soul |", "| 御魂 |"),
    (
        "anchored score (any law with mean 0.9)",
        "锚定分数（任意均值为 0.9 的分布律）",
    ),
    ("percentile, uniform", "百分位，均匀"),
    ("percentile, 7 steps", "百分位，7 个等距值"),
    ("percentile, 2 steps", "百分位，2 个等距值"),
    ("P(equal or better), uniform", "P(同等或更好)，均匀"),
    // Monte Carlo
    (" souls per row, seed ", " 个御魂每行，种子 "),
    (
        ", uniform increments, the reference measure. Each cell: rate (standard error), and its z-score against the exact rate. For speed, \"not eligible\" is e_Spd < 4.",
        "，均匀增量，参考测度。每格：比例（标准误），及其相对精确比例的 z 分数。对 speed，“不合格”即 e_Spd < 4。",
    ),
    ("| Row |", "| 行 |"),
    ("not eligible", "不合格"),
    ("| exact, ", "| 精确，"),
    ("| Monte Carlo, ", "| 蒙特卡罗，"),
    (
        "Largest |z| of a Monte Carlo rate against the exact rate:",
        "蒙特卡罗比例相对精确比例的最大 |z|：",
    ),
    ("Quality over the candidate archetypes", "候选原型上的品质"),
    ("| Accepting |", "| 接受的原型 |"),
    ("| unrated |", "| 不评级 |"),
    ("slots 1, 3, 5; Spd on 2", "1、3、5 号位；2 号位 Spd"),
    ("AtkPercent on 2, 4, 6", "2、4、6 号位 AtkPercent"),
    ("Crit, CritDmg on 6", "6 号位 Crit、CritDmg"),
    ("HpPercent on 2, 4, 6", "2、4、6 号位 HpPercent"),
    (
        "EffectHit on 4; DefPercent on 2, 4, 6",
        "4 号位 EffectHit；2、4、6 号位 DefPercent",
    ),
    ("EffectRes on 4", "4 号位 EffectRes"),
    // normalization
    (
        " souls, seed 0x4E4F524D, uniform increments; the same souls for every profile. Each cell: share of souls scoring at or above the output archetype's c_R / c_SR / c_SSR / above its t_UR, and the mean score.",
        " 个御魂，种子 0x4E4F524D，均匀增量；各属性组用同一批御魂。每格：分数不低于 output 原型的 c_R / c_SR / c_SSR、或高于其 t_UR 的御魂比例，以及平均分。",
    ),
    ("| Normalization |", "| 归一化 |"),
    ("| Profile |", "| 属性组 |"),
    ("| mean |", "| 平均分 |"),
    ("A. bound: 100·U/M", "A. 上限比：100·U/M"),
    (
        "C. mean ratio: min(100, 50·U/E)",
        "C. 期望比：min(100, 50·U/E)",
    ),
    ("H. anchored (the standard)", "H. 锚定（本标准）"),
    ("B. percentile (mid-rank)", "B. 百分位（中位秩）"),
    ("speed only (1 attribute)", "仅速度（1 个属性）"),
    ("crit pair (2 attributes)", "暴击对（2 个属性）"),
    ("output archetype (4 attributes)", "output 原型（4 个属性）"),
    // SSR candidates
    (
        "SP's floor stays 7μ = 6.3 and UR's boundary 8 in every row; only SSR's edge moves. The four-attribute archetypes of v2; speed has no SSR edge of this kind.",
        "每行中 SP 下限保持 7μ = 6.3、UR 边界保持 8，只有 SSR 边界移动。v2 的四属性原型；speed 没有这种 SSR 边界。",
    ),
    ("| Candidate |", "| 候选 |"),
    ("SSR or better", "SSR 及以上"),
    ("A. v1: 7μ", "A. v1：7μ"),
    ("B. v1.1: one perfect line", "B. v1.1：一条完美副属性"),
    (
        "D. SSR or better at 5% (uniform, output)",
        "D. SSR 及以上占 5%（均匀，output）",
    ),
    // vectors
    (
        "| Id | Soul | Slot, main, level | Best fit | U | Score | Specialized | Tier | Depth | Breadth | Growth | Every candidate |",
        "| 编号 | 御魂 | 位置、主属性、等级 | 最佳适配 | U | 分数 | 专精 | 档位 | 深度 | 广度 | 成长 | 全部候选 |",
    ),
    ("| Id | What it shows |", "| 编号 | 展示的性质 |"),
    ("| yes |", "| 是 |"),
    ("| no |", "| 否 |"),
    ("| none |", "| 无 |"),
    ("| n/a |", "| 不适用 |"),
];

/// What each golden vector shows, in Chinese.
const WHAT: &[(&str, &str)] = &[
    (
        "obviously poor: flat lines and one EffectRes increment, useful to no v2 archetype (resist, 17.07, in v1.1)",
        "明显很差：固定值副属性与一次 EffectRes 增量，对 v2 的任何原型都无用（v1.1 中为 resist，17.07）",
    ),
    (
        "ordinary useful: three useful increments",
        "普通有用：三次有用增量",
    ),
    (
        "strong balanced SSR, no specialized line",
        "强而均衡的 SSR，无专精副属性",
    ),
    (
        "one perfect line, nothing useful beside it: SSR, not SP (SR in v1)",
        "一条完美副属性、别无有用数值：SSR 而非 SP（v1 中为 SR）",
    ),
    ("valid SP", "有效的 SP"),
    (
        "near SP: five roll units exactly is not specialized",
        "接近 SP：恰好五个强化单位不算专精",
    ),
    (
        "specialized, below the SP floor: SSR, not SP (SR in v1)",
        "专精但低于 SP 下限：SSR 而非 SP（v1 中为 SR）",
    ),
    (
        "valid UR: the paper's theoretical output soul",
        "有效的 UR：分析文献的理论输出御魂",
    ),
    (
        "valid UR without a specialized line",
        "没有专精副属性的有效 UR",
    ),
    (
        "near UR: eight useful increments at maximum, one wasted",
        "接近 UR：八次最大有用增量，一次浪费",
    ),
    (
        "Speed-focused: UR under speed, SP under hit (hit SP in v1.1)",
        "偏重速度：speed 下为 UR，hit 下为 SP（v1.1 中为 hit SP）",
    ),
    (
        "CritDmg-focused: the counterpart of V12",
        "偏重暴击伤害：与 V12 对应",
    ),
    ("Crit-focused", "偏重暴击"),
    (
        "dominated: V15 improves its Crit",
        "被占优：V15 提高了其暴击",
    ),
    ("dominates V14", "占优于 V14"),
    (
        "rare main, useless lines: a rare main earns nothing",
        "稀有主属性、无用副属性：稀有主属性本身不得分",
    ),
    ("growth: +9, two rolls left", "成长：+9，余两次强化"),
    (
        "speed gate, just below: 3.97 roll units, speed is not a candidate",
        "速度门槛之下：3.97 个强化单位，speed 不是候选",
    ),
    (
        "speed gate, exactly four roll units: speed is a candidate",
        "速度门槛处：恰好四个强化单位，speed 是候选",
    ),
    ("speed gate, just above", "速度门槛之上"),
    (
        "four roll units from four maximal increments: eligible by value, like V19",
        "由四次最大增量构成的四个强化单位：按数值合格，与 V19 相同",
    ),
    ("strong Speed: 4.8 roll units", "强速度：4.8 个强化单位"),
    (
        "speed's UR boundary: exactly five roll units is SSR",
        "speed 的 UR 边界：恰好五个强化单位为 SSR",
    ),
    (
        "extreme Speed on an EffectRes main: UR under speed, its only candidate",
        "EffectRes 主属性上的极端速度：speed 下为 UR，speed 是其唯一候选",
    ),
    (
        "the theoretical maximum Speed line: 100 under speed",
        "理论最大速度副属性：speed 下为 100 分",
    ),
    (
        "unrated: an EffectRes main with Speed below the gate has no candidate archetype",
        "不评级：速度低于门槛的 EffectRes 主属性御魂没有候选原型",
    ),
    (
        "healing: SSR under healing, R under output on the same soul",
        "治疗：同一御魂在 healing 下为 SSR，在 output 下为 R",
    ),
    (
        "healing on an HpPercent main: SSR under healing, SR under hit",
        "HpPercent 主属性上的治疗：healing 下为 SSR，hit 下为 SR",
    ),
    (
        "hit: EffectHit and Speed, equal weights",
        "命中：EffectHit 与速度，等权",
    ),
    (
        "growth under speed: +12, eligible now, one roll left",
        "speed 下的成长：+12，当前已合格，余一次强化",
    ),
];

/// A generated block in Chinese.
pub fn zh(text: &str) -> String {
    let mut all: Vec<&(&str, &str)> = WHAT.iter().chain(PHRASES).collect();
    all.sort_by_key(|(en, _)| std::cmp::Reverse(en.len()));
    let mut s = text.to_owned();
    for (en, zh) in all {
        s = s.replace(en, zh);
    }
    // "(1 in 65)" becomes "（约 1/65）"; "81.24 to 85.93" becomes "81.24 至 85.93".
    let mut out = String::with_capacity(s.len());
    let mut rest = s.as_str();
    while let Some(i) = rest.find(" (1 in ") {
        out.push_str(&rest[..i]);
        let tail = &rest[i + " (1 in ".len()..];
        let j = tail.find(')').unwrap_or(tail.len());
        out.push_str("（约 1/");
        out.push_str(&tail[..j]);
        out.push('）');
        rest = tail.get(j + 1..).unwrap_or("");
    }
    out.push_str(rest);
    out.replace(" to ", " 至 ")
}

/// Every English phrase a Chinese block may still hold, so that a new heading or note cannot
/// reach the Chinese paper untranslated.
pub fn untranslated(text: &str) -> Vec<String> {
    const KEEP: &[&str] = &[
        "output",
        "hit",
        "healing",
        "speed",
        "AtkPercent",
        "AtkFlat",
        "DefPercent",
        "DefFlat",
        "HpPercent",
        "HpFlat",
        "Spd",
        "EffectHit",
        "EffectRes",
        "Crit",
        "CritDmg",
        "yata",
        "quality",
        "formal",
        "calibration",
        "constants",
        "json",
        "out",
        "SSR",
        "floor",
        "gate",
        "min",
        "resist",
    ];
    text.split(|c: char| !c.is_ascii_alphabetic())
        .filter(|w| w.len() >= 3 && !KEEP.contains(w))
        .map(str::to_owned)
        .collect()
}
