# Desktop capture v40

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v40 commit after `ac3f7d2` (v39).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `ModalWideNavigationRail.hideOnCollapse` + `Arrangement.Vertical`.

v40 started from v39 leftovers (`docs/qa/v39_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the missing hide-when-collapsed arrangement (in-flow/modal rails stayed on-screen when collapsed). Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `hideOnCollapse` slide (`hide_slide_offset_*`, 0↔220); items stay Start; `RailArrangement` Top/Center |
| Catalog | `data-hero="wide-rail-hide"` Menu ☰ + Inbox; rail `translateX` offscreen; `Arrangement.Center`; `applyRailHideSlide` |
| Hosts | Desktop + Android live dismissible modal (Menu toggle, scrim dismiss, Start pills, centered dests) |
| Inventory | Rail notes hideOnCollapse slide + Arrangement.Center |
| Tests | Hide helpers; catalog hide attrs; desktop Start-when-hidden + center |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture hideOnCollapse slide + Arrangement.Center against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, 32% scrim | OS `PopUp` unopened |
| Narrow modal | Live 80↔220 + 32% scrim | Same OS `PopUp` gap |
| Modal rail | 96↔220 + 32% scrim | Same OS `PopUp` gap |
| In-flow wide | 96↔220, no scrim, Inbox body | Arrangement stays Top (Compose default) |

## v41 leftovers

See `docs/qa/v40_leftovers.md`.
