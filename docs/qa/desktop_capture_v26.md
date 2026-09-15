# Desktop capture v26

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v26 commit after `1018320` (v25).  
**Official:** [m3.material.io icon buttons](https://m3.material.io/components/icon-buttons/specs).

v26 started from v25 leftovers (`docs/qa/v25_leftovers.md` as written on that landing). Highest-impact **implementable** gap was leftover **#9** (toggle icon buttons). Licensed stills, live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**. Standard button-group overflow remains the next component-level gap.

## What shipped

| Surface | Change |
|---|---|
| `icon_button.rs` | `IconButtonSelection::{Default,Unselected,Selected}` from m3.material.io color table + Compose `IconToggleButton`; round↔square rest morph |
| Catalog | `data-hero="icon-buttons-toggle"`; filled/tonal/outlined/standard unselected+selected; square-rest filled inverse morph |
| Hosts | `material_desktop_demo` + `component_demo` paint both toggle rows from `resolve_toggle()` / `resolve_selection()` |
| Inventory | Icon button notes toggle selected colors + round↔square |
| Tests | Filled/tonal/outlined/standard selected colors; 20↔12 rest corners; pressed 8dp; catalog evidence; desktop 12dp selected |

Host tests (this VM): **30** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture icon-button toggle against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Toggle colors | Filled unselected surface-container / selected primary; tonal secondary; outlined inverse-surface; standard primary icon | Catalog is static anatomy, not live optical-center press |
| Toggle shape | Rest round→square selected; square rest→round selected; press still 8dp S | No XS/L/XL toggle hero rows |
| Glyph | Unselected ☆ / selected ★ | Not a vector icon font pair (outlined vs filled Material Symbols) |
| Width axis | Unchanged S 32/40/52 + M 48/56/72 | Unchanged |
| Photos | Unchanged JPEG decode path | Still procedural, not licensed camera stills |
| Standard group | Unchanged 12dp / 0.15 | Still no overflow on the standard row |

## v27 leftovers

See `docs/qa/v26_leftovers.md`.
