# Desktop capture v69

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v69 after v68 modal date range input (`2701d70`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/guidelines) — Date range picker: tap a start date, then an end date. Compose `DateRangePicker` selection (start, then end ≥ start; a further tap restarts).

v69 started from v68 leftovers (`docs/qa/v68_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **live range start→end selection** on the existing range hero. Do not reopen modal date range input, YearPicker, live Picker↔Input, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DateRangeSelection`; `apply_range_tap`; `header_range_selection`; `month_grid_range_selection` |
| Catalog | `data-date-range-live` on `datepicker-range`; tap start→end; headline + InRange re-paint |
| Hosts | Desktop + Android range hero days apply Compose tap rules |
| Inventory | Date picker notes live start→end taps |
| Tests | Tap restart / start-only headline / empty grid |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture live range selection against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + **live range start→end** + docked; modal input; live showModeToggle; YearPicker; range input | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v70 leftovers

See `docs/qa/v69_leftovers.md`.
