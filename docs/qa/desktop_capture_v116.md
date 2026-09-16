# Desktop capture v116

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v116 after v115 TimeInput field outline (`d74b850`).  
**Official:** `TimePickerTokens.TimeSelectorContainerColor` / `TimeSelectorLabelTextColor` are `PrimaryContainer` / `OnPrimaryContainer` when the hour or minute display is selected; unselected uses `SurfaceContainerHighest` / `OnSurface`. Catalog previously painted both fields with ClockFace colors; hosts reused clock-number Primary / OnPrimary.

v116 started from current `main` and `docs/qa/v115_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimeSelector selected / idle colors**. Do not reopen TimeInput field outline, TimePickerDefaults.shapes CornerLarge 16, fullScreenContainedSearchBarColor, DatePicker WeekDays, VibrantTimePickerDialog, TimePicker ClockFaceSizeModifier, DatePickerDialog CrossAxisSpacing, TimePickerCustomLayout paddings / Cancel/OK, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Do not apply TimeSelector 2dp primary outline or periodSelectorShape CornerFull (keep CornerSmall 8 period buttons). Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TIME_SELECTOR_COLORS` + `TimePickerAppearance` selected PrimaryContainer / OnPrimaryContainer, idle SurfaceContainerHighest / OnSurface |
| Catalog | `data-time-selector-colors` + hour selected / minute idle; `setDial` swaps colors |
| Hosts | Desktop + Android TimeSelector fields use the new appearance roles (clock numbers stay Primary / OnPrimary) |
| Inventory | Time picker notes TimeSelector PrimaryContainer |
| Tests | token colors + catalog `data-time-selector-colors` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimeSelector colors against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + dividers + 360×568 + HeaderContainerHeight + MonthsNavigation + range-header close + DialogButtonsPadding + CrossAxisSpacing 8 + pads + empty headlines + SelectableDates + WeekDays BodyLarge / OnSurface / 48 | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + CornerLarge 16 + field outline + **TimeSelector PrimaryContainer** + SupportLabelTop + Title + CustomLayout + ClockFaceSizeModifier + VibrantTimePickerDialog + Cancel/OK + 24h ClockFace | Recapture stills when screenshot works |

## v117 leftovers

See `docs/qa/v116_leftovers.md`.
