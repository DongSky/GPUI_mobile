# Desktop capture v86

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v86 after v85 SelectableDates grid weekend disable (`62b0ab1`).  
**Official:** Compose `DateInputValidator` applies `SelectableDates` to DateRangePicker Input Start/End (demo weekends → `Date not allowed`).

v86 started from v85 leftovers (`docs/qa/v85_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePicker Input SelectableDates not-allowed**. Do not reopen calendar-grid weekend disable, single-date SelectableDates / Date not allowed, DatePicker Input format/year-range, DateRangePicker year-range/format/order, docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `range_input_error` docs: pattern → year range → SelectableDates → order (logic already returned `NotAllowed`) |
| Catalog | `datepicker-range-input-errors` fourth card: weekend start `09/12/2026` + `data-range-error="allowed"` |
| Hosts | Desktop + Android range Input error cards (format / year / not-allowed / order) |
| Inventory | Date picker notes range Input `Date not allowed` |
| Tests | Catalog `data-range-error="allowed"`; end-weekend `09/13/2026` NotAllowed; host range sample |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range Input not-allowed against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **Input + Picker SelectableDates weekends** + **range Input not-allowed** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v87 leftovers

See `docs/qa/v86_leftovers.md`.
