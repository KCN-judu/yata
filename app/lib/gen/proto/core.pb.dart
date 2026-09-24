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

/// Every failure (core-protocol.md, "Errors"). `code` is the contract; `message` is English and
/// never parsed.
class Error extends $pb.GeneratedMessage {
  factory Error({
    $core.String? code,
    $core.String? message,
    $core.List<$core.int>? details,
  }) {
    final result = Error._();
    if (code != null) result.code = code;
    if (message != null) result.message = message;
    if (details != null) result.details = details;
    return result;
  }

  Error._();

  factory Error.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Error()..mergeFromBuffer(data, registry);
  factory Error.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Error()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Error',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Error.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'code')
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'details', $pb.PbFieldType.OY)
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

  @$pb.TagNumber(1)
  $core.String get code => $_getSZ(0);
  @$pb.TagNumber(1)
  set code($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get details => $_getN(2);
  @$pb.TagNumber(3)
  set details($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasDetails() => $_has(2);
  @$pb.TagNumber(3)
  void clearDetails() => $_clearField(3);
}

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
    Innate? innate,
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
    if (innate != null) result.innate = innate;
    return result;
  }

  Soul._();

  factory Soul.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Soul()..mergeFromBuffer(data, registry);
  factory Soul.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Soul()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Soul',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Soul.$_createMessage)
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
    ..aOM<Innate>(9, _omitFieldNames ? '' : 'innate',
        subBuilder: Innate.$_createMessage)
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

  /// Absent: the input does not say whether the soul carries one, or which.
  @$pb.TagNumber(9)
  Innate get innate => $_getN(8);
  @$pb.TagNumber(9)
  set innate(Innate value) => $_setField(9, value);
  @$pb.TagNumber(9)
  $core.bool hasInnate() => $_has(8);
  @$pb.TagNumber(9)
  void clearInnate() => $_clearField(9);
  @$pb.TagNumber(9)
  Innate ensureInnate() => $_ensure(8);
}

enum Innate_State { absent, present, notSet }

/// A soul's 固有属性 (glossary.md, InnateAttribute).
class Innate extends $pb.GeneratedMessage {
  factory Innate({
    $core.bool? absent,
    SoulAttribute? present,
  }) {
    final result = Innate._();
    if (absent != null) result.absent = absent;
    if (present != null) result.present = present;
    return result;
  }

  Innate._();

