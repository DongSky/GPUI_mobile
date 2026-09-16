# Desktop capture v79

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v79 after v78 DatePicker docked live select (`8a29fce`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — Compose `DateRangePicker.kt` `VerticalMonthsList` (LazyColumn of months + `RangeSelectionMonthSubheadFont` / `CalendarMonthSubheadPadding` 24/20/8).

v79 started from v78 leftovers (`docs/qa/v78_leftovers.md`). Main had advanced past the original v75 start (`d189a5f`) through v75–v78; this pass took the next unused version. Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **range-hero `VerticalMonthsList` month subheads**. Do not reopen docked live select, docked YearPicker, single-date modal month nav / Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, live start→end, range input sibling, YearPicker sibling, live Picker↔Input on the single-date modal, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_VERTICAL_MONTHS`; `range_visible_months`; `month_subhead_label`; titleSmall + 24/20/8 pad |
| Catalog | `data-range-vertical-months` + `data-range-months` + `data-range-subhead`; live JS re-paints both months |
| Hosts | Desktop + Android stacked Sep+Oct with month/year subheads |
| Inventory | Date picker notes `VerticalMonthsList` |
| Tests | Visible window (2026,9)+(2026,10); catalog October 2026 subhead |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range-hero VerticalMonthsList against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + month nav + Confirm/Cancel + live range start→end + YearPicker + showModeToggle + half-cell connectors + **VerticalMonthsList subheads** + docked YearPicker + docked live select; modal input; live showModeToggle; YearPicker sibling; range input | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v80 leftovers

See `docs/qa/v79_leftovers.md`.
