# Desktop capture v31

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v31 commit after `882e800` (v30).  
**Official:** [m3.material.io menus](https://m3.material.io/components/menus/specs) + Compose `SegmentedMenuTokens` `ActiveContainerShape` 24 / `InactiveContainerShape` 8 + `MenuAnchorPosition.End`.

v31 started from v30 leftovers (`docs/qa/v30_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was More › still a trailing chevron with no nested surface, hover-open, or typeahead. Live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `menu.rs` | Submenu flyout: `ActiveContainerShape` 24 focused, `InactiveContainerShape` 8 unfocused parent, 4dp End gap. Share / Save / Sort nested items. WAI-ARIA typeahead (`typeahead_index` wraps on first letter) |
| Catalog | `data-hero="menu-submenu"` cascade (inactive grouped parent + 24dp flyout); hover-open / ArrowRight / typeahead `s` cycles Share→Save→Sort; More `aria-haspopup` |
| Hosts | `material_desktop_demo` + `component_demo` paint the cascade from `resolve_submenu` / `MenuFocus::Inactive` |
| Inventory | Menu notes submenu flyout + typeahead |
| Tests | 24/8 corners; typeahead wrap; catalog evidence; desktop 24 / Share |

Host tests (this VM): **34** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive submenu cascade against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Flyout placement | End of More row, 4dp gap, elev 2 | Catalog default-open anatomy; GPUI overlay still single-surface |
| Shape morph | Parent 8dp inactive / flyout 24dp active | Rest grouped 16/8 restored on mouseleave in HTML only |
| Hover-open | CSS/JS mouseenter on cascade | No hover delay; Android overlay does not hover-open |
| Typeahead | `s` cycles Share→Save→Sort; catalog keydown | GPUI hosts do not consume keyboard typeahead |
| Nested labels | Share / Save / Sort | Official overview copy may differ |

## v32 leftovers

See `docs/qa/v31_leftovers.md`.
