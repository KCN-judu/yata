// This is a generated file - do not edit.
//
// Generated from core.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class SoulSlot extends $pb.ProtobufEnum {
  static const SoulSlot SOUL_SLOT_UNSPECIFIED =
      SoulSlot._(0, _omitEnumNames ? '' : 'SOUL_SLOT_UNSPECIFIED');
  static const SoulSlot SOUL_SLOT_1 =
      SoulSlot._(1, _omitEnumNames ? '' : 'SOUL_SLOT_1');
  static const SoulSlot SOUL_SLOT_2 =
      SoulSlot._(2, _omitEnumNames ? '' : 'SOUL_SLOT_2');
  static const SoulSlot SOUL_SLOT_3 =
      SoulSlot._(3, _omitEnumNames ? '' : 'SOUL_SLOT_3');
  static const SoulSlot SOUL_SLOT_4 =
      SoulSlot._(4, _omitEnumNames ? '' : 'SOUL_SLOT_4');
  static const SoulSlot SOUL_SLOT_5 =
      SoulSlot._(5, _omitEnumNames ? '' : 'SOUL_SLOT_5');
  static const SoulSlot SOUL_SLOT_6 =
      SoulSlot._(6, _omitEnumNames ? '' : 'SOUL_SLOT_6');

  static const $core.List<SoulSlot> values = <SoulSlot>[
    SOUL_SLOT_UNSPECIFIED,
    SOUL_SLOT_1,
    SOUL_SLOT_2,
    SOUL_SLOT_3,
    SOUL_SLOT_4,
    SOUL_SLOT_5,
    SOUL_SLOT_6,
  ];

  static final $core.List<SoulSlot?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 6);
  static SoulSlot? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SoulSlot._(super.value, super.name);
}

