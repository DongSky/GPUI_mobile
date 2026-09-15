# Desktop capture v67

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v67 after v66 live Picker↔Input (`30c8731`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — modal menu button opens year selection. Compose `YearPicker` / `DatePickerDefaults.YearRange` 1900–2100 / `DatePickerModalTokens` 72×36.

v67 started from v66 leftovers (`docs/qa/v66_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **YearPicker**. Do not reopen live Picker↔Input, modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DatePickerPane` Calendar/Year; YearRange 1900–2100; 3×72×36; `year_window` / `classify_year` |
| Catalog | Live month ▾ on the modal; `data-hero="datepicker-year"` sibling starts on Year |
| Hosts | Desktop + Android month ▾ opens year pills; tap year returns to calendar |
| Inventory | Date picker notes YearPicker |
| Tests | Year tokens, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture YearPicker against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + range + docked; modal input sibling; live showModeToggle; YearPicker | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v68 leftovers

See `docs/qa/v67_leftovers.md`.
