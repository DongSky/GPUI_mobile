# Desktop capture v24

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v24 commit after `6379268` (v23).  
**Official:** [m3.material.io button groups](https://m3.material.io/components/button-groups/specs) + [lists](https://m3.material.io/components/lists/specs) + [tooltips](https://m3.material.io/components/tooltips/specs).

v24 started from v23 leftovers (`docs/qa/v23_leftovers.md` as written on that landing). Highest-impact **implementable** gaps were leftover **#9** (list swipe / tooltip hover) plus a **standard button group** variant the backlog had not named: connected groups shipped, but Expressive also requires the 12dp standard group with `ExpandedRatio` 0.15 neighbor morph. Licensed stills, live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `button_group.rs` | Standard tokens from Compose `ButtonGroupSmallTokens` / `ButtonGroupDefaults`: 12dp gap, ExpandedRatio 0.15, Start/Center/End tonal round → filled square; neighbor widths sum to `count * base` |
| `list.rs` | Swipe-to-reveal Archive (primary) / Delete (error) 80dp rails + 56dp threshold; segmented reorder list with 24dp drag handle |
| `tooltip.rs` | Hover trigger + 500ms long-press; catalog CSS `:hover` / `[data-open]` |
| Catalog | `data-hero="button-group-standard"`; `data-hero="list-swipe"`; `data-hero="list-reorder"`; `data-tooltip-trigger="hover"` |
| Hosts | `material_desktop_demo` + `component_demo` paint standard groups, swipe rails, reorder handles, click-to-toggle tooltips |
| Inventory | Button group notes standard+connected; List notes swipe/reorder; Tooltip notes hover/long-press |
| Tests | Standard widths/corners; swipe settle; reorder permutation; catalog evidence |

Host tests (this VM): **28** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture standard groups + list swipe/reorder + tooltip hover against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Standard button group | 12dp gap, 15% expand, neighbors compress, tonal→filled square, Start/Center/End | Catalog click-select, not pointer-down-only press; no overflow on the standard row |
| Connected group | Unchanged 2dp / 8dp inner | Still the Day/Week/Month + icon overflow heroes |
| List swipe | Archive primary / Delete error 80dp, Team sync notes offset | Scroll-wheel / pointer analog, not nested LazyColumn fling |
| List reorder | Segmented 2dp + 24dp handle, Morning briefing hero | Click-handle moves up, not live drag |
| Tooltip | Hover + 500ms long-press; caret still 16×8 | GPUI uses click-toggle (no hover API); HTML hover hides until pointer |
| Photos | Unchanged JPEG decode path | Still procedural, not licensed camera stills |

## v25 leftovers

See `docs/qa/v24_leftovers.md`.
