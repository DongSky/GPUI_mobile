# Desktop capture v77

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v77 after v76 DatePicker month nav (`4c0f22c`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — docked DatePicker month ▾ opens Compose `YearPicker` (3×72×36, YearRange 1900–2100).

v77 started from v76 leftovers (`docs/qa/v76_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **YearPicker on the docked popup**. Do not reopen DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DOCKED_YEAR_PANE`; `apply_docked_year` |
| Catalog | `data-docked-pane` / `data-docked-year-toggle` / `data-docked-years` / `data-docked-year` |
| Hosts | Desktop + Android `docked_pane` independent of `date_pane` / `range_pane` |
| Inventory | Date picker notes docked YearPicker |
| Tests | Year clamp, catalog docked year attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture docked YearPicker against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + **docked YearPicker** + docked month nav | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v78 leftovers

See `docs/qa/v77_leftovers.md`.
