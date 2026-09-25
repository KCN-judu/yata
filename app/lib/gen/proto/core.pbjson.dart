// This is a generated file - do not edit.
//
// Generated from core.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use soulSlotDescriptor instead')
const SoulSlot$json = {
  '1': 'SoulSlot',
  '2': [
    {'1': 'SOUL_SLOT_UNSPECIFIED', '2': 0},
    {'1': 'SOUL_SLOT_1', '2': 1},
    {'1': 'SOUL_SLOT_2', '2': 2},
    {'1': 'SOUL_SLOT_3', '2': 3},
    {'1': 'SOUL_SLOT_4', '2': 4},
    {'1': 'SOUL_SLOT_5', '2': 5},
    {'1': 'SOUL_SLOT_6', '2': 6},
  ],
};

/// Descriptor for `SoulSlot`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List soulSlotDescriptor = $convert.base64Decode(
    'CghTb3VsU2xvdBIZChVTT1VMX1NMT1RfVU5TUEVDSUZJRUQQABIPCgtTT1VMX1NMT1RfMRABEg'
    '8KC1NPVUxfU0xPVF8yEAISDwoLU09VTF9TTE9UXzMQAxIPCgtTT1VMX1NMT1RfNBAEEg8KC1NP'
    'VUxfU0xPVF81EAUSDwoLU09VTF9TTE9UXzYQBg==');

@$core.Deprecated('Use soulAttributeDescriptor instead')
const SoulAttribute$json = {
  '1': 'SoulAttribute',
  '2': [
    {'1': 'SOUL_ATTRIBUTE_UNSPECIFIED', '2': 0},
    {'1': 'SOUL_ATTRIBUTE_ATK_FLAT', '2': 1},
    {'1': 'SOUL_ATTRIBUTE_ATK_PERCENT', '2': 2},
    {'1': 'SOUL_ATTRIBUTE_DEF_FLAT', '2': 3},
    {'1': 'SOUL_ATTRIBUTE_DEF_PERCENT', '2': 4},
    {'1': 'SOUL_ATTRIBUTE_HP_FLAT', '2': 5},
    {'1': 'SOUL_ATTRIBUTE_HP_PERCENT', '2': 6},
    {'1': 'SOUL_ATTRIBUTE_SPD', '2': 7},
    {'1': 'SOUL_ATTRIBUTE_EFFECT_HIT', '2': 8},
    {'1': 'SOUL_ATTRIBUTE_EFFECT_RES', '2': 9},
    {'1': 'SOUL_ATTRIBUTE_CRIT', '2': 10},
    {'1': 'SOUL_ATTRIBUTE_CRIT_DMG', '2': 11},
  ],
};

/// Descriptor for `SoulAttribute`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List soulAttributeDescriptor = $convert.base64Decode(
    'Cg1Tb3VsQXR0cmlidXRlEh4KGlNPVUxfQVRUUklCVVRFX1VOU1BFQ0lGSUVEEAASGwoXU09VTF'
    '9BVFRSSUJVVEVfQVRLX0ZMQVQQARIeChpTT1VMX0FUVFJJQlVURV9BVEtfUEVSQ0VOVBACEhsK'
    'F1NPVUxfQVRUUklCVVRFX0RFRl9GTEFUEAMSHgoaU09VTF9BVFRSSUJVVEVfREVGX1BFUkNFTl'
    'QQBBIaChZTT1VMX0FUVFJJQlVURV9IUF9GTEFUEAUSHQoZU09VTF9BVFRSSUJVVEVfSFBfUEVS'
    'Q0VOVBAGEhYKElNPVUxfQVRUUklCVVRFX1NQRBAHEh0KGVNPVUxfQVRUUklCVVRFX0VGRkVDVF'
    '9ISVQQCBIdChlTT1VMX0FUVFJJQlVURV9FRkZFQ1RfUkVTEAkSFwoTU09VTF9BVFRSSUJVVEVf'
    'Q1JJVBAKEhsKF1NPVUxfQVRUUklCVVRFX0NSSVRfRE1HEAs=');

@$core.Deprecated('Use levelBandDescriptor instead')
const LevelBand$json = {
  '1': 'LevelBand',
  '2': [
    {'1': 'LEVEL_BAND_UNSPECIFIED', '2': 0},
    {'1': 'LEVEL_BAND_0_TO_2', '2': 1},
    {'1': 'LEVEL_BAND_3_TO_5', '2': 2},
    {'1': 'LEVEL_BAND_6_TO_8', '2': 3},
    {'1': 'LEVEL_BAND_9_TO_11', '2': 4},
    {'1': 'LEVEL_BAND_12_TO_14', '2': 5},
    {'1': 'LEVEL_BAND_15', '2': 6},
  ],
};

/// Descriptor for `LevelBand`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List levelBandDescriptor = $convert.base64Decode(
    'CglMZXZlbEJhbmQSGgoWTEVWRUxfQkFORF9VTlNQRUNJRklFRBAAEhUKEUxFVkVMX0JBTkRfMF'
    '9UT18yEAESFQoRTEVWRUxfQkFORF8zX1RPXzUQAhIVChFMRVZFTF9CQU5EXzZfVE9fOBADEhYK'
    'EkxFVkVMX0JBTkRfOV9UT18xMRAEEhcKE0xFVkVMX0JBTkRfMTJfVE9fMTQQBRIRCg1MRVZFTF'
    '9CQU5EXzE1EAY=');

@$core.Deprecated('Use subCountDescriptor instead')
const SubCount$json = {
  '1': 'SubCount',
  '2': [
    {'1': 'SUB_COUNT_UNSPECIFIED', '2': 0},
    {'1': 'SUB_COUNT_FEWER_THAN_TWO', '2': 1},
    {'1': 'SUB_COUNT_TWO', '2': 2},
    {'1': 'SUB_COUNT_THREE', '2': 3},
    {'1': 'SUB_COUNT_FOUR', '2': 4},
  ],
};

/// Descriptor for `SubCount`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List subCountDescriptor = $convert.base64Decode(
    'CghTdWJDb3VudBIZChVTVUJfQ09VTlRfVU5TUEVDSUZJRUQQABIcChhTVUJfQ09VTlRfRkVXRV'
    'JfVEhBTl9UV08QARIRCg1TVUJfQ09VTlRfVFdPEAISEwoPU1VCX0NPVU5UX1RIUkVFEAMSEgoO'
    'U1VCX0NPVU5UX0ZPVVIQBA==');

@$core.Deprecated('Use subAttributeModeDescriptor instead')
const SubAttributeMode$json = {
  '1': 'SubAttributeMode',
  '2': [
    {'1': 'SUB_ATTRIBUTE_MODE_UNSPECIFIED', '2': 0},
    {'1': 'SUB_ATTRIBUTE_MODE_INCLUDE', '2': 1},
    {'1': 'SUB_ATTRIBUTE_MODE_EXCLUDE', '2': 2},
  ],
};

/// Descriptor for `SubAttributeMode`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List subAttributeModeDescriptor = $convert.base64Decode(
    'ChBTdWJBdHRyaWJ1dGVNb2RlEiIKHlNVQl9BVFRSSUJVVEVfTU9ERV9VTlNQRUNJRklFRBAAEh'
    '4KGlNVQl9BVFRSSUJVVEVfTU9ERV9JTkNMVURFEAESHgoaU1VCX0FUVFJJQlVURV9NT0RFX0VY'
    'Q0xVREUQAg==');

@$core.Deprecated('Use collectionDescriptor instead')
const Collection$json = {
  '1': 'Collection',
  '2': [
    {'1': 'COLLECTION_UNSPECIFIED', '2': 0},
    {'1': 'COLLECTION_SOULS', '2': 1},
  ],
};

/// Descriptor for `Collection`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List collectionDescriptor = $convert.base64Decode(
    'CgpDb2xsZWN0aW9uEhoKFkNPTExFQ1RJT05fVU5TUEVDSUZJRUQQABIUChBDT0xMRUNUSU9OX1'
    'NPVUxTEAE=');

@$core.Deprecated('Use simpleFieldDescriptor instead')
const SimpleField$json = {
  '1': 'SimpleField',
  '2': [
    {'1': 'SIMPLE_FIELD_UNSPECIFIED', '2': 0},
    {'1': 'SIMPLE_FIELD_SET', '2': 1},
    {'1': 'SIMPLE_FIELD_SLOT', '2': 2},
    {'1': 'SIMPLE_FIELD_STAR', '2': 3},
    {'1': 'SIMPLE_FIELD_LEVEL', '2': 4},
    {'1': 'SIMPLE_FIELD_MAIN_ATTRIBUTE', '2': 5},
    {'1': 'SIMPLE_FIELD_MAIN_VALUE', '2': 6},
    {'1': 'SIMPLE_FIELD_SUB_COUNT', '2': 7},
    {'1': 'SIMPLE_FIELD_PRISTINE', '2': 8},
  ],
};

/// Descriptor for `SimpleField`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List simpleFieldDescriptor = $convert.base64Decode(
    'CgtTaW1wbGVGaWVsZBIcChhTSU1QTEVfRklFTERfVU5TUEVDSUZJRUQQABIUChBTSU1QTEVfRk'
    'lFTERfU0VUEAESFQoRU0lNUExFX0ZJRUxEX1NMT1QQAhIVChFTSU1QTEVfRklFTERfU1RBUhAD'
    'EhYKElNJTVBMRV9GSUVMRF9MRVZFTBAEEh8KG1NJTVBMRV9GSUVMRF9NQUlOX0FUVFJJQlVURR'
    'AFEhsKF1NJTVBMRV9GSUVMRF9NQUlOX1ZBTFVFEAYSGgoWU0lNUExFX0ZJRUxEX1NVQl9DT1VO'
    'VBAHEhkKFVNJTVBMRV9GSUVMRF9QUklTVElORRAI');

@$core.Deprecated('Use qualityComponentDescriptor instead')
const QualityComponent$json = {
  '1': 'QualityComponent',
  '2': [
    {'1': 'QUALITY_COMPONENT_UNSPECIFIED', '2': 0},
    {'1': 'QUALITY_COMPONENT_TOTAL', '2': 1},
    {'1': 'QUALITY_COMPONENT_DEPTH', '2': 2},
    {'1': 'QUALITY_COMPONENT_BREADTH', '2': 3},
  ],
};

/// Descriptor for `QualityComponent`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List qualityComponentDescriptor = $convert.base64Decode(
    'ChBRdWFsaXR5Q29tcG9uZW50EiEKHVFVQUxJVFlfQ09NUE9ORU5UX1VOU1BFQ0lGSUVEEAASGw'
    'oXUVVBTElUWV9DT01QT05FTlRfVE9UQUwQARIbChdRVUFMSVRZX0NPTVBPTkVOVF9ERVBUSBAC'
    'Eh0KGVFVQUxJVFlfQ09NUE9ORU5UX0JSRUFEVEgQAw==');

@$core.Deprecated('Use directionDescriptor instead')
const Direction$json = {
  '1': 'Direction',
  '2': [
    {'1': 'DIRECTION_UNSPECIFIED', '2': 0},
    {'1': 'DIRECTION_ASC', '2': 1},
    {'1': 'DIRECTION_DESC', '2': 2},
  ],
};

/// Descriptor for `Direction`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List directionDescriptor = $convert.base64Decode(
    'CglEaXJlY3Rpb24SGQoVRElSRUNUSU9OX1VOU1BFQ0lGSUVEEAASEQoNRElSRUNUSU9OX0FTQx'
    'ABEhIKDkRJUkVDVElPTl9ERVNDEAI=');

@$core.Deprecated('Use openRuleDescriptor instead')
const OpenRule$json = {
  '1': 'OpenRule',
  '2': [
    {'1': 'OPEN_RULE_UNSPECIFIED', '2': 0},
    {'1': 'OPEN_RULE_INNATE', '2': 1},
    {'1': 'OPEN_RULE_UNKNOWN_CONDITIONS', '2': 2},
  ],
};

/// Descriptor for `OpenRule`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List openRuleDescriptor = $convert.base64Decode(
    'CghPcGVuUnVsZRIZChVPUEVOX1JVTEVfVU5TUEVDSUZJRUQQABIUChBPUEVOX1JVTEVfSU5OQV'
    'RFEAESIAocT1BFTl9SVUxFX1VOS05PV05fQ09ORElUSU9OUxAC');

@$core.Deprecated('Use sectionKindDescriptor instead')
const SectionKind$json = {
  '1': 'SectionKind',
  '2': [
    {'1': 'SECTION_KIND_UNSPECIFIED', '2': 0},
    {'1': 'SECTION_KIND_SOULS', '2': 1},
    {'1': 'SECTION_KIND_SHIKIGAMI', '2': 2},
    {'1': 'SECTION_KIND_PRESETS', '2': 3},
    {'1': 'SECTION_KIND_ASSETS', '2': 4},
    {'1': 'SECTION_KIND_GUILD', '2': 5},
  ],
};

/// Descriptor for `SectionKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List sectionKindDescriptor = $convert.base64Decode(
    'CgtTZWN0aW9uS2luZBIcChhTRUNUSU9OX0tJTkRfVU5TUEVDSUZJRUQQABIWChJTRUNUSU9OX0'
    'tJTkRfU09VTFMQARIaChZTRUNUSU9OX0tJTkRfU0hJS0lHQU1JEAISGAoUU0VDVElPTl9LSU5E'
    'X1BSRVNFVFMQAxIXChNTRUNUSU9OX0tJTkRfQVNTRVRTEAQSFgoSU0VDVElPTl9LSU5EX0dVSU'
    'xEEAU=');

@$core.Deprecated('Use completenessDescriptor instead')
const Completeness$json = {
  '1': 'Completeness',
  '2': [
    {'1': 'COMPLETENESS_UNSPECIFIED', '2': 0},
    {'1': 'COMPLETENESS_UNSTATED', '2': 1},
    {'1': 'COMPLETENESS_PARTIAL', '2': 2},
    {'1': 'COMPLETENESS_COMPLETE', '2': 3},
  ],
};

/// Descriptor for `Completeness`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List completenessDescriptor = $convert.base64Decode(
    'CgxDb21wbGV0ZW5lc3MSHAoYQ09NUExFVEVORVNTX1VOU1BFQ0lGSUVEEAASGQoVQ09NUExFVE'
    'VORVNTX1VOU1RBVEVEEAESGAoUQ09NUExFVEVORVNTX1BBUlRJQUwQAhIZChVDT01QTEVURU5F'
    'U1NfQ09NUExFVEUQAw==');

@$core.Deprecated('Use capabilityDescriptor instead')
const Capability$json = {
  '1': 'Capability',
  '2': [
    {'1': 'CAPABILITY_UNSPECIFIED', '2': 0},
    {'1': 'CAPABILITY_INVENTORY', '2': 1},
    {'1': 'CAPABILITY_SHIKIGAMI_COLLECTION', '2': 2},
    {'1': 'CAPABILITY_GAME_PRESETS', '2': 3},
    {'1': 'CAPABILITY_ASSETS', '2': 4},
    {'1': 'CAPABILITY_GUILD_VIEW', '2': 5},
  ],
};

/// Descriptor for `Capability`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List capabilityDescriptor = $convert.base64Decode(
    'CgpDYXBhYmlsaXR5EhoKFkNBUEFCSUxJVFlfVU5TUEVDSUZJRUQQABIYChRDQVBBQklMSVRZX0'
    'lOVkVOVE9SWRABEiMKH0NBUEFCSUxJVFlfU0hJS0lHQU1JX0NPTExFQ1RJT04QAhIbChdDQVBB'
    'QklMSVRZX0dBTUVfUFJFU0VUUxADEhUKEUNBUEFCSUxJVFlfQVNTRVRTEAQSGQoVQ0FQQUJJTE'
    'lUWV9HVUlMRF9WSUVXEAU=');

@$core.Deprecated('Use schemeKindDescriptor instead')
const SchemeKind$json = {
  '1': 'SchemeKind',
  '2': [
    {'1': 'SCHEME_KIND_UNSPECIFIED', '2': 0},
    {'1': 'SCHEME_KIND_STRENGTHENING', '2': 1},
    {'1': 'SCHEME_KIND_DISCARD', '2': 2},
  ],
};

/// Descriptor for `SchemeKind`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List schemeKindDescriptor = $convert.base64Decode(
    'CgpTY2hlbWVLaW5kEhsKF1NDSEVNRV9LSU5EX1VOU1BFQ0lGSUVEEAASHQoZU0NIRU1FX0tJTk'
    'RfU1RSRU5HVEhFTklORxABEhcKE1NDSEVNRV9LSU5EX0RJU0NBUkQQAg==');

