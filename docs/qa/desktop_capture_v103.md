# Desktop capture v103

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v103 after origin v102 HeaderContainerHeight (`a0a7fd9`).  
**Official:** Compose `DatePickerDialog` uses `DatePickerModalTokens.ContainerWidth` 360 (`requiredWidth`) and `ContainerHeight` 568 (`heightIn(max)`). Docked DatePicker is not this dialog.

v103 started from current `main` (origin v102) and `docs/qa/v102_leftovers.md` after the timer’s v102 slot was already taken. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePickerDialog ContainerWidth / ContainerHeight**. Do not reopen HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `CONTAINER_W_DP` 360 / `CONTAINER_H_DP` 568 / `CONTAINER_SIZE` |
| Catalog | `.cal` is 360×568 max (`data-date-container`) |
| Hosts | Desktop + Android modal date / range / input cards use 360×568; docked stays field-width |
| Inventory | Date picker notes ContainerWidth / ContainerHeight |
| Tests | 360 / 568 tokens + catalog `data-date-container` / `max-width: 360px; max-height: 568px` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DatePickerDialog container size against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **official DatePickerDialog 360×568** + HeaderContainerHeight 120 / range 68 + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal | Recapture stills when screenshot works |

## v104 leftovers

See `docs/qa/v103_leftovers.md`.
