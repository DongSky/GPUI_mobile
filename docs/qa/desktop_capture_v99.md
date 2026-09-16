# Desktop capture v99

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v99 after origin v98 DateRangePicker header chrome (`39d398d`).  
**Official:** Compose `MonthsNavigation` uses `MonthYearHeight` 56 with the year menu at start and `RecommendedSizeForAccessibility` 48 prev/next IconButtons grouped at end. Year pane hides the arrows (`Arrangement.Start`).

v99 started from current `main` (origin v98) and `docs/qa/v98_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **MonthsNavigation MonthYearHeight**. Do not reopen DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, DateRangePickerTitlePadding / DateRangePickerHeadlinePadding, DatePickerModeTogglePadding, Date Input `InputTextFieldPadding`, end-only / start-only / empty headlines, DatePickerTitlePadding / DatePickerHeadlinePadding, SelectableDates, DateInputValidator errors, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `MONTH_YEAR_H_DP` 56 / `MONTH_NAV_ICON_DP` 48 / `MONTH_YEAR_NAV` |
| Catalog | `.month-nav` is 56dp; year label start + `.month-nav-arrows` 48×48 at end; year-pane hides arrows (`data-date-month-year`) |
| Hosts | Desktop + Android single-date / range / docked month rows: 56 height, label start, 48dp prev+next group |
| Inventory | Date picker notes MonthsNavigation MonthYearHeight |
| Tests | 56 / 48 tokens + catalog `data-date-month-year` / `month-nav-arrows` / `height: 56px` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture MonthsNavigation MonthYearHeight against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official MonthsNavigation (56 / label start / 48 arrows end)** + range-header close + DialogButtonsPadding + range title/headline pads + ModeTogglePadding + Input field padding + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v100 leftovers

See `docs/qa/v99_leftovers.md`.
