# Desktop capture v22

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v22 commit after `445caad` (v21).  
**Official:** [m3.material.io/develop/flutter/overview](https://m3.material.io/develop/flutter/overview) Expressive.

v22 started from v21 leftovers (`docs/qa/v21_leftovers.md` as written on that landing). Highest-impact **implementable** gaps were leftover **#9** (tooltip / banner) plus the Expressive nav-bar update called out on [m3.material.io navigation-bar specs](https://m3.material.io/components/navigation-bar/specs). Licensed stills, live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **banners are not on the live component list** (`/components/banners/specs` 404s) — do not add them as missing inventory. The baseline **80dp** nav bar is **not recommended**; use the 64dp flexible / short bar.

## What shipped

| Surface | Change |
|---|---|
| `tooltip.rs` | Plain inverse-surface 24dp extra-small + bodySmall (`Add to library` on a tonal +); rich surface-container medium elev 2 + titleSmall / bodyMedium + Learn more / Dismiss |
| `navigation_bar.rs` | Flexible 64dp (`NavigationBarTokens.ContainerHeight` v0_11_0); compact vertical 56×32 + secondary label; medium horizontal 40dp pill + on-secondary-container label; baseline 80dp kept as `TALL_HEIGHT_DP` only |
| Catalog | `data-hero="tooltip"`; `data-navbar` vertical + horizontal; Gmail peek inbox nav is 64dp / 56×32 |
| Hosts | `material_desktop_demo` + `component_demo` paint both tooltip variants and both nav layouts |
| Inventory | Tooltip added; Navigation bar notes flexible tokens. Banner / nav drawer stay out |
| Tests | Plain/rich tokens; flexible vs tall height; catalog evidence for tooltip + both nav layouts |

Host tests (this VM): **28** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture tooltip + flexible nav against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Plain tooltip | Inverse surface, 24dp, extra-small, bodySmall, 4dp above tonal + | Always visible (catalog hero), not hover / long-press |
| Rich tooltip | Surface-container, medium 12, elev 2, subhead + supporting + two actions | No caret triangle; max width shown as 320dp card |
| Flexible nav (compact) | 64dp, 56×32 indicator, secondary active label, Home / Search / Profile | Inbox peek still uses glyph fallbacks, not filled dest SVGs on the token strip |
| Flexible nav (medium) | 64dp, 40dp horizontal pills, 16dp pad, 4 dests centered | Arrangement is static catalog, not window-class adaptive |
| Banner | Correctly omitted | Not on current Expressive site |
| Photos | Unchanged JPEG decode path | Still procedural, not licensed camera stills |

## v23 leftovers

See `docs/qa/v22_leftovers.md`.
