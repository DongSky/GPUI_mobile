# Desktop capture v51

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v51 commit after `1d1b938` (v50).  
**Official:** [m3.material.io search](https://m3.material.io/components/search/specs) contained (recommended) + [time pickers](https://m3.material.io/components/time-pickers/specs) `TimeInput` + `ScrollDisplayModeToggle`.

v51 started from v50 leftovers (`docs/qa/v50_leftovers.md` as written on that landing). TimeScroll is shipped; leftover #8 named contained search and TimeInput + ScrollDisplayModeToggle as the next Visual QA targets. Highest-impact **implementable** gaps vs current Expressive (no NDK/gpui bump): contained search (persistent filled container, Corner 28 stays, 24→12dp margin, no divider) and Compose `TimeInput` (96×72 extra-large fields, `vibrantColors`) + `ScrollDisplayModeToggle` (Scroll↔Input). Live IME, caret bounds, and search `cx.transform` stay blocked. `gpui::LineCap` still not re-exported (hosts keep lyon `StrokeCap::Round`). Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `SearchStyle::Contained` (recommended) vs Divided; `ContainedFrame` 24→12 margin, Corner 28 stays, no divider; `DEMO_STYLE` contained |
| `time_picker.rs` | `TimeInput` 96×72 extra-large + digit editor; `TimePickerDisplayMode` + `ScrollDisplayModeToggle` (⌨/◷); `apply_display_toggle` syncs Scroll↔Input |
| Catalog | Contained hero (`data-search-style="contained"`); TimeScroll + TimeInput under `data-time-display` + toggle; divided activity remains under `h3` |
| Hosts | Desktop + Android contained expand (height/margin/corner, persistent fill); TimeInput + toggle on the TimeScroll hero |
| Inventory | Search notes contained; time picker notes TimeInput + ScrollDisplayModeToggle |
| Tests | Contained / TimeInput tokens; catalog attrs; desktop 96×72 + toggle |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK after installing fontconfig/xkb/libstdc++ for the desktop link. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture contained search + TimeInput toggle against `docs/catalog/material-catalog-light.html` when screenshot works.

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
| Time picker | TimeScroll two ScrollFields 200dp / wrap / vibrant primaryContainer; TimeInput 96×72 + ScrollDisplayModeToggle | 24-hour TimeInput not painted |
| Search | Contained persistent filled Corner 28, 24→12dp, no divider; divided activity still available | `cx.transform` search morph skipped; compact full-screen vs medium docked breakpoint not live |

## v52 leftovers

See `docs/qa/v51_leftovers.md`.
