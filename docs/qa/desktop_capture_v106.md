# Desktop capture v106

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v106 after v105 YearPicker trailing HorizontalDivider (`625897c`).  
**Official:** Compose TimeInput paints `SupportingText` (`Hour` / `Minute`) under each field with `padding(top = SupportLabelTop)` 7dp (`TimeInputTokens.TimeFieldSupportingTextColor` / `TimeFieldSupportingTextFont`).

v106 started from current `main` and `docs/qa/v105_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimeInput SupportLabelTop**. Do not reopen YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `SUPPORT_LABEL` / `SUPPORT_LABEL_TOP_DP` 7 / `INPUT_HOUR_LABEL` / `INPUT_MINUTE_LABEL` / `ScrollKind::support_label` + appearance bodySmall / on-surface-variant |
| Catalog | Hour / Minute labels under TimeInput fields (`data-time-support-label`, `.time-input-support` 7dp) |
| Hosts | Desktop + Android TimeInput columns paint supporting text under each 96×72 field |
| Inventory | Time picker notes SupportLabelTop |
| Tests | 7dp token + Hour/Minute labels + catalog `data-time-support-label` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimeInput SupportLabelTop against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + YearPicker trailing HorizontalDivider + DateEntryContainer header divider + DatePickerDialog 360×568 + HeaderContainerHeight 120 / range 68 + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + **Hour/Minute SupportLabelTop 7** + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal | Recapture stills when screenshot works |

## v107 leftovers

See `docs/qa/v106_leftovers.md`.
