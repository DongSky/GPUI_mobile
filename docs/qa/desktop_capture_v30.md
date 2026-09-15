# Desktop capture v30

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v30 commit after `811e419` (v29).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) + Compose `WideNavigationRailItem` `iconPositionFor` (Top collapsed / Start expanded).

v30 started from v29 leftovers (`docs/qa/v29_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was WideNavigationRailItem icon position still folded into the 80↔220 modal overlay. Live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | Compose `WideNavigationRailItemDefaults.iconPositionFor`: Top collapsed / Start expanded. Vertical 56×32 + label-medium; horizontal 56dp full-width pill + 16dp lead/trail + 8dp gap + label-large. Wide collapsed 96 / expanded max 360. Active label **secondary** (`NavigationRailColorTokens.ItemActiveLabelText`) |
| Catalog | `data-hero="wide-rail"` pair (96 Top + 220 Start) beside the existing modal 80↔220 overlay; dests carry `data-icon-position` + `--ind` |
| Hosts | `material_desktop_demo` + `component_demo` paint the pair + Start 56dp pills from `item_metrics` |
| Inventory | Navigation rail notes WideNavigationRailItem Top/Start |
| Tests | Icon-position tokens; catalog evidence; desktop 96 / 56 / secondary |

Host tests (this VM): **33** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive wide-rail Top/Start against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Collapsed Top | 96dp wide rail, 56×32 indicator, label-medium under icon | Narrow 80dp still used by the modal overlay toggle |
| Expanded Start | 220dp, 56dp full-round pill, icon+label, label-large | Non-modal expanded (no scrim) is the pair only; modal expanded stays overlay |
| Active label | Secondary (May 2025 Expressive mapping) | — |
| Modal overlay | 80↔220 + 32% scrim, FAB toggle | OS `PopUp` still unopened on Linux/NativeActivity |

## v31 leftovers

See `docs/qa/v30_leftovers.md`.
