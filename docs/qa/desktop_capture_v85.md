# Desktop capture v85

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v85 after v84 SelectableDates Input not-allowed (`b1d53e3`).  
**Official:** Compose `SelectableDates` greys out unselectable days on the Picker grid (demo Sat/Sun).

v85 started from v84 leftovers (`docs/qa/v84_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **calendar-grid weekend disable**. Do not reopen SelectableDates Input not-allowed, DatePicker Input format/year-range, DateRangePicker year-range/format/order, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DayKind::Disabled`; `SELECTABLE_DATES_GRID`; `day_accepts_tap`; weekend classify; tap helpers no-op |
| Catalog | Modal `data-selectable-dates`; Sat/Sun `data-kind="Disabled"`; JS skips weekend taps |
| Hosts | Desktop + Android grey weekend cells and ignore clicks |
| Inventory | Date picker notes grid weekend disable |
| Tests | Sep 12 Disabled; docked/range weekend taps no-op; catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture weekend-disabled grids against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **Input + Picker SelectableDates weekends** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v86 leftovers

See `docs/qa/v85_leftovers.md`.
