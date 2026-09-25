/// Scheme codes: the ones the user imported, and the ones the application will generate.
///
/// Decoding, the selection each plan means, and the QR matrix are the daemon's
/// (`scheme-code.md`, "Codec"); this file keeps the decoded results the views show. Imported
/// codes live for the session only: saving a scheme is a command the protocol does not have yet.
library;

import 'dart:typed_data';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'core.dart';

/// A code the user brought in, as the daemon decoded it.
final class ImportedScheme {
  const ImportedScheme({required this.key, required this.decoded});

  /// Identifies the entry in this session's list; not a domain id.
  final int key;
  final pb.SchemeCodeDecoded decoded;
}

/// Whether an import is running, and how the last one ended.
sealed class ImportStatus {
  const ImportStatus();
}

final class ImportIdle extends ImportStatus {
  const ImportIdle();
}

/// An import is being decoded; no second one starts until it ends.
final class ImportRunning extends ImportStatus {
  const ImportRunning();
}

/// The last import failed; the next attempt clears it.
final class ImportFailed extends ImportStatus {
  const ImportFailed(this.failure);

  final CoreFailure failure;
}

/// Which imported scheme the detail pane shows.
sealed class SchemeSelection {
  const SchemeSelection();
}

final class NoSchemeSelected extends SchemeSelection {
  const NoSchemeSelected();
}

final class SchemeSelected extends SchemeSelection {
  const SchemeSelected(this.key);

  final int key;
}

final class SchemeLibraryState {
  const SchemeLibraryState() : this._(const [], const NoSchemeSelected(), const ImportIdle());

  const SchemeLibraryState._(this.imported, this.selection, this.status);

  /// The list and its selection, changed together: a selection that names no entry of [imported]
  /// becomes [NoSchemeSelected], so a selection never dangles.
  factory SchemeLibraryState._listed(
    List<ImportedScheme> imported,
    SchemeSelection selection,
    ImportStatus status,
  ) => SchemeLibraryState._(List.unmodifiable(imported), switch (selection) {
    SchemeSelected(:final key) when imported.any((s) => s.key == key) => selection,
    SchemeSelected() || NoSchemeSelected() => const NoSchemeSelected(),
  }, status);

  final List<ImportedScheme> imported;
  final SchemeSelection selection;
  final ImportStatus status;

  /// The same list and selection with another import status.
  SchemeLibraryState withStatus(ImportStatus status) =>
      SchemeLibraryState._(imported, selection, status);

  /// Another list and selection, with the status kept.
  SchemeLibraryState withList(List<ImportedScheme> imported, SchemeSelection selection) =>
      SchemeLibraryState._listed(imported, selection, status);

  ImportedScheme? get selected => switch (selection) {
    SchemeSelected(:final key) => imported.where((s) => s.key == key).firstOrNull,
    NoSchemeSelected() => null,
  };
}

/// How one import request ended, for the control that made it.
sealed class ImportOutcome {
  const ImportOutcome();
}

final class Imported extends ImportOutcome {
  const Imported(this.key);

  final int key;
}

/// Refused: another import was still running.
final class AlreadyImporting extends ImportOutcome {
  const AlreadyImporting();
}

final class ImportRejected extends ImportOutcome {
  const ImportRejected(this.failure);

  final CoreFailure failure;
}

/// The library was disposed while the import ran; nothing was added.
final class ImportAbandoned extends ImportOutcome {
  const ImportAbandoned();
}

class SchemeLibrary extends Notifier<SchemeLibraryState> {
  int _nextKey = 1;

  @override
  SchemeLibraryState build() => const SchemeLibraryState();

  /// Import the Base64 text of a scheme code.
  Future<ImportOutcome> importText(String text) => _import(pb.DecodeSchemeCode(text: text));

  /// Import a PNG image holding one scheme QR code.
  Future<ImportOutcome> importPng(Uint8List png) => _import(pb.DecodeSchemeCode(png: png));

  Future<ImportOutcome> _import(pb.DecodeSchemeCode request) async {
    if (state.status is ImportRunning) return const AlreadyImporting();
    state = state.withStatus(const ImportRunning());
    try {
      final decoded = await coreCall(
        () => ref.read(daemonClientProvider).decodeSchemeCode(request),
      );
      if (!ref.mounted) return const ImportAbandoned();
      final entry = ImportedScheme(key: _nextKey++, decoded: decoded);
      state = state
          .withList([...state.imported, entry], SchemeSelected(entry.key))
          .withStatus(const ImportIdle());
      return Imported(entry.key);
    } on CoreFailure catch (e) {
      if (!ref.mounted) return const ImportAbandoned();
      state = state.withStatus(ImportFailed(e));
      return ImportRejected(e);
    } finally {
      // Whatever ended the import, it no longer runs: no path leaves the controls disabled.
      if (ref.mounted && state.status is ImportRunning) {
        state = state.withStatus(const ImportIdle());
      }
    }
  }

  /// Show [key]; a key that is not in the list selects nothing.
  void select(int key) => state = state.withList(state.imported, SchemeSelected(key));

  void remove(int key) => state = state.withList([
    for (final s in state.imported)
      if (s.key != key) s,
  ], state.selection);
}

final schemeLibraryProvider = NotifierProvider<SchemeLibrary, SchemeLibraryState>(
  SchemeLibrary.new,
);

/// Schemes the application proposes from the inventory: the discard and strengthen
/// recommendations of `feature-scope.md`. The daemon produces them from the scores, which do not
/// exist yet, so the source is unavailable, which is not the same as available and empty.
sealed class GeneratedSchemes {
  const GeneratedSchemes();
}

final class GeneratedUnavailable extends GeneratedSchemes {
  const GeneratedUnavailable();
}

final class GeneratedAvailable extends GeneratedSchemes {
  const GeneratedAvailable(this.schemes);

  final List<pb.SchemeCodeDecoded> schemes;
}

final generatedSchemesProvider = Provider<GeneratedSchemes>((ref) => const GeneratedUnavailable());
