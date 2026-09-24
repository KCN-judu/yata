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

@$core.Deprecated('Use fieldNameDescriptor instead')
const FieldName$json = {
  '1': 'FieldName',
  '2': [
    {'1': 'FIELD_NAME_UNSPECIFIED', '2': 0},
    {'1': 'FIELD_NAME_SET', '2': 1},
    {'1': 'FIELD_NAME_SLOT', '2': 2},
    {'1': 'FIELD_NAME_STAR', '2': 3},
    {'1': 'FIELD_NAME_LEVEL', '2': 4},
    {'1': 'FIELD_NAME_MAIN_ATTRIBUTE', '2': 5},
    {'1': 'FIELD_NAME_MAIN_VALUE', '2': 6},
    {'1': 'FIELD_NAME_SUB_VALUE', '2': 7},
    {'1': 'FIELD_NAME_HAS_SUB', '2': 8},
    {'1': 'FIELD_NAME_SUB_COUNT', '2': 9},
    {'1': 'FIELD_NAME_PRISTINE', '2': 10},
    {'1': 'FIELD_NAME_QUALITY_TOTAL', '2': 11},
    {'1': 'FIELD_NAME_QUALITY_DEPTH', '2': 12},
    {'1': 'FIELD_NAME_QUALITY_BREADTH', '2': 13},
  ],
};

/// Descriptor for `FieldName`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List fieldNameDescriptor = $convert.base64Decode(
    'CglGaWVsZE5hbWUSGgoWRklFTERfTkFNRV9VTlNQRUNJRklFRBAAEhIKDkZJRUxEX05BTUVfU0'
    'VUEAESEwoPRklFTERfTkFNRV9TTE9UEAISEwoPRklFTERfTkFNRV9TVEFSEAMSFAoQRklFTERf'
    'TkFNRV9MRVZFTBAEEh0KGUZJRUxEX05BTUVfTUFJTl9BVFRSSUJVVEUQBRIZChVGSUVMRF9OQU'
    '1FX01BSU5fVkFMVUUQBhIYChRGSUVMRF9OQU1FX1NVQl9WQUxVRRAHEhYKEkZJRUxEX05BTUVf'
    'SEFTX1NVQhAIEhgKFEZJRUxEX05BTUVfU1VCX0NPVU5UEAkSFwoTRklFTERfTkFNRV9QUklTVE'
    'lORRAKEhwKGEZJRUxEX05BTUVfUVVBTElUWV9UT1RBTBALEhwKGEZJRUxEX05BTUVfUVVBTElU'
    'WV9ERVBUSBAMEh4KGkZJRUxEX05BTUVfUVVBTElUWV9CUkVBRFRIEA0=');

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

@$core.Deprecated('Use errorDescriptor instead')
const Error$json = {
  '1': 'Error',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
    {'1': 'details', '3': 3, '4': 1, '5': 12, '10': 'details'},
  ],
};

/// Descriptor for `Error`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List errorDescriptor = $convert.base64Decode(
    'CgVFcnJvchISCgRjb2RlGAEgASgJUgRjb2RlEhgKB21lc3NhZ2UYAiABKAlSB21lc3NhZ2USGA'
    'oHZGV0YWlscxgDIAEoDFIHZGV0YWlscw==');

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
      '1': 'innate',
      '3': 9,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Innate',
      '10': 'innate'
    },
  ],
};

/// Descriptor for `Soul`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List soulDescriptor = $convert.base64Decode(
    'CgRTb3VsEhcKB3NvdWxfaWQYASABKAlSBnNvdWxJZBIbCglzdWl0X2NvZGUYAiABKA1SCHN1aX'
    'RDb2RlEioKBHNsb3QYAyABKA4yFi55YXRhLmNvcmUudjEuU291bFNsb3RSBHNsb3QSEgoEc3Rh'
    'chgEIAEoDVIEc3RhchIUCgVsZXZlbBgFIAEoDVIFbGV2ZWwSLwoEbWFpbhgGIAEoDjIbLnlhdG'
    'EuY29yZS52MS5Tb3VsQXR0cmlidXRlUgRtYWluEh0KCm1haW5fdmFsdWUYByABKAFSCW1haW5W'
    'YWx1ZRIuCgRzdWJzGAggAygLMhoueWF0YS5jb3JlLnYxLlN1YkF0dHJpYnV0ZVIEc3VicxIsCg'
    'Zpbm5hdGUYCSABKAsyFC55YXRhLmNvcmUudjEuSW5uYXRlUgZpbm5hdGU=');

