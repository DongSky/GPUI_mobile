# Desktop capture v98

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v98 after v97 DatePickerDialog DialogButtonsPadding (`6a84e6c`).  
**Official:** Compose `DateRangePickerTitlePadding` start 64 / `DateRangePickerHeadlinePadding` start 64 reserve the modal close/back slot (8 + 48 + 8). v96 applied the pads without chrome; v98 paints the leading close so the 64dp column is real header chrome.

v98 started from current `main` (v97) and `docs/qa/v97_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePicker range-header chrome**. Do not reopen DatePickerDialog DialogButtonsPadding, DateRangePickerTitlePadding / DateRangePickerHeadlinePadding, DatePickerModeTogglePadding, Date Input `InputTextFieldPadding`, end-only / start-only / empty headlines, DatePickerTitlePadding / DatePickerHeadlinePadding, SelectableDates, DateInputValidator errors, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_HEADER_CHROME` / `RANGE_HEADER_CLOSE` / `RANGE_HEADER_CLOSE_*`; `apply_range_header_close` |
| Catalog | Range headers paint leading ✕ over the official 64 start (`data-range-header-chrome` / `data-range-header-close`); live close dismisses like Cancel |
| Hosts | Desktop + Android range headers: 8+48+8 close slot inside the existing 64 start; live hero close dismisses like Cancel |
| Inventory | Date picker notes range-header chrome |
| Tests | 8+48+8 = 64 + catalog `data-range-header-close` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DateRangePicker range-header chrome against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official range-header close in the 64 start slot** + DialogButtonsPadding + range title/headline pads + ModeTogglePadding + Input field padding + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v99 leftovers

See `docs/qa/v98_leftovers.md`.
