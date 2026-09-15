# Desktop capture v28

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v28 commit after `13adb31` (v27).  
**Official:** [m3.material.io menus](https://m3.material.io/components/menus/specs) + [I/O 2026 Expressive lists & menus](https://m3.material.io/blog/whats-new-at-io26).

v28 started from v27 leftovers (`docs/qa/v27_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was leftover menus still on the baseline 4dp / 48dp / secondary-container shell. Live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `menu.rs` | Compose 24.1.2 `SegmentedMenuTokens` + `StandardMenuTokens` + `VibrantMenuTokens`: 44dp items, corner-large 16 container, elev 2, grouped 2dp / 4dp pad, standard surface-container-low + tertiary-container selected, vibrant tertiary-container + tertiary selected, horizontal 2dp pills (selected full-round) + 52dp icon-only |
| Catalog | `data-hero="menu"` standard + vibrant grouped (Italic/Bold/Underline, Cut/Copy/Paste, More ›); `data-hero="menu-horizontal"` Day/Week/Month/Year; `data-hero="menu-icons"` B/I/U |
| Hosts | `material_desktop_demo` + `component_demo` paint both schemes + both horizontal rows from `resolve_*()`; overlay uses grouped standard |
| Inventory | Menu notes Expressive vertical + horizontal |
| Tests | Token colors/corners/gaps; catalog evidence; desktop 16dp / 44dp / full-round Week |

Host tests (this VM): **31** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive menus against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Vertical standard | surface-container-low groups, 16/8 outer, 44dp items, Bold = tertiary-container + 12dp | Catalog is static anatomy, not hover-open / keyboard typeahead |
| Vertical vibrant | tertiary-container groups, Bold = tertiary / on-tertiary | Use sparingly; no live scheme toggle |
| Grouped gap | 2dp `SegmentedGap` + 4dp `GroupPadding` | Submenu does not fly out to a second surface |
| Horizontal | 2dp pills; selected `CornerFull`; icon-only 52 / 4dp gap | No live press→medium morph |
| Overflow / split | Inherit new standard shell (16dp / 44dp / tertiary selected) | Still a single surface, not grouped |

## v29 leftovers

See `docs/qa/v28_leftovers.md`.
