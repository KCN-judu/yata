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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'core.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'core.pbenum.dart';

class ProtocolVersion extends $pb.GeneratedMessage {
  factory ProtocolVersion({
    $core.int? major,
    $core.int? minor,
  }) {
    final result = ProtocolVersion._();
    if (major != null) result.major = major;
    if (minor != null) result.minor = minor;
    return result;
  }

  ProtocolVersion._();

  factory ProtocolVersion.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProtocolVersion()..mergeFromBuffer(data, registry);
  factory ProtocolVersion.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProtocolVersion()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ProtocolVersion',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ProtocolVersion.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'major', fieldType: $pb.PbFieldType.OU3)
    ..aI(2, _omitFieldNames ? '' : 'minor', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProtocolVersion clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProtocolVersion copyWith(void Function(ProtocolVersion) updates) =>
      super.copyWith((message) => updates(message as ProtocolVersion))
          as ProtocolVersion;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ProtocolVersion() / ProtocolVersion.new instead')
  static ProtocolVersion create() => ProtocolVersion._();
  static $pb.GeneratedMessage $_createMessage() => ProtocolVersion._();
  @$core.override
  ProtocolVersion createEmptyInstance() => ProtocolVersion._();
  @$core.pragma('dart2js:noInline')
  static ProtocolVersion getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ProtocolVersion>(
          ProtocolVersion.$_createMessage);
  static ProtocolVersion? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get major => $_getIZ(0);
  @$pb.TagNumber(1)
  set major($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMajor() => $_has(0);
  @$pb.TagNumber(1)
  void clearMajor() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get minor => $_getIZ(1);
  @$pb.TagNumber(2)
  set minor($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMinor() => $_has(1);
  @$pb.TagNumber(2)
  void clearMinor() => $_clearField(2);
}

enum Soul_Kind { ordinary, boss, notSet }

/// A soul in domain terms, not the game's record: the set by suit code, values in display units.
class Soul extends $pb.GeneratedMessage {
  factory Soul({
    $core.String? soulId,
    $core.int? suitCode,
    SoulSlot? slot,
    $core.int? star,
    $core.int? level,
    SoulAttribute? main,
    $core.double? mainValue,
    $core.Iterable<SubAttribute>? subs,
    OrdinarySoul? ordinary,
    BossSoul? boss,
  }) {
    final result = Soul._();
    if (soulId != null) result.soulId = soulId;
    if (suitCode != null) result.suitCode = suitCode;
    if (slot != null) result.slot = slot;
    if (star != null) result.star = star;
    if (level != null) result.level = level;
    if (main != null) result.main = main;
    if (mainValue != null) result.mainValue = mainValue;
    if (subs != null) result.subs.addAll(subs);
    if (ordinary != null) result.ordinary = ordinary;
    if (boss != null) result.boss = boss;
    return result;
  }

  Soul._();

  factory Soul.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Soul()..mergeFromBuffer(data, registry);
  factory Soul.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Soul()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Soul_Kind> _Soul_KindByTag = {
    10: Soul_Kind.ordinary,
    11: Soul_Kind.boss,
    0: Soul_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Soul',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Soul.$_createMessage)
    ..oo(0, [10, 11])
    ..aOS(1, _omitFieldNames ? '' : 'soulId')
    ..aI(2, _omitFieldNames ? '' : 'suitCode', fieldType: $pb.PbFieldType.OU3)
    ..aE<SoulSlot>(3, _omitFieldNames ? '' : 'slot',
        enumValues: SoulSlot.values)
    ..aI(4, _omitFieldNames ? '' : 'star', fieldType: $pb.PbFieldType.OU3)
    ..aI(5, _omitFieldNames ? '' : 'level', fieldType: $pb.PbFieldType.OU3)
    ..aE<SoulAttribute>(6, _omitFieldNames ? '' : 'main',
        enumValues: SoulAttribute.values)
    ..aD(7, _omitFieldNames ? '' : 'mainValue')
    ..pPM<SubAttribute>(8, _omitFieldNames ? '' : 'subs',
        subBuilder: SubAttribute.$_createMessage)
    ..aOM<OrdinarySoul>(10, _omitFieldNames ? '' : 'ordinary',
        subBuilder: OrdinarySoul.$_createMessage)
    ..aOM<BossSoul>(11, _omitFieldNames ? '' : 'boss',
        subBuilder: BossSoul.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Soul clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Soul copyWith(void Function(Soul) updates) =>
      super.copyWith((message) => updates(message as Soul)) as Soul;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Soul() / Soul.new instead')
  static Soul create() => Soul._();
  static $pb.GeneratedMessage $_createMessage() => Soul._();
  @$core.override
  Soul createEmptyInstance() => Soul._();
  @$core.pragma('dart2js:noInline')
  static Soul getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Soul>(Soul.$_createMessage);
  static Soul? _defaultInstance;

  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  Soul_Kind whichKind() => _Soul_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  void clearKind() => $_clearField($_whichOneof(0));

  /// The game's soul id: non-empty, at most 64 bytes, unique within an inventory.
  @$pb.TagNumber(1)
  $core.String get soulId => $_getSZ(0);
  @$pb.TagNumber(1)
  set soulId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSoulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSoulId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get suitCode => $_getIZ(1);
  @$pb.TagNumber(2)
  set suitCode($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasSuitCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearSuitCode() => $_clearField(2);

  @$pb.TagNumber(3)
  SoulSlot get slot => $_getN(2);
  @$pb.TagNumber(3)
  set slot(SoulSlot value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasSlot() => $_has(2);
  @$pb.TagNumber(3)
  void clearSlot() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.int get star => $_getIZ(3);
  @$pb.TagNumber(4)
  set star($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasStar() => $_has(3);
  @$pb.TagNumber(4)
  void clearStar() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.int get level => $_getIZ(4);
  @$pb.TagNumber(5)
  set level($core.int value) => $_setUnsignedInt32(4, value);
  @$pb.TagNumber(5)
  $core.bool hasLevel() => $_has(4);
  @$pb.TagNumber(5)
  void clearLevel() => $_clearField(5);

  @$pb.TagNumber(6)
  SoulAttribute get main => $_getN(5);
  @$pb.TagNumber(6)
  set main(SoulAttribute value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasMain() => $_has(5);
  @$pb.TagNumber(6)
  void clearMain() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.double get mainValue => $_getN(6);
  @$pb.TagNumber(7)
  set mainValue($core.double value) => $_setDouble(6, value);
  @$pb.TagNumber(7)
  $core.bool hasMainValue() => $_has(6);
  @$pb.TagNumber(7)
  void clearMainValue() => $_clearField(7);

  /// At most 4.
  @$pb.TagNumber(8)
  $pb.PbList<SubAttribute> get subs => $_getList(7);

  @$pb.TagNumber(10)
  OrdinarySoul get ordinary => $_getN(8);
  @$pb.TagNumber(10)
  set ordinary(OrdinarySoul value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasOrdinary() => $_has(8);
  @$pb.TagNumber(10)
  void clearOrdinary() => $_clearField(10);
  @$pb.TagNumber(10)
  OrdinarySoul ensureOrdinary() => $_ensure(8);

  @$pb.TagNumber(11)
  BossSoul get boss => $_getN(9);
  @$pb.TagNumber(11)
  set boss(BossSoul value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasBoss() => $_has(9);
  @$pb.TagNumber(11)
  void clearBoss() => $_clearField(11);
  @$pb.TagNumber(11)
  BossSoul ensureBoss() => $_ensure(9);
}

/// A soul that is not a boss soul: it carries no innate attribute. Empty on purpose, so that
/// choosing it is the whole message.
class OrdinarySoul extends $pb.GeneratedMessage {
  factory OrdinarySoul() => OrdinarySoul._();

  OrdinarySoul._();

  factory OrdinarySoul.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OrdinarySoul()..mergeFromBuffer(data, registry);
  factory OrdinarySoul.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OrdinarySoul()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OrdinarySoul',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: OrdinarySoul.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OrdinarySoul clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OrdinarySoul copyWith(void Function(OrdinarySoul) updates) =>
      super.copyWith((message) => updates(message as OrdinarySoul))
          as OrdinarySoul;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use OrdinarySoul() / OrdinarySoul.new instead')
  static OrdinarySoul create() => OrdinarySoul._();
  static $pb.GeneratedMessage $_createMessage() => OrdinarySoul._();
  @$core.override
  OrdinarySoul createEmptyInstance() => OrdinarySoul._();
  @$core.pragma('dart2js:noInline')
  static OrdinarySoul getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OrdinarySoul>(
          OrdinarySoul.$_createMessage);
  static OrdinarySoul? _defaultInstance;
}

/// A boss soul (首领御魂) and its 固有属性, one of the six innate attributes.
class BossSoul extends $pb.GeneratedMessage {
  factory BossSoul({
    SoulAttribute? innate,
  }) {
    final result = BossSoul._();
    if (innate != null) result.innate = innate;
    return result;
  }

  BossSoul._();

  factory BossSoul.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BossSoul()..mergeFromBuffer(data, registry);
  factory BossSoul.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      BossSoul()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BossSoul',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: BossSoul.$_createMessage)
    ..aE<SoulAttribute>(1, _omitFieldNames ? '' : 'innate',
        enumValues: SoulAttribute.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BossSoul clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BossSoul copyWith(void Function(BossSoul) updates) =>
      super.copyWith((message) => updates(message as BossSoul)) as BossSoul;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use BossSoul() / BossSoul.new instead')
  static BossSoul create() => BossSoul._();
  static $pb.GeneratedMessage $_createMessage() => BossSoul._();
  @$core.override
  BossSoul createEmptyInstance() => BossSoul._();
  @$core.pragma('dart2js:noInline')
  static BossSoul getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BossSoul>(BossSoul.$_createMessage);
  static BossSoul? _defaultInstance;

  @$pb.TagNumber(1)
  SoulAttribute get innate => $_getN(0);
  @$pb.TagNumber(1)
  set innate(SoulAttribute value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasInnate() => $_has(0);
  @$pb.TagNumber(1)
  void clearInnate() => $_clearField(1);
}

class SubAttribute extends $pb.GeneratedMessage {
  factory SubAttribute({
    SoulAttribute? attribute,
    $core.double? value,
    $core.int? enhancementCount,
  }) {
    final result = SubAttribute._();
    if (attribute != null) result.attribute = attribute;
    if (value != null) result.value = value;
    if (enhancementCount != null) result.enhancementCount = enhancementCount;
    return result;
  }

  SubAttribute._();

  factory SubAttribute.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SubAttribute()..mergeFromBuffer(data, registry);
  factory SubAttribute.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SubAttribute()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SubAttribute',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SubAttribute.$_createMessage)
    ..aE<SoulAttribute>(1, _omitFieldNames ? '' : 'attribute',
        enumValues: SoulAttribute.values)
    ..aD(2, _omitFieldNames ? '' : 'value')
    ..aI(3, _omitFieldNames ? '' : 'enhancementCount',
        fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SubAttribute clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SubAttribute copyWith(void Function(SubAttribute) updates) =>
      super.copyWith((message) => updates(message as SubAttribute))
          as SubAttribute;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SubAttribute() / SubAttribute.new instead')
  static SubAttribute create() => SubAttribute._();
  static $pb.GeneratedMessage $_createMessage() => SubAttribute._();
  @$core.override
  SubAttribute createEmptyInstance() => SubAttribute._();
  @$core.pragma('dart2js:noInline')
  static SubAttribute getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SubAttribute>(
          SubAttribute.$_createMessage);
  static SubAttribute? _defaultInstance;

  @$pb.TagNumber(1)
  SoulAttribute get attribute => $_getN(0);
  @$pb.TagNumber(1)
  set attribute(SoulAttribute value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAttribute() => $_has(0);
  @$pb.TagNumber(1)
  void clearAttribute() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.double get value => $_getN(1);
  @$pb.TagNumber(2)
  set value($core.double value) => $_setDouble(1, value);
  @$pb.TagNumber(2)
  $core.bool hasValue() => $_has(1);
  @$pb.TagNumber(2)
  void clearValue() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.int get enhancementCount => $_getIZ(2);
  @$pb.TagNumber(3)
  set enhancementCount($core.int value) => $_setUnsignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasEnhancementCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearEnhancementCount() => $_clearField(3);
}

/// 全部: no restriction on the set. The game's 类型 with nothing chosen means the same, and is
/// written this way only.
class AnySet extends $pb.GeneratedMessage {
  factory AnySet() => AnySet._();

  AnySet._();

  factory AnySet.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AnySet()..mergeFromBuffer(data, registry);
  factory AnySet.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      AnySet()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'AnySet',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: AnySet.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AnySet clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  AnySet copyWith(void Function(AnySet) updates) =>
      super.copyWith((message) => updates(message as AnySet)) as AnySet;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use AnySet() / AnySet.new instead')
  static AnySet create() => AnySet._();
  static $pb.GeneratedMessage $_createMessage() => AnySet._();
  @$core.override
  AnySet createEmptyInstance() => AnySet._();
  @$core.pragma('dart2js:noInline')
  static AnySet getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<AnySet>(AnySet.$_createMessage);
  static AnySet? _defaultInstance;
}

/// Chosen sets, by suit code. Never empty: "every set" is `AnySet`. At most 256.
class SuitCodes extends $pb.GeneratedMessage {
  factory SuitCodes({
    $core.Iterable<$core.int>? codes,
  }) {
    final result = SuitCodes._();
    if (codes != null) result.codes.addAll(codes);
    return result;
  }

  SuitCodes._();

  factory SuitCodes.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SuitCodes()..mergeFromBuffer(data, registry);
  factory SuitCodes.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SuitCodes()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SuitCodes',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SuitCodes.$_createMessage)
    ..p<$core.int>(1, _omitFieldNames ? '' : 'codes', $pb.PbFieldType.KU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SuitCodes clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SuitCodes copyWith(void Function(SuitCodes) updates) =>
      super.copyWith((message) => updates(message as SuitCodes)) as SuitCodes;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SuitCodes() / SuitCodes.new instead')
  static SuitCodes create() => SuitCodes._();
  static $pb.GeneratedMessage $_createMessage() => SuitCodes._();
  @$core.override
  SuitCodes createEmptyInstance() => SuitCodes._();
  @$core.pragma('dart2js:noInline')
  static SuitCodes getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SuitCodes>(SuitCodes.$_createMessage);
  static SuitCodes? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.int> get codes => $_getList(0);
}

class SubAttributeChoice extends $pb.GeneratedMessage {
  factory SubAttributeChoice({
    SoulAttribute? attribute,
    SubAttributeMode? mode,
  }) {
    final result = SubAttributeChoice._();
    if (attribute != null) result.attribute = attribute;
    if (mode != null) result.mode = mode;
    return result;
  }

  SubAttributeChoice._();

  factory SubAttributeChoice.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SubAttributeChoice()..mergeFromBuffer(data, registry);
  factory SubAttributeChoice.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SubAttributeChoice()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SubAttributeChoice',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SubAttributeChoice.$_createMessage)
    ..aE<SoulAttribute>(1, _omitFieldNames ? '' : 'attribute',
        enumValues: SoulAttribute.values)
    ..aE<SubAttributeMode>(2, _omitFieldNames ? '' : 'mode',
        enumValues: SubAttributeMode.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SubAttributeChoice clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SubAttributeChoice copyWith(void Function(SubAttributeChoice) updates) =>
      super.copyWith((message) => updates(message as SubAttributeChoice))
          as SubAttributeChoice;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SubAttributeChoice() / SubAttributeChoice.new instead')
  static SubAttributeChoice create() => SubAttributeChoice._();
  static $pb.GeneratedMessage $_createMessage() => SubAttributeChoice._();
  @$core.override
  SubAttributeChoice createEmptyInstance() => SubAttributeChoice._();
  @$core.pragma('dart2js:noInline')
  static SubAttributeChoice getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SubAttributeChoice>(
          SubAttributeChoice.$_createMessage);
  static SubAttributeChoice? _defaultInstance;

  @$pb.TagNumber(1)
  SoulAttribute get attribute => $_getN(0);
  @$pb.TagNumber(1)
  set attribute(SoulAttribute value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAttribute() => $_has(0);
  @$pb.TagNumber(1)
  void clearAttribute() => $_clearField(1);

  @$pb.TagNumber(2)
  SubAttributeMode get mode => $_getN(1);
  @$pb.TagNumber(2)
  set mode(SubAttributeMode value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasMode() => $_has(1);
  @$pb.TagNumber(2)
  void clearMode() => $_clearField(2);
}

enum SoulSelection_Sets { all, chosen, notSet }

/// A SoulSelection: each group a set of choices, an empty group no constraint. Each repeated
/// field holds at most 256 values.
class SoulSelection extends $pb.GeneratedMessage {
  factory SoulSelection({
    $core.Iterable<SoulSlot>? slots,
    $core.Iterable<$core.int>? stars,
    $core.Iterable<LevelBand>? levels,
    $core.Iterable<SoulAttribute>? mainAttributes,
    $core.Iterable<SoulAttribute>? innate,
    $core.Iterable<SubCount>? subCounts,
    AnySet? all,
    SuitCodes? chosen,
    $core.Iterable<SubAttributeChoice>? subAttributes,
  }) {
    final result = SoulSelection._();
    if (slots != null) result.slots.addAll(slots);
    if (stars != null) result.stars.addAll(stars);
    if (levels != null) result.levels.addAll(levels);
    if (mainAttributes != null) result.mainAttributes.addAll(mainAttributes);
    if (innate != null) result.innate.addAll(innate);
    if (subCounts != null) result.subCounts.addAll(subCounts);
    if (all != null) result.all = all;
    if (chosen != null) result.chosen = chosen;
    if (subAttributes != null) result.subAttributes.addAll(subAttributes);
    return result;
  }

  SoulSelection._();

  factory SoulSelection.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SoulSelection()..mergeFromBuffer(data, registry);
  factory SoulSelection.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SoulSelection()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, SoulSelection_Sets>
      _SoulSelection_SetsByTag = {
    11: SoulSelection_Sets.all,
    12: SoulSelection_Sets.chosen,
    0: SoulSelection_Sets.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SoulSelection',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SoulSelection.$_createMessage)
    ..oo(0, [11, 12])
    ..pc<SoulSlot>(3, _omitFieldNames ? '' : 'slots', $pb.PbFieldType.KE,
        valueOf: SoulSlot.valueOf,
        enumValues: SoulSlot.values,
        defaultEnumValue: SoulSlot.SOUL_SLOT_UNSPECIFIED)
    ..p<$core.int>(4, _omitFieldNames ? '' : 'stars', $pb.PbFieldType.KU3)
    ..pc<LevelBand>(5, _omitFieldNames ? '' : 'levels', $pb.PbFieldType.KE,
        valueOf: LevelBand.valueOf,
        enumValues: LevelBand.values,
        defaultEnumValue: LevelBand.LEVEL_BAND_UNSPECIFIED)
    ..pc<SoulAttribute>(
        6, _omitFieldNames ? '' : 'mainAttributes', $pb.PbFieldType.KE,
        valueOf: SoulAttribute.valueOf,
        enumValues: SoulAttribute.values,
        defaultEnumValue: SoulAttribute.SOUL_ATTRIBUTE_UNSPECIFIED)
    ..pc<SoulAttribute>(7, _omitFieldNames ? '' : 'innate', $pb.PbFieldType.KE,
        valueOf: SoulAttribute.valueOf,
        enumValues: SoulAttribute.values,
        defaultEnumValue: SoulAttribute.SOUL_ATTRIBUTE_UNSPECIFIED)
    ..pc<SubCount>(10, _omitFieldNames ? '' : 'subCounts', $pb.PbFieldType.KE,
        valueOf: SubCount.valueOf,
        enumValues: SubCount.values,
        defaultEnumValue: SubCount.SUB_COUNT_UNSPECIFIED)
    ..aOM<AnySet>(11, _omitFieldNames ? '' : 'all',
        subBuilder: AnySet.$_createMessage)
    ..aOM<SuitCodes>(12, _omitFieldNames ? '' : 'chosen',
        subBuilder: SuitCodes.$_createMessage)
    ..pPM<SubAttributeChoice>(13, _omitFieldNames ? '' : 'subAttributes',
        subBuilder: SubAttributeChoice.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SoulSelection clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SoulSelection copyWith(void Function(SoulSelection) updates) =>
      super.copyWith((message) => updates(message as SoulSelection))
          as SoulSelection;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SoulSelection() / SoulSelection.new instead')
  static SoulSelection create() => SoulSelection._();
  static $pb.GeneratedMessage $_createMessage() => SoulSelection._();
  @$core.override
  SoulSelection createEmptyInstance() => SoulSelection._();
  @$core.pragma('dart2js:noInline')
  static SoulSelection getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SoulSelection>(
          SoulSelection.$_createMessage);
  static SoulSelection? _defaultInstance;

  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  SoulSelection_Sets whichSets() => _SoulSelection_SetsByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  void clearSets() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(3)
  $pb.PbList<SoulSlot> get slots => $_getList(0);

  @$pb.TagNumber(4)
  $pb.PbList<$core.int> get stars => $_getList(1);

  @$pb.TagNumber(5)
  $pb.PbList<LevelBand> get levels => $_getList(2);

  @$pb.TagNumber(6)
  $pb.PbList<SoulAttribute> get mainAttributes => $_getList(3);

  /// One of the six innate attributes.
  @$pb.TagNumber(7)
  $pb.PbList<SoulAttribute> get innate => $_getList(4);

  @$pb.TagNumber(10)
  $pb.PbList<SubCount> get subCounts => $_getList(5);

  @$pb.TagNumber(11)
  AnySet get all => $_getN(6);
  @$pb.TagNumber(11)
  set all(AnySet value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasAll() => $_has(6);
  @$pb.TagNumber(11)
  void clearAll() => $_clearField(11);
  @$pb.TagNumber(11)
  AnySet ensureAll() => $_ensure(6);

  @$pb.TagNumber(12)
  SuitCodes get chosen => $_getN(7);
  @$pb.TagNumber(12)
  set chosen(SuitCodes value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasChosen() => $_has(7);
  @$pb.TagNumber(12)
  void clearChosen() => $_clearField(12);
  @$pb.TagNumber(12)
  SuitCodes ensureChosen() => $_ensure(7);

  /// 副属性: ○ or ✕ per attribute, each attribute at most once.
  @$pb.TagNumber(13)
  $pb.PbList<SubAttributeChoice> get subAttributes => $_getList(8);
}

enum Field_Field { simple, subValue, hasSub, quality, notSet }

/// A field of a soul row: exactly one is set, and a field that takes an attribute carries it.
class Field extends $pb.GeneratedMessage {
  factory Field({
    SimpleField? simple,
    SoulAttribute? subValue,
    SoulAttribute? hasSub,
    QualityComponent? quality,
  }) {
    final result = Field._();
    if (simple != null) result.simple = simple;
    if (subValue != null) result.subValue = subValue;
    if (hasSub != null) result.hasSub = hasSub;
    if (quality != null) result.quality = quality;
    return result;
  }

  Field._();

  factory Field.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Field()..mergeFromBuffer(data, registry);
  factory Field.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Field()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Field_Field> _Field_FieldByTag = {
    3: Field_Field.simple,
    4: Field_Field.subValue,
    5: Field_Field.hasSub,
    6: Field_Field.quality,
    0: Field_Field.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Field',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Field.$_createMessage)
    ..oo(0, [3, 4, 5, 6])
    ..aE<SimpleField>(3, _omitFieldNames ? '' : 'simple',
        enumValues: SimpleField.values)
    ..aE<SoulAttribute>(4, _omitFieldNames ? '' : 'subValue',
        enumValues: SoulAttribute.values)
    ..aE<SoulAttribute>(5, _omitFieldNames ? '' : 'hasSub',
        enumValues: SoulAttribute.values)
    ..aE<QualityComponent>(6, _omitFieldNames ? '' : 'quality',
        enumValues: QualityComponent.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Field clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Field copyWith(void Function(Field) updates) =>
      super.copyWith((message) => updates(message as Field)) as Field;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Field() / Field.new instead')
  static Field create() => Field._();
  static $pb.GeneratedMessage $_createMessage() => Field._();
  @$core.override
  Field createEmptyInstance() => Field._();
  @$core.pragma('dart2js:noInline')
  static Field getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Field>(Field.$_createMessage);
  static Field? _defaultInstance;

  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  Field_Field whichField_() => _Field_FieldByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearField_() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(3)
  SimpleField get simple => $_getN(0);
  @$pb.TagNumber(3)
  set simple(SimpleField value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasSimple() => $_has(0);
  @$pb.TagNumber(3)
  void clearSimple() => $_clearField(3);

  @$pb.TagNumber(4)
  SoulAttribute get subValue => $_getN(1);
  @$pb.TagNumber(4)
  set subValue(SoulAttribute value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasSubValue() => $_has(1);
  @$pb.TagNumber(4)
  void clearSubValue() => $_clearField(4);

  @$pb.TagNumber(5)
  SoulAttribute get hasSub => $_getN(2);
  @$pb.TagNumber(5)
  set hasSub(SoulAttribute value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasHasSub() => $_has(2);
  @$pb.TagNumber(5)
  void clearHasSub() => $_clearField(5);

  @$pb.TagNumber(6)
  QualityComponent get quality => $_getN(3);
  @$pb.TagNumber(6)
  set quality(QualityComponent value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasQuality() => $_has(3);
  @$pb.TagNumber(6)
  void clearQuality() => $_clearField(6);
}

class Slots extends $pb.GeneratedMessage {
  factory Slots({
    $core.Iterable<SoulSlot>? values,
  }) {
    final result = Slots._();
    if (values != null) result.values.addAll(values);
    return result;
  }

  Slots._();

  factory Slots.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Slots()..mergeFromBuffer(data, registry);
  factory Slots.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Slots()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Slots',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Slots.$_createMessage)
    ..pc<SoulSlot>(1, _omitFieldNames ? '' : 'values', $pb.PbFieldType.KE,
        valueOf: SoulSlot.valueOf,
        enumValues: SoulSlot.values,
        defaultEnumValue: SoulSlot.SOUL_SLOT_UNSPECIFIED)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Slots clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Slots copyWith(void Function(Slots) updates) =>
      super.copyWith((message) => updates(message as Slots)) as Slots;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Slots() / Slots.new instead')
  static Slots create() => Slots._();
  static $pb.GeneratedMessage $_createMessage() => Slots._();
  @$core.override
  Slots createEmptyInstance() => Slots._();
  @$core.pragma('dart2js:noInline')
  static Slots getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Slots>(Slots.$_createMessage);
  static Slots? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<SoulSlot> get values => $_getList(0);
}

class Attributes extends $pb.GeneratedMessage {
  factory Attributes({
    $core.Iterable<SoulAttribute>? values,
  }) {
    final result = Attributes._();
    if (values != null) result.values.addAll(values);
    return result;
  }

  Attributes._();

  factory Attributes.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Attributes()..mergeFromBuffer(data, registry);
  factory Attributes.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Attributes()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Attributes',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Attributes.$_createMessage)
    ..pc<SoulAttribute>(1, _omitFieldNames ? '' : 'values', $pb.PbFieldType.KE,
        valueOf: SoulAttribute.valueOf,
        enumValues: SoulAttribute.values,
        defaultEnumValue: SoulAttribute.SOUL_ATTRIBUTE_UNSPECIFIED)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Attributes clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Attributes copyWith(void Function(Attributes) updates) =>
      super.copyWith((message) => updates(message as Attributes)) as Attributes;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Attributes() / Attributes.new instead')
  static Attributes create() => Attributes._();
  static $pb.GeneratedMessage $_createMessage() => Attributes._();
  @$core.override
  Attributes createEmptyInstance() => Attributes._();
  @$core.pragma('dart2js:noInline')
  static Attributes getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Attributes>(Attributes.$_createMessage);
  static Attributes? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<SoulAttribute> get values => $_getList(0);
}

enum InTest_Values { setValues, slotValues, attributeValues, notSet }

/// The values of an `In`: one list, of the field's type. At most 256; never empty.
class InTest extends $pb.GeneratedMessage {
  factory InTest({
    SuitCodes? setValues,
    Slots? slotValues,
    Attributes? attributeValues,
  }) {
    final result = InTest._();
    if (setValues != null) result.setValues = setValues;
    if (slotValues != null) result.slotValues = slotValues;
    if (attributeValues != null) result.attributeValues = attributeValues;
    return result;
  }

  InTest._();

  factory InTest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InTest()..mergeFromBuffer(data, registry);
  factory InTest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InTest()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, InTest_Values> _InTest_ValuesByTag = {
    4: InTest_Values.setValues,
    5: InTest_Values.slotValues,
    6: InTest_Values.attributeValues,
    0: InTest_Values.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InTest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InTest.$_createMessage)
    ..oo(0, [4, 5, 6])
    ..aOM<SuitCodes>(4, _omitFieldNames ? '' : 'setValues',
        subBuilder: SuitCodes.$_createMessage)
    ..aOM<Slots>(5, _omitFieldNames ? '' : 'slotValues',
        subBuilder: Slots.$_createMessage)
    ..aOM<Attributes>(6, _omitFieldNames ? '' : 'attributeValues',
        subBuilder: Attributes.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InTest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InTest copyWith(void Function(InTest) updates) =>
      super.copyWith((message) => updates(message as InTest)) as InTest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use InTest() / InTest.new instead')
  static InTest create() => InTest._();
  static $pb.GeneratedMessage $_createMessage() => InTest._();
  @$core.override
  InTest createEmptyInstance() => InTest._();
  @$core.pragma('dart2js:noInline')
  static InTest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InTest>(InTest.$_createMessage);
  static InTest? _defaultInstance;

  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  InTest_Values whichValues() => _InTest_ValuesByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearValues() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(4)
  SuitCodes get setValues => $_getN(0);
  @$pb.TagNumber(4)
  set setValues(SuitCodes value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasSetValues() => $_has(0);
  @$pb.TagNumber(4)
  void clearSetValues() => $_clearField(4);
  @$pb.TagNumber(4)
  SuitCodes ensureSetValues() => $_ensure(0);

  @$pb.TagNumber(5)
  Slots get slotValues => $_getN(1);
  @$pb.TagNumber(5)
  set slotValues(Slots value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasSlotValues() => $_has(1);
  @$pb.TagNumber(5)
  void clearSlotValues() => $_clearField(5);
  @$pb.TagNumber(5)
  Slots ensureSlotValues() => $_ensure(1);

  @$pb.TagNumber(6)
  Attributes get attributeValues => $_getN(2);
  @$pb.TagNumber(6)
  set attributeValues(Attributes value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasAttributeValues() => $_has(2);
  @$pb.TagNumber(6)
  void clearAttributeValues() => $_clearField(6);
  @$pb.TagNumber(6)
  Attributes ensureAttributeValues() => $_ensure(2);
}

class IntBetween extends $pb.GeneratedMessage {
  factory IntBetween({
    $fixnum.Int64? min,
    $fixnum.Int64? max,
  }) {
    final result = IntBetween._();
    if (min != null) result.min = min;
    if (max != null) result.max = max;
    return result;
  }

  IntBetween._();

  factory IntBetween.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IntBetween()..mergeFromBuffer(data, registry);
  factory IntBetween.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IntBetween()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'IntBetween',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: IntBetween.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'min')
    ..aInt64(2, _omitFieldNames ? '' : 'max')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IntBetween clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IntBetween copyWith(void Function(IntBetween) updates) =>
      super.copyWith((message) => updates(message as IntBetween)) as IntBetween;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use IntBetween() / IntBetween.new instead')
  static IntBetween create() => IntBetween._();
  static $pb.GeneratedMessage $_createMessage() => IntBetween._();
  @$core.override
  IntBetween createEmptyInstance() => IntBetween._();
  @$core.pragma('dart2js:noInline')
  static IntBetween getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<IntBetween>(IntBetween.$_createMessage);
  static IntBetween? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get min => $_getI64(0);
  @$pb.TagNumber(1)
  set min($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMin() => $_has(0);
  @$pb.TagNumber(1)
  void clearMin() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get max => $_getI64(1);
  @$pb.TagNumber(2)
  set max($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMax() => $_has(1);
  @$pb.TagNumber(2)
  void clearMax() => $_clearField(2);
}

enum IntRange_Bound { atLeast, atMost, between, notSet }

/// Inclusive bounds: exactly one is set; `between` with `min` above `max` is query.malformed.
class IntRange extends $pb.GeneratedMessage {
  factory IntRange({
    $fixnum.Int64? atLeast,
    $fixnum.Int64? atMost,
    IntBetween? between,
  }) {
    final result = IntRange._();
    if (atLeast != null) result.atLeast = atLeast;
    if (atMost != null) result.atMost = atMost;
    if (between != null) result.between = between;
    return result;
  }

  IntRange._();

  factory IntRange.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IntRange()..mergeFromBuffer(data, registry);
  factory IntRange.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IntRange()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, IntRange_Bound> _IntRange_BoundByTag = {
    3: IntRange_Bound.atLeast,
    4: IntRange_Bound.atMost,
    5: IntRange_Bound.between,
    0: IntRange_Bound.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'IntRange',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: IntRange.$_createMessage)
    ..oo(0, [3, 4, 5])
    ..aInt64(3, _omitFieldNames ? '' : 'atLeast')
    ..aInt64(4, _omitFieldNames ? '' : 'atMost')
    ..aOM<IntBetween>(5, _omitFieldNames ? '' : 'between',
        subBuilder: IntBetween.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IntRange clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  IntRange copyWith(void Function(IntRange) updates) =>
      super.copyWith((message) => updates(message as IntRange)) as IntRange;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use IntRange() / IntRange.new instead')
  static IntRange create() => IntRange._();
  static $pb.GeneratedMessage $_createMessage() => IntRange._();
  @$core.override
  IntRange createEmptyInstance() => IntRange._();
  @$core.pragma('dart2js:noInline')
  static IntRange getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<IntRange>(IntRange.$_createMessage);
  static IntRange? _defaultInstance;

  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  IntRange_Bound whichBound() => _IntRange_BoundByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  void clearBound() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(3)
  $fixnum.Int64 get atLeast => $_getI64(0);
  @$pb.TagNumber(3)
  set atLeast($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(3)
  $core.bool hasAtLeast() => $_has(0);
  @$pb.TagNumber(3)
  void clearAtLeast() => $_clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get atMost => $_getI64(1);
  @$pb.TagNumber(4)
  set atMost($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(4)
  $core.bool hasAtMost() => $_has(1);
  @$pb.TagNumber(4)
  void clearAtMost() => $_clearField(4);

  @$pb.TagNumber(5)
  IntBetween get between => $_getN(2);
  @$pb.TagNumber(5)
  set between(IntBetween value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasBetween() => $_has(2);
  @$pb.TagNumber(5)
  void clearBetween() => $_clearField(5);
  @$pb.TagNumber(5)
  IntBetween ensureBetween() => $_ensure(2);
}

class NumberBetween extends $pb.GeneratedMessage {
  factory NumberBetween({
    $core.double? min,
    $core.double? max,
  }) {
    final result = NumberBetween._();
    if (min != null) result.min = min;
    if (max != null) result.max = max;
    return result;
  }

  NumberBetween._();

  factory NumberBetween.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NumberBetween()..mergeFromBuffer(data, registry);
  factory NumberBetween.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NumberBetween()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NumberBetween',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: NumberBetween.$_createMessage)
    ..aD(1, _omitFieldNames ? '' : 'min')
    ..aD(2, _omitFieldNames ? '' : 'max')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NumberBetween clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NumberBetween copyWith(void Function(NumberBetween) updates) =>
      super.copyWith((message) => updates(message as NumberBetween))
          as NumberBetween;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use NumberBetween() / NumberBetween.new instead')
  static NumberBetween create() => NumberBetween._();
  static $pb.GeneratedMessage $_createMessage() => NumberBetween._();
  @$core.override
  NumberBetween createEmptyInstance() => NumberBetween._();
  @$core.pragma('dart2js:noInline')
  static NumberBetween getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NumberBetween>(
          NumberBetween.$_createMessage);
  static NumberBetween? _defaultInstance;

  @$pb.TagNumber(1)
  $core.double get min => $_getN(0);
  @$pb.TagNumber(1)
  set min($core.double value) => $_setDouble(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMin() => $_has(0);
  @$pb.TagNumber(1)
  void clearMin() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.double get max => $_getN(1);
  @$pb.TagNumber(2)
  set max($core.double value) => $_setDouble(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMax() => $_has(1);
  @$pb.TagNumber(2)
  void clearMax() => $_clearField(2);
}

enum NumberRange_Bound { atLeast, atMost, between, notSet }

/// As `IntRange`, over finite numbers.
class NumberRange extends $pb.GeneratedMessage {
  factory NumberRange({
    $core.double? atLeast,
    $core.double? atMost,
    NumberBetween? between,
  }) {
    final result = NumberRange._();
    if (atLeast != null) result.atLeast = atLeast;
    if (atMost != null) result.atMost = atMost;
    if (between != null) result.between = between;
    return result;
  }

  NumberRange._();

  factory NumberRange.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NumberRange()..mergeFromBuffer(data, registry);
  factory NumberRange.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NumberRange()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, NumberRange_Bound> _NumberRange_BoundByTag =
      {
    3: NumberRange_Bound.atLeast,
    4: NumberRange_Bound.atMost,
    5: NumberRange_Bound.between,
    0: NumberRange_Bound.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NumberRange',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: NumberRange.$_createMessage)
    ..oo(0, [3, 4, 5])
    ..aD(3, _omitFieldNames ? '' : 'atLeast')
    ..aD(4, _omitFieldNames ? '' : 'atMost')
    ..aOM<NumberBetween>(5, _omitFieldNames ? '' : 'between',
        subBuilder: NumberBetween.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NumberRange clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NumberRange copyWith(void Function(NumberRange) updates) =>
      super.copyWith((message) => updates(message as NumberRange))
          as NumberRange;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use NumberRange() / NumberRange.new instead')
  static NumberRange create() => NumberRange._();
  static $pb.GeneratedMessage $_createMessage() => NumberRange._();
  @$core.override
  NumberRange createEmptyInstance() => NumberRange._();
  @$core.pragma('dart2js:noInline')
  static NumberRange getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NumberRange>(
          NumberRange.$_createMessage);
  static NumberRange? _defaultInstance;

  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  NumberRange_Bound whichBound() => _NumberRange_BoundByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  void clearBound() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(3)
  $core.double get atLeast => $_getN(0);
  @$pb.TagNumber(3)
  set atLeast($core.double value) => $_setDouble(0, value);
  @$pb.TagNumber(3)
  $core.bool hasAtLeast() => $_has(0);
  @$pb.TagNumber(3)
  void clearAtLeast() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.double get atMost => $_getN(1);
  @$pb.TagNumber(4)
  set atMost($core.double value) => $_setDouble(1, value);
  @$pb.TagNumber(4)
  $core.bool hasAtMost() => $_has(1);
  @$pb.TagNumber(4)
  void clearAtMost() => $_clearField(4);

  @$pb.TagNumber(5)
  NumberBetween get between => $_getN(2);
  @$pb.TagNumber(5)
  set between(NumberBetween value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasBetween() => $_has(2);
  @$pb.TagNumber(5)
  void clearBetween() => $_clearField(5);
  @$pb.TagNumber(5)
  NumberBetween ensureBetween() => $_ensure(2);
}

enum Predicate_Test { in_2, intRange, numberRange, is_5, notSet }

class Predicate extends $pb.GeneratedMessage {
  factory Predicate({
    Field? field_1,
    InTest? in_2,
    IntRange? intRange,
    NumberRange? numberRange,
    $core.bool? is_5,
  }) {
    final result = Predicate._();
    if (field_1 != null) result.field_1 = field_1;
    if (in_2 != null) result.in_2 = in_2;
    if (intRange != null) result.intRange = intRange;
    if (numberRange != null) result.numberRange = numberRange;
    if (is_5 != null) result.is_5 = is_5;
    return result;
  }

  Predicate._();

  factory Predicate.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Predicate()..mergeFromBuffer(data, registry);
  factory Predicate.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Predicate()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Predicate_Test> _Predicate_TestByTag = {
    2: Predicate_Test.in_2,
    3: Predicate_Test.intRange,
    4: Predicate_Test.numberRange,
    5: Predicate_Test.is_5,
    0: Predicate_Test.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Predicate',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Predicate.$_createMessage)
    ..oo(0, [2, 3, 4, 5])
    ..aOM<Field>(1, _omitFieldNames ? '' : 'field',
        subBuilder: Field.$_createMessage)
    ..aOM<InTest>(2, _omitFieldNames ? '' : 'in',
        subBuilder: InTest.$_createMessage)
    ..aOM<IntRange>(3, _omitFieldNames ? '' : 'intRange',
        subBuilder: IntRange.$_createMessage)
    ..aOM<NumberRange>(4, _omitFieldNames ? '' : 'numberRange',
        subBuilder: NumberRange.$_createMessage)
    ..aOB(5, _omitFieldNames ? '' : 'is')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Predicate clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Predicate copyWith(void Function(Predicate) updates) =>
      super.copyWith((message) => updates(message as Predicate)) as Predicate;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Predicate() / Predicate.new instead')
  static Predicate create() => Predicate._();
  static $pb.GeneratedMessage $_createMessage() => Predicate._();
  @$core.override
  Predicate createEmptyInstance() => Predicate._();
  @$core.pragma('dart2js:noInline')
  static Predicate getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Predicate>(Predicate.$_createMessage);
  static Predicate? _defaultInstance;

  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  Predicate_Test whichTest() => _Predicate_TestByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  void clearTest() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Field get field_1 => $_getN(0);
  @$pb.TagNumber(1)
  set field_1(Field value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasField_1() => $_has(0);
  @$pb.TagNumber(1)
  void clearField_1() => $_clearField(1);
  @$pb.TagNumber(1)
  Field ensureField_1() => $_ensure(0);

  @$pb.TagNumber(2)
  InTest get in_2 => $_getN(1);
  @$pb.TagNumber(2)
  set in_2(InTest value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasIn_2() => $_has(1);
  @$pb.TagNumber(2)
  void clearIn_2() => $_clearField(2);
  @$pb.TagNumber(2)
  InTest ensureIn_2() => $_ensure(1);

  @$pb.TagNumber(3)
  IntRange get intRange => $_getN(2);
  @$pb.TagNumber(3)
  set intRange(IntRange value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasIntRange() => $_has(2);
  @$pb.TagNumber(3)
  void clearIntRange() => $_clearField(3);
  @$pb.TagNumber(3)
  IntRange ensureIntRange() => $_ensure(2);

  @$pb.TagNumber(4)
  NumberRange get numberRange => $_getN(3);
  @$pb.TagNumber(4)
  set numberRange(NumberRange value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasNumberRange() => $_has(3);
  @$pb.TagNumber(4)
  void clearNumberRange() => $_clearField(4);
  @$pb.TagNumber(4)
  NumberRange ensureNumberRange() => $_ensure(3);

  @$pb.TagNumber(5)
  $core.bool get is_5 => $_getBF(4);
  @$pb.TagNumber(5)
  set is_5($core.bool value) => $_setBool(4, value);
  @$pb.TagNumber(5)
  $core.bool hasIs_5() => $_has(4);
  @$pb.TagNumber(5)
  void clearIs_5() => $_clearField(5);
}

class ExprList extends $pb.GeneratedMessage {
  factory ExprList({
    $core.Iterable<Expr>? items,
  }) {
    final result = ExprList._();
    if (items != null) result.items.addAll(items);
    return result;
  }

  ExprList._();

  factory ExprList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExprList()..mergeFromBuffer(data, registry);
  factory ExprList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExprList()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExprList',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ExprList.$_createMessage)
    ..pPM<Expr>(1, _omitFieldNames ? '' : 'items',
        subBuilder: Expr.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExprList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExprList copyWith(void Function(ExprList) updates) =>
      super.copyWith((message) => updates(message as ExprList)) as ExprList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ExprList() / ExprList.new instead')
  static ExprList create() => ExprList._();
  static $pb.GeneratedMessage $_createMessage() => ExprList._();
  @$core.override
  ExprList createEmptyInstance() => ExprList._();
  @$core.pragma('dart2js:noInline')
  static ExprList getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ExprList>(ExprList.$_createMessage);
  static ExprList? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Expr> get items => $_getList(0);
}

/// A scheme by its code text, and the plan or discard scheme within it. `entry` is required when
/// the code holds more than one.
class SchemeRef extends $pb.GeneratedMessage {
  factory SchemeRef({
    $core.String? code,
    $core.int? entry,
  }) {
    final result = SchemeRef._();
    if (code != null) result.code = code;
    if (entry != null) result.entry = entry;
    return result;
  }

  SchemeRef._();

  factory SchemeRef.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchemeRef()..mergeFromBuffer(data, registry);
  factory SchemeRef.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchemeRef()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SchemeRef',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SchemeRef.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'code')
    ..aI(2, _omitFieldNames ? '' : 'entry', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchemeRef clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchemeRef copyWith(void Function(SchemeRef) updates) =>
      super.copyWith((message) => updates(message as SchemeRef)) as SchemeRef;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SchemeRef() / SchemeRef.new instead')
  static SchemeRef create() => SchemeRef._();
  static $pb.GeneratedMessage $_createMessage() => SchemeRef._();
  @$core.override
  SchemeRef createEmptyInstance() => SchemeRef._();
  @$core.pragma('dart2js:noInline')
  static SchemeRef getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SchemeRef>(SchemeRef.$_createMessage);
  static SchemeRef? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get code => $_getSZ(0);
  @$pb.TagNumber(1)
  set code($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get entry => $_getIZ(1);
  @$pb.TagNumber(2)
  set entry($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasEntry() => $_has(1);
  @$pb.TagNumber(2)
  void clearEntry() => $_clearField(2);
}

enum Expr_Kind { and, or, not, pred, matches, matchesScheme, notSet }

/// A filter node. At most 256 nodes and depth 16 in one filter.
class Expr extends $pb.GeneratedMessage {
  factory Expr({
    ExprList? and,
    ExprList? or,
    Expr? not,
    Predicate? pred,
    SoulSelection? matches,
    SchemeRef? matchesScheme,
  }) {
    final result = Expr._();
    if (and != null) result.and = and;
    if (or != null) result.or = or;
    if (not != null) result.not = not;
    if (pred != null) result.pred = pred;
    if (matches != null) result.matches = matches;
    if (matchesScheme != null) result.matchesScheme = matchesScheme;
    return result;
  }

  Expr._();

  factory Expr.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Expr()..mergeFromBuffer(data, registry);
  factory Expr.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Expr()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Expr_Kind> _Expr_KindByTag = {
    1: Expr_Kind.and,
    2: Expr_Kind.or,
    3: Expr_Kind.not,
    4: Expr_Kind.pred,
    5: Expr_Kind.matches,
    6: Expr_Kind.matchesScheme,
    0: Expr_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Expr',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Expr.$_createMessage)
    ..oo(0, [1, 2, 3, 4, 5, 6])
    ..aOM<ExprList>(1, _omitFieldNames ? '' : 'and',
        subBuilder: ExprList.$_createMessage)
    ..aOM<ExprList>(2, _omitFieldNames ? '' : 'or',
        subBuilder: ExprList.$_createMessage)
    ..aOM<Expr>(3, _omitFieldNames ? '' : 'not',
        subBuilder: Expr.$_createMessage)
    ..aOM<Predicate>(4, _omitFieldNames ? '' : 'pred',
        subBuilder: Predicate.$_createMessage)
    ..aOM<SoulSelection>(5, _omitFieldNames ? '' : 'matches',
        subBuilder: SoulSelection.$_createMessage)
    ..aOM<SchemeRef>(6, _omitFieldNames ? '' : 'matchesScheme',
        subBuilder: SchemeRef.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Expr clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Expr copyWith(void Function(Expr) updates) =>
      super.copyWith((message) => updates(message as Expr)) as Expr;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Expr() / Expr.new instead')
  static Expr create() => Expr._();
  static $pb.GeneratedMessage $_createMessage() => Expr._();
  @$core.override
  Expr createEmptyInstance() => Expr._();
  @$core.pragma('dart2js:noInline')
  static Expr getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Expr>(Expr.$_createMessage);
  static Expr? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  Expr_Kind whichKind() => _Expr_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  @$pb.TagNumber(6)
  void clearKind() => $_clearField($_whichOneof(0));

  /// Empty: true.
  @$pb.TagNumber(1)
  ExprList get and => $_getN(0);
  @$pb.TagNumber(1)
  set and(ExprList value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasAnd() => $_has(0);
  @$pb.TagNumber(1)
  void clearAnd() => $_clearField(1);
  @$pb.TagNumber(1)
  ExprList ensureAnd() => $_ensure(0);

  /// Empty: false.
  @$pb.TagNumber(2)
  ExprList get or => $_getN(1);
  @$pb.TagNumber(2)
  set or(ExprList value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasOr() => $_has(1);
  @$pb.TagNumber(2)
  void clearOr() => $_clearField(2);
  @$pb.TagNumber(2)
  ExprList ensureOr() => $_ensure(1);

  @$pb.TagNumber(3)
  Expr get not => $_getN(2);
  @$pb.TagNumber(3)
  set not(Expr value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasNot() => $_has(2);
  @$pb.TagNumber(3)
  void clearNot() => $_clearField(3);
  @$pb.TagNumber(3)
  Expr ensureNot() => $_ensure(2);

  @$pb.TagNumber(4)
  Predicate get pred => $_getN(3);
  @$pb.TagNumber(4)
  set pred(Predicate value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasPred() => $_has(3);
  @$pb.TagNumber(4)
  void clearPred() => $_clearField(4);
  @$pb.TagNumber(4)
  Predicate ensurePred() => $_ensure(3);

  @$pb.TagNumber(5)
  SoulSelection get matches => $_getN(4);
  @$pb.TagNumber(5)
  set matches(SoulSelection value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasMatches() => $_has(4);
  @$pb.TagNumber(5)
  void clearMatches() => $_clearField(5);
  @$pb.TagNumber(5)
  SoulSelection ensureMatches() => $_ensure(4);

  @$pb.TagNumber(6)
  SchemeRef get matchesScheme => $_getN(5);
  @$pb.TagNumber(6)
  set matchesScheme(SchemeRef value) => $_setField(6, value);
  @$pb.TagNumber(6)
  $core.bool hasMatchesScheme() => $_has(5);
  @$pb.TagNumber(6)
  void clearMatchesScheme() => $_clearField(6);
  @$pb.TagNumber(6)
  SchemeRef ensureMatchesScheme() => $_ensure(5);
}

class SortKey extends $pb.GeneratedMessage {
  factory SortKey({
    Field? field_1,
    Direction? direction,
  }) {
    final result = SortKey._();
    if (field_1 != null) result.field_1 = field_1;
    if (direction != null) result.direction = direction;
    return result;
  }

  SortKey._();

  factory SortKey.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SortKey()..mergeFromBuffer(data, registry);
  factory SortKey.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SortKey()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SortKey',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SortKey.$_createMessage)
    ..aOM<Field>(1, _omitFieldNames ? '' : 'field',
        subBuilder: Field.$_createMessage)
    ..aE<Direction>(2, _omitFieldNames ? '' : 'direction',
        enumValues: Direction.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SortKey clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SortKey copyWith(void Function(SortKey) updates) =>
      super.copyWith((message) => updates(message as SortKey)) as SortKey;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SortKey() / SortKey.new instead')
  static SortKey create() => SortKey._();
  static $pb.GeneratedMessage $_createMessage() => SortKey._();
  @$core.override
  SortKey createEmptyInstance() => SortKey._();
  @$core.pragma('dart2js:noInline')
  static SortKey getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SortKey>(SortKey.$_createMessage);
  static SortKey? _defaultInstance;

  @$pb.TagNumber(1)
  Field get field_1 => $_getN(0);
  @$pb.TagNumber(1)
  set field_1(Field value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasField_1() => $_has(0);
  @$pb.TagNumber(1)
  void clearField_1() => $_clearField(1);
  @$pb.TagNumber(1)
  Field ensureField_1() => $_ensure(0);

  @$pb.TagNumber(2)
  Direction get direction => $_getN(1);
  @$pb.TagNumber(2)
  set direction(Direction value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasDirection() => $_has(1);
  @$pb.TagNumber(2)
  void clearDirection() => $_clearField(2);
}

class ParamSetRef extends $pb.GeneratedMessage {
  factory ParamSetRef({
    $core.String? id,
    $core.int? version,
  }) {
    final result = ParamSetRef._();
    if (id != null) result.id = id;
    if (version != null) result.version = version;
    return result;
  }

  ParamSetRef._();

  factory ParamSetRef.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ParamSetRef()..mergeFromBuffer(data, registry);
  factory ParamSetRef.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ParamSetRef()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ParamSetRef',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ParamSetRef.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aI(2, _omitFieldNames ? '' : 'version', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ParamSetRef clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ParamSetRef copyWith(void Function(ParamSetRef) updates) =>
      super.copyWith((message) => updates(message as ParamSetRef))
          as ParamSetRef;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ParamSetRef() / ParamSetRef.new instead')
  static ParamSetRef create() => ParamSetRef._();
  static $pb.GeneratedMessage $_createMessage() => ParamSetRef._();
  @$core.override
  ParamSetRef createEmptyInstance() => ParamSetRef._();
  @$core.pragma('dart2js:noInline')
  static ParamSetRef getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ParamSetRef>(
          ParamSetRef.$_createMessage);
  static ParamSetRef? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.int get version => $_getIZ(1);
  @$pb.TagNumber(2)
  set version($core.int value) => $_setUnsignedInt32(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearVersion() => $_clearField(2);
}

class PageRequest extends $pb.GeneratedMessage {
  factory PageRequest({
    $core.int? rowBudget,
    $core.List<$core.int>? cursor,
  }) {
    final result = PageRequest._();
    if (rowBudget != null) result.rowBudget = rowBudget;
    if (cursor != null) result.cursor = cursor;
    return result;
  }

  PageRequest._();

  factory PageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PageRequest()..mergeFromBuffer(data, registry);
  factory PageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      PageRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PageRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: PageRequest.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'rowBudget', fieldType: $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'cursor', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PageRequest copyWith(void Function(PageRequest) updates) =>
      super.copyWith((message) => updates(message as PageRequest))
          as PageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use PageRequest() / PageRequest.new instead')
  static PageRequest create() => PageRequest._();
  static $pb.GeneratedMessage $_createMessage() => PageRequest._();
  @$core.override
  PageRequest createEmptyInstance() => PageRequest._();
  @$core.pragma('dart2js:noInline')
  static PageRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PageRequest>(
          PageRequest.$_createMessage);
  static PageRequest? _defaultInstance;

  /// Absent: the daemon's default. 1 to 10 000.
  @$pb.TagNumber(1)
  $core.int get rowBudget => $_getIZ(0);
  @$pb.TagNumber(1)
  set rowBudget($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRowBudget() => $_has(0);
  @$pb.TagNumber(1)
  void clearRowBudget() => $_clearField(1);

  /// Absent: the first page. Otherwise a cursor this query returned, unmodified.
  @$pb.TagNumber(2)
  $core.List<$core.int> get cursor => $_getN(1);
  @$pb.TagNumber(2)
  set cursor($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCursor() => $_has(1);
  @$pb.TagNumber(2)
  void clearCursor() => $_clearField(2);
}

class Query extends $pb.GeneratedMessage {
  factory Query({
    Collection? collection,
    Expr? filter,
    $core.Iterable<SortKey>? sort,
    ParamSetRef? params,
  }) {
    final result = Query._();
    if (collection != null) result.collection = collection;
    if (filter != null) result.filter = filter;
    if (sort != null) result.sort.addAll(sort);
    if (params != null) result.params = params;
    return result;
  }

  Query._();

  factory Query.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Query()..mergeFromBuffer(data, registry);
  factory Query.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Query()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Query',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Query.$_createMessage)
    ..aE<Collection>(1, _omitFieldNames ? '' : 'collection',
        enumValues: Collection.values)
    ..aOM<Expr>(2, _omitFieldNames ? '' : 'filter',
        subBuilder: Expr.$_createMessage)
    ..pPM<SortKey>(3, _omitFieldNames ? '' : 'sort',
        subBuilder: SortKey.$_createMessage)
    ..aOM<ParamSetRef>(4, _omitFieldNames ? '' : 'params',
        subBuilder: ParamSetRef.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Query clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Query copyWith(void Function(Query) updates) =>
      super.copyWith((message) => updates(message as Query)) as Query;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Query() / Query.new instead')
  static Query create() => Query._();
  static $pb.GeneratedMessage $_createMessage() => Query._();
  @$core.override
  Query createEmptyInstance() => Query._();
  @$core.pragma('dart2js:noInline')
  static Query getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Query>(Query.$_createMessage);
  static Query? _defaultInstance;

  @$pb.TagNumber(1)
  Collection get collection => $_getN(0);
  @$pb.TagNumber(1)
  set collection(Collection value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCollection() => $_has(0);
  @$pb.TagNumber(1)
  void clearCollection() => $_clearField(1);

  /// Absent: every row.
  @$pb.TagNumber(2)
  Expr get filter => $_getN(1);
  @$pb.TagNumber(2)
  set filter(Expr value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasFilter() => $_has(1);
  @$pb.TagNumber(2)
  void clearFilter() => $_clearField(2);
  @$pb.TagNumber(2)
  Expr ensureFilter() => $_ensure(1);

  /// At most 8 keys; the row identity is the final key.
  @$pb.TagNumber(3)
  $pb.PbList<SortKey> get sort => $_getList(2);

  @$pb.TagNumber(4)
  ParamSetRef get params => $_getN(3);
  @$pb.TagNumber(4)
  set params(ParamSetRef value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasParams() => $_has(3);
  @$pb.TagNumber(4)
  void clearParams() => $_clearField(4);
  @$pb.TagNumber(4)
  ParamSetRef ensureParams() => $_ensure(3);
}

/// The row is in the game's selection however the open rules are settled (ADR-0026).
class ExactVerdict extends $pb.GeneratedMessage {
  factory ExactVerdict() => ExactVerdict._();

  ExactVerdict._();

  factory ExactVerdict.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExactVerdict()..mergeFromBuffer(data, registry);
  factory ExactVerdict.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ExactVerdict()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ExactVerdict',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ExactVerdict.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExactVerdict clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ExactVerdict copyWith(void Function(ExactVerdict) updates) =>
      super.copyWith((message) => updates(message as ExactVerdict))
          as ExactVerdict;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ExactVerdict() / ExactVerdict.new instead')
  static ExactVerdict create() => ExactVerdict._();
  static $pb.GeneratedMessage $_createMessage() => ExactVerdict._();
  @$core.override
  ExactVerdict createEmptyInstance() => ExactVerdict._();
  @$core.pragma('dart2js:noInline')
  static ExactVerdict getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ExactVerdict>(
          ExactVerdict.$_createMessage);
  static ExactVerdict? _defaultInstance;
}

/// The row is open, and rests on these rules: at least one, each once.
class OpenVerdict extends $pb.GeneratedMessage {
  factory OpenVerdict({
    $core.Iterable<OpenRule>? rules,
  }) {
    final result = OpenVerdict._();
    if (rules != null) result.rules.addAll(rules);
    return result;
  }

  OpenVerdict._();

  factory OpenVerdict.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OpenVerdict()..mergeFromBuffer(data, registry);
  factory OpenVerdict.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OpenVerdict()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OpenVerdict',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: OpenVerdict.$_createMessage)
    ..pc<OpenRule>(1, _omitFieldNames ? '' : 'rules', $pb.PbFieldType.KE,
        valueOf: OpenRule.valueOf,
        enumValues: OpenRule.values,
        defaultEnumValue: OpenRule.OPEN_RULE_UNSPECIFIED)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenVerdict clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenVerdict copyWith(void Function(OpenVerdict) updates) =>
      super.copyWith((message) => updates(message as OpenVerdict))
          as OpenVerdict;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use OpenVerdict() / OpenVerdict.new instead')
  static OpenVerdict create() => OpenVerdict._();
  static $pb.GeneratedMessage $_createMessage() => OpenVerdict._();
  @$core.override
  OpenVerdict createEmptyInstance() => OpenVerdict._();
  @$core.pragma('dart2js:noInline')
  static OpenVerdict getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OpenVerdict>(
          OpenVerdict.$_createMessage);
  static OpenVerdict? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<OpenRule> get rules => $_getList(0);
}

enum QueryRow_Verdict { exact, open, notSet }

class QueryRow extends $pb.GeneratedMessage {
  factory QueryRow({
    $core.String? soulId,
    ExactVerdict? exact,
    OpenVerdict? open,
  }) {
    final result = QueryRow._();
    if (soulId != null) result.soulId = soulId;
    if (exact != null) result.exact = exact;
    if (open != null) result.open = open;
    return result;
  }

  QueryRow._();

  factory QueryRow.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryRow()..mergeFromBuffer(data, registry);
  factory QueryRow.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryRow()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, QueryRow_Verdict> _QueryRow_VerdictByTag = {
    4: QueryRow_Verdict.exact,
    5: QueryRow_Verdict.open,
    0: QueryRow_Verdict.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryRow',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryRow.$_createMessage)
    ..oo(0, [4, 5])
    ..aOS(1, _omitFieldNames ? '' : 'soulId')
    ..aOM<ExactVerdict>(4, _omitFieldNames ? '' : 'exact',
        subBuilder: ExactVerdict.$_createMessage)
    ..aOM<OpenVerdict>(5, _omitFieldNames ? '' : 'open',
        subBuilder: OpenVerdict.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryRow clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryRow copyWith(void Function(QueryRow) updates) =>
      super.copyWith((message) => updates(message as QueryRow)) as QueryRow;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryRow() / QueryRow.new instead')
  static QueryRow create() => QueryRow._();
  static $pb.GeneratedMessage $_createMessage() => QueryRow._();
  @$core.override
  QueryRow createEmptyInstance() => QueryRow._();
  @$core.pragma('dart2js:noInline')
  static QueryRow getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryRow>(QueryRow.$_createMessage);
  static QueryRow? _defaultInstance;

  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  QueryRow_Verdict whichVerdict() => _QueryRow_VerdictByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  void clearVerdict() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get soulId => $_getSZ(0);
  @$pb.TagNumber(1)
  set soulId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSoulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSoulId() => $_clearField(1);

  @$pb.TagNumber(4)
  ExactVerdict get exact => $_getN(1);
  @$pb.TagNumber(4)
  set exact(ExactVerdict value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasExact() => $_has(1);
  @$pb.TagNumber(4)
  void clearExact() => $_clearField(4);
  @$pb.TagNumber(4)
  ExactVerdict ensureExact() => $_ensure(1);

  @$pb.TagNumber(5)
  OpenVerdict get open => $_getN(2);
  @$pb.TagNumber(5)
  set open(OpenVerdict value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasOpen() => $_has(2);
  @$pb.TagNumber(5)
  void clearOpen() => $_clearField(5);
  @$pb.TagNumber(5)
  OpenVerdict ensureOpen() => $_ensure(2);
}

class QueryPage extends $pb.GeneratedMessage {
  factory QueryPage({
    $core.Iterable<QueryRow>? rows,
    $fixnum.Int64? total,
    $core.List<$core.int>? nextCursor,
  }) {
    final result = QueryPage._();
    if (rows != null) result.rows.addAll(rows);
    if (total != null) result.total = total;
    if (nextCursor != null) result.nextCursor = nextCursor;
    return result;
  }

  QueryPage._();

  factory QueryPage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryPage()..mergeFromBuffer(data, registry);
  factory QueryPage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryPage()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryPage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryPage.$_createMessage)
    ..pPM<QueryRow>(1, _omitFieldNames ? '' : 'rows',
        subBuilder: QueryRow.$_createMessage)
    ..a<$fixnum.Int64>(5, _omitFieldNames ? '' : 'total', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(
        6, _omitFieldNames ? '' : 'nextCursor', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryPage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryPage copyWith(void Function(QueryPage) updates) =>
      super.copyWith((message) => updates(message as QueryPage)) as QueryPage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryPage() / QueryPage.new instead')
  static QueryPage create() => QueryPage._();
  static $pb.GeneratedMessage $_createMessage() => QueryPage._();
  @$core.override
  QueryPage createEmptyInstance() => QueryPage._();
  @$core.pragma('dart2js:noInline')
  static QueryPage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryPage>(QueryPage.$_createMessage);
  static QueryPage? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<QueryRow> get rows => $_getList(0);

  /// Every row the query keeps, exact or open, across all pages.
  @$pb.TagNumber(5)
  $fixnum.Int64 get total => $_getI64(1);
  @$pb.TagNumber(5)
  set total($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(5)
  $core.bool hasTotal() => $_has(1);
  @$pb.TagNumber(5)
  void clearTotal() => $_clearField(5);

  /// Present when more rows follow: the cursor of the next page. Opaque: sent back unmodified,
  /// never parsed.
  @$pb.TagNumber(6)
  $core.List<$core.int> get nextCursor => $_getN(2);
  @$pb.TagNumber(6)
  set nextCursor($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(6)
  $core.bool hasNextCursor() => $_has(2);
  @$pb.TagNumber(6)
  void clearNextCursor() => $_clearField(6);
}

/// A query over an inventory the request supplies, until the projection exists. One
/// EvaluateQueryResult answers each request, with the same `id`.
class EvaluateQuery extends $pb.GeneratedMessage {
  factory EvaluateQuery({
    $fixnum.Int64? id,
    ProtocolVersion? protocolVersion,
    $core.Iterable<Soul>? inventory,
    Query? query,
    PageRequest? page,
  }) {
    final result = EvaluateQuery._();
    if (id != null) result.id = id;
    if (protocolVersion != null) result.protocolVersion = protocolVersion;
    if (inventory != null) result.inventory.addAll(inventory);
    if (query != null) result.query = query;
    if (page != null) result.page = page;
    return result;
  }

  EvaluateQuery._();

  factory EvaluateQuery.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EvaluateQuery()..mergeFromBuffer(data, registry);
  factory EvaluateQuery.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EvaluateQuery()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EvaluateQuery',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: EvaluateQuery.$_createMessage)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<ProtocolVersion>(2, _omitFieldNames ? '' : 'protocolVersion',
        subBuilder: ProtocolVersion.$_createMessage)
    ..pPM<Soul>(3, _omitFieldNames ? '' : 'inventory',
        subBuilder: Soul.$_createMessage)
    ..aOM<Query>(4, _omitFieldNames ? '' : 'query',
        subBuilder: Query.$_createMessage)
    ..aOM<PageRequest>(5, _omitFieldNames ? '' : 'page',
        subBuilder: PageRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EvaluateQuery clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EvaluateQuery copyWith(void Function(EvaluateQuery) updates) =>
      super.copyWith((message) => updates(message as EvaluateQuery))
          as EvaluateQuery;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use EvaluateQuery() / EvaluateQuery.new instead')
  static EvaluateQuery create() => EvaluateQuery._();
  static $pb.GeneratedMessage $_createMessage() => EvaluateQuery._();
  @$core.override
  EvaluateQuery createEmptyInstance() => EvaluateQuery._();
  @$core.pragma('dart2js:noInline')
  static EvaluateQuery getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EvaluateQuery>(
          EvaluateQuery.$_createMessage);
  static EvaluateQuery? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  ProtocolVersion get protocolVersion => $_getN(1);
  @$pb.TagNumber(2)
  set protocolVersion(ProtocolVersion value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasProtocolVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearProtocolVersion() => $_clearField(2);
  @$pb.TagNumber(2)
  ProtocolVersion ensureProtocolVersion() => $_ensure(1);

  /// At most 100 000 souls.
  @$pb.TagNumber(3)
  $pb.PbList<Soul> get inventory => $_getList(2);

  @$pb.TagNumber(4)
  Query get query => $_getN(3);
  @$pb.TagNumber(4)
  set query(Query value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasQuery() => $_has(3);
  @$pb.TagNumber(4)
  void clearQuery() => $_clearField(4);
  @$pb.TagNumber(4)
  Query ensureQuery() => $_ensure(3);

  @$pb.TagNumber(5)
  PageRequest get page => $_getN(4);
  @$pb.TagNumber(5)
  set page(PageRequest value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasPage() => $_has(4);
  @$pb.TagNumber(5)
  void clearPage() => $_clearField(5);
  @$pb.TagNumber(5)
  PageRequest ensurePage() => $_ensure(4);
}

/// A request frame that is not an `EvaluateQuery`, so no id can be answered.
class Undecodable extends $pb.GeneratedMessage {
  factory Undecodable() => Undecodable._();

  Undecodable._();

  factory Undecodable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Undecodable()..mergeFromBuffer(data, registry);
  factory Undecodable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Undecodable()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Undecodable',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Undecodable.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Undecodable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Undecodable copyWith(void Function(Undecodable) updates) =>
      super.copyWith((message) => updates(message as Undecodable))
          as Undecodable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Undecodable() / Undecodable.new instead')
  static Undecodable create() => Undecodable._();
  static $pb.GeneratedMessage $_createMessage() => Undecodable._();
  @$core.override
  Undecodable createEmptyInstance() => Undecodable._();
  @$core.pragma('dart2js:noInline')
  static Undecodable getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Undecodable>(
          Undecodable.$_createMessage);
  static Undecodable? _defaultInstance;
}

enum EvaluateQueryResult_Subject { id, undecodable, notSet }

enum EvaluateQueryResult_Outcome { page, error, notSet }

class EvaluateQueryResult extends $pb.GeneratedMessage {
  factory EvaluateQueryResult({
    $fixnum.Int64? id,
    QueryPage? page,
    Error? error,
    Undecodable? undecodable,
  }) {
    final result = EvaluateQueryResult._();
    if (id != null) result.id = id;
    if (page != null) result.page = page;
    if (error != null) result.error = error;
    if (undecodable != null) result.undecodable = undecodable;
    return result;
  }

  EvaluateQueryResult._();

  factory EvaluateQueryResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EvaluateQueryResult()..mergeFromBuffer(data, registry);
  factory EvaluateQueryResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EvaluateQueryResult()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, EvaluateQueryResult_Subject>
      _EvaluateQueryResult_SubjectByTag = {
    1: EvaluateQueryResult_Subject.id,
    4: EvaluateQueryResult_Subject.undecodable,
    0: EvaluateQueryResult_Subject.notSet
  };
  static const $core.Map<$core.int, EvaluateQueryResult_Outcome>
      _EvaluateQueryResult_OutcomeByTag = {
    2: EvaluateQueryResult_Outcome.page,
    3: EvaluateQueryResult_Outcome.error,
    0: EvaluateQueryResult_Outcome.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EvaluateQueryResult',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: EvaluateQueryResult.$_createMessage)
    ..oo(0, [1, 4])
    ..oo(1, [2, 3])
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<QueryPage>(2, _omitFieldNames ? '' : 'page',
        subBuilder: QueryPage.$_createMessage)
    ..aOM<Error>(3, _omitFieldNames ? '' : 'error',
        subBuilder: Error.$_createMessage)
    ..aOM<Undecodable>(4, _omitFieldNames ? '' : 'undecodable',
        subBuilder: Undecodable.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EvaluateQueryResult clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EvaluateQueryResult copyWith(void Function(EvaluateQueryResult) updates) =>
      super.copyWith((message) => updates(message as EvaluateQueryResult))
          as EvaluateQueryResult;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use EvaluateQueryResult() / EvaluateQueryResult.new instead')
  static EvaluateQueryResult create() => EvaluateQueryResult._();
  static $pb.GeneratedMessage $_createMessage() => EvaluateQueryResult._();
  @$core.override
  EvaluateQueryResult createEmptyInstance() => EvaluateQueryResult._();
  @$core.pragma('dart2js:noInline')
  static EvaluateQueryResult getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EvaluateQueryResult>(
          EvaluateQueryResult.$_createMessage);
  static EvaluateQueryResult? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(4)
  EvaluateQueryResult_Subject whichSubject() =>
      _EvaluateQueryResult_SubjectByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(4)
  void clearSubject() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  EvaluateQueryResult_Outcome whichOutcome() =>
      _EvaluateQueryResult_OutcomeByTag[$_whichOneof(1)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  void clearOutcome() => $_clearField($_whichOneof(1));

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  QueryPage get page => $_getN(1);
  @$pb.TagNumber(2)
  set page(QueryPage value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(2)
  void clearPage() => $_clearField(2);
  @$pb.TagNumber(2)
  QueryPage ensurePage() => $_ensure(1);

  @$pb.TagNumber(3)
  Error get error => $_getN(2);
  @$pb.TagNumber(3)
  set error(Error value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasError() => $_has(2);
  @$pb.TagNumber(3)
  void clearError() => $_clearField(3);
  @$pb.TagNumber(3)
  Error ensureError() => $_ensure(2);

  @$pb.TagNumber(4)
  Undecodable get undecodable => $_getN(3);
  @$pb.TagNumber(4)
  set undecodable(Undecodable value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasUndecodable() => $_has(3);
  @$pb.TagNumber(4)
  void clearUndecodable() => $_clearField(4);
  @$pb.TagNumber(4)
  Undecodable ensureUndecodable() => $_ensure(3);
}

enum ClientMessage_Kind {
  openSession,
  shutdown,
  subscribe,
  listProfiles,
  decodeSchemeCode,
  sessionQuery,
  notSet
}

/// Client to daemon: one request per frame.
class ClientMessage extends $pb.GeneratedMessage {
  factory ClientMessage({
    $fixnum.Int64? id,
    OpenSession? openSession,
    Shutdown? shutdown,
    Subscribe? subscribe,
    ListProfiles? listProfiles,
    DecodeSchemeCode? decodeSchemeCode,
    SessionQuery? sessionQuery,
  }) {
    final result = ClientMessage._();
    if (id != null) result.id = id;
    if (openSession != null) result.openSession = openSession;
    if (shutdown != null) result.shutdown = shutdown;
    if (subscribe != null) result.subscribe = subscribe;
    if (listProfiles != null) result.listProfiles = listProfiles;
    if (decodeSchemeCode != null) result.decodeSchemeCode = decodeSchemeCode;
    if (sessionQuery != null) result.sessionQuery = sessionQuery;
    return result;
  }

  ClientMessage._();

  factory ClientMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientMessage()..mergeFromBuffer(data, registry);
  factory ClientMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientMessage()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ClientMessage_Kind>
      _ClientMessage_KindByTag = {
    10: ClientMessage_Kind.openSession,
    11: ClientMessage_Kind.shutdown,
    12: ClientMessage_Kind.subscribe,
    20: ClientMessage_Kind.listProfiles,
    22: ClientMessage_Kind.decodeSchemeCode,
    23: ClientMessage_Kind.sessionQuery,
    0: ClientMessage_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientMessage.$_createMessage)
    ..oo(0, [10, 11, 12, 20, 22, 23])
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<OpenSession>(10, _omitFieldNames ? '' : 'openSession',
        subBuilder: OpenSession.$_createMessage)
    ..aOM<Shutdown>(11, _omitFieldNames ? '' : 'shutdown',
        subBuilder: Shutdown.$_createMessage)
    ..aOM<Subscribe>(12, _omitFieldNames ? '' : 'subscribe',
        subBuilder: Subscribe.$_createMessage)
    ..aOM<ListProfiles>(20, _omitFieldNames ? '' : 'listProfiles',
        subBuilder: ListProfiles.$_createMessage)
    ..aOM<DecodeSchemeCode>(22, _omitFieldNames ? '' : 'decodeSchemeCode',
        subBuilder: DecodeSchemeCode.$_createMessage)
    ..aOM<SessionQuery>(23, _omitFieldNames ? '' : 'sessionQuery',
        subBuilder: SessionQuery.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientMessage copyWith(void Function(ClientMessage) updates) =>
      super.copyWith((message) => updates(message as ClientMessage))
          as ClientMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ClientMessage() / ClientMessage.new instead')
  static ClientMessage create() => ClientMessage._();
  static $pb.GeneratedMessage $_createMessage() => ClientMessage._();
  @$core.override
  ClientMessage createEmptyInstance() => ClientMessage._();
  @$core.pragma('dart2js:noInline')
  static ClientMessage getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClientMessage>(
          ClientMessage.$_createMessage);
  static ClientMessage? _defaultInstance;

  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(20)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  ClientMessage_Kind whichKind() => _ClientMessage_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(20)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  void clearKind() => $_clearField($_whichOneof(0));

  /// Chosen by the client, unique within the session, never reused. Never zero.
  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(10)
  OpenSession get openSession => $_getN(1);
  @$pb.TagNumber(10)
  set openSession(OpenSession value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasOpenSession() => $_has(1);
  @$pb.TagNumber(10)
  void clearOpenSession() => $_clearField(10);
  @$pb.TagNumber(10)
  OpenSession ensureOpenSession() => $_ensure(1);

  @$pb.TagNumber(11)
  Shutdown get shutdown => $_getN(2);
  @$pb.TagNumber(11)
  set shutdown(Shutdown value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasShutdown() => $_has(2);
  @$pb.TagNumber(11)
  void clearShutdown() => $_clearField(11);
  @$pb.TagNumber(11)
  Shutdown ensureShutdown() => $_ensure(2);

  @$pb.TagNumber(12)
  Subscribe get subscribe => $_getN(3);
  @$pb.TagNumber(12)
  set subscribe(Subscribe value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasSubscribe() => $_has(3);
  @$pb.TagNumber(12)
  void clearSubscribe() => $_clearField(12);
  @$pb.TagNumber(12)
  Subscribe ensureSubscribe() => $_ensure(3);

  /// Queries.
  @$pb.TagNumber(20)
  ListProfiles get listProfiles => $_getN(4);
  @$pb.TagNumber(20)
  set listProfiles(ListProfiles value) => $_setField(20, value);
  @$pb.TagNumber(20)
  $core.bool hasListProfiles() => $_has(4);
  @$pb.TagNumber(20)
  void clearListProfiles() => $_clearField(20);
  @$pb.TagNumber(20)
  ListProfiles ensureListProfiles() => $_ensure(4);

  @$pb.TagNumber(22)
  DecodeSchemeCode get decodeSchemeCode => $_getN(5);
  @$pb.TagNumber(22)
  set decodeSchemeCode(DecodeSchemeCode value) => $_setField(22, value);
  @$pb.TagNumber(22)
  $core.bool hasDecodeSchemeCode() => $_has(5);
  @$pb.TagNumber(22)
  void clearDecodeSchemeCode() => $_clearField(22);
  @$pb.TagNumber(22)
  DecodeSchemeCode ensureDecodeSchemeCode() => $_ensure(5);

  @$pb.TagNumber(23)
  SessionQuery get sessionQuery => $_getN(6);
  @$pb.TagNumber(23)
  set sessionQuery(SessionQuery value) => $_setField(23, value);
  @$pb.TagNumber(23)
  $core.bool hasSessionQuery() => $_has(6);
  @$pb.TagNumber(23)
  void clearSessionQuery() => $_clearField(23);
  @$pb.TagNumber(23)
  SessionQuery ensureSessionQuery() => $_ensure(6);
}

enum ServerMessage_Kind { response, event, notSet }

/// Daemon to client: a response to one request, or an event.
class ServerMessage extends $pb.GeneratedMessage {
  factory ServerMessage({
    Response? response,
    Event? event,
  }) {
    final result = ServerMessage._();
    if (response != null) result.response = response;
    if (event != null) result.event = event;
    return result;
  }

  ServerMessage._();

  factory ServerMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerMessage()..mergeFromBuffer(data, registry);
  factory ServerMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ServerMessage()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ServerMessage_Kind>
      _ServerMessage_KindByTag = {
    1: ServerMessage_Kind.response,
    2: ServerMessage_Kind.event,
    0: ServerMessage_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ServerMessage.$_createMessage)
    ..oo(0, [1, 2])
    ..aOM<Response>(1, _omitFieldNames ? '' : 'response',
        subBuilder: Response.$_createMessage)
    ..aOM<Event>(2, _omitFieldNames ? '' : 'event',
        subBuilder: Event.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerMessage copyWith(void Function(ServerMessage) updates) =>
      super.copyWith((message) => updates(message as ServerMessage))
          as ServerMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ServerMessage() / ServerMessage.new instead')
  static ServerMessage create() => ServerMessage._();
  static $pb.GeneratedMessage $_createMessage() => ServerMessage._();
  @$core.override
  ServerMessage createEmptyInstance() => ServerMessage._();
  @$core.pragma('dart2js:noInline')
  static ServerMessage getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ServerMessage>(
          ServerMessage.$_createMessage);
  static ServerMessage? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  ServerMessage_Kind whichKind() => _ServerMessage_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearKind() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Response get response => $_getN(0);
  @$pb.TagNumber(1)
  set response(Response value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasResponse() => $_has(0);
  @$pb.TagNumber(1)
  void clearResponse() => $_clearField(1);
  @$pb.TagNumber(1)
  Response ensureResponse() => $_ensure(0);

  @$pb.TagNumber(2)
  Event get event => $_getN(1);
  @$pb.TagNumber(2)
  set event(Event value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasEvent() => $_has(1);
  @$pb.TagNumber(2)
  void clearEvent() => $_clearField(2);
  @$pb.TagNumber(2)
  Event ensureEvent() => $_ensure(1);
}

enum Response_Result {
  error,
  sessionOpened,
  shutdownAccepted,
  subscribed,
  profileList,
  schemeCodeDecoded,
  sessionQueryPage,
  notSet
}

/// Exactly one per request, carrying the request's id.
class Response extends $pb.GeneratedMessage {
  factory Response({
    $fixnum.Int64? id,
    Error? error,
    SessionOpened? sessionOpened,
    ShutdownAccepted? shutdownAccepted,
    Subscribed? subscribed,
    ProfileList? profileList,
    SchemeCodeDecoded? schemeCodeDecoded,
    SessionQueryPage? sessionQueryPage,
  }) {
    final result = Response._();
    if (id != null) result.id = id;
    if (error != null) result.error = error;
    if (sessionOpened != null) result.sessionOpened = sessionOpened;
    if (shutdownAccepted != null) result.shutdownAccepted = shutdownAccepted;
    if (subscribed != null) result.subscribed = subscribed;
    if (profileList != null) result.profileList = profileList;
    if (schemeCodeDecoded != null) result.schemeCodeDecoded = schemeCodeDecoded;
    if (sessionQueryPage != null) result.sessionQueryPage = sessionQueryPage;
    return result;
  }

  Response._();

  factory Response.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Response()..mergeFromBuffer(data, registry);
  factory Response.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Response()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Response_Result> _Response_ResultByTag = {
    2: Response_Result.error,
    10: Response_Result.sessionOpened,
    11: Response_Result.shutdownAccepted,
    12: Response_Result.subscribed,
    20: Response_Result.profileList,
    22: Response_Result.schemeCodeDecoded,
    23: Response_Result.sessionQueryPage,
    0: Response_Result.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Response',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Response.$_createMessage)
    ..oo(0, [2, 10, 11, 12, 20, 22, 23])
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<Error>(2, _omitFieldNames ? '' : 'error',
        subBuilder: Error.$_createMessage)
    ..aOM<SessionOpened>(10, _omitFieldNames ? '' : 'sessionOpened',
        subBuilder: SessionOpened.$_createMessage)
    ..aOM<ShutdownAccepted>(11, _omitFieldNames ? '' : 'shutdownAccepted',
        subBuilder: ShutdownAccepted.$_createMessage)
    ..aOM<Subscribed>(12, _omitFieldNames ? '' : 'subscribed',
        subBuilder: Subscribed.$_createMessage)
    ..aOM<ProfileList>(20, _omitFieldNames ? '' : 'profileList',
        subBuilder: ProfileList.$_createMessage)
    ..aOM<SchemeCodeDecoded>(22, _omitFieldNames ? '' : 'schemeCodeDecoded',
        subBuilder: SchemeCodeDecoded.$_createMessage)
    ..aOM<SessionQueryPage>(23, _omitFieldNames ? '' : 'sessionQueryPage',
        subBuilder: SessionQueryPage.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Response clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Response copyWith(void Function(Response) updates) =>
      super.copyWith((message) => updates(message as Response)) as Response;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Response() / Response.new instead')
  static Response create() => Response._();
  static $pb.GeneratedMessage $_createMessage() => Response._();
  @$core.override
  Response createEmptyInstance() => Response._();
  @$core.pragma('dart2js:noInline')
  static Response getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Response>(Response.$_createMessage);
  static Response? _defaultInstance;

  @$pb.TagNumber(2)
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(20)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  Response_Result whichResult() => _Response_ResultByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(20)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  void clearResult() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  Error get error => $_getN(1);
  @$pb.TagNumber(2)
  set error(Error value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasError() => $_has(1);
  @$pb.TagNumber(2)
  void clearError() => $_clearField(2);
  @$pb.TagNumber(2)
  Error ensureError() => $_ensure(1);

  @$pb.TagNumber(10)
  SessionOpened get sessionOpened => $_getN(2);
  @$pb.TagNumber(10)
  set sessionOpened(SessionOpened value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasSessionOpened() => $_has(2);
  @$pb.TagNumber(10)
  void clearSessionOpened() => $_clearField(10);
  @$pb.TagNumber(10)
  SessionOpened ensureSessionOpened() => $_ensure(2);

  @$pb.TagNumber(11)
  ShutdownAccepted get shutdownAccepted => $_getN(3);
  @$pb.TagNumber(11)
  set shutdownAccepted(ShutdownAccepted value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasShutdownAccepted() => $_has(3);
  @$pb.TagNumber(11)
  void clearShutdownAccepted() => $_clearField(11);
  @$pb.TagNumber(11)
  ShutdownAccepted ensureShutdownAccepted() => $_ensure(3);

  @$pb.TagNumber(12)
  Subscribed get subscribed => $_getN(4);
  @$pb.TagNumber(12)
  set subscribed(Subscribed value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasSubscribed() => $_has(4);
  @$pb.TagNumber(12)
  void clearSubscribed() => $_clearField(12);
  @$pb.TagNumber(12)
  Subscribed ensureSubscribed() => $_ensure(4);

  @$pb.TagNumber(20)
  ProfileList get profileList => $_getN(5);
  @$pb.TagNumber(20)
  set profileList(ProfileList value) => $_setField(20, value);
  @$pb.TagNumber(20)
  $core.bool hasProfileList() => $_has(5);
  @$pb.TagNumber(20)
  void clearProfileList() => $_clearField(20);
  @$pb.TagNumber(20)
  ProfileList ensureProfileList() => $_ensure(5);

  @$pb.TagNumber(22)
  SchemeCodeDecoded get schemeCodeDecoded => $_getN(6);
  @$pb.TagNumber(22)
  set schemeCodeDecoded(SchemeCodeDecoded value) => $_setField(22, value);
  @$pb.TagNumber(22)
  $core.bool hasSchemeCodeDecoded() => $_has(6);
  @$pb.TagNumber(22)
  void clearSchemeCodeDecoded() => $_clearField(22);
  @$pb.TagNumber(22)
  SchemeCodeDecoded ensureSchemeCodeDecoded() => $_ensure(6);

  @$pb.TagNumber(23)
  SessionQueryPage get sessionQueryPage => $_getN(7);
  @$pb.TagNumber(23)
  set sessionQueryPage(SessionQueryPage value) => $_setField(23, value);
  @$pb.TagNumber(23)
  $core.bool hasSessionQueryPage() => $_has(7);
  @$pb.TagNumber(23)
  void clearSessionQueryPage() => $_clearField(23);
  @$pb.TagNumber(23)
  SessionQueryPage ensureSessionQueryPage() => $_ensure(7);
}

enum Event_Kind { projectionChanged, warning, sessionFailure, notSet }

/// Not a response: something the client did not ask for.
class Event extends $pb.GeneratedMessage {
  factory Event({
    ProjectionChanged? projectionChanged,
    Warning? warning,
    SessionFailed? sessionFailure,
  }) {
    final result = Event._();
    if (projectionChanged != null) result.projectionChanged = projectionChanged;
    if (warning != null) result.warning = warning;
    if (sessionFailure != null) result.sessionFailure = sessionFailure;
    return result;
  }

  Event._();

  factory Event.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Event()..mergeFromBuffer(data, registry);
  factory Event.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Event()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Event_Kind> _Event_KindByTag = {
    1: Event_Kind.projectionChanged,
    2: Event_Kind.warning,
    4: Event_Kind.sessionFailure,
    0: Event_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Event',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Event.$_createMessage)
    ..oo(0, [1, 2, 4])
    ..aOM<ProjectionChanged>(1, _omitFieldNames ? '' : 'projectionChanged',
        subBuilder: ProjectionChanged.$_createMessage)
    ..aOM<Warning>(2, _omitFieldNames ? '' : 'warning',
        subBuilder: Warning.$_createMessage)
    ..aOM<SessionFailed>(4, _omitFieldNames ? '' : 'sessionFailure',
        subBuilder: SessionFailed.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Event clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Event copyWith(void Function(Event) updates) =>
      super.copyWith((message) => updates(message as Event)) as Event;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Event() / Event.new instead')
  static Event create() => Event._();
  static $pb.GeneratedMessage $_createMessage() => Event._();
  @$core.override
  Event createEmptyInstance() => Event._();
  @$core.pragma('dart2js:noInline')
  static Event getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Event>(Event.$_createMessage);
  static Event? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(4)
  Event_Kind whichKind() => _Event_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(4)
  void clearKind() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  ProjectionChanged get projectionChanged => $_getN(0);
  @$pb.TagNumber(1)
  set projectionChanged(ProjectionChanged value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProjectionChanged() => $_has(0);
  @$pb.TagNumber(1)
  void clearProjectionChanged() => $_clearField(1);
  @$pb.TagNumber(1)
  ProjectionChanged ensureProjectionChanged() => $_ensure(0);

  @$pb.TagNumber(2)
  Warning get warning => $_getN(1);
  @$pb.TagNumber(2)
  set warning(Warning value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasWarning() => $_has(1);
  @$pb.TagNumber(2)
  void clearWarning() => $_clearField(2);
  @$pb.TagNumber(2)
  Warning ensureWarning() => $_ensure(1);

  /// The session cannot continue, and the daemon exits after sending it.
  @$pb.TagNumber(4)
  SessionFailed get sessionFailure => $_getN(2);
  @$pb.TagNumber(4)
  set sessionFailure(SessionFailed value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasSessionFailure() => $_has(2);
  @$pb.TagNumber(4)
  void clearSessionFailure() => $_clearField(4);
  @$pb.TagNumber(4)
  SessionFailed ensureSessionFailure() => $_ensure(2);
}

/// The first request of a session. Anything before it is refused with `session.not_open`.
class OpenSession extends $pb.GeneratedMessage {
  factory OpenSession({
    ProtocolVersion? clientVersion,
  }) {
    final result = OpenSession._();
    if (clientVersion != null) result.clientVersion = clientVersion;
    return result;
  }

  OpenSession._();

  factory OpenSession.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OpenSession()..mergeFromBuffer(data, registry);
  factory OpenSession.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      OpenSession()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'OpenSession',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: OpenSession.$_createMessage)
    ..aOM<ProtocolVersion>(1, _omitFieldNames ? '' : 'clientVersion',
        subBuilder: ProtocolVersion.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenSession clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  OpenSession copyWith(void Function(OpenSession) updates) =>
      super.copyWith((message) => updates(message as OpenSession))
          as OpenSession;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use OpenSession() / OpenSession.new instead')
  static OpenSession create() => OpenSession._();
  static $pb.GeneratedMessage $_createMessage() => OpenSession._();
  @$core.override
  OpenSession createEmptyInstance() => OpenSession._();
  @$core.pragma('dart2js:noInline')
  static OpenSession getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OpenSession>(
          OpenSession.$_createMessage);
  static OpenSession? _defaultInstance;

  /// The protocol version the client was built against; absent is refused.
  @$pb.TagNumber(1)
  ProtocolVersion get clientVersion => $_getN(0);
  @$pb.TagNumber(1)
  set clientVersion(ProtocolVersion value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasClientVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearClientVersion() => $_clearField(1);
  @$pb.TagNumber(1)
  ProtocolVersion ensureClientVersion() => $_ensure(0);
}

class SessionOpened extends $pb.GeneratedMessage {
  factory SessionOpened({
    ProtocolVersion? daemonVersion,
    $fixnum.Int64? revision,
  }) {
    final result = SessionOpened._();
    if (daemonVersion != null) result.daemonVersion = daemonVersion;
    if (revision != null) result.revision = revision;
    return result;
  }

  SessionOpened._();

  factory SessionOpened.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionOpened()..mergeFromBuffer(data, registry);
  factory SessionOpened.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionOpened()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionOpened',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionOpened.$_createMessage)
    ..aOM<ProtocolVersion>(1, _omitFieldNames ? '' : 'daemonVersion',
        subBuilder: ProtocolVersion.$_createMessage)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionOpened clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionOpened copyWith(void Function(SessionOpened) updates) =>
      super.copyWith((message) => updates(message as SessionOpened))
          as SessionOpened;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SessionOpened() / SessionOpened.new instead')
  static SessionOpened create() => SessionOpened._();
  static $pb.GeneratedMessage $_createMessage() => SessionOpened._();
  @$core.override
  SessionOpened createEmptyInstance() => SessionOpened._();
  @$core.pragma('dart2js:noInline')
  static SessionOpened getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SessionOpened>(
          SessionOpened.$_createMessage);
  static SessionOpened? _defaultInstance;

  @$pb.TagNumber(1)
  ProtocolVersion get daemonVersion => $_getN(0);
  @$pb.TagNumber(1)
  set daemonVersion(ProtocolVersion value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasDaemonVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearDaemonVersion() => $_clearField(1);
  @$pb.TagNumber(1)
  ProtocolVersion ensureDaemonVersion() => $_ensure(0);

  /// The projection's revision when the session opened: the `seq` of the last commit, 0 for the
  /// empty log (a real revision, not "none").
  @$pb.TagNumber(2)
  $fixnum.Int64 get revision => $_getI64(1);
  @$pb.TagNumber(2)
  set revision($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRevision() => $_has(1);
  @$pb.TagNumber(2)
  void clearRevision() => $_clearField(2);
}

/// Ends the session: the daemon answers, then exits once its output is flushed.
class Shutdown extends $pb.GeneratedMessage {
  factory Shutdown() => Shutdown._();

  Shutdown._();

  factory Shutdown.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Shutdown()..mergeFromBuffer(data, registry);
  factory Shutdown.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Shutdown()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Shutdown',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Shutdown.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Shutdown clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Shutdown copyWith(void Function(Shutdown) updates) =>
      super.copyWith((message) => updates(message as Shutdown)) as Shutdown;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Shutdown() / Shutdown.new instead')
  static Shutdown create() => Shutdown._();
  static $pb.GeneratedMessage $_createMessage() => Shutdown._();
  @$core.override
  Shutdown createEmptyInstance() => Shutdown._();
  @$core.pragma('dart2js:noInline')
  static Shutdown getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Shutdown>(Shutdown.$_createMessage);
  static Shutdown? _defaultInstance;
}

class ShutdownAccepted extends $pb.GeneratedMessage {
  factory ShutdownAccepted() => ShutdownAccepted._();

  ShutdownAccepted._();

  factory ShutdownAccepted.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ShutdownAccepted()..mergeFromBuffer(data, registry);
  factory ShutdownAccepted.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ShutdownAccepted()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ShutdownAccepted',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ShutdownAccepted.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ShutdownAccepted clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ShutdownAccepted copyWith(void Function(ShutdownAccepted) updates) =>
      super.copyWith((message) => updates(message as ShutdownAccepted))
          as ShutdownAccepted;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ShutdownAccepted() / ShutdownAccepted.new instead')
  static ShutdownAccepted create() => ShutdownAccepted._();
  static $pb.GeneratedMessage $_createMessage() => ShutdownAccepted._();
  @$core.override
  ShutdownAccepted createEmptyInstance() => ShutdownAccepted._();
  @$core.pragma('dart2js:noInline')
  static ShutdownAccepted getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ShutdownAccepted>(
          ShutdownAccepted.$_createMessage);
  static ShutdownAccepted? _defaultInstance;
}

/// Asks for a `ProjectionChanged` whenever the projection moves past `revision`.
class Subscribe extends $pb.GeneratedMessage {
  factory Subscribe({
    $fixnum.Int64? revision,
  }) {
    final result = Subscribe._();
    if (revision != null) result.revision = revision;
    return result;
  }

  Subscribe._();

  factory Subscribe.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Subscribe()..mergeFromBuffer(data, registry);
  factory Subscribe.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Subscribe()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Subscribe',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Subscribe.$_createMessage)
    ..a<$fixnum.Int64>(
        1, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Subscribe clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Subscribe copyWith(void Function(Subscribe) updates) =>
      super.copyWith((message) => updates(message as Subscribe)) as Subscribe;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Subscribe() / Subscribe.new instead')
  static Subscribe create() => Subscribe._();
  static $pb.GeneratedMessage $_createMessage() => Subscribe._();
  @$core.override
  Subscribe createEmptyInstance() => Subscribe._();
  @$core.pragma('dart2js:noInline')
  static Subscribe getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Subscribe>(Subscribe.$_createMessage);
  static Subscribe? _defaultInstance;

  /// The latest revision the client holds.
  @$pb.TagNumber(1)
  $fixnum.Int64 get revision => $_getI64(0);
  @$pb.TagNumber(1)
  set revision($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRevision() => $_has(0);
  @$pb.TagNumber(1)
  void clearRevision() => $_clearField(1);
}

class Subscribed extends $pb.GeneratedMessage {
  factory Subscribed({
    $fixnum.Int64? revision,
  }) {
    final result = Subscribed._();
    if (revision != null) result.revision = revision;
    return result;
  }

  Subscribed._();

  factory Subscribed.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Subscribed()..mergeFromBuffer(data, registry);
  factory Subscribed.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Subscribed()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Subscribed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Subscribed.$_createMessage)
    ..a<$fixnum.Int64>(
        1, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Subscribed clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Subscribed copyWith(void Function(Subscribed) updates) =>
      super.copyWith((message) => updates(message as Subscribed)) as Subscribed;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Subscribed() / Subscribed.new instead')
  static Subscribed create() => Subscribed._();
  static $pb.GeneratedMessage $_createMessage() => Subscribed._();
  @$core.override
  Subscribed createEmptyInstance() => Subscribed._();
  @$core.pragma('dart2js:noInline')
  static Subscribed getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Subscribed>(Subscribed.$_createMessage);
  static Subscribed? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get revision => $_getI64(0);
  @$pb.TagNumber(1)
  set revision($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRevision() => $_has(0);
  @$pb.TagNumber(1)
  void clearRevision() => $_clearField(1);
}

/// The projection moved. A revision, never a value: the client queries for what it shows.
class ProjectionChanged extends $pb.GeneratedMessage {
  factory ProjectionChanged({
    $fixnum.Int64? revision,
  }) {
    final result = ProjectionChanged._();
    if (revision != null) result.revision = revision;
    return result;
  }

  ProjectionChanged._();

  factory ProjectionChanged.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProjectionChanged()..mergeFromBuffer(data, registry);
  factory ProjectionChanged.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProjectionChanged()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ProjectionChanged',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ProjectionChanged.$_createMessage)
    ..a<$fixnum.Int64>(
        1, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProjectionChanged clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProjectionChanged copyWith(void Function(ProjectionChanged) updates) =>
      super.copyWith((message) => updates(message as ProjectionChanged))
          as ProjectionChanged;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ProjectionChanged() / ProjectionChanged.new instead')
  static ProjectionChanged create() => ProjectionChanged._();
  static $pb.GeneratedMessage $_createMessage() => ProjectionChanged._();
  @$core.override
  ProjectionChanged createEmptyInstance() => ProjectionChanged._();
  @$core.pragma('dart2js:noInline')
  static ProjectionChanged getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ProjectionChanged>(
          ProjectionChanged.$_createMessage);
  static ProjectionChanged? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get revision => $_getI64(0);
  @$pb.TagNumber(1)
  set revision($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRevision() => $_has(0);
  @$pb.TagNumber(1)
  void clearRevision() => $_clearField(1);
}

class ListProfiles extends $pb.GeneratedMessage {
  factory ListProfiles() => ListProfiles._();

  ListProfiles._();

  factory ListProfiles.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListProfiles()..mergeFromBuffer(data, registry);
  factory ListProfiles.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ListProfiles()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListProfiles',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ListProfiles.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListProfiles clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ListProfiles copyWith(void Function(ListProfiles) updates) =>
      super.copyWith((message) => updates(message as ListProfiles))
          as ListProfiles;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ListProfiles() / ListProfiles.new instead')
  static ListProfiles create() => ListProfiles._();
  static $pb.GeneratedMessage $_createMessage() => ListProfiles._();
  @$core.override
  ListProfiles createEmptyInstance() => ListProfiles._();
  @$core.pragma('dart2js:noInline')
  static ListProfiles getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListProfiles>(
          ListProfiles.$_createMessage);
  static ListProfiles? _defaultInstance;
}

class ProfileList extends $pb.GeneratedMessage {
  factory ProfileList({
    $fixnum.Int64? revision,
    $core.Iterable<Profile>? profiles,
  }) {
    final result = ProfileList._();
    if (revision != null) result.revision = revision;
    if (profiles != null) result.profiles.addAll(profiles);
    return result;
  }

  ProfileList._();

  factory ProfileList.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProfileList()..mergeFromBuffer(data, registry);
  factory ProfileList.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProfileList()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ProfileList',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ProfileList.$_createMessage)
    ..a<$fixnum.Int64>(
        1, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..pPM<Profile>(2, _omitFieldNames ? '' : 'profiles',
        subBuilder: Profile.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProfileList clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProfileList copyWith(void Function(ProfileList) updates) =>
      super.copyWith((message) => updates(message as ProfileList))
          as ProfileList;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ProfileList() / ProfileList.new instead')
  static ProfileList create() => ProfileList._();
  static $pb.GeneratedMessage $_createMessage() => ProfileList._();
  @$core.override
  ProfileList createEmptyInstance() => ProfileList._();
  @$core.pragma('dart2js:noInline')
  static ProfileList getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ProfileList>(
          ProfileList.$_createMessage);
  static ProfileList? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get revision => $_getI64(0);
  @$pb.TagNumber(1)
  set revision($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRevision() => $_has(0);
  @$pb.TagNumber(1)
  void clearRevision() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<Profile> get profiles => $_getList(1);
}

/// One game account (GameProfile).
class Profile extends $pb.GeneratedMessage {
  factory Profile({
    $core.String? id,
    $core.String? name,
    $core.Iterable<ProfileCapability>? capabilities,
  }) {
    final result = Profile._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
    if (capabilities != null) result.capabilities.addAll(capabilities);
    return result;
  }

  Profile._();

  factory Profile.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Profile()..mergeFromBuffer(data, registry);
  factory Profile.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Profile()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Profile',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Profile.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..pPM<ProfileCapability>(3, _omitFieldNames ? '' : 'capabilities',
        subBuilder: ProfileCapability.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Profile clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Profile copyWith(void Function(Profile) updates) =>
      super.copyWith((message) => updates(message as Profile)) as Profile;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Profile() / Profile.new instead')
  static Profile create() => Profile._();
  static $pb.GeneratedMessage $_createMessage() => Profile._();
  @$core.override
  Profile createEmptyInstance() => Profile._();
  @$core.pragma('dart2js:noInline')
  static Profile getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Profile>(Profile.$_createMessage);
  static Profile? _defaultInstance;

  /// The profile's id: 32 lowercase hex digits, its 16 bytes. Sent back unmodified.
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// The user's own name for it.
  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// What the profile can do, derived from the sections its imports hold (snapshot-ir.md,
  /// "Capabilities"), never from a file's format. Every capability of this schema, each once, in
  /// the order of `Capability`.
  @$pb.TagNumber(3)
  $pb.PbList<ProfileCapability> get capabilities => $_getList(2);
}

enum ProfileCapability_Availability { unavailable, available, notSet }

/// One capability of a profile and whether it can be used.
class ProfileCapability extends $pb.GeneratedMessage {
  factory ProfileCapability({
    Capability? capability,
    CapabilityUnavailable? unavailable,
    CapabilityAvailable? available,
  }) {
    final result = ProfileCapability._();
    if (capability != null) result.capability = capability;
    if (unavailable != null) result.unavailable = unavailable;
    if (available != null) result.available = available;
    return result;
  }

  ProfileCapability._();

  factory ProfileCapability.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProfileCapability()..mergeFromBuffer(data, registry);
  factory ProfileCapability.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ProfileCapability()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ProfileCapability_Availability>
      _ProfileCapability_AvailabilityByTag = {
    2: ProfileCapability_Availability.unavailable,
    3: ProfileCapability_Availability.available,
    0: ProfileCapability_Availability.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ProfileCapability',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ProfileCapability.$_createMessage)
    ..oo(0, [2, 3])
    ..aE<Capability>(1, _omitFieldNames ? '' : 'capability',
        enumValues: Capability.values)
    ..aOM<CapabilityUnavailable>(2, _omitFieldNames ? '' : 'unavailable',
        subBuilder: CapabilityUnavailable.$_createMessage)
    ..aOM<CapabilityAvailable>(3, _omitFieldNames ? '' : 'available',
        subBuilder: CapabilityAvailable.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProfileCapability clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ProfileCapability copyWith(void Function(ProfileCapability) updates) =>
      super.copyWith((message) => updates(message as ProfileCapability))
          as ProfileCapability;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ProfileCapability() / ProfileCapability.new instead')
  static ProfileCapability create() => ProfileCapability._();
  static $pb.GeneratedMessage $_createMessage() => ProfileCapability._();
  @$core.override
  ProfileCapability createEmptyInstance() => ProfileCapability._();
  @$core.pragma('dart2js:noInline')
  static ProfileCapability getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ProfileCapability>(
          ProfileCapability.$_createMessage);
  static ProfileCapability? _defaultInstance;

  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  ProfileCapability_Availability whichAvailability() =>
      _ProfileCapability_AvailabilityByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  void clearAvailability() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Capability get capability => $_getN(0);
  @$pb.TagNumber(1)
  set capability(Capability value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCapability() => $_has(0);
  @$pb.TagNumber(1)
  void clearCapability() => $_clearField(1);

  @$pb.TagNumber(2)
  CapabilityUnavailable get unavailable => $_getN(1);
  @$pb.TagNumber(2)
  set unavailable(CapabilityUnavailable value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasUnavailable() => $_has(1);
  @$pb.TagNumber(2)
  void clearUnavailable() => $_clearField(2);
  @$pb.TagNumber(2)
  CapabilityUnavailable ensureUnavailable() => $_ensure(1);

  @$pb.TagNumber(3)
  CapabilityAvailable get available => $_getN(2);
  @$pb.TagNumber(3)
  set available(CapabilityAvailable value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasAvailable() => $_has(2);
  @$pb.TagNumber(3)
  void clearAvailable() => $_clearField(3);
  @$pb.TagNumber(3)
  CapabilityAvailable ensureAvailable() => $_ensure(2);
}

/// The profile lacks sections the capability needs. It is answered with these, never with empty
/// data.
class CapabilityUnavailable extends $pb.GeneratedMessage {
  factory CapabilityUnavailable({
    $core.Iterable<SectionKind>? missing,
  }) {
    final result = CapabilityUnavailable._();
    if (missing != null) result.missing.addAll(missing);
    return result;
  }

  CapabilityUnavailable._();

  factory CapabilityUnavailable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CapabilityUnavailable()..mergeFromBuffer(data, registry);
  factory CapabilityUnavailable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CapabilityUnavailable()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CapabilityUnavailable',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: CapabilityUnavailable.$_createMessage)
    ..pc<SectionKind>(1, _omitFieldNames ? '' : 'missing', $pb.PbFieldType.KE,
        valueOf: SectionKind.valueOf,
        enumValues: SectionKind.values,
        defaultEnumValue: SectionKind.SECTION_KIND_UNSPECIFIED)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CapabilityUnavailable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CapabilityUnavailable copyWith(
          void Function(CapabilityUnavailable) updates) =>
      super.copyWith((message) => updates(message as CapabilityUnavailable))
          as CapabilityUnavailable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use CapabilityUnavailable() / CapabilityUnavailable.new instead')
  static CapabilityUnavailable create() => CapabilityUnavailable._();
  static $pb.GeneratedMessage $_createMessage() => CapabilityUnavailable._();
  @$core.override
  CapabilityUnavailable createEmptyInstance() => CapabilityUnavailable._();
  @$core.pragma('dart2js:noInline')
  static CapabilityUnavailable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CapabilityUnavailable>(
          CapabilityUnavailable.$_createMessage);
  static CapabilityUnavailable? _defaultInstance;

  /// The sections missing, each once; never empty.
  @$pb.TagNumber(1)
  $pb.PbList<SectionKind> get missing => $_getList(0);
}

/// The profile holds every section the capability needs.
class CapabilityAvailable extends $pb.GeneratedMessage {
  factory CapabilityAvailable({
    Completeness? completeness,
  }) {
    final result = CapabilityAvailable._();
    if (completeness != null) result.completeness = completeness;
    return result;
  }

  CapabilityAvailable._();

  factory CapabilityAvailable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CapabilityAvailable()..mergeFromBuffer(data, registry);
  factory CapabilityAvailable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CapabilityAvailable()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CapabilityAvailable',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: CapabilityAvailable.$_createMessage)
    ..aE<Completeness>(1, _omitFieldNames ? '' : 'completeness',
        enumValues: Completeness.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CapabilityAvailable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CapabilityAvailable copyWith(void Function(CapabilityAvailable) updates) =>
      super.copyWith((message) => updates(message as CapabilityAvailable))
          as CapabilityAvailable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use CapabilityAvailable() / CapabilityAvailable.new instead')
  static CapabilityAvailable create() => CapabilityAvailable._();
  static $pb.GeneratedMessage $_createMessage() => CapabilityAvailable._();
  @$core.override
  CapabilityAvailable createEmptyInstance() => CapabilityAvailable._();
  @$core.pragma('dart2js:noInline')
  static CapabilityAvailable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CapabilityAvailable>(
          CapabilityAvailable.$_createMessage);
  static CapabilityAvailable? _defaultInstance;

  /// The weakest completeness of those sections.
  @$pb.TagNumber(1)
  Completeness get completeness => $_getN(0);
  @$pb.TagNumber(1)
  set completeness(Completeness value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCompleteness() => $_has(0);
  @$pb.TagNumber(1)
  void clearCompleteness() => $_clearField(1);
}

enum SessionQuery_Position { first, next, notSet }

/// A query over one profile's souls. A later page carries the cursor and the revision its scan
/// began at together, so a cursor never travels without its scan.
class SessionQuery extends $pb.GeneratedMessage {
  factory SessionQuery({
    $core.String? profileId,
    Query? query,
    $core.int? rowBudget,
    FirstPage? first,
    NextPage? next,
  }) {
    final result = SessionQuery._();
    if (profileId != null) result.profileId = profileId;
    if (query != null) result.query = query;
    if (rowBudget != null) result.rowBudget = rowBudget;
    if (first != null) result.first = first;
    if (next != null) result.next = next;
    return result;
  }

  SessionQuery._();

  factory SessionQuery.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionQuery()..mergeFromBuffer(data, registry);
  factory SessionQuery.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionQuery()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, SessionQuery_Position>
      _SessionQuery_PositionByTag = {
    4: SessionQuery_Position.first,
    5: SessionQuery_Position.next,
    0: SessionQuery_Position.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionQuery',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionQuery.$_createMessage)
    ..oo(0, [4, 5])
    ..aOS(1, _omitFieldNames ? '' : 'profileId')
    ..aOM<Query>(2, _omitFieldNames ? '' : 'query',
        subBuilder: Query.$_createMessage)
    ..aI(3, _omitFieldNames ? '' : 'rowBudget', fieldType: $pb.PbFieldType.OU3)
    ..aOM<FirstPage>(4, _omitFieldNames ? '' : 'first',
        subBuilder: FirstPage.$_createMessage)
    ..aOM<NextPage>(5, _omitFieldNames ? '' : 'next',
        subBuilder: NextPage.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionQuery clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionQuery copyWith(void Function(SessionQuery) updates) =>
      super.copyWith((message) => updates(message as SessionQuery))
          as SessionQuery;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SessionQuery() / SessionQuery.new instead')
  static SessionQuery create() => SessionQuery._();
  static $pb.GeneratedMessage $_createMessage() => SessionQuery._();
  @$core.override
  SessionQuery createEmptyInstance() => SessionQuery._();
  @$core.pragma('dart2js:noInline')
  static SessionQuery getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SessionQuery>(
          SessionQuery.$_createMessage);
  static SessionQuery? _defaultInstance;

  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  SessionQuery_Position whichPosition() =>
      _SessionQuery_PositionByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(4)
  @$pb.TagNumber(5)
  void clearPosition() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get profileId => $_getSZ(0);
  @$pb.TagNumber(1)
  set profileId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProfileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProfileId() => $_clearField(1);

  @$pb.TagNumber(2)
  Query get query => $_getN(1);
  @$pb.TagNumber(2)
  set query(Query value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasQuery() => $_has(1);
  @$pb.TagNumber(2)
  void clearQuery() => $_clearField(2);
  @$pb.TagNumber(2)
  Query ensureQuery() => $_ensure(1);

  /// Absent: the daemon's default. 1 to 10 000.
  @$pb.TagNumber(3)
  $core.int get rowBudget => $_getIZ(2);
  @$pb.TagNumber(3)
  set rowBudget($core.int value) => $_setUnsignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRowBudget() => $_has(2);
  @$pb.TagNumber(3)
  void clearRowBudget() => $_clearField(3);

  @$pb.TagNumber(4)
  FirstPage get first => $_getN(3);
  @$pb.TagNumber(4)
  set first(FirstPage value) => $_setField(4, value);
  @$pb.TagNumber(4)
  $core.bool hasFirst() => $_has(3);
  @$pb.TagNumber(4)
  void clearFirst() => $_clearField(4);
  @$pb.TagNumber(4)
  FirstPage ensureFirst() => $_ensure(3);

  @$pb.TagNumber(5)
  NextPage get next => $_getN(4);
  @$pb.TagNumber(5)
  set next(NextPage value) => $_setField(5, value);
  @$pb.TagNumber(5)
  $core.bool hasNext() => $_has(4);
  @$pb.TagNumber(5)
  void clearNext() => $_clearField(5);
  @$pb.TagNumber(5)
  NextPage ensureNext() => $_ensure(4);
}

class FirstPage extends $pb.GeneratedMessage {
  factory FirstPage() => FirstPage._();

  FirstPage._();

  factory FirstPage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      FirstPage()..mergeFromBuffer(data, registry);
  factory FirstPage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      FirstPage()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FirstPage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: FirstPage.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FirstPage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  FirstPage copyWith(void Function(FirstPage) updates) =>
      super.copyWith((message) => updates(message as FirstPage)) as FirstPage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use FirstPage() / FirstPage.new instead')
  static FirstPage create() => FirstPage._();
  static $pb.GeneratedMessage $_createMessage() => FirstPage._();
  @$core.override
  FirstPage createEmptyInstance() => FirstPage._();
  @$core.pragma('dart2js:noInline')
  static FirstPage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FirstPage>(FirstPage.$_createMessage);
  static FirstPage? _defaultInstance;
}

class NextPage extends $pb.GeneratedMessage {
  factory NextPage({
    $core.List<$core.int>? cursor,
    $fixnum.Int64? scan,
  }) {
    final result = NextPage._();
    if (cursor != null) result.cursor = cursor;
    if (scan != null) result.scan = scan;
    return result;
  }

  NextPage._();

  factory NextPage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NextPage()..mergeFromBuffer(data, registry);
  factory NextPage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NextPage()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NextPage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: NextPage.$_createMessage)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'cursor', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'scan', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NextPage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  NextPage copyWith(void Function(NextPage) updates) =>
      super.copyWith((message) => updates(message as NextPage)) as NextPage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use NextPage() / NextPage.new instead')
  static NextPage create() => NextPage._();
  static $pb.GeneratedMessage $_createMessage() => NextPage._();
  @$core.override
  NextPage createEmptyInstance() => NextPage._();
  @$core.pragma('dart2js:noInline')
  static NextPage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NextPage>(NextPage.$_createMessage);
  static NextPage? _defaultInstance;

  /// A cursor a page of this scan returned, unmodified; never empty.
  @$pb.TagNumber(1)
  $core.List<$core.int> get cursor => $_getN(0);
  @$pb.TagNumber(1)
  set cursor($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCursor() => $_has(0);
  @$pb.TagNumber(1)
  void clearCursor() => $_clearField(1);

  /// The revision the scan's first page was valid at. Refused with `query.stale_revision` when
  /// the projection has moved.
  @$pb.TagNumber(2)
  $fixnum.Int64 get scan => $_getI64(1);
  @$pb.TagNumber(2)
  set scan($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasScan() => $_has(1);
  @$pb.TagNumber(2)
  void clearScan() => $_clearField(2);
}

class SessionQueryPage extends $pb.GeneratedMessage {
  factory SessionQueryPage({
    $core.Iterable<SessionRow>? rows,
    $core.List<$core.int>? nextCursor,
    $fixnum.Int64? total,
    $fixnum.Int64? revision,
  }) {
    final result = SessionQueryPage._();
    if (rows != null) result.rows.addAll(rows);
    if (nextCursor != null) result.nextCursor = nextCursor;
    if (total != null) result.total = total;
    if (revision != null) result.revision = revision;
    return result;
  }

  SessionQueryPage._();

  factory SessionQueryPage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionQueryPage()..mergeFromBuffer(data, registry);
  factory SessionQueryPage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionQueryPage()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionQueryPage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionQueryPage.$_createMessage)
    ..pPM<SessionRow>(1, _omitFieldNames ? '' : 'rows',
        subBuilder: SessionRow.$_createMessage)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'nextCursor', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'total', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        4, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionQueryPage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionQueryPage copyWith(void Function(SessionQueryPage) updates) =>
      super.copyWith((message) => updates(message as SessionQueryPage))
          as SessionQueryPage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SessionQueryPage() / SessionQueryPage.new instead')
  static SessionQueryPage create() => SessionQueryPage._();
  static $pb.GeneratedMessage $_createMessage() => SessionQueryPage._();
  @$core.override
  SessionQueryPage createEmptyInstance() => SessionQueryPage._();
  @$core.pragma('dart2js:noInline')
  static SessionQueryPage getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SessionQueryPage>(
          SessionQueryPage.$_createMessage);
  static SessionQueryPage? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<SessionRow> get rows => $_getList(0);

  /// Present when more rows follow.
  @$pb.TagNumber(2)
  $core.List<$core.int> get nextCursor => $_getN(1);
  @$pb.TagNumber(2)
  set nextCursor($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasNextCursor() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextCursor() => $_clearField(2);

  /// Every row the query keeps, exact or open, across all pages.
  @$pb.TagNumber(3)
  $fixnum.Int64 get total => $_getI64(2);
  @$pb.TagNumber(3)
  set total($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasTotal() => $_has(2);
  @$pb.TagNumber(3)
  void clearTotal() => $_clearField(3);

  /// The revision the page is valid at.
  @$pb.TagNumber(4)
  $fixnum.Int64 get revision => $_getI64(3);
  @$pb.TagNumber(4)
  set revision($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRevision() => $_has(3);
  @$pb.TagNumber(4)
  void clearRevision() => $_clearField(4);
}

enum SessionRow_Verdict { exact, open, notSet }

/// A row with the values the application shows: the soul, and the query's verdict on it.
class SessionRow extends $pb.GeneratedMessage {
  factory SessionRow({
    Soul? soul,
    ExactVerdict? exact,
    OpenVerdict? open,
  }) {
    final result = SessionRow._();
    if (soul != null) result.soul = soul;
    if (exact != null) result.exact = exact;
    if (open != null) result.open = open;
    return result;
  }

  SessionRow._();

  factory SessionRow.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionRow()..mergeFromBuffer(data, registry);
  factory SessionRow.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionRow()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, SessionRow_Verdict>
      _SessionRow_VerdictByTag = {
    2: SessionRow_Verdict.exact,
    3: SessionRow_Verdict.open,
    0: SessionRow_Verdict.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionRow',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionRow.$_createMessage)
    ..oo(0, [2, 3])
    ..aOM<Soul>(1, _omitFieldNames ? '' : 'soul',
        subBuilder: Soul.$_createMessage)
    ..aOM<ExactVerdict>(2, _omitFieldNames ? '' : 'exact',
        subBuilder: ExactVerdict.$_createMessage)
    ..aOM<OpenVerdict>(3, _omitFieldNames ? '' : 'open',
        subBuilder: OpenVerdict.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionRow clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionRow copyWith(void Function(SessionRow) updates) =>
      super.copyWith((message) => updates(message as SessionRow)) as SessionRow;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SessionRow() / SessionRow.new instead')
  static SessionRow create() => SessionRow._();
  static $pb.GeneratedMessage $_createMessage() => SessionRow._();
  @$core.override
  SessionRow createEmptyInstance() => SessionRow._();
  @$core.pragma('dart2js:noInline')
  static SessionRow getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionRow>(SessionRow.$_createMessage);
  static SessionRow? _defaultInstance;

  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  SessionRow_Verdict whichVerdict() =>
      _SessionRow_VerdictByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  void clearVerdict() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Soul get soul => $_getN(0);
  @$pb.TagNumber(1)
  set soul(Soul value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSoul() => $_has(0);
  @$pb.TagNumber(1)
  void clearSoul() => $_clearField(1);
  @$pb.TagNumber(1)
  Soul ensureSoul() => $_ensure(0);

  @$pb.TagNumber(2)
  ExactVerdict get exact => $_getN(1);
  @$pb.TagNumber(2)
  set exact(ExactVerdict value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasExact() => $_has(1);
  @$pb.TagNumber(2)
  void clearExact() => $_clearField(2);
  @$pb.TagNumber(2)
  ExactVerdict ensureExact() => $_ensure(1);

  @$pb.TagNumber(3)
  OpenVerdict get open => $_getN(2);
  @$pb.TagNumber(3)
  set open(OpenVerdict value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasOpen() => $_has(2);
  @$pb.TagNumber(3)
  void clearOpen() => $_clearField(3);
  @$pb.TagNumber(3)
  OpenVerdict ensureOpen() => $_ensure(2);
}

enum DecodeSchemeCode_Source { text, png, notSet }

/// A scheme code from its QR image or its Base64 text.
class DecodeSchemeCode extends $pb.GeneratedMessage {
  factory DecodeSchemeCode({
    $core.String? text,
    $core.List<$core.int>? png,
  }) {
    final result = DecodeSchemeCode._();
    if (text != null) result.text = text;
    if (png != null) result.png = png;
    return result;
  }

  DecodeSchemeCode._();

  factory DecodeSchemeCode.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeSchemeCode()..mergeFromBuffer(data, registry);
  factory DecodeSchemeCode.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeSchemeCode()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, DecodeSchemeCode_Source>
      _DecodeSchemeCode_SourceByTag = {
    1: DecodeSchemeCode_Source.text,
    2: DecodeSchemeCode_Source.png,
    0: DecodeSchemeCode_Source.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeSchemeCode',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeSchemeCode.$_createMessage)
    ..oo(0, [1, 2])
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'png', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeSchemeCode clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeSchemeCode copyWith(void Function(DecodeSchemeCode) updates) =>
      super.copyWith((message) => updates(message as DecodeSchemeCode))
          as DecodeSchemeCode;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DecodeSchemeCode() / DecodeSchemeCode.new instead')
  static DecodeSchemeCode create() => DecodeSchemeCode._();
  static $pb.GeneratedMessage $_createMessage() => DecodeSchemeCode._();
  @$core.override
  DecodeSchemeCode createEmptyInstance() => DecodeSchemeCode._();
  @$core.pragma('dart2js:noInline')
  static DecodeSchemeCode getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DecodeSchemeCode>(
          DecodeSchemeCode.$_createMessage);
  static DecodeSchemeCode? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  DecodeSchemeCode_Source whichSource() =>
      _DecodeSchemeCode_SourceByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearSource() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => $_clearField(1);

  /// One PNG image holding one QR code.
  @$pb.TagNumber(2)
  $core.List<$core.int> get png => $_getN(1);
  @$pb.TagNumber(2)
  set png($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPng() => $_has(1);
  @$pb.TagNumber(2)
  void clearPng() => $_clearField(2);
}

class SchemeCodeDecoded extends $pb.GeneratedMessage {
  factory SchemeCodeDecoded({
    SchemeKind? kind,
    $core.Iterable<SchemeEntry>? entries,
    EncodedScheme? encoded,
  }) {
    final result = SchemeCodeDecoded._();
    if (kind != null) result.kind = kind;
    if (entries != null) result.entries.addAll(entries);
    if (encoded != null) result.encoded = encoded;
    return result;
  }

  SchemeCodeDecoded._();

  factory SchemeCodeDecoded.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchemeCodeDecoded()..mergeFromBuffer(data, registry);
  factory SchemeCodeDecoded.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchemeCodeDecoded()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SchemeCodeDecoded',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SchemeCodeDecoded.$_createMessage)
    ..aE<SchemeKind>(1, _omitFieldNames ? '' : 'kind',
        enumValues: SchemeKind.values)
    ..pPM<SchemeEntry>(2, _omitFieldNames ? '' : 'entries',
        subBuilder: SchemeEntry.$_createMessage)
    ..aOM<EncodedScheme>(3, _omitFieldNames ? '' : 'encoded',
        subBuilder: EncodedScheme.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchemeCodeDecoded clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchemeCodeDecoded copyWith(void Function(SchemeCodeDecoded) updates) =>
      super.copyWith((message) => updates(message as SchemeCodeDecoded))
          as SchemeCodeDecoded;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SchemeCodeDecoded() / SchemeCodeDecoded.new instead')
  static SchemeCodeDecoded create() => SchemeCodeDecoded._();
  static $pb.GeneratedMessage $_createMessage() => SchemeCodeDecoded._();
  @$core.override
  SchemeCodeDecoded createEmptyInstance() => SchemeCodeDecoded._();
  @$core.pragma('dart2js:noInline')
  static SchemeCodeDecoded getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SchemeCodeDecoded>(
          SchemeCodeDecoded.$_createMessage);
  static SchemeCodeDecoded? _defaultInstance;

  @$pb.TagNumber(1)
  SchemeKind get kind => $_getN(0);
  @$pb.TagNumber(1)
  set kind(SchemeKind value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasKind() => $_has(0);
  @$pb.TagNumber(1)
  void clearKind() => $_clearField(1);

  /// The plans of a strengthening scheme set, or the discard schemes, in code order.
  @$pb.TagNumber(2)
  $pb.PbList<SchemeEntry> get entries => $_getList(1);

  /// The code as it was read, for presentation and sharing.
  @$pb.TagNumber(3)
  EncodedScheme get encoded => $_getN(2);
  @$pb.TagNumber(3)
  set encoded(EncodedScheme value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasEncoded() => $_has(2);
  @$pb.TagNumber(3)
  void clearEncoded() => $_clearField(3);
  @$pb.TagNumber(3)
  EncodedScheme ensureEncoded() => $_ensure(2);
}

/// A strengthening plan or a discard scheme: a name and a selection.
class SchemeEntry extends $pb.GeneratedMessage {
  factory SchemeEntry({
    $core.String? name,
    SoulSelection? selection,
    $core.bool? hasUnknownConditions,
  }) {
    final result = SchemeEntry._();
    if (name != null) result.name = name;
    if (selection != null) result.selection = selection;
    if (hasUnknownConditions != null)
      result.hasUnknownConditions = hasUnknownConditions;
    return result;
  }

  SchemeEntry._();

  factory SchemeEntry.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchemeEntry()..mergeFromBuffer(data, registry);
  factory SchemeEntry.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SchemeEntry()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SchemeEntry',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SchemeEntry.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOM<SoulSelection>(2, _omitFieldNames ? '' : 'selection',
        subBuilder: SoulSelection.$_createMessage)
    ..aOB(3, _omitFieldNames ? '' : 'hasUnknownConditions')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchemeEntry clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SchemeEntry copyWith(void Function(SchemeEntry) updates) =>
      super.copyWith((message) => updates(message as SchemeEntry))
          as SchemeEntry;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SchemeEntry() / SchemeEntry.new instead')
  static SchemeEntry create() => SchemeEntry._();
  static $pb.GeneratedMessage $_createMessage() => SchemeEntry._();
  @$core.override
  SchemeEntry createEmptyInstance() => SchemeEntry._();
  @$core.pragma('dart2js:noInline')
  static SchemeEntry getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SchemeEntry>(
          SchemeEntry.$_createMessage);
  static SchemeEntry? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  @$pb.TagNumber(2)
  SoulSelection get selection => $_getN(1);
  @$pb.TagNumber(2)
  set selection(SoulSelection value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasSelection() => $_has(1);
  @$pb.TagNumber(2)
  void clearSelection() => $_clearField(2);
  @$pb.TagNumber(2)
  SoulSelection ensureSelection() => $_ensure(1);

  /// The record selects on a condition the model cannot see, so no evaluation of it is exact.
  @$pb.TagNumber(3)
  $core.bool get hasUnknownConditions => $_getBF(2);
  @$pb.TagNumber(3)
  set hasUnknownConditions($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasHasUnknownConditions() => $_has(2);
  @$pb.TagNumber(3)
  void clearHasUnknownConditions() => $_clearField(3);
}

/// The scheme text and the QR matrix the application draws; the application never encodes.
class EncodedScheme extends $pb.GeneratedMessage {
  factory EncodedScheme({
    $core.String? text,
    QrMatrix? qr,
  }) {
    final result = EncodedScheme._();
    if (text != null) result.text = text;
    if (qr != null) result.qr = qr;
    return result;
  }

  EncodedScheme._();

  factory EncodedScheme.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EncodedScheme()..mergeFromBuffer(data, registry);
  factory EncodedScheme.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EncodedScheme()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EncodedScheme',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: EncodedScheme.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..aOM<QrMatrix>(2, _omitFieldNames ? '' : 'qr',
        subBuilder: QrMatrix.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EncodedScheme clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  EncodedScheme copyWith(void Function(EncodedScheme) updates) =>
      super.copyWith((message) => updates(message as EncodedScheme))
          as EncodedScheme;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use EncodedScheme() / EncodedScheme.new instead')
  static EncodedScheme create() => EncodedScheme._();
  static $pb.GeneratedMessage $_createMessage() => EncodedScheme._();
  @$core.override
  EncodedScheme createEmptyInstance() => EncodedScheme._();
  @$core.pragma('dart2js:noInline')
  static EncodedScheme getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<EncodedScheme>(
          EncodedScheme.$_createMessage);
  static EncodedScheme? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => $_clearField(1);

  /// Absent when the text does not fit a QR code.
  @$pb.TagNumber(2)
  QrMatrix get qr => $_getN(1);
  @$pb.TagNumber(2)
  set qr(QrMatrix value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasQr() => $_has(1);
  @$pb.TagNumber(2)
  void clearQr() => $_clearField(2);
  @$pb.TagNumber(2)
  QrMatrix ensureQr() => $_ensure(1);
}

/// Modules row by row from the top left, one byte each: 1 is dark. No quiet zone included.
class QrMatrix extends $pb.GeneratedMessage {
  factory QrMatrix({
    $core.int? size,
    $core.List<$core.int>? modules,
  }) {
    final result = QrMatrix._();
    if (size != null) result.size = size;
    if (modules != null) result.modules = modules;
    return result;
  }

  QrMatrix._();

  factory QrMatrix.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QrMatrix()..mergeFromBuffer(data, registry);
  factory QrMatrix.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QrMatrix()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QrMatrix',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QrMatrix.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'size', fieldType: $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'modules', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QrMatrix clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QrMatrix copyWith(void Function(QrMatrix) updates) =>
      super.copyWith((message) => updates(message as QrMatrix)) as QrMatrix;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QrMatrix() / QrMatrix.new instead')
  static QrMatrix create() => QrMatrix._();
  static $pb.GeneratedMessage $_createMessage() => QrMatrix._();
  @$core.override
  QrMatrix createEmptyInstance() => QrMatrix._();
  @$core.pragma('dart2js:noInline')
  static QrMatrix getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QrMatrix>(QrMatrix.$_createMessage);
  static QrMatrix? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get size => $_getIZ(0);
  @$pb.TagNumber(1)
  set size($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSize() => $_has(0);
  @$pb.TagNumber(1)
  void clearSize() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get modules => $_getN(1);
  @$pb.TagNumber(2)
  set modules($core.List<$core.int> value) => $_setBytes(1, value);
  @$pb.TagNumber(2)
  $core.bool hasModules() => $_has(1);
  @$pb.TagNumber(2)
  void clearModules() => $_clearField(2);
}

enum Error_Kind {
  sessionProtocolUnsupported,
  sessionNotOpen,
  sessionAlreadyOpen,
  sessionInvalidRequestId,
  sessionUnknownRequest,
  queryUnknownProfile,
  queryStaleRevision,
  queryMalformed,
  queryUnknownField,
  queryTooComplex,
  queryTypeMismatch,
  queryParamSetRequired,
  queryFieldUnavailable,
  queryUnknownScheme,
  queryMalformedCursor,
  decodeNoInput,
  decodeMalformedText,
  decodeUnknownFormat,
  decodeMalformedLayout,
  decodeMalformedScheme,
  decodeImageInvalid,
  decodeNoQrCode,
  decodeSeveralQrCodes,
  decodeQrUnreadable,
  importUnknownFormat,
  importAmbiguousFormat,
  importUnsupportedVersion,
  importMalformedSource,
  importNormalizationFailed,
  importUnsupportedSourceValue,
  importInconsistentReference,
  importAdmissionRefused,
  importAccountMismatch,
  commandRefused,
  commandTooLarge,
  storeFailure,
  storeInvalidLog,
  storeNewerFormat,
  storeMalformedCommit,
  storeMissing,
  storeNotADatabase,
  storeForeign,
  storeNoFormatVersion,
  storeDamaged,
  storeUninitialized,
  storeRetiredFormat,
  internalPanic,
  internalQrTooLong,
  internalResponseTooLarge,
  internalPageWithoutSoul,
  internalImportMismatch,
  notSet
}

class Error extends $pb.GeneratedMessage {
  factory Error({
    $core.String? message,
    SessionProtocolUnsupported? sessionProtocolUnsupported,
    SessionNotOpen? sessionNotOpen,
    SessionAlreadyOpen? sessionAlreadyOpen,
    SessionInvalidRequestId? sessionInvalidRequestId,
    SessionUnknownRequest? sessionUnknownRequest,
    QueryUnknownProfile? queryUnknownProfile,
    QueryStaleRevision? queryStaleRevision,
    QueryMalformed? queryMalformed,
    QueryUnknownField? queryUnknownField,
    QueryTooComplex? queryTooComplex,
    QueryTypeMismatch? queryTypeMismatch,
    QueryParamSetRequired? queryParamSetRequired,
    QueryFieldUnavailable? queryFieldUnavailable,
    QueryUnknownScheme? queryUnknownScheme,
    QueryMalformedCursor? queryMalformedCursor,
    DecodeNoInput? decodeNoInput,
    DecodeMalformedText? decodeMalformedText,
    DecodeUnknownFormat? decodeUnknownFormat,
    DecodeMalformedLayout? decodeMalformedLayout,
    DecodeMalformedScheme? decodeMalformedScheme,
    DecodeImageInvalid? decodeImageInvalid,
    DecodeNoQrCode? decodeNoQrCode,
    DecodeSeveralQrCodes? decodeSeveralQrCodes,
    DecodeQrUnreadable? decodeQrUnreadable,
    ImportUnknownFormat? importUnknownFormat,
    ImportAmbiguousFormat? importAmbiguousFormat,
    ImportUnsupportedVersion? importUnsupportedVersion,
    ImportMalformedSource? importMalformedSource,
    ImportNormalizationFailed? importNormalizationFailed,
    ImportUnsupportedSourceValue? importUnsupportedSourceValue,
    ImportInconsistentReference? importInconsistentReference,
    ImportAdmissionRefused? importAdmissionRefused,
    ImportAccountMismatch? importAccountMismatch,
    CommandRefused? commandRefused,
    CommandTooLarge? commandTooLarge,
    StoreFailure? storeFailure,
    StoreInvalidLog? storeInvalidLog,
    StoreNewerFormat? storeNewerFormat,
    StoreMalformedCommit? storeMalformedCommit,
    StoreMissing? storeMissing,
    StoreNotADatabase? storeNotADatabase,
    StoreForeign? storeForeign,
    StoreNoFormatVersion? storeNoFormatVersion,
    StoreDamaged? storeDamaged,
    StoreUninitialized? storeUninitialized,
    StoreRetiredFormat? storeRetiredFormat,
    InternalPanic? internalPanic,
    InternalQrTooLong? internalQrTooLong,
    InternalResponseTooLarge? internalResponseTooLarge,
    InternalPageWithoutSoul? internalPageWithoutSoul,
    InternalImportMismatch? internalImportMismatch,
  }) {
    final result = Error._();
    if (message != null) result.message = message;
    if (sessionProtocolUnsupported != null)
      result.sessionProtocolUnsupported = sessionProtocolUnsupported;
    if (sessionNotOpen != null) result.sessionNotOpen = sessionNotOpen;
    if (sessionAlreadyOpen != null)
      result.sessionAlreadyOpen = sessionAlreadyOpen;
    if (sessionInvalidRequestId != null)
      result.sessionInvalidRequestId = sessionInvalidRequestId;
    if (sessionUnknownRequest != null)
      result.sessionUnknownRequest = sessionUnknownRequest;
    if (queryUnknownProfile != null)
      result.queryUnknownProfile = queryUnknownProfile;
    if (queryStaleRevision != null)
      result.queryStaleRevision = queryStaleRevision;
    if (queryMalformed != null) result.queryMalformed = queryMalformed;
    if (queryUnknownField != null) result.queryUnknownField = queryUnknownField;
    if (queryTooComplex != null) result.queryTooComplex = queryTooComplex;
    if (queryTypeMismatch != null) result.queryTypeMismatch = queryTypeMismatch;
    if (queryParamSetRequired != null)
      result.queryParamSetRequired = queryParamSetRequired;
    if (queryFieldUnavailable != null)
      result.queryFieldUnavailable = queryFieldUnavailable;
    if (queryUnknownScheme != null)
      result.queryUnknownScheme = queryUnknownScheme;
    if (queryMalformedCursor != null)
      result.queryMalformedCursor = queryMalformedCursor;
    if (decodeNoInput != null) result.decodeNoInput = decodeNoInput;
    if (decodeMalformedText != null)
      result.decodeMalformedText = decodeMalformedText;
    if (decodeUnknownFormat != null)
      result.decodeUnknownFormat = decodeUnknownFormat;
    if (decodeMalformedLayout != null)
      result.decodeMalformedLayout = decodeMalformedLayout;
    if (decodeMalformedScheme != null)
      result.decodeMalformedScheme = decodeMalformedScheme;
    if (decodeImageInvalid != null)
      result.decodeImageInvalid = decodeImageInvalid;
    if (decodeNoQrCode != null) result.decodeNoQrCode = decodeNoQrCode;
    if (decodeSeveralQrCodes != null)
      result.decodeSeveralQrCodes = decodeSeveralQrCodes;
    if (decodeQrUnreadable != null)
      result.decodeQrUnreadable = decodeQrUnreadable;
    if (importUnknownFormat != null)
      result.importUnknownFormat = importUnknownFormat;
    if (importAmbiguousFormat != null)
      result.importAmbiguousFormat = importAmbiguousFormat;
    if (importUnsupportedVersion != null)
      result.importUnsupportedVersion = importUnsupportedVersion;
    if (importMalformedSource != null)
      result.importMalformedSource = importMalformedSource;
    if (importNormalizationFailed != null)
      result.importNormalizationFailed = importNormalizationFailed;
    if (importUnsupportedSourceValue != null)
      result.importUnsupportedSourceValue = importUnsupportedSourceValue;
    if (importInconsistentReference != null)
      result.importInconsistentReference = importInconsistentReference;
    if (importAdmissionRefused != null)
      result.importAdmissionRefused = importAdmissionRefused;
    if (importAccountMismatch != null)
      result.importAccountMismatch = importAccountMismatch;
    if (commandRefused != null) result.commandRefused = commandRefused;
    if (commandTooLarge != null) result.commandTooLarge = commandTooLarge;
    if (storeFailure != null) result.storeFailure = storeFailure;
    if (storeInvalidLog != null) result.storeInvalidLog = storeInvalidLog;
    if (storeNewerFormat != null) result.storeNewerFormat = storeNewerFormat;
    if (storeMalformedCommit != null)
      result.storeMalformedCommit = storeMalformedCommit;
    if (storeMissing != null) result.storeMissing = storeMissing;
    if (storeNotADatabase != null) result.storeNotADatabase = storeNotADatabase;
    if (storeForeign != null) result.storeForeign = storeForeign;
    if (storeNoFormatVersion != null)
      result.storeNoFormatVersion = storeNoFormatVersion;
    if (storeDamaged != null) result.storeDamaged = storeDamaged;
    if (storeUninitialized != null)
      result.storeUninitialized = storeUninitialized;
    if (storeRetiredFormat != null)
      result.storeRetiredFormat = storeRetiredFormat;
    if (internalPanic != null) result.internalPanic = internalPanic;
    if (internalQrTooLong != null) result.internalQrTooLong = internalQrTooLong;
    if (internalResponseTooLarge != null)
      result.internalResponseTooLarge = internalResponseTooLarge;
    if (internalPageWithoutSoul != null)
      result.internalPageWithoutSoul = internalPageWithoutSoul;
    if (internalImportMismatch != null)
      result.internalImportMismatch = internalImportMismatch;
    return result;
  }

  Error._();

  factory Error.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Error()..mergeFromBuffer(data, registry);
  factory Error.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Error()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Error_Kind> _Error_KindByTag = {
    10: Error_Kind.sessionProtocolUnsupported,
    11: Error_Kind.sessionNotOpen,
    12: Error_Kind.sessionAlreadyOpen,
    13: Error_Kind.sessionInvalidRequestId,
    14: Error_Kind.sessionUnknownRequest,
    20: Error_Kind.queryUnknownProfile,
    21: Error_Kind.queryStaleRevision,
    22: Error_Kind.queryMalformed,
    23: Error_Kind.queryUnknownField,
    24: Error_Kind.queryTooComplex,
    25: Error_Kind.queryTypeMismatch,
    26: Error_Kind.queryParamSetRequired,
    27: Error_Kind.queryFieldUnavailable,
    28: Error_Kind.queryUnknownScheme,
    29: Error_Kind.queryMalformedCursor,
    40: Error_Kind.decodeNoInput,
    41: Error_Kind.decodeMalformedText,
    42: Error_Kind.decodeUnknownFormat,
    43: Error_Kind.decodeMalformedLayout,
    44: Error_Kind.decodeMalformedScheme,
    45: Error_Kind.decodeImageInvalid,
    46: Error_Kind.decodeNoQrCode,
    47: Error_Kind.decodeSeveralQrCodes,
    48: Error_Kind.decodeQrUnreadable,
    50: Error_Kind.importUnknownFormat,
    51: Error_Kind.importAmbiguousFormat,
    52: Error_Kind.importUnsupportedVersion,
    53: Error_Kind.importMalformedSource,
    54: Error_Kind.importNormalizationFailed,
    55: Error_Kind.importUnsupportedSourceValue,
    56: Error_Kind.importInconsistentReference,
    57: Error_Kind.importAdmissionRefused,
    58: Error_Kind.importAccountMismatch,
    70: Error_Kind.commandRefused,
    71: Error_Kind.commandTooLarge,
    80: Error_Kind.storeFailure,
    81: Error_Kind.storeInvalidLog,
    82: Error_Kind.storeNewerFormat,
    83: Error_Kind.storeMalformedCommit,
    84: Error_Kind.storeMissing,
    85: Error_Kind.storeNotADatabase,
    86: Error_Kind.storeForeign,
    87: Error_Kind.storeNoFormatVersion,
    88: Error_Kind.storeDamaged,
    89: Error_Kind.storeUninitialized,
    90: Error_Kind.storeRetiredFormat,
    100: Error_Kind.internalPanic,
    101: Error_Kind.internalQrTooLong,
    102: Error_Kind.internalResponseTooLarge,
    103: Error_Kind.internalPageWithoutSoul,
    104: Error_Kind.internalImportMismatch,
    0: Error_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Error',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Error.$_createMessage)
    ..oo(0, [
      10,
      11,
      12,
      13,
      14,
      20,
      21,
      22,
      23,
      24,
      25,
      26,
      27,
      28,
      29,
      40,
      41,
      42,
      43,
      44,
      45,
      46,
      47,
      48,
      50,
      51,
      52,
      53,
      54,
      55,
      56,
      57,
      58,
      70,
      71,
      80,
      81,
      82,
      83,
      84,
      85,
      86,
      87,
      88,
      89,
      90,
      100,
      101,
      102,
      103,
      104
    ])
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..aOM<SessionProtocolUnsupported>(
        10, _omitFieldNames ? '' : 'sessionProtocolUnsupported',
        subBuilder: SessionProtocolUnsupported.$_createMessage)
    ..aOM<SessionNotOpen>(11, _omitFieldNames ? '' : 'sessionNotOpen',
        subBuilder: SessionNotOpen.$_createMessage)
    ..aOM<SessionAlreadyOpen>(12, _omitFieldNames ? '' : 'sessionAlreadyOpen',
        subBuilder: SessionAlreadyOpen.$_createMessage)
    ..aOM<SessionInvalidRequestId>(
        13, _omitFieldNames ? '' : 'sessionInvalidRequestId',
        subBuilder: SessionInvalidRequestId.$_createMessage)
    ..aOM<SessionUnknownRequest>(
        14, _omitFieldNames ? '' : 'sessionUnknownRequest',
        subBuilder: SessionUnknownRequest.$_createMessage)
    ..aOM<QueryUnknownProfile>(20, _omitFieldNames ? '' : 'queryUnknownProfile',
        subBuilder: QueryUnknownProfile.$_createMessage)
    ..aOM<QueryStaleRevision>(21, _omitFieldNames ? '' : 'queryStaleRevision',
        subBuilder: QueryStaleRevision.$_createMessage)
    ..aOM<QueryMalformed>(22, _omitFieldNames ? '' : 'queryMalformed',
        subBuilder: QueryMalformed.$_createMessage)
    ..aOM<QueryUnknownField>(23, _omitFieldNames ? '' : 'queryUnknownField',
        subBuilder: QueryUnknownField.$_createMessage)
    ..aOM<QueryTooComplex>(24, _omitFieldNames ? '' : 'queryTooComplex',
        subBuilder: QueryTooComplex.$_createMessage)
    ..aOM<QueryTypeMismatch>(25, _omitFieldNames ? '' : 'queryTypeMismatch',
        subBuilder: QueryTypeMismatch.$_createMessage)
    ..aOM<QueryParamSetRequired>(
        26, _omitFieldNames ? '' : 'queryParamSetRequired',
        subBuilder: QueryParamSetRequired.$_createMessage)
    ..aOM<QueryFieldUnavailable>(
        27, _omitFieldNames ? '' : 'queryFieldUnavailable',
        subBuilder: QueryFieldUnavailable.$_createMessage)
    ..aOM<QueryUnknownScheme>(28, _omitFieldNames ? '' : 'queryUnknownScheme',
        subBuilder: QueryUnknownScheme.$_createMessage)
    ..aOM<QueryMalformedCursor>(
        29, _omitFieldNames ? '' : 'queryMalformedCursor',
        subBuilder: QueryMalformedCursor.$_createMessage)
    ..aOM<DecodeNoInput>(40, _omitFieldNames ? '' : 'decodeNoInput',
        subBuilder: DecodeNoInput.$_createMessage)
    ..aOM<DecodeMalformedText>(41, _omitFieldNames ? '' : 'decodeMalformedText',
        subBuilder: DecodeMalformedText.$_createMessage)
    ..aOM<DecodeUnknownFormat>(42, _omitFieldNames ? '' : 'decodeUnknownFormat',
        subBuilder: DecodeUnknownFormat.$_createMessage)
    ..aOM<DecodeMalformedLayout>(
        43, _omitFieldNames ? '' : 'decodeMalformedLayout',
        subBuilder: DecodeMalformedLayout.$_createMessage)
    ..aOM<DecodeMalformedScheme>(
        44, _omitFieldNames ? '' : 'decodeMalformedScheme',
        subBuilder: DecodeMalformedScheme.$_createMessage)
    ..aOM<DecodeImageInvalid>(45, _omitFieldNames ? '' : 'decodeImageInvalid',
        subBuilder: DecodeImageInvalid.$_createMessage)
    ..aOM<DecodeNoQrCode>(46, _omitFieldNames ? '' : 'decodeNoQrCode',
        subBuilder: DecodeNoQrCode.$_createMessage)
    ..aOM<DecodeSeveralQrCodes>(
        47, _omitFieldNames ? '' : 'decodeSeveralQrCodes',
        subBuilder: DecodeSeveralQrCodes.$_createMessage)
    ..aOM<DecodeQrUnreadable>(48, _omitFieldNames ? '' : 'decodeQrUnreadable',
        subBuilder: DecodeQrUnreadable.$_createMessage)
    ..aOM<ImportUnknownFormat>(50, _omitFieldNames ? '' : 'importUnknownFormat',
        subBuilder: ImportUnknownFormat.$_createMessage)
    ..aOM<ImportAmbiguousFormat>(
        51, _omitFieldNames ? '' : 'importAmbiguousFormat',
        subBuilder: ImportAmbiguousFormat.$_createMessage)
    ..aOM<ImportUnsupportedVersion>(
        52, _omitFieldNames ? '' : 'importUnsupportedVersion',
        subBuilder: ImportUnsupportedVersion.$_createMessage)
    ..aOM<ImportMalformedSource>(
        53, _omitFieldNames ? '' : 'importMalformedSource',
        subBuilder: ImportMalformedSource.$_createMessage)
    ..aOM<ImportNormalizationFailed>(
        54, _omitFieldNames ? '' : 'importNormalizationFailed',
        subBuilder: ImportNormalizationFailed.$_createMessage)
    ..aOM<ImportUnsupportedSourceValue>(
        55, _omitFieldNames ? '' : 'importUnsupportedSourceValue',
        subBuilder: ImportUnsupportedSourceValue.$_createMessage)
    ..aOM<ImportInconsistentReference>(
        56, _omitFieldNames ? '' : 'importInconsistentReference',
        subBuilder: ImportInconsistentReference.$_createMessage)
    ..aOM<ImportAdmissionRefused>(
        57, _omitFieldNames ? '' : 'importAdmissionRefused',
        subBuilder: ImportAdmissionRefused.$_createMessage)
    ..aOM<ImportAccountMismatch>(
        58, _omitFieldNames ? '' : 'importAccountMismatch',
        subBuilder: ImportAccountMismatch.$_createMessage)
    ..aOM<CommandRefused>(70, _omitFieldNames ? '' : 'commandRefused',
        subBuilder: CommandRefused.$_createMessage)
    ..aOM<CommandTooLarge>(71, _omitFieldNames ? '' : 'commandTooLarge',
        subBuilder: CommandTooLarge.$_createMessage)
    ..aOM<StoreFailure>(80, _omitFieldNames ? '' : 'storeFailure',
        subBuilder: StoreFailure.$_createMessage)
    ..aOM<StoreInvalidLog>(81, _omitFieldNames ? '' : 'storeInvalidLog',
        subBuilder: StoreInvalidLog.$_createMessage)
    ..aOM<StoreNewerFormat>(82, _omitFieldNames ? '' : 'storeNewerFormat',
        subBuilder: StoreNewerFormat.$_createMessage)
    ..aOM<StoreMalformedCommit>(
        83, _omitFieldNames ? '' : 'storeMalformedCommit',
        subBuilder: StoreMalformedCommit.$_createMessage)
    ..aOM<StoreMissing>(84, _omitFieldNames ? '' : 'storeMissing',
        subBuilder: StoreMissing.$_createMessage)
    ..aOM<StoreNotADatabase>(85, _omitFieldNames ? '' : 'storeNotADatabase',
        subBuilder: StoreNotADatabase.$_createMessage)
    ..aOM<StoreForeign>(86, _omitFieldNames ? '' : 'storeForeign',
        subBuilder: StoreForeign.$_createMessage)
    ..aOM<StoreNoFormatVersion>(
        87, _omitFieldNames ? '' : 'storeNoFormatVersion',
        subBuilder: StoreNoFormatVersion.$_createMessage)
    ..aOM<StoreDamaged>(88, _omitFieldNames ? '' : 'storeDamaged',
        subBuilder: StoreDamaged.$_createMessage)
    ..aOM<StoreUninitialized>(89, _omitFieldNames ? '' : 'storeUninitialized',
        subBuilder: StoreUninitialized.$_createMessage)
    ..aOM<StoreRetiredFormat>(90, _omitFieldNames ? '' : 'storeRetiredFormat',
        subBuilder: StoreRetiredFormat.$_createMessage)
    ..aOM<InternalPanic>(100, _omitFieldNames ? '' : 'internalPanic',
        subBuilder: InternalPanic.$_createMessage)
    ..aOM<InternalQrTooLong>(101, _omitFieldNames ? '' : 'internalQrTooLong',
        subBuilder: InternalQrTooLong.$_createMessage)
    ..aOM<InternalResponseTooLarge>(
        102, _omitFieldNames ? '' : 'internalResponseTooLarge',
        subBuilder: InternalResponseTooLarge.$_createMessage)
    ..aOM<InternalPageWithoutSoul>(
        103, _omitFieldNames ? '' : 'internalPageWithoutSoul',
        subBuilder: InternalPageWithoutSoul.$_createMessage)
    ..aOM<InternalImportMismatch>(
        104, _omitFieldNames ? '' : 'internalImportMismatch',
        subBuilder: InternalImportMismatch.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Error clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Error copyWith(void Function(Error) updates) =>
      super.copyWith((message) => updates(message as Error)) as Error;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Error() / Error.new instead')
  static Error create() => Error._();
  static $pb.GeneratedMessage $_createMessage() => Error._();
  @$core.override
  Error createEmptyInstance() => Error._();
  @$core.pragma('dart2js:noInline')
  static Error getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Error>(Error.$_createMessage);
  static Error? _defaultInstance;

  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  @$pb.TagNumber(24)
  @$pb.TagNumber(25)
  @$pb.TagNumber(26)
  @$pb.TagNumber(27)
  @$pb.TagNumber(28)
  @$pb.TagNumber(29)
  @$pb.TagNumber(40)
  @$pb.TagNumber(41)
  @$pb.TagNumber(42)
  @$pb.TagNumber(43)
  @$pb.TagNumber(44)
  @$pb.TagNumber(45)
  @$pb.TagNumber(46)
  @$pb.TagNumber(47)
  @$pb.TagNumber(48)
  @$pb.TagNumber(50)
  @$pb.TagNumber(51)
  @$pb.TagNumber(52)
  @$pb.TagNumber(53)
  @$pb.TagNumber(54)
  @$pb.TagNumber(55)
  @$pb.TagNumber(56)
  @$pb.TagNumber(57)
  @$pb.TagNumber(58)
  @$pb.TagNumber(70)
  @$pb.TagNumber(71)
  @$pb.TagNumber(80)
  @$pb.TagNumber(81)
  @$pb.TagNumber(82)
  @$pb.TagNumber(83)
  @$pb.TagNumber(84)
  @$pb.TagNumber(85)
  @$pb.TagNumber(86)
  @$pb.TagNumber(87)
  @$pb.TagNumber(88)
  @$pb.TagNumber(89)
  @$pb.TagNumber(90)
  @$pb.TagNumber(100)
  @$pb.TagNumber(101)
  @$pb.TagNumber(102)
  @$pb.TagNumber(103)
  @$pb.TagNumber(104)
  Error_Kind whichKind() => _Error_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  @$pb.TagNumber(23)
  @$pb.TagNumber(24)
  @$pb.TagNumber(25)
  @$pb.TagNumber(26)
  @$pb.TagNumber(27)
  @$pb.TagNumber(28)
  @$pb.TagNumber(29)
  @$pb.TagNumber(40)
  @$pb.TagNumber(41)
  @$pb.TagNumber(42)
  @$pb.TagNumber(43)
  @$pb.TagNumber(44)
  @$pb.TagNumber(45)
  @$pb.TagNumber(46)
  @$pb.TagNumber(47)
  @$pb.TagNumber(48)
  @$pb.TagNumber(50)
  @$pb.TagNumber(51)
  @$pb.TagNumber(52)
  @$pb.TagNumber(53)
  @$pb.TagNumber(54)
  @$pb.TagNumber(55)
  @$pb.TagNumber(56)
  @$pb.TagNumber(57)
  @$pb.TagNumber(58)
  @$pb.TagNumber(70)
  @$pb.TagNumber(71)
  @$pb.TagNumber(80)
  @$pb.TagNumber(81)
  @$pb.TagNumber(82)
  @$pb.TagNumber(83)
  @$pb.TagNumber(84)
  @$pb.TagNumber(85)
  @$pb.TagNumber(86)
  @$pb.TagNumber(87)
  @$pb.TagNumber(88)
  @$pb.TagNumber(89)
  @$pb.TagNumber(90)
  @$pb.TagNumber(100)
  @$pb.TagNumber(101)
  @$pb.TagNumber(102)
  @$pb.TagNumber(103)
  @$pb.TagNumber(104)
  void clearKind() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(0);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(0, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);

  @$pb.TagNumber(10)
  SessionProtocolUnsupported get sessionProtocolUnsupported => $_getN(1);
  @$pb.TagNumber(10)
  set sessionProtocolUnsupported(SessionProtocolUnsupported value) =>
      $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasSessionProtocolUnsupported() => $_has(1);
  @$pb.TagNumber(10)
  void clearSessionProtocolUnsupported() => $_clearField(10);
  @$pb.TagNumber(10)
  SessionProtocolUnsupported ensureSessionProtocolUnsupported() => $_ensure(1);

  @$pb.TagNumber(11)
  SessionNotOpen get sessionNotOpen => $_getN(2);
  @$pb.TagNumber(11)
  set sessionNotOpen(SessionNotOpen value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasSessionNotOpen() => $_has(2);
  @$pb.TagNumber(11)
  void clearSessionNotOpen() => $_clearField(11);
  @$pb.TagNumber(11)
  SessionNotOpen ensureSessionNotOpen() => $_ensure(2);

  @$pb.TagNumber(12)
  SessionAlreadyOpen get sessionAlreadyOpen => $_getN(3);
  @$pb.TagNumber(12)
  set sessionAlreadyOpen(SessionAlreadyOpen value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasSessionAlreadyOpen() => $_has(3);
  @$pb.TagNumber(12)
  void clearSessionAlreadyOpen() => $_clearField(12);
  @$pb.TagNumber(12)
  SessionAlreadyOpen ensureSessionAlreadyOpen() => $_ensure(3);

  @$pb.TagNumber(13)
  SessionInvalidRequestId get sessionInvalidRequestId => $_getN(4);
  @$pb.TagNumber(13)
  set sessionInvalidRequestId(SessionInvalidRequestId value) =>
      $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasSessionInvalidRequestId() => $_has(4);
  @$pb.TagNumber(13)
  void clearSessionInvalidRequestId() => $_clearField(13);
  @$pb.TagNumber(13)
  SessionInvalidRequestId ensureSessionInvalidRequestId() => $_ensure(4);

  @$pb.TagNumber(14)
  SessionUnknownRequest get sessionUnknownRequest => $_getN(5);
  @$pb.TagNumber(14)
  set sessionUnknownRequest(SessionUnknownRequest value) =>
      $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasSessionUnknownRequest() => $_has(5);
  @$pb.TagNumber(14)
  void clearSessionUnknownRequest() => $_clearField(14);
  @$pb.TagNumber(14)
  SessionUnknownRequest ensureSessionUnknownRequest() => $_ensure(5);

  @$pb.TagNumber(20)
  QueryUnknownProfile get queryUnknownProfile => $_getN(6);
  @$pb.TagNumber(20)
  set queryUnknownProfile(QueryUnknownProfile value) => $_setField(20, value);
  @$pb.TagNumber(20)
  $core.bool hasQueryUnknownProfile() => $_has(6);
  @$pb.TagNumber(20)
  void clearQueryUnknownProfile() => $_clearField(20);
  @$pb.TagNumber(20)
  QueryUnknownProfile ensureQueryUnknownProfile() => $_ensure(6);

  @$pb.TagNumber(21)
  QueryStaleRevision get queryStaleRevision => $_getN(7);
  @$pb.TagNumber(21)
  set queryStaleRevision(QueryStaleRevision value) => $_setField(21, value);
  @$pb.TagNumber(21)
  $core.bool hasQueryStaleRevision() => $_has(7);
  @$pb.TagNumber(21)
  void clearQueryStaleRevision() => $_clearField(21);
  @$pb.TagNumber(21)
  QueryStaleRevision ensureQueryStaleRevision() => $_ensure(7);

  @$pb.TagNumber(22)
  QueryMalformed get queryMalformed => $_getN(8);
  @$pb.TagNumber(22)
  set queryMalformed(QueryMalformed value) => $_setField(22, value);
  @$pb.TagNumber(22)
  $core.bool hasQueryMalformed() => $_has(8);
  @$pb.TagNumber(22)
  void clearQueryMalformed() => $_clearField(22);
  @$pb.TagNumber(22)
  QueryMalformed ensureQueryMalformed() => $_ensure(8);

  @$pb.TagNumber(23)
  QueryUnknownField get queryUnknownField => $_getN(9);
  @$pb.TagNumber(23)
  set queryUnknownField(QueryUnknownField value) => $_setField(23, value);
  @$pb.TagNumber(23)
  $core.bool hasQueryUnknownField() => $_has(9);
  @$pb.TagNumber(23)
  void clearQueryUnknownField() => $_clearField(23);
  @$pb.TagNumber(23)
  QueryUnknownField ensureQueryUnknownField() => $_ensure(9);

  @$pb.TagNumber(24)
  QueryTooComplex get queryTooComplex => $_getN(10);
  @$pb.TagNumber(24)
  set queryTooComplex(QueryTooComplex value) => $_setField(24, value);
  @$pb.TagNumber(24)
  $core.bool hasQueryTooComplex() => $_has(10);
  @$pb.TagNumber(24)
  void clearQueryTooComplex() => $_clearField(24);
  @$pb.TagNumber(24)
  QueryTooComplex ensureQueryTooComplex() => $_ensure(10);

  @$pb.TagNumber(25)
  QueryTypeMismatch get queryTypeMismatch => $_getN(11);
  @$pb.TagNumber(25)
  set queryTypeMismatch(QueryTypeMismatch value) => $_setField(25, value);
  @$pb.TagNumber(25)
  $core.bool hasQueryTypeMismatch() => $_has(11);
  @$pb.TagNumber(25)
  void clearQueryTypeMismatch() => $_clearField(25);
  @$pb.TagNumber(25)
  QueryTypeMismatch ensureQueryTypeMismatch() => $_ensure(11);

  @$pb.TagNumber(26)
  QueryParamSetRequired get queryParamSetRequired => $_getN(12);
  @$pb.TagNumber(26)
  set queryParamSetRequired(QueryParamSetRequired value) =>
      $_setField(26, value);
  @$pb.TagNumber(26)
  $core.bool hasQueryParamSetRequired() => $_has(12);
  @$pb.TagNumber(26)
  void clearQueryParamSetRequired() => $_clearField(26);
  @$pb.TagNumber(26)
  QueryParamSetRequired ensureQueryParamSetRequired() => $_ensure(12);

  @$pb.TagNumber(27)
  QueryFieldUnavailable get queryFieldUnavailable => $_getN(13);
  @$pb.TagNumber(27)
  set queryFieldUnavailable(QueryFieldUnavailable value) =>
      $_setField(27, value);
  @$pb.TagNumber(27)
  $core.bool hasQueryFieldUnavailable() => $_has(13);
  @$pb.TagNumber(27)
  void clearQueryFieldUnavailable() => $_clearField(27);
  @$pb.TagNumber(27)
  QueryFieldUnavailable ensureQueryFieldUnavailable() => $_ensure(13);

  @$pb.TagNumber(28)
  QueryUnknownScheme get queryUnknownScheme => $_getN(14);
  @$pb.TagNumber(28)
  set queryUnknownScheme(QueryUnknownScheme value) => $_setField(28, value);
  @$pb.TagNumber(28)
  $core.bool hasQueryUnknownScheme() => $_has(14);
  @$pb.TagNumber(28)
  void clearQueryUnknownScheme() => $_clearField(28);
  @$pb.TagNumber(28)
  QueryUnknownScheme ensureQueryUnknownScheme() => $_ensure(14);

  @$pb.TagNumber(29)
  QueryMalformedCursor get queryMalformedCursor => $_getN(15);
  @$pb.TagNumber(29)
  set queryMalformedCursor(QueryMalformedCursor value) => $_setField(29, value);
  @$pb.TagNumber(29)
  $core.bool hasQueryMalformedCursor() => $_has(15);
  @$pb.TagNumber(29)
  void clearQueryMalformedCursor() => $_clearField(29);
  @$pb.TagNumber(29)
  QueryMalformedCursor ensureQueryMalformedCursor() => $_ensure(15);

  @$pb.TagNumber(40)
  DecodeNoInput get decodeNoInput => $_getN(16);
  @$pb.TagNumber(40)
  set decodeNoInput(DecodeNoInput value) => $_setField(40, value);
  @$pb.TagNumber(40)
  $core.bool hasDecodeNoInput() => $_has(16);
  @$pb.TagNumber(40)
  void clearDecodeNoInput() => $_clearField(40);
  @$pb.TagNumber(40)
  DecodeNoInput ensureDecodeNoInput() => $_ensure(16);

  @$pb.TagNumber(41)
  DecodeMalformedText get decodeMalformedText => $_getN(17);
  @$pb.TagNumber(41)
  set decodeMalformedText(DecodeMalformedText value) => $_setField(41, value);
  @$pb.TagNumber(41)
  $core.bool hasDecodeMalformedText() => $_has(17);
  @$pb.TagNumber(41)
  void clearDecodeMalformedText() => $_clearField(41);
  @$pb.TagNumber(41)
  DecodeMalformedText ensureDecodeMalformedText() => $_ensure(17);

  @$pb.TagNumber(42)
  DecodeUnknownFormat get decodeUnknownFormat => $_getN(18);
  @$pb.TagNumber(42)
  set decodeUnknownFormat(DecodeUnknownFormat value) => $_setField(42, value);
  @$pb.TagNumber(42)
  $core.bool hasDecodeUnknownFormat() => $_has(18);
  @$pb.TagNumber(42)
  void clearDecodeUnknownFormat() => $_clearField(42);
  @$pb.TagNumber(42)
  DecodeUnknownFormat ensureDecodeUnknownFormat() => $_ensure(18);

  @$pb.TagNumber(43)
  DecodeMalformedLayout get decodeMalformedLayout => $_getN(19);
  @$pb.TagNumber(43)
  set decodeMalformedLayout(DecodeMalformedLayout value) =>
      $_setField(43, value);
  @$pb.TagNumber(43)
  $core.bool hasDecodeMalformedLayout() => $_has(19);
  @$pb.TagNumber(43)
  void clearDecodeMalformedLayout() => $_clearField(43);
  @$pb.TagNumber(43)
  DecodeMalformedLayout ensureDecodeMalformedLayout() => $_ensure(19);

  @$pb.TagNumber(44)
  DecodeMalformedScheme get decodeMalformedScheme => $_getN(20);
  @$pb.TagNumber(44)
  set decodeMalformedScheme(DecodeMalformedScheme value) =>
      $_setField(44, value);
  @$pb.TagNumber(44)
  $core.bool hasDecodeMalformedScheme() => $_has(20);
  @$pb.TagNumber(44)
  void clearDecodeMalformedScheme() => $_clearField(44);
  @$pb.TagNumber(44)
  DecodeMalformedScheme ensureDecodeMalformedScheme() => $_ensure(20);

  @$pb.TagNumber(45)
  DecodeImageInvalid get decodeImageInvalid => $_getN(21);
  @$pb.TagNumber(45)
  set decodeImageInvalid(DecodeImageInvalid value) => $_setField(45, value);
  @$pb.TagNumber(45)
  $core.bool hasDecodeImageInvalid() => $_has(21);
  @$pb.TagNumber(45)
  void clearDecodeImageInvalid() => $_clearField(45);
  @$pb.TagNumber(45)
  DecodeImageInvalid ensureDecodeImageInvalid() => $_ensure(21);

  @$pb.TagNumber(46)
  DecodeNoQrCode get decodeNoQrCode => $_getN(22);
  @$pb.TagNumber(46)
  set decodeNoQrCode(DecodeNoQrCode value) => $_setField(46, value);
  @$pb.TagNumber(46)
  $core.bool hasDecodeNoQrCode() => $_has(22);
  @$pb.TagNumber(46)
  void clearDecodeNoQrCode() => $_clearField(46);
  @$pb.TagNumber(46)
  DecodeNoQrCode ensureDecodeNoQrCode() => $_ensure(22);

  @$pb.TagNumber(47)
  DecodeSeveralQrCodes get decodeSeveralQrCodes => $_getN(23);
  @$pb.TagNumber(47)
  set decodeSeveralQrCodes(DecodeSeveralQrCodes value) => $_setField(47, value);
  @$pb.TagNumber(47)
  $core.bool hasDecodeSeveralQrCodes() => $_has(23);
  @$pb.TagNumber(47)
  void clearDecodeSeveralQrCodes() => $_clearField(47);
  @$pb.TagNumber(47)
  DecodeSeveralQrCodes ensureDecodeSeveralQrCodes() => $_ensure(23);

  @$pb.TagNumber(48)
  DecodeQrUnreadable get decodeQrUnreadable => $_getN(24);
  @$pb.TagNumber(48)
  set decodeQrUnreadable(DecodeQrUnreadable value) => $_setField(48, value);
  @$pb.TagNumber(48)
  $core.bool hasDecodeQrUnreadable() => $_has(24);
  @$pb.TagNumber(48)
  void clearDecodeQrUnreadable() => $_clearField(48);
  @$pb.TagNumber(48)
  DecodeQrUnreadable ensureDecodeQrUnreadable() => $_ensure(24);

  /// An imported file (snapshot-ir.md; ADR-0031, rule 6): refused whole, typed by the stage
  /// that refused it.
  @$pb.TagNumber(50)
  ImportUnknownFormat get importUnknownFormat => $_getN(25);
  @$pb.TagNumber(50)
  set importUnknownFormat(ImportUnknownFormat value) => $_setField(50, value);
  @$pb.TagNumber(50)
  $core.bool hasImportUnknownFormat() => $_has(25);
  @$pb.TagNumber(50)
  void clearImportUnknownFormat() => $_clearField(50);
  @$pb.TagNumber(50)
  ImportUnknownFormat ensureImportUnknownFormat() => $_ensure(25);

  @$pb.TagNumber(51)
  ImportAmbiguousFormat get importAmbiguousFormat => $_getN(26);
  @$pb.TagNumber(51)
  set importAmbiguousFormat(ImportAmbiguousFormat value) =>
      $_setField(51, value);
  @$pb.TagNumber(51)
  $core.bool hasImportAmbiguousFormat() => $_has(26);
  @$pb.TagNumber(51)
  void clearImportAmbiguousFormat() => $_clearField(51);
  @$pb.TagNumber(51)
  ImportAmbiguousFormat ensureImportAmbiguousFormat() => $_ensure(26);

  @$pb.TagNumber(52)
  ImportUnsupportedVersion get importUnsupportedVersion => $_getN(27);
  @$pb.TagNumber(52)
  set importUnsupportedVersion(ImportUnsupportedVersion value) =>
      $_setField(52, value);
  @$pb.TagNumber(52)
  $core.bool hasImportUnsupportedVersion() => $_has(27);
  @$pb.TagNumber(52)
  void clearImportUnsupportedVersion() => $_clearField(52);
  @$pb.TagNumber(52)
  ImportUnsupportedVersion ensureImportUnsupportedVersion() => $_ensure(27);

  @$pb.TagNumber(53)
  ImportMalformedSource get importMalformedSource => $_getN(28);
  @$pb.TagNumber(53)
  set importMalformedSource(ImportMalformedSource value) =>
      $_setField(53, value);
  @$pb.TagNumber(53)
  $core.bool hasImportMalformedSource() => $_has(28);
  @$pb.TagNumber(53)
  void clearImportMalformedSource() => $_clearField(53);
  @$pb.TagNumber(53)
  ImportMalformedSource ensureImportMalformedSource() => $_ensure(28);

  @$pb.TagNumber(54)
  ImportNormalizationFailed get importNormalizationFailed => $_getN(29);
  @$pb.TagNumber(54)
  set importNormalizationFailed(ImportNormalizationFailed value) =>
      $_setField(54, value);
  @$pb.TagNumber(54)
  $core.bool hasImportNormalizationFailed() => $_has(29);
  @$pb.TagNumber(54)
  void clearImportNormalizationFailed() => $_clearField(54);
  @$pb.TagNumber(54)
  ImportNormalizationFailed ensureImportNormalizationFailed() => $_ensure(29);

  @$pb.TagNumber(55)
  ImportUnsupportedSourceValue get importUnsupportedSourceValue => $_getN(30);
  @$pb.TagNumber(55)
  set importUnsupportedSourceValue(ImportUnsupportedSourceValue value) =>
      $_setField(55, value);
  @$pb.TagNumber(55)
  $core.bool hasImportUnsupportedSourceValue() => $_has(30);
  @$pb.TagNumber(55)
  void clearImportUnsupportedSourceValue() => $_clearField(55);
  @$pb.TagNumber(55)
  ImportUnsupportedSourceValue ensureImportUnsupportedSourceValue() =>
      $_ensure(30);

  @$pb.TagNumber(56)
  ImportInconsistentReference get importInconsistentReference => $_getN(31);
  @$pb.TagNumber(56)
  set importInconsistentReference(ImportInconsistentReference value) =>
      $_setField(56, value);
  @$pb.TagNumber(56)
  $core.bool hasImportInconsistentReference() => $_has(31);
  @$pb.TagNumber(56)
  void clearImportInconsistentReference() => $_clearField(56);
  @$pb.TagNumber(56)
  ImportInconsistentReference ensureImportInconsistentReference() =>
      $_ensure(31);

  @$pb.TagNumber(57)
  ImportAdmissionRefused get importAdmissionRefused => $_getN(32);
  @$pb.TagNumber(57)
  set importAdmissionRefused(ImportAdmissionRefused value) =>
      $_setField(57, value);
  @$pb.TagNumber(57)
  $core.bool hasImportAdmissionRefused() => $_has(32);
  @$pb.TagNumber(57)
  void clearImportAdmissionRefused() => $_clearField(57);
  @$pb.TagNumber(57)
  ImportAdmissionRefused ensureImportAdmissionRefused() => $_ensure(32);

  @$pb.TagNumber(58)
  ImportAccountMismatch get importAccountMismatch => $_getN(33);
  @$pb.TagNumber(58)
  set importAccountMismatch(ImportAccountMismatch value) =>
      $_setField(58, value);
  @$pb.TagNumber(58)
  $core.bool hasImportAccountMismatch() => $_has(33);
  @$pb.TagNumber(58)
  void clearImportAccountMismatch() => $_clearField(58);
  @$pb.TagNumber(58)
  ImportAccountMismatch ensureImportAccountMismatch() => $_ensure(33);

  @$pb.TagNumber(70)
  CommandRefused get commandRefused => $_getN(34);
  @$pb.TagNumber(70)
  set commandRefused(CommandRefused value) => $_setField(70, value);
  @$pb.TagNumber(70)
  $core.bool hasCommandRefused() => $_has(34);
  @$pb.TagNumber(70)
  void clearCommandRefused() => $_clearField(70);
  @$pb.TagNumber(70)
  CommandRefused ensureCommandRefused() => $_ensure(34);

  @$pb.TagNumber(71)
  CommandTooLarge get commandTooLarge => $_getN(35);
  @$pb.TagNumber(71)
  set commandTooLarge(CommandTooLarge value) => $_setField(71, value);
  @$pb.TagNumber(71)
  $core.bool hasCommandTooLarge() => $_has(35);
  @$pb.TagNumber(71)
  void clearCommandTooLarge() => $_clearField(71);
  @$pb.TagNumber(71)
  CommandTooLarge ensureCommandTooLarge() => $_ensure(35);

  @$pb.TagNumber(80)
  StoreFailure get storeFailure => $_getN(36);
  @$pb.TagNumber(80)
  set storeFailure(StoreFailure value) => $_setField(80, value);
  @$pb.TagNumber(80)
  $core.bool hasStoreFailure() => $_has(36);
  @$pb.TagNumber(80)
  void clearStoreFailure() => $_clearField(80);
  @$pb.TagNumber(80)
  StoreFailure ensureStoreFailure() => $_ensure(36);

  @$pb.TagNumber(81)
  StoreInvalidLog get storeInvalidLog => $_getN(37);
  @$pb.TagNumber(81)
  set storeInvalidLog(StoreInvalidLog value) => $_setField(81, value);
  @$pb.TagNumber(81)
  $core.bool hasStoreInvalidLog() => $_has(37);
  @$pb.TagNumber(81)
  void clearStoreInvalidLog() => $_clearField(81);
  @$pb.TagNumber(81)
  StoreInvalidLog ensureStoreInvalidLog() => $_ensure(37);

  @$pb.TagNumber(82)
  StoreNewerFormat get storeNewerFormat => $_getN(38);
  @$pb.TagNumber(82)
  set storeNewerFormat(StoreNewerFormat value) => $_setField(82, value);
  @$pb.TagNumber(82)
  $core.bool hasStoreNewerFormat() => $_has(38);
  @$pb.TagNumber(82)
  void clearStoreNewerFormat() => $_clearField(82);
  @$pb.TagNumber(82)
  StoreNewerFormat ensureStoreNewerFormat() => $_ensure(38);

  @$pb.TagNumber(83)
  StoreMalformedCommit get storeMalformedCommit => $_getN(39);
  @$pb.TagNumber(83)
  set storeMalformedCommit(StoreMalformedCommit value) => $_setField(83, value);
  @$pb.TagNumber(83)
  $core.bool hasStoreMalformedCommit() => $_has(39);
  @$pb.TagNumber(83)
  void clearStoreMalformedCommit() => $_clearField(83);
  @$pb.TagNumber(83)
  StoreMalformedCommit ensureStoreMalformedCommit() => $_ensure(39);

  @$pb.TagNumber(84)
  StoreMissing get storeMissing => $_getN(40);
  @$pb.TagNumber(84)
  set storeMissing(StoreMissing value) => $_setField(84, value);
  @$pb.TagNumber(84)
  $core.bool hasStoreMissing() => $_has(40);
  @$pb.TagNumber(84)
  void clearStoreMissing() => $_clearField(84);
  @$pb.TagNumber(84)
  StoreMissing ensureStoreMissing() => $_ensure(40);

  @$pb.TagNumber(85)
  StoreNotADatabase get storeNotADatabase => $_getN(41);
  @$pb.TagNumber(85)
  set storeNotADatabase(StoreNotADatabase value) => $_setField(85, value);
  @$pb.TagNumber(85)
  $core.bool hasStoreNotADatabase() => $_has(41);
  @$pb.TagNumber(85)
  void clearStoreNotADatabase() => $_clearField(85);
  @$pb.TagNumber(85)
  StoreNotADatabase ensureStoreNotADatabase() => $_ensure(41);

  @$pb.TagNumber(86)
  StoreForeign get storeForeign => $_getN(42);
  @$pb.TagNumber(86)
  set storeForeign(StoreForeign value) => $_setField(86, value);
  @$pb.TagNumber(86)
  $core.bool hasStoreForeign() => $_has(42);
  @$pb.TagNumber(86)
  void clearStoreForeign() => $_clearField(86);
  @$pb.TagNumber(86)
  StoreForeign ensureStoreForeign() => $_ensure(42);

  @$pb.TagNumber(87)
  StoreNoFormatVersion get storeNoFormatVersion => $_getN(43);
  @$pb.TagNumber(87)
  set storeNoFormatVersion(StoreNoFormatVersion value) => $_setField(87, value);
  @$pb.TagNumber(87)
  $core.bool hasStoreNoFormatVersion() => $_has(43);
  @$pb.TagNumber(87)
  void clearStoreNoFormatVersion() => $_clearField(87);
  @$pb.TagNumber(87)
  StoreNoFormatVersion ensureStoreNoFormatVersion() => $_ensure(43);

  @$pb.TagNumber(88)
  StoreDamaged get storeDamaged => $_getN(44);
  @$pb.TagNumber(88)
  set storeDamaged(StoreDamaged value) => $_setField(88, value);
  @$pb.TagNumber(88)
  $core.bool hasStoreDamaged() => $_has(44);
  @$pb.TagNumber(88)
  void clearStoreDamaged() => $_clearField(88);
  @$pb.TagNumber(88)
  StoreDamaged ensureStoreDamaged() => $_ensure(44);

  @$pb.TagNumber(89)
  StoreUninitialized get storeUninitialized => $_getN(45);
  @$pb.TagNumber(89)
  set storeUninitialized(StoreUninitialized value) => $_setField(89, value);
  @$pb.TagNumber(89)
  $core.bool hasStoreUninitialized() => $_has(45);
  @$pb.TagNumber(89)
  void clearStoreUninitialized() => $_clearField(89);
  @$pb.TagNumber(89)
  StoreUninitialized ensureStoreUninitialized() => $_ensure(45);

  @$pb.TagNumber(90)
  StoreRetiredFormat get storeRetiredFormat => $_getN(46);
  @$pb.TagNumber(90)
  set storeRetiredFormat(StoreRetiredFormat value) => $_setField(90, value);
  @$pb.TagNumber(90)
  $core.bool hasStoreRetiredFormat() => $_has(46);
  @$pb.TagNumber(90)
  void clearStoreRetiredFormat() => $_clearField(90);
  @$pb.TagNumber(90)
  StoreRetiredFormat ensureStoreRetiredFormat() => $_ensure(46);

  @$pb.TagNumber(100)
  InternalPanic get internalPanic => $_getN(47);
  @$pb.TagNumber(100)
  set internalPanic(InternalPanic value) => $_setField(100, value);
  @$pb.TagNumber(100)
  $core.bool hasInternalPanic() => $_has(47);
  @$pb.TagNumber(100)
  void clearInternalPanic() => $_clearField(100);
  @$pb.TagNumber(100)
  InternalPanic ensureInternalPanic() => $_ensure(47);

  @$pb.TagNumber(101)
  InternalQrTooLong get internalQrTooLong => $_getN(48);
  @$pb.TagNumber(101)
  set internalQrTooLong(InternalQrTooLong value) => $_setField(101, value);
  @$pb.TagNumber(101)
  $core.bool hasInternalQrTooLong() => $_has(48);
  @$pb.TagNumber(101)
  void clearInternalQrTooLong() => $_clearField(101);
  @$pb.TagNumber(101)
  InternalQrTooLong ensureInternalQrTooLong() => $_ensure(48);

  @$pb.TagNumber(102)
  InternalResponseTooLarge get internalResponseTooLarge => $_getN(49);
  @$pb.TagNumber(102)
  set internalResponseTooLarge(InternalResponseTooLarge value) =>
      $_setField(102, value);
  @$pb.TagNumber(102)
  $core.bool hasInternalResponseTooLarge() => $_has(49);
  @$pb.TagNumber(102)
  void clearInternalResponseTooLarge() => $_clearField(102);
  @$pb.TagNumber(102)
  InternalResponseTooLarge ensureInternalResponseTooLarge() => $_ensure(49);

  @$pb.TagNumber(103)
  InternalPageWithoutSoul get internalPageWithoutSoul => $_getN(50);
  @$pb.TagNumber(103)
  set internalPageWithoutSoul(InternalPageWithoutSoul value) =>
      $_setField(103, value);
  @$pb.TagNumber(103)
  $core.bool hasInternalPageWithoutSoul() => $_has(50);
  @$pb.TagNumber(103)
  void clearInternalPageWithoutSoul() => $_clearField(103);
  @$pb.TagNumber(103)
  InternalPageWithoutSoul ensureInternalPageWithoutSoul() => $_ensure(50);

  @$pb.TagNumber(104)
  InternalImportMismatch get internalImportMismatch => $_getN(51);
  @$pb.TagNumber(104)
  set internalImportMismatch(InternalImportMismatch value) =>
      $_setField(104, value);
  @$pb.TagNumber(104)
  $core.bool hasInternalImportMismatch() => $_has(51);
  @$pb.TagNumber(104)
  void clearInternalImportMismatch() => $_clearField(104);
  @$pb.TagNumber(104)
  InternalImportMismatch ensureInternalImportMismatch() => $_ensure(51);
}

enum SessionFailed_Kind {
  sessionMalformedFrame,
  sessionMalformedMessage,
  internalIo,
  notSet
}

/// The session cannot continue; the daemon exits after sending it. No request id is known for it.
class SessionFailed extends $pb.GeneratedMessage {
  factory SessionFailed({
    $core.String? message,
    SessionMalformedFrame? sessionMalformedFrame,
    SessionMalformedMessage? sessionMalformedMessage,
    InternalIo? internalIo,
  }) {
    final result = SessionFailed._();
    if (message != null) result.message = message;
    if (sessionMalformedFrame != null)
      result.sessionMalformedFrame = sessionMalformedFrame;
    if (sessionMalformedMessage != null)
      result.sessionMalformedMessage = sessionMalformedMessage;
    if (internalIo != null) result.internalIo = internalIo;
    return result;
  }

  SessionFailed._();

  factory SessionFailed.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionFailed()..mergeFromBuffer(data, registry);
  factory SessionFailed.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionFailed()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, SessionFailed_Kind>
      _SessionFailed_KindByTag = {
    10: SessionFailed_Kind.sessionMalformedFrame,
    11: SessionFailed_Kind.sessionMalformedMessage,
    12: SessionFailed_Kind.internalIo,
    0: SessionFailed_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionFailed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionFailed.$_createMessage)
    ..oo(0, [10, 11, 12])
    ..aOS(1, _omitFieldNames ? '' : 'message')
    ..aOM<SessionMalformedFrame>(
        10, _omitFieldNames ? '' : 'sessionMalformedFrame',
        subBuilder: SessionMalformedFrame.$_createMessage)
    ..aOM<SessionMalformedMessage>(
        11, _omitFieldNames ? '' : 'sessionMalformedMessage',
        subBuilder: SessionMalformedMessage.$_createMessage)
    ..aOM<InternalIo>(12, _omitFieldNames ? '' : 'internalIo',
        subBuilder: InternalIo.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionFailed clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionFailed copyWith(void Function(SessionFailed) updates) =>
      super.copyWith((message) => updates(message as SessionFailed))
          as SessionFailed;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SessionFailed() / SessionFailed.new instead')
  static SessionFailed create() => SessionFailed._();
  static $pb.GeneratedMessage $_createMessage() => SessionFailed._();
  @$core.override
  SessionFailed createEmptyInstance() => SessionFailed._();
  @$core.pragma('dart2js:noInline')
  static SessionFailed getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SessionFailed>(
          SessionFailed.$_createMessage);
  static SessionFailed? _defaultInstance;

  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  SessionFailed_Kind whichKind() => _SessionFailed_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  void clearKind() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get message => $_getSZ(0);
  @$pb.TagNumber(1)
  set message($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessage() => $_clearField(1);

  @$pb.TagNumber(10)
  SessionMalformedFrame get sessionMalformedFrame => $_getN(1);
  @$pb.TagNumber(10)
  set sessionMalformedFrame(SessionMalformedFrame value) =>
      $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasSessionMalformedFrame() => $_has(1);
  @$pb.TagNumber(10)
  void clearSessionMalformedFrame() => $_clearField(10);
  @$pb.TagNumber(10)
  SessionMalformedFrame ensureSessionMalformedFrame() => $_ensure(1);

  @$pb.TagNumber(11)
  SessionMalformedMessage get sessionMalformedMessage => $_getN(2);
  @$pb.TagNumber(11)
  set sessionMalformedMessage(SessionMalformedMessage value) =>
      $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasSessionMalformedMessage() => $_has(2);
  @$pb.TagNumber(11)
  void clearSessionMalformedMessage() => $_clearField(11);
  @$pb.TagNumber(11)
  SessionMalformedMessage ensureSessionMalformedMessage() => $_ensure(2);

  @$pb.TagNumber(12)
  InternalIo get internalIo => $_getN(3);
  @$pb.TagNumber(12)
  set internalIo(InternalIo value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasInternalIo() => $_has(3);
  @$pb.TagNumber(12)
  void clearInternalIo() => $_clearField(12);
  @$pb.TagNumber(12)
  InternalIo ensureInternalIo() => $_ensure(3);
}

enum ClientFailure_Kind {
  clientDaemonNotFound,
  clientDaemonStartFailed,
  clientDaemonExited,
  clientTimeout,
  clientProtocolError,
  clientNotConnected,
  clientUnexpected,
  notSet
}

/// Raised by the application for what only it can see. Never sent: it is in this file so that
/// every code has one list and both sides read it from one place.
class ClientFailure extends $pb.GeneratedMessage {
  factory ClientFailure({
    ClientDaemonNotFound? clientDaemonNotFound,
    ClientDaemonStartFailed? clientDaemonStartFailed,
    ClientDaemonExited? clientDaemonExited,
    ClientTimeout? clientTimeout,
    ClientProtocolError? clientProtocolError,
    ClientNotConnected? clientNotConnected,
    ClientUnexpected? clientUnexpected,
  }) {
    final result = ClientFailure._();
    if (clientDaemonNotFound != null)
      result.clientDaemonNotFound = clientDaemonNotFound;
    if (clientDaemonStartFailed != null)
      result.clientDaemonStartFailed = clientDaemonStartFailed;
    if (clientDaemonExited != null)
      result.clientDaemonExited = clientDaemonExited;
    if (clientTimeout != null) result.clientTimeout = clientTimeout;
    if (clientProtocolError != null)
      result.clientProtocolError = clientProtocolError;
    if (clientNotConnected != null)
      result.clientNotConnected = clientNotConnected;
    if (clientUnexpected != null) result.clientUnexpected = clientUnexpected;
    return result;
  }

  ClientFailure._();

  factory ClientFailure.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientFailure()..mergeFromBuffer(data, registry);
  factory ClientFailure.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientFailure()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ClientFailure_Kind>
      _ClientFailure_KindByTag = {
    10: ClientFailure_Kind.clientDaemonNotFound,
    11: ClientFailure_Kind.clientDaemonStartFailed,
    12: ClientFailure_Kind.clientDaemonExited,
    13: ClientFailure_Kind.clientTimeout,
    14: ClientFailure_Kind.clientProtocolError,
    15: ClientFailure_Kind.clientNotConnected,
    16: ClientFailure_Kind.clientUnexpected,
    0: ClientFailure_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientFailure',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientFailure.$_createMessage)
    ..oo(0, [10, 11, 12, 13, 14, 15, 16])
    ..aOM<ClientDaemonNotFound>(
        10, _omitFieldNames ? '' : 'clientDaemonNotFound',
        subBuilder: ClientDaemonNotFound.$_createMessage)
    ..aOM<ClientDaemonStartFailed>(
        11, _omitFieldNames ? '' : 'clientDaemonStartFailed',
        subBuilder: ClientDaemonStartFailed.$_createMessage)
    ..aOM<ClientDaemonExited>(12, _omitFieldNames ? '' : 'clientDaemonExited',
        subBuilder: ClientDaemonExited.$_createMessage)
    ..aOM<ClientTimeout>(13, _omitFieldNames ? '' : 'clientTimeout',
        subBuilder: ClientTimeout.$_createMessage)
    ..aOM<ClientProtocolError>(14, _omitFieldNames ? '' : 'clientProtocolError',
        subBuilder: ClientProtocolError.$_createMessage)
    ..aOM<ClientNotConnected>(15, _omitFieldNames ? '' : 'clientNotConnected',
        subBuilder: ClientNotConnected.$_createMessage)
    ..aOM<ClientUnexpected>(16, _omitFieldNames ? '' : 'clientUnexpected',
        subBuilder: ClientUnexpected.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientFailure clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientFailure copyWith(void Function(ClientFailure) updates) =>
      super.copyWith((message) => updates(message as ClientFailure))
          as ClientFailure;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ClientFailure() / ClientFailure.new instead')
  static ClientFailure create() => ClientFailure._();
  static $pb.GeneratedMessage $_createMessage() => ClientFailure._();
  @$core.override
  ClientFailure createEmptyInstance() => ClientFailure._();
  @$core.pragma('dart2js:noInline')
  static ClientFailure getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClientFailure>(
          ClientFailure.$_createMessage);
  static ClientFailure? _defaultInstance;

  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  ClientFailure_Kind whichKind() => _ClientFailure_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  @$pb.TagNumber(15)
  @$pb.TagNumber(16)
  void clearKind() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(10)
  ClientDaemonNotFound get clientDaemonNotFound => $_getN(0);
  @$pb.TagNumber(10)
  set clientDaemonNotFound(ClientDaemonNotFound value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasClientDaemonNotFound() => $_has(0);
  @$pb.TagNumber(10)
  void clearClientDaemonNotFound() => $_clearField(10);
  @$pb.TagNumber(10)
  ClientDaemonNotFound ensureClientDaemonNotFound() => $_ensure(0);

  @$pb.TagNumber(11)
  ClientDaemonStartFailed get clientDaemonStartFailed => $_getN(1);
  @$pb.TagNumber(11)
  set clientDaemonStartFailed(ClientDaemonStartFailed value) =>
      $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasClientDaemonStartFailed() => $_has(1);
  @$pb.TagNumber(11)
  void clearClientDaemonStartFailed() => $_clearField(11);
  @$pb.TagNumber(11)
  ClientDaemonStartFailed ensureClientDaemonStartFailed() => $_ensure(1);

  @$pb.TagNumber(12)
  ClientDaemonExited get clientDaemonExited => $_getN(2);
  @$pb.TagNumber(12)
  set clientDaemonExited(ClientDaemonExited value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasClientDaemonExited() => $_has(2);
  @$pb.TagNumber(12)
  void clearClientDaemonExited() => $_clearField(12);
  @$pb.TagNumber(12)
  ClientDaemonExited ensureClientDaemonExited() => $_ensure(2);

  @$pb.TagNumber(13)
  ClientTimeout get clientTimeout => $_getN(3);
  @$pb.TagNumber(13)
  set clientTimeout(ClientTimeout value) => $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasClientTimeout() => $_has(3);
  @$pb.TagNumber(13)
  void clearClientTimeout() => $_clearField(13);
  @$pb.TagNumber(13)
  ClientTimeout ensureClientTimeout() => $_ensure(3);

  @$pb.TagNumber(14)
  ClientProtocolError get clientProtocolError => $_getN(4);
  @$pb.TagNumber(14)
  set clientProtocolError(ClientProtocolError value) => $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasClientProtocolError() => $_has(4);
  @$pb.TagNumber(14)
  void clearClientProtocolError() => $_clearField(14);
  @$pb.TagNumber(14)
  ClientProtocolError ensureClientProtocolError() => $_ensure(4);

  @$pb.TagNumber(15)
  ClientNotConnected get clientNotConnected => $_getN(5);
  @$pb.TagNumber(15)
  set clientNotConnected(ClientNotConnected value) => $_setField(15, value);
  @$pb.TagNumber(15)
  $core.bool hasClientNotConnected() => $_has(5);
  @$pb.TagNumber(15)
  void clearClientNotConnected() => $_clearField(15);
  @$pb.TagNumber(15)
  ClientNotConnected ensureClientNotConnected() => $_ensure(5);

  @$pb.TagNumber(16)
  ClientUnexpected get clientUnexpected => $_getN(6);
  @$pb.TagNumber(16)
  set clientUnexpected(ClientUnexpected value) => $_setField(16, value);
  @$pb.TagNumber(16)
  $core.bool hasClientUnexpected() => $_has(6);
  @$pb.TagNumber(16)
  void clearClientUnexpected() => $_clearField(16);
  @$pb.TagNumber(16)
  ClientUnexpected ensureClientUnexpected() => $_ensure(6);
}

class SessionProtocolUnsupported extends $pb.GeneratedMessage {
  factory SessionProtocolUnsupported({
    ProtocolVersion? client,
    ProtocolVersion? daemon,
  }) {
    final result = SessionProtocolUnsupported._();
    if (client != null) result.client = client;
    if (daemon != null) result.daemon = daemon;
    return result;
  }

  SessionProtocolUnsupported._();

  factory SessionProtocolUnsupported.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionProtocolUnsupported()..mergeFromBuffer(data, registry);
  factory SessionProtocolUnsupported.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionProtocolUnsupported()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionProtocolUnsupported',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionProtocolUnsupported.$_createMessage)
    ..aOM<ProtocolVersion>(1, _omitFieldNames ? '' : 'client',
        subBuilder: ProtocolVersion.$_createMessage)
    ..aOM<ProtocolVersion>(2, _omitFieldNames ? '' : 'daemon',
        subBuilder: ProtocolVersion.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionProtocolUnsupported clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionProtocolUnsupported copyWith(
          void Function(SessionProtocolUnsupported) updates) =>
      super.copyWith(
              (message) => updates(message as SessionProtocolUnsupported))
          as SessionProtocolUnsupported;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SessionProtocolUnsupported() / SessionProtocolUnsupported.new instead')
  static SessionProtocolUnsupported create() => SessionProtocolUnsupported._();
  static $pb.GeneratedMessage $_createMessage() =>
      SessionProtocolUnsupported._();
  @$core.override
  SessionProtocolUnsupported createEmptyInstance() =>
      SessionProtocolUnsupported._();
  @$core.pragma('dart2js:noInline')
  static SessionProtocolUnsupported getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionProtocolUnsupported>(
          SessionProtocolUnsupported.$_createMessage);
  static SessionProtocolUnsupported? _defaultInstance;

  /// Absent when the client sent no version.
  @$pb.TagNumber(1)
  ProtocolVersion get client => $_getN(0);
  @$pb.TagNumber(1)
  set client(ProtocolVersion value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasClient() => $_has(0);
  @$pb.TagNumber(1)
  void clearClient() => $_clearField(1);
  @$pb.TagNumber(1)
  ProtocolVersion ensureClient() => $_ensure(0);

  @$pb.TagNumber(2)
  ProtocolVersion get daemon => $_getN(1);
  @$pb.TagNumber(2)
  set daemon(ProtocolVersion value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasDaemon() => $_has(1);
  @$pb.TagNumber(2)
  void clearDaemon() => $_clearField(2);
  @$pb.TagNumber(2)
  ProtocolVersion ensureDaemon() => $_ensure(1);
}

class SessionNotOpen extends $pb.GeneratedMessage {
  factory SessionNotOpen({
    $core.String? request,
  }) {
    final result = SessionNotOpen._();
    if (request != null) result.request = request;
    return result;
  }

  SessionNotOpen._();

  factory SessionNotOpen.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionNotOpen()..mergeFromBuffer(data, registry);
  factory SessionNotOpen.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionNotOpen()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionNotOpen',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionNotOpen.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'request')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionNotOpen clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionNotOpen copyWith(void Function(SessionNotOpen) updates) =>
      super.copyWith((message) => updates(message as SessionNotOpen))
          as SessionNotOpen;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SessionNotOpen() / SessionNotOpen.new instead')
  static SessionNotOpen create() => SessionNotOpen._();
  static $pb.GeneratedMessage $_createMessage() => SessionNotOpen._();
  @$core.override
  SessionNotOpen createEmptyInstance() => SessionNotOpen._();
  @$core.pragma('dart2js:noInline')
  static SessionNotOpen getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SessionNotOpen>(
          SessionNotOpen.$_createMessage);
  static SessionNotOpen? _defaultInstance;

  /// The request kind that came before `OpenSession`, as its field name.
  @$pb.TagNumber(1)
  $core.String get request => $_getSZ(0);
  @$pb.TagNumber(1)
  set request($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequest() => $_clearField(1);
}

class SessionAlreadyOpen extends $pb.GeneratedMessage {
  factory SessionAlreadyOpen() => SessionAlreadyOpen._();

  SessionAlreadyOpen._();

  factory SessionAlreadyOpen.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionAlreadyOpen()..mergeFromBuffer(data, registry);
  factory SessionAlreadyOpen.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionAlreadyOpen()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionAlreadyOpen',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionAlreadyOpen.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionAlreadyOpen clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionAlreadyOpen copyWith(void Function(SessionAlreadyOpen) updates) =>
      super.copyWith((message) => updates(message as SessionAlreadyOpen))
          as SessionAlreadyOpen;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use SessionAlreadyOpen() / SessionAlreadyOpen.new instead')
  static SessionAlreadyOpen create() => SessionAlreadyOpen._();
  static $pb.GeneratedMessage $_createMessage() => SessionAlreadyOpen._();
  @$core.override
  SessionAlreadyOpen createEmptyInstance() => SessionAlreadyOpen._();
  @$core.pragma('dart2js:noInline')
  static SessionAlreadyOpen getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionAlreadyOpen>(
          SessionAlreadyOpen.$_createMessage);
  static SessionAlreadyOpen? _defaultInstance;
}

class SessionInvalidRequestId extends $pb.GeneratedMessage {
  factory SessionInvalidRequestId({
    $fixnum.Int64? id,
  }) {
    final result = SessionInvalidRequestId._();
    if (id != null) result.id = id;
    return result;
  }

  SessionInvalidRequestId._();

  factory SessionInvalidRequestId.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionInvalidRequestId()..mergeFromBuffer(data, registry);
  factory SessionInvalidRequestId.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionInvalidRequestId()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionInvalidRequestId',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionInvalidRequestId.$_createMessage)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionInvalidRequestId clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionInvalidRequestId copyWith(
          void Function(SessionInvalidRequestId) updates) =>
      super.copyWith((message) => updates(message as SessionInvalidRequestId))
          as SessionInvalidRequestId;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SessionInvalidRequestId() / SessionInvalidRequestId.new instead')
  static SessionInvalidRequestId create() => SessionInvalidRequestId._();
  static $pb.GeneratedMessage $_createMessage() => SessionInvalidRequestId._();
  @$core.override
  SessionInvalidRequestId createEmptyInstance() => SessionInvalidRequestId._();
  @$core.pragma('dart2js:noInline')
  static SessionInvalidRequestId getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionInvalidRequestId>(
          SessionInvalidRequestId.$_createMessage);
  static SessionInvalidRequestId? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class SessionUnknownRequest extends $pb.GeneratedMessage {
  factory SessionUnknownRequest() => SessionUnknownRequest._();

  SessionUnknownRequest._();

  factory SessionUnknownRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionUnknownRequest()..mergeFromBuffer(data, registry);
  factory SessionUnknownRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionUnknownRequest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionUnknownRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionUnknownRequest.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionUnknownRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionUnknownRequest copyWith(
          void Function(SessionUnknownRequest) updates) =>
      super.copyWith((message) => updates(message as SessionUnknownRequest))
          as SessionUnknownRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SessionUnknownRequest() / SessionUnknownRequest.new instead')
  static SessionUnknownRequest create() => SessionUnknownRequest._();
  static $pb.GeneratedMessage $_createMessage() => SessionUnknownRequest._();
  @$core.override
  SessionUnknownRequest createEmptyInstance() => SessionUnknownRequest._();
  @$core.pragma('dart2js:noInline')
  static SessionUnknownRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionUnknownRequest>(
          SessionUnknownRequest.$_createMessage);
  static SessionUnknownRequest? _defaultInstance;
}

class QueryUnknownProfile extends $pb.GeneratedMessage {
  factory QueryUnknownProfile({
    $core.String? profileId,
  }) {
    final result = QueryUnknownProfile._();
    if (profileId != null) result.profileId = profileId;
    return result;
  }

  QueryUnknownProfile._();

  factory QueryUnknownProfile.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryUnknownProfile()..mergeFromBuffer(data, registry);
  factory QueryUnknownProfile.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryUnknownProfile()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryUnknownProfile',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryUnknownProfile.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'profileId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryUnknownProfile clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryUnknownProfile copyWith(void Function(QueryUnknownProfile) updates) =>
      super.copyWith((message) => updates(message as QueryUnknownProfile))
          as QueryUnknownProfile;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use QueryUnknownProfile() / QueryUnknownProfile.new instead')
  static QueryUnknownProfile create() => QueryUnknownProfile._();
  static $pb.GeneratedMessage $_createMessage() => QueryUnknownProfile._();
  @$core.override
  QueryUnknownProfile createEmptyInstance() => QueryUnknownProfile._();
  @$core.pragma('dart2js:noInline')
  static QueryUnknownProfile getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryUnknownProfile>(
          QueryUnknownProfile.$_createMessage);
  static QueryUnknownProfile? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get profileId => $_getSZ(0);
  @$pb.TagNumber(1)
  set profileId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProfileId() => $_has(0);
  @$pb.TagNumber(1)
  void clearProfileId() => $_clearField(1);
}

class QueryStaleRevision extends $pb.GeneratedMessage {
  factory QueryStaleRevision({
    $fixnum.Int64? scan,
    $fixnum.Int64? current,
  }) {
    final result = QueryStaleRevision._();
    if (scan != null) result.scan = scan;
    if (current != null) result.current = current;
    return result;
  }

  QueryStaleRevision._();

  factory QueryStaleRevision.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryStaleRevision()..mergeFromBuffer(data, registry);
  factory QueryStaleRevision.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryStaleRevision()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryStaleRevision',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryStaleRevision.$_createMessage)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'scan', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'current', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryStaleRevision clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryStaleRevision copyWith(void Function(QueryStaleRevision) updates) =>
      super.copyWith((message) => updates(message as QueryStaleRevision))
          as QueryStaleRevision;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryStaleRevision() / QueryStaleRevision.new instead')
  static QueryStaleRevision create() => QueryStaleRevision._();
  static $pb.GeneratedMessage $_createMessage() => QueryStaleRevision._();
  @$core.override
  QueryStaleRevision createEmptyInstance() => QueryStaleRevision._();
  @$core.pragma('dart2js:noInline')
  static QueryStaleRevision getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryStaleRevision>(
          QueryStaleRevision.$_createMessage);
  static QueryStaleRevision? _defaultInstance;

  /// The revision the scan began at, and the projection's.
  @$pb.TagNumber(1)
  $fixnum.Int64 get scan => $_getI64(0);
  @$pb.TagNumber(1)
  set scan($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasScan() => $_has(0);
  @$pb.TagNumber(1)
  void clearScan() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get current => $_getI64(1);
  @$pb.TagNumber(2)
  set current($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCurrent() => $_has(1);
  @$pb.TagNumber(2)
  void clearCurrent() => $_clearField(2);
}

class QueryMalformed extends $pb.GeneratedMessage {
  factory QueryMalformed({
    $core.String? problem,
  }) {
    final result = QueryMalformed._();
    if (problem != null) result.problem = problem;
    return result;
  }

  QueryMalformed._();

  factory QueryMalformed.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryMalformed()..mergeFromBuffer(data, registry);
  factory QueryMalformed.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryMalformed()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryMalformed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryMalformed.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryMalformed clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryMalformed copyWith(void Function(QueryMalformed) updates) =>
      super.copyWith((message) => updates(message as QueryMalformed))
          as QueryMalformed;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryMalformed() / QueryMalformed.new instead')
  static QueryMalformed create() => QueryMalformed._();
  static $pb.GeneratedMessage $_createMessage() => QueryMalformed._();
  @$core.override
  QueryMalformed createEmptyInstance() => QueryMalformed._();
  @$core.pragma('dart2js:noInline')
  static QueryMalformed getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryMalformed>(
          QueryMalformed.$_createMessage);
  static QueryMalformed? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class QueryUnknownField extends $pb.GeneratedMessage {
  factory QueryUnknownField({
    $core.int? value,
  }) {
    final result = QueryUnknownField._();
    if (value != null) result.value = value;
    return result;
  }

  QueryUnknownField._();

  factory QueryUnknownField.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryUnknownField()..mergeFromBuffer(data, registry);
  factory QueryUnknownField.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryUnknownField()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryUnknownField',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryUnknownField.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'value')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryUnknownField clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryUnknownField copyWith(void Function(QueryUnknownField) updates) =>
      super.copyWith((message) => updates(message as QueryUnknownField))
          as QueryUnknownField;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryUnknownField() / QueryUnknownField.new instead')
  static QueryUnknownField create() => QueryUnknownField._();
  static $pb.GeneratedMessage $_createMessage() => QueryUnknownField._();
  @$core.override
  QueryUnknownField createEmptyInstance() => QueryUnknownField._();
  @$core.pragma('dart2js:noInline')
  static QueryUnknownField getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryUnknownField>(
          QueryUnknownField.$_createMessage);
  static QueryUnknownField? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get value => $_getIZ(0);
  @$pb.TagNumber(1)
  set value($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasValue() => $_has(0);
  @$pb.TagNumber(1)
  void clearValue() => $_clearField(1);
}

class QueryTooComplex extends $pb.GeneratedMessage {
  factory QueryTooComplex({
    $core.String? limit,
    $fixnum.Int64? found,
    $fixnum.Int64? max,
  }) {
    final result = QueryTooComplex._();
    if (limit != null) result.limit = limit;
    if (found != null) result.found = found;
    if (max != null) result.max = max;
    return result;
  }

  QueryTooComplex._();

  factory QueryTooComplex.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryTooComplex()..mergeFromBuffer(data, registry);
  factory QueryTooComplex.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryTooComplex()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryTooComplex',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryTooComplex.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'limit')
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'found', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'max', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryTooComplex clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryTooComplex copyWith(void Function(QueryTooComplex) updates) =>
      super.copyWith((message) => updates(message as QueryTooComplex))
          as QueryTooComplex;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryTooComplex() / QueryTooComplex.new instead')
  static QueryTooComplex create() => QueryTooComplex._();
  static $pb.GeneratedMessage $_createMessage() => QueryTooComplex._();
  @$core.override
  QueryTooComplex createEmptyInstance() => QueryTooComplex._();
  @$core.pragma('dart2js:noInline')
  static QueryTooComplex getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryTooComplex>(
          QueryTooComplex.$_createMessage);
  static QueryTooComplex? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get limit => $_getSZ(0);
  @$pb.TagNumber(1)
  set limit($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasLimit() => $_has(0);
  @$pb.TagNumber(1)
  void clearLimit() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get found => $_getI64(1);
  @$pb.TagNumber(2)
  set found($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFound() => $_has(1);
  @$pb.TagNumber(2)
  void clearFound() => $_clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get max => $_getI64(2);
  @$pb.TagNumber(3)
  set max($fixnum.Int64 value) => $_setInt64(2, value);
  @$pb.TagNumber(3)
  $core.bool hasMax() => $_has(2);
  @$pb.TagNumber(3)
  void clearMax() => $_clearField(3);
}

class QueryTypeMismatch extends $pb.GeneratedMessage {
  factory QueryTypeMismatch({
    $core.String? problem,
  }) {
    final result = QueryTypeMismatch._();
    if (problem != null) result.problem = problem;
    return result;
  }

  QueryTypeMismatch._();

  factory QueryTypeMismatch.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryTypeMismatch()..mergeFromBuffer(data, registry);
  factory QueryTypeMismatch.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryTypeMismatch()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryTypeMismatch',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryTypeMismatch.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryTypeMismatch clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryTypeMismatch copyWith(void Function(QueryTypeMismatch) updates) =>
      super.copyWith((message) => updates(message as QueryTypeMismatch))
          as QueryTypeMismatch;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryTypeMismatch() / QueryTypeMismatch.new instead')
  static QueryTypeMismatch create() => QueryTypeMismatch._();
  static $pb.GeneratedMessage $_createMessage() => QueryTypeMismatch._();
  @$core.override
  QueryTypeMismatch createEmptyInstance() => QueryTypeMismatch._();
  @$core.pragma('dart2js:noInline')
  static QueryTypeMismatch getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryTypeMismatch>(
          QueryTypeMismatch.$_createMessage);
  static QueryTypeMismatch? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class QueryParamSetRequired extends $pb.GeneratedMessage {
  factory QueryParamSetRequired({
    $core.String? field_1,
  }) {
    final result = QueryParamSetRequired._();
    if (field_1 != null) result.field_1 = field_1;
    return result;
  }

  QueryParamSetRequired._();

  factory QueryParamSetRequired.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryParamSetRequired()..mergeFromBuffer(data, registry);
  factory QueryParamSetRequired.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryParamSetRequired()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryParamSetRequired',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryParamSetRequired.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'field')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryParamSetRequired clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryParamSetRequired copyWith(
          void Function(QueryParamSetRequired) updates) =>
      super.copyWith((message) => updates(message as QueryParamSetRequired))
          as QueryParamSetRequired;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use QueryParamSetRequired() / QueryParamSetRequired.new instead')
  static QueryParamSetRequired create() => QueryParamSetRequired._();
  static $pb.GeneratedMessage $_createMessage() => QueryParamSetRequired._();
  @$core.override
  QueryParamSetRequired createEmptyInstance() => QueryParamSetRequired._();
  @$core.pragma('dart2js:noInline')
  static QueryParamSetRequired getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryParamSetRequired>(
          QueryParamSetRequired.$_createMessage);
  static QueryParamSetRequired? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get field_1 => $_getSZ(0);
  @$pb.TagNumber(1)
  set field_1($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasField_1() => $_has(0);
  @$pb.TagNumber(1)
  void clearField_1() => $_clearField(1);
}

class QueryFieldUnavailable extends $pb.GeneratedMessage {
  factory QueryFieldUnavailable({
    $core.String? field_1,
  }) {
    final result = QueryFieldUnavailable._();
    if (field_1 != null) result.field_1 = field_1;
    return result;
  }

  QueryFieldUnavailable._();

  factory QueryFieldUnavailable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryFieldUnavailable()..mergeFromBuffer(data, registry);
  factory QueryFieldUnavailable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryFieldUnavailable()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryFieldUnavailable',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryFieldUnavailable.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'field')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryFieldUnavailable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryFieldUnavailable copyWith(
          void Function(QueryFieldUnavailable) updates) =>
      super.copyWith((message) => updates(message as QueryFieldUnavailable))
          as QueryFieldUnavailable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use QueryFieldUnavailable() / QueryFieldUnavailable.new instead')
  static QueryFieldUnavailable create() => QueryFieldUnavailable._();
  static $pb.GeneratedMessage $_createMessage() => QueryFieldUnavailable._();
  @$core.override
  QueryFieldUnavailable createEmptyInstance() => QueryFieldUnavailable._();
  @$core.pragma('dart2js:noInline')
  static QueryFieldUnavailable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryFieldUnavailable>(
          QueryFieldUnavailable.$_createMessage);
  static QueryFieldUnavailable? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get field_1 => $_getSZ(0);
  @$pb.TagNumber(1)
  set field_1($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasField_1() => $_has(0);
  @$pb.TagNumber(1)
  void clearField_1() => $_clearField(1);
}

class QueryUnknownScheme extends $pb.GeneratedMessage {
  factory QueryUnknownScheme({
    $core.String? problem,
  }) {
    final result = QueryUnknownScheme._();
    if (problem != null) result.problem = problem;
    return result;
  }

  QueryUnknownScheme._();

  factory QueryUnknownScheme.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryUnknownScheme()..mergeFromBuffer(data, registry);
  factory QueryUnknownScheme.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryUnknownScheme()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryUnknownScheme',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryUnknownScheme.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryUnknownScheme clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryUnknownScheme copyWith(void Function(QueryUnknownScheme) updates) =>
      super.copyWith((message) => updates(message as QueryUnknownScheme))
          as QueryUnknownScheme;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use QueryUnknownScheme() / QueryUnknownScheme.new instead')
  static QueryUnknownScheme create() => QueryUnknownScheme._();
  static $pb.GeneratedMessage $_createMessage() => QueryUnknownScheme._();
  @$core.override
  QueryUnknownScheme createEmptyInstance() => QueryUnknownScheme._();
  @$core.pragma('dart2js:noInline')
  static QueryUnknownScheme getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryUnknownScheme>(
          QueryUnknownScheme.$_createMessage);
  static QueryUnknownScheme? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class QueryMalformedCursor extends $pb.GeneratedMessage {
  factory QueryMalformedCursor() => QueryMalformedCursor._();

  QueryMalformedCursor._();

  factory QueryMalformedCursor.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryMalformedCursor()..mergeFromBuffer(data, registry);
  factory QueryMalformedCursor.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryMalformedCursor()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryMalformedCursor',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryMalformedCursor.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryMalformedCursor clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  QueryMalformedCursor copyWith(void Function(QueryMalformedCursor) updates) =>
      super.copyWith((message) => updates(message as QueryMalformedCursor))
          as QueryMalformedCursor;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use QueryMalformedCursor() / QueryMalformedCursor.new instead')
  static QueryMalformedCursor create() => QueryMalformedCursor._();
  static $pb.GeneratedMessage $_createMessage() => QueryMalformedCursor._();
  @$core.override
  QueryMalformedCursor createEmptyInstance() => QueryMalformedCursor._();
  @$core.pragma('dart2js:noInline')
  static QueryMalformedCursor getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<QueryMalformedCursor>(
          QueryMalformedCursor.$_createMessage);
  static QueryMalformedCursor? _defaultInstance;
}

class DecodeNoInput extends $pb.GeneratedMessage {
  factory DecodeNoInput() => DecodeNoInput._();

  DecodeNoInput._();

  factory DecodeNoInput.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeNoInput()..mergeFromBuffer(data, registry);
  factory DecodeNoInput.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeNoInput()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeNoInput',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeNoInput.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeNoInput clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeNoInput copyWith(void Function(DecodeNoInput) updates) =>
      super.copyWith((message) => updates(message as DecodeNoInput))
          as DecodeNoInput;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DecodeNoInput() / DecodeNoInput.new instead')
  static DecodeNoInput create() => DecodeNoInput._();
  static $pb.GeneratedMessage $_createMessage() => DecodeNoInput._();
  @$core.override
  DecodeNoInput createEmptyInstance() => DecodeNoInput._();
  @$core.pragma('dart2js:noInline')
  static DecodeNoInput getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DecodeNoInput>(
          DecodeNoInput.$_createMessage);
  static DecodeNoInput? _defaultInstance;
}

class DecodeMalformedText extends $pb.GeneratedMessage {
  factory DecodeMalformedText({
    $core.String? problem,
  }) {
    final result = DecodeMalformedText._();
    if (problem != null) result.problem = problem;
    return result;
  }

  DecodeMalformedText._();

  factory DecodeMalformedText.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeMalformedText()..mergeFromBuffer(data, registry);
  factory DecodeMalformedText.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeMalformedText()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeMalformedText',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeMalformedText.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeMalformedText clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeMalformedText copyWith(void Function(DecodeMalformedText) updates) =>
      super.copyWith((message) => updates(message as DecodeMalformedText))
          as DecodeMalformedText;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use DecodeMalformedText() / DecodeMalformedText.new instead')
  static DecodeMalformedText create() => DecodeMalformedText._();
  static $pb.GeneratedMessage $_createMessage() => DecodeMalformedText._();
  @$core.override
  DecodeMalformedText createEmptyInstance() => DecodeMalformedText._();
  @$core.pragma('dart2js:noInline')
  static DecodeMalformedText getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecodeMalformedText>(
          DecodeMalformedText.$_createMessage);
  static DecodeMalformedText? _defaultInstance;

  /// The transport layer that refused the text, with its position.
  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class DecodeUnknownFormat extends $pb.GeneratedMessage {
  factory DecodeUnknownFormat({
    $fixnum.Int64? payloadBytes,
  }) {
    final result = DecodeUnknownFormat._();
    if (payloadBytes != null) result.payloadBytes = payloadBytes;
    return result;
  }

  DecodeUnknownFormat._();

  factory DecodeUnknownFormat.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeUnknownFormat()..mergeFromBuffer(data, registry);
  factory DecodeUnknownFormat.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeUnknownFormat()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeUnknownFormat',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeUnknownFormat.$_createMessage)
    ..a<$fixnum.Int64>(
        1, _omitFieldNames ? '' : 'payloadBytes', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeUnknownFormat clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeUnknownFormat copyWith(void Function(DecodeUnknownFormat) updates) =>
      super.copyWith((message) => updates(message as DecodeUnknownFormat))
          as DecodeUnknownFormat;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use DecodeUnknownFormat() / DecodeUnknownFormat.new instead')
  static DecodeUnknownFormat create() => DecodeUnknownFormat._();
  static $pb.GeneratedMessage $_createMessage() => DecodeUnknownFormat._();
  @$core.override
  DecodeUnknownFormat createEmptyInstance() => DecodeUnknownFormat._();
  @$core.pragma('dart2js:noInline')
  static DecodeUnknownFormat getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecodeUnknownFormat>(
          DecodeUnknownFormat.$_createMessage);
  static DecodeUnknownFormat? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get payloadBytes => $_getI64(0);
  @$pb.TagNumber(1)
  set payloadBytes($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPayloadBytes() => $_has(0);
  @$pb.TagNumber(1)
  void clearPayloadBytes() => $_clearField(1);
}

class DecodeMalformedLayout extends $pb.GeneratedMessage {
  factory DecodeMalformedLayout({
    $core.String? problem,
  }) {
    final result = DecodeMalformedLayout._();
    if (problem != null) result.problem = problem;
    return result;
  }

  DecodeMalformedLayout._();

  factory DecodeMalformedLayout.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeMalformedLayout()..mergeFromBuffer(data, registry);
  factory DecodeMalformedLayout.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeMalformedLayout()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeMalformedLayout',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeMalformedLayout.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeMalformedLayout clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeMalformedLayout copyWith(
          void Function(DecodeMalformedLayout) updates) =>
      super.copyWith((message) => updates(message as DecodeMalformedLayout))
          as DecodeMalformedLayout;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DecodeMalformedLayout() / DecodeMalformedLayout.new instead')
  static DecodeMalformedLayout create() => DecodeMalformedLayout._();
  static $pb.GeneratedMessage $_createMessage() => DecodeMalformedLayout._();
  @$core.override
  DecodeMalformedLayout createEmptyInstance() => DecodeMalformedLayout._();
  @$core.pragma('dart2js:noInline')
  static DecodeMalformedLayout getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecodeMalformedLayout>(
          DecodeMalformedLayout.$_createMessage);
  static DecodeMalformedLayout? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class DecodeMalformedScheme extends $pb.GeneratedMessage {
  factory DecodeMalformedScheme({
    $core.int? record,
    $core.String? problem,
  }) {
    final result = DecodeMalformedScheme._();
    if (record != null) result.record = record;
    if (problem != null) result.problem = problem;
    return result;
  }

  DecodeMalformedScheme._();

  factory DecodeMalformedScheme.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeMalformedScheme()..mergeFromBuffer(data, registry);
  factory DecodeMalformedScheme.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeMalformedScheme()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeMalformedScheme',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeMalformedScheme.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'record', fieldType: $pb.PbFieldType.OU3)
    ..aOS(2, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeMalformedScheme clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeMalformedScheme copyWith(
          void Function(DecodeMalformedScheme) updates) =>
      super.copyWith((message) => updates(message as DecodeMalformedScheme))
          as DecodeMalformedScheme;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DecodeMalformedScheme() / DecodeMalformedScheme.new instead')
  static DecodeMalformedScheme create() => DecodeMalformedScheme._();
  static $pb.GeneratedMessage $_createMessage() => DecodeMalformedScheme._();
  @$core.override
  DecodeMalformedScheme createEmptyInstance() => DecodeMalformedScheme._();
  @$core.pragma('dart2js:noInline')
  static DecodeMalformedScheme getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecodeMalformedScheme>(
          DecodeMalformedScheme.$_createMessage);
  static DecodeMalformedScheme? _defaultInstance;

  /// The plan or discard scheme, by position in the code; absent when no one record is at fault.
  @$pb.TagNumber(1)
  $core.int get record => $_getIZ(0);
  @$pb.TagNumber(1)
  set record($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRecord() => $_has(0);
  @$pb.TagNumber(1)
  void clearRecord() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get problem => $_getSZ(1);
  @$pb.TagNumber(2)
  set problem($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasProblem() => $_has(1);
  @$pb.TagNumber(2)
  void clearProblem() => $_clearField(2);
}

class DecodeImageInvalid extends $pb.GeneratedMessage {
  factory DecodeImageInvalid({
    $core.String? problem,
  }) {
    final result = DecodeImageInvalid._();
    if (problem != null) result.problem = problem;
    return result;
  }

  DecodeImageInvalid._();

  factory DecodeImageInvalid.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeImageInvalid()..mergeFromBuffer(data, registry);
  factory DecodeImageInvalid.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeImageInvalid()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeImageInvalid',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeImageInvalid.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeImageInvalid clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeImageInvalid copyWith(void Function(DecodeImageInvalid) updates) =>
      super.copyWith((message) => updates(message as DecodeImageInvalid))
          as DecodeImageInvalid;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DecodeImageInvalid() / DecodeImageInvalid.new instead')
  static DecodeImageInvalid create() => DecodeImageInvalid._();
  static $pb.GeneratedMessage $_createMessage() => DecodeImageInvalid._();
  @$core.override
  DecodeImageInvalid createEmptyInstance() => DecodeImageInvalid._();
  @$core.pragma('dart2js:noInline')
  static DecodeImageInvalid getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecodeImageInvalid>(
          DecodeImageInvalid.$_createMessage);
  static DecodeImageInvalid? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class DecodeNoQrCode extends $pb.GeneratedMessage {
  factory DecodeNoQrCode() => DecodeNoQrCode._();

  DecodeNoQrCode._();

  factory DecodeNoQrCode.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeNoQrCode()..mergeFromBuffer(data, registry);
  factory DecodeNoQrCode.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeNoQrCode()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeNoQrCode',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeNoQrCode.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeNoQrCode clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeNoQrCode copyWith(void Function(DecodeNoQrCode) updates) =>
      super.copyWith((message) => updates(message as DecodeNoQrCode))
          as DecodeNoQrCode;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DecodeNoQrCode() / DecodeNoQrCode.new instead')
  static DecodeNoQrCode create() => DecodeNoQrCode._();
  static $pb.GeneratedMessage $_createMessage() => DecodeNoQrCode._();
  @$core.override
  DecodeNoQrCode createEmptyInstance() => DecodeNoQrCode._();
  @$core.pragma('dart2js:noInline')
  static DecodeNoQrCode getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DecodeNoQrCode>(
          DecodeNoQrCode.$_createMessage);
  static DecodeNoQrCode? _defaultInstance;
}

class DecodeSeveralQrCodes extends $pb.GeneratedMessage {
  factory DecodeSeveralQrCodes({
    $core.int? count,
  }) {
    final result = DecodeSeveralQrCodes._();
    if (count != null) result.count = count;
    return result;
  }

  DecodeSeveralQrCodes._();

  factory DecodeSeveralQrCodes.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeSeveralQrCodes()..mergeFromBuffer(data, registry);
  factory DecodeSeveralQrCodes.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeSeveralQrCodes()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeSeveralQrCodes',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeSeveralQrCodes.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'count', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeSeveralQrCodes clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeSeveralQrCodes copyWith(void Function(DecodeSeveralQrCodes) updates) =>
      super.copyWith((message) => updates(message as DecodeSeveralQrCodes))
          as DecodeSeveralQrCodes;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use DecodeSeveralQrCodes() / DecodeSeveralQrCodes.new instead')
  static DecodeSeveralQrCodes create() => DecodeSeveralQrCodes._();
  static $pb.GeneratedMessage $_createMessage() => DecodeSeveralQrCodes._();
  @$core.override
  DecodeSeveralQrCodes createEmptyInstance() => DecodeSeveralQrCodes._();
  @$core.pragma('dart2js:noInline')
  static DecodeSeveralQrCodes getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecodeSeveralQrCodes>(
          DecodeSeveralQrCodes.$_createMessage);
  static DecodeSeveralQrCodes? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get count => $_getIZ(0);
  @$pb.TagNumber(1)
  set count($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCount() => $_has(0);
  @$pb.TagNumber(1)
  void clearCount() => $_clearField(1);
}

class DecodeQrUnreadable extends $pb.GeneratedMessage {
  factory DecodeQrUnreadable({
    $core.String? problem,
  }) {
    final result = DecodeQrUnreadable._();
    if (problem != null) result.problem = problem;
    return result;
  }

  DecodeQrUnreadable._();

  factory DecodeQrUnreadable.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeQrUnreadable()..mergeFromBuffer(data, registry);
  factory DecodeQrUnreadable.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      DecodeQrUnreadable()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DecodeQrUnreadable',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: DecodeQrUnreadable.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeQrUnreadable clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DecodeQrUnreadable copyWith(void Function(DecodeQrUnreadable) updates) =>
      super.copyWith((message) => updates(message as DecodeQrUnreadable))
          as DecodeQrUnreadable;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use DecodeQrUnreadable() / DecodeQrUnreadable.new instead')
  static DecodeQrUnreadable create() => DecodeQrUnreadable._();
  static $pb.GeneratedMessage $_createMessage() => DecodeQrUnreadable._();
  @$core.override
  DecodeQrUnreadable createEmptyInstance() => DecodeQrUnreadable._();
  @$core.pragma('dart2js:noInline')
  static DecodeQrUnreadable getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DecodeQrUnreadable>(
          DecodeQrUnreadable.$_createMessage);
  static DecodeQrUnreadable? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

/// No format recognises the file's header.
class ImportUnknownFormat extends $pb.GeneratedMessage {
  factory ImportUnknownFormat({
    $core.String? stated,
  }) {
    final result = ImportUnknownFormat._();
    if (stated != null) result.stated = stated;
    return result;
  }

  ImportUnknownFormat._();

  factory ImportUnknownFormat.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportUnknownFormat()..mergeFromBuffer(data, registry);
  factory ImportUnknownFormat.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportUnknownFormat()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportUnknownFormat',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportUnknownFormat.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'stated')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportUnknownFormat clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportUnknownFormat copyWith(void Function(ImportUnknownFormat) updates) =>
      super.copyWith((message) => updates(message as ImportUnknownFormat))
          as ImportUnknownFormat;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ImportUnknownFormat() / ImportUnknownFormat.new instead')
  static ImportUnknownFormat create() => ImportUnknownFormat._();
  static $pb.GeneratedMessage $_createMessage() => ImportUnknownFormat._();
  @$core.override
  ImportUnknownFormat createEmptyInstance() => ImportUnknownFormat._();
  @$core.pragma('dart2js:noInline')
  static ImportUnknownFormat getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportUnknownFormat>(
          ImportUnknownFormat.$_createMessage);
  static ImportUnknownFormat? _defaultInstance;

  /// The format the file names, when it names one.
  @$pb.TagNumber(1)
  $core.String get stated => $_getSZ(0);
  @$pb.TagNumber(1)
  set stated($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasStated() => $_has(0);
  @$pb.TagNumber(1)
  void clearStated() => $_clearField(1);
}

/// More than one format recognises the header.
class ImportAmbiguousFormat extends $pb.GeneratedMessage {
  factory ImportAmbiguousFormat({
    $core.Iterable<$core.String>? formats,
  }) {
    final result = ImportAmbiguousFormat._();
    if (formats != null) result.formats.addAll(formats);
    return result;
  }

  ImportAmbiguousFormat._();

  factory ImportAmbiguousFormat.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportAmbiguousFormat()..mergeFromBuffer(data, registry);
  factory ImportAmbiguousFormat.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportAmbiguousFormat()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportAmbiguousFormat',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportAmbiguousFormat.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'formats')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportAmbiguousFormat clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportAmbiguousFormat copyWith(
          void Function(ImportAmbiguousFormat) updates) =>
      super.copyWith((message) => updates(message as ImportAmbiguousFormat))
          as ImportAmbiguousFormat;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportAmbiguousFormat() / ImportAmbiguousFormat.new instead')
  static ImportAmbiguousFormat create() => ImportAmbiguousFormat._();
  static $pb.GeneratedMessage $_createMessage() => ImportAmbiguousFormat._();
  @$core.override
  ImportAmbiguousFormat createEmptyInstance() => ImportAmbiguousFormat._();
  @$core.pragma('dart2js:noInline')
  static ImportAmbiguousFormat getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportAmbiguousFormat>(
          ImportAmbiguousFormat.$_createMessage);
  static ImportAmbiguousFormat? _defaultInstance;

  /// The formats that do, by tag.
  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get formats => $_getList(0);
}

/// The format is known and the file's version of it is not one this build reads.
class ImportUnsupportedVersion extends $pb.GeneratedMessage {
  factory ImportUnsupportedVersion({
    $core.String? format,
    $core.String? version,
  }) {
    final result = ImportUnsupportedVersion._();
    if (format != null) result.format = format;
    if (version != null) result.version = version;
    return result;
  }

  ImportUnsupportedVersion._();

  factory ImportUnsupportedVersion.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportUnsupportedVersion()..mergeFromBuffer(data, registry);
  factory ImportUnsupportedVersion.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportUnsupportedVersion()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportUnsupportedVersion',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportUnsupportedVersion.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'format')
    ..aOS(2, _omitFieldNames ? '' : 'version')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportUnsupportedVersion clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportUnsupportedVersion copyWith(
          void Function(ImportUnsupportedVersion) updates) =>
      super.copyWith((message) => updates(message as ImportUnsupportedVersion))
          as ImportUnsupportedVersion;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportUnsupportedVersion() / ImportUnsupportedVersion.new instead')
  static ImportUnsupportedVersion create() => ImportUnsupportedVersion._();
  static $pb.GeneratedMessage $_createMessage() => ImportUnsupportedVersion._();
  @$core.override
  ImportUnsupportedVersion createEmptyInstance() =>
      ImportUnsupportedVersion._();
  @$core.pragma('dart2js:noInline')
  static ImportUnsupportedVersion getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportUnsupportedVersion>(
          ImportUnsupportedVersion.$_createMessage);
  static ImportUnsupportedVersion? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get format => $_getSZ(0);
  @$pb.TagNumber(1)
  set format($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFormat() => $_has(0);
  @$pb.TagNumber(1)
  void clearFormat() => $_clearField(1);

  /// The version as the file states it.
  @$pb.TagNumber(2)
  $core.String get version => $_getSZ(1);
  @$pb.TagNumber(2)
  set version($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearVersion() => $_clearField(2);
}

/// Not the syntax or shape of any snapshot: not JSON, over the file limit, a file-level field
/// missing or of the wrong kind.
class ImportMalformedSource extends $pb.GeneratedMessage {
  factory ImportMalformedSource({
    $core.String? problem,
  }) {
    final result = ImportMalformedSource._();
    if (problem != null) result.problem = problem;
    return result;
  }

  ImportMalformedSource._();

  factory ImportMalformedSource.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportMalformedSource()..mergeFromBuffer(data, registry);
  factory ImportMalformedSource.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportMalformedSource()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportMalformedSource',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportMalformedSource.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportMalformedSource clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportMalformedSource copyWith(
          void Function(ImportMalformedSource) updates) =>
      super.copyWith((message) => updates(message as ImportMalformedSource))
          as ImportMalformedSource;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportMalformedSource() / ImportMalformedSource.new instead')
  static ImportMalformedSource create() => ImportMalformedSource._();
  static $pb.GeneratedMessage $_createMessage() => ImportMalformedSource._();
  @$core.override
  ImportMalformedSource createEmptyInstance() => ImportMalformedSource._();
  @$core.pragma('dart2js:noInline')
  static ImportMalformedSource getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportMalformedSource>(
          ImportMalformedSource.$_createMessage);
  static ImportMalformedSource? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

/// The file parsed, and what it normalized to breaks the snapshot IR's own rules: a count over its
/// limit, an id twice.
class ImportNormalizationFailed extends $pb.GeneratedMessage {
  factory ImportNormalizationFailed({
    $core.String? problem,
  }) {
    final result = ImportNormalizationFailed._();
    if (problem != null) result.problem = problem;
    return result;
  }

  ImportNormalizationFailed._();

  factory ImportNormalizationFailed.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportNormalizationFailed()..mergeFromBuffer(data, registry);
  factory ImportNormalizationFailed.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportNormalizationFailed()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportNormalizationFailed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportNormalizationFailed.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportNormalizationFailed clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportNormalizationFailed copyWith(
          void Function(ImportNormalizationFailed) updates) =>
      super.copyWith((message) => updates(message as ImportNormalizationFailed))
          as ImportNormalizationFailed;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportNormalizationFailed() / ImportNormalizationFailed.new instead')
  static ImportNormalizationFailed create() => ImportNormalizationFailed._();
  static $pb.GeneratedMessage $_createMessage() =>
      ImportNormalizationFailed._();
  @$core.override
  ImportNormalizationFailed createEmptyInstance() =>
      ImportNormalizationFailed._();
  @$core.pragma('dart2js:noInline')
  static ImportNormalizationFailed getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportNormalizationFailed>(
          ImportNormalizationFailed.$_createMessage);
  static ImportNormalizationFailed? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

/// A file-level value the format does not define.
class ImportUnsupportedSourceValue extends $pb.GeneratedMessage {
  factory ImportUnsupportedSourceValue({
    $core.String? field_1,
    $core.String? value,
  }) {
    final result = ImportUnsupportedSourceValue._();
    if (field_1 != null) result.field_1 = field_1;
    if (value != null) result.value = value;
    return result;
  }

  ImportUnsupportedSourceValue._();

  factory ImportUnsupportedSourceValue.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportUnsupportedSourceValue()..mergeFromBuffer(data, registry);
  factory ImportUnsupportedSourceValue.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportUnsupportedSourceValue()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportUnsupportedSourceValue',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportUnsupportedSourceValue.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'field')
    ..aOS(2, _omitFieldNames ? '' : 'value')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportUnsupportedSourceValue clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportUnsupportedSourceValue copyWith(
          void Function(ImportUnsupportedSourceValue) updates) =>
      super.copyWith(
              (message) => updates(message as ImportUnsupportedSourceValue))
          as ImportUnsupportedSourceValue;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportUnsupportedSourceValue() / ImportUnsupportedSourceValue.new instead')
  static ImportUnsupportedSourceValue create() =>
      ImportUnsupportedSourceValue._();
  static $pb.GeneratedMessage $_createMessage() =>
      ImportUnsupportedSourceValue._();
  @$core.override
  ImportUnsupportedSourceValue createEmptyInstance() =>
      ImportUnsupportedSourceValue._();
  @$core.pragma('dart2js:noInline')
  static ImportUnsupportedSourceValue getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportUnsupportedSourceValue>(
          ImportUnsupportedSourceValue.$_createMessage);
  static ImportUnsupportedSourceValue? _defaultInstance;

  /// The field, as the format names it, and its value.
  @$pb.TagNumber(1)
  $core.String get field_1 => $_getSZ(0);
  @$pb.TagNumber(1)
  set field_1($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasField_1() => $_has(0);
  @$pb.TagNumber(1)
  void clearField_1() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get value => $_getSZ(1);
  @$pb.TagNumber(2)
  set value($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasValue() => $_has(1);
  @$pb.TagNumber(2)
  void clearValue() => $_clearField(2);
}

/// A record names another that the snapshot does not hold.
class ImportInconsistentReference extends $pb.GeneratedMessage {
  factory ImportInconsistentReference({
    $core.String? problem,
  }) {
    final result = ImportInconsistentReference._();
    if (problem != null) result.problem = problem;
    return result;
  }

  ImportInconsistentReference._();

  factory ImportInconsistentReference.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportInconsistentReference()..mergeFromBuffer(data, registry);
  factory ImportInconsistentReference.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportInconsistentReference()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportInconsistentReference',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportInconsistentReference.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportInconsistentReference clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportInconsistentReference copyWith(
          void Function(ImportInconsistentReference) updates) =>
      super.copyWith(
              (message) => updates(message as ImportInconsistentReference))
          as ImportInconsistentReference;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportInconsistentReference() / ImportInconsistentReference.new instead')
  static ImportInconsistentReference create() =>
      ImportInconsistentReference._();
  static $pb.GeneratedMessage $_createMessage() =>
      ImportInconsistentReference._();
  @$core.override
  ImportInconsistentReference createEmptyInstance() =>
      ImportInconsistentReference._();
  @$core.pragma('dart2js:noInline')
  static ImportInconsistentReference getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportInconsistentReference>(
          ImportInconsistentReference.$_createMessage);
  static ImportInconsistentReference? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

/// The domain refused what the snapshot holds.
class ImportAdmissionRefused extends $pb.GeneratedMessage {
  factory ImportAdmissionRefused({
    $core.String? problem,
  }) {
    final result = ImportAdmissionRefused._();
    if (problem != null) result.problem = problem;
    return result;
  }

  ImportAdmissionRefused._();

  factory ImportAdmissionRefused.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportAdmissionRefused()..mergeFromBuffer(data, registry);
  factory ImportAdmissionRefused.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportAdmissionRefused()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportAdmissionRefused',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportAdmissionRefused.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportAdmissionRefused clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportAdmissionRefused copyWith(
          void Function(ImportAdmissionRefused) updates) =>
      super.copyWith((message) => updates(message as ImportAdmissionRefused))
          as ImportAdmissionRefused;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportAdmissionRefused() / ImportAdmissionRefused.new instead')
  static ImportAdmissionRefused create() => ImportAdmissionRefused._();
  static $pb.GeneratedMessage $_createMessage() => ImportAdmissionRefused._();
  @$core.override
  ImportAdmissionRefused createEmptyInstance() => ImportAdmissionRefused._();
  @$core.pragma('dart2js:noInline')
  static ImportAdmissionRefused getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportAdmissionRefused>(
          ImportAdmissionRefused.$_createMessage);
  static ImportAdmissionRefused? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

/// The file states an account other than the one the profile is bound to. The accounts are
/// account data, so they are in `problem` alone, never in fields of their own.
class ImportAccountMismatch extends $pb.GeneratedMessage {
  factory ImportAccountMismatch({
    $core.String? problem,
  }) {
    final result = ImportAccountMismatch._();
    if (problem != null) result.problem = problem;
    return result;
  }

  ImportAccountMismatch._();

  factory ImportAccountMismatch.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportAccountMismatch()..mergeFromBuffer(data, registry);
  factory ImportAccountMismatch.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ImportAccountMismatch()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ImportAccountMismatch',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ImportAccountMismatch.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportAccountMismatch clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ImportAccountMismatch copyWith(
          void Function(ImportAccountMismatch) updates) =>
      super.copyWith((message) => updates(message as ImportAccountMismatch))
          as ImportAccountMismatch;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ImportAccountMismatch() / ImportAccountMismatch.new instead')
  static ImportAccountMismatch create() => ImportAccountMismatch._();
  static $pb.GeneratedMessage $_createMessage() => ImportAccountMismatch._();
  @$core.override
  ImportAccountMismatch createEmptyInstance() => ImportAccountMismatch._();
  @$core.pragma('dart2js:noInline')
  static ImportAccountMismatch getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ImportAccountMismatch>(
          ImportAccountMismatch.$_createMessage);
  static ImportAccountMismatch? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class CommandRefused extends $pb.GeneratedMessage {
  factory CommandRefused({
    $core.String? problem,
  }) {
    final result = CommandRefused._();
    if (problem != null) result.problem = problem;
    return result;
  }

  CommandRefused._();

  factory CommandRefused.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CommandRefused()..mergeFromBuffer(data, registry);
  factory CommandRefused.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CommandRefused()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CommandRefused',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: CommandRefused.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CommandRefused clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CommandRefused copyWith(void Function(CommandRefused) updates) =>
      super.copyWith((message) => updates(message as CommandRefused))
          as CommandRefused;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use CommandRefused() / CommandRefused.new instead')
  static CommandRefused create() => CommandRefused._();
  static $pb.GeneratedMessage $_createMessage() => CommandRefused._();
  @$core.override
  CommandRefused createEmptyInstance() => CommandRefused._();
  @$core.pragma('dart2js:noInline')
  static CommandRefused getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CommandRefused>(
          CommandRefused.$_createMessage);
  static CommandRefused? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class CommandTooLarge extends $pb.GeneratedMessage {
  factory CommandTooLarge({
    $core.String? problem,
  }) {
    final result = CommandTooLarge._();
    if (problem != null) result.problem = problem;
    return result;
  }

  CommandTooLarge._();

  factory CommandTooLarge.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CommandTooLarge()..mergeFromBuffer(data, registry);
  factory CommandTooLarge.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      CommandTooLarge()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CommandTooLarge',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: CommandTooLarge.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CommandTooLarge clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CommandTooLarge copyWith(void Function(CommandTooLarge) updates) =>
      super.copyWith((message) => updates(message as CommandTooLarge))
          as CommandTooLarge;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use CommandTooLarge() / CommandTooLarge.new instead')
  static CommandTooLarge create() => CommandTooLarge._();
  static $pb.GeneratedMessage $_createMessage() => CommandTooLarge._();
  @$core.override
  CommandTooLarge createEmptyInstance() => CommandTooLarge._();
  @$core.pragma('dart2js:noInline')
  static CommandTooLarge getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CommandTooLarge>(
          CommandTooLarge.$_createMessage);
  static CommandTooLarge? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class StoreFailure extends $pb.GeneratedMessage {
  factory StoreFailure({
    $core.String? problem,
  }) {
    final result = StoreFailure._();
    if (problem != null) result.problem = problem;
    return result;
  }

  StoreFailure._();

  factory StoreFailure.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreFailure()..mergeFromBuffer(data, registry);
  factory StoreFailure.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreFailure()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreFailure',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreFailure.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreFailure clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreFailure copyWith(void Function(StoreFailure) updates) =>
      super.copyWith((message) => updates(message as StoreFailure))
          as StoreFailure;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreFailure() / StoreFailure.new instead')
  static StoreFailure create() => StoreFailure._();
  static $pb.GeneratedMessage $_createMessage() => StoreFailure._();
  @$core.override
  StoreFailure createEmptyInstance() => StoreFailure._();
  @$core.pragma('dart2js:noInline')
  static StoreFailure getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoreFailure>(
          StoreFailure.$_createMessage);
  static StoreFailure? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class StoreInvalidLog extends $pb.GeneratedMessage {
  factory StoreInvalidLog({
    $core.String? problem,
  }) {
    final result = StoreInvalidLog._();
    if (problem != null) result.problem = problem;
    return result;
  }

  StoreInvalidLog._();

  factory StoreInvalidLog.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreInvalidLog()..mergeFromBuffer(data, registry);
  factory StoreInvalidLog.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreInvalidLog()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreInvalidLog',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreInvalidLog.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreInvalidLog clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreInvalidLog copyWith(void Function(StoreInvalidLog) updates) =>
      super.copyWith((message) => updates(message as StoreInvalidLog))
          as StoreInvalidLog;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreInvalidLog() / StoreInvalidLog.new instead')
  static StoreInvalidLog create() => StoreInvalidLog._();
  static $pb.GeneratedMessage $_createMessage() => StoreInvalidLog._();
  @$core.override
  StoreInvalidLog createEmptyInstance() => StoreInvalidLog._();
  @$core.pragma('dart2js:noInline')
  static StoreInvalidLog getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoreInvalidLog>(
          StoreInvalidLog.$_createMessage);
  static StoreInvalidLog? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class StoreNewerFormat extends $pb.GeneratedMessage {
  factory StoreNewerFormat({
    $core.String? problem,
  }) {
    final result = StoreNewerFormat._();
    if (problem != null) result.problem = problem;
    return result;
  }

  StoreNewerFormat._();

  factory StoreNewerFormat.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreNewerFormat()..mergeFromBuffer(data, registry);
  factory StoreNewerFormat.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreNewerFormat()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreNewerFormat',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreNewerFormat.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreNewerFormat clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreNewerFormat copyWith(void Function(StoreNewerFormat) updates) =>
      super.copyWith((message) => updates(message as StoreNewerFormat))
          as StoreNewerFormat;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreNewerFormat() / StoreNewerFormat.new instead')
  static StoreNewerFormat create() => StoreNewerFormat._();
  static $pb.GeneratedMessage $_createMessage() => StoreNewerFormat._();
  @$core.override
  StoreNewerFormat createEmptyInstance() => StoreNewerFormat._();
  @$core.pragma('dart2js:noInline')
  static StoreNewerFormat getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoreNewerFormat>(
          StoreNewerFormat.$_createMessage);
  static StoreNewerFormat? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class StoreMalformedCommit extends $pb.GeneratedMessage {
  factory StoreMalformedCommit({
    $core.String? problem,
  }) {
    final result = StoreMalformedCommit._();
    if (problem != null) result.problem = problem;
    return result;
  }

  StoreMalformedCommit._();

  factory StoreMalformedCommit.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreMalformedCommit()..mergeFromBuffer(data, registry);
  factory StoreMalformedCommit.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreMalformedCommit()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreMalformedCommit',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreMalformedCommit.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreMalformedCommit clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreMalformedCommit copyWith(void Function(StoreMalformedCommit) updates) =>
      super.copyWith((message) => updates(message as StoreMalformedCommit))
          as StoreMalformedCommit;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use StoreMalformedCommit() / StoreMalformedCommit.new instead')
  static StoreMalformedCommit create() => StoreMalformedCommit._();
  static $pb.GeneratedMessage $_createMessage() => StoreMalformedCommit._();
  @$core.override
  StoreMalformedCommit createEmptyInstance() => StoreMalformedCommit._();
  @$core.pragma('dart2js:noInline')
  static StoreMalformedCommit getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StoreMalformedCommit>(
          StoreMalformedCommit.$_createMessage);
  static StoreMalformedCommit? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class StoreMissing extends $pb.GeneratedMessage {
  factory StoreMissing({
    $core.String? path,
  }) {
    final result = StoreMissing._();
    if (path != null) result.path = path;
    return result;
  }

  StoreMissing._();

  factory StoreMissing.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreMissing()..mergeFromBuffer(data, registry);
  factory StoreMissing.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreMissing()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreMissing',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreMissing.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreMissing clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreMissing copyWith(void Function(StoreMissing) updates) =>
      super.copyWith((message) => updates(message as StoreMissing))
          as StoreMissing;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreMissing() / StoreMissing.new instead')
  static StoreMissing create() => StoreMissing._();
  static $pb.GeneratedMessage $_createMessage() => StoreMissing._();
  @$core.override
  StoreMissing createEmptyInstance() => StoreMissing._();
  @$core.pragma('dart2js:noInline')
  static StoreMissing getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoreMissing>(
          StoreMissing.$_createMessage);
  static StoreMissing? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => $_clearField(1);
}

class StoreNotADatabase extends $pb.GeneratedMessage {
  factory StoreNotADatabase() => StoreNotADatabase._();

  StoreNotADatabase._();

  factory StoreNotADatabase.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreNotADatabase()..mergeFromBuffer(data, registry);
  factory StoreNotADatabase.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreNotADatabase()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreNotADatabase',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreNotADatabase.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreNotADatabase clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreNotADatabase copyWith(void Function(StoreNotADatabase) updates) =>
      super.copyWith((message) => updates(message as StoreNotADatabase))
          as StoreNotADatabase;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreNotADatabase() / StoreNotADatabase.new instead')
  static StoreNotADatabase create() => StoreNotADatabase._();
  static $pb.GeneratedMessage $_createMessage() => StoreNotADatabase._();
  @$core.override
  StoreNotADatabase createEmptyInstance() => StoreNotADatabase._();
  @$core.pragma('dart2js:noInline')
  static StoreNotADatabase getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoreNotADatabase>(
          StoreNotADatabase.$_createMessage);
  static StoreNotADatabase? _defaultInstance;
}

class StoreForeign extends $pb.GeneratedMessage {
  factory StoreForeign({
    $fixnum.Int64? applicationId,
    $fixnum.Int64? objects,
  }) {
    final result = StoreForeign._();
    if (applicationId != null) result.applicationId = applicationId;
    if (objects != null) result.objects = objects;
    return result;
  }

  StoreForeign._();

  factory StoreForeign.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreForeign()..mergeFromBuffer(data, registry);
  factory StoreForeign.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreForeign()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreForeign',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreForeign.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'applicationId')
    ..aInt64(2, _omitFieldNames ? '' : 'objects')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreForeign clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreForeign copyWith(void Function(StoreForeign) updates) =>
      super.copyWith((message) => updates(message as StoreForeign))
          as StoreForeign;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreForeign() / StoreForeign.new instead')
  static StoreForeign create() => StoreForeign._();
  static $pb.GeneratedMessage $_createMessage() => StoreForeign._();
  @$core.override
  StoreForeign createEmptyInstance() => StoreForeign._();
  @$core.pragma('dart2js:noInline')
  static StoreForeign getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoreForeign>(
          StoreForeign.$_createMessage);
  static StoreForeign? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get applicationId => $_getI64(0);
  @$pb.TagNumber(1)
  set applicationId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasApplicationId() => $_has(0);
  @$pb.TagNumber(1)
  void clearApplicationId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get objects => $_getI64(1);
  @$pb.TagNumber(2)
  set objects($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasObjects() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjects() => $_clearField(2);
}

class StoreNoFormatVersion extends $pb.GeneratedMessage {
  factory StoreNoFormatVersion() => StoreNoFormatVersion._();

  StoreNoFormatVersion._();

  factory StoreNoFormatVersion.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreNoFormatVersion()..mergeFromBuffer(data, registry);
  factory StoreNoFormatVersion.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreNoFormatVersion()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreNoFormatVersion',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreNoFormatVersion.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreNoFormatVersion clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreNoFormatVersion copyWith(void Function(StoreNoFormatVersion) updates) =>
      super.copyWith((message) => updates(message as StoreNoFormatVersion))
          as StoreNoFormatVersion;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use StoreNoFormatVersion() / StoreNoFormatVersion.new instead')
  static StoreNoFormatVersion create() => StoreNoFormatVersion._();
  static $pb.GeneratedMessage $_createMessage() => StoreNoFormatVersion._();
  @$core.override
  StoreNoFormatVersion createEmptyInstance() => StoreNoFormatVersion._();
  @$core.pragma('dart2js:noInline')
  static StoreNoFormatVersion getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StoreNoFormatVersion>(
          StoreNoFormatVersion.$_createMessage);
  static StoreNoFormatVersion? _defaultInstance;
}

class StoreDamaged extends $pb.GeneratedMessage {
  factory StoreDamaged({
    $core.Iterable<$core.String>? problems,
  }) {
    final result = StoreDamaged._();
    if (problems != null) result.problems.addAll(problems);
    return result;
  }

  StoreDamaged._();

  factory StoreDamaged.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreDamaged()..mergeFromBuffer(data, registry);
  factory StoreDamaged.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreDamaged()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreDamaged',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreDamaged.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'problems')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreDamaged clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreDamaged copyWith(void Function(StoreDamaged) updates) =>
      super.copyWith((message) => updates(message as StoreDamaged))
          as StoreDamaged;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreDamaged() / StoreDamaged.new instead')
  static StoreDamaged create() => StoreDamaged._();
  static $pb.GeneratedMessage $_createMessage() => StoreDamaged._();
  @$core.override
  StoreDamaged createEmptyInstance() => StoreDamaged._();
  @$core.pragma('dart2js:noInline')
  static StoreDamaged getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StoreDamaged>(
          StoreDamaged.$_createMessage);
  static StoreDamaged? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get problems => $_getList(0);
}

class StoreUninitialized extends $pb.GeneratedMessage {
  factory StoreUninitialized() => StoreUninitialized._();

  StoreUninitialized._();

  factory StoreUninitialized.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreUninitialized()..mergeFromBuffer(data, registry);
  factory StoreUninitialized.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreUninitialized()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreUninitialized',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreUninitialized.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreUninitialized clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreUninitialized copyWith(void Function(StoreUninitialized) updates) =>
      super.copyWith((message) => updates(message as StoreUninitialized))
          as StoreUninitialized;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreUninitialized() / StoreUninitialized.new instead')
  static StoreUninitialized create() => StoreUninitialized._();
  static $pb.GeneratedMessage $_createMessage() => StoreUninitialized._();
  @$core.override
  StoreUninitialized createEmptyInstance() => StoreUninitialized._();
  @$core.pragma('dart2js:noInline')
  static StoreUninitialized getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StoreUninitialized>(
          StoreUninitialized.$_createMessage);
  static StoreUninitialized? _defaultInstance;
}

/// The store is in a format no build reads any more (ADR-0032, rule 3). Nothing was written to it.
class StoreRetiredFormat extends $pb.GeneratedMessage {
  factory StoreRetiredFormat({
    $core.int? found,
  }) {
    final result = StoreRetiredFormat._();
    if (found != null) result.found = found;
    return result;
  }

  StoreRetiredFormat._();

  factory StoreRetiredFormat.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreRetiredFormat()..mergeFromBuffer(data, registry);
  factory StoreRetiredFormat.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      StoreRetiredFormat()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StoreRetiredFormat',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: StoreRetiredFormat.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'found', fieldType: $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreRetiredFormat clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StoreRetiredFormat copyWith(void Function(StoreRetiredFormat) updates) =>
      super.copyWith((message) => updates(message as StoreRetiredFormat))
          as StoreRetiredFormat;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use StoreRetiredFormat() / StoreRetiredFormat.new instead')
  static StoreRetiredFormat create() => StoreRetiredFormat._();
  static $pb.GeneratedMessage $_createMessage() => StoreRetiredFormat._();
  @$core.override
  StoreRetiredFormat createEmptyInstance() => StoreRetiredFormat._();
  @$core.pragma('dart2js:noInline')
  static StoreRetiredFormat getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StoreRetiredFormat>(
          StoreRetiredFormat.$_createMessage);
  static StoreRetiredFormat? _defaultInstance;

  /// The store's format version.
  @$pb.TagNumber(1)
  $core.int get found => $_getIZ(0);
  @$pb.TagNumber(1)
  set found($core.int value) => $_setUnsignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasFound() => $_has(0);
  @$pb.TagNumber(1)
  void clearFound() => $_clearField(1);
}

class InternalPanic extends $pb.GeneratedMessage {
  factory InternalPanic() => InternalPanic._();

  InternalPanic._();

  factory InternalPanic.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalPanic()..mergeFromBuffer(data, registry);
  factory InternalPanic.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalPanic()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternalPanic',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InternalPanic.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalPanic clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalPanic copyWith(void Function(InternalPanic) updates) =>
      super.copyWith((message) => updates(message as InternalPanic))
          as InternalPanic;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use InternalPanic() / InternalPanic.new instead')
  static InternalPanic create() => InternalPanic._();
  static $pb.GeneratedMessage $_createMessage() => InternalPanic._();
  @$core.override
  InternalPanic createEmptyInstance() => InternalPanic._();
  @$core.pragma('dart2js:noInline')
  static InternalPanic getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternalPanic>(
          InternalPanic.$_createMessage);
  static InternalPanic? _defaultInstance;
}

class InternalQrTooLong extends $pb.GeneratedMessage {
  factory InternalQrTooLong({
    $fixnum.Int64? bits,
    $fixnum.Int64? capacityBits,
  }) {
    final result = InternalQrTooLong._();
    if (bits != null) result.bits = bits;
    if (capacityBits != null) result.capacityBits = capacityBits;
    return result;
  }

  InternalQrTooLong._();

  factory InternalQrTooLong.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalQrTooLong()..mergeFromBuffer(data, registry);
  factory InternalQrTooLong.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalQrTooLong()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternalQrTooLong',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InternalQrTooLong.$_createMessage)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'bits', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'capacityBits', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalQrTooLong clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalQrTooLong copyWith(void Function(InternalQrTooLong) updates) =>
      super.copyWith((message) => updates(message as InternalQrTooLong))
          as InternalQrTooLong;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use InternalQrTooLong() / InternalQrTooLong.new instead')
  static InternalQrTooLong create() => InternalQrTooLong._();
  static $pb.GeneratedMessage $_createMessage() => InternalQrTooLong._();
  @$core.override
  InternalQrTooLong createEmptyInstance() => InternalQrTooLong._();
  @$core.pragma('dart2js:noInline')
  static InternalQrTooLong getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InternalQrTooLong>(
          InternalQrTooLong.$_createMessage);
  static InternalQrTooLong? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get bits => $_getI64(0);
  @$pb.TagNumber(1)
  set bits($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasBits() => $_has(0);
  @$pb.TagNumber(1)
  void clearBits() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get capacityBits => $_getI64(1);
  @$pb.TagNumber(2)
  set capacityBits($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasCapacityBits() => $_has(1);
  @$pb.TagNumber(2)
  void clearCapacityBits() => $_clearField(2);
}

class InternalResponseTooLarge extends $pb.GeneratedMessage {
  factory InternalResponseTooLarge({
    $core.String? problem,
  }) {
    final result = InternalResponseTooLarge._();
    if (problem != null) result.problem = problem;
    return result;
  }

  InternalResponseTooLarge._();

  factory InternalResponseTooLarge.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalResponseTooLarge()..mergeFromBuffer(data, registry);
  factory InternalResponseTooLarge.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalResponseTooLarge()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternalResponseTooLarge',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InternalResponseTooLarge.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalResponseTooLarge clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalResponseTooLarge copyWith(
          void Function(InternalResponseTooLarge) updates) =>
      super.copyWith((message) => updates(message as InternalResponseTooLarge))
          as InternalResponseTooLarge;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use InternalResponseTooLarge() / InternalResponseTooLarge.new instead')
  static InternalResponseTooLarge create() => InternalResponseTooLarge._();
  static $pb.GeneratedMessage $_createMessage() => InternalResponseTooLarge._();
  @$core.override
  InternalResponseTooLarge createEmptyInstance() =>
      InternalResponseTooLarge._();
  @$core.pragma('dart2js:noInline')
  static InternalResponseTooLarge getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InternalResponseTooLarge>(
          InternalResponseTooLarge.$_createMessage);
  static InternalResponseTooLarge? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

/// A page row whose soul is not among the souls the page was selected from: a daemon bug.
class InternalPageWithoutSoul extends $pb.GeneratedMessage {
  factory InternalPageWithoutSoul({
    $core.String? soulId,
  }) {
    final result = InternalPageWithoutSoul._();
    if (soulId != null) result.soulId = soulId;
    return result;
  }

  InternalPageWithoutSoul._();

  factory InternalPageWithoutSoul.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalPageWithoutSoul()..mergeFromBuffer(data, registry);
  factory InternalPageWithoutSoul.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalPageWithoutSoul()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternalPageWithoutSoul',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InternalPageWithoutSoul.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'soulId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalPageWithoutSoul clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalPageWithoutSoul copyWith(
          void Function(InternalPageWithoutSoul) updates) =>
      super.copyWith((message) => updates(message as InternalPageWithoutSoul))
          as InternalPageWithoutSoul;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use InternalPageWithoutSoul() / InternalPageWithoutSoul.new instead')
  static InternalPageWithoutSoul create() => InternalPageWithoutSoul._();
  static $pb.GeneratedMessage $_createMessage() => InternalPageWithoutSoul._();
  @$core.override
  InternalPageWithoutSoul createEmptyInstance() => InternalPageWithoutSoul._();
  @$core.pragma('dart2js:noInline')
  static InternalPageWithoutSoul getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InternalPageWithoutSoul>(
          InternalPageWithoutSoul.$_createMessage);
  static InternalPageWithoutSoul? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get soulId => $_getSZ(0);
  @$pb.TagNumber(1)
  set soulId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSoulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSoulId() => $_clearField(1);
}

/// The daemon paired a snapshot with other original bytes than the file its provenance names: a
/// daemon bug, not a defect of the file.
class InternalImportMismatch extends $pb.GeneratedMessage {
  factory InternalImportMismatch({
    $core.String? problem,
  }) {
    final result = InternalImportMismatch._();
    if (problem != null) result.problem = problem;
    return result;
  }

  InternalImportMismatch._();

  factory InternalImportMismatch.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalImportMismatch()..mergeFromBuffer(data, registry);
  factory InternalImportMismatch.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalImportMismatch()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternalImportMismatch',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InternalImportMismatch.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalImportMismatch clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalImportMismatch copyWith(
          void Function(InternalImportMismatch) updates) =>
      super.copyWith((message) => updates(message as InternalImportMismatch))
          as InternalImportMismatch;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use InternalImportMismatch() / InternalImportMismatch.new instead')
  static InternalImportMismatch create() => InternalImportMismatch._();
  static $pb.GeneratedMessage $_createMessage() => InternalImportMismatch._();
  @$core.override
  InternalImportMismatch createEmptyInstance() => InternalImportMismatch._();
  @$core.pragma('dart2js:noInline')
  static InternalImportMismatch getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InternalImportMismatch>(
          InternalImportMismatch.$_createMessage);
  static InternalImportMismatch? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class SessionMalformedFrame extends $pb.GeneratedMessage {
  factory SessionMalformedFrame({
    $core.String? problem,
  }) {
    final result = SessionMalformedFrame._();
    if (problem != null) result.problem = problem;
    return result;
  }

  SessionMalformedFrame._();

  factory SessionMalformedFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionMalformedFrame()..mergeFromBuffer(data, registry);
  factory SessionMalformedFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionMalformedFrame()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionMalformedFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionMalformedFrame.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionMalformedFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionMalformedFrame copyWith(
          void Function(SessionMalformedFrame) updates) =>
      super.copyWith((message) => updates(message as SessionMalformedFrame))
          as SessionMalformedFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SessionMalformedFrame() / SessionMalformedFrame.new instead')
  static SessionMalformedFrame create() => SessionMalformedFrame._();
  static $pb.GeneratedMessage $_createMessage() => SessionMalformedFrame._();
  @$core.override
  SessionMalformedFrame createEmptyInstance() => SessionMalformedFrame._();
  @$core.pragma('dart2js:noInline')
  static SessionMalformedFrame getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionMalformedFrame>(
          SessionMalformedFrame.$_createMessage);
  static SessionMalformedFrame? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class SessionMalformedMessage extends $pb.GeneratedMessage {
  factory SessionMalformedMessage({
    $core.String? problem,
  }) {
    final result = SessionMalformedMessage._();
    if (problem != null) result.problem = problem;
    return result;
  }

  SessionMalformedMessage._();

  factory SessionMalformedMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionMalformedMessage()..mergeFromBuffer(data, registry);
  factory SessionMalformedMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SessionMalformedMessage()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionMalformedMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SessionMalformedMessage.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionMalformedMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SessionMalformedMessage copyWith(
          void Function(SessionMalformedMessage) updates) =>
      super.copyWith((message) => updates(message as SessionMalformedMessage))
          as SessionMalformedMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use SessionMalformedMessage() / SessionMalformedMessage.new instead')
  static SessionMalformedMessage create() => SessionMalformedMessage._();
  static $pb.GeneratedMessage $_createMessage() => SessionMalformedMessage._();
  @$core.override
  SessionMalformedMessage createEmptyInstance() => SessionMalformedMessage._();
  @$core.pragma('dart2js:noInline')
  static SessionMalformedMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionMalformedMessage>(
          SessionMalformedMessage.$_createMessage);
  static SessionMalformedMessage? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class InternalIo extends $pb.GeneratedMessage {
  factory InternalIo({
    $core.String? problem,
  }) {
    final result = InternalIo._();
    if (problem != null) result.problem = problem;
    return result;
  }

  InternalIo._();

  factory InternalIo.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalIo()..mergeFromBuffer(data, registry);
  factory InternalIo.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InternalIo()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InternalIo',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InternalIo.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalIo clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  InternalIo copyWith(void Function(InternalIo) updates) =>
      super.copyWith((message) => updates(message as InternalIo)) as InternalIo;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use InternalIo() / InternalIo.new instead')
  static InternalIo create() => InternalIo._();
  static $pb.GeneratedMessage $_createMessage() => InternalIo._();
  @$core.override
  InternalIo createEmptyInstance() => InternalIo._();
  @$core.pragma('dart2js:noInline')
  static InternalIo getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InternalIo>(InternalIo.$_createMessage);
  static InternalIo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class ClientDaemonNotFound extends $pb.GeneratedMessage {
  factory ClientDaemonNotFound({
    $core.Iterable<$core.String>? searched,
  }) {
    final result = ClientDaemonNotFound._();
    if (searched != null) result.searched.addAll(searched);
    return result;
  }

  ClientDaemonNotFound._();

  factory ClientDaemonNotFound.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientDaemonNotFound()..mergeFromBuffer(data, registry);
  factory ClientDaemonNotFound.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientDaemonNotFound()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientDaemonNotFound',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientDaemonNotFound.$_createMessage)
    ..pPS(1, _omitFieldNames ? '' : 'searched')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientDaemonNotFound clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientDaemonNotFound copyWith(void Function(ClientDaemonNotFound) updates) =>
      super.copyWith((message) => updates(message as ClientDaemonNotFound))
          as ClientDaemonNotFound;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ClientDaemonNotFound() / ClientDaemonNotFound.new instead')
  static ClientDaemonNotFound create() => ClientDaemonNotFound._();
  static $pb.GeneratedMessage $_createMessage() => ClientDaemonNotFound._();
  @$core.override
  ClientDaemonNotFound createEmptyInstance() => ClientDaemonNotFound._();
  @$core.pragma('dart2js:noInline')
  static ClientDaemonNotFound getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ClientDaemonNotFound>(
          ClientDaemonNotFound.$_createMessage);
  static ClientDaemonNotFound? _defaultInstance;

  /// Every path tried, in order.
  @$pb.TagNumber(1)
  $pb.PbList<$core.String> get searched => $_getList(0);
}

class ClientDaemonStartFailed extends $pb.GeneratedMessage {
  factory ClientDaemonStartFailed({
    $core.String? path,
    $core.String? problem,
  }) {
    final result = ClientDaemonStartFailed._();
    if (path != null) result.path = path;
    if (problem != null) result.problem = problem;
    return result;
  }

  ClientDaemonStartFailed._();

  factory ClientDaemonStartFailed.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientDaemonStartFailed()..mergeFromBuffer(data, registry);
  factory ClientDaemonStartFailed.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientDaemonStartFailed()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientDaemonStartFailed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientDaemonStartFailed.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..aOS(2, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientDaemonStartFailed clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientDaemonStartFailed copyWith(
          void Function(ClientDaemonStartFailed) updates) =>
      super.copyWith((message) => updates(message as ClientDaemonStartFailed))
          as ClientDaemonStartFailed;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated(
      'Use ClientDaemonStartFailed() / ClientDaemonStartFailed.new instead')
  static ClientDaemonStartFailed create() => ClientDaemonStartFailed._();
  static $pb.GeneratedMessage $_createMessage() => ClientDaemonStartFailed._();
  @$core.override
  ClientDaemonStartFailed createEmptyInstance() => ClientDaemonStartFailed._();
  @$core.pragma('dart2js:noInline')
  static ClientDaemonStartFailed getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ClientDaemonStartFailed>(
          ClientDaemonStartFailed.$_createMessage);
  static ClientDaemonStartFailed? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get problem => $_getSZ(1);
  @$pb.TagNumber(2)
  set problem($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasProblem() => $_has(1);
  @$pb.TagNumber(2)
  void clearProblem() => $_clearField(2);
}

class ClientDaemonExited extends $pb.GeneratedMessage {
  factory ClientDaemonExited({
    $core.int? exitCode,
    $core.String? lastLogLine,
  }) {
    final result = ClientDaemonExited._();
    if (exitCode != null) result.exitCode = exitCode;
    if (lastLogLine != null) result.lastLogLine = lastLogLine;
    return result;
  }

  ClientDaemonExited._();

  factory ClientDaemonExited.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientDaemonExited()..mergeFromBuffer(data, registry);
  factory ClientDaemonExited.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientDaemonExited()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientDaemonExited',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientDaemonExited.$_createMessage)
    ..aI(1, _omitFieldNames ? '' : 'exitCode')
    ..aOS(2, _omitFieldNames ? '' : 'lastLogLine')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientDaemonExited clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientDaemonExited copyWith(void Function(ClientDaemonExited) updates) =>
      super.copyWith((message) => updates(message as ClientDaemonExited))
          as ClientDaemonExited;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ClientDaemonExited() / ClientDaemonExited.new instead')
  static ClientDaemonExited create() => ClientDaemonExited._();
  static $pb.GeneratedMessage $_createMessage() => ClientDaemonExited._();
  @$core.override
  ClientDaemonExited createEmptyInstance() => ClientDaemonExited._();
  @$core.pragma('dart2js:noInline')
  static ClientDaemonExited getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ClientDaemonExited>(
          ClientDaemonExited.$_createMessage);
  static ClientDaemonExited? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get exitCode => $_getIZ(0);
  @$pb.TagNumber(1)
  set exitCode($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasExitCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearExitCode() => $_clearField(1);

  /// The daemon's last stderr line; absent when it wrote none.
  @$pb.TagNumber(2)
  $core.String get lastLogLine => $_getSZ(1);
  @$pb.TagNumber(2)
  set lastLogLine($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLastLogLine() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastLogLine() => $_clearField(2);
}

class ClientTimeout extends $pb.GeneratedMessage {
  factory ClientTimeout({
    $fixnum.Int64? requestId,
    $fixnum.Int64? limitMs,
  }) {
    final result = ClientTimeout._();
    if (requestId != null) result.requestId = requestId;
    if (limitMs != null) result.limitMs = limitMs;
    return result;
  }

  ClientTimeout._();

  factory ClientTimeout.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientTimeout()..mergeFromBuffer(data, registry);
  factory ClientTimeout.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientTimeout()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientTimeout',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientTimeout.$_createMessage)
    ..a<$fixnum.Int64>(
        1, _omitFieldNames ? '' : 'requestId', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'limitMs', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientTimeout clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientTimeout copyWith(void Function(ClientTimeout) updates) =>
      super.copyWith((message) => updates(message as ClientTimeout))
          as ClientTimeout;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ClientTimeout() / ClientTimeout.new instead')
  static ClientTimeout create() => ClientTimeout._();
  static $pb.GeneratedMessage $_createMessage() => ClientTimeout._();
  @$core.override
  ClientTimeout createEmptyInstance() => ClientTimeout._();
  @$core.pragma('dart2js:noInline')
  static ClientTimeout getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClientTimeout>(
          ClientTimeout.$_createMessage);
  static ClientTimeout? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get requestId => $_getI64(0);
  @$pb.TagNumber(1)
  set requestId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRequestId() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequestId() => $_clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get limitMs => $_getI64(1);
  @$pb.TagNumber(2)
  set limitMs($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasLimitMs() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimitMs() => $_clearField(2);
}

class ClientProtocolError extends $pb.GeneratedMessage {
  factory ClientProtocolError({
    $core.String? problem,
  }) {
    final result = ClientProtocolError._();
    if (problem != null) result.problem = problem;
    return result;
  }

  ClientProtocolError._();

  factory ClientProtocolError.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientProtocolError()..mergeFromBuffer(data, registry);
  factory ClientProtocolError.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientProtocolError()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientProtocolError',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientProtocolError.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientProtocolError clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientProtocolError copyWith(void Function(ClientProtocolError) updates) =>
      super.copyWith((message) => updates(message as ClientProtocolError))
          as ClientProtocolError;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core
      .Deprecated('Use ClientProtocolError() / ClientProtocolError.new instead')
  static ClientProtocolError create() => ClientProtocolError._();
  static $pb.GeneratedMessage $_createMessage() => ClientProtocolError._();
  @$core.override
  ClientProtocolError createEmptyInstance() => ClientProtocolError._();
  @$core.pragma('dart2js:noInline')
  static ClientProtocolError getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ClientProtocolError>(
          ClientProtocolError.$_createMessage);
  static ClientProtocolError? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

class ClientNotConnected extends $pb.GeneratedMessage {
  factory ClientNotConnected() => ClientNotConnected._();

  ClientNotConnected._();

  factory ClientNotConnected.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientNotConnected()..mergeFromBuffer(data, registry);
  factory ClientNotConnected.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientNotConnected()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientNotConnected',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientNotConnected.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientNotConnected clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientNotConnected copyWith(void Function(ClientNotConnected) updates) =>
      super.copyWith((message) => updates(message as ClientNotConnected))
          as ClientNotConnected;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ClientNotConnected() / ClientNotConnected.new instead')
  static ClientNotConnected create() => ClientNotConnected._();
  static $pb.GeneratedMessage $_createMessage() => ClientNotConnected._();
  @$core.override
  ClientNotConnected createEmptyInstance() => ClientNotConnected._();
  @$core.pragma('dart2js:noInline')
  static ClientNotConnected getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ClientNotConnected>(
          ClientNotConnected.$_createMessage);
  static ClientNotConnected? _defaultInstance;
}

class ClientUnexpected extends $pb.GeneratedMessage {
  factory ClientUnexpected({
    $core.String? problem,
  }) {
    final result = ClientUnexpected._();
    if (problem != null) result.problem = problem;
    return result;
  }

  ClientUnexpected._();

  factory ClientUnexpected.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientUnexpected()..mergeFromBuffer(data, registry);
  factory ClientUnexpected.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientUnexpected()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientUnexpected',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientUnexpected.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'problem')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientUnexpected clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientUnexpected copyWith(void Function(ClientUnexpected) updates) =>
      super.copyWith((message) => updates(message as ClientUnexpected))
          as ClientUnexpected;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ClientUnexpected() / ClientUnexpected.new instead')
  static ClientUnexpected create() => ClientUnexpected._();
  static $pb.GeneratedMessage $_createMessage() => ClientUnexpected._();
  @$core.override
  ClientUnexpected createEmptyInstance() => ClientUnexpected._();
  @$core.pragma('dart2js:noInline')
  static ClientUnexpected getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClientUnexpected>(
          ClientUnexpected.$_createMessage);
  static ClientUnexpected? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get problem => $_getSZ(0);
  @$pb.TagNumber(1)
  set problem($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasProblem() => $_has(0);
  @$pb.TagNumber(1)
  void clearProblem() => $_clearField(1);
}

enum Warning_Kind { clientOutdated, notSet }

/// Surfaced to the user; changes no state and advances no revision. Not a failure: it has no
/// code, only its kind.
class Warning extends $pb.GeneratedMessage {
  factory Warning({
    $core.String? message,
    ClientOutdated? clientOutdated,
  }) {
    final result = Warning._();
    if (message != null) result.message = message;
    if (clientOutdated != null) result.clientOutdated = clientOutdated;
    return result;
  }

  Warning._();

  factory Warning.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Warning()..mergeFromBuffer(data, registry);
  factory Warning.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Warning()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Warning_Kind> _Warning_KindByTag = {
    10: Warning_Kind.clientOutdated,
    0: Warning_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Warning',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Warning.$_createMessage)
    ..oo(0, [10])
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..aOM<ClientOutdated>(10, _omitFieldNames ? '' : 'clientOutdated',
        subBuilder: ClientOutdated.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Warning clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Warning copyWith(void Function(Warning) updates) =>
      super.copyWith((message) => updates(message as Warning)) as Warning;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Warning() / Warning.new instead')
  static Warning create() => Warning._();
  static $pb.GeneratedMessage $_createMessage() => Warning._();
  @$core.override
  Warning createEmptyInstance() => Warning._();
  @$core.pragma('dart2js:noInline')
  static Warning getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Warning>(Warning.$_createMessage);
  static Warning? _defaultInstance;

  @$pb.TagNumber(10)
  Warning_Kind whichKind() => _Warning_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  void clearKind() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(0);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(0, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);

  @$pb.TagNumber(10)
  ClientOutdated get clientOutdated => $_getN(1);
  @$pb.TagNumber(10)
  set clientOutdated(ClientOutdated value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasClientOutdated() => $_has(1);
  @$pb.TagNumber(10)
  void clearClientOutdated() => $_clearField(10);
  @$pb.TagNumber(10)
  ClientOutdated ensureClientOutdated() => $_ensure(1);
}

/// The client was built against an older major than the daemon's; the session opened anyway.
class ClientOutdated extends $pb.GeneratedMessage {
  factory ClientOutdated({
    ProtocolVersion? client,
    ProtocolVersion? daemon,
  }) {
    final result = ClientOutdated._();
    if (client != null) result.client = client;
    if (daemon != null) result.daemon = daemon;
    return result;
  }

  ClientOutdated._();

  factory ClientOutdated.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientOutdated()..mergeFromBuffer(data, registry);
  factory ClientOutdated.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      ClientOutdated()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientOutdated',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientOutdated.$_createMessage)
    ..aOM<ProtocolVersion>(1, _omitFieldNames ? '' : 'client',
        subBuilder: ProtocolVersion.$_createMessage)
    ..aOM<ProtocolVersion>(2, _omitFieldNames ? '' : 'daemon',
        subBuilder: ProtocolVersion.$_createMessage)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientOutdated clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientOutdated copyWith(void Function(ClientOutdated) updates) =>
      super.copyWith((message) => updates(message as ClientOutdated))
          as ClientOutdated;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use ClientOutdated() / ClientOutdated.new instead')
  static ClientOutdated create() => ClientOutdated._();
  static $pb.GeneratedMessage $_createMessage() => ClientOutdated._();
  @$core.override
  ClientOutdated createEmptyInstance() => ClientOutdated._();
  @$core.pragma('dart2js:noInline')
  static ClientOutdated getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClientOutdated>(
          ClientOutdated.$_createMessage);
  static ClientOutdated? _defaultInstance;

  @$pb.TagNumber(1)
  ProtocolVersion get client => $_getN(0);
  @$pb.TagNumber(1)
  set client(ProtocolVersion value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasClient() => $_has(0);
  @$pb.TagNumber(1)
  void clearClient() => $_clearField(1);
  @$pb.TagNumber(1)
  ProtocolVersion ensureClient() => $_ensure(0);

  @$pb.TagNumber(2)
  ProtocolVersion get daemon => $_getN(1);
  @$pb.TagNumber(2)
  set daemon(ProtocolVersion value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasDaemon() => $_has(1);
  @$pb.TagNumber(2)
  void clearDaemon() => $_clearField(2);
  @$pb.TagNumber(2)
  ProtocolVersion ensureDaemon() => $_ensure(1);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
