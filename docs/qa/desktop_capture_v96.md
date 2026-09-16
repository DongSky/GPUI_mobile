# Desktop capture v96

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v96 after v95 DatePickerModeTogglePadding (`093d62a`).  
**Official:** Compose `DateRangePickerTitlePadding` = PaddingValues(start 64, end 12) and `DateRangePickerHeadlinePadding` = PaddingValues(start 64, end 12, bottom 12). Compose does not render Save / X header chrome; the 64 start is the range title/headline indent.

v96 started from current `main` (v95) and `docs/qa/v95_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePickerTitlePadding / DateRangePickerHeadlinePadding**. Do not reopen DatePickerModeTogglePadding, Date Input InputTextFieldPadding, DateRangePickerHeadline end-only / start-only / empty, DatePickerTitlePadding / DatePickerHeadlinePadding, DateRangePickerTitle Picker empty, DatePickerHeadline Picker/Input empty, SelectableDates, DateInputValidator errors, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped. Do not add Save / X range-header chrome.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_TITLE_PAD_*` / `RANGE_HEADLINE_PAD_*` / `RANGE_HEADER_PADDINGS`; `range_title_padding_css` / `range_headline_padding_css` |
| Catalog | range heroes use start 64; `data-date-range-title-pad` / `data-date-range-headline-pad` / `data-date-range-header-paddings` |
| Hosts | Desktop + Android range headers use official start 64 (single-date stays start 24) |
| Inventory | Date picker notes DateRangePickerTitlePadding / DateRangePickerHeadlinePadding |
| Tests | Token start 64 + catalog CSS / range pad attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DateRangePickerTitlePadding / DateRangePickerHeadlinePadding against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official range title/headline start 64** + mode-toggle padding + Input field padding + end-only / start-only / empty headlines + title/headline padding + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v97 leftovers

See `docs/qa/v96_leftovers.md`.
