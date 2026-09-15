# Desktop capture v75

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v75 after v74 DateRangePicker range connectors (`d189a5f`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — modal DatePicker drafts the day; **OK** commits, **Cancel** discards; header divider. Docked still writes on select.

v75 started from v74 leftovers (`docs/qa/v74_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **Confirm/Cancel + header divider** on the single-date modal. Do not reopen range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, live start→end, range input sibling, YearPicker sibling, live Picker↔Input on the modal. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DATE_ACTIONS`; `DATE_DIVIDER_H_DP`; `apply_date_confirm` / `dismiss`; `date_month_of` |
| Catalog | `data-date-actions-live` + divider + Cancel/OK; live day grid; commit attrs; JS restores draft on Cancel |
| Hosts | Desktop + Android `selected_committed`; OK commits, Cancel restores; docked select also commits |
| Inventory | Date picker notes modal Confirm/Cancel (docked still immediate) |
| Tests | Divider 1dp, confirm/dismiss, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture modal Confirm/Cancel against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal calendar + live range + connectors + range Confirm/Cancel + **modal Confirm/Cancel draft** + docked; modal input; YearPicker sibling; range input | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v76 leftovers

See `docs/qa/v75_leftovers.md`.
