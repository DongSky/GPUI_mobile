# Desktop capture v23

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v23 commit after `15b64b9` (v22).  
**Official:** [m3.material.io](https://m3.material.io/components/lists/specs) Expressive lists (December 2025) + [tooltips](https://m3.material.io/components/tooltips/specs).

v23 started from v22 leftovers (`docs/qa/v22_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs the live Expressive site was leftover **#9** (tooltip caret) plus a **December 2025 lists update** the backlog had not named: baseline 0-corner lists are **not recommended**; use segmented lists (2dp gap, 4dp inner / 16dp outer, selected 16dp + secondary-container). Licensed stills, live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `list.rs` | Segmented tokens from Compose `ListTokens` 29.0.0: `SEGMENTED_GAP` 2, inner extra-small 4 / outer large 16, selected large 16 + secondary-container / on-secondary-container; Wi-Fi / Bluetooth / Airplane hero |
| `tooltip.rs` | 16×8 caret geometry (`caret_down_points` / `caret_up_points`); HTML CSS triangle + GPUI filled polygon |
| Catalog | `data-hero="list"` segmented group (click morphs selection); `data-tooltip-caret` plain + rich |
| Hosts | `material_desktop_demo` + `component_demo` paint the segmented settings list and tooltip carets |
| Inventory | List notes expressive segmented; Tooltip notes caret |
| Tests | Segmented corners/gap/selected colors; catalog evidence for Wi-Fi hero + both carets |

Host tests (this VM): **28** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture lists + tooltip caret against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Segmented list | 2dp gap, 16/4 corners, selected 16 + secondary-container, trailing switches, 20dp leading | Click-select, not drag-reorder / swipe; slots are icon+text+switch only |
| Baseline list | 56/72/88 still shown as token row | Correctly not the hero |
| Plain tooltip | Inverse 24dp extra-small + downward caret + 4dp gap to tonal + | Persistent catalog hero, not hover / long-press |
| Rich tooltip | Surface-container medium elev 2 + upward caret + two actions | Max width shown as 280/320 card; no hover |
| Photos | Unchanged JPEG decode path | Still procedural, not licensed camera stills |

## v24 leftovers

See `docs/qa/v23_leftovers.md`.
