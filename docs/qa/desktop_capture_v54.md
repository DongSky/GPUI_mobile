# Desktop capture v54

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v54 commit after `a18679e` (v53).  
**Official:** [m3.material.io search guidelines](https://m3.material.io/components/search/guidelines) — “Use gaps to separate a list of suggestions or results into groups.”

v54 started from v53 leftovers (`docs/qa/v53_leftovers.md` as written on that landing). Compact→docked search ships; leftover #2 said remaining implementable inventory is thin. Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): Expressive search **suggestion groups** with gaps (Recent / Suggestions). Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `SuggestionGroup` Recent / Suggestions; 8dp group gap + 32dp titles; `filter_grouped_suggestions` |
| Catalog | `data-search-group` blocks; group titles; JS hides empty groups while typing |
| Hosts | Desktop + Android render grouped suggestion lists |
| Inventory | Search notes 8dp group gaps |
| Tests | Group tokens, filter, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture grouped search against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen < 600dp; medium docked Corner 28 + 24→12; Recent / Suggestions groups with 8dp gaps | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23 | Dial stays 12-hour only |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v55 leftovers

See `docs/qa/v54_leftovers.md`.
