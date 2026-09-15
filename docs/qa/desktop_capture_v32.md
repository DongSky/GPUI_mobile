# Desktop capture v32

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v32 commit after `448e4c1` (v31).  
**Official:** [m3.material.io menus](https://m3.material.io/components/menus/specs) + Compose `MenuAnchorPosition.End` flyout from a grouped overlay (not a single surface).

v32 started from v31 leftovers (`docs/qa/v31_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was GPUI overlay menus remaining a single grouped surface (click More dismissed). Live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `menu.rs` | `OverlayMenuSession`: overlay starts grouped (`OVERLAY_FLYOUT_OPEN` false); More hover/click opens End flyout (Stay, does not dismiss); leaf Commit; Escape Dismiss; ArrowRight/Left; WAI-ARIA typeahead on parent or flyout |
| Catalog | `data-hero="menu-overlay"` scrim + grouped parent (`data-open="0"`); More mouseenter/click opens 24dp flyout; typeahead/keyboard |
| Hosts | Android + desktop `Overlay::Menu` paint parent + flyout; live cascade hover-open + typeahead; desktop Menu button |
| Inventory | Overlay starts grouped, More opens End flyout + live typeahead |
| Tests | Overlay session: More Stay, `s` Share→Save, leaf Commit; catalog overlay evidence; desktop 24 / Stay |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive overlay flyout against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Overlay default | Grouped 16/8 surfaces, flyout hidden | Official dropdowns are unscrimmed popups near an anchor |
| More | Hover/click opens End 24dp flyout; parent morphs to Inactive 8 | No hover delay; OS `PopUp` unused |
| Typeahead | `s` cycles Share→Save→Sort while flyout open | GPUI needs focus (`tab_index`) before keys |
| Leaf | Commit dismisses overlay | — |

## v33 leftovers

See `docs/qa/v32_leftovers.md`.
