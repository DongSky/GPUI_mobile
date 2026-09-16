# Desktop capture v102

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v102 after origin v101 TimePicker DisplaySeparator / PeriodToggleMargin (`6ceab0e`).  
**Official:** Compose `DatePickerModalTokens.HeaderContainerHeight` is 120. `DateRangePicker` uses `RangeSelectionHeaderContainerHeight` 128 minus `HeaderHeightOffset` 60 because the default layout does not render a Save/X toolbar (close stays in the 64dp start slot).

v102 started from current `main` (origin v101) after rebasing past `1e47f4a` / v100 / v101. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **HeaderContainerHeight**. Do not reopen TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, DateRangePickerTitlePadding / DateRangePickerHeadlinePadding, DatePickerModeTogglePadding, Date Input `InputTextFieldPadding`, end-only / start-only / empty headlines, DatePickerTitlePadding / DatePickerHeadlinePadding, SelectableDates, DateInputValidator errors, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `HEADER_CONTAINER_H_DP` 120 / `RANGE_HEADER_CONTAINER_H_DP` 128 / `RANGE_HEADER_HEIGHT_OFFSET_DP` 60 / `RANGE_HEADER_MIN_H_DP` 68 / `HEADER_CONTAINER_HEIGHTS` |
| Catalog | Single-date `.head` `min-height` 120 (`data-date-header-min`); range `.head` `min-height` 68 (`data-date-range-header-min`) |
| Hosts | Desktop + Android modal / Input / range headers apply official min-heights |
| Inventory | Date picker notes HeaderContainerHeight + range 128−60 |
| Tests | 120 / 128 / 60 / 68 tokens + catalog `data-date-header-min` / `data-date-range-header-min` / `min-height: 120px` / `68px` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture HeaderContainerHeight against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official HeaderContainerHeight 120 / range 68 (128−60)** + MonthsNavigation (56 / label start / 48 arrows end) + range-header close + DialogButtonsPadding + range title/headline pads + ModeTogglePadding + Input field padding + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + ClockFace margins 36/24 + **DisplaySeparator 24 / PeriodToggleMargin 12** + Horizontal | Recapture stills when screenshot works |

## v103 leftovers

See `docs/qa/v102_leftovers.md`.
