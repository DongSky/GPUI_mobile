# Desktop capture v48

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v48 commit after `a6a17b9` (v47).  
**Official:** [m3.material.io lists](https://m3.material.io/components/lists/specs) Expressive swipe-to-reveal (`SwipeToDismissBox` / MDC `SwipeableListItem`) with LazyColumn-style inertial fling.

v48 started from v47 leftovers (`docs/qa/v47_leftovers.md` as written on that landing). Rail FAB family is shipped; highest-impact **implementable** gap vs current Expressive was leftover **#8** list swipe-as-LazyColumn fling. Hosts and the HTML catalog still snapped on pointer-up / wheel instead of integrating `v e^{-kt}` to Closed / Open / `STATE_SWIPE_PRIMARY_ACTION` with 16dp overshoot and a growing reveal rail. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `list.rs` | `SwipePhase` Closed/Dragging/Settling/Open/Primary; `impulse` + `step_live` LazyColumn fling (`FLING_DECAY` 2); 16dp overshoot; primary 360 / threshold 180; growing reveal rails |
| Catalog | `data-swipe-fling`; rAF integrator + wheel; growing Archive/Delete rails; hero stays Open Archive at 80 |
| Hosts | Desktop + Android wheel `apply_wheel` + vsync `tick_list_swipe`; rail width follows offset past 80 |
| Inventory | List notes LazyColumn fling + growing reveal; motion notes swipe fling clock |
| Tests | Overshoot / primary / fling settle; catalog fling attrs; desktop Open phase |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture list swipe LazyColumn fling against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface, ContentPadding 44, Extended FAB Create 56↔188 | OS `PopUp` unopened (n/a for standard) |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface, ContentPadding 44 | Same OS `PopUp` gap |
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer, ContentPadding 44, header-less Extended FAB Create 56↔188 | OS `PopUp` unopened; predictive-back scale skipped |
| Narrow modal | Live 80↔220 + same modal container + ContentPadding 44 + Extended FAB Create (12dp collapsed inset) | Same OS `PopUp` + predictive-back gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape, ContentPadding 44, always-extended Create ~188 | OS `PopUp` unopened; predictive-back scale skipped |
| List swipe | Archive/Delete 80dp rails, Team sync notes Open, LazyColumn fling to Closed/Open/primary, 16dp overshoot, growing reveal | Nested Android `LazyColumn` + OS overscroll glow not in GPUI; predictive-back scale skipped |

## v49 leftovers

See `docs/qa/v48_leftovers.md`.
