/// The Chinese text for every error and warning code the application can meet (ADR-0012,
/// "Localization and error text").
///
/// Only the code is read. An unmapped code falls back to a generic message, and the views show
/// the code beside it for bug reports. The `error-codes` preflight check reports a code the daemon
/// or the client can produce that has no case here.
library;

import '../../gen/l10n/app_localizations.dart';

/// The text for [code], or `null` when the code has none of its own.
String? mappedErrorText(AppLocalizations l, String code) => switch (code) {
  'client.unexpected' => l.errorUnexpected,
  'client.daemon_not_found' => l.errorDaemonNotFound,
  'client.daemon_start_failed' => l.errorDaemonStartFailed,
  'client.daemon_exited' => l.errorDaemonExited,
  'client.timeout' => l.errorTimeout,
  'client.protocol_error' => l.errorProtocol,
  'client.not_connected' => l.errorNotConnected,
  'session.protocol_unsupported' => l.errorProtocolUnsupported,
  'session.not_open' ||
  'session.already_open' ||
  'session.invalid_request_id' ||
  'session.unknown_request' ||
  'session.malformed_frame' ||
  'session.malformed_message' => l.errorSessionBroken,
  'query.unknown_profile' => l.errorUnknownProfile,
  'query.stale_revision' => l.errorStaleRevision,
  'query.malformed_cursor' => l.errorMalformedCursor,
  'query.malformed' => l.errorQueryMalformed,
  'query.unknown_field' => l.errorQueryUnknownField,
  'query.too_complex' => l.errorQueryTooComplex,
  'query.type_mismatch' => l.errorQueryTypeMismatch,
  'query.param_set_required' => l.errorQueryParamSetRequired,
  'query.field_unavailable' => l.errorQueryFieldUnavailable,
  'query.unknown_scheme' => l.errorQueryUnknownScheme,
  'decode.no_input' => l.errorDecodeNoInput,
  'decode.malformed_text' => l.errorDecodeMalformedText,
  'decode.unknown_format' => l.errorDecodeUnknownFormat,
  'decode.malformed_layout' || 'decode.malformed_scheme' => l.errorDecodeMalformed,
  'decode.no_qr_code' => l.errorDecodeNoQrCode,
  'decode.several_qr_codes' => l.errorDecodeSeveralQrCodes,
  'decode.qr_unreadable' => l.errorDecodeQrUnreadable,
  'decode.image_invalid' => l.errorDecodeImageInvalid,
  _ => null,
};

String errorText(AppLocalizations l, String code) => mappedErrorText(l, code) ?? l.errorUnknown;

String warningText(AppLocalizations l, String code) => switch (code) {
  'session.client_outdated' => l.warningClientOutdated,
  _ => l.warningUnknown,
};
