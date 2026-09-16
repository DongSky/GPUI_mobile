# Desktop capture v76

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v76 after v75 DatePicker Confirm/Cancel (`776b500`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — modal DatePicker month pager (prev / next) clamped to YearRange 1900–2100.

v76 started from v75 leftovers (`docs/qa/v75_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **month prev/next** on the single-date modal (catalog was month ▾ only; hosts paged without YearRange clamp). Do not reopen DatePicker Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DATE_MONTH_NAV`; `DATE_PREV_MONTH` / `NEXT`; `apply_date_month` YearRange clamp |
| Catalog | `data-date-month-nav` / `data-date-month-delta`; JS pages the live grid |
| Hosts | Desktop + Android `shift_date_month` uses YearRange clamp |
| Inventory | Date picker notes modal month nav (independent of the range hero) |
| Tests | Clamp 1900/2100, catalog month-nav attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture modal month nav against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + **month nav** + Confirm/Cancel + live range + connectors + docked | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v77 leftovers

See `docs/qa/v76_leftovers.md`.