@$core.Deprecated('Use innateDescriptor instead')
const Innate$json = {
  '1': 'Innate',
  '2': [
    {'1': 'absent', '3': 1, '4': 1, '5': 8, '9': 0, '10': 'absent'},
    {
      '1': 'present',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '9': 0,
      '10': 'present'
    },
  ],
  '8': [
    {'1': 'state'},
  ],
};

/// Descriptor for `Innate`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List innateDescriptor = $convert.base64Decode(
    'CgZJbm5hdGUSGAoGYWJzZW50GAEgASgISABSBmFic2VudBI3CgdwcmVzZW50GAIgASgOMhsueW'
    'F0YS5jb3JlLnYxLlNvdWxBdHRyaWJ1dGVIAFIHcHJlc2VudEIHCgVzdGF0ZQ==');

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

@$core.Deprecated('Use soulSelectionDescriptor instead')
const SoulSelection$json = {
  '1': 'SoulSelection',
  '2': [
    {'1': 'any_set', '3': 1, '4': 1, '5': 8, '10': 'anySet'},
    {'1': 'suit_codes', '3': 2, '4': 3, '5': 13, '10': 'suitCodes'},
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
      '1': 'sub_included',
      '3': 8,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'subIncluded'
    },
    {
      '1': 'sub_excluded',
      '3': 9,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'subExcluded'
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
};

/// Descriptor for `SoulSelection`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List soulSelectionDescriptor = $convert.base64Decode(
    'Cg1Tb3VsU2VsZWN0aW9uEhcKB2FueV9zZXQYASABKAhSBmFueVNldBIdCgpzdWl0X2NvZGVzGA'
    'IgAygNUglzdWl0Q29kZXMSLAoFc2xvdHMYAyADKA4yFi55YXRhLmNvcmUudjEuU291bFNsb3RS'
    'BXNsb3RzEhQKBXN0YXJzGAQgAygNUgVzdGFycxIvCgZsZXZlbHMYBSADKA4yFy55YXRhLmNvcm'
    'UudjEuTGV2ZWxCYW5kUgZsZXZlbHMSRAoPbWFpbl9hdHRyaWJ1dGVzGAYgAygOMhsueWF0YS5j'
    'b3JlLnYxLlNvdWxBdHRyaWJ1dGVSDm1haW5BdHRyaWJ1dGVzEjMKBmlubmF0ZRgHIAMoDjIbLn'
    'lhdGEuY29yZS52MS5Tb3VsQXR0cmlidXRlUgZpbm5hdGUSPgoMc3ViX2luY2x1ZGVkGAggAygO'
    'MhsueWF0YS5jb3JlLnYxLlNvdWxBdHRyaWJ1dGVSC3N1YkluY2x1ZGVkEj4KDHN1Yl9leGNsdW'
    'RlZBgJIAMoDjIbLnlhdGEuY29yZS52MS5Tb3VsQXR0cmlidXRlUgtzdWJFeGNsdWRlZBI1Cgpz'
    'dWJfY291bnRzGAogAygOMhYueWF0YS5jb3JlLnYxLlN1YkNvdW50UglzdWJDb3VudHM=');

@$core.Deprecated('Use fieldDescriptor instead')
const Field$json = {
  '1': 'Field',
  '2': [
    {
      '1': 'name',
      '3': 1,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.FieldName',
      '10': 'name'
    },
    {
      '1': 'attribute',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'attribute'
    },
  ],
};

/// Descriptor for `Field`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fieldDescriptor = $convert.base64Decode(
    'CgVGaWVsZBIrCgRuYW1lGAEgASgOMhcueWF0YS5jb3JlLnYxLkZpZWxkTmFtZVIEbmFtZRI5Cg'
    'lhdHRyaWJ1dGUYAiABKA4yGy55YXRhLmNvcmUudjEuU291bEF0dHJpYnV0ZVIJYXR0cmlidXRl');

@$core.Deprecated('Use inTestDescriptor instead')
const InTest$json = {
  '1': 'InTest',
  '2': [
    {'1': 'suit_codes', '3': 1, '4': 3, '5': 13, '10': 'suitCodes'},
    {
      '1': 'slots',
      '3': 2,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulSlot',
      '10': 'slots'
    },
    {
      '1': 'attributes',
      '3': 3,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.SoulAttribute',
      '10': 'attributes'
    },
  ],
};

/// Descriptor for `InTest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List inTestDescriptor = $convert.base64Decode(
    'CgZJblRlc3QSHQoKc3VpdF9jb2RlcxgBIAMoDVIJc3VpdENvZGVzEiwKBXNsb3RzGAIgAygOMh'
    'YueWF0YS5jb3JlLnYxLlNvdWxTbG90UgVzbG90cxI7CgphdHRyaWJ1dGVzGAMgAygOMhsueWF0'
    'YS5jb3JlLnYxLlNvdWxBdHRyaWJ1dGVSCmF0dHJpYnV0ZXM=');

