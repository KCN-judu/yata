/// The one exception every daemon failure becomes (ADR-0012, "Package layout"), as three types
/// by what the receiver does with it (`core-protocol.md`, "Errors"):
///
/// - [RequestFailure]: the daemon refused one request; the session goes on.
/// - [SessionFailure]: the session cannot continue; the daemon has exited or is exiting.
/// - [RaisedFailure]: a failure only the application can see, raised by itself.
///
/// Each carries its generated protobuf failure, whose `kind` case is its code and whose record is
/// the debug information of that code. The views write the user's text from the case, and show
/// the record only in the detail view. Nothing branches on [CoreFailure.message].
library;

import 'package:fixnum/fixnum.dart';
import 'package:protobuf/protobuf.dart' show GeneratedMessage;

import '../gen/proto/core.pb.dart' as pb;

sealed class CoreFailure implements Exception {
  const CoreFailure();

  /// The stable dotted code (`query.stale_revision`), for logs, bug reports and the detail view;
  /// or `unknown (tag N)` for a case this build does not know.
  String get code;

  /// English, for developers. Never parsed, never the user's text.
  String get message;

  /// The debug record of the code, in protobuf text form; empty when the code has none.
  String get debugRecord;
}

final class RequestFailure extends CoreFailure {
  const RequestFailure(this.error);

  final pb.Error error;

  @override
  String get code => codeOf(error.whichKind().name, error);

  @override
  String get message => error.message;

  @override
  String get debugRecord => _record(_kindMessage(error));

  @override
  String toString() => 'RequestFailure($code: $message)';
}

final class SessionFailure extends CoreFailure {
  const SessionFailure(this.failed);

  final pb.SessionFailed failed;

  @override
  String get code => codeOf(failed.whichKind().name, failed);

  @override
  String get message => failed.message;

  @override
  String get debugRecord => _record(_kindMessage(failed));

  @override
  String toString() => 'SessionFailure($code: $message)';
}

final class RaisedFailure extends CoreFailure {
  const RaisedFailure(this.failure, this.message);

  final pb.ClientFailure failure;

  @override
  final String message;

  @override
  String get code => codeOf(failure.whichKind().name, failure);

  @override
  String get debugRecord => _record(_kindMessage(failure));

  @override
  String toString() => 'RaisedFailure($code: $message)';

  // The client's own failures, one constructor each.

  static RaisedFailure daemonNotFound(List<String> searched) => RaisedFailure(
    pb.ClientFailure(clientDaemonNotFound: pb.ClientDaemonNotFound(searched: searched)),
    'no yata-daemon executable',
  );

  static RaisedFailure daemonStartFailed(String path, String problem) => RaisedFailure(
    pb.ClientFailure(
      clientDaemonStartFailed: pb.ClientDaemonStartFailed(path: path, problem: problem),
    ),
    'the daemon could not be started',
  );

  static RaisedFailure daemonExited(int exitCode, String? lastLogLine, {required bool asked}) {
    final exited = pb.ClientDaemonExited(exitCode: exitCode);
    if (lastLogLine != null) exited.lastLogLine = lastLogLine;
    return RaisedFailure(
      pb.ClientFailure(clientDaemonExited: exited),
      asked ? 'the daemon shut down' : 'the daemon exited',
    );
  }

  static RaisedFailure timeout(int requestId, Duration limit) => RaisedFailure(
    pb.ClientFailure(
      clientTimeout: pb.ClientTimeout(
        requestId: Int64(requestId),
        limitMs: Int64(limit.inMilliseconds),
      ),
    ),
    'no response within the time limit',
  );

  static RaisedFailure protocolError(String problem) => RaisedFailure(
    pb.ClientFailure(clientProtocolError: pb.ClientProtocolError(problem: problem)),
    'the daemon broke the protocol',
  );

  static RaisedFailure notConnected() => RaisedFailure(
    pb.ClientFailure(clientNotConnected: pb.ClientNotConnected()),
    'the daemon is not running',
  );

  static RaisedFailure unexpected(Object error) => RaisedFailure(
    pb.ClientFailure(clientUnexpected: pb.ClientUnexpected(problem: error.toString())),
    'a failure in the application itself',
  );
}

/// Any value as a failure: a [CoreFailure] as it is, anything else as `client.unexpected`.
CoreFailure failureOf(Object error) => switch (error) {
  CoreFailure f => f,
  _ => RaisedFailure.unexpected(error),
};

/// The dotted code of a generated oneof case: `queryStaleRevision` is `query.stale_revision`. The
/// rule is the one the daemon applies to the schema's field names; no code is spelled here.
String codeOf(String caseName, GeneratedMessage message) {
  if (caseName == 'notSet') {
    final tags = message.unknownFields.asMap().keys.toList()..sort();
    return tags.isEmpty ? 'unknown' : 'unknown (tag ${tags.join(', ')})';
  }
  final snake = caseName.replaceAllMapped(RegExp('[A-Z]'), (m) => '_${m[0]!.toLowerCase()}');
  return snake.replaceFirst('_', '.');
}

/// The record of a failure's `kind` case, if one is set. Each failure message has one oneof,
/// `kind`, so its oneof index is 0.
GeneratedMessage? _kindMessage(GeneratedMessage failure) {
  final tag = failure.$_whichOneof(0);
  return tag == 0 ? null : failure.getField(tag) as GeneratedMessage?;
}

String _record(GeneratedMessage? record) => record?.toString().trim() ?? '';
