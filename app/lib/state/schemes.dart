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
  const ImportFailed(this.error);

  final CoreError error;
}

/// Which imported scheme the detail pane shows. A selection always names an entry of the list.
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
  const SchemeLibraryState({
    this.imported = const [],
    this.selection = const NoSchemeSelected(),
    this.status = const ImportIdle(),
  });

  final List<ImportedScheme> imported;
  final SchemeSelection selection;
  final ImportStatus status;

  /// Every field this call does not name is kept as it is, so no transition resets another.
  SchemeLibraryState copyWith({
    List<ImportedScheme>? imported,
    SchemeSelection? selection,
    ImportStatus? status,
  }) => SchemeLibraryState(
    imported: imported ?? this.imported,
    selection: selection ?? this.selection,
    status: status ?? this.status,
  );

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
  const ImportRejected(this.error);

  final CoreError error;
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
    state = state.copyWith(status: const ImportRunning());
    try {
      final decoded = await coreCall(ref.read(daemonClientProvider).decodeSchemeCode(request));
      final entry = ImportedScheme(key: _nextKey++, decoded: decoded);
      if (ref.mounted) {
        state = state.copyWith(
          imported: [...state.imported, entry],
          selection: SchemeSelected(entry.key),
          status: const ImportIdle(),
        );
      }
      return Imported(entry.key);
    } on CoreError catch (e) {
      if (ref.mounted) state = state.copyWith(status: ImportFailed(e));
      return ImportRejected(e);
    }
  }

  /// Show [key]; a key that is not in the list is refused, so the selection never dangles.
  void select(int key) {
    if (state.imported.any((s) => s.key == key)) {
      state = state.copyWith(selection: SchemeSelected(key));
    }
  }

  void remove(int key) => state = state.copyWith(
    imported: [
      for (final s in state.imported)
        if (s.key != key) s,
    ],
    selection: switch (state.selection) {
      SchemeSelected(key: final k) when k == key => const NoSchemeSelected(),
      final other => other,
    },
  );
}

final schemeLibraryProvider = NotifierProvider<SchemeLibrary, SchemeLibraryState>(
  SchemeLibrary.new,
);

/// Schemes the application proposes from the inventory: the discard and strengthen
/// recommendations of `feature-scope.md`. They are produced by the daemon from the scores, which
/// do not exist yet, so the source reports itself unavailable rather than empty.
final class GeneratedSchemes {
  const GeneratedSchemes.unavailable() : available = false, schemes = const [];

  final bool available;
  final List<pb.SchemeCodeDecoded> schemes;
}

final generatedSchemesProvider = Provider<GeneratedSchemes>(
  (ref) => const GeneratedSchemes.unavailable(),
);
