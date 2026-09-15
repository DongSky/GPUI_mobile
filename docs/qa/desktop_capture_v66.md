# Desktop capture v66

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v66 after v65 modal date input (`c3ae7fa`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/guidelines) — “You can swap between the modal date picker and modal date input using the edit or calendar icon.” Compose `showModeToggle` / `DatePickerState.displayMode`.

v66 started from v65 leftovers (`docs/qa/v65_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **live Picker↔Input toggle** on the existing modal. Do not reopen modal date input sibling, filter chips, search empty/40/20/clear-X/two-line/segmented, TimePicker Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `LIVE_DISPLAY_MODE` Picker; `SHOW_MODE_TOGGLE`; `apply_display_toggle`; `supporting_for` |
| Catalog | `data-hero="datepicker"` is live (`data-date-display-live`); edit/calendar swaps grid ↔ outlined field; sibling stays static Input |
| Hosts | Desktop + Android modal `date_display` + clickable 48dp toggle |
| Inventory | Date picker notes live Picker↔Input showModeToggle |
| Tests | Live attrs, toggle tokens |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture live Picker↔Input against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + range + docked; modal input sibling; live showModeToggle | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v67 leftovers

See `docs/qa/v66_leftovers.md`.