@$core.Deprecated('Use intRangeDescriptor instead')
const IntRange$json = {
  '1': 'IntRange',
  '2': [
    {'1': 'min', '3': 1, '4': 1, '5': 3, '9': 0, '10': 'min', '17': true},
    {'1': 'max', '3': 2, '4': 1, '5': 3, '9': 1, '10': 'max', '17': true},
  ],
  '8': [
    {'1': '_min'},
    {'1': '_max'},
  ],
};

/// Descriptor for `IntRange`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List intRangeDescriptor = $convert.base64Decode(
    'CghJbnRSYW5nZRIVCgNtaW4YASABKANIAFIDbWluiAEBEhUKA21heBgCIAEoA0gBUgNtYXiIAQ'
    'FCBgoEX21pbkIGCgRfbWF4');

@$core.Deprecated('Use numberRangeDescriptor instead')
const NumberRange$json = {
  '1': 'NumberRange',
  '2': [
    {'1': 'min', '3': 1, '4': 1, '5': 1, '9': 0, '10': 'min', '17': true},
    {'1': 'max', '3': 2, '4': 1, '5': 1, '9': 1, '10': 'max', '17': true},
  ],
  '8': [
    {'1': '_min'},
    {'1': '_max'},
  ],
};

/// Descriptor for `NumberRange`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List numberRangeDescriptor = $convert.base64Decode(
    'CgtOdW1iZXJSYW5nZRIVCgNtaW4YASABKAFIAFIDbWluiAEBEhUKA21heBgCIAEoAUgBUgNtYX'
    'iIAQFCBgoEX21pbkIGCgRfbWF4');

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
    {'1': 'cursor', '3': 2, '4': 1, '5': 12, '10': 'cursor'},
  ],
  '8': [
    {'1': '_row_budget'},
  ],
};

/// Descriptor for `PageRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pageRequestDescriptor = $convert.base64Decode(
    'CgtQYWdlUmVxdWVzdBIiCgpyb3dfYnVkZ2V0GAEgASgNSABSCXJvd0J1ZGdldIgBARIWCgZjdX'
    'Jzb3IYAiABKAxSBmN1cnNvckINCgtfcm93X2J1ZGdldA==');

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
    {
      '1': 'page',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.PageRequest',
      '10': 'page'
    },
    {'1': 'profile_id', '3': 6, '4': 1, '5': 9, '10': 'profileId'},
    {
      '1': 'scan_revision',
      '3': 7,
      '4': 1,
      '5': 4,
      '9': 0,
      '10': 'scanRevision',
      '17': true
    },
  ],
  '8': [
    {'1': '_scan_revision'},
  ],
};

/// Descriptor for `Query`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryDescriptor = $convert.base64Decode(
    'CgVRdWVyeRI4Cgpjb2xsZWN0aW9uGAEgASgOMhgueWF0YS5jb3JlLnYxLkNvbGxlY3Rpb25SCm'
    'NvbGxlY3Rpb24SKgoGZmlsdGVyGAIgASgLMhIueWF0YS5jb3JlLnYxLkV4cHJSBmZpbHRlchIp'
    'CgRzb3J0GAMgAygLMhUueWF0YS5jb3JlLnYxLlNvcnRLZXlSBHNvcnQSMQoGcGFyYW1zGAQgAS'
    'gLMhkueWF0YS5jb3JlLnYxLlBhcmFtU2V0UmVmUgZwYXJhbXMSLQoEcGFnZRgFIAEoCzIZLnlh'
    'dGEuY29yZS52MS5QYWdlUmVxdWVzdFIEcGFnZRIdCgpwcm9maWxlX2lkGAYgASgJUglwcm9maW'
    'xlSWQSKAoNc2Nhbl9yZXZpc2lvbhgHIAEoBEgAUgxzY2FuUmV2aXNpb26IAQFCEAoOX3NjYW5f'
    'cmV2aXNpb24=');

@$core.Deprecated('Use queryRowDescriptor instead')
const QueryRow$json = {
  '1': 'QueryRow',
  '2': [
    {'1': 'soul_id', '3': 1, '4': 1, '5': 9, '10': 'soulId'},
    {
      '1': 'open_rules',
      '3': 2,
      '4': 3,
      '5': 14,
      '6': '.yata.core.v1.OpenRule',
      '10': 'openRules'
    },
    {
      '1': 'soul',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Soul',
      '10': 'soul'
    },
  ],
};

