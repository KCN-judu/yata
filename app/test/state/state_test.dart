// The state layer over a fake daemon client: loading, data, and failure by case; paging by cursor;
// a stale scan restarting; revision-driven refetch; a new session refetching; core status.

import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/daemon_client.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/state/core.dart';
import 'package:yata/state/inventory.dart';
import 'package:yata/state/profiles.dart';
import 'package:yata/state/schemes.dart';

import '../support/fake_daemon.dart';
import '../support/recorded.dart';

ProviderContainer containerFor(FakeDaemonClient client) =>
    ProviderContainer.test(overrides: fakeOverrides(client), retry: (_, _) => null);

const spec = SoulQuerySpec(profileId: fixtureProfile, rowBudget: 8);

/// A client failure of [kind]: its case is its code.
Matcher raised(pb.ClientFailure_Kind kind) =>
    isA<RaisedFailure>().having((e) => e.failure.whichKind(), 'kind', kind);

void main() {
  test('profiles load, and the first becomes the selection', () async {
    final c = containerFor(FakeDaemonClient());
    expect(c.read(profilesProvider), isA<AsyncLoading<List<pb.Profile>>>());
    await c.read(profilesProvider.future);
    expect(c.read(selectedProfileProvider), fixtureProfile);
    c.read(selectedProfileProvider.notifier).select(emptyProfile);
    expect(c.read(soulQuerySpecProvider)?.profileId, emptyProfile);
  });

  test('a failed call surfaces as the typed failure the daemon sent', () async {
    final client = FakeDaemonClient()
      ..onListProfiles = () async =>
          throw refused(pb.Error(queryUnknownProfile: pb.QueryUnknownProfile()));
    final c = containerFor(client);
    c.listen(profilesProvider, (_, _) {});
    await expectLater(c.read(profilesProvider.future), throwsA(isA<RequestFailure>()));
    final error = c.read(profilesProvider).error;
    expect(
      error,
      isA<RequestFailure>().having(
        (e) => e.error.whichKind(),
        'kind',
        pb.Error_Kind.queryUnknownProfile,
      ),
    );
  });

  test('anything else thrown becomes client.unexpected, never a raw exception', () async {
    final client = FakeDaemonClient()..onListProfiles = () async => throw StateError('boom');
    final c = containerFor(client);
    c.listen(profilesProvider, (_, _) {});
    await expectLater(
      c.read(profilesProvider.future),
      throwsA(raised(pb.ClientFailure_Kind.clientUnexpected)),
    );
  });

  test('a page loads, turns forward by cursor, and back', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(soulPagesProvider(spec), (_, _) {});
    final first = await c.read(soulPagesProvider(spec).future);
    expect(first.souls, hasLength(8));
    expect((first.total, first.index, first.hasMore), (Int64(12), 0, true));
    await c.read(soulPagesProvider(spec).notifier).next();
    final second = c.read(soulPagesProvider(spec)).requireValue;
    expect((second.index, second.firstRow, second.hasMore), (1, 8, false));
    expect(client.queries.last.next.cursor, recordedFirstPage().nextCursor);
    expect(Revision(client.queries.last.next.scan), first.revision);
    await c.read(soulPagesProvider(spec).notifier).previous();
    expect(c.read(soulPagesProvider(spec)).requireValue.index, 0);
    expect(client.queries.last.hasFirst(), isTrue);
  });

  test('a stale scan restarts from the first page', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(soulPagesProvider(spec), (_, _) {});
    await c.read(soulPagesProvider(spec).future);
    client.onQuery = (q) async => q.hasFirst()
        ? recordedFirstPage()
        : throw refused(pb.Error(queryStaleRevision: pb.QueryStaleRevision()));
    await c.read(soulPagesProvider(spec).notifier).next();
    final restarted = await c.read(soulPagesProvider(spec).future);
    expect(restarted.index, 0);
    expect(client.queries.map((q) => q.hasFirst()), [true, false, true]);
  });

  test('another failed page turn keeps the page and shows the failure', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(soulPagesProvider(spec), (_, _) {});
    await c.read(soulPagesProvider(spec).future);
    client.onQuery = (_) async => throw RaisedFailure.timeout(9, const Duration(seconds: 30));
    await c.read(soulPagesProvider(spec).notifier).next();
    final page = c.read(soulPagesProvider(spec)).requireValue;
    expect(page.index, 0);
    expect(
      page.turn,
      isA<TurnFailed>().having(
        (t) => t.failure,
        'failure',
        raised(pb.ClientFailure_Kind.clientTimeout),
      ),
    );
  });

  test('a projection change past the held revision refetches, and one at it does not', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(soulPagesProvider(spec), (_, _) {});
    c.listen(projectionRevisionProvider, (_, _) {});
    await c.read(soulPagesProvider(spec).future);
    client.emit(pb.Event(projectionChanged: pb.ProjectionChanged(revision: Int64.ONE)));
    await Future<void>.delayed(Duration.zero);
    expect(client.queries, hasLength(1));
    client.emit(pb.Event(projectionChanged: pb.ProjectionChanged(revision: Int64(2))));
    await Future<void>.delayed(Duration.zero);
    await c.read(soulPagesProvider(spec).future);
    expect(client.queries, hasLength(2));
  });

  test('a new session refetches what the old one served', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(profilesProvider, (_, _) {});
    c.listen(sessionEpochProvider, (_, _) {});
    await c.read(profilesProvider.future);
    expect(client.profileCalls, 1);
    client.setHealth(
      DaemonReady(
        daemonVersion: pb.ProtocolVersion(major: 1),
        revision: Int64.ONE,
        session: 2,
        restarted: true,
      ),
    );
    await Future<void>.delayed(Duration.zero);
    await c.read(profilesProvider.future);
    expect(client.profileCalls, 2);
    expect(
      c.read(coreStatusProvider),
      isA<CoreReady>().having((s) => s.restarted, 'restarted', isTrue),
    );
  });

  test('core status follows the client health, with the failure', () async {
    final client = FakeDaemonClient(health: const DaemonStopped());
    final c = containerFor(client);
    c.listen(coreStatusProvider, (_, _) {});
    expect(c.read(coreStatusProvider), isA<CoreStopped>());
    client.setHealth(const DaemonStarting());
    await Future<void>.delayed(Duration.zero);
    expect(c.read(coreStatusProvider), isA<CoreStarting>());
    client.setHealth(
      DaemonRestarting(attempt: 2, cause: RaisedFailure.daemonExited(3, null, asked: false)),
    );
    await Future<void>.delayed(Duration.zero);
    expect(
      c.read(coreStatusProvider),
      isA<CoreRestarting>()
          .having((s) => s.attempt, 'attempt', 2)
          .having((s) => s.cause, 'cause', raised(pb.ClientFailure_Kind.clientDaemonExited)),
    );
    client.setHealth(
      DaemonFailed(refused(pb.Error(sessionProtocolUnsupported: pb.SessionProtocolUnsupported()))),
    );
    await Future<void>.delayed(Duration.zero);
    final failed = c.read(coreStatusProvider) as CoreFailed;
    expect(failed.failure.code, 'session.protocol_unsupported');
    expect(failed.log, isNotEmpty);
  });

  test('warnings from the daemon are collected by kind', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(coreWarningsProvider, (_, _) {});
    client.emit(
      pb.Event(
        warning: pb.Warning(message: 'x', clientOutdated: pb.ClientOutdated()),
      ),
    );
    await Future<void>.delayed(Duration.zero);
    expect(c.read(coreWarningsProvider).single.warning.whichKind(), pb.Warning_Kind.clientOutdated);
  });

  test('importing a scheme keeps the daemon decode, and a failure keeps its case', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(schemeLibraryProvider, (_, _) {});
    await c.read(schemeLibraryProvider.notifier).importText('ignored by the fake');
    final library = c.read(schemeLibraryProvider);
    expect(library.selected?.decoded.entries.map((e) => e.name), ['spd', 'six']);
    client.onDecode = (_) async =>
        throw refused(pb.Error(decodeUnknownFormat: pb.DecodeUnknownFormat()));
    final outcome = await c.read(schemeLibraryProvider.notifier).importText('x');
    expect(
      outcome,
      isA<ImportRejected>().having((o) => o.failure.code, 'code', 'decode.unknown_format'),
    );
    expect(
      c.read(schemeLibraryProvider).status,
      isA<ImportFailed>().having((s) => s.failure.code, 'code', 'decode.unknown_format'),
    );
    expect(c.read(schemeLibraryProvider).imported, hasLength(1));
    expect(c.read(generatedSchemesProvider), isA<GeneratedUnavailable>());
  });

  test('an in-flight import is not doubled', () async {
    final client = FakeDaemonClient();
    final gate = Completer<pb.SchemeCodeDecoded>();
    var calls = 0;
    client.onDecode = (_) {
      calls++;
      return gate.future;
    };
    final c = containerFor(client);
    c.listen(schemeLibraryProvider, (_, _) {});
    final first = c.read(schemeLibraryProvider.notifier).importText('a');
    expect(await c.read(schemeLibraryProvider.notifier).importText('b'), isA<AlreadyImporting>());
    gate.complete(recordedScheme());
    expect(await first, isA<Imported>());
    expect(calls, 1);
  });

  // The bug A1 of the 2026-09-25 audit: a selection or removal during an import rebuilt the state
  // without its import status, so the import controls came back and a second import could start.
  test('selecting or removing during an import keeps it running, and refuses a second', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(schemeLibraryProvider, (_, _) {});
    final library = c.read(schemeLibraryProvider.notifier);
    final firstKey = switch (await library.importText('first')) {
      Imported(:final key) => key,
      final other => fail('expected an import, got $other'),
    };
    final gate = Completer<pb.SchemeCodeDecoded>();
    var calls = 0;
    client.onDecode = (_) {
      calls++;
      return gate.future;
    };
    final pending = library.importText('second');
    library.select(firstKey);
    expect(c.read(schemeLibraryProvider).status, isA<ImportRunning>());
    library.remove(firstKey);
    expect(c.read(schemeLibraryProvider).status, isA<ImportRunning>());
    expect(c.read(schemeLibraryProvider).selection, isA<NoSchemeSelected>());
    expect(await library.importText('third'), isA<AlreadyImporting>());
    gate.complete(recordedScheme());
    expect(await pending, isA<Imported>());
    expect(calls, 1);
    expect(c.read(schemeLibraryProvider).status, isA<ImportIdle>());
  });

  test('a selection never names a scheme that is not in the list', () async {
    final c = containerFor(FakeDaemonClient());
    c.listen(schemeLibraryProvider, (_, _) {});
    c.read(schemeLibraryProvider.notifier).select(42);
    expect(c.read(schemeLibraryProvider).selection, isA<NoSchemeSelected>());
  });
}
