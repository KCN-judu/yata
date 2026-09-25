/// A soul set (套装), identified by the game's suit code: the id every soul record carries,
/// `suitId = 300000 + suit code` (`scheme-code.md`, "SoulSet identity is the game's suit code").
///
/// The code is the identity and nothing else is: the scheme bit and the UI order are mappings
/// from it, never the reverse. Any code can be held; whether the game uses it is a question for
/// the mapping that consumes it, which refuses a code it does not know.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SoulSet(u8);

impl SoulSet {
    pub const fn from_suit_code(code: u8) -> SoulSet {
        SoulSet(code)
    }

    pub const fn suit_code(self) -> u8 {
        self.0
    }

    /// The set a name, as the game shows it, names: `None` for a name no set has.
    pub fn from_name(name: &str) -> Option<SoulSet> {
        NAMES.iter().find(|(_, n)| *n == name).map(|&(s, _)| s)
    }

    /// The set's name as the game shows it, or `None` for a code no named set has.
    pub fn name(self) -> Option<&'static str> {
        NAMES.iter().find(|(s, _)| *s == self).map(|&(_, n)| n)
    }
}

/// Every set's name, by suit code. The names follow the game's spelling, and each is the soul the
/// game shows for its scheme bit, confirmed by import (`scheme::mapping`); the code beside it is
/// inherited from the prior tool (ADR-0014).
const NAMES: [(SoulSet, &str); 70] = [
    (SoulSet(2), "雪幽魂"),
    (SoulSet(3), "地藏像"),
    (SoulSet(4), "蝠翼"),
    (SoulSet(6), "涅槃之火"),
    (SoulSet(7), "三味"),
    (SoulSet(8), "魍魉之匣"),
    (SoulSet(9), "被服"),
    (SoulSet(10), "招财猫"),
    (SoulSet(11), "反枕"),
    (SoulSet(12), "轮入道"),
    (SoulSet(13), "日女巳时"),
    (SoulSet(14), "镜姬"),
    (SoulSet(15), "钟灵"),
    (SoulSet(18), "狰"),
    (SoulSet(19), "火灵"),
    (SoulSet(20), "鸣屋"),
    (SoulSet(21), "薙魂"),
    (SoulSet(22), "心眼"),
    (SoulSet(23), "木魅"),
    (SoulSet(24), "树妖"),
    (SoulSet(26), "网切"),
    (SoulSet(27), "阴摩罗"),
    (SoulSet(29), "伤魂鸟"),
    (SoulSet(30), "破势"),
    (SoulSet(31), "镇墓兽"),
    (SoulSet(32), "珍珠"),
    (SoulSet(33), "骰子鬼"),
    (SoulSet(34), "蚌精"),
    (SoulSet(35), "魅妖"),
    (SoulSet(36), "针女"),
    (SoulSet(39), "返魂香"),
    (SoulSet(48), "狂骨"),
    (SoulSet(49), "幽谷响"),
    (SoulSet(50), "土蜘蛛"),
    (SoulSet(51), "胧车"),
    (SoulSet(52), "荒骷髅"),
    (SoulSet(53), "地震鲶"),
    (SoulSet(54), "蜃气楼"),
    (SoulSet(55), "片叶之苇"),
    (SoulSet(56), "尘冢"),
    (SoulSet(57), "油赤子"),
    (SoulSet(58), "夜啼石"),
    (SoulSet(59), "夜送犬"),
    (SoulSet(60), "雨降"),
    (SoulSet(73), "飞缘魔"),
    (SoulSet(74), "兵主部"),
    (SoulSet(75), "青女房"),
    (SoulSet(76), "涂佛"),
    (SoulSet(77), "鬼灵歌伎"),
    (SoulSet(79), "遗念火"),
    (SoulSet(80), "共潜"),
    (SoulSet(81), "恶楼"),
    (SoulSet(82), "贝吹坊"),
    (SoulSet(83), "海月火玉"),
    (SoulSet(84), "出世螺"),
    (SoulSet(85), "火之车"),
    (SoulSet(86), "隐念"),
    (SoulSet(87), "叠叩"),
    (SoulSet(88), "应声虫"),
    (SoulSet(89), "元兴寺"),
    (SoulSet(90), "钓瓶火"),
    (SoulSet(91), "夜荒魂"),
    (SoulSet(92), "无刀取"),
    (SoulSet(93), "奉海图"),
    (SoulSet(94), "八咫镜"),
    (SoulSet(95), "天羽羽斩"),
    (SoulSet(96), "预言星盘"),
    (SoulSet(97), "月之石"),
    (SoulSet(98), "纺缘锤"),
    (SoulSet(99), "稻荷穗箭"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_codes_are_one_to_one() {
        for &(set, name) in &NAMES {
            assert_eq!(SoulSet::from_name(name), Some(set), "{name}");
            assert_eq!(set.name(), Some(name));
        }
        let mut codes: Vec<u8> = NAMES.iter().map(|(s, _)| s.suit_code()).collect();
        codes.dedup();
        assert_eq!(codes.len(), NAMES.len(), "codes are ascending and distinct");
        assert!(codes.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn a_name_no_set_has_is_refused() {
        assert_eq!(SoulSet::from_name("破 势"), None);
        assert_eq!(SoulSet::from_name(""), None);
        assert_eq!(SoulSet::from_suit_code(1).name(), None);
    }
}
