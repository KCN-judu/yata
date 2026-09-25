/// What the user reads about a failure: its cause and its remedy (A-Q4 of the 2026-09-25 audit).
///
/// Every code of `core.proto`'s three failure sets is a case of a generated `Kind` enum, and each
/// switch here names every case with no catch-all, so a new code does not compile until it has
/// text. Codes the user acts on alike share one explanation, but are still named one by one. The
/// debug record of a code is never part of this text: it is shown in the detail view only.
library;

import '../../gen/l10n/app_localizations.dart';
import '../../gen/proto/core.pb.dart' as pb;
import '../../state/core.dart';

final class Explanation {
  const Explanation(this.cause, this.remedy);

  final String cause;
  final String remedy;
}

Explanation explain(AppLocalizations l, CoreFailure failure) => switch (failure) {
  RequestFailure(:final error) => _request(l, error.whichKind()),
  SessionFailure(:final failed) => _session(l, failed.whichKind()),
  RaisedFailure(failure: final raised) => _client(l, raised.whichKind()),
};

Explanation _request(AppLocalizations l, pb.Error_Kind kind) => switch (kind) {
  pb.Error_Kind.sessionProtocolUnsupported => Explanation(
    l.causeProtocolUnsupported,
    l.remedyProtocolUnsupported,
  ),
  pb.Error_Kind.sessionNotOpen ||
  pb.Error_Kind.sessionAlreadyOpen ||
  pb.Error_Kind.sessionInvalidRequestId ||
  pb.Error_Kind.sessionUnknownRequest => Explanation(l.causeRequestRefused, l.remedyRequestRefused),
  pb.Error_Kind.queryUnknownProfile => Explanation(l.causeUnknownProfile, l.remedyUnknownProfile),
  pb.Error_Kind.queryStaleRevision => Explanation(l.causeStaleRevision, l.remedyStaleRevision),
  pb.Error_Kind.queryMalformed => Explanation(l.causeQueryMalformed, l.remedyQueryMalformed),
  pb.Error_Kind.queryUnknownField => Explanation(
    l.causeQueryUnknownField,
    l.remedyQueryUnknownField,
  ),
  pb.Error_Kind.queryTooComplex => Explanation(l.causeQueryTooComplex, l.remedyQueryTooComplex),
  pb.Error_Kind.queryTypeMismatch => Explanation(
    l.causeQueryTypeMismatch,
    l.remedyQueryTypeMismatch,
  ),
  pb.Error_Kind.queryParamSetRequired => Explanation(
    l.causeQueryParamSetRequired,
    l.remedyQueryParamSetRequired,
  ),
  pb.Error_Kind.queryFieldUnavailable => Explanation(
    l.causeQueryFieldUnavailable,
    l.remedyQueryFieldUnavailable,
  ),
  pb.Error_Kind.queryUnknownScheme => Explanation(
    l.causeQueryUnknownScheme,
    l.remedyQueryUnknownScheme,
  ),
  pb.Error_Kind.queryMalformedCursor => Explanation(
    l.causeMalformedCursor,
    l.remedyMalformedCursor,
  ),
  pb.Error_Kind.decodeNoInput => Explanation(l.causeDecodeNoInput, l.remedyDecodeNoInput),
  pb.Error_Kind.decodeMalformedText => Explanation(
    l.causeDecodeMalformedText,
    l.remedyDecodeMalformedText,
  ),
  pb.Error_Kind.decodeUnknownFormat => Explanation(
    l.causeDecodeUnknownFormat,
    l.remedyDecodeUnknownFormat,
  ),
  pb.Error_Kind.decodeMalformedLayout || pb.Error_Kind.decodeMalformedScheme => Explanation(
    l.causeDecodeMalformed,
    l.remedyDecodeMalformed,
  ),
  pb.Error_Kind.decodeImageInvalid => Explanation(
    l.causeDecodeImageInvalid,
    l.remedyDecodeImageInvalid,
  ),
  pb.Error_Kind.decodeNoQrCode => Explanation(l.causeDecodeNoQrCode, l.remedyDecodeNoQrCode),
  pb.Error_Kind.decodeSeveralQrCodes => Explanation(
    l.causeDecodeSeveralQrCodes,
    l.remedyDecodeSeveralQrCodes,
  ),
  pb.Error_Kind.decodeQrUnreadable => Explanation(
    l.causeDecodeQrUnreadable,
    l.remedyDecodeQrUnreadable,
  ),
  pb.Error_Kind.importUnknownFormat => Explanation(
    l.causeImportUnknownFormat,
    l.remedyImportUnknownFormat,
  ),
  pb.Error_Kind.importAmbiguousFormat => Explanation(
    l.causeImportAmbiguousFormat,
    l.remedyImportAmbiguousFormat,
  ),
  pb.Error_Kind.importUnsupportedVersion => Explanation(
    l.causeImportUnsupportedVersion,
    l.remedyImportUnsupportedVersion,
  ),
  pb.Error_Kind.importMalformedSource => Explanation(
    l.causeImportMalformedSource,
    l.remedyImportMalformedSource,
  ),
  pb.Error_Kind.importNormalizationFailed => Explanation(
    l.causeImportNormalizationFailed,
    l.remedyImportNormalizationFailed,
  ),
  pb.Error_Kind.importUnsupportedSourceValue => Explanation(
    l.causeImportUnsupportedSourceValue,
    l.remedyImportUnsupportedSourceValue,
  ),
  pb.Error_Kind.importInconsistentReference => Explanation(
    l.causeImportInconsistentReference,
    l.remedyImportInconsistentReference,
  ),
  pb.Error_Kind.importAdmissionRefused => Explanation(
    l.causeImportAdmissionRefused,
    l.remedyImportAdmissionRefused,
  ),
  pb.Error_Kind.importAccountMismatch => Explanation(
    l.causeImportAccountMismatch,
    l.remedyImportAccountMismatch,
  ),
  pb.Error_Kind.commandRefused => Explanation(l.causeCommandRefused, l.remedyCommandRefused),
  pb.Error_Kind.commandTooLarge => Explanation(l.causeCommandTooLarge, l.remedyCommandTooLarge),
  pb.Error_Kind.storeFailure => Explanation(l.causeStoreFailure, l.remedyStoreFailure),
  pb.Error_Kind.storeInvalidLog ||
  pb.Error_Kind.storeMalformedCommit ||
  pb.Error_Kind.storeDamaged => Explanation(l.causeStoreDamaged, l.remedyStoreDamaged),
  pb.Error_Kind.storeNewerFormat => Explanation(l.causeStoreNewerFormat, l.remedyStoreNewerFormat),
  pb.Error_Kind.storeMissing => Explanation(l.causeStoreMissing, l.remedyStoreMissing),
  pb.Error_Kind.storeRetiredFormat => Explanation(
    l.causeStoreRetiredFormat,
    l.remedyStoreRetiredFormat,
  ),
  pb.Error_Kind.storeNotADatabase ||
  pb.Error_Kind.storeForeign ||
  pb.Error_Kind.storeNoFormatVersion ||
  pb.Error_Kind.storeUninitialized => Explanation(l.causeStoreNotOurs, l.remedyStoreNotOurs),
  pb.Error_Kind.internalPanic ||
  pb.Error_Kind.internalQrTooLong ||
  pb.Error_Kind.internalResponseTooLarge ||
  pb.Error_Kind.internalPageWithoutSoul ||
  pb.Error_Kind.internalImportMismatch => Explanation(l.causeInternal, l.remedyInternal),
  pb.Error_Kind.notSet => Explanation(l.causeUnknown, l.remedyUnknown),
};

