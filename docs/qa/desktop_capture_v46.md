# Desktop capture v46

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v46 commit after `45c5d60` (v45).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `ExtendedFloatingActionButton(expanded = railExpanded)` on header-less “three items with a FAB”.

v46 started from v45 leftovers (`docs/qa/v45_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the header-less modal / narrow WideNavigationRail FAB slot: hosts and the HTML catalog still used a +/← expand toggle instead of Compose `ExtendedFloatingActionButton` (Create, Regular 56 ↔ Extended ~188; narrow collapsed inset 12). Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `MODAL_DEMO_HAS_EXTENDED_FAB` + `NARROW_DEMO_HAS_EXTENDED_FAB`; header-less modal/narrow document Create 56↔188 |
| Catalog | `data-modal-fab-extend` / `data-narrow-fab-extend`; stretch modal/narrow; narrow collapsed FAB inset 12; `applyRailFabMorph` already width/margin/label fade |
| Hosts | Desktop + Android live modal + narrow rails (Extended FAB Create; click still expands/collapses) |
| Inventory | Rail notes header-less modal/narrow Extended FAB Create (56↔188); hideOnCollapse keeps FAB toggle |
| Tests | Modal/narrow extend attrs; demo flags; wide 20 / narrow 12 collapsed insets; desktop Create 56↔188 |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture header-less modal/narrow Extended FAB Create against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface, ContentPadding 44, Extended FAB Create 56↔188 | OS `PopUp` unopened (n/a for standard) |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface, ContentPadding 44 | Same OS `PopUp` gap |
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer, ContentPadding 44, header-less Extended FAB Create 56↔188 | OS `PopUp` unopened; predictive-back scale skipped |
| Narrow modal | Live 80↔220 + same modal container + ContentPadding 44 + Extended FAB Create (12dp collapsed inset) | Same OS `PopUp` + predictive-back gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape, ContentPadding 44 | FAB slot stays +/← toggle (not Extended); OS `PopUp` unopened |

## v47 leftovers

See `docs/qa/v46_leftovers.md`.
