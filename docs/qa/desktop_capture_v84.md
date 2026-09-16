# Desktop capture v84

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v84 after origin v83 DatePicker Input errors (`5171398`).  
**Official:** Compose `SelectableDates` / `m3c_date_input_invalid_not_allowed` — `Date not allowed: %1$s` after format and year-range.

v84 started from origin v83 leftovers (`docs/qa/v83_leftovers.md`). A parallel agent already landed single-date Input format/year-range; this pass closes the named SelectableDates delta. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Do not reopen DatePicker Input format/year-range, DateRangePicker year-range/format/order, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `SELECTABLE_DATES`; weekend `is_selectable_date`; `DateInputError::NotAllowed`; validator after year-range |
| Catalog | `datepicker-input-errors` third card: `09/12/2026` + `Date not allowed: Sat, Sep 12` |
| Hosts | Desktop + Android Input error cards add the not-allowed sample |
| Inventory | Date picker notes SelectableDates weekends |
| Tests | Weekend vs weekday; catalog not-allowed copy + attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DatePicker Input not-allowed against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **Input format / year-range / not-allowed** | Recapture stills; calendar-grid weekend disable is a follow-up |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v85 leftovers

See `docs/qa/v84_leftovers.md`.
