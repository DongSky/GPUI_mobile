# Desktop capture v36

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v36 commit after `c097122` (v35).  
**Official:** [m3.material.io chips](https://m3.material.io/components/chips/specs) Expressive `ChipShapes` + Compose `InputChip` avatar / `rememberAnimatedShape`.

v36 started from v35 leftovers (`docs/qa/v35_leftovers.md` as written on that landing). Highest-impact **implementable** gaps vs current Expressive were catalog chips snapping `ChipShapes` (no press interpolation), InputChip avatar unpainted, and overflow/split typeahead needing cascade focus. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `chip.rs` | `animated_corner_dp` / `press_t` spatial-fast lerp rest/selected → CornerSmall 8 (`rememberAnimatedShape`). `AVATAR_DP` 24, `AVATAR_PAD_START_DP` 4, compact 4dp gap when avatar. `INPUT_AVATAR_HERO` Sofia + licensed portrait |
| Catalog | Morph chips: `--press-r`, `:active` + click toggle rest↔selected (CSS spatial-fast). InputChip avatar row. Overflow/split `focusTypeahead` + `data-typeahead-autofocus` |
| Hosts | Desktop + Android paint avatar + compact gap; live mouse-down press; `FocusHandle` autofocus on overflow / split / overlay open |
| Inventory | Chip notes interpolation + avatar; menu notes typeahead autofocus |
| Tests | Avatar pads/gap; mid-press lerp; autofocus kind; catalog `rememberAnimatedShape` / `data-chip-avatar` / `focusTypeahead` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture InputChip avatar + live chip press against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Filter morph | Elevator 12 / Washer 16 / Pets 8 + live press interpolation | GPUI press is discrete Pressed; catalog CSS interpolates |
| Input + avatar | Portland place chips + Sofia 24dp still, 4dp compact | Compact 4dp is avatar arrangement (not a global chip row token) |
| Typeahead | Overflow / split / overlay autofocus the cascade | In-page submenu cascade still click-to-focus |
| Wide rail | Top vs Start pair | Live Top→Start interpolation still a swap |

## v37 leftovers

See `docs/qa/v36_leftovers.md`.
