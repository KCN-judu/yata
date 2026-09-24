// The state layer over a fake daemon client: loading, data, and error by code; paging by cursor;
// a stale scan restarting; revision-driven refetch; a new session refetching; core status.

import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/daemon_client.dart';
import 'package:yata/daemon/daemon_error.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/state/core.dart';
import 'package:yata/state/inventory.dart';
import 'package:yata/state/profiles.dart';
import 'package:yata/state/schemes.dart';

import '../support/fake_daemon.dart';
import '../support/recorded.dart';

ProviderContainer containerFor(FakeDaemonClient client) =>
    ProviderContainer.test(overrides: fakeOverrides(client), retry: (_, _) => null);

const spec = SoulQuerySpec(profileId: 'fixture', rowBudget: 8);

void main() {
  test('profiles load, and the first becomes the selection', () async {
    final c = containerFor(FakeDaemonClient());
    expect(c.read(profilesProvider), isA<AsyncLoading<List<pb.Profile>>>());
    await c.read(profilesProvider.future);
    expect(c.read(selectedProfileProvider), 'fixture');
    c.read(selectedProfileProvider.notifier).select('fixture-empty');
    expect(c.read(soulQuerySpecProvider)?.profileId, 'fixture-empty');
  });

  test('a failed call surfaces as a CoreError carrying the code', () async {
    final client = FakeDaemonClient()
      ..onListProfiles = () async => throw daemonError('query.unknown_profile');
    final c = containerFor(client);
    c.listen(profilesProvider, (_, _) {});
    await expectLater(c.read(profilesProvider.future), throwsA(isA<CoreError>()));
    final error = c.read(profilesProvider).error;
    expect(error, isA<CoreError>().having((e) => e.code, 'code', 'query.unknown_profile'));
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
    expect(client.queries.last.page.cursor, recordedFirstPage().cursor);
    expect(client.queries.last.scanRevision, first.revision);
    await c.read(soulPagesProvider(spec).notifier).previous();
    expect(c.read(soulPagesProvider(spec)).requireValue.index, 0);
    expect(client.queries.last.page.hasCursor(), isFalse);
  });

  test('a stale scan restarts from the first page', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(soulPagesProvider(spec), (_, _) {});
    await c.read(soulPagesProvider(spec).future);
    client.onQuery = (q) async =>
        q.page.cursor.isEmpty ? recordedFirstPage() : throw daemonError('query.stale_revision');
    await c.read(soulPagesProvider(spec).notifier).next();
    final restarted = await c.read(soulPagesProvider(spec).future);
    expect(restarted.index, 0);
    expect(client.queries.map((q) => q.page.cursor.isEmpty), [true, false, true]);
  });

  test('another failed page turn keeps the page and shows the error', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(soulPagesProvider(spec), (_, _) {});
    await c.read(soulPagesProvider(spec).future);
    client.onQuery = (_) async => throw daemonError(ClientErrorCode.timeout);
    await c.read(soulPagesProvider(spec).notifier).next();
    final page = c.read(soulPagesProvider(spec)).requireValue;
    expect(page.index, 0);
    expect(page.turnError?.code, ClientErrorCode.timeout);
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

  test('core status follows the client health, with the failure code', () async {
    final client = FakeDaemonClient(health: const DaemonStopped());
    final c = containerFor(client);
    c.listen(coreStatusProvider, (_, _) {});
    expect(c.read(coreStatusProvider), isA<CoreStopped>());
    client.setHealth(DaemonStarting(attempt: 2, after: daemonError(ClientErrorCode.daemonExited)));
    await Future<void>.delayed(Duration.zero);
    expect(
      c.read(coreStatusProvider),
      isA<CoreStarting>().having((s) => s.cause?.code, 'cause', ClientErrorCode.daemonExited),
    );
    client.setHealth(DaemonFailed(daemonError('session.protocol_unsupported')));
    await Future<void>.delayed(Duration.zero);
    final failed = c.read(coreStatusProvider) as CoreFailed;
    expect(failed.error.code, 'session.protocol_unsupported');
    expect(failed.log, isNotEmpty);
  });

  test('warnings from the daemon are collected by code', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(coreWarningsProvider, (_, _) {});
    client.emit(
      pb.Event(
        warning: pb.Warning(code: 'session.client_outdated', message: 'x'),
      ),
    );
    await Future<void>.delayed(Duration.zero);
    expect(c.read(coreWarningsProvider).single.code, 'session.client_outdated');
  });

  test('importing a scheme keeps the daemon decode, and a failure keeps its code', () async {
    final client = FakeDaemonClient();
    final c = containerFor(client);
    c.listen(schemeLibraryProvider, (_, _) {});
    await c.read(schemeLibraryProvider.notifier).importText('ignored by the fake');
    final library = c.read(schemeLibraryProvider);
    expect(library.selected?.decoded.entries.map((e) => e.name), ['spd', 'six']);
    client.onDecode = (_) async => throw daemonError('decode.unknown_format');
    final outcome = await c.read(schemeLibraryProvider.notifier).importText('x');
    expect(
      outcome,
      isA<ImportRejected>().having((o) => o.error.code, 'code', 'decode.unknown_format'),
    );
    expect(
      c.read(schemeLibraryProvider).status,
      isA<ImportFailed>().having((s) => s.error.code, 'code', 'decode.unknown_format'),
    );
    expect(c.read(schemeLibraryProvider).imported, hasLength(1));
    expect(c.read(generatedSchemesProvider).available, isFalse);
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
