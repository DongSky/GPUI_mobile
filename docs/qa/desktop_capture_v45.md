# Desktop capture v45

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v45 commit after `2bce8e9` (v44).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `ExtendedFloatingActionButton(expanded = railExpanded)`.

v45 started from v44 leftovers (`docs/qa/v44_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the in-flow standard WideNavigationRail FAB slot: hosts and the HTML catalog still used a +/← expand toggle instead of Compose `ExtendedFloatingActionButton` (Create, Regular 56 ↔ Extended ~188). Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `WIDE_DEMO_HAS_EXTENDED_FAB`; in-flow standard rail documents Create 56↔188 |
| Catalog | `data-inflow-fab-extend` on `wide-rail-inflow`; `applyRailFabMorph` width/margin/label fade; JS skips +/← overwrite on extend slots |
| Hosts | Desktop + Android live in-flow rail (Extended FAB Create, Inbox body shifts, no scrim) |
| Inventory | Rail notes in-flow Extended FAB Create (56↔188); modal/narrow keep FAB toggle |
| Tests | In-flow extend attr; `WIDE_DEMO_HAS_EXTENDED_FAB`; desktop Create 56↔188 |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture in-flow Extended FAB Create against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface, ContentPadding 44, Extended FAB Create 56↔188 | OS `PopUp` unopened (n/a for standard) |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface, ContentPadding 44 | Same OS `PopUp` gap |
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer, ContentPadding 44 | FAB slot stays +/← toggle (not Extended); OS `PopUp` unopened; predictive-back scale skipped |
| Narrow modal | Live 80↔220 + same modal container + ContentPadding 44 | Same +/← FAB + OS `PopUp` gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape, ContentPadding 44 | Same OS `PopUp` gap |

## v46 leftovers

See `docs/qa/v45_leftovers.md`.
