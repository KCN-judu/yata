/// The soul inventory as pages of the daemon's query (`core-protocol.md`, "Queries and pages").
///
/// One page is held at a time, so the Flutter heap is never the inventory store (ADR-0004, rule
/// 12). A later page sends the previous page's cursor with the revision its scan began at; a scan
/// whose revision went stale restarts from the first page, as the protocol requires. The order and
/// the selection are the daemon's: nothing here sorts or filters a row.
library;

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'core.dart';
import 'profiles.dart';

/// Everything a soul query is keyed by. The filter pane and the sort control attach here: when
/// they exist, they become fields of this key and are sent in the query, so a changed filter is a
/// new provider, never a list operation in Dart.
final class SoulQuerySpec {
  const SoulQuerySpec({required this.profileId, this.rowBudget = defaultRowBudget});

  static const defaultRowBudget = 200;

  final ProfileId profileId;
  final int rowBudget;

  @override
  bool operator ==(Object other) =>
      other is SoulQuerySpec && other.profileId == profileId && other.rowBudget == rowBudget;

  @override
  int get hashCode => Object.hash(profileId, rowBudget);
}

/// The query of the selected profile; `null` while no profile exists.
final soulQuerySpecProvider = Provider<SoulQuerySpec?>((ref) {
  final profile = ref.watch(selectedProfileProvider);
  return profile == null ? null : SoulQuerySpec(profileId: profile);
});

/// Which page of a scan a query asks for.
sealed class QueryPosition {
  const QueryPosition();
}

final class FirstPosition extends QueryPosition {
  const FirstPosition();
}

/// The page after the one whose [cursor] this is, in the scan that began at [scan].
final class NextPosition extends QueryPosition {
  const NextPosition(this.cursor, this.scan);

  final List<int> cursor;
  final Revision scan;
}

/// Where a page of the scan begins: how to ask for it again, and its first row's position.
final class PageAnchor {
  const PageAnchor(this.position, this.firstRow);

  final QueryPosition position;

  /// The zero-based position of the page's first row within the scan.
  final int firstRow;
}

/// The page turn in progress, or how the last one ended.
sealed class TurnState {
  const TurnState();
}

final class TurnSettled extends TurnState {
  const TurnSettled();
}

final class Turning extends TurnState {
  const Turning({required this.forward});

  final bool forward;
}

/// The last turn failed; this page stays shown beside the failure.
final class TurnFailed extends TurnState {
  const TurnFailed(this.failure);

  final CoreFailure failure;
}

/// One page of a scan, with the way back to every page before it.
final class SoulPage {
  const SoulPage({
    required this.rows,
    required this.total,
    required this.revision,
    required this.anchors,
    required this.next,
    this.turn = const TurnSettled(),
  });

  /// The rows as the daemon returned them: each soul with its verdict, exact or open (ADR-0026).
  final List<pb.SessionRow> rows;

  List<pb.Soul> get souls => [for (final r in rows) r.soul];

  /// Every row the query keeps, across all pages.
  final Int64 total;

  /// The revision the scan is valid at.
  final Revision revision;

  /// This page's anchor and those of the pages before it, first page first. Never empty.
  final List<PageAnchor> anchors;

  /// The cursor of the page after this one; `null` on the last page.
  final List<int>? next;

  final TurnState turn;

  int get index => anchors.length - 1;
  int get firstRow => anchors.last.firstRow;
  bool get hasMore => next != null;
  bool get hasPrevious => anchors.length > 1;
  bool get turning => turn is Turning;

  SoulPage withTurn(TurnState turn) => SoulPage(
    rows: rows,
    total: total,
    revision: revision,
    anchors: anchors,
    next: next,
    turn: turn,
  );
}

/// The session query for one page of [spec]. Each field follows `clientProtocolVersion`'s encoding
/// rule, so the bytes match the Rust encoder's.
pb.SessionQuery soulQuery(SoulQuerySpec spec, QueryPosition position) {
  final query = pb.SessionQuery(
    profileId: spec.profileId.hex,
    query: pb.Query(collection: pb.Collection.COLLECTION_SOULS),
    rowBudget: spec.rowBudget,
  );
  switch (position) {
    case FirstPosition():
      query.first = pb.FirstPage();
    case NextPosition(:final cursor, :final scan):
      final next = pb.NextPage(cursor: cursor);
      if (scan.seq != Int64.ZERO) next.scan = scan.seq;
      query.next = next;
  }
  return query;
}

class SoulPages extends AsyncNotifier<SoulPage> {
  SoulPages(this.spec);

  final SoulQuerySpec spec;

  @override
  Future<SoulPage> build() async {
    ref.watch(sessionEpochProvider);
    ref.listen(projectionRevisionProvider, (_, held) {
      final page = state.value;
      if (page != null && isStale(page.revision, held)) ref.invalidateSelf();
    });
    return _fetch(const [PageAnchor(FirstPosition(), 0)]);
  }

  /// The page the last of [anchors] names.
  Future<SoulPage> _fetch(List<PageAnchor> anchors) async {
    final r = await coreCall(
      () => ref.read(daemonClientProvider).query(soulQuery(spec, anchors.last.position)),
    );
    final revision = Revision(r.revision);
    if (ref.mounted) ref.read(projectionRevisionProvider.notifier).observe(revision);
    return SoulPage(
      rows: r.rows,
      total: r.total,
      revision: revision,
      anchors: List.unmodifiable(anchors),
      next: r.hasNextCursor() ? r.nextCursor : null,
    );
  }

  Future<void> next() => _turn(forward: true);

  Future<void> previous() => _turn(forward: false);

  Future<void> _turn({required bool forward}) async {
    final page = state.value;
    if (page == null || page.turning) return;
    final List<PageAnchor> anchors;
    if (forward) {
      final cursor = page.next;
      if (cursor == null) return;
      anchors = [
        ...page.anchors,
        PageAnchor(NextPosition(cursor, page.revision), page.firstRow + page.rows.length),
      ];
    } else {
      if (!page.hasPrevious) return;
      anchors = page.anchors.sublist(0, page.anchors.length - 1);
    }
    state = AsyncData(page.withTurn(Turning(forward: forward)));
    try {
      final turned = await _fetch(anchors);
      if (ref.mounted) state = AsyncData(turned);
    } on CoreFailure catch (e) {
      if (!ref.mounted) return;
      if (e case RequestFailure(
        :final error,
      ) when error.whichKind() == pb.Error_Kind.queryStaleRevision) {
        // The projection moved under the scan: start it again from the first page.
        ref.invalidateSelf();
        return;
      }
      state = AsyncData(page.withTurn(TurnFailed(e)));
    }
  }
}

final soulPagesProvider = AsyncNotifierProvider.family<SoulPages, SoulPage, SoulQuerySpec>(
  SoulPages.new,
);

/// The soul the detail pane shows: the daemon's row as it was on the page it was chosen from.
/// View state; it is cleared when the profile changes.
class SelectedSoul extends Notifier<pb.Soul?> {
  @override
  pb.Soul? build() {
    ref.watch(selectedProfileProvider);
    return null;
  }

  void select(pb.Soul? soul) => state = soul;
}

final selectedSoulProvider = NotifierProvider<SelectedSoul, pb.Soul?>(SelectedSoul.new);
