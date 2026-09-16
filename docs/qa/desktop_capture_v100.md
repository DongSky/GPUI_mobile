# Desktop capture v100

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v100 after v99 MonthsNavigation MonthYearHeight (`1e47f4a`).  
**Official:** Compose `VerticalTimePicker` inserts `ClockDisplayBottomMargin` 36 between ClockDisplay and ClockFace, then `ClockFaceBottomMargin` 24 below the dial. Horizontal still uses the shipped 24dp selector↔ClockFace gap (`HORIZONTAL_GAP_DP`), not the 36dp width spacer.

v100 started from current `main` (v99) and `docs/qa/v99_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePicker ClockFace margins**. Do not reopen MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, DateRangePickerTitlePadding / DateRangePickerHeadlinePadding, DatePickerModeTogglePadding, Date Input `InputTextFieldPadding`, end-only / start-only / empty headlines, DatePickerTitlePadding / DatePickerHeadlinePadding, SelectableDates, DateInputValidator errors, docked / range-hero / modal chrome, TimePickerLayoutType Horizontal 24dp gap. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `CLOCK_DISPLAY_BOTTOM_MARGIN_DP` 36 / `CLOCK_FACE_BOTTOM_MARGIN_DP` 24 / `CLOCK_FACE_MARGINS` |
| Catalog | Vertical dial: clock `margin-top: 36` / `margin-bottom: 24` (`data-clock-face-margins`); horizontal 24 gap unchanged |
| Hosts | Android vertical dial: 36 above / 24 below ClockFace |
| Inventory | Time picker notes ClockDisplayBottomMargin / ClockFaceBottomMargin |
| Tests | 36 / 24 tokens + catalog `data-clock-face-margins` / CSS margins |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimePicker ClockFace margins against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal + **official vertical ClockFace 36/24 margins** | Recapture stills when screenshot works |

## v101 leftovers

See `docs/qa/v100_leftovers.md`.
