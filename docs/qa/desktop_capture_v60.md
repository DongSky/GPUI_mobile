# Desktop capture v60

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v60 after `3efb491` (v59 segmented search result rows).  
**Official:** [m3.material.io search specs](https://m3.material.io/components/search/specs) + [lists specs](https://m3.material.io/components/lists/specs) — queried search results are two-line list items (72dp, bodyLarge headline + bodyMedium supporting) with an optional trailing open affordance on submitted Results.

v60 started from v59 leftovers (`docs/qa/v59_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **two-line search result rows with supporting text and a trailing open affordance on Results**. Do not reopen segmented 2dp filled rows, TimePickerLayoutType Horizontal, ClockFace rings, TimeInput, docked search scrim, Results/Quick results status chrome, compact→docked, or contained search chrome. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `RESULT_H_DP` 72; `SUGGESTION_SUPPORTING` + `supporting_for`; `RESULT_OPEN` ↗; status helpers `uses_two_line_rows` / `shows_open_affordance`; queried list height uses 72dp |
| Catalog | Quick results / Results rows are `data-search-lines="two"` with supporting + Results `data-search-open` |
| Hosts | Desktop + Android queried rows paint headline / supporting / trailing open |
| Inventory | Search notes two-line queried rows + open affordance |
| Tests | Height, supporting copy, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture queried two-line search rows against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen; medium docked + 32% scrim; Quick results / Results; segmented 2dp filled rows; two-line queried + Results open | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings; Horizontal landscape sibling | — |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v61 leftovers

See `docs/qa/v60_leftovers.md`.
