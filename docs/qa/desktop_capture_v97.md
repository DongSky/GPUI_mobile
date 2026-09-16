# Desktop capture v97

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v97 after v96 DateRangePickerTitlePadding / HeadlinePadding (`6ede6a4`).  
**Official:** Compose `DatePickerDialog` `DialogButtonsPadding` = PaddingValues(bottom 8, end 6), `DialogButtonsMainAxisSpacing` 8, `DialogButtonsCrossAxisSpacing` 12.

v97 started from current `main` (v96) and `docs/qa/v96_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePickerDialog button padding**. Do not reopen DateRangePickerTitlePadding / HeadlinePadding, DatePickerModeTogglePadding, Date Input InputTextFieldPadding, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Do not add Save / X range-header chrome. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DIALOG_BUTTONS_PAD_*` / `DIALOG_BUTTONS_MAIN_GAP_DP` / `DIALOG_BUTTONS_CROSS_GAP_DP` / `DIALOG_BUTTONS_PADDINGS`; `dialog_buttons_padding_css` |
| Catalog | `.cal [data-date-dialog-buttons]` uses 0/6/8/0 + 8dp gap |
| Hosts | Desktop + Android Cancel/OK rows use official end/bottom pads and 8dp gap |
| Inventory | Date picker notes DialogButtonsPadding |
| Tests | Token end 6 / bottom 8 + catalog CSS / `data-date-dialog-buttons` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DatePickerDialog DialogButtonsPadding against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official dialog button padding** + range title/headline start 64 + mode-toggle padding + Input field padding + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v98 leftovers

See `docs/qa/v97_leftovers.md`.
