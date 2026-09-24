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

final class SchemeLibraryState {
  const SchemeLibraryState({
    this.imported = const [],
    this.selectedKey,
    this.importing = false,
    this.importError,
  });

  final List<ImportedScheme> imported;
  final int? selectedKey;
  final bool importing;

  /// Why the last import failed; cleared by the next attempt.
  final CoreError? importError;

  ImportedScheme? get selected => imported.where((s) => s.key == selectedKey).firstOrNull;
}

class SchemeLibrary extends Notifier<SchemeLibraryState> {
  int _nextKey = 1;

  @override
  SchemeLibraryState build() => const SchemeLibraryState();

  /// Import the Base64 text of a scheme code.
  Future<void> importText(String text) => _import(pb.DecodeSchemeCode(text: text));

  /// Import a PNG image holding one scheme QR code.
  Future<void> importPng(Uint8List png) => _import(pb.DecodeSchemeCode(png: png));

  Future<void> _import(pb.DecodeSchemeCode request) async {
    if (state.importing) return;
    state = SchemeLibraryState(
      imported: state.imported,
      selectedKey: state.selectedKey,
      importing: true,
    );
    try {
      final decoded = await coreCall(ref.read(daemonClientProvider).decodeSchemeCode(request));
      final entry = ImportedScheme(key: _nextKey++, decoded: decoded);
      state = SchemeLibraryState(imported: [...state.imported, entry], selectedKey: entry.key);
    } on CoreError catch (e) {
      state = SchemeLibraryState(
        imported: state.imported,
        selectedKey: state.selectedKey,
        importError: e,
      );
    }
  }

  void select(int key) => state = SchemeLibraryState(imported: state.imported, selectedKey: key);

  void remove(int key) => state = SchemeLibraryState(
    imported: [
      for (final s in state.imported)
        if (s.key != key) s,
    ],
    selectedKey: state.selectedKey == key ? null : state.selectedKey,
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
