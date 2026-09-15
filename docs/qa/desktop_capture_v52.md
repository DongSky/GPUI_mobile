# Desktop capture v52

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v52 commit after `4c31f42` (v51).  
**Official:** [m3.material.io time pickers](https://m3.material.io/components/time-pickers/specs) `TimePickerState.is24Hour` (00–23, no AM/PM).

v52 started from v51 leftovers (`docs/qa/v51_leftovers.md` as written on that landing). TimeInput + ScrollDisplayModeToggle and contained search are shipped; leftover #8 named 24-hour TimeInput and contained compact→docked as the next Visual QA targets. Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): Compose `is24Hour` TimeInput (00–23 fields, hidden AM/PM) with a 12/24 toggle and a matching TimeScroll hour wheel. Contained compact→docked breakpoint stays for later. Live IME, caret bounds, and search `cx.transform` stay blocked. `gpui::LineCap` still not re-exported (hosts keep lyon `StrokeCap::Round`). Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TimeFormat` Hour12/Hour24 (`is24Hour`); 00–23 hour wheel + TimeInput; hide AM/PM; `apply_format_toggle`; demo hero 18:30 Input |
| Catalog | `data-time-format="24"` + format toggle; CSS hides `.period`; JS hour mapping uses `data-count` 24; TimeInput accepts 00–23 |
| Hosts | Desktop + Android 12/24 toggle; period column hidden in 24h; dial stays 1–12 via `dial_clock_hour` / `hour_from_dial` |
| Inventory | Time picker notes 24-hour `is24Hour` 00–23, hidden AM/PM |
| Tests | 24h tokens, wrap 23→0 / 0→23, format toggle 6 PM → 18, catalog `data-time-format="24"` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture 24-hour TimeInput against `docs/catalog/material-catalog-light.html` when screenshot works.

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
| Time picker | TimeScroll two ScrollFields 200dp / wrap / vibrant primaryContainer; TimeInput 96×72 + ScrollDisplayModeToggle; 24-hour 00–23, no AM/PM | — |
| Search | Contained persistent filled Corner 28, 24→12dp, no divider; divided activity still available | `cx.transform` search morph skipped; compact full-screen vs medium docked breakpoint not live |

## v53 leftovers

See `docs/qa/v52_leftovers.md`.
