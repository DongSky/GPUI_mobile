# Desktop capture v25

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v25 commit after `33aacf9` (v24).  
**Official:** [m3.material.io icon buttons](https://m3.material.io/components/icon-buttons/specs).

v25 started from v24 leftovers (`docs/qa/v24_leftovers.md` as written on that landing). Highest-impact **implementable** gap was leftover **#9** (icon-button narrow/wide width axis). Licensed stills, live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**. Toggle (selected) icon buttons remain the next component-level gap.

## What shipped

| Surface | Change |
|---|---|
| `icon_button.rs` | `IconButtonWidth::{Narrow,Default,Wide}` from MDC `m3_comp_icon_button_*_{narrow,default,wide}_leading_space`; Compose `IconButtonWidthOption` |
| Catalog | `data-hero="icon-buttons-width"`; S 32/40/52 + M 48/56/72 filled round rows |
| Hosts | `material_desktop_demo` + `component_demo` paint both width rows from `resolve_width()` |
| Inventory | Icon button notes narrow/default/wide |
| Tests | All 15 size×width containers; catalog evidence; desktop 32/52/24 tokens |

Host tests (this VM): **29** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture icon-button widths against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Width axis | S 4/8/14 leading → 32/40/52; M 12/16/24 → 48/56/72; XS/L/XL tokenized; 48dp min target on XS/S | Catalog is static, not optical-center live; no XS/L/XL width hero rows |
| Default size/shape | Unchanged XS–XL square-when-uniform, round pill when wide | Toggle selected round↔square + selected colors still missing |
| Photos | Unchanged JPEG decode path | Still procedural, not licensed camera stills |
| Standard group | Unchanged 12dp / 0.15 | Still no overflow on the standard row |

## v26 leftovers

See `docs/qa/v25_leftovers.md`.
