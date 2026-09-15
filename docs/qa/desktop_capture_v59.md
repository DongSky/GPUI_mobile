# Desktop capture v59

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v59 commit after `b9269b4` (v58 TimePickerLayoutType Horizontal).  
**Official:** [m3.material.io search guidelines](https://m3.material.io/components/search/guidelines) / [specs](https://m3.material.io/components/search/specs) — suggestions and results use the list component. Expressive lists use `segmentedShapes`: 2dp gap, 4dp inner / 16dp outer corners, selected 16dp + secondary-container.

v59 started from v58 leftovers (`docs/qa/v58_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **segmented 2dp filled list items inside search result groups**. Do not reopen TimePickerLayoutType Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual tokens. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `ROW_GAP_DP` 2 / inner 4 / outer 16; `row_corners` / `row_container` / `row_content`; 2dp intra-group gaps in list heights |
| Catalog | `.sv-group` gap 2px + 8px inset; `data-search-row="segmented"` + `data-search-list="segmented"`; selected = query match; flatten corners when queried |
| Hosts | Desktop + Android: group `.gap(ROW_GAP_DP).px(8)`; per-row bg + four-corner rounded; selected via query match |
| Inventory | Search notes segmented filled rows |
| Tests | `ROW_GAP` 2, first-row outer 16, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture segmented search rows against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen; medium docked + 32% scrim; Quick results / Results; groups; segmented 2/4/16 rows | `cx.transform` search morph skipped; trailing clear-X / no-results empty state still open |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings; Horizontal landscape sibling | Recapture stills when screenshot works |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v60 leftovers

See `docs/qa/v59_leftovers.md`.
