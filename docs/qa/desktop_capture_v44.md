# Desktop capture v44

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v44 commit after `f2010d5` (v43).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `WideNavigationRailDefaults.ContentPadding`.

v44 started from v43 leftovers (`docs/qa/v43_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the missing Compose `WideNavigationRailDefaults.ContentPadding` (`WNRVerticalPadding` = `NavigationRailCollapsedTokens.TopSpace` 44, start/end 0). Hosts and the HTML catalog still used a 16dp top inset (Android columns had none). Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `RailContentPadding`; ContentPadding 0/44/0/44; `PAD_TOP_DP` / `PAD_BOTTOM_DP` alias TopSpace |
| Catalog | `.nav-rail` padding 44 0; `data-content-padding` / `data-content-pad-v` / `data-content-pad-h` on all rail heroes |
| Hosts | Desktop + Android live rails use ContentPadding top+bottom (static pair, in-flow, modal, narrow, hide, header) |
| Inventory | Rail notes ContentPadding 0/44/0/44 (WNRVerticalPadding = TopSpace) |
| Tests | Padding helpers; catalog attrs; desktop TopSpace alias |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture ContentPadding 0/44/0/44 against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer, ContentPadding 44 | OS `PopUp` unopened; predictive-back scale skipped |
| Narrow modal | Live 80↔220 + same modal container + ContentPadding 44 | Same OS `PopUp` gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape, ContentPadding 44 | Same OS `PopUp` gap |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface, ContentPadding 44 | Same OS `PopUp` gap |
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface, ContentPadding 44 | FAB slot stays +/← toggle (not Extended) |

## v45 leftovers

See `docs/qa/v44_leftovers.md`.
