/// The byte pipe to one daemon process, and the process behind it.
///
/// The connection speaks the protocol over a [DaemonTransport]; only [ProcessTransport] touches
/// `dart:io`. Tests put an in-memory transport in its place.
library;

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'failure.dart';

abstract interface class DaemonTransport {
  /// The daemon's stdout: frames and nothing else. Listened to once.
  Stream<List<int>> get output;

  /// Write to the daemon's stdin.
  void send(List<int> bytes);

  /// Close the daemon's stdin: a daemon at a frame boundary then exits cleanly.
  Future<void> closeInput();

  /// Completes with the exit code when the process has ended.
  Future<int> get exitCode;

  /// End the process now.
  void kill();

  /// The last lines the daemon wrote to stderr, oldest first, for diagnostics.
  List<String> get recentLog;
}

/// Starts a daemon process and returns its transport, or throws a [RaisedFailure].
typedef DaemonLauncher = Future<DaemonTransport> Function();

/// A daemon child process on its stdio pipes (ADR-0004).
class ProcessTransport implements DaemonTransport {
  ProcessTransport._(this._process) {
    // A malformed byte in the log is shown as a replacement character, not a silenced log.
    _process.stderr
        .transform(const Utf8Decoder(allowMalformed: true))
        .transform(const LineSplitter())
        .listen(_log);
  }

  /// How many stderr lines are kept.
  static const logLines = 200;

  final Process _process;
  final List<String> _recent = [];

  static Future<ProcessTransport> start(String executable, List<String> arguments) async {
    try {
      final process = await Process.start(executable, arguments);
      return ProcessTransport._(process);
    } on ProcessException catch (e) {
      throw RaisedFailure.daemonStartFailed(executable, e.message);
    }
  }

  void _log(String line) {
    _recent.add(line);
    if (_recent.length > logLines) _recent.removeAt(0);
  }

  @override
  Stream<List<int>> get output => _process.stdout;

  @override
  void send(List<int> bytes) => _process.stdin.add(bytes);

  @override
  Future<void> closeInput() async {
    try {
      await _process.stdin.close();
    } on Object {
      // The process may already be gone; its exit is reported through [exitCode].
    }
  }

  @override
  Future<int> get exitCode => _process.exitCode;

  @override
  void kill() => _process.kill();

  @override
  List<String> get recentLog => List.unmodifiable(_recent);
}
