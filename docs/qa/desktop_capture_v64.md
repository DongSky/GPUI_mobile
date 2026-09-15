# Desktop capture v64

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v64 after `07702ea` (v63 no-results empty state).  
**Official:** [m3.material.io search guidelines](https://m3.material.io/components/search/guidelines) — “To help people find information quickly, consider adding … Filter chips to narrow down results.”

v64 started from v63 leftovers (`docs/qa/v63_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **search result filter chips**. Do not reopen no-results empty, leading 40/20, trailing clear-X, two-line supporting/open, segmented rows, TimePickerLayoutType Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual tokens. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

This agent originally targeted the v60 leftover (40/20 leading) as v61; that already landed as **v62**. Continued from current `main` as **v64**.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `SearchFilter` All/Apps/Shortcuts/Settings; `FILTER_ROW_H_DP` 48; `filter_*_in`; Settings + `app` is empty |
| Catalog | `.sv-filters` FilterChips; `data-search-filter` / `data-search-filter-chip`; `data-hero="search-filters"` |
| Hosts | Desktop + Android paint chip row when queried; click sets `search_filter` |
| Inventory | Search notes filter chips |
| Tests | Filter match, height includes 48dp, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture search filter chips against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen; medium docked + 32% scrim; Quick results / Results; groups; segmented 2/4/16; two-line 72dp; trailing clear-X; 40/20 leading; no-results; filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings; Horizontal landscape sibling | — |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v65 leftovers

See `docs/qa/v64_leftovers.md`.
