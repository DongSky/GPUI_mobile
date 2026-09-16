# Desktop capture v83

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v83 after v82 DateInputValidator year-range (`a2f13bc`).  
**Official:** Compose `DateInputValidator` on single-date Input — `Date format not recognized` / `Date out of expected year range 1900 - 2100`.

v83 started from v82 leftovers (`docs/qa/v82_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **single-date modal DateInputValidator supporting-text** (format / year-range). Do not reopen range year-range, DateRangePicker format/order, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DATE_INPUT_ERRORS`; `date_input_error` (pattern → YearRange) |
| Catalog | `datepicker-input-errors` hero: format `13/40/2026` + year `09/15/1890` |
| Hosts | Desktop + Android single-date Input error cards (format + year-range) |
| Inventory | Date picker notes single-date DateInputValidator errors |
| Tests | Format / year-range / empty cases; catalog error copy + attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture modal input errors against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + range Input errors + **modal Input errors** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v84 leftovers

See `docs/qa/v83_leftovers.md`.
