# Desktop capture v91

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v91 after v90 DateRangePickerTitle Picker empty (`42c01a1`).  
**Official:** Compose `DateRangePickerHeadline` after the first tap uses `mtrl_picker_range_header_only_start_selected` (`Sep 15 – End date`).

v91 started from current `main` (v90; requested v86-from-v85 `62b0ab1` already landed) and `docs/qa/v90_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePickerHeadline start-only**. Do not reopen DateRangePickerTitle Picker empty, DatePickerHeadline Picker/Input empty, DateRangePickerHeadline empty, range Input SelectableDates not-allowed, calendar-grid weekend disable, DateInputValidator format/year-range/not-allowed/order, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DateRangeSelection::start_only()`, `RANGE_START_ONLY`; headline `Sep 15 – End date` |
| Catalog | `datepicker-range-start-only` sibling: Start `09/15/2026`, End `MM/DD/YYYY` |
| Hosts | Desktop + Android start-only Input cards (`date-range-input-start-only`) |
| Inventory | Date picker notes DateRangePickerHeadline start-only |
| Tests | Catalog `data-range-start-only` / `Sep 15 – End date`; `header_range_selection(start_only())` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DateRangePickerHeadline start-only against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **range Input start-only headline** + Picker/Input/range empty headlines + DateRangePickerTitle empty + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v92 leftovers

See `docs/qa/v91_leftovers.md`.
