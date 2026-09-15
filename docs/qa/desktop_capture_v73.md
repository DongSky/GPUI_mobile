# Desktop capture v73

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v73 after v72 DateRangePicker showModeToggle (`1c14f48`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — modal DateRangePicker drafts the range; **OK** commits, **Cancel** discards; header divider.

v73 started from v72 leftovers (`docs/qa/v72_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **Confirm/Cancel + header divider** on the existing range hero. Do not reopen range-hero showModeToggle, YearPicker, month nav, live start→end, range input sibling, YearPicker sibling, live Picker↔Input on the single-date modal. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `RANGE_ACTIONS`; `RANGE_DIVIDER_H_DP`; `apply_range_confirm` / `dismiss`; `range_month_of` |
| Catalog | `data-range-divider` + Cancel/OK; commit attrs; JS restores draft on Cancel |
| Hosts | Desktop + Android `date_range_committed`; OK commits, Cancel restores |
| Inventory | Date picker notes range-hero Confirm/Cancel |
| Tests | Divider 1dp, dismiss/demo month, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture range-hero Confirm/Cancel against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + live range start→end + month nav + YearPicker + showModeToggle + **Confirm/Cancel draft** + docked; modal input; YearPicker sibling; range input | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v74 leftovers

See `docs/qa/v73_leftovers.md`.