@$core.Deprecated('Use protocolVersionDescriptor instead')
const ProtocolVersion$json = {
  '1': 'ProtocolVersion',
  '2': [
    {'1': 'major', '3': 1, '4': 1, '5': 13, '10': 'major'},
    {'1': 'minor', '3': 2, '4': 1, '5': 13, '10': 'minor'},
  ],
};

/// Descriptor for `ProtocolVersion`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List protocolVersionDescriptor = $convert.base64Decode(
    'Cg9Qcm90b2NvbFZlcnNpb24SFAoFbWFqb3IYASABKA1SBW1ham9yEhQKBW1pbm9yGAIgASgNUg'
    'VtaW5vcg==');

@$core.Deprecated('Use soulDescriptor instead')
const Soul$json = {
  '1': 'Soul',
  '2': [
    {'1': 'soul_id', '3': 1, '4': 1, '5': 9, '10': 'soulId'},
    {'1': 'suit_code', '3': 2, '4': 1, '5': 13, '10': 'suitCode'},
    {
      '1': 'slot',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulSlot',
      '10': 'slot'
    },
    {'1': 'star', '3': 4, '4': 1, '5': 13, '10': 'star'},
    {'1': 'level', '3': 5, '4': 1, '5': 13, '10': 'level'},
    {
      '1': 'main',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'main'
    },
    {'1': 'main_value', '3': 7, '4': 1, '5': 1, '10': 'mainValue'},
    {
      '1': 'subs',
      '3': 8,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.SubAttribute',
      '10': 'subs'
    },
    {
      '1': 'ordinary',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.OrdinarySoul',
      '9': 0,
      '10': 'ordinary'
    },
    {
      '1': 'boss',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.BossSoul',
      '9': 0,
      '10': 'boss'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
  '9': [
    {'1': 9, '2': 10},
  ],
  '10': ['innate'],
};

/// Descriptor for `Soul`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List soulDescriptor = $convert.base64Decode(
    'CgRTb3VsEhcKB3NvdWxfaWQYASABKAlSBnNvdWxJZBIbCglzdWl0X2NvZGUYAiABKA1SCHN1aX'
    'RDb2RlEioKBHNsb3QYAyABKA4yFi55YXRhLmNvcmUudjEuU291bFNsb3RSBHNsb3QSEgoEc3Rh'
    'chgEIAEoDVIEc3RhchIUCgVsZXZlbBgFIAEoDVIFbGV2ZWwSLwoEbWFpbhgGIAEoDjIbLnlhdG'
    'EuY29yZS52MS5Tb3VsQXR0cmlidXRlUgRtYWluEh0KCm1haW5fdmFsdWUYByABKAFSCW1haW5W'
    'YWx1ZRIuCgRzdWJzGAggAygLMhoueWF0YS5jb3JlLnYxLlN1YkF0dHJpYnV0ZVIEc3VicxI4Cg'
    'hvcmRpbmFyeRgKIAEoCzIaLnlhdGEuY29yZS52MS5PcmRpbmFyeVNvdWxIAFIIb3JkaW5hcnkS'
    'LAoEYm9zcxgLIAEoCzIWLnlhdGEuY29yZS52MS5Cb3NzU291bEgAUgRib3NzQgYKBGtpbmRKBA'
    'gJEApSBmlubmF0ZQ==');

@$core.Deprecated('Use ordinarySoulDescriptor instead')
const OrdinarySoul$json = {
  '1': 'OrdinarySoul',
};

/// Descriptor for `OrdinarySoul`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List ordinarySoulDescriptor =
    $convert.base64Decode('CgxPcmRpbmFyeVNvdWw=');

@$core.Deprecated('Use bossSoulDescriptor instead')
const BossSoul$json = {
  '1': 'BossSoul',
  '2': [
    {
      '1': 'innate',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'innate'
    },
  ],
};

/// Descriptor for `BossSoul`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bossSoulDescriptor = $convert.base64Decode(
    'CghCb3NzU291bBIzCgZpbm5hdGUYASABKA4yGy55YXRhLmNvcmUudjEuU291bEF0dHJpYnV0ZV'
    'IGaW5uYXRl');

@$core.Deprecated('Use subAttributeDescriptor instead')
const SubAttribute$json = {
  '1': 'SubAttribute',
  '2': [
    {
      '1': 'attribute',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'attribute'
    },
    {'1': 'value', '3': 2, '4': 1, '5': 1, '10': 'value'},
    {
      '1': 'enhancement_count',
      '3': 3,
      '4': 1,
      '5': 13,
      '9': 0,
      '10': 'enhancementCount',
      '17': true
    },
  ],
  '8': [
    {'1': '_enhancement_count'},
  ],
};

/// Descriptor for `SubAttribute`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List subAttributeDescriptor = $convert.base64Decode(
    'CgxTdWJBdHRyaWJ1dGUSOQoJYXR0cmlidXRlGAEgASgOMhsueWF0YS5jb3JlLnYxLlNvdWxBdH'
    'RyaWJ1dGVSCWF0dHJpYnV0ZRIUCgV2YWx1ZRgCIAEoAVIFdmFsdWUSMAoRZW5oYW5jZW1lbnRf'
    'Y291bnQYAyABKA1IAFIQZW5oYW5jZW1lbnRDb3VudIgBAUIUChJfZW5oYW5jZW1lbnRfY291bn'
    'Q=');

@$core.Deprecated('Use anySetDescriptor instead')
const AnySet$json = {
  '1': 'AnySet',
};

/// Descriptor for `AnySet`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List anySetDescriptor =
    $convert.base64Decode('CgZBbnlTZXQ=');

@$core.Deprecated('Use suitCodesDescriptor instead')
const SuitCodes$json = {
  '1': 'SuitCodes',
  '2': [
    {'1': 'codes', '3': 1, '4': 3, '5': 13, '10': 'codes'},
  ],
};

/// Descriptor for `SuitCodes`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List suitCodesDescriptor =
    $convert.base64Decode('CglTdWl0Q29kZXMSFAoFY29kZXMYASADKA1SBWNvZGVz');

@$core.Deprecated('Use subAttributeChoiceDescriptor instead')
const SubAttributeChoice$json = {
  '1': 'SubAttributeChoice',
  '2': [
    {
      '1': 'attribute',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'attribute'
    },
    {
      '1': 'mode',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SubAttributeMode',
      '10': 'mode'
    },
  ],
};

/// Descriptor for `SubAttributeChoice`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List subAttributeChoiceDescriptor = $convert.base64Decode(
    'ChJTdWJBdHRyaWJ1dGVDaG9pY2USOQoJYXR0cmlidXRlGAEgASgOMhsueWF0YS5jb3JlLnYxLl'
    'NvdWxBdHRyaWJ1dGVSCWF0dHJpYnV0ZRIyCgRtb2RlGAIgASgOMh4ueWF0YS5jb3JlLnYxLlN1'
    'YkF0dHJpYnV0ZU1vZGVSBG1vZGU=');

@$core.Deprecated('Use soulSelectionDescriptor instead')
const SoulSelection$json = {
  '1': 'SoulSelection',
  '2': [
    {
      '1': 'all',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.AnySet',
      '9': 0,
      '10': 'all'
    },
    {
      '1': 'chosen',
      '3': 12,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SuitCodes',
      '9': 0,
      '10': 'chosen'
    },
    {
      '1': 'slots',
      '3': 3,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulSlot',
      '10': 'slots'
    },
    {'1': 'stars', '3': 4, '4': 3, '5': 13, '10': 'stars'},
    {
      '1': 'levels',
      '3': 5,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.LevelBand',
      '10': 'levels'
    },
    {
      '1': 'main_attributes',
      '3': 6,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'mainAttributes'
    },
    {
      '1': 'innate',
      '3': 7,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'innate'
    },
    {
      '1': 'sub_attributes',
      '3': 13,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.SubAttributeChoice',
      '10': 'subAttributes'
    },
    {
      '1': 'sub_counts',
      '3': 10,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SubCount',
      '10': 'subCounts'
    },
  ],
  '8': [
    {'1': 'sets'},
  ],
  '9': [
    {'1': 1, '2': 2},
    {'1': 2, '2': 3},
    {'1': 8, '2': 9},
    {'1': 9, '2': 10},
  ],
  '10': ['any_set', 'suit_codes', 'sub_included', 'sub_excluded'],
};

/// Descriptor for `SoulSelection`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List soulSelectionDescriptor = $convert.base64Decode(
    'Cg1Tb3VsU2VsZWN0aW9uEigKA2FsbBgLIAEoCzIULnlhdGEuY29yZS52MS5BbnlTZXRIAFIDYW'
    'xsEjEKBmNob3NlbhgMIAEoCzIXLnlhdGEuY29yZS52MS5TdWl0Q29kZXNIAFIGY2hvc2VuEiwK'
    'BXNsb3RzGAMgAygOMhYueWF0YS5jb3JlLnYxLlNvdWxTbG90UgVzbG90cxIUCgVzdGFycxgEIA'
    'MoDVIFc3RhcnMSLwoGbGV2ZWxzGAUgAygOMhcueWF0YS5jb3JlLnYxLkxldmVsQmFuZFIGbGV2'
    'ZWxzEkQKD21haW5fYXR0cmlidXRlcxgGIAMoDjIbLnlhdGEuY29yZS52MS5Tb3VsQXR0cmlidX'
    'RlUg5tYWluQXR0cmlidXRlcxIzCgZpbm5hdGUYByADKA4yGy55YXRhLmNvcmUudjEuU291bEF0'
    'dHJpYnV0ZVIGaW5uYXRlEkcKDnN1Yl9hdHRyaWJ1dGVzGA0gAygLMiAueWF0YS5jb3JlLnYxLl'
    'N1YkF0dHJpYnV0ZUNob2ljZVINc3ViQXR0cmlidXRlcxI1CgpzdWJfY291bnRzGAogAygOMhYu'
    'eWF0YS5jb3JlLnYxLlN1YkNvdW50UglzdWJDb3VudHNCBgoEc2V0c0oECAEQAkoECAIQA0oECA'
    'gQCUoECAkQClIHYW55X3NldFIKc3VpdF9jb2Rlc1IMc3ViX2luY2x1ZGVkUgxzdWJfZXhjbHVk'
    'ZWQ=');

@$core.Deprecated('Use fieldDescriptor instead')
const Field$json = {
  '1': 'Field',
  '2': [
    {
      '1': 'simple',
      '3': 3,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SimpleField',
      '9': 0,
      '10': 'simple'
    },
    {
      '1': 'sub_value',
      '3': 4,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '9': 0,
      '10': 'subValue'
    },
    {
      '1': 'has_sub',
      '3': 5,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '9': 0,
      '10': 'hasSub'
    },
    {
      '1': 'quality',
      '3': 6,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.QualityComponent',
      '9': 0,
      '10': 'quality'
    },
  ],
  '8': [
    {'1': 'field'},
  ],
  '9': [
    {'1': 1, '2': 2},
    {'1': 2, '2': 3},
  ],
  '10': ['name', 'attribute'],
};

/// Descriptor for `Field`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fieldDescriptor = $convert.base64Decode(
    'CgVGaWVsZBIzCgZzaW1wbGUYAyABKA4yGS55YXRhLmNvcmUudjEuU2ltcGxlRmllbGRIAFIGc2'
    'ltcGxlEjoKCXN1Yl92YWx1ZRgEIAEoDjIbLnlhdGEuY29yZS52MS5Tb3VsQXR0cmlidXRlSABS'
    'CHN1YlZhbHVlEjYKB2hhc19zdWIYBSABKA4yGy55YXRhLmNvcmUudjEuU291bEF0dHJpYnV0ZU'
    'gAUgZoYXNTdWISOgoHcXVhbGl0eRgGIAEoDjIeLnlhdGEuY29yZS52MS5RdWFsaXR5Q29tcG9u'
    'ZW50SABSB3F1YWxpdHlCBwoFZmllbGRKBAgBEAJKBAgCEANSBG5hbWVSCWF0dHJpYnV0ZQ==');

@$core.Deprecated('Use slotsDescriptor instead')
const Slots$json = {
  '1': 'Slots',
  '2': [
    {
      '1': 'values',
      '3': 1,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulSlot',
      '10': 'values'
    },
  ],
};

/// Descriptor for `Slots`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List slotsDescriptor = $convert.base64Decode(
    'CgVTbG90cxIuCgZ2YWx1ZXMYASADKA4yFi55YXRhLmNvcmUudjEuU291bFNsb3RSBnZhbHVlcw'
    '==');

@$core.Deprecated('Use attributesDescriptor instead')
const Attributes$json = {
  '1': 'Attributes',
  '2': [
    {
      '1': 'values',
      '3': 1,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'values'
    },
  ],
};

/// Descriptor for `Attributes`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List attributesDescriptor = $convert.base64Decode(
    'CgpBdHRyaWJ1dGVzEjMKBnZhbHVlcxgBIAMoDjIbLnlhdGEuY29yZS52MS5Tb3VsQXR0cmlidX'
    'RlUgZ2YWx1ZXM=');

@$core.Deprecated('Use inTestDescriptor instead')
const InTest$json = {
  '1': 'InTest',
  '2': [
    {
      '1': 'set_values',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SuitCodes',
      '9': 0,
      '10': 'setValues'
    },
    {
      '1': 'slot_values',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Slots',
      '9': 0,
      '10': 'slotValues'
    },
    {
      '1': 'attribute_values',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Attributes',
      '9': 0,
      '10': 'attributeValues'
    },
  ],
  '8': [
    {'1': 'values'},
  ],
  '9': [
    {'1': 1, '2': 2},
    {'1': 2, '2': 3},
    {'1': 3, '2': 4},
  ],
  '10': ['suit_codes', 'slots', 'attributes'],
};

/// Descriptor for `InTest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List inTestDescriptor = $convert.base64Decode(
    'CgZJblRlc3QSOAoKc2V0X3ZhbHVlcxgEIAEoCzIXLnlhdGEuY29yZS52MS5TdWl0Q29kZXNIAF'
    'IJc2V0VmFsdWVzEjYKC3Nsb3RfdmFsdWVzGAUgASgLMhMueWF0YS5jb3JlLnYxLlNsb3RzSABS'
    'CnNsb3RWYWx1ZXMSRQoQYXR0cmlidXRlX3ZhbHVlcxgGIAEoCzIYLnlhdGEuY29yZS52MS5BdH'
    'RyaWJ1dGVzSABSD2F0dHJpYnV0ZVZhbHVlc0IICgZ2YWx1ZXNKBAgBEAJKBAgCEANKBAgDEARS'
    'CnN1aXRfY29kZXNSBXNsb3RzUgphdHRyaWJ1dGVz');

@$core.Deprecated('Use intBetweenDescriptor instead')
const IntBetween$json = {
  '1': 'IntBetween',
  '2': [
    {'1': 'min', '3': 1, '4': 1, '5': 3, '10': 'min'},
    {'1': 'max', '3': 2, '4': 1, '5': 3, '10': 'max'},
  ],
};

/// Descriptor for `IntBetween`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List intBetweenDescriptor = $convert.base64Decode(
    'CgpJbnRCZXR3ZWVuEhAKA21pbhgBIAEoA1IDbWluEhAKA21heBgCIAEoA1IDbWF4');

@$core.Deprecated('Use intRangeDescriptor instead')
const IntRange$json = {
  '1': 'IntRange',
  '2': [
    {'1': 'at_least', '3': 3, '4': 1, '5': 3, '9': 0, '10': 'atLeast'},
    {'1': 'at_most', '3': 4, '4': 1, '5': 3, '9': 0, '10': 'atMost'},
    {
      '1': 'between',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.IntBetween',
      '9': 0,
      '10': 'between'
    },
  ],
  '8': [
    {'1': 'bound'},
  ],
  '9': [
    {'1': 1, '2': 2},
    {'1': 2, '2': 3},
  ],
  '10': ['min', 'max'],
};

/// Descriptor for `IntRange`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List intRangeDescriptor = $convert.base64Decode(
    'CghJbnRSYW5nZRIbCghhdF9sZWFzdBgDIAEoA0gAUgdhdExlYXN0EhkKB2F0X21vc3QYBCABKA'
    'NIAFIGYXRNb3N0EjQKB2JldHdlZW4YBSABKAsyGC55YXRhLmNvcmUudjEuSW50QmV0d2VlbkgA'
    'UgdiZXR3ZWVuQgcKBWJvdW5kSgQIARACSgQIAhADUgNtaW5SA21heA==');

@$core.Deprecated('Use numberBetweenDescriptor instead')
const NumberBetween$json = {
  '1': 'NumberBetween',
  '2': [
    {'1': 'min', '3': 1, '4': 1, '5': 1, '10': 'min'},
    {'1': 'max', '3': 2, '4': 1, '5': 1, '10': 'max'},
  ],
};

