# Desktop capture v114

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v114 after origin v113 fullScreenContainedSearchBarColor (`dd7015e`). Main advanced past `5773d15` (v112 DatePicker WeekDays, v113 search color), so this slot is **v114**.  
**Official:** Compose `TimePickerDefaults.shapes().timeFieldShape` and `ScrollFieldDefaults.shape` are `ShapeKeyTokens.CornerLarge` / `ShapeDefaults.Large` (16). TimeScroll / TimeInput fields used unofficial ExtraLarge 28.

v114 started from current `main` and `docs/qa/v113_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePickerDefaults.shapes / ScrollFieldDefaults.shape CornerLarge**. Do not reopen fullScreenContainedSearchBarColor, DatePicker WeekDays, VibrantTimePickerDialog, TimePicker ClockFaceSizeModifier, DatePickerDialog CrossAxisSpacing, TimePickerCustomLayout paddings / Cancel/OK, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Do not apply VibrantTimePicker field sizes. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TIME_PICKER_SHAPES` / `TIME_FIELD_SHAPE_CORNER_DP` 16 / `SCROLL_FIELD_CORNER_DP` / `INPUT_FIELD_CORNER_DP` / `time_field_shape_corner_css` |
| Catalog | Expressive hero `data-time-picker-shapes` / `data-time-field-shape="large"` + 16dp TimeScroll / TimeInput fields |
| Hosts | Desktop + Android already paint `field_corners` (16) |
| Inventory | Time picker notes CornerLarge 16 |
| Tests | token 16 + catalog `data-time-picker-shapes` / `data-time-field-shape` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimePickerDefaults.shapes CornerLarge against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + WeekDays BodyLarge/OnSurface + 48dp row + dividers + 360×568 + HeaderContainerHeight + MonthsNavigation + range-header close + DialogButtonsPadding + CrossAxisSpacing 8 + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + fullScreenContainedSearchBarColor + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + SupportLabelTop + Title + CustomLayout + ClockFaceSizeModifier + VibrantTimePickerDialog + **TimePickerDefaults.shapes / ScrollFieldDefaults.shape CornerLarge 16** + Cancel/OK + 24h ClockFace | Recapture stills when screenshot works |

## v115 leftovers

See `docs/qa/v114_leftovers.md`.
