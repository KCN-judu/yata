/// The soul inventory as pages of the daemon's query (`core-protocol.md`, "Queries and pages").
///
/// One page is held at a time, so the Flutter heap is never the inventory store (ADR-0004, rule
/// 12). Moving forward sends the previous page's cursor and the scan's revision; a scan whose
/// revision went stale restarts from the first page, as the protocol requires. The order and the
/// selection are the daemon's: nothing here sorts or filters a row.
library;

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'core.dart';
import 'profiles.dart';

/// Everything a soul query is keyed by. The filter pane and the sort control attach here: when
/// the schema carries `query.md`'s filter and sort, they become fields of this key and are sent
/// as query parameters, so a changed filter is a new provider, never a list operation in Dart.
final class SoulQuerySpec {
  const SoulQuerySpec({required this.profileId, this.rowBudget = defaultRowBudget});

  static const defaultRowBudget = 200;

  final String profileId;
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

/// One page of a scan.
final class SoulPage {
  const SoulPage({
    required this.rows,
    required this.total,
    required this.revision,
    required this.index,
    required this.firstRow,
    required this.hasMore,
    this.turning = false,
    this.turnError,
  });

  /// The rows as the daemon returned them: each soul's values and its verdict, exact or open
  /// (ADR-0026).
  final List<pb.QueryRow> rows;

  List<pb.Soul> get souls => [for (final r in rows) r.soul];

  /// Every row the query selects, across all pages.
  final Int64 total;

  /// The revision the scan is valid at.
  final Int64 revision;

  /// Zero-based page number within the scan.
  final int index;

  /// The zero-based position of the page's first row within the scan.
  final int firstRow;
  final bool hasMore;

  /// A neighbouring page is being fetched; this one stays shown meanwhile.
  final bool turning;

  /// Why the last page turn failed, shown beside this page.
  final CoreError? turnError;

  bool get hasPrevious => index > 0;

  SoulPage copyWith({bool? turning, CoreError? turnError}) => SoulPage(
    rows: rows,
    total: total,
    revision: revision,
    index: index,
    firstRow: firstRow,
    hasMore: hasMore,
    turning: turning ?? this.turning,
    turnError: turnError,
  );
}

/// The query for one page of [spec]. Only non-default scalars are set, so the bytes match the
/// Rust encoder's (see `clientProtocolVersion`).
pb.Query soulQuery(SoulQuerySpec spec, {List<int> cursor = const [], Int64? scan}) {
  final page = pb.PageRequest(rowBudget: spec.rowBudget);
  if (cursor.isNotEmpty) page.cursor = cursor;
  final query = pb.Query(
    profileId: spec.profileId,
    collection: pb.Collection.COLLECTION_SOULS,
    page: page,
  );
  if (scan != null) query.scanRevision = scan;
  return query;
}

class SoulPages extends AsyncNotifier<SoulPage> {
  SoulPages(this.spec);

  final SoulQuerySpec spec;

  /// The cursor that fetches page `i` is `_cursors[i]`; page 0's is empty.
  final List<List<int>> _cursors = [];
  final List<int> _firstRows = [];
  List<int> _nextCursor = const [];

  @override
  Future<SoulPage> build() async {
    ref.watch(sessionEpochProvider);
    ref.listen(projectionRevisionProvider, (_, held) {
      final page = state.value;
      if (page != null && page.revision < held) ref.invalidateSelf();
    });
    _cursors
      ..clear()
      ..add(const []);
    _firstRows
      ..clear()
      ..add(0);
    return _fetch(0, scan: null);
  }

  Future<SoulPage> _fetch(int index, {required Int64? scan}) async {
    final query = soulQuery(spec, cursor: _cursors[index], scan: scan);
    final r = await coreCall(ref.read(daemonClientProvider).query(query));
    ref.read(projectionRevisionProvider.notifier).observe(r.revision);
    _nextCursor = r.hasNextCursor() ? r.nextCursor : const [];
    return SoulPage(
      rows: r.rows,
      total: r.total,
      revision: r.revision,
      index: index,
      firstRow: _firstRows[index],
      hasMore: r.hasNextCursor(),
    );
  }

  Future<void> next() => _turn(forward: true);

  Future<void> previous() => _turn(forward: false);

  Future<void> _turn({required bool forward}) async {
    final page = state.value;
    if (page == null || page.turning) return;
    if (forward ? !page.hasMore : !page.hasPrevious) return;
    final index = forward ? page.index + 1 : page.index - 1;
    if (forward && _cursors.length == index) {
      _cursors.add(_nextCursor);
      _firstRows.add(page.firstRow + page.souls.length);
    }
    state = AsyncData(page.copyWith(turning: true));
    try {
      state = AsyncData(await _fetch(index, scan: page.revision));
    } on CoreError catch (e) {
      if (e.code == 'query.stale_revision') {
        // The projection moved under the scan: start it again from the first page.
        ref.invalidateSelf();
        return;
      }
      state = AsyncData(page.copyWith(turnError: e));
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