/// Descriptor for `NumberBetween`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List numberBetweenDescriptor = $convert.base64Decode(
    'Cg1OdW1iZXJCZXR3ZWVuEhAKA21pbhgBIAEoAVIDbWluEhAKA21heBgCIAEoAVIDbWF4');

@$core.Deprecated('Use numberRangeDescriptor instead')
const NumberRange$json = {
  '1': 'NumberRange',
  '2': [
    {'1': 'at_least', '3': 3, '4': 1, '5': 1, '9': 0, '10': 'atLeast'},
    {'1': 'at_most', '3': 4, '4': 1, '5': 1, '9': 0, '10': 'atMost'},
    {
      '1': 'between',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.NumberBetween',
      '9': 0,
      '10': 'between'
    },
  ],
  '8': [
    {'1': 'bound'},
  ],
  '9': [
    {'1': 1, '2': 2},
    {'1': 2, '2': 3},
  ],
  '10': ['min', 'max'],
};

/// Descriptor for `NumberRange`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List numberRangeDescriptor = $convert.base64Decode(
    'CgtOdW1iZXJSYW5nZRIbCghhdF9sZWFzdBgDIAEoAUgAUgdhdExlYXN0EhkKB2F0X21vc3QYBC'
    'ABKAFIAFIGYXRNb3N0EjcKB2JldHdlZW4YBSABKAsyGy55YXRhLmNvcmUudjEuTnVtYmVyQmV0'
    'd2VlbkgAUgdiZXR3ZWVuQgcKBWJvdW5kSgQIARACSgQIAhADUgNtaW5SA21heA==');

@$core.Deprecated('Use predicateDescriptor instead')
const Predicate$json = {
  '1': 'Predicate',
  '2': [
    {
      '1': 'field',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Field',
      '10': 'field'
    },
    {
      '1': 'in',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.InTest',
      '9': 0,
      '10': 'in'
    },
    {
      '1': 'int_range',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.IntRange',
      '9': 0,
      '10': 'intRange'
    },
    {
      '1': 'number_range',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.NumberRange',
      '9': 0,
      '10': 'numberRange'
    },
    {'1': 'is', '3': 5, '4': 1, '5': 8, '9': 0, '10': 'is'},
  ],
  '8': [
    {'1': 'test'},
  ],
};

/// Descriptor for `Predicate`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List predicateDescriptor = $convert.base64Decode(
    'CglQcmVkaWNhdGUSKQoFZmllbGQYASABKAsyEy55YXRhLmNvcmUudjEuRmllbGRSBWZpZWxkEi'
    'YKAmluGAIgASgLMhQueWF0YS5jb3JlLnYxLkluVGVzdEgAUgJpbhI1CglpbnRfcmFuZ2UYAyAB'
    'KAsyFi55YXRhLmNvcmUudjEuSW50UmFuZ2VIAFIIaW50UmFuZ2USPgoMbnVtYmVyX3JhbmdlGA'
    'QgASgLMhkueWF0YS5jb3JlLnYxLk51bWJlclJhbmdlSABSC251bWJlclJhbmdlEhAKAmlzGAUg'
    'ASgISABSAmlzQgYKBHRlc3Q=');

@$core.Deprecated('Use exprListDescriptor instead')
const ExprList$json = {
  '1': 'ExprList',
  '2': [
    {
      '1': 'items',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.Expr',
      '10': 'items'
    },
  ],
};

/// Descriptor for `ExprList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exprListDescriptor = $convert.base64Decode(
    'CghFeHByTGlzdBIoCgVpdGVtcxgBIAMoCzISLnlhdGEuY29yZS52MS5FeHByUgVpdGVtcw==');

@$core.Deprecated('Use schemeRefDescriptor instead')
const SchemeRef$json = {
  '1': 'SchemeRef',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    {'1': 'entry', '3': 2, '4': 1, '5': 13, '9': 0, '10': 'entry', '17': true},
  ],
  '8': [
    {'1': '_entry'},
  ],
};

/// Descriptor for `SchemeRef`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List schemeRefDescriptor = $convert.base64Decode(
    'CglTY2hlbWVSZWYSEgoEY29kZRgBIAEoCVIEY29kZRIZCgVlbnRyeRgCIAEoDUgAUgVlbnRyeY'
    'gBAUIICgZfZW50cnk=');

@$core.Deprecated('Use exprDescriptor instead')
const Expr$json = {
  '1': 'Expr',
  '2': [
    {
      '1': 'and',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ExprList',
      '9': 0,
      '10': 'and'
    },
    {
      '1': 'or',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ExprList',
      '9': 0,
      '10': 'or'
    },
    {
      '1': 'not',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Expr',
      '9': 0,
      '10': 'not'
    },
    {
      '1': 'pred',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Predicate',
      '9': 0,
      '10': 'pred'
    },
    {
      '1': 'matches',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SoulSelection',
      '9': 0,
      '10': 'matches'
    },
    {
      '1': 'matches_scheme',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SchemeRef',
      '9': 0,
      '10': 'matchesScheme'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
};

/// Descriptor for `Expr`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exprDescriptor = $convert.base64Decode(
    'CgRFeHByEioKA2FuZBgBIAEoCzIWLnlhdGEuY29yZS52MS5FeHByTGlzdEgAUgNhbmQSKAoCb3'
    'IYAiABKAsyFi55YXRhLmNvcmUudjEuRXhwckxpc3RIAFICb3ISJgoDbm90GAMgASgLMhIueWF0'
    'YS5jb3JlLnYxLkV4cHJIAFIDbm90Ei0KBHByZWQYBCABKAsyFy55YXRhLmNvcmUudjEuUHJlZG'
    'ljYXRlSABSBHByZWQSNwoHbWF0Y2hlcxgFIAEoCzIbLnlhdGEuY29yZS52MS5Tb3VsU2VsZWN0'
    'aW9uSABSB21hdGNoZXMSQAoObWF0Y2hlc19zY2hlbWUYBiABKAsyFy55YXRhLmNvcmUudjEuU2'
    'NoZW1lUmVmSABSDW1hdGNoZXNTY2hlbWVCBgoEa2luZA==');

@$core.Deprecated('Use sortKeyDescriptor instead')
const SortKey$json = {
  '1': 'SortKey',
  '2': [
    {
      '1': 'field',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Field',
      '10': 'field'
    },
    {
      '1': 'direction',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.Direction',
      '10': 'direction'
    },
  ],
};

/// Descriptor for `SortKey`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sortKeyDescriptor = $convert.base64Decode(
    'CgdTb3J0S2V5EikKBWZpZWxkGAEgASgLMhMueWF0YS5jb3JlLnYxLkZpZWxkUgVmaWVsZBI1Cg'
    'lkaXJlY3Rpb24YAiABKA4yFy55YXRhLmNvcmUudjEuRGlyZWN0aW9uUglkaXJlY3Rpb24=');

@$core.Deprecated('Use paramSetRefDescriptor instead')
const ParamSetRef$json = {
  '1': 'ParamSetRef',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'version', '3': 2, '4': 1, '5': 13, '10': 'version'},
  ],
};

/// Descriptor for `ParamSetRef`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List paramSetRefDescriptor = $convert.base64Decode(
    'CgtQYXJhbVNldFJlZhIOCgJpZBgBIAEoCVICaWQSGAoHdmVyc2lvbhgCIAEoDVIHdmVyc2lvbg'
    '==');

@$core.Deprecated('Use pageRequestDescriptor instead')
const PageRequest$json = {
  '1': 'PageRequest',
  '2': [
    {
      '1': 'row_budget',
      '3': 1,
      '4': 1,
      '5': 13,
      '9': 0,
      '10': 'rowBudget',
      '17': true
    },
    {
      '1': 'cursor',
      '3': 2,
      '4': 1,
      '5': 12,
      '9': 1,
      '10': 'cursor',
      '17': true
    },
  ],
  '8': [
    {'1': '_row_budget'},
    {'1': '_cursor'},
  ],
};

/// Descriptor for `PageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pageRequestDescriptor = $convert.base64Decode(
    'CgtQYWdlUmVxdWVzdBIiCgpyb3dfYnVkZ2V0GAEgASgNSABSCXJvd0J1ZGdldIgBARIbCgZjdX'
    'Jzb3IYAiABKAxIAVIGY3Vyc29yiAEBQg0KC19yb3dfYnVkZ2V0QgkKB19jdXJzb3I=');

@$core.Deprecated('Use queryDescriptor instead')
const Query$json = {
  '1': 'Query',
  '2': [
    {
      '1': 'collection',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.Collection',
      '10': 'collection'
    },
    {
      '1': 'filter',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Expr',
      '10': 'filter'
    },
    {
      '1': 'sort',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.SortKey',
      '10': 'sort'
    },
    {
      '1': 'params',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ParamSetRef',
      '10': 'params'
    },
  ],
  '9': [
    {'1': 5, '2': 6},
    {'1': 6, '2': 7},
    {'1': 7, '2': 8},
  ],
  '10': ['page', 'profile_id', 'scan_revision'],
};

/// Descriptor for `Query`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryDescriptor = $convert.base64Decode(
    'CgVRdWVyeRI4Cgpjb2xsZWN0aW9uGAEgASgOMhgueWF0YS5jb3JlLnYxLkNvbGxlY3Rpb25SCm'
    'NvbGxlY3Rpb24SKgoGZmlsdGVyGAIgASgLMhIueWF0YS5jb3JlLnYxLkV4cHJSBmZpbHRlchIp'
    'CgRzb3J0GAMgAygLMhUueWF0YS5jb3JlLnYxLlNvcnRLZXlSBHNvcnQSMQoGcGFyYW1zGAQgAS'
    'gLMhkueWF0YS5jb3JlLnYxLlBhcmFtU2V0UmVmUgZwYXJhbXNKBAgFEAZKBAgGEAdKBAgHEAhS'
    'BHBhZ2VSCnByb2ZpbGVfaWRSDXNjYW5fcmV2aXNpb24=');

@$core.Deprecated('Use exactVerdictDescriptor instead')
const ExactVerdict$json = {
  '1': 'ExactVerdict',
};

/// Descriptor for `ExactVerdict`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exactVerdictDescriptor =
    $convert.base64Decode('CgxFeGFjdFZlcmRpY3Q=');

@$core.Deprecated('Use openVerdictDescriptor instead')
const OpenVerdict$json = {
  '1': 'OpenVerdict',
  '2': [
    {
      '1': 'rules',
      '3': 1,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.OpenRule',
      '10': 'rules'
    },
  ],
};

/// Descriptor for `OpenVerdict`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List openVerdictDescriptor = $convert.base64Decode(
    'CgtPcGVuVmVyZGljdBIsCgVydWxlcxgBIAMoDjIWLnlhdGEuY29yZS52MS5PcGVuUnVsZVIFcn'
    'VsZXM=');

@$core.Deprecated('Use queryRowDescriptor instead')
const QueryRow$json = {
  '1': 'QueryRow',
  '2': [
    {'1': 'soul_id', '3': 1, '4': 1, '5': 9, '10': 'soulId'},
    {
      '1': 'exact',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ExactVerdict',
      '9': 0,
      '10': 'exact'
    },
    {
      '1': 'open',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.OpenVerdict',
      '9': 0,
      '10': 'open'
    },
  ],
  '8': [
    {'1': 'verdict'},
  ],
  '9': [
    {'1': 2, '2': 3},
    {'1': 3, '2': 4},
  ],
  '10': ['open_rules', 'soul'],
};

/// Descriptor for `QueryRow`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryRowDescriptor = $convert.base64Decode(
    'CghRdWVyeVJvdxIXCgdzb3VsX2lkGAEgASgJUgZzb3VsSWQSMgoFZXhhY3QYBCABKAsyGi55YX'
    'RhLmNvcmUudjEuRXhhY3RWZXJkaWN0SABSBWV4YWN0Ei8KBG9wZW4YBSABKAsyGS55YXRhLmNv'
    'cmUudjEuT3BlblZlcmRpY3RIAFIEb3BlbkIJCgd2ZXJkaWN0SgQIAhADSgQIAxAEUgpvcGVuX3'
    'J1bGVzUgRzb3Vs');

@$core.Deprecated('Use queryPageDescriptor instead')
const QueryPage$json = {
  '1': 'QueryPage',
  '2': [
    {
      '1': 'rows',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.QueryRow',
      '10': 'rows'
    },
    {
      '1': 'next_cursor',
      '3': 6,
      '4': 1,
      '5': 12,
      '9': 0,
      '10': 'nextCursor',
      '17': true
    },
    {'1': 'total', '3': 5, '4': 1, '5': 4, '10': 'total'},
  ],
  '8': [
    {'1': '_next_cursor'},
  ],
  '9': [
    {'1': 2, '2': 3},
    {'1': 3, '2': 4},
    {'1': 4, '2': 5},
  ],
  '10': ['has_more', 'cursor', 'revision'],
};

/// Descriptor for `QueryPage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryPageDescriptor = $convert.base64Decode(
    'CglRdWVyeVBhZ2USKgoEcm93cxgBIAMoCzIWLnlhdGEuY29yZS52MS5RdWVyeVJvd1IEcm93cx'
    'IkCgtuZXh0X2N1cnNvchgGIAEoDEgAUgpuZXh0Q3Vyc29yiAEBEhQKBXRvdGFsGAUgASgEUgV0'
    'b3RhbEIOCgxfbmV4dF9jdXJzb3JKBAgCEANKBAgDEARKBAgEEAVSCGhhc19tb3JlUgZjdXJzb3'
    'JSCHJldmlzaW9u');

@$core.Deprecated('Use evaluateQueryDescriptor instead')
const EvaluateQuery$json = {
  '1': 'EvaluateQuery',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 4, '10': 'id'},
    {
      '1': 'protocol_version',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'protocolVersion'
    },
    {
      '1': 'inventory',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.Soul',
      '10': 'inventory'
    },
    {
      '1': 'query',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Query',
      '10': 'query'
    },
    {
      '1': 'page',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.PageRequest',
      '10': 'page'
    },
  ],
};

/// Descriptor for `EvaluateQuery`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List evaluateQueryDescriptor = $convert.base64Decode(
    'Cg1FdmFsdWF0ZVF1ZXJ5Eg4KAmlkGAEgASgEUgJpZBJIChBwcm90b2NvbF92ZXJzaW9uGAIgAS'
    'gLMh0ueWF0YS5jb3JlLnYxLlByb3RvY29sVmVyc2lvblIPcHJvdG9jb2xWZXJzaW9uEjAKCWlu'
    'dmVudG9yeRgDIAMoCzISLnlhdGEuY29yZS52MS5Tb3VsUglpbnZlbnRvcnkSKQoFcXVlcnkYBC'
    'ABKAsyEy55YXRhLmNvcmUudjEuUXVlcnlSBXF1ZXJ5Ei0KBHBhZ2UYBSABKAsyGS55YXRhLmNv'
    'cmUudjEuUGFnZVJlcXVlc3RSBHBhZ2U=');

@$core.Deprecated('Use undecodableDescriptor instead')
const Undecodable$json = {
  '1': 'Undecodable',
};

/// Descriptor for `Undecodable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List undecodableDescriptor =
    $convert.base64Decode('CgtVbmRlY29kYWJsZQ==');

@$core.Deprecated('Use evaluateQueryResultDescriptor instead')
const EvaluateQueryResult$json = {
  '1': 'EvaluateQueryResult',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 4, '9': 0, '10': 'id'},
    {
      '1': 'undecodable',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Undecodable',
      '9': 0,
      '10': 'undecodable'
    },
    {
      '1': 'page',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryPage',
      '9': 1,
      '10': 'page'
    },
    {
      '1': 'error',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Error',
      '9': 1,
      '10': 'error'
    },
  ],
  '8': [
    {'1': 'subject'},
    {'1': 'outcome'},
  ],
};

/// Descriptor for `EvaluateQueryResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List evaluateQueryResultDescriptor = $convert.base64Decode(
    'ChNFdmFsdWF0ZVF1ZXJ5UmVzdWx0EhAKAmlkGAEgASgESABSAmlkEj0KC3VuZGVjb2RhYmxlGA'
    'QgASgLMhkueWF0YS5jb3JlLnYxLlVuZGVjb2RhYmxlSABSC3VuZGVjb2RhYmxlEi0KBHBhZ2UY'
    'AiABKAsyFy55YXRhLmNvcmUudjEuUXVlcnlQYWdlSAFSBHBhZ2USKwoFZXJyb3IYAyABKAsyEy'
    '55YXRhLmNvcmUudjEuRXJyb3JIAVIFZXJyb3JCCQoHc3ViamVjdEIJCgdvdXRjb21l');

