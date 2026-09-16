# Desktop capture v101

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v101 after v100 TimePicker ClockFace margins (`65db2b7`).  
**Official:** Compose `ClockDisplayNumbers` sizes the hour:minute colon with `DisplaySeparatorWidth` 24 × `PeriodSelectorVerticalContainerHeight` 80. TimeInput uses the same 24 width × `PeriodSelectorContainerHeight` 72. `PeriodToggleMargin` 12 insets AM/PM (start on vertical, top on horizontal).

v101 started from current `main` (v100) and `docs/qa/v100_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePicker DisplaySeparator + PeriodToggleMargin**. Do not reopen ClockFace 36/24 margins, horizontal 24 gap, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome, TimeScroll colon. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `DISPLAY_SEPARATOR_W_DP` 24 / `DISPLAY_SEPARATOR_H_DP` 80 / `INPUT_DISPLAY_SEPARATOR_H_DP` 72 / `PERIOD_TOGGLE_MARGIN_DP` 12 |
| Catalog | Dial + TimeInput colons are 24-wide slots (`data-display-separator`); period inset 12 (`data-period-toggle-margin`) |
| Hosts | Desktop + Android dial / TimeInput colons 24-wide; period 12 start or top |
| Inventory | Time picker notes DisplaySeparatorWidth / PeriodToggleMargin |
| Tests | 24 / 80 / 72 / 12 tokens + catalog attributes |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimePicker DisplaySeparator against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal + ClockFace 36/24 + **DisplaySeparator 24 + PeriodToggleMargin 12** | Recapture stills when screenshot works |

## v102 leftovers

See `docs/qa/v101_leftovers.md`.
