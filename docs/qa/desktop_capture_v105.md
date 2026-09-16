# Desktop capture v105

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v105 after origin v104 DateEntryContainer HorizontalDivider (`388f2a2`).  
**Official:** Compose `SwitchableDateEntryContent` paints `HorizontalDivider(color = colors.dividerColor)` under `YearPicker` while the year pane is open (`yearPickerVisible`). This is not the DateEntryContainer header divider and not the DatePickerDialog Cancel/OK actions divider.

v105 started from current `main` (origin v104) and `docs/qa/v104_leftovers.md` after this agent’s v104 slot was already taken. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **YearPicker trailing HorizontalDivider**. Do not reopen DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `YEAR_PICKER_DIVIDER` / `YEAR_PICKER_DIVIDER_H_DP` 1 / `year_picker_divider_height_css` / `year_picker_divider_visible` |
| Catalog | `.dp-year-divider` after `.dp-years` on live / range / docked / year sibling (`data-date-year-divider`); hidden in calendar/input |
| Hosts | Desktop + Android paint a 1dp outline-variant divider after the year grid while the year pane is open |
| Inventory | Date picker notes YearPicker trailing HorizontalDivider |
| Tests | tokens + visibility Year vs Calendar + 4 catalog `data-date-year-divider` markers |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture YearPicker trailing HorizontalDivider against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **YearPicker trailing HorizontalDivider** + DateEntryContainer header divider + DatePickerDialog 360×568 + HeaderContainerHeight 120 / range 68 + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal | Recapture stills when screenshot works |

## v106 leftovers

See `docs/qa/v105_leftovers.md`.
