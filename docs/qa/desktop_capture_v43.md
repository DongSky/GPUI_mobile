# Desktop capture v43

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v43 commit after `80c0cd3` (v42).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `ModalWideNavigationRail` container.

v43 started from v42 leftovers (`docs/qa/v42_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the missing modal container: Compose `WideNavigationRailDefaults.modalExpandedShape` (`NavigationRailExpandedTokens.ModalContainerShape` CornerLarge 16) and `ModalContainerColor` SurfaceContainer, with a spatial-fast lerp from collapsed CornerNone / Surface. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `RailContainerMorph`; modal CornerLarge 16 + SurfaceContainer; standard stays CornerNone / Surface; hide overlay keeps expanded modal chrome |
| Catalog | `applyRailContainerMorph` / `hexMix`; modal/narrow/hide heroes use `data-modal-expanded-shape="CornerLarge"` |
| Hosts | Desktop + Android live modal/narrow rails lerp corner + container; hide rail paints expanded modal shape |
| Inventory | Rail notes modalExpandedShape CornerLarge 16 + ModalContainerColor SurfaceContainer |
| Tests | Shape/color helpers; catalog morph attrs; desktop expanded modal SurfaceContainer |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture modal CornerLarge 16 / SurfaceContainer against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer when expanded | OS `PopUp` unopened |
| Narrow modal | Live 80↔220 + same modal container morph | Same OS `PopUp` gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape | Same OS `PopUp` gap |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface | Same OS `PopUp` gap |
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface | FAB slot stays +/← toggle (not Extended) |

## v44 leftovers

See `docs/qa/v43_leftovers.md`.
