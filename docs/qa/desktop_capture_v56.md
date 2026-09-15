# Desktop capture v56

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v56 commit after `6a8b361` (v55 24-hour ClockFace).  
**Official:** [m3.material.io search guidelines](https://m3.material.io/components/search/guidelines) — “For accessibility, focused search needs a clear status indicator that it’s searching content, like a search icon or Results label… Show search results in a compact, organized list, with an indicator like Quick results… When search results are queried, the input text should remain visible, but not in focus.”

v56 started from v55 leftovers (`docs/qa/v55_leftovers.md` as written on that landing; a parallel agent had already taken v55 for ClockFace). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): official search **Results / Quick results** status indicator + live region. Do not reopen 24-hour ClockFace. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `SearchListStatus` Suggestions / QuickResults / Results; 32dp status row; `list_status` / `status_live_text` / `expanded_list_h_dp`; enter unfocuses (query stays) |
| Catalog | `data-search-status` + `aria-live` status row; queried compact siblings (`app` Quick results / `App` Results); JS sync on type / enter / blur / pick |
| Hosts | Desktop + Android paint status row; hide group titles when queried; pick/enter leave field unfocused |
| Inventory | Search notes Quick results + Results |
| Tests | Status tokens, live text, catalog attrs, enter unfocus |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture queried search (Quick results / Results) against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen < 600dp; medium docked Corner 28 + 24→12; Recent / Suggestions groups; Quick results while typing; Results after submit (query visible, not focused) | `cx.transform` search morph skipped; docked 32% scrim and in-list segmented items still open |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings | Horizontal TimePickerLayoutType unpainted |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v57 leftovers

See `docs/qa/v56_leftovers.md`.
