# Desktop capture v89

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v89 after v88 DateRangePickerHeadline empty (`3bb8f14`).  
**Official:** Compose `DatePickerHeadline` shows `Selected date` (`m3c_date_picker_headline`) when Picker has no selection.

v89 started from v88 leftovers (`docs/qa/v88_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePickerHeadline Picker empty**. Do not reopen DateRangePickerHeadline empty, DatePickerHeadline Input empty, range/single-date SelectableDates not-allowed, weekend grid disable, DateInputValidator year-range/format/order, docked trailing DateRange, VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors/Confirm/Cancel/showModeToggle/YearPicker/month nav/live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `PICKER_EMPTY`; `month_grid_unselected` (today + SelectableDates, no Selected) |
| Catalog | `datepicker-picker-empty` sibling: `Selected date` + calendar with no selected day |
| Hosts | Desktop + Android empty Picker cards |
| Inventory | Date picker notes DatePickerHeadline Picker empty |
| Tests | Catalog `data-picker-empty`; unselected grid has Today 11, InMonth 15, no Selected |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Picker empty headline against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + Input/range empty + **Picker empty Selected date** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v90 leftovers

See `docs/qa/v89_leftovers.md`.
