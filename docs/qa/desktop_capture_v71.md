# Desktop capture v71

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v71 after v70 DateRangePicker month nav (`ac54d7f`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — DateRangePicker month ▾ opens YearPicker (3×72×36, YearRange 1900–2100).

v71 started from v70 leftovers (`docs/qa/v70_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **YearPicker on the existing range hero**. Do not reopen range-hero month nav, live range start→end, range input, YearPicker sibling, live Picker↔Input on the single-date modal, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_YEAR_PANE`; `apply_range_year` keeps month, clamps YearRange |
| Catalog | `data-range-year-toggle` / `data-range-years`; month ▾ swaps pane + re-paints years |
| Hosts | Desktop + Android `range_pane`; hide prev/next in year pane |
| Inventory | Date picker notes range-hero YearPicker |
| Tests | `apply_range_year` 2026→2027 / clamp 1890→1900; catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range-hero YearPicker against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + live range start→end + month nav + **range YearPicker** + docked; modal input; live showModeToggle; YearPicker sibling; range input | Range-hero showModeToggle still missing |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v72 leftovers

See `docs/qa/v71_leftovers.md`.
