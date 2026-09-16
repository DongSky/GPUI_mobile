# Desktop capture v81

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v81 after v80 DatePicker docked trailing DateRange (`47a4743`).  
**Official:** Compose `DateInputValidator` — `Date format not recognized` / `End date can't be before start date`.

v81 started from v80 leftovers (`docs/qa/v80_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateRangePicker Input supporting-text errors**. Do not reopen docked trailing DateRange, range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_INPUT_ERRORS`; `INPUT_ERROR_FORMAT` / `RANGE_INPUT_ERROR_ORDER`; `DateInputError`; `range_input_error` |
| Catalog | `datepicker-range-input-errors` hero: format `13/40/2026` + order 09/21–09/15 |
| Hosts | Desktop + Android range Input supporting slot when validator fails |
| Inventory | Date picker notes DateInputValidator errors |
| Tests | Format/order/empty cases; catalog error copy + attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range input errors against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + **range Input errors** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v82 leftovers

See `docs/qa/v81_leftovers.md`.
