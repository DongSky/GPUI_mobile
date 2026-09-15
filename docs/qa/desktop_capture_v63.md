# Desktop capture v63

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v63 commit after `d706dc4` (v62 leading 40/20).  
**Official:** [m3.material.io search guidelines](https://m3.material.io/components/search/guidelines) — queried search can show no matches; the status stays Results / Quick results and the list is empty.

v63 started from v62 leftovers (`docs/qa/v62_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **search no-results catalog sibling**. Hosts already painted `EMPTY_SUGGESTIONS`; catalog now has a submitted-empty hero. Do not reopen leading 40/20, trailing clear-X, two-line supporting/open, segmented rows, TimePickerLayoutType Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual tokens. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `DEMO_EMPTY_QUERY` `zzz`, `shows_empty`, `EMPTY_H_DP` 56, `empty_content` on-surface-variant |
| Catalog | `data-hero="search-empty"` sibling; `data-search-empty`; live “0 results”; JS toggles empty on type |
| Hosts | Empty row uses `EMPTY_H_DP` + `empty_content` |
| Inventory | Search notes no-results empty state |
| Tests | Empty query, live text, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture the no-results sibling against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen; medium docked + 32% scrim; Quick results / Results; groups; segmented 2/4/16; two-line 72dp; trailing clear-X; 40/20 leading; no-results sibling | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings; Horizontal landscape sibling | Recapture stills when screenshot works |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v64 leftovers

See `docs/qa/v63_leftovers.md`.
