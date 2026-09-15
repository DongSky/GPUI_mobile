# Desktop capture v57

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v57 commit after `15169bf` (v56 Results status).  
**Official:** [m3.material.io search guidelines](https://m3.material.io/components/search/guidelines) / [specs](https://m3.material.io/components/search/specs) — “Docked opens a list below the search bar, with a scrim covering main content.” Docked container height min 240dp, max ⅔ of screen. Scrims use the scrim role at 32%.

v57 started from v56 leftovers (`docs/qa/v56_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **docked search 32% scrim + min 240 / max ⅔**, with the results list scrolling beneath the pinned 56dp bar. Do not reopen Results/Quick results, ClockFace, groups, compact→docked corners/margins, TimeInput. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `DOCKED_MIN_H_DP` 240, max ⅔ `DEMO_SCREEN_H_DP`, 360–720 width; `docked_scrim` 32%; `uses_docked_scrim` / `dismiss_on_scrim`; docked height clamp |
| Catalog | Medium docked wrapped in `data-search-docked-stage` + 32% scrim over placeholder content; list `overflow-y` under the bar; scrim click collapses |
| Hosts | Desktop (medium docked) paints scrim + max height; compact Android stays full-screen (no scrim) |
| Inventory | Search notes 240 / ⅔ / 32% scrim |
| Tests | Docked clamp, scrim tokens, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture docked search + scrim against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen < 600dp; medium docked Corner 28 + 24→12; 32% scrim; min 240 / max ⅔; Quick results / Results | `cx.transform` search morph skipped; segmented result rows still open |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings | Horizontal TimePickerLayoutType unpainted |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v58 leftovers

See `docs/qa/v57_leftovers.md`.
