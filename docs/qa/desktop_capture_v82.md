# Desktop capture v82

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v82 after v81 DateRangePicker input errors (`d5b60ee`).  
**Official:** Compose `DateInputValidator` — `m3c_date_input_invalid_year_range` / `Date out of expected year range 1900 - 2100`.

v82 started from v81 leftovers (`docs/qa/v81_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateInputValidator year-range error**. Do not reopen DateRangePicker format/order errors, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `INPUT_ERROR_YEAR_RANGE` / `INPUT_ERROR_YEAR_SAMPLE`; `DateInputError::YearRange`; `range_input_error` pattern → year range → order |
| Catalog | `datepicker-range-input-errors` third card: year `09/15/1890` + `data-range-error="year"` |
| Hosts | Desktop + Android range Input supporting slot already uses `range_input_error().label()` |
| Inventory | Date picker notes year-range validator copy |
| Tests | Year-range below/above YearRange; catalog year-range copy + attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range input year-range errors against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **range Input format / year-range / order** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v83 leftovers

See `docs/qa/v82_leftovers.md`.
