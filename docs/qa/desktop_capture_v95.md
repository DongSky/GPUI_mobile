# Desktop capture v95

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v95 after v94 Date Input InputTextFieldPadding (`8d7d228`).  
**Official:** Compose `DatePickerModeTogglePadding` = PaddingValues(end 12, bottom 12).

v95 started from current `main` (v94) and `docs/qa/v94_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePickerModeTogglePadding**. Do not reopen Date Input InputTextFieldPadding, DateRangePickerHeadline end-only / start-only / empty, DatePickerTitlePadding / DatePickerHeadlinePadding, DateRangePickerTitle Picker empty, DatePickerHeadline Picker/Input empty, SelectableDates, DateInputValidator errors, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped. DateRangePickerTitlePadding start 64 stays skipped without official range-header chrome.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `TOGGLE_PAD_END_DP` / `TOGGLE_PAD_BOTTOM_DP` / `TOGGLE_PADDINGS`; `toggle_padding_css` |
| Catalog | `.cal [data-date-toggle-pad]` uses 0/12/12/0; `data-date-toggle-pad` on mode toggles |
| Hosts | Desktop + Android date-picker mode toggles use official end/bottom pads |
| Inventory | Date picker notes DatePickerModeTogglePadding |
| Tests | Token end 12 / bottom 12 + catalog CSS / `data-date-toggle-pad` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DatePickerModeTogglePadding against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official mode-toggle padding** + Input field padding + end-only / start-only / empty headlines + title/headline padding + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v96 leftovers

See `docs/qa/v95_leftovers.md`.
