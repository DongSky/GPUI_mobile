# Desktop capture v70

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v70 after v69 live date range start→end (`21edb69`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — DateRangePicker month pager: prev/next; InRange fill continues across months.

v70 started from v69 leftovers (`docs/qa/v69_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **DateRangePicker month navigation** on the existing range hero. Do not reopen live range start→end, range input, YearPicker, live Picker↔Input, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_MONTH_NAV`; `apply_range_month` YearRange clamp |
| Catalog | `data-range-month-nav` prev/next; JS pages `data-range-year`/`month` and re-paints |
| Hosts | Desktop + Android `range_year` / `range_month` + prev/next clicks |
| Inventory | Date picker notes range-hero month nav + cross-month InRange |
| Tests | Sept→Oct, YearRange clamp, Sept 28–Oct 5 InRange |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range-hero month nav against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + live range start→end + **range month nav** + docked; modal input; live showModeToggle; YearPicker; range input | Range-hero YearPicker / showModeToggle still missing |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v71 leftovers

See `docs/qa/v70_leftovers.md`.