/// In glossary order, which is the order a sort by attribute uses.
class SoulAttribute extends $pb.ProtobufEnum {
  static const SoulAttribute SOUL_ATTRIBUTE_UNSPECIFIED =
      SoulAttribute._(0, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_UNSPECIFIED');
  static const SoulAttribute SOUL_ATTRIBUTE_ATK_FLAT =
      SoulAttribute._(1, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_ATK_FLAT');
  static const SoulAttribute SOUL_ATTRIBUTE_ATK_PERCENT =
      SoulAttribute._(2, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_ATK_PERCENT');
  static const SoulAttribute SOUL_ATTRIBUTE_DEF_FLAT =
      SoulAttribute._(3, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_DEF_FLAT');
  static const SoulAttribute SOUL_ATTRIBUTE_DEF_PERCENT =
      SoulAttribute._(4, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_DEF_PERCENT');
  static const SoulAttribute SOUL_ATTRIBUTE_HP_FLAT =
      SoulAttribute._(5, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_HP_FLAT');
  static const SoulAttribute SOUL_ATTRIBUTE_HP_PERCENT =
      SoulAttribute._(6, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_HP_PERCENT');
  static const SoulAttribute SOUL_ATTRIBUTE_SPD =
      SoulAttribute._(7, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_SPD');
  static const SoulAttribute SOUL_ATTRIBUTE_EFFECT_HIT =
      SoulAttribute._(8, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_EFFECT_HIT');
  static const SoulAttribute SOUL_ATTRIBUTE_EFFECT_RES =
      SoulAttribute._(9, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_EFFECT_RES');
  static const SoulAttribute SOUL_ATTRIBUTE_CRIT =
      SoulAttribute._(10, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_CRIT');
  static const SoulAttribute SOUL_ATTRIBUTE_CRIT_DMG =
      SoulAttribute._(11, _omitEnumNames ? '' : 'SOUL_ATTRIBUTE_CRIT_DMG');

  static const $core.List<SoulAttribute> values = <SoulAttribute>[
    SOUL_ATTRIBUTE_UNSPECIFIED,
    SOUL_ATTRIBUTE_ATK_FLAT,
    SOUL_ATTRIBUTE_ATK_PERCENT,
    SOUL_ATTRIBUTE_DEF_FLAT,
    SOUL_ATTRIBUTE_DEF_PERCENT,
    SOUL_ATTRIBUTE_HP_FLAT,
    SOUL_ATTRIBUTE_HP_PERCENT,
    SOUL_ATTRIBUTE_SPD,
    SOUL_ATTRIBUTE_EFFECT_HIT,
    SOUL_ATTRIBUTE_EFFECT_RES,
    SOUL_ATTRIBUTE_CRIT,
    SOUL_ATTRIBUTE_CRIT_DMG,
  ];

  static final $core.List<SoulAttribute?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 11);
  static SoulAttribute? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SoulAttribute._(super.value, super.name);
}

class LevelBand extends $pb.ProtobufEnum {
  static const LevelBand LEVEL_BAND_UNSPECIFIED =
      LevelBand._(0, _omitEnumNames ? '' : 'LEVEL_BAND_UNSPECIFIED');
  static const LevelBand LEVEL_BAND_0_TO_2 =
      LevelBand._(1, _omitEnumNames ? '' : 'LEVEL_BAND_0_TO_2');
  static const LevelBand LEVEL_BAND_3_TO_5 =
      LevelBand._(2, _omitEnumNames ? '' : 'LEVEL_BAND_3_TO_5');
  static const LevelBand LEVEL_BAND_6_TO_8 =
      LevelBand._(3, _omitEnumNames ? '' : 'LEVEL_BAND_6_TO_8');
  static const LevelBand LEVEL_BAND_9_TO_11 =
      LevelBand._(4, _omitEnumNames ? '' : 'LEVEL_BAND_9_TO_11');
  static const LevelBand LEVEL_BAND_12_TO_14 =
      LevelBand._(5, _omitEnumNames ? '' : 'LEVEL_BAND_12_TO_14');
  static const LevelBand LEVEL_BAND_15 =
      LevelBand._(6, _omitEnumNames ? '' : 'LEVEL_BAND_15');

  static const $core.List<LevelBand> values = <LevelBand>[
    LEVEL_BAND_UNSPECIFIED,
    LEVEL_BAND_0_TO_2,
    LEVEL_BAND_3_TO_5,
    LEVEL_BAND_6_TO_8,
    LEVEL_BAND_9_TO_11,
    LEVEL_BAND_12_TO_14,
    LEVEL_BAND_15,
  ];

  static final $core.List<LevelBand?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 6);
  static LevelBand? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const LevelBand._(super.value, super.name);
}

class SubCount extends $pb.ProtobufEnum {
  static const SubCount SUB_COUNT_UNSPECIFIED =
      SubCount._(0, _omitEnumNames ? '' : 'SUB_COUNT_UNSPECIFIED');
  static const SubCount SUB_COUNT_FEWER_THAN_TWO =
      SubCount._(1, _omitEnumNames ? '' : 'SUB_COUNT_FEWER_THAN_TWO');
  static const SubCount SUB_COUNT_TWO =
      SubCount._(2, _omitEnumNames ? '' : 'SUB_COUNT_TWO');
  static const SubCount SUB_COUNT_THREE =
      SubCount._(3, _omitEnumNames ? '' : 'SUB_COUNT_THREE');
  static const SubCount SUB_COUNT_FOUR =
      SubCount._(4, _omitEnumNames ? '' : 'SUB_COUNT_FOUR');

  static const $core.List<SubCount> values = <SubCount>[
    SUB_COUNT_UNSPECIFIED,
    SUB_COUNT_FEWER_THAN_TWO,
    SUB_COUNT_TWO,
    SUB_COUNT_THREE,
    SUB_COUNT_FOUR,
  ];

  static final $core.List<SubCount?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static SubCount? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SubCount._(super.value, super.name);
}

class SubAttributeMode extends $pb.ProtobufEnum {
  static const SubAttributeMode SUB_ATTRIBUTE_MODE_UNSPECIFIED =
      SubAttributeMode._(
          0, _omitEnumNames ? '' : 'SUB_ATTRIBUTE_MODE_UNSPECIFIED');

  /// ○: the soul has it.
  static const SubAttributeMode SUB_ATTRIBUTE_MODE_INCLUDE =
      SubAttributeMode._(1, _omitEnumNames ? '' : 'SUB_ATTRIBUTE_MODE_INCLUDE');

  /// ✕: the soul does not have it.
  static const SubAttributeMode SUB_ATTRIBUTE_MODE_EXCLUDE =
      SubAttributeMode._(2, _omitEnumNames ? '' : 'SUB_ATTRIBUTE_MODE_EXCLUDE');

  static const $core.List<SubAttributeMode> values = <SubAttributeMode>[
    SUB_ATTRIBUTE_MODE_UNSPECIFIED,
    SUB_ATTRIBUTE_MODE_INCLUDE,
    SUB_ATTRIBUTE_MODE_EXCLUDE,
  ];

  static final $core.List<SubAttributeMode?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static SubAttributeMode? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SubAttributeMode._(super.value, super.name);
}

class Collection extends $pb.ProtobufEnum {
  static const Collection COLLECTION_UNSPECIFIED =
      Collection._(0, _omitEnumNames ? '' : 'COLLECTION_UNSPECIFIED');
  static const Collection COLLECTION_SOULS =
      Collection._(1, _omitEnumNames ? '' : 'COLLECTION_SOULS');

  static const $core.List<Collection> values = <Collection>[
    COLLECTION_UNSPECIFIED,
    COLLECTION_SOULS,
  ];

  static final $core.List<Collection?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 1);
  static Collection? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const Collection._(super.value, super.name);
}

/// The soul fields that take nothing beside their name. A field is added when it can be evaluated.
class SimpleField extends $pb.ProtobufEnum {
  static const SimpleField SIMPLE_FIELD_UNSPECIFIED =
      SimpleField._(0, _omitEnumNames ? '' : 'SIMPLE_FIELD_UNSPECIFIED');
  static const SimpleField SIMPLE_FIELD_SET =
      SimpleField._(1, _omitEnumNames ? '' : 'SIMPLE_FIELD_SET');
  static const SimpleField SIMPLE_FIELD_SLOT =
      SimpleField._(2, _omitEnumNames ? '' : 'SIMPLE_FIELD_SLOT');
  static const SimpleField SIMPLE_FIELD_STAR =
      SimpleField._(3, _omitEnumNames ? '' : 'SIMPLE_FIELD_STAR');
  static const SimpleField SIMPLE_FIELD_LEVEL =
      SimpleField._(4, _omitEnumNames ? '' : 'SIMPLE_FIELD_LEVEL');
  static const SimpleField SIMPLE_FIELD_MAIN_ATTRIBUTE =
      SimpleField._(5, _omitEnumNames ? '' : 'SIMPLE_FIELD_MAIN_ATTRIBUTE');
  static const SimpleField SIMPLE_FIELD_MAIN_VALUE =
      SimpleField._(6, _omitEnumNames ? '' : 'SIMPLE_FIELD_MAIN_VALUE');
  static const SimpleField SIMPLE_FIELD_SUB_COUNT =
      SimpleField._(7, _omitEnumNames ? '' : 'SIMPLE_FIELD_SUB_COUNT');
  static const SimpleField SIMPLE_FIELD_PRISTINE =
      SimpleField._(8, _omitEnumNames ? '' : 'SIMPLE_FIELD_PRISTINE');

  static const $core.List<SimpleField> values = <SimpleField>[
    SIMPLE_FIELD_UNSPECIFIED,
    SIMPLE_FIELD_SET,
    SIMPLE_FIELD_SLOT,
    SIMPLE_FIELD_STAR,
    SIMPLE_FIELD_LEVEL,
    SIMPLE_FIELD_MAIN_ATTRIBUTE,
    SIMPLE_FIELD_MAIN_VALUE,
    SIMPLE_FIELD_SUB_COUNT,
    SIMPLE_FIELD_PRISTINE,
  ];

  static final $core.List<SimpleField?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 8);
  static SimpleField? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SimpleField._(super.value, super.name);
}

/// The components of a quality score. Need `params`; refused with query.field_unavailable until
/// pass 1 exists.
class QualityComponent extends $pb.ProtobufEnum {
  static const QualityComponent QUALITY_COMPONENT_UNSPECIFIED =
      QualityComponent._(
          0, _omitEnumNames ? '' : 'QUALITY_COMPONENT_UNSPECIFIED');
  static const QualityComponent QUALITY_COMPONENT_TOTAL =
      QualityComponent._(1, _omitEnumNames ? '' : 'QUALITY_COMPONENT_TOTAL');
  static const QualityComponent QUALITY_COMPONENT_DEPTH =
      QualityComponent._(2, _omitEnumNames ? '' : 'QUALITY_COMPONENT_DEPTH');
  static const QualityComponent QUALITY_COMPONENT_BREADTH =
      QualityComponent._(3, _omitEnumNames ? '' : 'QUALITY_COMPONENT_BREADTH');

  static const $core.List<QualityComponent> values = <QualityComponent>[
    QUALITY_COMPONENT_UNSPECIFIED,
    QUALITY_COMPONENT_TOTAL,
    QUALITY_COMPONENT_DEPTH,
    QUALITY_COMPONENT_BREADTH,
  ];

  static final $core.List<QualityComponent?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static QualityComponent? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const QualityComponent._(super.value, super.name);
}

class Direction extends $pb.ProtobufEnum {
  static const Direction DIRECTION_UNSPECIFIED =
      Direction._(0, _omitEnumNames ? '' : 'DIRECTION_UNSPECIFIED');
  static const Direction DIRECTION_ASC =
      Direction._(1, _omitEnumNames ? '' : 'DIRECTION_ASC');
  static const Direction DIRECTION_DESC =
      Direction._(2, _omitEnumNames ? '' : 'DIRECTION_DESC');

  static const $core.List<Direction> values = <Direction>[
    DIRECTION_UNSPECIFIED,
    DIRECTION_ASC,
    DIRECTION_DESC,
  ];

  static final $core.List<Direction?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static Direction? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const Direction._(super.value, super.name);
}

class OpenRule extends $pb.ProtobufEnum {
  static const OpenRule OPEN_RULE_UNSPECIFIED =
      OpenRule._(0, _omitEnumNames ? '' : 'OPEN_RULE_UNSPECIFIED');
  static const OpenRule OPEN_RULE_INNATE =
      OpenRule._(1, _omitEnumNames ? '' : 'OPEN_RULE_INNATE');
  static const OpenRule OPEN_RULE_UNKNOWN_CONDITIONS =
      OpenRule._(2, _omitEnumNames ? '' : 'OPEN_RULE_UNKNOWN_CONDITIONS');

  static const $core.List<OpenRule> values = <OpenRule>[
    OPEN_RULE_UNSPECIFIED,
    OPEN_RULE_INNATE,
    OPEN_RULE_UNKNOWN_CONDITIONS,
  ];

  static final $core.List<OpenRule?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static OpenRule? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const OpenRule._(super.value, super.name);
}

/// A section of an imported snapshot (snapshot-ir.md).
class SectionKind extends $pb.ProtobufEnum {
  static const SectionKind SECTION_KIND_UNSPECIFIED =
      SectionKind._(0, _omitEnumNames ? '' : 'SECTION_KIND_UNSPECIFIED');
  static const SectionKind SECTION_KIND_SOULS =
      SectionKind._(1, _omitEnumNames ? '' : 'SECTION_KIND_SOULS');
  static const SectionKind SECTION_KIND_SHIKIGAMI =
      SectionKind._(2, _omitEnumNames ? '' : 'SECTION_KIND_SHIKIGAMI');
  static const SectionKind SECTION_KIND_PRESETS =
      SectionKind._(3, _omitEnumNames ? '' : 'SECTION_KIND_PRESETS');
  static const SectionKind SECTION_KIND_ASSETS =
      SectionKind._(4, _omitEnumNames ? '' : 'SECTION_KIND_ASSETS');
  static const SectionKind SECTION_KIND_GUILD =
      SectionKind._(5, _omitEnumNames ? '' : 'SECTION_KIND_GUILD');

  static const $core.List<SectionKind> values = <SectionKind>[
    SECTION_KIND_UNSPECIFIED,
    SECTION_KIND_SOULS,
    SECTION_KIND_SHIKIGAMI,
    SECTION_KIND_PRESETS,
    SECTION_KIND_ASSETS,
    SECTION_KIND_GUILD,
  ];

  static final $core.List<SectionKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static SectionKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SectionKind._(super.value, super.name);
}

/// Whether a present section holds everything it covers, by the source's own statement. Weakest
/// first.
class Completeness extends $pb.ProtobufEnum {
  static const Completeness COMPLETENESS_UNSPECIFIED =
      Completeness._(0, _omitEnumNames ? '' : 'COMPLETENESS_UNSPECIFIED');
  static const Completeness COMPLETENESS_UNSTATED =
      Completeness._(1, _omitEnumNames ? '' : 'COMPLETENESS_UNSTATED');
  static const Completeness COMPLETENESS_PARTIAL =
      Completeness._(2, _omitEnumNames ? '' : 'COMPLETENESS_PARTIAL');
  static const Completeness COMPLETENESS_COMPLETE =
      Completeness._(3, _omitEnumNames ? '' : 'COMPLETENESS_COMPLETE');

  static const $core.List<Completeness> values = <Completeness>[
    COMPLETENESS_UNSPECIFIED,
    COMPLETENESS_UNSTATED,
    COMPLETENESS_PARTIAL,
    COMPLETENESS_COMPLETE,
  ];

  static final $core.List<Completeness?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static Completeness? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const Completeness._(super.value, super.name);
}

/// What a profile can do (snapshot-ir.md, "Capabilities").
class Capability extends $pb.ProtobufEnum {
  static const Capability CAPABILITY_UNSPECIFIED =
      Capability._(0, _omitEnumNames ? '' : 'CAPABILITY_UNSPECIFIED');

  /// The soul list, the filters, quality, and the recommendations.
  static const Capability CAPABILITY_INVENTORY =
      Capability._(1, _omitEnumNames ? '' : 'CAPABILITY_INVENTORY');
  static const Capability CAPABILITY_SHIKIGAMI_COLLECTION =
      Capability._(2, _omitEnumNames ? '' : 'CAPABILITY_SHIKIGAMI_COLLECTION');

  /// The player's saved loadouts.
  static const Capability CAPABILITY_GAME_PRESETS =
      Capability._(3, _omitEnumNames ? '' : 'CAPABILITY_GAME_PRESETS');
  static const Capability CAPABILITY_ASSETS =
      Capability._(4, _omitEnumNames ? '' : 'CAPABILITY_ASSETS');
  static const Capability CAPABILITY_GUILD_VIEW =
      Capability._(5, _omitEnumNames ? '' : 'CAPABILITY_GUILD_VIEW');

  static const $core.List<Capability> values = <Capability>[
    CAPABILITY_UNSPECIFIED,
    CAPABILITY_INVENTORY,
    CAPABILITY_SHIKIGAMI_COLLECTION,
    CAPABILITY_GAME_PRESETS,
    CAPABILITY_ASSETS,
    CAPABILITY_GUILD_VIEW,
  ];

  static final $core.List<Capability?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 5);
  static Capability? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const Capability._(super.value, super.name);
}

class SchemeKind extends $pb.ProtobufEnum {
  static const SchemeKind SCHEME_KIND_UNSPECIFIED =
      SchemeKind._(0, _omitEnumNames ? '' : 'SCHEME_KIND_UNSPECIFIED');
  static const SchemeKind SCHEME_KIND_STRENGTHENING =
      SchemeKind._(1, _omitEnumNames ? '' : 'SCHEME_KIND_STRENGTHENING');
  static const SchemeKind SCHEME_KIND_DISCARD =
      SchemeKind._(2, _omitEnumNames ? '' : 'SCHEME_KIND_DISCARD');

  static const $core.List<SchemeKind> values = <SchemeKind>[
    SCHEME_KIND_UNSPECIFIED,
    SCHEME_KIND_STRENGTHENING,
    SCHEME_KIND_DISCARD,
  ];

  static final $core.List<SchemeKind?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static SchemeKind? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const SchemeKind._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
