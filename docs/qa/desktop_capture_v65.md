# Desktop capture v65

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v65 commit after `4f56725` (v64 search filter chips).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/overview) — three variants: docked, modal, and **modal date input**. Compose `DatePickerDisplayMode.Input`.

v65 started from v64 leftovers (`docs/qa/v64_leftovers.md`). A parallel agent took v64 for search filter chips. Highest-impact **newly published spec delta** without an NDK/gpui bump: **modal date input**. Do not reopen filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DatePickerDisplayMode` Picker/Input; `input_field_value` / `parse_input_field`; outlined MM/DD/YYYY tokens |
| Catalog | `data-hero="datepicker-input"` sibling: headline, supporting, toggle, outlined Date field, Cancel/OK |
| Hosts | Desktop + Android paint a modal-input card next to the calendar |
| Inventory | Date picker notes modal date input |
| Tests | Parse/format, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture modal date input against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + range + docked; modal input MM/DD/YYYY sibling | Live Picker↔Input toggle on the existing modal still open |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v66 leftovers

See `docs/qa/v65_leftovers.md`.
