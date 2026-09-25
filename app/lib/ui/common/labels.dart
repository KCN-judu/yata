/// The Chinese names of the protocol's enum values, and how a value is written.
///
/// Presentation only: every value here comes from the daemon, and this file only chooses the
/// words for it. A set is shown by its suit code until the core supplies set names; there is no
/// suit-code table in the application (ADR-0004, rule 11).
library;

import '../../gen/l10n/app_localizations.dart';
import '../../gen/proto/core.pb.dart' as pb;

String attributeName(AppLocalizations l, pb.SoulAttribute a) => switch (a) {
  pb.SoulAttribute.SOUL_ATTRIBUTE_ATK_FLAT => l.attrAtkFlat,
  pb.SoulAttribute.SOUL_ATTRIBUTE_ATK_PERCENT => l.attrAtkPercent,
  pb.SoulAttribute.SOUL_ATTRIBUTE_DEF_FLAT => l.attrDefFlat,
  pb.SoulAttribute.SOUL_ATTRIBUTE_DEF_PERCENT => l.attrDefPercent,
  pb.SoulAttribute.SOUL_ATTRIBUTE_HP_FLAT => l.attrHpFlat,
  pb.SoulAttribute.SOUL_ATTRIBUTE_HP_PERCENT => l.attrHpPercent,
  pb.SoulAttribute.SOUL_ATTRIBUTE_SPD => l.attrSpd,
  pb.SoulAttribute.SOUL_ATTRIBUTE_EFFECT_HIT => l.attrEffectHit,
  pb.SoulAttribute.SOUL_ATTRIBUTE_EFFECT_RES => l.attrEffectRes,
  pb.SoulAttribute.SOUL_ATTRIBUTE_CRIT => l.attrCrit,
  pb.SoulAttribute.SOUL_ATTRIBUTE_CRIT_DMG => l.attrCritDmg,
  _ => l.attrUnknown,
};

/// Attributes the game writes with a percent sign; values arrive in percentage points.
const _percent = {
  pb.SoulAttribute.SOUL_ATTRIBUTE_ATK_PERCENT,
  pb.SoulAttribute.SOUL_ATTRIBUTE_DEF_PERCENT,
  pb.SoulAttribute.SOUL_ATTRIBUTE_HP_PERCENT,
  pb.SoulAttribute.SOUL_ATTRIBUTE_EFFECT_HIT,
  pb.SoulAttribute.SOUL_ATTRIBUTE_EFFECT_RES,
  pb.SoulAttribute.SOUL_ATTRIBUTE_CRIT,
  pb.SoulAttribute.SOUL_ATTRIBUTE_CRIT_DMG,
};

/// A stored value as text: one decimal where it has one, and the attribute's unit. The stored
/// value is shown, not the game's rounded display value.
String attributeValue(pb.SoulAttribute a, double value) {
  final fixed = value.toStringAsFixed(1);
  final text = fixed.endsWith('.0') ? fixed.substring(0, fixed.length - 2) : fixed;
  return _percent.contains(a) ? '$text%' : text;
}

String setName(AppLocalizations l, int suitCode) => l.soulSet(suitCode);

/// The slot number, 1 to 6, of a slot value; 0 for an unknown one.
/// The slot's number, 1 to 6; `null` for a value this build does not know, never 0.
int? slotNumber(pb.SoulSlot k) => switch (k) {
  pb.SoulSlot.SOUL_SLOT_1 => 1,
  pb.SoulSlot.SOUL_SLOT_2 => 2,
  pb.SoulSlot.SOUL_SLOT_3 => 3,
  pb.SoulSlot.SOUL_SLOT_4 => 4,
  pb.SoulSlot.SOUL_SLOT_5 => 5,
  pb.SoulSlot.SOUL_SLOT_6 => 6,
  // Generated enums are classes, so no switch over them is exhaustive; unspecified and any
  // value of a newer daemon land here, as unknown.
  _ => null,
};

String slotName(AppLocalizations l, pb.SoulSlot k) => switch (slotNumber(k)) {
  final n? => l.soulSlot(n),
  null => l.valueUnknown,
};

String levelBandName(AppLocalizations l, pb.LevelBand b) => switch (b) {
  pb.LevelBand.LEVEL_BAND_0_TO_2 => l.levelBand0to2,
  pb.LevelBand.LEVEL_BAND_3_TO_5 => l.levelBand3to5,
  pb.LevelBand.LEVEL_BAND_6_TO_8 => l.levelBand6to8,
  pb.LevelBand.LEVEL_BAND_9_TO_11 => l.levelBand9to11,
  pb.LevelBand.LEVEL_BAND_12_TO_14 => l.levelBand12to14,
  pb.LevelBand.LEVEL_BAND_15 => l.levelBand15,
  _ => l.valueUnknown,
};

String subCountName(AppLocalizations l, pb.SubCount c) => switch (c) {
  pb.SubCount.SUB_COUNT_FEWER_THAN_TWO => l.subCountFewerThanTwo,
  pb.SubCount.SUB_COUNT_TWO => l.subCountTwo,
  pb.SubCount.SUB_COUNT_THREE => l.subCountThree,
  pb.SubCount.SUB_COUNT_FOUR => l.subCountFour,
  _ => l.valueUnknown,
};

String schemeKindName(AppLocalizations l, pb.SchemeKind k) => switch (k) {
  pb.SchemeKind.SCHEME_KIND_STRENGTHENING => l.schemeKindStrengthening,
  pb.SchemeKind.SCHEME_KIND_DISCARD => l.schemeKindDiscard,
  _ => l.valueUnknown,
};

/// A soul's 固有属性: a boss soul's attribute, or none for an ordinary soul (ADR-0029). A soul
/// without a kind is a gap in the wire, never shown as none (`core.proto`, `Soul.kind`).
String innateText(AppLocalizations l, pb.Soul s) => switch (s.whichKind()) {
  pb.Soul_Kind.boss => attributeName(l, s.boss.innate),
  pb.Soul_Kind.ordinary => l.labelNone,
  pb.Soul_Kind.notSet => l.valueUnknown,
};

/// The accessible name of a soul, reading its layers in order (PRP-0002, "The soul tile").
String soulAccessibleName(AppLocalizations l, pb.Soul s) =>
    l.soulAccessibleName(setName(l, s.suitCode), slotName(l, s.slot), l.soulStar(s.star), s.level);
