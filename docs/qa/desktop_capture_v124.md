# Desktop capture v124

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v124 after v123 TimePicker PeriodToggle a11y (`69dfea1`).  
**Official:** TimeSelector uses `TimePickerHourSelection` “Select hour” and `TimePickerMinuteSelection` “Select minutes”. TimeInput fields use `TimeInputHourTextField` “for hour” and `TimeInputMinuteTextField` “for minutes”.

v124 started from current `main` and `docs/qa/v123_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePicker hour/minute a11y**. Do not reopen PeriodToggle a11y, YearPicker a11y, MonthsNavigation a11y, ScrollDisplayModeToggle SwipeVertical, or DisplayModeToggle strings. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `HOUR_MINUTE_A11Y` / `HOUR_SELECTION` / `MINUTE_SELECTION` / `INPUT_HOUR_FIELD` / `INPUT_MINUTE_FIELD` / `ScrollKind::selection_label` / `ScrollKind::input_field_label` |
| Catalog | `data-hour-minute-a11y` + title / aria-label on TimeSelector hour/minute and TimeInput fields |
| Hosts | Desktop + Android hover tooltip on live TimeSelector + TimeInput fields |
| Inventory | Time picker notes TimeSelector / TimeInput field a11y |
| Tests | Official strings + catalog `data-hour-minute-a11y` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture hour/minute a11y against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + MonthsNavigation a11y + YearPicker a11y | Recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + PeriodSelector Outline + DisplayModeToggle a11y + PeriodToggle a11y + **hour/minute a11y** | Clock-number suffixes still leftover; recapture stills when screenshot works |

## v125 leftovers

See `docs/qa/v124_leftovers.md`.