@$core.Deprecated('Use clientMessageDescriptor instead')
const ClientMessage$json = {
  '1': 'ClientMessage',
  '2': [
    {'1': 'id', '3': 2, '4': 1, '5': 4, '10': 'id'},
    {
      '1': 'open_session',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.OpenSession',
      '9': 0,
      '10': 'openSession'
    },
    {
      '1': 'shutdown',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Shutdown',
      '9': 0,
      '10': 'shutdown'
    },
    {
      '1': 'subscribe',
      '3': 12,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Subscribe',
      '9': 0,
      '10': 'subscribe'
    },
    {
      '1': 'list_profiles',
      '3': 20,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ListProfiles',
      '9': 0,
      '10': 'listProfiles'
    },
    {
      '1': 'session_query',
      '3': 23,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionQuery',
      '9': 0,
      '10': 'sessionQuery'
    },
    {
      '1': 'decode_scheme_code',
      '3': 22,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeSchemeCode',
      '9': 0,
      '10': 'decodeSchemeCode'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
  '9': [
    {'1': 1, '2': 2},
    {'1': 21, '2': 22},
  ],
  '10': ['protocol_version', 'query'],
};

/// Descriptor for `ClientMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientMessageDescriptor = $convert.base64Decode(
    'Cg1DbGllbnRNZXNzYWdlEg4KAmlkGAIgASgEUgJpZBI+CgxvcGVuX3Nlc3Npb24YCiABKAsyGS'
    '55YXRhLmNvcmUudjEuT3BlblNlc3Npb25IAFILb3BlblNlc3Npb24SNAoIc2h1dGRvd24YCyAB'
    'KAsyFi55YXRhLmNvcmUudjEuU2h1dGRvd25IAFIIc2h1dGRvd24SNwoJc3Vic2NyaWJlGAwgAS'
    'gLMhcueWF0YS5jb3JlLnYxLlN1YnNjcmliZUgAUglzdWJzY3JpYmUSQQoNbGlzdF9wcm9maWxl'
    'cxgUIAEoCzIaLnlhdGEuY29yZS52MS5MaXN0UHJvZmlsZXNIAFIMbGlzdFByb2ZpbGVzEkEKDX'
    'Nlc3Npb25fcXVlcnkYFyABKAsyGi55YXRhLmNvcmUudjEuU2Vzc2lvblF1ZXJ5SABSDHNlc3Np'
    'b25RdWVyeRJOChJkZWNvZGVfc2NoZW1lX2NvZGUYFiABKAsyHi55YXRhLmNvcmUudjEuRGVjb2'
    'RlU2NoZW1lQ29kZUgAUhBkZWNvZGVTY2hlbWVDb2RlQgYKBGtpbmRKBAgBEAJKBAgVEBZSEHBy'
    'b3RvY29sX3ZlcnNpb25SBXF1ZXJ5');

@$core.Deprecated('Use serverMessageDescriptor instead')
const ServerMessage$json = {
  '1': 'ServerMessage',
  '2': [
    {
      '1': 'response',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Response',
      '9': 0,
      '10': 'response'
    },
    {
      '1': 'event',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Event',
      '9': 0,
      '10': 'event'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
};

/// Descriptor for `ServerMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverMessageDescriptor = $convert.base64Decode(
    'Cg1TZXJ2ZXJNZXNzYWdlEjQKCHJlc3BvbnNlGAEgASgLMhYueWF0YS5jb3JlLnYxLlJlc3Bvbn'
    'NlSABSCHJlc3BvbnNlEisKBWV2ZW50GAIgASgLMhMueWF0YS5jb3JlLnYxLkV2ZW50SABSBWV2'
    'ZW50QgYKBGtpbmQ=');

@$core.Deprecated('Use responseDescriptor instead')
const Response$json = {
  '1': 'Response',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 4, '10': 'id'},
    {
      '1': 'error',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Error',
      '9': 0,
      '10': 'error'
    },
    {
      '1': 'session_opened',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionOpened',
      '9': 0,
      '10': 'sessionOpened'
    },
    {
      '1': 'shutdown_accepted',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ShutdownAccepted',
      '9': 0,
      '10': 'shutdownAccepted'
    },
    {
      '1': 'subscribed',
      '3': 12,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Subscribed',
      '9': 0,
      '10': 'subscribed'
    },
    {
      '1': 'profile_list',
      '3': 20,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProfileList',
      '9': 0,
      '10': 'profileList'
    },
    {
      '1': 'session_query_page',
      '3': 23,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionQueryPage',
      '9': 0,
      '10': 'sessionQueryPage'
    },
    {
      '1': 'scheme_code_decoded',
      '3': 22,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SchemeCodeDecoded',
      '9': 0,
      '10': 'schemeCodeDecoded'
    },
  ],
  '8': [
    {'1': 'result'},
  ],
  '9': [
    {'1': 21, '2': 22},
  ],
  '10': ['query_page'],
};

/// Descriptor for `Response`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List responseDescriptor = $convert.base64Decode(
    'CghSZXNwb25zZRIOCgJpZBgBIAEoBFICaWQSKwoFZXJyb3IYAiABKAsyEy55YXRhLmNvcmUudj'
    'EuRXJyb3JIAFIFZXJyb3ISRAoOc2Vzc2lvbl9vcGVuZWQYCiABKAsyGy55YXRhLmNvcmUudjEu'
    'U2Vzc2lvbk9wZW5lZEgAUg1zZXNzaW9uT3BlbmVkEk0KEXNodXRkb3duX2FjY2VwdGVkGAsgAS'
    'gLMh4ueWF0YS5jb3JlLnYxLlNodXRkb3duQWNjZXB0ZWRIAFIQc2h1dGRvd25BY2NlcHRlZBI6'
    'CgpzdWJzY3JpYmVkGAwgASgLMhgueWF0YS5jb3JlLnYxLlN1YnNjcmliZWRIAFIKc3Vic2NyaW'
    'JlZBI+Cgxwcm9maWxlX2xpc3QYFCABKAsyGS55YXRhLmNvcmUudjEuUHJvZmlsZUxpc3RIAFIL'
    'cHJvZmlsZUxpc3QSTgoSc2Vzc2lvbl9xdWVyeV9wYWdlGBcgASgLMh4ueWF0YS5jb3JlLnYxLl'
    'Nlc3Npb25RdWVyeVBhZ2VIAFIQc2Vzc2lvblF1ZXJ5UGFnZRJRChNzY2hlbWVfY29kZV9kZWNv'
    'ZGVkGBYgASgLMh8ueWF0YS5jb3JlLnYxLlNjaGVtZUNvZGVEZWNvZGVkSABSEXNjaGVtZUNvZG'
    'VEZWNvZGVkQggKBnJlc3VsdEoECBUQFlIKcXVlcnlfcGFnZQ==');

@$core.Deprecated('Use eventDescriptor instead')
const Event$json = {
  '1': 'Event',
  '2': [
    {
      '1': 'projection_changed',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProjectionChanged',
      '9': 0,
      '10': 'projectionChanged'
    },
    {
      '1': 'warning',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Warning',
      '9': 0,
      '10': 'warning'
    },
    {
      '1': 'session_failure',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionFailed',
      '9': 0,
      '10': 'sessionFailure'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
  '9': [
    {'1': 3, '2': 4},
  ],
  '10': ['session_failed'],
};

/// Descriptor for `Event`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List eventDescriptor = $convert.base64Decode(
    'CgVFdmVudBJQChJwcm9qZWN0aW9uX2NoYW5nZWQYASABKAsyHy55YXRhLmNvcmUudjEuUHJvam'
    'VjdGlvbkNoYW5nZWRIAFIRcHJvamVjdGlvbkNoYW5nZWQSMQoHd2FybmluZxgCIAEoCzIVLnlh'
    'dGEuY29yZS52MS5XYXJuaW5nSABSB3dhcm5pbmcSRgoPc2Vzc2lvbl9mYWlsdXJlGAQgASgLMh'
    'sueWF0YS5jb3JlLnYxLlNlc3Npb25GYWlsZWRIAFIOc2Vzc2lvbkZhaWx1cmVCBgoEa2luZEoE'
    'CAMQBFIOc2Vzc2lvbl9mYWlsZWQ=');

@$core.Deprecated('Use openSessionDescriptor instead')
const OpenSession$json = {
  '1': 'OpenSession',
  '2': [
    {
      '1': 'client_version',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'clientVersion'
    },
  ],
};

/// Descriptor for `OpenSession`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List openSessionDescriptor = $convert.base64Decode(
    'CgtPcGVuU2Vzc2lvbhJECg5jbGllbnRfdmVyc2lvbhgBIAEoCzIdLnlhdGEuY29yZS52MS5Qcm'
    '90b2NvbFZlcnNpb25SDWNsaWVudFZlcnNpb24=');

@$core.Deprecated('Use sessionOpenedDescriptor instead')
const SessionOpened$json = {
  '1': 'SessionOpened',
  '2': [
    {
      '1': 'daemon_version',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'daemonVersion'
    },
    {'1': 'revision', '3': 2, '4': 1, '5': 4, '10': 'revision'},
  ],
};

/// Descriptor for `SessionOpened`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionOpenedDescriptor = $convert.base64Decode(
    'Cg1TZXNzaW9uT3BlbmVkEkQKDmRhZW1vbl92ZXJzaW9uGAEgASgLMh0ueWF0YS5jb3JlLnYxLl'
    'Byb3RvY29sVmVyc2lvblINZGFlbW9uVmVyc2lvbhIaCghyZXZpc2lvbhgCIAEoBFIIcmV2aXNp'
    'b24=');

@$core.Deprecated('Use shutdownDescriptor instead')
const Shutdown$json = {
  '1': 'Shutdown',
};

/// Descriptor for `Shutdown`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List shutdownDescriptor =
    $convert.base64Decode('CghTaHV0ZG93bg==');

@$core.Deprecated('Use shutdownAcceptedDescriptor instead')
const ShutdownAccepted$json = {
  '1': 'ShutdownAccepted',
};

/// Descriptor for `ShutdownAccepted`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List shutdownAcceptedDescriptor =
    $convert.base64Decode('ChBTaHV0ZG93bkFjY2VwdGVk');

@$core.Deprecated('Use subscribeDescriptor instead')
const Subscribe$json = {
  '1': 'Subscribe',
  '2': [
    {'1': 'revision', '3': 1, '4': 1, '5': 4, '10': 'revision'},
  ],
};

/// Descriptor for `Subscribe`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List subscribeDescriptor = $convert
    .base64Decode('CglTdWJzY3JpYmUSGgoIcmV2aXNpb24YASABKARSCHJldmlzaW9u');

@$core.Deprecated('Use subscribedDescriptor instead')
const Subscribed$json = {
  '1': 'Subscribed',
  '2': [
    {'1': 'revision', '3': 1, '4': 1, '5': 4, '10': 'revision'},
  ],
};

/// Descriptor for `Subscribed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List subscribedDescriptor = $convert
    .base64Decode('CgpTdWJzY3JpYmVkEhoKCHJldmlzaW9uGAEgASgEUghyZXZpc2lvbg==');

@$core.Deprecated('Use projectionChangedDescriptor instead')
const ProjectionChanged$json = {
  '1': 'ProjectionChanged',
  '2': [
    {'1': 'revision', '3': 1, '4': 1, '5': 4, '10': 'revision'},
  ],
};

/// Descriptor for `ProjectionChanged`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List projectionChangedDescriptor = $convert.base64Decode(
    'ChFQcm9qZWN0aW9uQ2hhbmdlZBIaCghyZXZpc2lvbhgBIAEoBFIIcmV2aXNpb24=');

@$core.Deprecated('Use listProfilesDescriptor instead')
const ListProfiles$json = {
  '1': 'ListProfiles',
};

/// Descriptor for `ListProfiles`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listProfilesDescriptor =
    $convert.base64Decode('CgxMaXN0UHJvZmlsZXM=');

@$core.Deprecated('Use profileListDescriptor instead')
const ProfileList$json = {
  '1': 'ProfileList',
  '2': [
    {'1': 'revision', '3': 1, '4': 1, '5': 4, '10': 'revision'},
    {
      '1': 'profiles',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.Profile',
      '10': 'profiles'
    },
  ],
};

/// Descriptor for `ProfileList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileListDescriptor = $convert.base64Decode(
    'CgtQcm9maWxlTGlzdBIaCghyZXZpc2lvbhgBIAEoBFIIcmV2aXNpb24SMQoIcHJvZmlsZXMYAi'
    'ADKAsyFS55YXRhLmNvcmUudjEuUHJvZmlsZVIIcHJvZmlsZXM=');

@$core.Deprecated('Use profileDescriptor instead')
const Profile$json = {
  '1': 'Profile',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'capabilities',
      '3': 3,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.ProfileCapability',
      '10': 'capabilities'
    },
  ],
};

/// Descriptor for `Profile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileDescriptor = $convert.base64Decode(
    'CgdQcm9maWxlEg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW1lEkMKDGNhcGFiaW'
    'xpdGllcxgDIAMoCzIfLnlhdGEuY29yZS52MS5Qcm9maWxlQ2FwYWJpbGl0eVIMY2FwYWJpbGl0'
    'aWVz');

@$core.Deprecated('Use profileCapabilityDescriptor instead')
const ProfileCapability$json = {
  '1': 'ProfileCapability',
  '2': [
    {
      '1': 'capability',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.Capability',
      '10': 'capability'
    },
    {
      '1': 'unavailable',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.CapabilityUnavailable',
      '9': 0,
      '10': 'unavailable'
    },
    {
      '1': 'available',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.CapabilityAvailable',
      '9': 0,
      '10': 'available'
    },
  ],
  '8': [
    {'1': 'availability'},
  ],
};

/// Descriptor for `ProfileCapability`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileCapabilityDescriptor = $convert.base64Decode(
    'ChFQcm9maWxlQ2FwYWJpbGl0eRI4CgpjYXBhYmlsaXR5GAEgASgOMhgueWF0YS5jb3JlLnYxLk'
    'NhcGFiaWxpdHlSCmNhcGFiaWxpdHkSRwoLdW5hdmFpbGFibGUYAiABKAsyIy55YXRhLmNvcmUu'
    'djEuQ2FwYWJpbGl0eVVuYXZhaWxhYmxlSABSC3VuYXZhaWxhYmxlEkEKCWF2YWlsYWJsZRgDIA'
    'EoCzIhLnlhdGEuY29yZS52MS5DYXBhYmlsaXR5QXZhaWxhYmxlSABSCWF2YWlsYWJsZUIOCgxh'
    'dmFpbGFiaWxpdHk=');

@$core.Deprecated('Use capabilityUnavailableDescriptor instead')
const CapabilityUnavailable$json = {
  '1': 'CapabilityUnavailable',
  '2': [
    {
      '1': 'missing',
      '3': 1,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SectionKind',
      '10': 'missing'
    },
  ],
};

/// Descriptor for `CapabilityUnavailable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List capabilityUnavailableDescriptor = $convert.base64Decode(
    'ChVDYXBhYmlsaXR5VW5hdmFpbGFibGUSMwoHbWlzc2luZxgBIAMoDjIZLnlhdGEuY29yZS52MS'
    '5TZWN0aW9uS2luZFIHbWlzc2luZw==');

@$core.Deprecated('Use capabilityAvailableDescriptor instead')
const CapabilityAvailable$json = {
  '1': 'CapabilityAvailable',
  '2': [
    {
      '1': 'completeness',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.Completeness',
      '10': 'completeness'
    },
  ],
};

/// Descriptor for `CapabilityAvailable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List capabilityAvailableDescriptor = $convert.base64Decode(
    'ChNDYXBhYmlsaXR5QXZhaWxhYmxlEj4KDGNvbXBsZXRlbmVzcxgBIAEoDjIaLnlhdGEuY29yZS'
    '52MS5Db21wbGV0ZW5lc3NSDGNvbXBsZXRlbmVzcw==');

@$core.Deprecated('Use sessionQueryDescriptor instead')
const SessionQuery$json = {
  '1': 'SessionQuery',
  '2': [
    {'1': 'profile_id', '3': 1, '4': 1, '5': 9, '10': 'profileId'},
    {
      '1': 'query',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Query',
      '10': 'query'
    },
    {
      '1': 'row_budget',
      '3': 3,
      '4': 1,
      '5': 13,
      '9': 1,
      '10': 'rowBudget',
      '17': true
    },
    {
      '1': 'first',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.FirstPage',
      '9': 0,
      '10': 'first'
    },
    {
      '1': 'next',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.NextPage',
      '9': 0,
      '10': 'next'
    },
  ],
  '8': [
    {'1': 'position'},
    {'1': '_row_budget'},
  ],
};

