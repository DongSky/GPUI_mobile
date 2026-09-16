# Desktop capture v109

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v109 after v108 TimePickerCustomLayout (`bff6fbc`).  
**Official:** androidx-main `DatePickerDialog.android.kt` `DialogButtonsCrossAxisSpacing` is now 8dp (was 12). `AlertDialogFlowRow` uses that as wrap row-gap.

v109 started from current `main` and `docs/qa/v108_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePickerDialog DialogButtonsCrossAxisSpacing 8**. Do not reopen TimePickerCustomLayout, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding (end 6 / bottom 8 / main 8), paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DIALOG_BUTTONS_CROSS_GAP_DP` 12→8 / `dialog_buttons_cross_gap_css` |
| Catalog | Date dialog actions `flex-wrap` + `row-gap` 8 (`data-date-dialog-buttons`) |
| Hosts | Desktop + Android Cancel/OK rows `flex_wrap` with 8dp gap |
| Inventory | Date picker notes 8dp cross-axis |
| Tests | cross-gap 8 + catalog `row-gap: 8px` / `flex-wrap: wrap` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DatePickerDialog button wrap against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + YearPicker trailing HorizontalDivider + DateEntryContainer header divider + DatePickerDialog 360×568 + HeaderContainerHeight 120 / range 68 + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + **CrossAxisSpacing 8** + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + SupportLabelTop + Title + CustomLayout + Cancel/OK + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal | Recapture stills when screenshot works |

## v110 leftovers

See `docs/qa/v109_leftovers.md`.
