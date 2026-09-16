# Desktop capture v88

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v88 after v87 DatePickerHeadline Input empty (`7e91ed2`).  
**Official:** Compose `DateRangePickerHeadline` shows `Start date – End date` when both ends are unset.

v88 started from v87 leftovers (`docs/qa/v87_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePickerHeadline empty**. Do not reopen DatePickerHeadline Input empty, range/single-date SelectableDates not-allowed, weekend grid disable, DateInputValidator year-range/format/order, docked trailing DateRange, VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors/Confirm/Cancel/showModeToggle/YearPicker/month nav/live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_EMPTY` / `RANGE_EMPTY_HEADLINE`; `header_range_selection(empty())` already matched |
| Catalog | `datepicker-range-input-empty` sibling: `Start date – End date` + empty Start/End `MM/DD/YYYY` |
| Hosts | Desktop + Android empty range Input cards |
| Inventory | Date picker notes DateRangePickerHeadline empty |
| Tests | Catalog `data-range-input-empty`; empty selection headline |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range Input empty headline against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + Input empty + **range Input empty headline** | Recapture stills; Picker empty `Selected date` unused while demo has a date |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v89 leftovers

See `docs/qa/v88_leftovers.md`.