/// Descriptor for `SessionQuery`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionQueryDescriptor = $convert.base64Decode(
    'CgxTZXNzaW9uUXVlcnkSHQoKcHJvZmlsZV9pZBgBIAEoCVIJcHJvZmlsZUlkEikKBXF1ZXJ5GA'
    'IgASgLMhMueWF0YS5jb3JlLnYxLlF1ZXJ5UgVxdWVyeRIiCgpyb3dfYnVkZ2V0GAMgASgNSAFS'
    'CXJvd0J1ZGdldIgBARIvCgVmaXJzdBgEIAEoCzIXLnlhdGEuY29yZS52MS5GaXJzdFBhZ2VIAF'
    'IFZmlyc3QSLAoEbmV4dBgFIAEoCzIWLnlhdGEuY29yZS52MS5OZXh0UGFnZUgAUgRuZXh0QgoK'
    'CHBvc2l0aW9uQg0KC19yb3dfYnVkZ2V0');

@$core.Deprecated('Use firstPageDescriptor instead')
const FirstPage$json = {
  '1': 'FirstPage',
};

/// Descriptor for `FirstPage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List firstPageDescriptor =
    $convert.base64Decode('CglGaXJzdFBhZ2U=');

@$core.Deprecated('Use nextPageDescriptor instead')
const NextPage$json = {
  '1': 'NextPage',
  '2': [
    {'1': 'cursor', '3': 1, '4': 1, '5': 12, '10': 'cursor'},
    {'1': 'scan', '3': 2, '4': 1, '5': 4, '10': 'scan'},
  ],
};

/// Descriptor for `NextPage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List nextPageDescriptor = $convert.base64Decode(
    'CghOZXh0UGFnZRIWCgZjdXJzb3IYASABKAxSBmN1cnNvchISCgRzY2FuGAIgASgEUgRzY2Fu');

@$core.Deprecated('Use sessionQueryPageDescriptor instead')
const SessionQueryPage$json = {
  '1': 'SessionQueryPage',
  '2': [
    {
      '1': 'rows',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.SessionRow',
      '10': 'rows'
    },
    {
      '1': 'next_cursor',
      '3': 2,
      '4': 1,
      '5': 12,
      '9': 0,
      '10': 'nextCursor',
      '17': true
    },
    {'1': 'total', '3': 3, '4': 1, '5': 4, '10': 'total'},
    {'1': 'revision', '3': 4, '4': 1, '5': 4, '10': 'revision'},
  ],
  '8': [
    {'1': '_next_cursor'},
  ],
};

/// Descriptor for `SessionQueryPage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionQueryPageDescriptor = $convert.base64Decode(
    'ChBTZXNzaW9uUXVlcnlQYWdlEiwKBHJvd3MYASADKAsyGC55YXRhLmNvcmUudjEuU2Vzc2lvbl'
    'Jvd1IEcm93cxIkCgtuZXh0X2N1cnNvchgCIAEoDEgAUgpuZXh0Q3Vyc29yiAEBEhQKBXRvdGFs'
    'GAMgASgEUgV0b3RhbBIaCghyZXZpc2lvbhgEIAEoBFIIcmV2aXNpb25CDgoMX25leHRfY3Vyc2'
    '9y');

@$core.Deprecated('Use sessionRowDescriptor instead')
const SessionRow$json = {
  '1': 'SessionRow',
  '2': [
    {
      '1': 'soul',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Soul',
      '10': 'soul'
    },
    {
      '1': 'exact',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ExactVerdict',
      '9': 0,
      '10': 'exact'
    },
    {
      '1': 'open',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.OpenVerdict',
      '9': 0,
      '10': 'open'
    },
  ],
  '8': [
    {'1': 'verdict'},
  ],
};

/// Descriptor for `SessionRow`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionRowDescriptor = $convert.base64Decode(
    'CgpTZXNzaW9uUm93EiYKBHNvdWwYASABKAsyEi55YXRhLmNvcmUudjEuU291bFIEc291bBIyCg'
    'VleGFjdBgCIAEoCzIaLnlhdGEuY29yZS52MS5FeGFjdFZlcmRpY3RIAFIFZXhhY3QSLwoEb3Bl'
    'bhgDIAEoCzIZLnlhdGEuY29yZS52MS5PcGVuVmVyZGljdEgAUgRvcGVuQgkKB3ZlcmRpY3Q=');

@$core.Deprecated('Use decodeSchemeCodeDescriptor instead')
const DecodeSchemeCode$json = {
  '1': 'DecodeSchemeCode',
  '2': [
    {'1': 'text', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'text'},
    {'1': 'png', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'png'},
  ],
  '8': [
    {'1': 'source'},
  ],
};

/// Descriptor for `DecodeSchemeCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeSchemeCodeDescriptor = $convert.base64Decode(
    'ChBEZWNvZGVTY2hlbWVDb2RlEhQKBHRleHQYASABKAlIAFIEdGV4dBISCgNwbmcYAiABKAxIAF'
    'IDcG5nQggKBnNvdXJjZQ==');

@$core.Deprecated('Use schemeCodeDecodedDescriptor instead')
const SchemeCodeDecoded$json = {
  '1': 'SchemeCodeDecoded',
  '2': [
    {
      '1': 'kind',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SchemeKind',
      '10': 'kind'
    },
    {
      '1': 'entries',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.yata.core.v1.SchemeEntry',
      '10': 'entries'
    },
    {
      '1': 'encoded',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.EncodedScheme',
      '10': 'encoded'
    },
  ],
};

/// Descriptor for `SchemeCodeDecoded`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List schemeCodeDecodedDescriptor = $convert.base64Decode(
    'ChFTY2hlbWVDb2RlRGVjb2RlZBIsCgRraW5kGAEgASgOMhgueWF0YS5jb3JlLnYxLlNjaGVtZU'
    'tpbmRSBGtpbmQSMwoHZW50cmllcxgCIAMoCzIZLnlhdGEuY29yZS52MS5TY2hlbWVFbnRyeVIH'
    'ZW50cmllcxI1CgdlbmNvZGVkGAMgASgLMhsueWF0YS5jb3JlLnYxLkVuY29kZWRTY2hlbWVSB2'
    'VuY29kZWQ=');

@$core.Deprecated('Use schemeEntryDescriptor instead')
const SchemeEntry$json = {
  '1': 'SchemeEntry',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {
      '1': 'selection',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SoulSelection',
      '10': 'selection'
    },
    {
      '1': 'has_unknown_conditions',
      '3': 3,
      '4': 1,
      '5': 8,
      '10': 'hasUnknownConditions'
    },
  ],
};

/// Descriptor for `SchemeEntry`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List schemeEntryDescriptor = $convert.base64Decode(
    'CgtTY2hlbWVFbnRyeRISCgRuYW1lGAEgASgJUgRuYW1lEjkKCXNlbGVjdGlvbhgCIAEoCzIbLn'
    'lhdGEuY29yZS52MS5Tb3VsU2VsZWN0aW9uUglzZWxlY3Rpb24SNAoWaGFzX3Vua25vd25fY29u'
    'ZGl0aW9ucxgDIAEoCFIUaGFzVW5rbm93bkNvbmRpdGlvbnM=');

@$core.Deprecated('Use encodedSchemeDescriptor instead')
const EncodedScheme$json = {
  '1': 'EncodedScheme',
  '2': [
    {'1': 'text', '3': 1, '4': 1, '5': 9, '10': 'text'},
    {
      '1': 'qr',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QrMatrix',
      '10': 'qr'
    },
  ],
};

/// Descriptor for `EncodedScheme`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List encodedSchemeDescriptor = $convert.base64Decode(
    'Cg1FbmNvZGVkU2NoZW1lEhIKBHRleHQYASABKAlSBHRleHQSJgoCcXIYAiABKAsyFi55YXRhLm'
    'NvcmUudjEuUXJNYXRyaXhSAnFy');

@$core.Deprecated('Use qrMatrixDescriptor instead')
const QrMatrix$json = {
  '1': 'QrMatrix',
  '2': [
    {'1': 'size', '3': 1, '4': 1, '5': 13, '10': 'size'},
    {'1': 'modules', '3': 2, '4': 1, '5': 12, '10': 'modules'},
  ],
};

/// Descriptor for `QrMatrix`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List qrMatrixDescriptor = $convert.base64Decode(
    'CghRck1hdHJpeBISCgRzaXplGAEgASgNUgRzaXplEhgKB21vZHVsZXMYAiABKAxSB21vZHVsZX'
    'M=');

