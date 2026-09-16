# Desktop capture v94

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v94 after v93 DateRangePickerHeadline end-only (`756eeee`).  
**Official:** Compose `InputTextFieldPadding` = PaddingValues(start 24, end 24, top 10) plus `InputTextNonErroneousBottomPadding` 16.

v94 started from current `main` (v93) and `docs/qa/v93_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **Date Input field padding**. Do not reopen DateRangePickerHeadline end-only / start-only / empty, DatePickerTitlePadding / DatePickerHeadlinePadding, DateRangePickerTitle Picker empty, DatePickerHeadline Picker/Input empty, SelectableDates, DateInputValidator errors, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped. DateRangePickerTitlePadding start 64 stays skipped without official range-header chrome.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `INPUT_FIELD_PAD_*` / `INPUT_FIELD_PADDINGS`; `input_field_padding_css` |
| Catalog | `.cal .dp-input` / `.dp-range-input` use 10/24/16/24; `data-date-input-pad` |
| Hosts | Desktop + Android Input field chrome uses official pads |
| Inventory | Date picker notes InputTextFieldPadding |
| Tests | Token 24/24/10/16 + catalog CSS / `data-date-input-pad` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Date Input `InputTextFieldPadding` against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official Input field padding** + end-only / start-only / empty headlines + title/headline padding + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v95 leftovers

See `docs/qa/v94_leftovers.md`.