Explanation _session(AppLocalizations l, pb.SessionFailed_Kind kind) => switch (kind) {
  pb.SessionFailed_Kind.sessionMalformedFrame || pb.SessionFailed_Kind.sessionMalformedMessage =>
    Explanation(l.causeStreamBroken, l.remedyStreamBroken),
  pb.SessionFailed_Kind.internalIo => Explanation(l.causeStreamIo, l.remedyStreamIo),
  pb.SessionFailed_Kind.notSet => Explanation(l.causeUnknown, l.remedyUnknown),
};

Explanation _client(AppLocalizations l, pb.ClientFailure_Kind kind) => switch (kind) {
  pb.ClientFailure_Kind.clientDaemonNotFound => Explanation(
    l.causeDaemonNotFound,
    l.remedyDaemonNotFound,
  ),
  pb.ClientFailure_Kind.clientDaemonStartFailed => Explanation(
    l.causeDaemonStartFailed,
    l.remedyDaemonStartFailed,
  ),
  pb.ClientFailure_Kind.clientDaemonExited => Explanation(
    l.causeDaemonExited,
    l.remedyDaemonExited,
  ),
  pb.ClientFailure_Kind.clientTimeout => Explanation(l.causeTimeout, l.remedyTimeout),
  pb.ClientFailure_Kind.clientProtocolError => Explanation(
    l.causeProtocolError,
    l.remedyProtocolError,
  ),
  pb.ClientFailure_Kind.clientNotConnected => Explanation(
    l.causeNotConnected,
    l.remedyNotConnected,
  ),
  pb.ClientFailure_Kind.clientUnexpected => Explanation(l.causeUnexpected, l.remedyUnexpected),
  pb.ClientFailure_Kind.notSet => Explanation(l.causeUnknown, l.remedyUnknown),
};

/// Whether this build knows the failure's code: an unknown one is shown with its code visible.
bool isKnown(CoreFailure failure) => switch (failure) {
  RequestFailure(:final error) => error.whichKind() != pb.Error_Kind.notSet,
  SessionFailure(:final failed) => failed.whichKind() != pb.SessionFailed_Kind.notSet,
  RaisedFailure(failure: final raised) => raised.whichKind() != pb.ClientFailure_Kind.notSet,
};

String warningText(AppLocalizations l, pb.Warning warning) => switch (warning.whichKind()) {
  pb.Warning_Kind.clientOutdated => l.warningClientOutdated,
  pb.Warning_Kind.notSet => l.warningUnknown,
};
