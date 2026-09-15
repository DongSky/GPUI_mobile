# Desktop capture v74

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v74 after v73 DateRangePicker Confirm/Cancel (`3da9545`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — Compose `DateRangePicker.kt` `drawRangeBackground` (`firstIsSelectionStart` / `lastIsSelectionEnd` half-cell offsets).

v74 started from v73 leftovers (`docs/qa/v73_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **range-hero `drawRangeBackground` half-cell connectors**. Do not reopen range-hero Confirm/Cancel (draft until OK, header divider), showModeToggle, YearPicker, month nav, live start→end, range input sibling, YearPicker sibling, live Picker↔Input on the single-date modal, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_CONNECTOR`; `RangeFill` + `range_fills` (StartHalf / EndHalf / Full) |
| Catalog | `data-range-connector` + `data-range-fill`; live JS re-paint matches Compose half-cell bars |
| Hosts | Desktop + Android half-bar + selected circle on the live range hero |
| Inventory | Date picker notes `drawRangeBackground` |
| Tests | StartHalf day 15, Full day 18, EndHalf day 21; catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range-hero connectors against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + live range start→end + month nav + YearPicker + showModeToggle + Confirm/Cancel + **half-cell connectors** + docked; modal input; live showModeToggle; YearPicker sibling; range input | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v75 leftovers

See `docs/qa/v74_leftovers.md`.
