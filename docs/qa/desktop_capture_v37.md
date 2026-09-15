# Desktop capture v37

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v37 commit after `6a2d855` (v36).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `WideNavigationRailItem` `iconPosition` + [chips](https://m3.material.io/components/chips/specs) `rememberAnimatedShape`.

v37 started from v36 leftovers (`docs/qa/v36_leftovers.md` as written on that landing). Highest-impact **implementable** gaps vs current Expressive were the modal wide rail swapping Top↔Start without a layout animation, and GPUI chip press snapping `press_t` 0/1. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `item_morph` / `icon_position_eased` / `morph_width_eased`: spatial-fast lerp of dest box, 56×32→24 icon, label-medium→large, indicator crossfade (Compose `iconPosition` follows `railExpanded`) |
| Catalog | Modal rail `data-icon-morph` + `applyRailIconMorph` rAF (same clock as GPUI). Static 96 Top / 220 Start pair unchanged |
| Hosts | Desktop + Android dests `with_animation` Top→Start; chip `with_animation` press clock (`press_t_anim` / `press_ms`) |
| Inventory | Rail notes live iconPosition lerp; chip notes GPUI with_animation press |
| Tests | Morph endpoints + interior samples; catalog `applyRailIconMorph`; press clock; desktop mid-lerp |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture live Top→Start rail morph + GPUI chip press against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Modal rail | 80↔220 + 32% scrim, FAB toggle, live Top→Start iconPosition lerp | Narrow 80 still used by the modal (pair stays 96); OS `PopUp` unopened |
| Wide pair | Static 96 Top + 220 Start | Non-modal expanded (no scrim, in-flow) is still the pair only |
| Filter press | Catalog CSS + GPUI `with_animation` spatial-fast to CornerSmall 8 | First paint stays discrete until a press clock starts |
| Typeahead | Overflow / split / overlay autofocus | In-page submenu cascade still click-to-focus |

## v38 leftovers

See `docs/qa/v37_leftovers.md`.
