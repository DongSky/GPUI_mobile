# Desktop capture v55

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v55 commit after `a1ecb57` (v54 search suggestion groups).  
**Official:** [m3.material.io time pickers](https://m3.material.io/components/time-pickers/specs) Compose `ClockFace` 24-hour dial (`OuterCircleToSizeRatio` 101/256, `InnerCircleToSizeRatio` 69/256).

v55 started from v54 leftovers (`docs/qa/v54_leftovers.md` as written on that landing). Search suggestion groups, compact→docked search, and 24-hour TimeInput ship; v54 capture still listed **dial stays 12-hour only**. Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): Compose 24-hour `ClockFace` (outer 00–11 / inner 12–23, selector shortens to the inner ring, no AM/PM, time selector 114dp). Live IME, caret bounds, and search `cx.transform` stay blocked. `gpui::LineCap` still not re-exported (hosts keep lyon `StrokeCap::Round`). Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable). v52–v54 search/TimeInput work was not reopened.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `DialRing` + Compose 101/69 radii; `hour_cells` 00–11 outer / 12–23 inner; selector hand uses ring radius; time selector 114dp (24h) / 96dp |
| Catalog | 24h hour-face hero (`data-dial="hour"`, `data-ring`, `data-format`); format toggle remaps dial; CSS hides period + 114dp fields |
| Hosts | Desktop + Android paint `hour_cells(format)`; hide AM/PM; inner-ring hand; header `00`–`23` |
| Inventory | Time picker notes 24h ClockFace dual ring 101/69 |
| Tests | Inner/outer tokens, 18 inner / 00 outer, catalog `data-ring`, DEMO_DIAL hour |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` after catalog regen. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture 24-hour dual-ring dial against `docs/catalog/material-catalog-light.html` when screenshot works.

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
| Time picker | TimeScroll + TimeInput 96×72 + ScrollDisplayModeToggle; 24-hour is24Hour 00–23 no period; ClockFace outer 00–11 / inner 12–23 (101/69) | Horizontal `TimePickerLayoutType` unpainted |
| Search | Compact fullscreen 0/0 below 600dp; medium docked Corner 28 + 24→12; Recent / Suggestions 8dp groups; divided activity still available | `cx.transform` search morph skipped |

## v56 leftovers

See `docs/qa/v55_leftovers.md`.
