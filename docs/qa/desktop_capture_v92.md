# Desktop capture v92

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v92 after v91 DateRangePickerHeadline start-only (`2c01d91`).  
**Official:** Compose `DatePickerTitlePadding` = PaddingValues(start 24, end 12, top 16); `DatePickerHeadlinePadding` = PaddingValues(start 24, end 12, bottom 12).

v92 started from current `main` (v91) and `docs/qa/v91_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePickerTitlePadding / DatePickerHeadlinePadding**. Do not reopen DateRangePickerHeadline start-only, DateRangePickerTitle Picker empty, DatePickerHeadline Picker/Input empty, DateRangePickerHeadline empty, range Input SelectableDates not-allowed, calendar-grid weekend disable, DateInputValidator format/year-range/not-allowed/order, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `TITLE_PAD_*` / `HEADLINE_PAD_*` / `HEADER_PADDINGS`; `title_padding_css` / `headline_padding_css` |
| Catalog | `.cal .head` uses official title/headline pads (`data-date-title-pad` / `data-date-headline-pad` / `data-date-header-paddings`) |
| Hosts | Desktop + Android date-picker headers apply Compose title/headline paddings |
| Inventory | Date picker notes DatePickerTitlePadding / DatePickerHeadlinePadding |
| Tests | Token 24/12/16/12 + catalog CSS / `data-date-title-pad` / `data-date-headline-pad` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DatePickerTitlePadding / DatePickerHeadlinePadding against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official title/headline padding** + range Input start-only headline + Picker/Input/range empty headlines + DateRangePickerTitle empty + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v93 leftovers

See `docs/qa/v92_leftovers.md`.
