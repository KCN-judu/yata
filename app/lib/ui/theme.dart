/// A plain theme from the one accent (PRP-0002, "Accent: 紫"). Final visual design is the UI
/// work's; this only keeps the shell legible in both brightnesses.
library;

import 'package:flutter/material.dart';

const _accent = Color(0xFF6B3FA0);

ThemeData yataTheme(Brightness brightness) => ThemeData(
  colorScheme: ColorScheme.fromSeed(seedColor: _accent, brightness: brightness),
  visualDensity: VisualDensity.compact,
);
