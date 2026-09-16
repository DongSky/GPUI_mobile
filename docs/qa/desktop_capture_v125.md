# Desktop capture v125

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v125 after v124 TimePicker hour/minute a11y (`019176f`).  
**Official:** ClockFace numbers use `TimePickerHourSuffix` “%1$ o'clock”, `TimePickerMinuteSuffix` “%1$ minutes”, and `TimePicker24HourSuffix` “%1$ hours”.

v125 started from current `main` and `docs/qa/v124_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePicker ClockFace number a11y**. Do not reopen TimeSelector/TimeInput hour/minute a11y, PeriodToggle a11y, YearPicker a11y, MonthsNavigation a11y, ScrollDisplayModeToggle SwipeVertical, or DisplayModeToggle strings. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `CLOCK_NUMBER_A11Y` / `HOUR_SUFFIX` `o'clock` / `MINUTE_SUFFIX` `minutes` / `HOUR_24_SUFFIX` `hours` / `hour_suffix_label` / `minute_suffix_label` / `hour_24_suffix_label` / `clock_number_label` |
| Catalog | `data-clock-number-a11y` + title / aria-label on ClockFace hour/minute numbers |
| Hosts | Desktop + Android hover tooltip on live ClockFace numbers |
| Inventory | Time picker notes ClockFace number a11y |
| Tests | Official strings + catalog `data-clock-number-a11y` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture ClockFace number a11y against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + MonthsNavigation a11y + YearPicker a11y | Day-cell a11y still leftover; recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + PeriodSelector Outline + DisplayModeToggle a11y + PeriodToggle a11y + hour/minute a11y + **ClockFace number a11y** | Recapture stills when screenshot works |

## v126 leftovers

See `docs/qa/v125_leftovers.md`.
