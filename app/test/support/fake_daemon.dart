/// A [DaemonClient] with scripted answers, for the state and widget tests (ADR-0012, "Tests").
library;

import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/misc.dart' show Override;
import 'package:yata/daemon/daemon_client.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/platform/platform_services.dart';
import 'package:yata/state/core.dart';
import 'package:yata/state/profiles.dart';

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
  Future<pb.SessionQueryPage> Function(pb.SessionQuery) onQuery = (q) async =>
      q.hasNext() ? recordedSecondPage() : recordedFirstPage();
  Future<pb.SchemeCodeDecoded> Function(pb.DecodeSchemeCode) onDecode = (_) async =>
      recordedScheme();

  final queries = <pb.SessionQuery>[];
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
  Future<pb.SessionQueryPage> query(pb.SessionQuery query) {
    queries.add(query);
    return onQuery(query);
  }

  @override
  Future<pb.SchemeCodeDecoded> decodeSchemeCode(pb.DecodeSchemeCode request) => onDecode(request);
}

/// A capability the profile can use, as the daemon sends it.
pb.ProfileCapability availableAs(Capability c, Completeness completeness) => pb.ProfileCapability(
  capability: c.wire,
  available: pb.CapabilityAvailable(completeness: completeness.wire),
);

/// A capability the profile lacks [missing] for, as the daemon sends it.
pb.ProfileCapability unavailableFor(Capability c, List<SectionKind> missing) =>
    pb.ProfileCapability(
      capability: c.wire,
      unavailable: pb.CapabilityUnavailable(missing: [for (final k in missing) k.wire]),
    );

/// The recorded profiles, each with [capabilities] in place of what the daemon's fixture holds.
pb.ProfileList profilesWith(List<pb.ProfileCapability> capabilities) {
  final list = recordedProfiles().deepCopy();
  for (final p in list.profiles) {
    p.capabilities
      ..clear()
      ..addAll(capabilities.map((c) => c.deepCopy()));
  }
  return list;
}

/// The daemon's refusal of one request, as the client raises it.
RequestFailure refused(pb.Error error) => RequestFailure(error..message = 'test refusal');

class FakePlatform implements PlatformServices {
  const FakePlatform({this.fixture = false});

  final bool fixture;

  @override
  DaemonLocation locateDaemon() => const DaemonNotFound(searched: []);

  @override
  bool get serveFixture => fixture;

  @override
  String? localIconPath(IconKind kind, IconRole role, int id) => null;

  @override
  Future<Never> pickPngImage() => throw UnimplementedError();
}

/// Overrides that put [client] behind every provider.
List<Override> fakeOverrides(FakeDaemonClient client) => [
  daemonClientProvider.overrideWithValue(client),
  platformServicesProvider.overrideWithValue(const FakePlatform()),
];
