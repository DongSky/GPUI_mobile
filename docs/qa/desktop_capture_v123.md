# Desktop capture v123

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v123 after v122 DatePicker YearPicker a11y (`2346b4f`).  
**Official:** TimePicker uses `TimePickerPeriodToggle` “Select AM or PM”. Period shells had outline + size tokens but no PeriodToggle contentDescription.

v123 started from current `main` and `docs/qa/v122_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePicker PeriodToggle a11y**. Do not reopen YearPicker a11y, MonthsNavigation a11y, ScrollDisplayModeToggle SwipeVertical, or DisplayModeToggle strings. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `PERIOD_TOGGLE_A11Y` / `PERIOD_TOGGLE_LABEL` `Select AM or PM` |
| Catalog | `data-period-toggle-a11y` + title / aria-label on TimeScroll / TimeInput / dial period shells |
| Hosts | Desktop + Android hover tooltip on live scroll / input / dial period toggles |
| Inventory | Time picker notes PeriodToggle a11y |
| Tests | Official string + catalog `data-period-toggle-a11y` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture PeriodToggle a11y against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + MonthsNavigation a11y + YearPicker a11y | Recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + PeriodSelector Outline + DisplayModeToggle a11y + ScrollDisplayModeToggle SwipeVertical + **PeriodToggle a11y** | Recapture stills when screenshot works |

## v124 leftovers

See `docs/qa/v123_leftovers.md`.