/// Descriptor for `QueryRow`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryRowDescriptor = $convert.base64Decode(
    'CghRdWVyeVJvdxIXCgdzb3VsX2lkGAEgASgJUgZzb3VsSWQSNQoKb3Blbl9ydWxlcxgCIAMoDj'
    'IWLnlhdGEuY29yZS52MS5PcGVuUnVsZVIJb3BlblJ1bGVzEiYKBHNvdWwYAyABKAsyEi55YXRh'
    'LmNvcmUudjEuU291bFIEc291bA==');

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
    {'1': 'has_more', '3': 2, '4': 1, '5': 8, '10': 'hasMore'},
    {'1': 'cursor', '3': 3, '4': 1, '5': 12, '10': 'cursor'},
    {'1': 'revision', '3': 4, '4': 1, '5': 4, '10': 'revision'},
    {'1': 'total', '3': 5, '4': 1, '5': 4, '10': 'total'},
  ],
};

/// Descriptor for `QueryPage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryPageDescriptor = $convert.base64Decode(
    'CglRdWVyeVBhZ2USKgoEcm93cxgBIAMoCzIWLnlhdGEuY29yZS52MS5RdWVyeVJvd1IEcm93cx'
    'IZCghoYXNfbW9yZRgCIAEoCFIHaGFzTW9yZRIWCgZjdXJzb3IYAyABKAxSBmN1cnNvchIaCghy'
    'ZXZpc2lvbhgEIAEoBFIIcmV2aXNpb24SFAoFdG90YWwYBSABKARSBXRvdGFs');

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
  ],
};

/// Descriptor for `EvaluateQuery`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List evaluateQueryDescriptor = $convert.base64Decode(
    'Cg1FdmFsdWF0ZVF1ZXJ5Eg4KAmlkGAEgASgEUgJpZBJIChBwcm90b2NvbF92ZXJzaW9uGAIgAS'
    'gLMh0ueWF0YS5jb3JlLnYxLlByb3RvY29sVmVyc2lvblIPcHJvdG9jb2xWZXJzaW9uEjAKCWlu'
    'dmVudG9yeRgDIAMoCzISLnlhdGEuY29yZS52MS5Tb3VsUglpbnZlbnRvcnkSKQoFcXVlcnkYBC'
    'ABKAsyEy55YXRhLmNvcmUudjEuUXVlcnlSBXF1ZXJ5');

@$core.Deprecated('Use evaluateQueryResultDescriptor instead')
const EvaluateQueryResult$json = {
  '1': 'EvaluateQueryResult',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 4, '10': 'id'},
    {
      '1': 'page',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryPage',
      '9': 0,
      '10': 'page'
    },
    {
      '1': 'error',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Error',
      '9': 0,
      '10': 'error'
    },
  ],
  '8': [
    {'1': 'outcome'},
  ],
};

/// Descriptor for `EvaluateQueryResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List evaluateQueryResultDescriptor = $convert.base64Decode(
    'ChNFdmFsdWF0ZVF1ZXJ5UmVzdWx0Eg4KAmlkGAEgASgEUgJpZBItCgRwYWdlGAIgASgLMhcueW'
    'F0YS5jb3JlLnYxLlF1ZXJ5UGFnZUgAUgRwYWdlEisKBWVycm9yGAMgASgLMhMueWF0YS5jb3Jl'
    'LnYxLkVycm9ySABSBWVycm9yQgkKB291dGNvbWU=');

@$core.Deprecated('Use clientMessageDescriptor instead')
const ClientMessage$json = {
  '1': 'ClientMessage',
  '2': [
    {
      '1': 'protocol_version',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.ProtocolVersion',
      '10': 'protocolVersion'
    },
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
      '1': 'query',
      '3': 21,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Query',
      '9': 0,
      '10': 'query'
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
};

/// Descriptor for `ClientMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clientMessageDescriptor = $convert.base64Decode(
    'Cg1DbGllbnRNZXNzYWdlEkgKEHByb3RvY29sX3ZlcnNpb24YASABKAsyHS55YXRhLmNvcmUudj'
    'EuUHJvdG9jb2xWZXJzaW9uUg9wcm90b2NvbFZlcnNpb24SDgoCaWQYAiABKARSAmlkEj4KDG9w'
    'ZW5fc2Vzc2lvbhgKIAEoCzIZLnlhdGEuY29yZS52MS5PcGVuU2Vzc2lvbkgAUgtvcGVuU2Vzc2'
    'lvbhI0CghzaHV0ZG93bhgLIAEoCzIWLnlhdGEuY29yZS52MS5TaHV0ZG93bkgAUghzaHV0ZG93'
    'bhI3CglzdWJzY3JpYmUYDCABKAsyFy55YXRhLmNvcmUudjEuU3Vic2NyaWJlSABSCXN1YnNjcm'
    'liZRJBCg1saXN0X3Byb2ZpbGVzGBQgASgLMhoueWF0YS5jb3JlLnYxLkxpc3RQcm9maWxlc0gA'
    'UgxsaXN0UHJvZmlsZXMSKwoFcXVlcnkYFSABKAsyEy55YXRhLmNvcmUudjEuUXVlcnlIAFIFcX'
    'VlcnkSTgoSZGVjb2RlX3NjaGVtZV9jb2RlGBYgASgLMh4ueWF0YS5jb3JlLnYxLkRlY29kZVNj'
    'aGVtZUNvZGVIAFIQZGVjb2RlU2NoZW1lQ29kZUIGCgRraW5k');

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
      '1': 'query_page',
      '3': 21,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.QueryPage',
      '9': 0,
      '10': 'queryPage'
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
};