@$core.Deprecated('Use errorDescriptor instead')
const Error$json = {
  '1': 'Error',
  '2': [
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
    {
      '1': 'session_protocol_unsupported',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionProtocolUnsupported',
      '9': 0,
      '10': 'sessionProtocolUnsupported'
    },
    {
      '1': 'session_not_open',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionNotOpen',
      '9': 0,
      '10': 'sessionNotOpen'
    },
    {
      '1': 'session_already_open',
      '3': 12,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionAlreadyOpen',
      '9': 0,
      '10': 'sessionAlreadyOpen'
    },
    {
      '1': 'session_invalid_request_id',
      '3': 13,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionInvalidRequestId',
      '9': 0,
      '10': 'sessionInvalidRequestId'
    },
    {
      '1': 'session_unknown_request',
      '3': 14,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionUnknownRequest',
      '9': 0,
      '10': 'sessionUnknownRequest'
    },
    {
      '1': 'query_unknown_profile',
      '3': 20,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryUnknownProfile',
      '9': 0,
      '10': 'queryUnknownProfile'
    },
    {
      '1': 'query_stale_revision',
      '3': 21,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryStaleRevision',
      '9': 0,
      '10': 'queryStaleRevision'
    },
    {
      '1': 'query_malformed',
      '3': 22,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryMalformed',
      '9': 0,
      '10': 'queryMalformed'
    },
    {
      '1': 'query_unknown_field',
      '3': 23,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryUnknownField',
      '9': 0,
      '10': 'queryUnknownField'
    },
    {
      '1': 'query_too_complex',
      '3': 24,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryTooComplex',
      '9': 0,
      '10': 'queryTooComplex'
    },
    {
      '1': 'query_type_mismatch',
      '3': 25,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryTypeMismatch',
      '9': 0,
      '10': 'queryTypeMismatch'
    },
    {
      '1': 'query_param_set_required',
      '3': 26,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryParamSetRequired',
      '9': 0,
      '10': 'queryParamSetRequired'
    },
    {
      '1': 'query_field_unavailable',
      '3': 27,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryFieldUnavailable',
      '9': 0,
      '10': 'queryFieldUnavailable'
    },
    {
      '1': 'query_unknown_scheme',
      '3': 28,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryUnknownScheme',
      '9': 0,
      '10': 'queryUnknownScheme'
    },
    {
      '1': 'query_malformed_cursor',
      '3': 29,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryMalformedCursor',
      '9': 0,
      '10': 'queryMalformedCursor'
    },
    {
      '1': 'decode_no_input',
      '3': 40,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeNoInput',
      '9': 0,
      '10': 'decodeNoInput'
    },
    {
      '1': 'decode_malformed_text',
      '3': 41,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeMalformedText',
      '9': 0,
      '10': 'decodeMalformedText'
    },
    {
      '1': 'decode_unknown_format',
      '3': 42,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeUnknownFormat',
      '9': 0,
      '10': 'decodeUnknownFormat'
    },
    {
      '1': 'decode_malformed_layout',
      '3': 43,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeMalformedLayout',
      '9': 0,
      '10': 'decodeMalformedLayout'
    },
    {
      '1': 'decode_malformed_scheme',
      '3': 44,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeMalformedScheme',
      '9': 0,
      '10': 'decodeMalformedScheme'
    },
    {
      '1': 'decode_image_invalid',
      '3': 45,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeImageInvalid',
      '9': 0,
      '10': 'decodeImageInvalid'
    },
    {
      '1': 'decode_no_qr_code',
      '3': 46,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeNoQrCode',
      '9': 0,
      '10': 'decodeNoQrCode'
    },
    {
      '1': 'decode_several_qr_codes',
      '3': 47,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeSeveralQrCodes',
      '9': 0,
      '10': 'decodeSeveralQrCodes'
    },
    {
      '1': 'decode_qr_unreadable',
      '3': 48,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.DecodeQrUnreadable',
      '9': 0,
      '10': 'decodeQrUnreadable'
    },
    {
      '1': 'import_unknown_format',
      '3': 50,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportUnknownFormat',
      '9': 0,
      '10': 'importUnknownFormat'
    },
    {
      '1': 'import_ambiguous_format',
      '3': 51,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportAmbiguousFormat',
      '9': 0,
      '10': 'importAmbiguousFormat'
    },
    {
      '1': 'import_unsupported_version',
      '3': 52,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportUnsupportedVersion',
      '9': 0,
      '10': 'importUnsupportedVersion'
    },
    {
      '1': 'import_malformed_source',
      '3': 53,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportMalformedSource',
      '9': 0,
      '10': 'importMalformedSource'
    },
    {
      '1': 'import_normalization_failed',
      '3': 54,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportNormalizationFailed',
      '9': 0,
      '10': 'importNormalizationFailed'
    },
    {
      '1': 'import_unsupported_source_value',
      '3': 55,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportUnsupportedSourceValue',
      '9': 0,
      '10': 'importUnsupportedSourceValue'
    },
    {
      '1': 'import_inconsistent_reference',
      '3': 56,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportInconsistentReference',
      '9': 0,
      '10': 'importInconsistentReference'
    },
    {
      '1': 'import_admission_refused',
      '3': 57,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportAdmissionRefused',
      '9': 0,
      '10': 'importAdmissionRefused'
    },
    {
      '1': 'import_account_mismatch',
      '3': 58,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ImportAccountMismatch',
      '9': 0,
      '10': 'importAccountMismatch'
    },
    {
      '1': 'command_refused',
      '3': 70,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.CommandRefused',
      '9': 0,
      '10': 'commandRefused'
    },
    {
      '1': 'command_too_large',
      '3': 71,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.CommandTooLarge',
      '9': 0,
      '10': 'commandTooLarge'
    },
    {
      '1': 'store_failure',
      '3': 80,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreFailure',
      '9': 0,
      '10': 'storeFailure'
    },
    {
      '1': 'store_invalid_log',
      '3': 81,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreInvalidLog',
      '9': 0,
      '10': 'storeInvalidLog'
    },
    {
      '1': 'store_newer_format',
      '3': 82,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreNewerFormat',
      '9': 0,
      '10': 'storeNewerFormat'
    },
    {
      '1': 'store_malformed_commit',
      '3': 83,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreMalformedCommit',
      '9': 0,
      '10': 'storeMalformedCommit'
    },
    {
      '1': 'store_missing',
      '3': 84,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreMissing',
      '9': 0,
      '10': 'storeMissing'
    },
    {
      '1': 'store_not_a_database',
      '3': 85,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreNotADatabase',
      '9': 0,
      '10': 'storeNotADatabase'
    },
    {
      '1': 'store_foreign',
      '3': 86,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreForeign',
      '9': 0,
      '10': 'storeForeign'
    },
    {
      '1': 'store_no_format_version',
      '3': 87,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreNoFormatVersion',
      '9': 0,
      '10': 'storeNoFormatVersion'
    },
    {
      '1': 'store_damaged',
      '3': 88,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreDamaged',
      '9': 0,
      '10': 'storeDamaged'
    },
    {
      '1': 'store_uninitialized',
      '3': 89,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreUninitialized',
      '9': 0,
      '10': 'storeUninitialized'
    },
    {
      '1': 'store_retired_format',
      '3': 90,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.StoreRetiredFormat',
      '9': 0,
      '10': 'storeRetiredFormat'
    },
    {
      '1': 'internal_panic',
      '3': 100,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.InternalPanic',
      '9': 0,
      '10': 'internalPanic'
    },
    {
      '1': 'internal_qr_too_long',
      '3': 101,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.InternalQrTooLong',
      '9': 0,
      '10': 'internalQrTooLong'
    },
    {
      '1': 'internal_response_too_large',
      '3': 102,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.InternalResponseTooLarge',
      '9': 0,
      '10': 'internalResponseTooLarge'
    },
    {
      '1': 'internal_page_without_soul',
      '3': 103,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.InternalPageWithoutSoul',
      '9': 0,
      '10': 'internalPageWithoutSoul'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
  '9': [
    {'1': 1, '2': 2},
    {'1': 3, '2': 4},
    {'1': 60, '2': 65},
  ],
  '10': [
    'code',
    'details',
    'import_profile_mismatch',
    'import_malformed_reading',
    'import_reading_too_large',
    'import_unestablished_identity',
    'import_duplicate_soul'
  ],
};

/// Descriptor for `Error`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List errorDescriptor = $convert.base64Decode(
    'CgVFcnJvchIYCgdtZXNzYWdlGAIgASgJUgdtZXNzYWdlEmwKHHNlc3Npb25fcHJvdG9jb2xfdW'
    '5zdXBwb3J0ZWQYCiABKAsyKC55YXRhLmNvcmUudjEuU2Vzc2lvblByb3RvY29sVW5zdXBwb3J0'
    'ZWRIAFIac2Vzc2lvblByb3RvY29sVW5zdXBwb3J0ZWQSSAoQc2Vzc2lvbl9ub3Rfb3BlbhgLIA'
    'EoCzIcLnlhdGEuY29yZS52MS5TZXNzaW9uTm90T3BlbkgAUg5zZXNzaW9uTm90T3BlbhJUChRz'
    'ZXNzaW9uX2FscmVhZHlfb3BlbhgMIAEoCzIgLnlhdGEuY29yZS52MS5TZXNzaW9uQWxyZWFkeU'
    '9wZW5IAFISc2Vzc2lvbkFscmVhZHlPcGVuEmQKGnNlc3Npb25faW52YWxpZF9yZXF1ZXN0X2lk'
    'GA0gASgLMiUueWF0YS5jb3JlLnYxLlNlc3Npb25JbnZhbGlkUmVxdWVzdElkSABSF3Nlc3Npb2'
    '5JbnZhbGlkUmVxdWVzdElkEl0KF3Nlc3Npb25fdW5rbm93bl9yZXF1ZXN0GA4gASgLMiMueWF0'
    'YS5jb3JlLnYxLlNlc3Npb25Vbmtub3duUmVxdWVzdEgAUhVzZXNzaW9uVW5rbm93blJlcXVlc3'
    'QSVwoVcXVlcnlfdW5rbm93bl9wcm9maWxlGBQgASgLMiEueWF0YS5jb3JlLnYxLlF1ZXJ5VW5r'
    'bm93blByb2ZpbGVIAFITcXVlcnlVbmtub3duUHJvZmlsZRJUChRxdWVyeV9zdGFsZV9yZXZpc2'
    'lvbhgVIAEoCzIgLnlhdGEuY29yZS52MS5RdWVyeVN0YWxlUmV2aXNpb25IAFIScXVlcnlTdGFs'
    'ZVJldmlzaW9uEkcKD3F1ZXJ5X21hbGZvcm1lZBgWIAEoCzIcLnlhdGEuY29yZS52MS5RdWVyeU'
    '1hbGZvcm1lZEgAUg5xdWVyeU1hbGZvcm1lZBJRChNxdWVyeV91bmtub3duX2ZpZWxkGBcgASgL'
    'Mh8ueWF0YS5jb3JlLnYxLlF1ZXJ5VW5rbm93bkZpZWxkSABSEXF1ZXJ5VW5rbm93bkZpZWxkEk'
    'sKEXF1ZXJ5X3Rvb19jb21wbGV4GBggASgLMh0ueWF0YS5jb3JlLnYxLlF1ZXJ5VG9vQ29tcGxl'
    'eEgAUg9xdWVyeVRvb0NvbXBsZXgSUQoTcXVlcnlfdHlwZV9taXNtYXRjaBgZIAEoCzIfLnlhdG'
    'EuY29yZS52MS5RdWVyeVR5cGVNaXNtYXRjaEgAUhFxdWVyeVR5cGVNaXNtYXRjaBJeChhxdWVy'
    'eV9wYXJhbV9zZXRfcmVxdWlyZWQYGiABKAsyIy55YXRhLmNvcmUudjEuUXVlcnlQYXJhbVNldF'
    'JlcXVpcmVkSABSFXF1ZXJ5UGFyYW1TZXRSZXF1aXJlZBJdChdxdWVyeV9maWVsZF91bmF2YWls'
    'YWJsZRgbIAEoCzIjLnlhdGEuY29yZS52MS5RdWVyeUZpZWxkVW5hdmFpbGFibGVIAFIVcXVlcn'
    'lGaWVsZFVuYXZhaWxhYmxlElQKFHF1ZXJ5X3Vua25vd25fc2NoZW1lGBwgASgLMiAueWF0YS5j'
    'b3JlLnYxLlF1ZXJ5VW5rbm93blNjaGVtZUgAUhJxdWVyeVVua25vd25TY2hlbWUSWgoWcXVlcn'
    'lfbWFsZm9ybWVkX2N1cnNvchgdIAEoCzIiLnlhdGEuY29yZS52MS5RdWVyeU1hbGZvcm1lZEN1'
    'cnNvckgAUhRxdWVyeU1hbGZvcm1lZEN1cnNvchJFCg9kZWNvZGVfbm9faW5wdXQYKCABKAsyGy'
    '55YXRhLmNvcmUudjEuRGVjb2RlTm9JbnB1dEgAUg1kZWNvZGVOb0lucHV0ElcKFWRlY29kZV9t'
    'YWxmb3JtZWRfdGV4dBgpIAEoCzIhLnlhdGEuY29yZS52MS5EZWNvZGVNYWxmb3JtZWRUZXh0SA'
    'BSE2RlY29kZU1hbGZvcm1lZFRleHQSVwoVZGVjb2RlX3Vua25vd25fZm9ybWF0GCogASgLMiEu'
    'eWF0YS5jb3JlLnYxLkRlY29kZVVua25vd25Gb3JtYXRIAFITZGVjb2RlVW5rbm93bkZvcm1hdB'
    'JdChdkZWNvZGVfbWFsZm9ybWVkX2xheW91dBgrIAEoCzIjLnlhdGEuY29yZS52MS5EZWNvZGVN'
    'YWxmb3JtZWRMYXlvdXRIAFIVZGVjb2RlTWFsZm9ybWVkTGF5b3V0El0KF2RlY29kZV9tYWxmb3'
    'JtZWRfc2NoZW1lGCwgASgLMiMueWF0YS5jb3JlLnYxLkRlY29kZU1hbGZvcm1lZFNjaGVtZUgA'
    'UhVkZWNvZGVNYWxmb3JtZWRTY2hlbWUSVAoUZGVjb2RlX2ltYWdlX2ludmFsaWQYLSABKAsyIC'
    '55YXRhLmNvcmUudjEuRGVjb2RlSW1hZ2VJbnZhbGlkSABSEmRlY29kZUltYWdlSW52YWxpZBJJ'
    'ChFkZWNvZGVfbm9fcXJfY29kZRguIAEoCzIcLnlhdGEuY29yZS52MS5EZWNvZGVOb1FyQ29kZU'
    'gAUg5kZWNvZGVOb1FyQ29kZRJbChdkZWNvZGVfc2V2ZXJhbF9xcl9jb2RlcxgvIAEoCzIiLnlh'
    'dGEuY29yZS52MS5EZWNvZGVTZXZlcmFsUXJDb2Rlc0gAUhRkZWNvZGVTZXZlcmFsUXJDb2Rlcx'
    'JUChRkZWNvZGVfcXJfdW5yZWFkYWJsZRgwIAEoCzIgLnlhdGEuY29yZS52MS5EZWNvZGVRclVu'
    'cmVhZGFibGVIAFISZGVjb2RlUXJVbnJlYWRhYmxlElcKFWltcG9ydF91bmtub3duX2Zvcm1hdB'
    'gyIAEoCzIhLnlhdGEuY29yZS52MS5JbXBvcnRVbmtub3duRm9ybWF0SABSE2ltcG9ydFVua25v'
    'd25Gb3JtYXQSXQoXaW1wb3J0X2FtYmlndW91c19mb3JtYXQYMyABKAsyIy55YXRhLmNvcmUudj'
    'EuSW1wb3J0QW1iaWd1b3VzRm9ybWF0SABSFWltcG9ydEFtYmlndW91c0Zvcm1hdBJmChppbXBv'
    'cnRfdW5zdXBwb3J0ZWRfdmVyc2lvbhg0IAEoCzImLnlhdGEuY29yZS52MS5JbXBvcnRVbnN1cH'
    'BvcnRlZFZlcnNpb25IAFIYaW1wb3J0VW5zdXBwb3J0ZWRWZXJzaW9uEl0KF2ltcG9ydF9tYWxm'
    'b3JtZWRfc291cmNlGDUgASgLMiMueWF0YS5jb3JlLnYxLkltcG9ydE1hbGZvcm1lZFNvdXJjZU'
    'gAUhVpbXBvcnRNYWxmb3JtZWRTb3VyY2USaQobaW1wb3J0X25vcm1hbGl6YXRpb25fZmFpbGVk'
    'GDYgASgLMicueWF0YS5jb3JlLnYxLkltcG9ydE5vcm1hbGl6YXRpb25GYWlsZWRIAFIZaW1wb3'
    'J0Tm9ybWFsaXphdGlvbkZhaWxlZBJzCh9pbXBvcnRfdW5zdXBwb3J0ZWRfc291cmNlX3ZhbHVl'
    'GDcgASgLMioueWF0YS5jb3JlLnYxLkltcG9ydFVuc3VwcG9ydGVkU291cmNlVmFsdWVIAFIcaW'
    '1wb3J0VW5zdXBwb3J0ZWRTb3VyY2VWYWx1ZRJvCh1pbXBvcnRfaW5jb25zaXN0ZW50X3JlZmVy'
    'ZW5jZRg4IAEoCzIpLnlhdGEuY29yZS52MS5JbXBvcnRJbmNvbnNpc3RlbnRSZWZlcmVuY2VIAF'
    'IbaW1wb3J0SW5jb25zaXN0ZW50UmVmZXJlbmNlEmAKGGltcG9ydF9hZG1pc3Npb25fcmVmdXNl'
    'ZBg5IAEoCzIkLnlhdGEuY29yZS52MS5JbXBvcnRBZG1pc3Npb25SZWZ1c2VkSABSFmltcG9ydE'
    'FkbWlzc2lvblJlZnVzZWQSXQoXaW1wb3J0X2FjY291bnRfbWlzbWF0Y2gYOiABKAsyIy55YXRh'
    'LmNvcmUudjEuSW1wb3J0QWNjb3VudE1pc21hdGNoSABSFWltcG9ydEFjY291bnRNaXNtYXRjaB'
    'JHCg9jb21tYW5kX3JlZnVzZWQYRiABKAsyHC55YXRhLmNvcmUudjEuQ29tbWFuZFJlZnVzZWRI'
    'AFIOY29tbWFuZFJlZnVzZWQSSwoRY29tbWFuZF90b29fbGFyZ2UYRyABKAsyHS55YXRhLmNvcm'
    'UudjEuQ29tbWFuZFRvb0xhcmdlSABSD2NvbW1hbmRUb29MYXJnZRJBCg1zdG9yZV9mYWlsdXJl'
    'GFAgASgLMhoueWF0YS5jb3JlLnYxLlN0b3JlRmFpbHVyZUgAUgxzdG9yZUZhaWx1cmUSSwoRc3'
    'RvcmVfaW52YWxpZF9sb2cYUSABKAsyHS55YXRhLmNvcmUudjEuU3RvcmVJbnZhbGlkTG9nSABS'
    'D3N0b3JlSW52YWxpZExvZxJOChJzdG9yZV9uZXdlcl9mb3JtYXQYUiABKAsyHi55YXRhLmNvcm'
    'UudjEuU3RvcmVOZXdlckZvcm1hdEgAUhBzdG9yZU5ld2VyRm9ybWF0EloKFnN0b3JlX21hbGZv'
    'cm1lZF9jb21taXQYUyABKAsyIi55YXRhLmNvcmUudjEuU3RvcmVNYWxmb3JtZWRDb21taXRIAF'
    'IUc3RvcmVNYWxmb3JtZWRDb21taXQSQQoNc3RvcmVfbWlzc2luZxhUIAEoCzIaLnlhdGEuY29y'
    'ZS52MS5TdG9yZU1pc3NpbmdIAFIMc3RvcmVNaXNzaW5nElIKFHN0b3JlX25vdF9hX2RhdGFiYX'
    'NlGFUgASgLMh8ueWF0YS5jb3JlLnYxLlN0b3JlTm90QURhdGFiYXNlSABSEXN0b3JlTm90QURh'
    'dGFiYXNlEkEKDXN0b3JlX2ZvcmVpZ24YViABKAsyGi55YXRhLmNvcmUudjEuU3RvcmVGb3JlaW'
    'duSABSDHN0b3JlRm9yZWlnbhJbChdzdG9yZV9ub19mb3JtYXRfdmVyc2lvbhhXIAEoCzIiLnlh'
    'dGEuY29yZS52MS5TdG9yZU5vRm9ybWF0VmVyc2lvbkgAUhRzdG9yZU5vRm9ybWF0VmVyc2lvbh'
    'JBCg1zdG9yZV9kYW1hZ2VkGFggASgLMhoueWF0YS5jb3JlLnYxLlN0b3JlRGFtYWdlZEgAUgxz'
    'dG9yZURhbWFnZWQSUwoTc3RvcmVfdW5pbml0aWFsaXplZBhZIAEoCzIgLnlhdGEuY29yZS52MS'
    '5TdG9yZVVuaW5pdGlhbGl6ZWRIAFISc3RvcmVVbmluaXRpYWxpemVkElQKFHN0b3JlX3JldGly'
    'ZWRfZm9ybWF0GFogASgLMiAueWF0YS5jb3JlLnYxLlN0b3JlUmV0aXJlZEZvcm1hdEgAUhJzdG'
    '9yZVJldGlyZWRGb3JtYXQSRAoOaW50ZXJuYWxfcGFuaWMYZCABKAsyGy55YXRhLmNvcmUudjEu'
    'SW50ZXJuYWxQYW5pY0gAUg1pbnRlcm5hbFBhbmljElIKFGludGVybmFsX3FyX3Rvb19sb25nGG'
    'UgASgLMh8ueWF0YS5jb3JlLnYxLkludGVybmFsUXJUb29Mb25nSABSEWludGVybmFsUXJUb29M'
    'b25nEmcKG2ludGVybmFsX3Jlc3BvbnNlX3Rvb19sYXJnZRhmIAEoCzImLnlhdGEuY29yZS52MS'
    '5JbnRlcm5hbFJlc3BvbnNlVG9vTGFyZ2VIAFIYaW50ZXJuYWxSZXNwb25zZVRvb0xhcmdlEmQK'
    'GmludGVybmFsX3BhZ2Vfd2l0aG91dF9zb3VsGGcgASgLMiUueWF0YS5jb3JlLnYxLkludGVybm'
    'FsUGFnZVdpdGhvdXRTb3VsSABSF2ludGVybmFsUGFnZVdpdGhvdXRTb3VsQgYKBGtpbmRKBAgB'
    'EAJKBAgDEARKBAg8EEFSBGNvZGVSB2RldGFpbHNSF2ltcG9ydF9wcm9maWxlX21pc21hdGNoUh'
    'hpbXBvcnRfbWFsZm9ybWVkX3JlYWRpbmdSGGltcG9ydF9yZWFkaW5nX3Rvb19sYXJnZVIdaW1w'
    'b3J0X3VuZXN0YWJsaXNoZWRfaWRlbnRpdHlSFWltcG9ydF9kdXBsaWNhdGVfc291bA==');

@$core.Deprecated('Use sessionFailedDescriptor instead')
const SessionFailed$json = {
  '1': 'SessionFailed',
  '2': [
    {'1': 'message', '3': 1, '4': 1, '5': 9, '10': 'message'},
    {
      '1': 'session_malformed_frame',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionMalformedFrame',
      '9': 0,
      '10': 'sessionMalformedFrame'
    },
    {
      '1': 'session_malformed_message',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.SessionMalformedMessage',
      '9': 0,
      '10': 'sessionMalformedMessage'
    },
    {
      '1': 'internal_io',
      '3': 12,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.InternalIo',
      '9': 0,
      '10': 'internalIo'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
};

/// Descriptor for `SessionFailed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionFailedDescriptor = $convert.base64Decode(
    'Cg1TZXNzaW9uRmFpbGVkEhgKB21lc3NhZ2UYASABKAlSB21lc3NhZ2USXQoXc2Vzc2lvbl9tYW'
    'xmb3JtZWRfZnJhbWUYCiABKAsyIy55YXRhLmNvcmUudjEuU2Vzc2lvbk1hbGZvcm1lZEZyYW1l'
    'SABSFXNlc3Npb25NYWxmb3JtZWRGcmFtZRJjChlzZXNzaW9uX21hbGZvcm1lZF9tZXNzYWdlGA'
    'sgASgLMiUueWF0YS5jb3JlLnYxLlNlc3Npb25NYWxmb3JtZWRNZXNzYWdlSABSF3Nlc3Npb25N'
    'YWxmb3JtZWRNZXNzYWdlEjsKC2ludGVybmFsX2lvGAwgASgLMhgueWF0YS5jb3JlLnYxLkludG'
    'VybmFsSW9IAFIKaW50ZXJuYWxJb0IGCgRraW5k');

@$core.Deprecated('Use clientFailureDescriptor instead')
const ClientFailure$json = {
  '1': 'ClientFailure',
  '2': [
    {
      '1': 'client_daemon_not_found',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientDaemonNotFound',
      '9': 0,
      '10': 'clientDaemonNotFound'
    },
    {
      '1': 'client_daemon_start_failed',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientDaemonStartFailed',
      '9': 0,
      '10': 'clientDaemonStartFailed'
    },
    {
      '1': 'client_daemon_exited',
      '3': 12,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientDaemonExited',
      '9': 0,
      '10': 'clientDaemonExited'
    },
    {
      '1': 'client_timeout',
      '3': 13,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientTimeout',
      '9': 0,
      '10': 'clientTimeout'
    },
    {
      '1': 'client_protocol_error',
      '3': 14,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientProtocolError',
      '9': 0,
      '10': 'clientProtocolError'
    },
    {
      '1': 'client_not_connected',
      '3': 15,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientNotConnected',
      '9': 0,
      '10': 'clientNotConnected'
    },
    {
      '1': 'client_unexpected',
      '3': 16,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientUnexpected',
      '9': 0,
      '10': 'clientUnexpected'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
};

/// Descriptor for `ClientFailure`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientFailureDescriptor = $convert.base64Decode(
    'Cg1DbGllbnRGYWlsdXJlElsKF2NsaWVudF9kYWVtb25fbm90X2ZvdW5kGAogASgLMiIueWF0YS'
    '5jb3JlLnYxLkNsaWVudERhZW1vbk5vdEZvdW5kSABSFGNsaWVudERhZW1vbk5vdEZvdW5kEmQK'
    'GmNsaWVudF9kYWVtb25fc3RhcnRfZmFpbGVkGAsgASgLMiUueWF0YS5jb3JlLnYxLkNsaWVudE'
    'RhZW1vblN0YXJ0RmFpbGVkSABSF2NsaWVudERhZW1vblN0YXJ0RmFpbGVkElQKFGNsaWVudF9k'
    'YWVtb25fZXhpdGVkGAwgASgLMiAueWF0YS5jb3JlLnYxLkNsaWVudERhZW1vbkV4aXRlZEgAUh'
    'JjbGllbnREYWVtb25FeGl0ZWQSRAoOY2xpZW50X3RpbWVvdXQYDSABKAsyGy55YXRhLmNvcmUu'
    'djEuQ2xpZW50VGltZW91dEgAUg1jbGllbnRUaW1lb3V0ElcKFWNsaWVudF9wcm90b2NvbF9lcn'
    'JvchgOIAEoCzIhLnlhdGEuY29yZS52MS5DbGllbnRQcm90b2NvbEVycm9ySABSE2NsaWVudFBy'
    'b3RvY29sRXJyb3ISVAoUY2xpZW50X25vdF9jb25uZWN0ZWQYDyABKAsyIC55YXRhLmNvcmUudj'
    'EuQ2xpZW50Tm90Q29ubmVjdGVkSABSEmNsaWVudE5vdENvbm5lY3RlZBJNChFjbGllbnRfdW5l'
    'eHBlY3RlZBgQIAEoCzIeLnlhdGEuY29yZS52MS5DbGllbnRVbmV4cGVjdGVkSABSEGNsaWVudF'
    'VuZXhwZWN0ZWRCBgoEa2luZA==');

@$core.Deprecated('Use sessionProtocolUnsupportedDescriptor instead')
const SessionProtocolUnsupported$json = {
  '1': 'SessionProtocolUnsupported',
  '2': [
    {
      '1': 'client',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'client'
    },
    {
      '1': 'daemon',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'daemon'
    },
  ],
};

/// Descriptor for `SessionProtocolUnsupported`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionProtocolUnsupportedDescriptor =
    $convert.base64Decode(
        'ChpTZXNzaW9uUHJvdG9jb2xVbnN1cHBvcnRlZBI1CgZjbGllbnQYASABKAsyHS55YXRhLmNvcm'
        'UudjEuUHJvdG9jb2xWZXJzaW9uUgZjbGllbnQSNQoGZGFlbW9uGAIgASgLMh0ueWF0YS5jb3Jl'
        'LnYxLlByb3RvY29sVmVyc2lvblIGZGFlbW9u');

@$core.Deprecated('Use sessionNotOpenDescriptor instead')
const SessionNotOpen$json = {
  '1': 'SessionNotOpen',
  '2': [
    {'1': 'request', '3': 1, '4': 1, '5': 9, '10': 'request'},
  ],
};

/// Descriptor for `SessionNotOpen`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionNotOpenDescriptor = $convert
    .base64Decode('Cg5TZXNzaW9uTm90T3BlbhIYCgdyZXF1ZXN0GAEgASgJUgdyZXF1ZXN0');

@$core.Deprecated('Use sessionAlreadyOpenDescriptor instead')
const SessionAlreadyOpen$json = {
  '1': 'SessionAlreadyOpen',
};

/// Descriptor for `SessionAlreadyOpen`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionAlreadyOpenDescriptor =
    $convert.base64Decode('ChJTZXNzaW9uQWxyZWFkeU9wZW4=');

@$core.Deprecated('Use sessionInvalidRequestIdDescriptor instead')
const SessionInvalidRequestId$json = {
  '1': 'SessionInvalidRequestId',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 4, '10': 'id'},
  ],
};

/// Descriptor for `SessionInvalidRequestId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionInvalidRequestIdDescriptor = $convert
    .base64Decode('ChdTZXNzaW9uSW52YWxpZFJlcXVlc3RJZBIOCgJpZBgBIAEoBFICaWQ=');

@$core.Deprecated('Use sessionUnknownRequestDescriptor instead')
const SessionUnknownRequest$json = {
  '1': 'SessionUnknownRequest',
};

/// Descriptor for `SessionUnknownRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionUnknownRequestDescriptor =
    $convert.base64Decode('ChVTZXNzaW9uVW5rbm93blJlcXVlc3Q=');

@$core.Deprecated('Use queryUnknownProfileDescriptor instead')
const QueryUnknownProfile$json = {
  '1': 'QueryUnknownProfile',
  '2': [
    {'1': 'profile_id', '3': 1, '4': 1, '5': 9, '10': 'profileId'},
  ],
};

/// Descriptor for `QueryUnknownProfile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryUnknownProfileDescriptor = $convert.base64Decode(
    'ChNRdWVyeVVua25vd25Qcm9maWxlEh0KCnByb2ZpbGVfaWQYASABKAlSCXByb2ZpbGVJZA==');

