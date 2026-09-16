# Desktop capture v90

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v90 after v89 DatePickerHeadline Picker empty (`d54df87`).  
**Official:** Compose `DateRangePickerTitle` is `Select dates` when Picker has no selection; headline stays `Start date – End date`.

v90 started from v89 leftovers (`docs/qa/v89_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePicker Picker empty**. Do not reopen DatePickerHeadline Picker/Input empty, DateRangePickerHeadline Input empty, range/single-date SelectableDates not-allowed, weekend grid disable, DateInputValidator year-range/format/order, docked trailing DateRange, VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors/Confirm/Cancel/showModeToggle/YearPicker/month nav/live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_PICKER_TITLE` / `RANGE_PICKER_EMPTY` (`Select dates`) |
| Catalog | `datepicker-range-picker-empty` sibling: unselected VerticalMonthsList |
| Hosts | Desktop + Android empty range Picker cards |
| Inventory | Date picker notes DateRangePickerTitle Picker empty |
| Tests | Catalog `data-range-picker-empty`; empty range grid has no Selected/InRange |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range Picker empty against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + empty headlines + **range Picker empty Select dates** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v91 leftovers

See `docs/qa/v90_leftovers.md`.
