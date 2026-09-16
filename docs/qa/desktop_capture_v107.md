# Desktop capture v107

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v107 after v106 TimeInput SupportLabelTop (`4106433`).  
**Official:** Compose `TimePickerDialogDefaults.Title` selects `TimePickerDialogTitle` / `TimeInputDialogTitle` / `TimeScrollDialogTitle` (`Select Time` / `Enter Time` / `Select Time`) with `Modifier.padding(bottom = 20.dp)` and `labelMedium`.

v107 started from current `main` and `docs/qa/v106_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePickerDialogDefaults.Title**. Do not reopen TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `DIALOG_TITLE` / `TITLE_PAD_BOTTOM_DP` 20 / `INPUT_TITLE` / `SCROLL_TITLE` / `title_for` / `title_pad_bottom_css` + appearance `labelMedium` |
| Catalog | Live mode title (`data-time-dialog-title`, Input `Enter time`) + 20dp head pad + labelMedium; dial Picker title 20dp |
| Hosts | Desktop + Android expressive heroes use `title_for(mode)` + 20dp bottom; dial title 20dp |
| Inventory | Time picker notes TimePickerDialogDefaults.Title |
| Tests | title_for Scroll/Input/Picker + 20dp + catalog `data-time-dialog-title` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimePickerDialogDefaults.Title against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + YearPicker trailing HorizontalDivider + DateEntryContainer header divider + DatePickerDialog 360×568 + HeaderContainerHeight 120 / range 68 + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + SupportLabelTop + **Title Picker/Scroll Select time, Input Enter time, 20dp + labelMedium** + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal | Recapture stills when screenshot works |

## v108 leftovers

See `docs/qa/v107_leftovers.md`.
