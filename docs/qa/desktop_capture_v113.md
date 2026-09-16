# Desktop capture v113

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v113 after v112 DatePicker WeekDays (`a573d7d`).  
**Official:** `SearchBarDefaults.fullScreenContainedSearchBarColor` is `surfaceContainerLow`. Collapsed and docked contained search stay `SearchBarTokens.ContainerColor` (`surfaceContainerHigh`).

v113 started from current `main` and `docs/qa/v112_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **fullScreenContainedSearchBarColor**. Do not reopen DatePicker WeekDays, VibrantTimePickerDialog, TimePicker ClockFaceSizeModifier, DatePickerDialog CrossAxisSpacing, TimePickerCustomLayout paddings / Cancel/OK, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `FULL_SCREEN_CONTAINED_COLOR` / `full_screen_contained_container` / `contained_container_for` |
| Catalog | Compact expanded heroes `data-search-fullscreen-color` + surfaceContainerLow |
| Hosts | Android compact open uses surfaceContainerLow; desktop docked stays high |
| Inventory | Search notes fullScreenContainedSearchBarColor / surfaceContainerLow |
| Tests | token colors + catalog `data-search-fullscreen-color` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture fullScreenContainedSearchBarColor against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + dividers + 360×568 + HeaderContainerHeight + MonthsNavigation + range-header close + DialogButtonsPadding + CrossAxisSpacing 8 + pads + empty headlines + SelectableDates + WeekDays BodyLarge / OnSurface / 48 | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips + **fullscreen surfaceContainerLow** | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + SupportLabelTop + Title + CustomLayout + ClockFaceSizeModifier + VibrantTimePickerDialog + Cancel/OK + 24h ClockFace | Recapture stills when screenshot works |

## v114 leftovers

See `docs/qa/v113_leftovers.md`.
