# Desktop capture v93

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v93 after v92 DatePickerTitlePadding / DatePickerHeadlinePadding (`a5ef179`).  
**Official:** Compose `DateRangePickerHeadline` when only end is set uses `mtrl_picker_range_header_only_end_selected` (`Start date – Sep 21`).

v93 started from current `main` (v92) and `docs/qa/v92_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePickerHeadline end-only**. Do not reopen DatePickerTitlePadding / DatePickerHeadlinePadding, DateRangePickerHeadline start-only, DateRangePickerTitle Picker empty, DatePickerHeadline Picker/Input empty, DateRangePickerHeadline empty, range Input SelectableDates not-allowed, calendar-grid weekend disable, DateInputValidator format/year-range/not-allowed/order, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped. DateRangePickerTitlePadding start 64 stays skipped without official range-header chrome.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DateRangeSelection::end_only()`, `RANGE_END_ONLY` / `RANGE_END_ONLY_HEADLINE`; headline `Start date – Sep 21` |
| Catalog | `datepicker-range-end-only` sibling: Start `MM/DD/YYYY`, End `09/21/2026` |
| Hosts | Desktop + Android end-only Input cards (`date-range-input-end-only`) |
| Inventory | Date picker notes DateRangePickerHeadline end-only |
| Tests | Catalog `data-range-end-only` / `Start date – Sep 21`; `header_range_selection(end_only())` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DateRangePickerHeadline end-only against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **range Input end-only headline** + start-only + Picker/Input/range empty headlines + DateRangePickerTitle empty + title/headline padding + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v94 leftovers

See `docs/qa/v93_leftovers.md`.
