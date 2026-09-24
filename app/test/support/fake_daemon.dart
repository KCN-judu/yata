/// A [DaemonClient] with scripted answers, for the state and widget tests (ADR-0012, "Tests").
library;

import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/misc.dart' show Override;
import 'package:yata/daemon/daemon_client.dart';
import 'package:yata/daemon/daemon_error.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/platform/platform_services.dart';
import 'package:yata/state/core.dart';

import 'recorded.dart';

class FakeDaemonClient implements DaemonClient {
  FakeDaemonClient({DaemonHealth? health})
    : _health =
          health ??
          DaemonReady(
            daemonVersion: pb.ProtocolVersion(major: 1, minor: 0),
            revision: Int64.ONE,
            session: 1,
            restarted: false,
          );

  DaemonHealth _health;
  final _healthChanges = StreamController<DaemonHealth>.broadcast();
  final _events = StreamController<pb.Event>.broadcast();

  /// Answers; by default the recorded session's.
  Future<pb.ProfileList> Function() onListProfiles = () async => recordedProfiles();
  Future<pb.QueryPage> Function(pb.Query) onQuery = (q) async =>
      q.page.cursor.isEmpty ? recordedFirstPage() : recordedSecondPage();
  Future<pb.SchemeCodeDecoded> Function(pb.DecodeSchemeCode) onDecode = (_) async =>
      recordedScheme();

  final queries = <pb.Query>[];
  int profileCalls = 0;
  int restarts = 0;

  void setHealth(DaemonHealth h) {
    _health = h;
    _healthChanges.add(h);
  }

  void emit(pb.Event e) => _events.add(e);

  @override
  DaemonHealth get health => _health;

  @override
  Stream<DaemonHealth> get healthChanges => _healthChanges.stream;

  @override
  Stream<pb.Event> get events => _events.stream;

  @override
  List<String> get recentLog => const ['fake daemon log line'];

  @override
  Future<void> start() async {}

  @override
  Future<void> restart() async => restarts++;

  @override
  Future<void> shutdown() async {}

  @override
  Future<pb.ProfileList> listProfiles() {
    profileCalls++;
    return onListProfiles();
  }

  @override
  Future<pb.QueryPage> query(pb.Query query) {
    queries.add(query);
    return onQuery(query);
  }

  @override
  Future<pb.SchemeCodeDecoded> decodeSchemeCode(pb.DecodeSchemeCode request) => onDecode(request);
}

/// A daemon error, as the client would raise it.
DaemonException daemonError(String code) => DaemonException(code, 'test: $code');

class FakePlatform implements PlatformServices {
  const FakePlatform({this.fixture = false});

  final bool fixture;

  @override
  DaemonLocation locateDaemon() => const DaemonLocation(path: null, searched: []);

  @override
  bool get serveFixture => fixture;

  @override
  String? localIconPath(String kind, String role, int id) => null;

  @override
  Future<Never> pickPngImage() => throw UnimplementedError();
}

/// Overrides that put [client] behind every provider.
List<Override> fakeOverrides(FakeDaemonClient client) => [
  daemonClientProvider.overrideWithValue(client),
  platformServicesProvider.overrideWithValue(const FakePlatform()),
];
