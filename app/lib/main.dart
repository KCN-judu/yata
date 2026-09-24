import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'ui/app.dart';

void main() {
  // No automatic retry: a failed query shows its error and a retry action, and the daemon
  // client decides restarts itself (ADR-0012).
  runApp(ProviderScope(retry: (_, _) => null, child: const YataApp()));
}
