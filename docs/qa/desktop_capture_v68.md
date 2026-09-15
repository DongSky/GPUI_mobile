# Desktop capture v68

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v68 after v67 YearPicker (`147db96`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — modal date input configurations: single date input **and date range input**. Compose `DateRangePicker` `DisplayMode.Input`.

v68 started from v67 leftovers (`docs/qa/v67_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **modal date range input**. Do not reopen YearPicker, live Picker↔Input, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_INPUT_HEADLINE` Enter dates; Start/End labels; `is_range_input_valid` |
| Catalog | `data-hero="datepicker-range-input"`: two outlined fields, header divider, Cancel/OK |
| Hosts | Desktop + Android range-input cards |
| Inventory | Date picker notes modal date range input |
| Tests | Range parse order, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture date range input against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + range hero + docked; modal input; live showModeToggle; YearPicker; range input | Live range start→end selection still static hero |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v69 leftovers

See `docs/qa/v68_leftovers.md`.
