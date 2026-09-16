# Desktop capture v87

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v87 after v86 DateRangePicker Input SelectableDates not-allowed (`656d475`).  
**Official:** Compose `DatePickerHeadline` shows `Entered date` (`m3c_date_input_headline`) when Input has no selection.

v87 started from v86 leftovers (`docs/qa/v86_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePickerHeadline Input empty**. Do not reopen range/single-date SelectableDates not-allowed, weekend grid disable, DateInputValidator year-range/format/order, docked trailing DateRange, VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors/Confirm/Cancel/showModeToggle/YearPicker/month nav/live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `INPUT_EMPTY_HEADLINE` / `PICKER_EMPTY_HEADLINE`; `date_headline` / `empty_headline` |
| Catalog | `datepicker-input-empty` sibling: `Entered date` + empty `MM/DD/YYYY` |
| Hosts | Desktop + Android empty Input cards |
| Inventory | Date picker notes DatePickerHeadline Input empty |
| Tests | Catalog `data-date-input-empty`; `date_headline(Input, None) == "Entered date"` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Input empty headline against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + Input errors + **Input empty Entered date** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v88 leftovers

See `docs/qa/v87_leftovers.md`.
