/// Draws a QR module matrix the daemon produced. Drawing is presentation; making the matrix is
/// encoding, which the application never does (`scheme-code.md`, "Codec").
library;

import 'dart:typed_data';

import 'package:flutter/material.dart';

import '../../gen/proto/core.pb.dart' as pb;

/// A QR matrix checked once: square, one byte per module, each 0 or 1. Only [parse] builds one, so
/// a view never draws a matrix it cannot draw correctly.
final class QrModules {
  const QrModules._(this.size, this._dark);

  final int size;
  final Uint8List _dark;

  bool isDark(int x, int y) => _dark[y * size + x] == 1;

  /// The matrix, or `null` when the message is not one: an empty or non-square matrix, or a
  /// module that is neither 0 nor 1.
  static QrModules? parse(pb.QrMatrix m) {
    final n = m.size;
    if (n == 0 || m.modules.length != n * n || m.modules.any((b) => b > 1)) return null;
    return QrModules._(n, Uint8List.fromList(m.modules));
  }
}

class QrMatrixView extends StatelessWidget {
  const QrMatrixView({
    super.key,
    required this.modules,
    required this.size,
    required this.semanticLabel,
  });

  final QrModules modules;
  final double size;
  final String semanticLabel;

  @override
  Widget build(BuildContext context) => Semantics(
    label: semanticLabel,
    image: true,
    child: CustomPaint(size: Size.square(size), painter: _QrPainter(modules)),
  );
}

class _QrPainter extends CustomPainter {
  _QrPainter(this.modules);

  final QrModules modules;

  /// The standard's quiet zone, in modules, on every side.
  static const quiet = 4;

  @override
  void paint(Canvas canvas, Size size) {
    // Black on white in both themes: a scanner needs the contrast the standard assumes.
    canvas.drawRect(Offset.zero & size, Paint()..color = Colors.white);
    final n = modules.size;
    final module = size.width / (n + 2 * quiet);
    final dark = Paint()..color = Colors.black;
    for (var y = 0; y < n; y++) {
      for (var x = 0; x < n; x++) {
        if (modules.isDark(x, y)) {
          canvas.drawRect(
            Rect.fromLTWH((x + quiet) * module, (y + quiet) * module, module, module),
            dark,
          );
        }
      }
    }
  }

  @override
  bool shouldRepaint(_QrPainter old) => !identical(old.modules, modules);
}
