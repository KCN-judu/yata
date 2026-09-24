/// Draws a QR module matrix the daemon produced. Drawing is presentation; making the matrix is
/// encoding, which the application never does (`scheme-code.md`, "Codec").
library;

import 'package:flutter/material.dart';

import '../../gen/proto/core.pb.dart' as pb;

class QrMatrixView extends StatelessWidget {
  const QrMatrixView({
    super.key,
    required this.matrix,
    required this.size,
    required this.semanticLabel,
  });

  final pb.QrMatrix matrix;
  final double size;
  final String semanticLabel;

  @override
  Widget build(BuildContext context) => Semantics(
    label: semanticLabel,
    image: true,
    child: CustomPaint(size: Size.square(size), painter: _QrPainter(matrix)),
  );
}

class _QrPainter extends CustomPainter {
  _QrPainter(this.matrix);

  final pb.QrMatrix matrix;

  /// The standard's quiet zone, in modules, on every side.
  static const quiet = 4;

  @override
  void paint(Canvas canvas, Size size) {
    // Black on white in both themes: a scanner needs the contrast the standard assumes.
    canvas.drawRect(Offset.zero & size, Paint()..color = Colors.white);
    final n = matrix.size;
    if (n == 0 || matrix.modules.length != n * n) return;
    final module = size.width / (n + 2 * quiet);
    final dark = Paint()..color = Colors.black;
    for (var y = 0; y < n; y++) {
      for (var x = 0; x < n; x++) {
        if (matrix.modules[y * n + x] != 0) {
          canvas.drawRect(
            Rect.fromLTWH((x + quiet) * module, (y + quiet) * module, module, module),
            dark,
          );
        }
      }
    }
  }

  @override
  bool shouldRepaint(_QrPainter old) => !identical(old.matrix, matrix);
}
