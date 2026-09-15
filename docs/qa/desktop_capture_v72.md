# Desktop capture v72

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v72 after v71 DateRangePicker YearPicker (`1b5504f`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — DateRangePicker `showModeToggle`: Picker calendar ↔ Input Start/End outlined MM/DD/YYYY.

v72 started from v71 leftovers (`docs/qa/v71_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **showModeToggle on the existing range hero**. Do not reopen range-hero YearPicker, month nav, live start→end, range input sibling, YearPicker sibling, live Picker↔Input on the single-date modal, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_SHOW_MODE_TOGGLE`; `range_title_for`; `range_field_value` |
| Catalog | `data-range-display-live` + toggle; Input shows Start/End fields |
| Hosts | Desktop + Android `range_display` independent of `date_display` |
| Inventory | Date picker notes range-hero showModeToggle |
| Tests | Title swap, field value, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range-hero showModeToggle against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + live range start→end + month nav + YearPicker + **range showModeToggle** + docked; modal input; live showModeToggle; YearPicker sibling; range input | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v73 leftovers

See `docs/qa/v72_leftovers.md`.
