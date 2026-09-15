# Desktop capture v50

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v50 commit after `324227b` (v49).  
**Official:** [m3.material.io time pickers](https://m3.material.io/components/time-pickers/specs) Expressive `TimeScroll` + `ScrollField` (I/O 2026 tactile input).

v50 started from v49 leftovers (`docs/qa/v49_leftovers.md` as written on that landing). Text-field rounded + tonal is shipped; leftover #8 is only blocked items. Highest-impact **implementable** gap vs current Expressive was Compose `TimeScroll` (two wrapping `ScrollField`s, `ScrollFieldDefaults.ScrollFieldHeight` 200 / 3-item, `TimePickerDefaults.vibrantColors()` primaryContainer). Hosts and the HTML catalog still used the baseline polar dial as the only hero. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable). Contained search (recommended over divided) stays a leftover — skip search transform this pass.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TimeScroll` / `ScrollField` wrap + `v e^{-kt}` fling + snap; `resolve_scroll()` vibrantColors; demo style Scroll |
| Catalog | TimeScroll hero (`data-time-scroll`, hour/minute fields); rAF + wheel/drag; dial remains under `h3` |
| Hosts | Desktop + Android TimeScroll heroes; vsync `tick_time_scroll`; wheel + click-to-snap |
| Inventory | Time picker notes TimeScroll + vibrant + fling; motion notes ScrollField clock |
| Tests | Scroll tokens / wrap / fling settle; catalog attrs; desktop vibrant container |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive TimeScroll against `docs/catalog/material-catalog-light.html` when screenshot works.

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
| Time picker | TimeScroll two ScrollFields 200dp / wrap / vibrant primaryContainer; dial still available | TimeInput text entry + ScrollDisplayModeToggle not painted; contained search still divided |

## v51 leftovers

See `docs/qa/v50_leftovers.md`.
