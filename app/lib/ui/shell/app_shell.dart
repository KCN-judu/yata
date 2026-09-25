/// The window: a navigation rail, the place's surface, the core banner above it, and the status
/// bar below. Navigation is the simplest that shows the milestone's places (ADR-0012,
/// "Routing"): a rail and an index, with no routes until the information architecture is
/// settled.
library;

import 'package:flutter/material.dart';

import '../../gen/l10n/app_localizations.dart';
import '../inventory/inventory_page.dart';
import '../schemes/schemes_page.dart';
import '../settings/settings_page.dart';
import '../shikigami/shikigami_page.dart';
import 'core_banner.dart';
import 'status_bar.dart';

enum Place { inventory, schemes, shikigami, settings }

class AppShell extends StatefulWidget {
  const AppShell({super.key});

  @override
  State<AppShell> createState() => _AppShellState();
}

class _AppShellState extends State<AppShell> {
  Place _place = Place.inventory;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    // Each place's rail entry, by an exhaustive switch: a new place has to name its own.
    (IconData, IconData, String) entry(Place p) => switch (p) {
      Place.inventory => (Icons.grid_view_outlined, Icons.grid_view, l.navInventory),
      Place.schemes => (Icons.qr_code_2_outlined, Icons.qr_code_2, l.navSchemes),
      Place.shikigami => (Icons.people_outline, Icons.people, l.navShikigami),
      Place.settings => (Icons.settings_outlined, Icons.settings, l.navSettings),
    };
    return Scaffold(
      body: Column(
        children: [
          Expanded(
            child: Row(
              children: [
                NavigationRail(
                  selectedIndex: _place.index,
                  labelType: NavigationRailLabelType.all,
                  minWidth: 64,
                  onDestinationSelected: (i) => setState(() => _place = Place.values[i]),
                  destinations: [
                    for (final (icon, selected, label) in Place.values.map(entry))
                      NavigationRailDestination(
                        icon: Icon(icon),
                        selectedIcon: Icon(selected),
                        label: Text(label),
                      ),
                  ],
                ),
                const VerticalDivider(width: 1),
                Expanded(
                  child: Column(
                    children: [
                      const CoreBanner(),
                      Expanded(
                        child: switch (_place) {
                          Place.inventory => const InventoryPage(),
                          Place.schemes => const SchemesPage(),
                          Place.shikigami => const ShikigamiPage(),
                          Place.settings => const SettingsPage(),
                        },
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
          const Divider(height: 1),
          const StatusBar(),
        ],
      ),
    );
  }
}
