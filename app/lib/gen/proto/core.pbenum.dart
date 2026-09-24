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

/// The soul fields this build can name. A field is added here when it can be evaluated.
class FieldName extends $pb.ProtobufEnum {
  static const FieldName FIELD_NAME_UNSPECIFIED =
      FieldName._(0, _omitEnumNames ? '' : 'FIELD_NAME_UNSPECIFIED');
  static const FieldName FIELD_NAME_SET =
      FieldName._(1, _omitEnumNames ? '' : 'FIELD_NAME_SET');
  static const FieldName FIELD_NAME_SLOT =
      FieldName._(2, _omitEnumNames ? '' : 'FIELD_NAME_SLOT');
  static const FieldName FIELD_NAME_STAR =
      FieldName._(3, _omitEnumNames ? '' : 'FIELD_NAME_STAR');
  static const FieldName FIELD_NAME_LEVEL =
      FieldName._(4, _omitEnumNames ? '' : 'FIELD_NAME_LEVEL');
  static const FieldName FIELD_NAME_MAIN_ATTRIBUTE =
      FieldName._(5, _omitEnumNames ? '' : 'FIELD_NAME_MAIN_ATTRIBUTE');
  static const FieldName FIELD_NAME_MAIN_VALUE =
      FieldName._(6, _omitEnumNames ? '' : 'FIELD_NAME_MAIN_VALUE');

  /// Takes an attribute.
  static const FieldName FIELD_NAME_SUB_VALUE =
      FieldName._(7, _omitEnumNames ? '' : 'FIELD_NAME_SUB_VALUE');

  /// Takes an attribute.
  static const FieldName FIELD_NAME_HAS_SUB =
      FieldName._(8, _omitEnumNames ? '' : 'FIELD_NAME_HAS_SUB');
  static const FieldName FIELD_NAME_SUB_COUNT =
      FieldName._(9, _omitEnumNames ? '' : 'FIELD_NAME_SUB_COUNT');
  static const FieldName FIELD_NAME_PRISTINE =
      FieldName._(10, _omitEnumNames ? '' : 'FIELD_NAME_PRISTINE');

  /// Scores: need `params`; refused with query.field_unavailable until pass 1 exists.
  static const FieldName FIELD_NAME_QUALITY_TOTAL =
      FieldName._(11, _omitEnumNames ? '' : 'FIELD_NAME_QUALITY_TOTAL');
  static const FieldName FIELD_NAME_QUALITY_DEPTH =
      FieldName._(12, _omitEnumNames ? '' : 'FIELD_NAME_QUALITY_DEPTH');
  static const FieldName FIELD_NAME_QUALITY_BREADTH =
      FieldName._(13, _omitEnumNames ? '' : 'FIELD_NAME_QUALITY_BREADTH');

  static const $core.List<FieldName> values = <FieldName>[
    FIELD_NAME_UNSPECIFIED,
    FIELD_NAME_SET,
    FIELD_NAME_SLOT,
    FIELD_NAME_STAR,
    FIELD_NAME_LEVEL,
    FIELD_NAME_MAIN_ATTRIBUTE,
    FIELD_NAME_MAIN_VALUE,
    FIELD_NAME_SUB_VALUE,
    FIELD_NAME_HAS_SUB,
    FIELD_NAME_SUB_COUNT,
    FIELD_NAME_PRISTINE,
    FIELD_NAME_QUALITY_TOTAL,
    FIELD_NAME_QUALITY_DEPTH,
    FIELD_NAME_QUALITY_BREADTH,
  ];

  static final $core.List<FieldName?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 13);
  static FieldName? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const FieldName._(super.value, super.name);
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
