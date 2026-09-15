# Desktop capture v27

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v27 commit after `1071c5d` (v26).  
**Official:** [m3.material.io button groups](https://m3.material.io/components/button-groups/specs) + catalog photos vs [m3.material.io](https://m3.material.io) overviews.

v27 started from v26 leftovers (`docs/qa/v26_leftovers.md` as written on that landing). Highest-impact **implementable** gaps were leftover **#1** (licensed camera stills) and leftover **#9** (standard button-group overflow). Live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `assets/photos/*` | 19 small licensed stills (Commons CC0/BY/BY-SA/PD + Unsplash License) with `README.md` credits |
| `photo_stub.rs` | `include_bytes!` camera JPEGs; `PhotoCredit` license/source; sampled fill colors; procedural painter removed |
| `button_group.rs` | Standard trailing `resolve_standard_overflow()` = Compose `OverflowIndicator` filled icon button; Left/Right/Justify menu |
| Catalog | `data-licensed-camera`; `data-standard-overflow` + overflow menu on the 12dp standard row |
| Hosts | `material_desktop_demo` + `component_demo` paint standard overflow + toggle |
| Inventory | Button group notes OverflowIndicator; scene notes say licensed stills |
| Tests | License credits; SOI on every kind; overflow colors/corners; catalog evidence |

Host tests (this VM): **30** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture licensed stills + standard overflow against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Licensed stills | Real camera JPEGs for Bloom/Egret/basket/mugs/dog/party/landscapes/portraits; decode + mosaic unchanged | 128–240px recrops, not print-size official kit photos; Unsplash portraits are license-ok stand-ins, not the Material kit faces |
| Standard overflow | Trailing filled S icon button, 12dp gap, menu Left/Right/Justify; overflow excluded from ExpandedRatio | Catalog is always-open anatomy, not breakpoint collapse of Start/Center/End themselves |
| Connected overflow | Unchanged Cut/Copy/Paste icon row | Unchanged |
| Photos on GPUI | Mosaic samples the new stills | No `img` element; mosaic grid is still the host paint path |
| Toggle / width | Unchanged v25–v26 | Unchanged |

## v28 leftovers

See `docs/qa/v27_leftovers.md`.
