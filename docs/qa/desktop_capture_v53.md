# Desktop capture v53

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v53 commit after `3dab7af` (v52).  
**Official:** [m3.material.io search](https://m3.material.io/components/search/specs) compact→docked breakpoint (`ExpandedFullScreenSearchBar` / `ExpandedDockedSearchBar`).

v53 started from v52 leftovers (`docs/qa/v52_leftovers.md` as written on that landing). 24-hour TimeInput ships; leftover #8 named **contained compact→docked** (full-screen default below 600dp, docked at medium) as the next Visual QA target. Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): Compose window-width search. Live IME, caret bounds, and search `cx.transform` stay blocked. `gpui::LineCap` still not re-exported (hosts keep lyon `StrokeCap::Round`). Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable). 24-hour TimeInput was not reopened.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `WindowWidthClass` Compact `<600` / Medium `≥600`; compact fullscreen 0/0; medium docked Corner 28 + 24→12 |
| Catalog | Compact contained hero + medium docked sibling (`data-width-class`, `data-search-expanded`) |
| Hosts | Android compact search; desktop medium docked |
| Inventory | Search notes compact/600dp/fullscreen |
| Tests | Compact frame tokens; catalog attrs; desktop fullscreen corner 0 |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` after catalog regen. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture compact/docked search against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface, ContentPadding 44, Extended FAB Create 56↔188 | OS `PopUp` unopened (n/a for standard) |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface, ContentPadding 44 | Same OS `PopUp` gap |
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer, ContentPadding 44, header-less Extended FAB Create 56↔188 | OS `PopUp` unopened; predictive-back scale skipped |
| Narrow modal | Live 80↔220 + same modal container + ContentPadding 44 + Extended FAB Create (12dp collapsed inset) | Same OS `PopUp` + predictive-back gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape, ContentPadding 44, always-extended Create ~188 | OS `PopUp` unopened; predictive-back scale skipped |
| List swipe | Archive/Delete 80dp rails, Team sync notes Open, LazyColumn fling to Closed/Open/primary, 16dp overshoot, growing reveal | Nested Android `LazyColumn` + OS overscroll glow not in GPUI; predictive-back scale skipped |
| Text field | Expressive rounded 12 + tonal filled SurfaceContainer / outlined OnPrimary, Inside label | Live JVM IME / GPUI caret bounds still blocked |
| Time picker | TimeScroll + TimeInput 96×72 + ScrollDisplayModeToggle; 24-hour is24Hour 00–23 no period | Dial stays 12-hour only |
| Search | Compact fullscreen 0/0 below 600dp; medium docked Corner 28 + 24→12; divided activity still available | `cx.transform` search morph skipped |

## v54 leftovers

See `docs/qa/v53_leftovers.md`.
