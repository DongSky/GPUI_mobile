# Desktop capture v61

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v61 commit after `522bddf` (v60 two-line search results).  
**Official:** [m3.material.io search guidelines](https://m3.material.io/components/search/guidelines) — “Focused search can show an optional clear icon to remove input text.” AndroidX SearchView shows/hides a clear-text button when the query is non-empty.

v61 started from v60 leftovers (`docs/qa/v60_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **expanded-search trailing clear-X**. Do not reopen two-line queried rows, segmented search rows, TimePickerLayoutType Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual tokens. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `TRAILING_CLEAR` ✕, `shows_clear` / `trailing_action` / `apply_clear` (empty + refocus) |
| Catalog | Trailing slot `data-search-trailing` + `data-search-clear`; queried heroes show ✕; click empties + focus |
| Hosts | Desktop + Android: mic when empty, ✕ when queried; click calls `apply_clear` |
| Inventory | Search notes trailing clear-X |
| Tests | `shows_clear`, trailing swap, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture trailing clear-X against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen; medium docked + 32% scrim; Quick results / Results; groups; segmented 2/4/16 rows; two-line queried rows; trailing clear-X | `cx.transform` search morph skipped; leading 40dp avatar / 20dp icon still open |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings; Horizontal landscape sibling | Recapture stills when screenshot works |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v62 leftovers

See `docs/qa/v61_leftovers.md`.
