# Desktop capture v126

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v126 after v125 TimePicker clock-number a11y (`6636cad`).  
**Official:** DatePicker day cells use `DatePickerTodayDescription` “Today” and `DateRangePickerDayInRange` “In range”.

v126 started from current `main` and `docs/qa/v125_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePicker day-cell a11y**. Do not reopen clock-number a11y, hour/minute a11y, PeriodToggle a11y, YearPicker a11y, MonthsNavigation a11y, ScrollDisplayModeToggle SwipeVertical, or DisplayModeToggle strings. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DAY_CELL_A11Y` / `TODAY_DESCRIPTION` / `DAY_IN_RANGE` / `day_cell_a11y` |
| Catalog | `data-day-cell-a11y` + title / aria-label on Today / In range cells + live JS paint |
| Hosts | Desktop + Android hover tooltip on live date / range / docked today and in-range days |
| Inventory | Date picker notes day-cell a11y |
| Tests | Official strings + catalog `data-day-cell-a11y` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture day-cell a11y against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + MonthsNavigation a11y + YearPicker a11y + **day-cell a11y** | HeadlineDescription / NoSelection still leftover; recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + PeriodToggle + hour/minute + clock-number a11y | Recapture stills when screenshot works |

## v127 leftovers

See `docs/qa/v126_leftovers.md`.