@$core.Deprecated('Use queryStaleRevisionDescriptor instead')
const QueryStaleRevision$json = {
  '1': 'QueryStaleRevision',
  '2': [
    {'1': 'scan', '3': 1, '4': 1, '5': 4, '10': 'scan'},
    {'1': 'current', '3': 2, '4': 1, '5': 4, '10': 'current'},
  ],
};

/// Descriptor for `QueryStaleRevision`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryStaleRevisionDescriptor = $convert.base64Decode(
    'ChJRdWVyeVN0YWxlUmV2aXNpb24SEgoEc2NhbhgBIAEoBFIEc2NhbhIYCgdjdXJyZW50GAIgAS'
    'gEUgdjdXJyZW50');

@$core.Deprecated('Use queryMalformedDescriptor instead')
const QueryMalformed$json = {
  '1': 'QueryMalformed',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `QueryMalformed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryMalformedDescriptor = $convert
    .base64Decode('Cg5RdWVyeU1hbGZvcm1lZBIYCgdwcm9ibGVtGAEgASgJUgdwcm9ibGVt');

@$core.Deprecated('Use queryUnknownFieldDescriptor instead')
const QueryUnknownField$json = {
  '1': 'QueryUnknownField',
  '2': [
    {'1': 'value', '3': 1, '4': 1, '5': 5, '10': 'value'},
  ],
};

/// Descriptor for `QueryUnknownField`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryUnknownFieldDescriptor = $convert
    .base64Decode('ChFRdWVyeVVua25vd25GaWVsZBIUCgV2YWx1ZRgBIAEoBVIFdmFsdWU=');

@$core.Deprecated('Use queryTooComplexDescriptor instead')
const QueryTooComplex$json = {
  '1': 'QueryTooComplex',
  '2': [
    {'1': 'limit', '3': 1, '4': 1, '5': 9, '10': 'limit'},
    {'1': 'found', '3': 2, '4': 1, '5': 4, '10': 'found'},
    {'1': 'max', '3': 3, '4': 1, '5': 4, '10': 'max'},
  ],
};

/// Descriptor for `QueryTooComplex`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryTooComplexDescriptor = $convert.base64Decode(
    'Cg9RdWVyeVRvb0NvbXBsZXgSFAoFbGltaXQYASABKAlSBWxpbWl0EhQKBWZvdW5kGAIgASgEUg'
    'Vmb3VuZBIQCgNtYXgYAyABKARSA21heA==');

@$core.Deprecated('Use queryTypeMismatchDescriptor instead')
const QueryTypeMismatch$json = {
  '1': 'QueryTypeMismatch',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `QueryTypeMismatch`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryTypeMismatchDescriptor = $convert.base64Decode(
    'ChFRdWVyeVR5cGVNaXNtYXRjaBIYCgdwcm9ibGVtGAEgASgJUgdwcm9ibGVt');

@$core.Deprecated('Use queryParamSetRequiredDescriptor instead')
const QueryParamSetRequired$json = {
  '1': 'QueryParamSetRequired',
  '2': [
    {'1': 'field', '3': 1, '4': 1, '5': 9, '10': 'field'},
  ],
};

/// Descriptor for `QueryParamSetRequired`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryParamSetRequiredDescriptor =
    $convert.base64Decode(
        'ChVRdWVyeVBhcmFtU2V0UmVxdWlyZWQSFAoFZmllbGQYASABKAlSBWZpZWxk');

@$core.Deprecated('Use queryFieldUnavailableDescriptor instead')
const QueryFieldUnavailable$json = {
  '1': 'QueryFieldUnavailable',
  '2': [
    {'1': 'field', '3': 1, '4': 1, '5': 9, '10': 'field'},
  ],
};

/// Descriptor for `QueryFieldUnavailable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryFieldUnavailableDescriptor =
    $convert.base64Decode(
        'ChVRdWVyeUZpZWxkVW5hdmFpbGFibGUSFAoFZmllbGQYASABKAlSBWZpZWxk');

@$core.Deprecated('Use queryUnknownSchemeDescriptor instead')
const QueryUnknownScheme$json = {
  '1': 'QueryUnknownScheme',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `QueryUnknownScheme`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryUnknownSchemeDescriptor =
    $convert.base64Decode(
        'ChJRdWVyeVVua25vd25TY2hlbWUSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use queryMalformedCursorDescriptor instead')
const QueryMalformedCursor$json = {
  '1': 'QueryMalformedCursor',
};

/// Descriptor for `QueryMalformedCursor`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryMalformedCursorDescriptor =
    $convert.base64Decode('ChRRdWVyeU1hbGZvcm1lZEN1cnNvcg==');

@$core.Deprecated('Use decodeNoInputDescriptor instead')
const DecodeNoInput$json = {
  '1': 'DecodeNoInput',
};

/// Descriptor for `DecodeNoInput`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeNoInputDescriptor =
    $convert.base64Decode('Cg1EZWNvZGVOb0lucHV0');

@$core.Deprecated('Use decodeMalformedTextDescriptor instead')
const DecodeMalformedText$json = {
  '1': 'DecodeMalformedText',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `DecodeMalformedText`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeMalformedTextDescriptor =
    $convert.base64Decode(
        'ChNEZWNvZGVNYWxmb3JtZWRUZXh0EhgKB3Byb2JsZW0YASABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use decodeUnknownFormatDescriptor instead')
const DecodeUnknownFormat$json = {
  '1': 'DecodeUnknownFormat',
  '2': [
    {'1': 'payload_bytes', '3': 1, '4': 1, '5': 4, '10': 'payloadBytes'},
  ],
};

/// Descriptor for `DecodeUnknownFormat`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeUnknownFormatDescriptor = $convert.base64Decode(
    'ChNEZWNvZGVVbmtub3duRm9ybWF0EiMKDXBheWxvYWRfYnl0ZXMYASABKARSDHBheWxvYWRCeX'
    'Rlcw==');

@$core.Deprecated('Use decodeMalformedLayoutDescriptor instead')
const DecodeMalformedLayout$json = {
  '1': 'DecodeMalformedLayout',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `DecodeMalformedLayout`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeMalformedLayoutDescriptor =
    $convert.base64Decode(
        'ChVEZWNvZGVNYWxmb3JtZWRMYXlvdXQSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use decodeMalformedSchemeDescriptor instead')
const DecodeMalformedScheme$json = {
  '1': 'DecodeMalformedScheme',
  '2': [
    {
      '1': 'record',
      '3': 1,
      '4': 1,
      '5': 13,
      '9': 0,
      '10': 'record',
      '17': true
    },
    {'1': 'problem', '3': 2, '4': 1, '5': 9, '10': 'problem'},
  ],
  '8': [
    {'1': '_record'},
  ],
};

/// Descriptor for `DecodeMalformedScheme`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeMalformedSchemeDescriptor = $convert.base64Decode(
    'ChVEZWNvZGVNYWxmb3JtZWRTY2hlbWUSGwoGcmVjb3JkGAEgASgNSABSBnJlY29yZIgBARIYCg'
    'dwcm9ibGVtGAIgASgJUgdwcm9ibGVtQgkKB19yZWNvcmQ=');

@$core.Deprecated('Use decodeImageInvalidDescriptor instead')
const DecodeImageInvalid$json = {
  '1': 'DecodeImageInvalid',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `DecodeImageInvalid`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeImageInvalidDescriptor =
    $convert.base64Decode(
        'ChJEZWNvZGVJbWFnZUludmFsaWQSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use decodeNoQrCodeDescriptor instead')
const DecodeNoQrCode$json = {
  '1': 'DecodeNoQrCode',
};

/// Descriptor for `DecodeNoQrCode`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeNoQrCodeDescriptor =
    $convert.base64Decode('Cg5EZWNvZGVOb1FyQ29kZQ==');

@$core.Deprecated('Use decodeSeveralQrCodesDescriptor instead')
const DecodeSeveralQrCodes$json = {
  '1': 'DecodeSeveralQrCodes',
  '2': [
    {'1': 'count', '3': 1, '4': 1, '5': 13, '10': 'count'},
  ],
};

/// Descriptor for `DecodeSeveralQrCodes`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeSeveralQrCodesDescriptor =
    $convert.base64Decode(
        'ChREZWNvZGVTZXZlcmFsUXJDb2RlcxIUCgVjb3VudBgBIAEoDVIFY291bnQ=');

@$core.Deprecated('Use decodeQrUnreadableDescriptor instead')
const DecodeQrUnreadable$json = {
  '1': 'DecodeQrUnreadable',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `DecodeQrUnreadable`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decodeQrUnreadableDescriptor =
    $convert.base64Decode(
        'ChJEZWNvZGVRclVucmVhZGFibGUSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use importUnknownFormatDescriptor instead')
const ImportUnknownFormat$json = {
  '1': 'ImportUnknownFormat',
  '2': [
    {'1': 'stated', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'stated', '17': true},
  ],
  '8': [
    {'1': '_stated'},
  ],
};

/// Descriptor for `ImportUnknownFormat`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importUnknownFormatDescriptor = $convert.base64Decode(
    'ChNJbXBvcnRVbmtub3duRm9ybWF0EhsKBnN0YXRlZBgBIAEoCUgAUgZzdGF0ZWSIAQFCCQoHX3'
    'N0YXRlZA==');

@$core.Deprecated('Use importAmbiguousFormatDescriptor instead')
const ImportAmbiguousFormat$json = {
  '1': 'ImportAmbiguousFormat',
  '2': [
    {'1': 'formats', '3': 1, '4': 3, '5': 9, '10': 'formats'},
  ],
};

/// Descriptor for `ImportAmbiguousFormat`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importAmbiguousFormatDescriptor =
    $convert.base64Decode(
        'ChVJbXBvcnRBbWJpZ3VvdXNGb3JtYXQSGAoHZm9ybWF0cxgBIAMoCVIHZm9ybWF0cw==');

@$core.Deprecated('Use importUnsupportedVersionDescriptor instead')
const ImportUnsupportedVersion$json = {
  '1': 'ImportUnsupportedVersion',
  '2': [
    {'1': 'format', '3': 1, '4': 1, '5': 9, '10': 'format'},
    {'1': 'version', '3': 2, '4': 1, '5': 9, '10': 'version'},
  ],
};

/// Descriptor for `ImportUnsupportedVersion`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importUnsupportedVersionDescriptor =
    $convert.base64Decode(
        'ChhJbXBvcnRVbnN1cHBvcnRlZFZlcnNpb24SFgoGZm9ybWF0GAEgASgJUgZmb3JtYXQSGAoHdm'
        'Vyc2lvbhgCIAEoCVIHdmVyc2lvbg==');

@$core.Deprecated('Use importMalformedSourceDescriptor instead')
const ImportMalformedSource$json = {
  '1': 'ImportMalformedSource',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ImportMalformedSource`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importMalformedSourceDescriptor =
    $convert.base64Decode(
        'ChVJbXBvcnRNYWxmb3JtZWRTb3VyY2USGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use importNormalizationFailedDescriptor instead')
const ImportNormalizationFailed$json = {
  '1': 'ImportNormalizationFailed',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ImportNormalizationFailed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importNormalizationFailedDescriptor =
    $convert.base64Decode(
        'ChlJbXBvcnROb3JtYWxpemF0aW9uRmFpbGVkEhgKB3Byb2JsZW0YASABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use importUnsupportedSourceValueDescriptor instead')
const ImportUnsupportedSourceValue$json = {
  '1': 'ImportUnsupportedSourceValue',
  '2': [
    {'1': 'field', '3': 1, '4': 1, '5': 9, '10': 'field'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
};

/// Descriptor for `ImportUnsupportedSourceValue`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importUnsupportedSourceValueDescriptor =
    $convert.base64Decode(
        'ChxJbXBvcnRVbnN1cHBvcnRlZFNvdXJjZVZhbHVlEhQKBWZpZWxkGAEgASgJUgVmaWVsZBIUCg'
        'V2YWx1ZRgCIAEoCVIFdmFsdWU=');

@$core.Deprecated('Use importInconsistentReferenceDescriptor instead')
const ImportInconsistentReference$json = {
  '1': 'ImportInconsistentReference',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ImportInconsistentReference`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importInconsistentReferenceDescriptor =
    $convert.base64Decode(
        'ChtJbXBvcnRJbmNvbnNpc3RlbnRSZWZlcmVuY2USGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ'
        '==');

@$core.Deprecated('Use importAdmissionRefusedDescriptor instead')
const ImportAdmissionRefused$json = {
  '1': 'ImportAdmissionRefused',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ImportAdmissionRefused`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importAdmissionRefusedDescriptor =
    $convert.base64Decode(
        'ChZJbXBvcnRBZG1pc3Npb25SZWZ1c2VkEhgKB3Byb2JsZW0YASABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use importAccountMismatchDescriptor instead')
const ImportAccountMismatch$json = {
  '1': 'ImportAccountMismatch',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ImportAccountMismatch`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List importAccountMismatchDescriptor =
    $convert.base64Decode(
        'ChVJbXBvcnRBY2NvdW50TWlzbWF0Y2gSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use commandRefusedDescriptor instead')
const CommandRefused$json = {
  '1': 'CommandRefused',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `CommandRefused`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List commandRefusedDescriptor = $convert
    .base64Decode('Cg5Db21tYW5kUmVmdXNlZBIYCgdwcm9ibGVtGAEgASgJUgdwcm9ibGVt');

@$core.Deprecated('Use commandTooLargeDescriptor instead')
const CommandTooLarge$json = {
  '1': 'CommandTooLarge',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `CommandTooLarge`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List commandTooLargeDescriptor = $convert.base64Decode(
    'Cg9Db21tYW5kVG9vTGFyZ2USGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use storeFailureDescriptor instead')
const StoreFailure$json = {
  '1': 'StoreFailure',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `StoreFailure`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeFailureDescriptor = $convert
    .base64Decode('CgxTdG9yZUZhaWx1cmUSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use storeInvalidLogDescriptor instead')
const StoreInvalidLog$json = {
  '1': 'StoreInvalidLog',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `StoreInvalidLog`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeInvalidLogDescriptor = $convert.base64Decode(
    'Cg9TdG9yZUludmFsaWRMb2cSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use storeNewerFormatDescriptor instead')
const StoreNewerFormat$json = {
  '1': 'StoreNewerFormat',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `StoreNewerFormat`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeNewerFormatDescriptor = $convert.base64Decode(
    'ChBTdG9yZU5ld2VyRm9ybWF0EhgKB3Byb2JsZW0YASABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use storeMalformedCommitDescriptor instead')
const StoreMalformedCommit$json = {
  '1': 'StoreMalformedCommit',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `StoreMalformedCommit`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeMalformedCommitDescriptor =
    $convert.base64Decode(
        'ChRTdG9yZU1hbGZvcm1lZENvbW1pdBIYCgdwcm9ibGVtGAEgASgJUgdwcm9ibGVt');

@$core.Deprecated('Use storeMissingDescriptor instead')
const StoreMissing$json = {
  '1': 'StoreMissing',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `StoreMissing`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeMissingDescriptor =
    $convert.base64Decode('CgxTdG9yZU1pc3NpbmcSEgoEcGF0aBgBIAEoCVIEcGF0aA==');

@$core.Deprecated('Use storeNotADatabaseDescriptor instead')
const StoreNotADatabase$json = {
  '1': 'StoreNotADatabase',
};

/// Descriptor for `StoreNotADatabase`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeNotADatabaseDescriptor =
    $convert.base64Decode('ChFTdG9yZU5vdEFEYXRhYmFzZQ==');

@$core.Deprecated('Use storeForeignDescriptor instead')
const StoreForeign$json = {
  '1': 'StoreForeign',
  '2': [
    {'1': 'application_id', '3': 1, '4': 1, '5': 3, '10': 'applicationId'},
    {'1': 'objects', '3': 2, '4': 1, '5': 3, '10': 'objects'},
  ],
};

/// Descriptor for `StoreForeign`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeForeignDescriptor = $convert.base64Decode(
    'CgxTdG9yZUZvcmVpZ24SJQoOYXBwbGljYXRpb25faWQYASABKANSDWFwcGxpY2F0aW9uSWQSGA'
    'oHb2JqZWN0cxgCIAEoA1IHb2JqZWN0cw==');

@$core.Deprecated('Use storeNoFormatVersionDescriptor instead')
const StoreNoFormatVersion$json = {
  '1': 'StoreNoFormatVersion',
};

/// Descriptor for `StoreNoFormatVersion`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeNoFormatVersionDescriptor =
    $convert.base64Decode('ChRTdG9yZU5vRm9ybWF0VmVyc2lvbg==');

@$core.Deprecated('Use storeDamagedDescriptor instead')
const StoreDamaged$json = {
  '1': 'StoreDamaged',
  '2': [
    {'1': 'problems', '3': 1, '4': 3, '5': 9, '10': 'problems'},
  ],
};

/// Descriptor for `StoreDamaged`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeDamagedDescriptor = $convert
    .base64Decode('CgxTdG9yZURhbWFnZWQSGgoIcHJvYmxlbXMYASADKAlSCHByb2JsZW1z');

@$core.Deprecated('Use storeUninitializedDescriptor instead')
const StoreUninitialized$json = {
  '1': 'StoreUninitialized',
};

/// Descriptor for `StoreUninitialized`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeUninitializedDescriptor =
    $convert.base64Decode('ChJTdG9yZVVuaW5pdGlhbGl6ZWQ=');

@$core.Deprecated('Use storeRetiredFormatDescriptor instead')
const StoreRetiredFormat$json = {
  '1': 'StoreRetiredFormat',
  '2': [
    {'1': 'found', '3': 1, '4': 1, '5': 13, '10': 'found'},
  ],
};

/// Descriptor for `StoreRetiredFormat`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List storeRetiredFormatDescriptor = $convert
    .base64Decode('ChJTdG9yZVJldGlyZWRGb3JtYXQSFAoFZm91bmQYASABKA1SBWZvdW5k');

@$core.Deprecated('Use internalPanicDescriptor instead')
const InternalPanic$json = {
  '1': 'InternalPanic',
};

/// Descriptor for `InternalPanic`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internalPanicDescriptor =
    $convert.base64Decode('Cg1JbnRlcm5hbFBhbmlj');

@$core.Deprecated('Use internalQrTooLongDescriptor instead')
const InternalQrTooLong$json = {
  '1': 'InternalQrTooLong',
  '2': [
    {'1': 'bits', '3': 1, '4': 1, '5': 4, '10': 'bits'},
    {'1': 'capacity_bits', '3': 2, '4': 1, '5': 4, '10': 'capacityBits'},
  ],
};

/// Descriptor for `InternalQrTooLong`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internalQrTooLongDescriptor = $convert.base64Decode(
    'ChFJbnRlcm5hbFFyVG9vTG9uZxISCgRiaXRzGAEgASgEUgRiaXRzEiMKDWNhcGFjaXR5X2JpdH'
    'MYAiABKARSDGNhcGFjaXR5Qml0cw==');

@$core.Deprecated('Use internalResponseTooLargeDescriptor instead')
const InternalResponseTooLarge$json = {
  '1': 'InternalResponseTooLarge',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `InternalResponseTooLarge`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internalResponseTooLargeDescriptor =
    $convert.base64Decode(
        'ChhJbnRlcm5hbFJlc3BvbnNlVG9vTGFyZ2USGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use internalPageWithoutSoulDescriptor instead')
const InternalPageWithoutSoul$json = {
  '1': 'InternalPageWithoutSoul',
  '2': [
    {'1': 'soul_id', '3': 1, '4': 1, '5': 9, '10': 'soulId'},
  ],
};

/// Descriptor for `InternalPageWithoutSoul`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internalPageWithoutSoulDescriptor =
    $convert.base64Decode(
        'ChdJbnRlcm5hbFBhZ2VXaXRob3V0U291bBIXCgdzb3VsX2lkGAEgASgJUgZzb3VsSWQ=');

@$core.Deprecated('Use sessionMalformedFrameDescriptor instead')
const SessionMalformedFrame$json = {
  '1': 'SessionMalformedFrame',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `SessionMalformedFrame`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionMalformedFrameDescriptor =
    $convert.base64Decode(
        'ChVTZXNzaW9uTWFsZm9ybWVkRnJhbWUSGAoHcHJvYmxlbRgBIAEoCVIHcHJvYmxlbQ==');

@$core.Deprecated('Use sessionMalformedMessageDescriptor instead')
const SessionMalformedMessage$json = {
  '1': 'SessionMalformedMessage',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `SessionMalformedMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionMalformedMessageDescriptor =
    $convert.base64Decode(
        'ChdTZXNzaW9uTWFsZm9ybWVkTWVzc2FnZRIYCgdwcm9ibGVtGAEgASgJUgdwcm9ibGVt');

@$core.Deprecated('Use internalIoDescriptor instead')
const InternalIo$json = {
  '1': 'InternalIo',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `InternalIo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List internalIoDescriptor = $convert
    .base64Decode('CgpJbnRlcm5hbElvEhgKB3Byb2JsZW0YASABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use clientDaemonNotFoundDescriptor instead')
const ClientDaemonNotFound$json = {
  '1': 'ClientDaemonNotFound',
  '2': [
    {'1': 'searched', '3': 1, '4': 3, '5': 9, '10': 'searched'},
  ],
};

/// Descriptor for `ClientDaemonNotFound`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientDaemonNotFoundDescriptor =
    $convert.base64Decode(
        'ChRDbGllbnREYWVtb25Ob3RGb3VuZBIaCghzZWFyY2hlZBgBIAMoCVIIc2VhcmNoZWQ=');

@$core.Deprecated('Use clientDaemonStartFailedDescriptor instead')
const ClientDaemonStartFailed$json = {
  '1': 'ClientDaemonStartFailed',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'problem', '3': 2, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ClientDaemonStartFailed`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientDaemonStartFailedDescriptor =
    $convert.base64Decode(
        'ChdDbGllbnREYWVtb25TdGFydEZhaWxlZBISCgRwYXRoGAEgASgJUgRwYXRoEhgKB3Byb2JsZW'
        '0YAiABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use clientDaemonExitedDescriptor instead')
const ClientDaemonExited$json = {
  '1': 'ClientDaemonExited',
  '2': [
    {'1': 'exit_code', '3': 1, '4': 1, '5': 5, '10': 'exitCode'},
    {
      '1': 'last_log_line',
      '3': 2,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'lastLogLine',
      '17': true
    },
  ],
  '8': [
    {'1': '_last_log_line'},
  ],
};

/// Descriptor for `ClientDaemonExited`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientDaemonExitedDescriptor = $convert.base64Decode(
    'ChJDbGllbnREYWVtb25FeGl0ZWQSGwoJZXhpdF9jb2RlGAEgASgFUghleGl0Q29kZRInCg1sYX'
    'N0X2xvZ19saW5lGAIgASgJSABSC2xhc3RMb2dMaW5liAEBQhAKDl9sYXN0X2xvZ19saW5l');

@$core.Deprecated('Use clientTimeoutDescriptor instead')
const ClientTimeout$json = {
  '1': 'ClientTimeout',
  '2': [
    {'1': 'request_id', '3': 1, '4': 1, '5': 4, '10': 'requestId'},
    {'1': 'limit_ms', '3': 2, '4': 1, '5': 4, '10': 'limitMs'},
  ],
};

/// Descriptor for `ClientTimeout`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientTimeoutDescriptor = $convert.base64Decode(
    'Cg1DbGllbnRUaW1lb3V0Eh0KCnJlcXVlc3RfaWQYASABKARSCXJlcXVlc3RJZBIZCghsaW1pdF'
    '9tcxgCIAEoBFIHbGltaXRNcw==');

@$core.Deprecated('Use clientProtocolErrorDescriptor instead')
const ClientProtocolError$json = {
  '1': 'ClientProtocolError',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ClientProtocolError`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientProtocolErrorDescriptor =
    $convert.base64Decode(
        'ChNDbGllbnRQcm90b2NvbEVycm9yEhgKB3Byb2JsZW0YASABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use clientNotConnectedDescriptor instead')
const ClientNotConnected$json = {
  '1': 'ClientNotConnected',
};

/// Descriptor for `ClientNotConnected`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientNotConnectedDescriptor =
    $convert.base64Decode('ChJDbGllbnROb3RDb25uZWN0ZWQ=');

@$core.Deprecated('Use clientUnexpectedDescriptor instead')
const ClientUnexpected$json = {
  '1': 'ClientUnexpected',
  '2': [
    {'1': 'problem', '3': 1, '4': 1, '5': 9, '10': 'problem'},
  ],
};

/// Descriptor for `ClientUnexpected`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientUnexpectedDescriptor = $convert.base64Decode(
    'ChBDbGllbnRVbmV4cGVjdGVkEhgKB3Byb2JsZW0YASABKAlSB3Byb2JsZW0=');

@$core.Deprecated('Use warningDescriptor instead')
const Warning$json = {
  '1': 'Warning',
  '2': [
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
    {
      '1': 'client_outdated',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ClientOutdated',
      '9': 0,
      '10': 'clientOutdated'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
  '9': [
    {'1': 1, '2': 2},
  ],
  '10': ['code'],
};

/// Descriptor for `Warning`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List warningDescriptor = $convert.base64Decode(
    'CgdXYXJuaW5nEhgKB21lc3NhZ2UYAiABKAlSB21lc3NhZ2USRwoPY2xpZW50X291dGRhdGVkGA'
    'ogASgLMhwueWF0YS5jb3JlLnYxLkNsaWVudE91dGRhdGVkSABSDmNsaWVudE91dGRhdGVkQgYK'
    'BGtpbmRKBAgBEAJSBGNvZGU=');

@$core.Deprecated('Use clientOutdatedDescriptor instead')
const ClientOutdated$json = {
  '1': 'ClientOutdated',
  '2': [
    {
      '1': 'client',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'client'
    },
    {
      '1': 'daemon',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'daemon'
    },
  ],
};

/// Descriptor for `ClientOutdated`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientOutdatedDescriptor = $convert.base64Decode(
    'Cg5DbGllbnRPdXRkYXRlZBI1CgZjbGllbnQYASABKAsyHS55YXRhLmNvcmUudjEuUHJvdG9jb2'
    'xWZXJzaW9uUgZjbGllbnQSNQoGZGFlbW9uGAIgASgLMh0ueWF0YS5jb3JlLnYxLlByb3RvY29s'
    'VmVyc2lvblIGZGFlbW9u');
