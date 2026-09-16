# Desktop capture v115

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v115 after origin v114 TimePickerDefaults shapes CornerLarge (`a74d66d`). Timer asked v114 while origin landed CornerLarge as v114 — this slot is **v115**.  
**Official:** `TimeInputDefaults.vibrantColors()` / `defaultVibrantTimeInputColors` uses `SurfaceContainerLowest` field containers, focused text `Primary`, unfocused text `OnSurface`, focused border `Primary` 2dp, unfocused border `Transparent` 1dp (`OutlinedTextFieldDefaults.Container`). Field shape stays shipped CornerLarge 16.

v115 started from current `main` and `docs/qa/v114_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimeInput vibrant field outline + colors**. Do not reopen TimePickerDefaults.shapes / ScrollFieldDefaults.shape CornerLarge 16, fullScreenContainedSearchBarColor, DatePicker WeekDays, VibrantTimePickerDialog, TimePicker ClockFaceSizeModifier, DatePickerDialog CrossAxisSpacing, TimePickerCustomLayout paddings / Cancel/OK, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped. Do not apply VibrantTimeField 100/132×120, VibrantHorizontalGap 52, or VibrantTimePickerCustomLayout 12. Do not restore ExtraLarge 28 on TimeScroll / TimeInput fields.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TIME_FIELD_FOCUS_OUTLINE_W_DP` 2 / `TIME_FIELD_UNFOCUSED_OUTLINE_W_DP` 1 / `TIME_FIELD_OUTLINE` / `time_field_outline_w_css`; `TimeInputAppearance.field_outline` / `field_focused_outline`; `resolve_input` SurfaceContainerLowest + Primary focused text/outline + Transparent idle outline |
| Catalog | `data-time-field-outline` + hour 2px Primary / minute 1px Transparent; focus/blur paint; CornerLarge 16 stays |
| Hosts | `desktop_time_input_field` / `android_time_input_field` `border_2` focused / `border_1` idle |
| Inventory | Time picker notes vibrant field outline |
| Tests | token colors + catalog `data-time-field-outline` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimeInput field outline against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + dividers + 360×568 + HeaderContainerHeight + MonthsNavigation + range-header close + DialogButtonsPadding + CrossAxisSpacing 8 + pads + empty headlines + SelectableDates + WeekDays BodyLarge / OnSurface / 48 | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + CornerLarge 16 + **vibrant field outline** + SupportLabelTop + Title + CustomLayout + ClockFaceSizeModifier + VibrantTimePickerDialog + Cancel/OK + 24h ClockFace | Recapture stills when screenshot works |

## v116 leftovers

See `docs/qa/v115_leftovers.md`.