  factory Innate.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Innate()..mergeFromBuffer(data, registry);
  factory Innate.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Innate()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, Innate_State> _Innate_StateByTag = {
    1: Innate_State.absent,
    2: Innate_State.present,
    0: Innate_State.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Innate',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Innate.$_createMessage)
    ..oo(0, [1, 2])
    ..aOB(1, _omitFieldNames ? '' : 'absent')
    ..aE<SoulAttribute>(2, _omitFieldNames ? '' : 'present',
        enumValues: SoulAttribute.values)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Innate clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Innate copyWith(void Function(Innate) updates) =>
      super.copyWith((message) => updates(message as Innate)) as Innate;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  @$core.Deprecated('Use Innate() / Innate.new instead')
  static Innate create() => Innate._();
  static $pb.GeneratedMessage $_createMessage() => Innate._();
  @$core.override
  Innate createEmptyInstance() => Innate._();
  @$core.pragma('dart2js:noInline')
  static Innate getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Innate>(Innate.$_createMessage);
  static Innate? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  Innate_State whichState() => _Innate_StateByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearState() => $_clearField($_whichOneof(0));

  /// The soul carries none.
  @$pb.TagNumber(1)
  $core.bool get absent => $_getBF(0);
  @$pb.TagNumber(1)
  set absent($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAbsent() => $_has(0);
  @$pb.TagNumber(1)
  void clearAbsent() => $_clearField(1);

  @$pb.TagNumber(2)
  SoulAttribute get present => $_getN(1);
  @$pb.TagNumber(2)
  set present(SoulAttribute value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasPresent() => $_has(1);
  @$pb.TagNumber(2)
  void clearPresent() => $_clearField(2);
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

/// A SoulSelection: each group a set of choices, an empty group no constraint. Each repeated
/// field holds at most 256 values.
class SoulSelection extends $pb.GeneratedMessage {
  factory SoulSelection({
    $core.bool? anySet,
    $core.Iterable<$core.int>? suitCodes,
    $core.Iterable<SoulSlot>? slots,
    $core.Iterable<$core.int>? stars,
    $core.Iterable<LevelBand>? levels,
    $core.Iterable<SoulAttribute>? mainAttributes,
    $core.Iterable<SoulAttribute>? innate,
    $core.Iterable<SoulAttribute>? subIncluded,
    $core.Iterable<SoulAttribute>? subExcluded,
    $core.Iterable<SubCount>? subCounts,
  }) {
    final result = SoulSelection._();
    if (anySet != null) result.anySet = anySet;
    if (suitCodes != null) result.suitCodes.addAll(suitCodes);
    if (slots != null) result.slots.addAll(slots);
    if (stars != null) result.stars.addAll(stars);
    if (levels != null) result.levels.addAll(levels);
    if (mainAttributes != null) result.mainAttributes.addAll(mainAttributes);
    if (innate != null) result.innate.addAll(innate);
    if (subIncluded != null) result.subIncluded.addAll(subIncluded);
    if (subExcluded != null) result.subExcluded.addAll(subExcluded);
    if (subCounts != null) result.subCounts.addAll(subCounts);
    return result;
  }

  SoulSelection._();

  factory SoulSelection.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SoulSelection()..mergeFromBuffer(data, registry);
  factory SoulSelection.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      SoulSelection()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SoulSelection',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: SoulSelection.$_createMessage)
    ..aOB(1, _omitFieldNames ? '' : 'anySet')
    ..p<$core.int>(2, _omitFieldNames ? '' : 'suitCodes', $pb.PbFieldType.KU3)
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
    ..pc<SoulAttribute>(
        8, _omitFieldNames ? '' : 'subIncluded', $pb.PbFieldType.KE,
        valueOf: SoulAttribute.valueOf,
        enumValues: SoulAttribute.values,
        defaultEnumValue: SoulAttribute.SOUL_ATTRIBUTE_UNSPECIFIED)
    ..pc<SoulAttribute>(
        9, _omitFieldNames ? '' : 'subExcluded', $pb.PbFieldType.KE,
        valueOf: SoulAttribute.valueOf,
        enumValues: SoulAttribute.values,
        defaultEnumValue: SoulAttribute.SOUL_ATTRIBUTE_UNSPECIFIED)
    ..pc<SubCount>(10, _omitFieldNames ? '' : 'subCounts', $pb.PbFieldType.KE,
        valueOf: SubCount.valueOf,
        enumValues: SubCount.values,
        defaultEnumValue: SubCount.SUB_COUNT_UNSPECIFIED)
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

  /// 全部: every set. `suit_codes` is then empty.
  @$pb.TagNumber(1)
  $core.bool get anySet => $_getBF(0);
  @$pb.TagNumber(1)
  set anySet($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasAnySet() => $_has(0);
  @$pb.TagNumber(1)
  void clearAnySet() => $_clearField(1);

  @$pb.TagNumber(2)
  $pb.PbList<$core.int> get suitCodes => $_getList(1);

  @$pb.TagNumber(3)
  $pb.PbList<SoulSlot> get slots => $_getList(2);

  @$pb.TagNumber(4)
  $pb.PbList<$core.int> get stars => $_getList(3);

  @$pb.TagNumber(5)
  $pb.PbList<LevelBand> get levels => $_getList(4);

  @$pb.TagNumber(6)
  $pb.PbList<SoulAttribute> get mainAttributes => $_getList(5);

  /// One of the six innate attributes.
  @$pb.TagNumber(7)
  $pb.PbList<SoulAttribute> get innate => $_getList(6);

  /// ○ and ✕; an attribute is in at most one.
  @$pb.TagNumber(8)
  $pb.PbList<SoulAttribute> get subIncluded => $_getList(7);

  @$pb.TagNumber(9)
  $pb.PbList<SoulAttribute> get subExcluded => $_getList(8);

  @$pb.TagNumber(10)
  $pb.PbList<SubCount> get subCounts => $_getList(9);
}

class Field extends $pb.GeneratedMessage {
  factory Field({
    FieldName? name,
    SoulAttribute? attribute,
  }) {
    final result = Field._();
    if (name != null) result.name = name;
    if (attribute != null) result.attribute = attribute;
    return result;
  }

  Field._();

  factory Field.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Field()..mergeFromBuffer(data, registry);
  factory Field.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Field()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Field',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Field.$_createMessage)
    ..aE<FieldName>(1, _omitFieldNames ? '' : 'name',
        enumValues: FieldName.values)
    ..aE<SoulAttribute>(2, _omitFieldNames ? '' : 'attribute',
        enumValues: SoulAttribute.values)
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

  @$pb.TagNumber(1)
  FieldName get name => $_getN(0);
  @$pb.TagNumber(1)
  set name(FieldName value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// For SUB_VALUE and HAS_SUB; unspecified for every other field.
  @$pb.TagNumber(2)
  SoulAttribute get attribute => $_getN(1);
  @$pb.TagNumber(2)
  set attribute(SoulAttribute value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasAttribute() => $_has(1);
  @$pb.TagNumber(2)
  void clearAttribute() => $_clearField(2);
}

/// The values of an `In`, in the one list that fits the field's type. At most 256.
class InTest extends $pb.GeneratedMessage {
  factory InTest({
    $core.Iterable<$core.int>? suitCodes,
    $core.Iterable<SoulSlot>? slots,
    $core.Iterable<SoulAttribute>? attributes,
  }) {
    final result = InTest._();
    if (suitCodes != null) result.suitCodes.addAll(suitCodes);
    if (slots != null) result.slots.addAll(slots);
    if (attributes != null) result.attributes.addAll(attributes);
    return result;
  }

  InTest._();

  factory InTest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InTest()..mergeFromBuffer(data, registry);
  factory InTest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      InTest()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InTest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: InTest.$_createMessage)
    ..p<$core.int>(1, _omitFieldNames ? '' : 'suitCodes', $pb.PbFieldType.KU3)
    ..pc<SoulSlot>(2, _omitFieldNames ? '' : 'slots', $pb.PbFieldType.KE,
        valueOf: SoulSlot.valueOf,
        enumValues: SoulSlot.values,
        defaultEnumValue: SoulSlot.SOUL_SLOT_UNSPECIFIED)
    ..pc<SoulAttribute>(
        3, _omitFieldNames ? '' : 'attributes', $pb.PbFieldType.KE,
        valueOf: SoulAttribute.valueOf,
        enumValues: SoulAttribute.values,
        defaultEnumValue: SoulAttribute.SOUL_ATTRIBUTE_UNSPECIFIED)
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

  @$pb.TagNumber(1)
  $pb.PbList<$core.int> get suitCodes => $_getList(0);

  @$pb.TagNumber(2)
  $pb.PbList<SoulSlot> get slots => $_getList(1);

  @$pb.TagNumber(3)
  $pb.PbList<SoulAttribute> get attributes => $_getList(2);
}

/// Both bounds inclusive; at least one present.
class IntRange extends $pb.GeneratedMessage {
  factory IntRange({
    $fixnum.Int64? min,
    $fixnum.Int64? max,
  }) {
    final result = IntRange._();
    if (min != null) result.min = min;
    if (max != null) result.max = max;
    return result;
  }

  IntRange._();

  factory IntRange.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IntRange()..mergeFromBuffer(data, registry);
  factory IntRange.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      IntRange()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'IntRange',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: IntRange.$_createMessage)
    ..aInt64(1, _omitFieldNames ? '' : 'min')
    ..aInt64(2, _omitFieldNames ? '' : 'max')
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

/// Both bounds inclusive and finite; at least one present.
class NumberRange extends $pb.GeneratedMessage {
  factory NumberRange({
    $core.double? min,
    $core.double? max,
  }) {
    final result = NumberRange._();
    if (min != null) result.min = min;
    if (max != null) result.max = max;
    return result;
  }

  NumberRange._();

  factory NumberRange.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NumberRange()..mergeFromBuffer(data, registry);
  factory NumberRange.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      NumberRange()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NumberRange',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: NumberRange.$_createMessage)
    ..aD(1, _omitFieldNames ? '' : 'min')
    ..aD(2, _omitFieldNames ? '' : 'max')
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

  /// Absent or empty: the first page. Otherwise a cursor this query returned, unmodified.
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
    PageRequest? page,
    $core.String? profileId,
    $fixnum.Int64? scanRevision,
  }) {
    final result = Query._();
    if (collection != null) result.collection = collection;
    if (filter != null) result.filter = filter;
    if (sort != null) result.sort.addAll(sort);
    if (params != null) result.params = params;
    if (page != null) result.page = page;
    if (profileId != null) result.profileId = profileId;
    if (scanRevision != null) result.scanRevision = scanRevision;
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
    ..aOM<PageRequest>(5, _omitFieldNames ? '' : 'page',
        subBuilder: PageRequest.$_createMessage)
    ..aOS(6, _omitFieldNames ? '' : 'profileId')
    ..a<$fixnum.Int64>(
        7, _omitFieldNames ? '' : 'scanRevision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
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

  /// The session only (`ClientMessage.query`): the profile the rows come from. The headless
  /// endpoint supplies its own inventory and ignores it.
  @$pb.TagNumber(6)
  $core.String get profileId => $_getSZ(5);
  @$pb.TagNumber(6)
  set profileId($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasProfileId() => $_has(5);
  @$pb.TagNumber(6)
  void clearProfileId() => $_clearField(6);

  /// The session only: absent on a scan's first page; every later page repeats the revision the
  /// first page was valid at, and is refused with `query.stale_revision` if the projection moved.
  @$pb.TagNumber(7)
  $fixnum.Int64 get scanRevision => $_getI64(6);
  @$pb.TagNumber(7)
  set scanRevision($fixnum.Int64 value) => $_setInt64(6, value);
  @$pb.TagNumber(7)
  $core.bool hasScanRevision() => $_has(6);
  @$pb.TagNumber(7)
  void clearScanRevision() => $_clearField(7);
}

class QueryRow extends $pb.GeneratedMessage {
  factory QueryRow({
    $core.String? soulId,
    $core.Iterable<OpenRule>? openRules,
    Soul? soul,
  }) {
    final result = QueryRow._();
    if (soulId != null) result.soulId = soulId;
    if (openRules != null) result.openRules.addAll(openRules);
    if (soul != null) result.soul = soul;
    return result;
  }

  QueryRow._();

  factory QueryRow.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryRow()..mergeFromBuffer(data, registry);
  factory QueryRow.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      QueryRow()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'QueryRow',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: QueryRow.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'soulId')
    ..pc<OpenRule>(2, _omitFieldNames ? '' : 'openRules', $pb.PbFieldType.KE,
        valueOf: OpenRule.valueOf,
        enumValues: OpenRule.values,
        defaultEnumValue: OpenRule.OPEN_RULE_UNSPECIFIED)
    ..aOM<Soul>(3, _omitFieldNames ? '' : 'soul',
        subBuilder: Soul.$_createMessage)
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

  @$pb.TagNumber(1)
  $core.String get soulId => $_getSZ(0);
  @$pb.TagNumber(1)
  set soulId($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSoulId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSoulId() => $_clearField(1);

  /// Empty: the game's answer is exact. Otherwise the row is open and rests on these rules
  /// (ADR-0026).
  @$pb.TagNumber(2)
  $pb.PbList<OpenRule> get openRules => $_getList(1);

  /// The session only: the row's values, so the application shows what the daemon holds without a
  /// second request. The headless endpoint leaves it absent; its caller supplied the souls.
  @$pb.TagNumber(3)
  Soul get soul => $_getN(2);
  @$pb.TagNumber(3)
  set soul(Soul value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasSoul() => $_has(2);
  @$pb.TagNumber(3)
  void clearSoul() => $_clearField(3);
  @$pb.TagNumber(3)
  Soul ensureSoul() => $_ensure(2);
}

class QueryPage extends $pb.GeneratedMessage {
  factory QueryPage({
    $core.Iterable<QueryRow>? rows,
    $core.bool? hasMore,
    $core.List<$core.int>? cursor,
    $fixnum.Int64? revision,
    $fixnum.Int64? total,
  }) {
    final result = QueryPage._();
    if (rows != null) result.rows.addAll(rows);
    if (hasMore != null) result.hasMore = hasMore;
    if (cursor != null) result.cursor = cursor;
    if (revision != null) result.revision = revision;
    if (total != null) result.total = total;
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
    ..aOB(2, _omitFieldNames ? '' : 'hasMore')
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'cursor', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(
        4, _omitFieldNames ? '' : 'revision', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(5, _omitFieldNames ? '' : 'total', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
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

  @$pb.TagNumber(2)
  $core.bool get hasMore => $_getBF(1);
  @$pb.TagNumber(2)
  set hasMore($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasHasMore() => $_has(1);
  @$pb.TagNumber(2)
  void clearHasMore() => $_clearField(2);

  /// Empty when `has_more` is false. Opaque: sent back unmodified, never parsed.
  @$pb.TagNumber(3)
  $core.List<$core.int> get cursor => $_getN(2);
  @$pb.TagNumber(3)
  set cursor($core.List<$core.int> value) => $_setBytes(2, value);
  @$pb.TagNumber(3)
  $core.bool hasCursor() => $_has(2);
  @$pb.TagNumber(3)
  void clearCursor() => $_clearField(3);

  /// The revision the page is valid at; zero from the headless endpoint, which has no projection.
  @$pb.TagNumber(4)
  $fixnum.Int64 get revision => $_getI64(3);
  @$pb.TagNumber(4)
  set revision($fixnum.Int64 value) => $_setInt64(3, value);
  @$pb.TagNumber(4)
  $core.bool hasRevision() => $_has(3);
  @$pb.TagNumber(4)
  void clearRevision() => $_clearField(4);

  /// Every row the query keeps, exact or open, across all pages.
  @$pb.TagNumber(5)
  $fixnum.Int64 get total => $_getI64(4);
  @$pb.TagNumber(5)
  set total($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTotal() => $_has(4);
  @$pb.TagNumber(5)
  void clearTotal() => $_clearField(5);
}

/// A query over an inventory the request supplies, until the projection exists. One
/// EvaluateQueryResult answers each request, with the same `id`.
class EvaluateQuery extends $pb.GeneratedMessage {
  factory EvaluateQuery({
    $fixnum.Int64? id,
    ProtocolVersion? protocolVersion,
    $core.Iterable<Soul>? inventory,
    Query? query,
  }) {
    final result = EvaluateQuery._();
    if (id != null) result.id = id;
    if (protocolVersion != null) result.protocolVersion = protocolVersion;
    if (inventory != null) result.inventory.addAll(inventory);
    if (query != null) result.query = query;
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
}

enum EvaluateQueryResult_Outcome { page, error, notSet }

class EvaluateQueryResult extends $pb.GeneratedMessage {
  factory EvaluateQueryResult({
    $fixnum.Int64? id,
    QueryPage? page,
    Error? error,
  }) {
    final result = EvaluateQueryResult._();
    if (id != null) result.id = id;
    if (page != null) result.page = page;
    if (error != null) result.error = error;
    return result;
  }

  EvaluateQueryResult._();

  factory EvaluateQueryResult.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EvaluateQueryResult()..mergeFromBuffer(data, registry);
  factory EvaluateQueryResult.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      EvaluateQueryResult()..mergeFromJson(json, registry);

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
    ..oo(0, [2, 3])
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<QueryPage>(2, _omitFieldNames ? '' : 'page',
        subBuilder: QueryPage.$_createMessage)
    ..aOM<Error>(3, _omitFieldNames ? '' : 'error',
        subBuilder: Error.$_createMessage)
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

  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  EvaluateQueryResult_Outcome whichOutcome() =>
      _EvaluateQueryResult_OutcomeByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
  void clearOutcome() => $_clearField($_whichOneof(0));

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
}

enum ClientMessage_Kind {
  openSession,
  shutdown,
  subscribe,
  listProfiles,
  query,
  decodeSchemeCode,
  notSet
}

/// Client to daemon: one request per frame.
class ClientMessage extends $pb.GeneratedMessage {
  factory ClientMessage({
    ProtocolVersion? protocolVersion,
    $fixnum.Int64? id,
    OpenSession? openSession,
    Shutdown? shutdown,
    Subscribe? subscribe,
    ListProfiles? listProfiles,
    Query? query,
    DecodeSchemeCode? decodeSchemeCode,
  }) {
    final result = ClientMessage._();
    if (protocolVersion != null) result.protocolVersion = protocolVersion;
    if (id != null) result.id = id;
    if (openSession != null) result.openSession = openSession;
    if (shutdown != null) result.shutdown = shutdown;
    if (subscribe != null) result.subscribe = subscribe;
    if (listProfiles != null) result.listProfiles = listProfiles;
    if (query != null) result.query = query;
    if (decodeSchemeCode != null) result.decodeSchemeCode = decodeSchemeCode;
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
    21: ClientMessage_Kind.query,
    22: ClientMessage_Kind.decodeSchemeCode,
    0: ClientMessage_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: ClientMessage.$_createMessage)
    ..oo(0, [10, 11, 12, 20, 21, 22])
    ..aOM<ProtocolVersion>(1, _omitFieldNames ? '' : 'protocolVersion',
        subBuilder: ProtocolVersion.$_createMessage)
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
    ..aOM<Query>(21, _omitFieldNames ? '' : 'query',
        subBuilder: Query.$_createMessage)
    ..aOM<DecodeSchemeCode>(22, _omitFieldNames ? '' : 'decodeSchemeCode',
        subBuilder: DecodeSchemeCode.$_createMessage)
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
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  ClientMessage_Kind whichKind() => _ClientMessage_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  void clearKind() => $_clearField($_whichOneof(0));

  /// The protocol version the client was built against. Compared at `OpenSession`.
  @$pb.TagNumber(1)
  ProtocolVersion get protocolVersion => $_getN(0);
  @$pb.TagNumber(1)
  set protocolVersion(ProtocolVersion value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasProtocolVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearProtocolVersion() => $_clearField(1);
  @$pb.TagNumber(1)
  ProtocolVersion ensureProtocolVersion() => $_ensure(0);

  /// Chosen by the client, unique within the session, never reused. Never zero.
  @$pb.TagNumber(2)
  $fixnum.Int64 get id => $_getI64(1);
  @$pb.TagNumber(2)
  set id($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => $_clearField(2);

  @$pb.TagNumber(10)
  OpenSession get openSession => $_getN(2);
  @$pb.TagNumber(10)
  set openSession(OpenSession value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasOpenSession() => $_has(2);
  @$pb.TagNumber(10)
  void clearOpenSession() => $_clearField(10);
  @$pb.TagNumber(10)
  OpenSession ensureOpenSession() => $_ensure(2);

  @$pb.TagNumber(11)
  Shutdown get shutdown => $_getN(3);
  @$pb.TagNumber(11)
  set shutdown(Shutdown value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasShutdown() => $_has(3);
  @$pb.TagNumber(11)
  void clearShutdown() => $_clearField(11);
  @$pb.TagNumber(11)
  Shutdown ensureShutdown() => $_ensure(3);

  @$pb.TagNumber(12)
  Subscribe get subscribe => $_getN(4);
  @$pb.TagNumber(12)
  set subscribe(Subscribe value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasSubscribe() => $_has(4);
  @$pb.TagNumber(12)
  void clearSubscribe() => $_clearField(12);
  @$pb.TagNumber(12)
  Subscribe ensureSubscribe() => $_ensure(4);

  /// Queries.
  @$pb.TagNumber(20)
  ListProfiles get listProfiles => $_getN(5);
  @$pb.TagNumber(20)
  set listProfiles(ListProfiles value) => $_setField(20, value);
  @$pb.TagNumber(20)
  $core.bool hasListProfiles() => $_has(5);
  @$pb.TagNumber(20)
  void clearListProfiles() => $_clearField(20);
  @$pb.TagNumber(20)
  ListProfiles ensureListProfiles() => $_ensure(5);

  /// Carries `profile_id`, and `scan_revision` after the first page.
  @$pb.TagNumber(21)
  Query get query => $_getN(6);
  @$pb.TagNumber(21)
  set query(Query value) => $_setField(21, value);
  @$pb.TagNumber(21)
  $core.bool hasQuery() => $_has(6);
  @$pb.TagNumber(21)
  void clearQuery() => $_clearField(21);
  @$pb.TagNumber(21)
  Query ensureQuery() => $_ensure(6);

  @$pb.TagNumber(22)
  DecodeSchemeCode get decodeSchemeCode => $_getN(7);
  @$pb.TagNumber(22)
  set decodeSchemeCode(DecodeSchemeCode value) => $_setField(22, value);
  @$pb.TagNumber(22)
  $core.bool hasDecodeSchemeCode() => $_has(7);
  @$pb.TagNumber(22)
  void clearDecodeSchemeCode() => $_clearField(22);
  @$pb.TagNumber(22)
  DecodeSchemeCode ensureDecodeSchemeCode() => $_ensure(7);
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
  queryPage,
  schemeCodeDecoded,
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
    QueryPage? queryPage,
    SchemeCodeDecoded? schemeCodeDecoded,
  }) {
    final result = Response._();
    if (id != null) result.id = id;
    if (error != null) result.error = error;
    if (sessionOpened != null) result.sessionOpened = sessionOpened;
    if (shutdownAccepted != null) result.shutdownAccepted = shutdownAccepted;
    if (subscribed != null) result.subscribed = subscribed;
    if (profileList != null) result.profileList = profileList;
    if (queryPage != null) result.queryPage = queryPage;
    if (schemeCodeDecoded != null) result.schemeCodeDecoded = schemeCodeDecoded;
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
    21: Response_Result.queryPage,
    22: Response_Result.schemeCodeDecoded,
    0: Response_Result.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Response',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Response.$_createMessage)
    ..oo(0, [2, 10, 11, 12, 20, 21, 22])
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
    ..aOM<QueryPage>(21, _omitFieldNames ? '' : 'queryPage',
        subBuilder: QueryPage.$_createMessage)
    ..aOM<SchemeCodeDecoded>(22, _omitFieldNames ? '' : 'schemeCodeDecoded',
        subBuilder: SchemeCodeDecoded.$_createMessage)
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
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
  Response_Result whichResult() => _Response_ResultByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(2)
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(20)
  @$pb.TagNumber(21)
  @$pb.TagNumber(22)
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

  /// Every row carries `soul`, and the page its `revision` and `total`.
  @$pb.TagNumber(21)
  QueryPage get queryPage => $_getN(6);
  @$pb.TagNumber(21)
  set queryPage(QueryPage value) => $_setField(21, value);
  @$pb.TagNumber(21)
  $core.bool hasQueryPage() => $_has(6);
  @$pb.TagNumber(21)
  void clearQueryPage() => $_clearField(21);
  @$pb.TagNumber(21)
  QueryPage ensureQueryPage() => $_ensure(6);

  @$pb.TagNumber(22)
  SchemeCodeDecoded get schemeCodeDecoded => $_getN(7);
  @$pb.TagNumber(22)
  set schemeCodeDecoded(SchemeCodeDecoded value) => $_setField(22, value);
  @$pb.TagNumber(22)
  $core.bool hasSchemeCodeDecoded() => $_has(7);
  @$pb.TagNumber(22)
  void clearSchemeCodeDecoded() => $_clearField(22);
  @$pb.TagNumber(22)
  SchemeCodeDecoded ensureSchemeCodeDecoded() => $_ensure(7);
}

enum Event_Kind { projectionChanged, warning, sessionFailed, notSet }

/// Not a response: something the client did not ask for.
class Event extends $pb.GeneratedMessage {
  factory Event({
    ProjectionChanged? projectionChanged,
    Warning? warning,
    Error? sessionFailed,
  }) {
    final result = Event._();
    if (projectionChanged != null) result.projectionChanged = projectionChanged;
    if (warning != null) result.warning = warning;
    if (sessionFailed != null) result.sessionFailed = sessionFailed;
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
    3: Event_Kind.sessionFailed,
    0: Event_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Event',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Event.$_createMessage)
    ..oo(0, [1, 2, 3])
    ..aOM<ProjectionChanged>(1, _omitFieldNames ? '' : 'projectionChanged',
        subBuilder: ProjectionChanged.$_createMessage)
    ..aOM<Warning>(2, _omitFieldNames ? '' : 'warning',
        subBuilder: Warning.$_createMessage)
    ..aOM<Error>(3, _omitFieldNames ? '' : 'sessionFailed',
        subBuilder: Error.$_createMessage)
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
  @$pb.TagNumber(3)
  Event_Kind whichKind() => _Event_KindByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  @$pb.TagNumber(3)
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

  /// The session cannot continue, and the daemon exits after sending it: a frame that does not
  /// decode, or a payload that is not a `ClientMessage`. No request id is known for it.
  @$pb.TagNumber(3)
  Error get sessionFailed => $_getN(2);
  @$pb.TagNumber(3)
  set sessionFailed(Error value) => $_setField(3, value);
  @$pb.TagNumber(3)
  $core.bool hasSessionFailed() => $_has(2);
  @$pb.TagNumber(3)
  void clearSessionFailed() => $_clearField(3);
  @$pb.TagNumber(3)
  Error ensureSessionFailed() => $_ensure(2);
}

/// Surfaced to the user; changes no state and advances no revision.
class Warning extends $pb.GeneratedMessage {
  factory Warning({
    $core.String? code,
    $core.String? message,
  }) {
    final result = Warning._();
    if (code != null) result.code = code;
    if (message != null) result.message = message;
    return result;
  }

  Warning._();

  factory Warning.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Warning()..mergeFromBuffer(data, registry);
  factory Warning.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      Warning()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Warning',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'yata.core.v1'),
      createEmptyInstance: Warning.$_createMessage)
    ..aOS(1, _omitFieldNames ? '' : 'code')
    ..aOS(2, _omitFieldNames ? '' : 'message')
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

  @$pb.TagNumber(1)
  $core.String get code => $_getSZ(0);
  @$pb.TagNumber(1)
  set code($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

/// The first request of a session. Anything before it is refused with `session.not_open`.
class OpenSession extends $pb.GeneratedMessage {
  factory OpenSession() => OpenSession._();

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

  /// The projection's revision when the session opened.
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
  }) {
    final result = Profile._();
    if (id != null) result.id = id;
    if (name != null) result.name = name;
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

  /// Opaque; sent back unmodified.
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

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
