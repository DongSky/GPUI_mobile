# Desktop capture v108

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v108 after v107 TimePickerDialogDefaults Title (`df4de63`).  
**Official:** Compose `TimePickerCustomLayout` uses portrait title top 24 / actions bottom 24 and landscape title 24 / content top 16 / content-actions 4 / actions bottom 8, with Cancel / OK in an 8dp-spaced row.

v108 started from current `main` and `docs/qa/v107_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePickerCustomLayout + dialog actions**. Do not reopen TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `DIALOG_ACTIONS` / `PORT_TITLE_TOP_DP` 24 / `PORT_ACTIONS_BOTTOM_DP` 24 / `LAND_TITLE_TOP_DP` 24 / `LAND_CONTENT_TOP_DP` 16 / `LAND_CONTENT_ACTIONS_DP` 4 / `LAND_ACTIONS_BOTTOM_DP` 8 / `DIALOG_ACTIONS_GAP_DP` 8 / `DIALOG_OK` / `DIALOG_CANCEL` / `actions_bottom_dp` |
| Catalog | Cancel / OK on expressive + vertical + horizontal (`data-time-dialog-actions`); landscape wraps `.time-body` + 8dp action bottom |
| Hosts | Desktop + Android expressive and dial heroes paint Cancel / OK with official bottom insets |
| Inventory | Time picker notes TimePickerCustomLayout + Cancel/OK |
| Tests | layout tokens + catalog `data-time-dialog-actions` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimePickerCustomLayout against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + YearPicker trailing HorizontalDivider + DateEntryContainer header divider + DatePickerDialog 360×568 + HeaderContainerHeight 120 / range 68 + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + SupportLabelTop + Title + **CustomLayout 24/24 portrait, 24/16/8 landscape + Cancel/OK** + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal | Recapture stills when screenshot works |

## v109 leftovers

See `docs/qa/v108_leftovers.md`.
