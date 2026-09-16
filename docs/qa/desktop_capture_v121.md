# Desktop capture v121

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v121 after v120 ScrollDisplayModeToggle SwipeVertical (`657c145`).  
**Official:** DatePicker `MonthsNavigation` uses `DatePickerSwitchToPreviousMonth` / `DatePickerSwitchToNextMonth` / `DatePickerSwitchToYearSelection` / `DatePickerSwitchToDaySelection`. Hosts/catalog still painted unofficial “Previous month” / “Next month” and unlabeled year menus.

v121 started from current `main` and `docs/qa/v120_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePicker MonthsNavigation a11y**. Do not reopen ScrollDisplayModeToggle SwipeVertical or DisplayModeToggle / DatePicker DisplayModeToggleButton a11y strings. Do not apply `periodSelectorShape` CornerFull or TimeSelector 2dp selected outline. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `MONTH_NAV_A11Y` / `PREV_MONTH` / `NEXT_MONTH` / `SWITCH_TO_YEAR` / `SWITCH_TO_DAY` / `year_toggle_label` |
| Catalog | `data-month-nav-a11y` + title / aria-label on month arrows and year menus + live JS pane flip |
| Hosts | Desktop + Android hover tooltip on live date / range / docked month arrows and year toggles |
| Inventory | Date picker notes MonthsNavigation a11y |
| Tests | Official strings + catalog `data-month-nav-a11y` + no unofficial Previous/Next month |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture MonthsNavigation a11y against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + WeekDays + DisplayModeToggleButton a11y + **MonthsNavigation a11y** | Recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + PeriodSelector Outline + DisplayModeToggle a11y + ScrollDisplayModeToggle SwipeVertical | Recapture stills when screenshot works |

## v122 leftovers

See `docs/qa/v121_leftovers.md`.
