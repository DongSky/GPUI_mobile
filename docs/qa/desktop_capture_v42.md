# Desktop capture v42

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v42 commit after `daba030` (v41).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `WideNavigationRail` header FAB / Extended FAB.

v42 started from v41 leftovers (`docs/qa/v41_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the missing official “menu and FAB” header: Compose `ExtendedFloatingActionButton(expanded = railExpanded)` (Create, Regular 56 ↔ Extended ~188). Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `RailFabMorph`; Regular 56 → Extended (rail − 32) with spatial-fast; Create label; header Menu + FAB tokens |
| Catalog | `data-hero="wide-rail-header"` Menu + Extended FAB; `applyRailFabMorph` width/margin/label fade |
| Hosts | Desktop + Android live header rail (Menu toggle, Extended FAB Create, hover tooltip, Bottom dests, Inbox body) |
| Inventory | Rail notes header Menu/MenuOpen + Extended FAB Create + tooltip Above + Arrangement.Bottom |
| Tests | FAB morph helpers; catalog extend attrs; desktop Create 56↔188 |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture header Menu + Extended FAB Create against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim | OS `PopUp` unopened |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, 32% scrim | Same OS `PopUp` gap |
| Narrow modal | Live 80↔220 + 32% scrim | Same OS `PopUp` gap |
| Modal rail | 96↔220 + 32% scrim | Same OS `PopUp` gap |
| In-flow wide | 96↔220, no scrim, Inbox body | FAB slot stays +/← toggle (not Extended) |

## v43 leftovers

See `docs/qa/v42_leftovers.md`.
