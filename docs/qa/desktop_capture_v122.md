# Desktop capture v122

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v122 after v121 DatePicker MonthsNavigation a11y (`0099b34`).  
**Official:** YearPicker uses `DatePickerYearPickerPaneTitle` “Year picker visible” and `DatePickerNavigateToYearDescription` “Navigate to year %1$”. Year cells had no a11y.

v122 started from current `main` and `docs/qa/v121_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePicker YearPicker a11y**. Do not reopen MonthsNavigation a11y, ScrollDisplayModeToggle SwipeVertical, or DisplayModeToggle strings. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `YEAR_PICKER_A11Y` / `YEAR_PICKER_PANE_TITLE` / `NAVIGATE_TO_YEAR` / `navigate_to_year_label` |
| Catalog | `data-year-picker-a11y` + year-region aria-label + year-cell title / aria-label + live JS |
| Hosts | Desktop + Android hover tooltip on live date / range / docked year cells |
| Inventory | Date picker notes YearPicker a11y |
| Tests | Official strings + catalog `data-year-picker-a11y` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture YearPicker a11y against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + MonthsNavigation a11y + **YearPicker a11y** | Recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + PeriodSelector Outline + DisplayModeToggle a11y + ScrollDisplayModeToggle SwipeVertical | Recapture stills when screenshot works |

## v123 leftovers

See `docs/qa/v122_leftovers.md`.
