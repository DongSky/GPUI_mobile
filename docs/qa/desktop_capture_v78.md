# Desktop capture v78

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v78 after v77 DatePicker docked YearPicker (`b4ad240`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — docked `DatePickerDocked` tap writes the field and dismisses.

v78 started from v77 leftovers (`docs/qa/v77_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **live day select on the docked popup**. Do not reopen docked YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DOCKED_LIVE_SELECT`; `apply_docked_tap` |
| Catalog | `data-docked-select-live` / `data-docked-dismiss-select`; tap InMonth day writes field `.val` + dismisses |
| Hosts | Desktop + Android `select_docked_day` |
| Inventory | Date picker notes live docked day select |
| Tests | `apply_docked_tap` commit + dismiss; catalog live-select attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture docked live select against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked YearPicker + **docked live select** + docked month nav | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v79 leftovers

See `docs/qa/v78_leftovers.md`.