/// Descriptor for `Response`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List responseDescriptor = $convert.base64Decode(
    'CghSZXNwb25zZRIOCgJpZBgBIAEoBFICaWQSKwoFZXJyb3IYAiABKAsyEy55YXRhLmNvcmUudj'
    'EuRXJyb3JIAFIFZXJyb3ISRAoOc2Vzc2lvbl9vcGVuZWQYCiABKAsyGy55YXRhLmNvcmUudjEu'
    'U2Vzc2lvbk9wZW5lZEgAUg1zZXNzaW9uT3BlbmVkEk0KEXNodXRkb3duX2FjY2VwdGVkGAsgAS'
    'gLMh4ueWF0YS5jb3JlLnYxLlNodXRkb3duQWNjZXB0ZWRIAFIQc2h1dGRvd25BY2NlcHRlZBI6'
    'CgpzdWJzY3JpYmVkGAwgASgLMhgueWF0YS5jb3JlLnYxLlN1YnNjcmliZWRIAFIKc3Vic2NyaW'
    'JlZBI+Cgxwcm9maWxlX2xpc3QYFCABKAsyGS55YXRhLmNvcmUudjEuUHJvZmlsZUxpc3RIAFIL'
    'cHJvZmlsZUxpc3QSOAoKcXVlcnlfcGFnZRgVIAEoCzIXLnlhdGEuY29yZS52MS5RdWVyeVBhZ2'
    'VIAFIJcXVlcnlQYWdlElEKE3NjaGVtZV9jb2RlX2RlY29kZWQYFiABKAsyHy55YXRhLmNvcmUu'
    'djEuU2NoZW1lQ29kZURlY29kZWRIAFIRc2NoZW1lQ29kZURlY29kZWRCCAoGcmVzdWx0');

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
      '1': 'session_failed',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.yata.core.v1.Error',
      '9': 0,
      '10': 'sessionFailed'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
};

/// Descriptor for `Event`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List eventDescriptor = $convert.base64Decode(
    'CgVFdmVudBJQChJwcm9qZWN0aW9uX2NoYW5nZWQYASABKAsyHy55YXRhLmNvcmUudjEuUHJvam'
    'VjdGlvbkNoYW5nZWRIAFIRcHJvamVjdGlvbkNoYW5nZWQSMQoHd2FybmluZxgCIAEoCzIVLnlh'
    'dGEuY29yZS52MS5XYXJuaW5nSABSB3dhcm5pbmcSPAoOc2Vzc2lvbl9mYWlsZWQYAyABKAsyEy'
    '55YXRhLmNvcmUudjEuRXJyb3JIAFINc2Vzc2lvbkZhaWxlZEIGCgRraW5k');

@$core.Deprecated('Use warningDescriptor instead')
const Warning$json = {
  '1': 'Warning',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `Warning`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List warningDescriptor = $convert.base64Decode(
    'CgdXYXJuaW5nEhIKBGNvZGUYASABKAlSBGNvZGUSGAoHbWVzc2FnZRgCIAEoCVIHbWVzc2FnZQ'
    '==');

@$core.Deprecated('Use openSessionDescriptor instead')
const OpenSession$json = {
  '1': 'OpenSession',
};

/// Descriptor for `OpenSession`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List openSessionDescriptor =
    $convert.base64Decode('CgtPcGVuU2Vzc2lvbg==');

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
  ],
};

/// Descriptor for `Profile`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List profileDescriptor = $convert.base64Decode(
    'CgdQcm9maWxlEg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW1l');

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
