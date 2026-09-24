/// The one exception every daemon failure becomes (ADR-0012, "Package layout").
///
/// A failure the daemon reports crosses as `Error { code, message, details }`
/// (`core-protocol.md`, "Errors") and arrives here unchanged. A failure only the client can see
/// — the daemon missing, exited, or silent — gets a code in the `client.*` namespace, which never
/// crosses the wire. Either way `code` is the contract: nothing branches on `message`.
library;

import 'dart:typed_data';

import '../gen/proto/core.pb.dart' as pb;

/// Codes the client raises itself (`core-protocol.md`, "Errors", client-side codes).
abstract final class ClientErrorCode {
  /// No daemon executable where the platform looks for one.
  static const daemonNotFound = 'client.daemon_not_found';

  /// The operating system refused to start the daemon.
  static const daemonStartFailed = 'client.daemon_start_failed';

  /// The daemon exited while the request was in flight, or before the session opened.
  static const daemonExited = 'client.daemon_exited';

  /// The daemon did not answer within the request's time limit.
  static const timeout = 'client.timeout';

  /// The daemon's output broke the protocol: a bad frame, a message that does not decode, or a
  /// response to no request.
  static const protocolError = 'client.protocol_error';

  /// A request was made while no session was open and none is being opened.
  static const notConnected = 'client.not_connected';
}

/// Renamed codes: old name to new. Both decode; only the new one is emitted
/// (`protocol-versions.md`, "Error-code aliases"). Empty until a code is renamed.
const Map<String, String> errorCodeAliases = {};

/// The current name of [code].
String canonicalErrorCode(String code) => errorCodeAliases[code] ?? code;

class DaemonException implements Exception {
  DaemonException(String code, this.message, [Uint8List? details])
    : code = canonicalErrorCode(code),
      details = details ?? Uint8List(0);

  factory DaemonException.fromWire(pb.Error e) =>
      DaemonException(e.code, e.message, Uint8List.fromList(e.details));

  /// Stable, namespaced, machine-readable. The UI maps it to its own text.
  final String code;

  /// English, for developers and the expert view. Never parsed.
  final String message;

  /// A serialized protobuf message whose type [code] names; empty when there is none.
  final Uint8List details;

  @override
  String toString() => 'DaemonException($code: $message)';
}
