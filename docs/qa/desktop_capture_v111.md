# Desktop capture v111

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v111 after origin v110 TimePicker ClockFaceSizeModifier (`7c8602e`).  
**Official:** `TimePickerDialogDefaults.vibrantContainerColor` is `surfaceContainer` and `vibrantShape` is `CornerExtraLarge`. Title sits `onSurface`; mode toggles use `onSurfaceVariant`. TimeScroll `vibrantColors()` still paint the fields.

v111 started from current `main` and `docs/qa/v110_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **VibrantTimePickerDialog chrome**. Do not reopen TimePicker ClockFaceSizeModifier, DatePickerDialog CrossAxisSpacing, TimePickerCustomLayout paddings / Cancel/OK, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `VIBRANT_DIALOG` / `VIBRANT_DIALOG_CORNER_DP` 28 / `vibrant_dialog_container` / `vibrant_dialog_on_container` / `vibrant_dialog_toggle` |
| Catalog | Expressive hero `data-time-vibrant-dialog` + surfaceContainer shell |
| Hosts | Desktop + Android TimeScroll/TimeInput dialogs use surfaceContainer + onSurface title |
| Inventory | Time picker notes vibrantContainerColor / surfaceContainer |
| Tests | token colors + catalog `data-time-vibrant-dialog` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture VibrantTimePickerDialog against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + dividers + 360×568 + HeaderContainerHeight + MonthsNavigation + range-header close + DialogButtonsPadding + CrossAxisSpacing 8 + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + SupportLabelTop + Title + CustomLayout + ClockFaceSizeModifier + **VibrantTimePickerDialog surfaceContainer** + Cancel/OK + 24h ClockFace | Recapture stills when screenshot works |

## v112 leftovers

See `docs/qa/v111_leftovers.md`.
