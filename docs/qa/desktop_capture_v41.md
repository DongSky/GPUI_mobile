# Desktop capture v41

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v41 commit after `9810df9` (v40).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `WideNavigationRail` header + `Arrangement.Vertical`.

v41 started from v40 leftovers (`docs/qa/v40_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the missing Compose header slot (Menu / MenuOpen + plain tooltip Above) and `Arrangement.Bottom`. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `RailArrangement::Bottom`; header Menu/MenuOpen tokens; tooltip Above; `header_space_dp` (Top-only 40) |
| Catalog | `data-hero="wide-rail-header"` in-flow 96↔220; header slot + Bottom dests; hover tooltip |
| Hosts | Desktop + Android live header rail (Menu toggle, hover tooltip, Bottom dests, Inbox body) |
| Inventory | Rail notes header Menu/MenuOpen + tooltip Above + Arrangement.Bottom |
| Tests | Header helpers; catalog header attrs; desktop Bottom + MenuOpen |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture header Menu/MenuOpen + Arrangement.Bottom against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Header + Bottom | Menu/MenuOpen, tooltip Above, dests flex-end, Inbox shifts, no scrim | OS `PopUp` unopened |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, 32% scrim | Same OS `PopUp` gap |
| Narrow modal | Live 80↔220 + 32% scrim | Same OS `PopUp` gap |
| Modal rail | 96↔220 + 32% scrim | Same OS `PopUp` gap |
| In-flow wide | 96↔220, no scrim, Inbox body | Arrangement stays Top (Compose default) |

## v42 leftovers

See `docs/qa/v41_leftovers.md`.
