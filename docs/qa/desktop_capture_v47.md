# Desktop capture v47

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v47 commit after `6fbfc81` (v46).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `hideOnCollapse` overlay with always-extended `ExtendedFloatingActionButton` Create (~188). Menu ☰ remains the show/hide control; destinations stay Start.

v47 started from v46 leftovers (`docs/qa/v46_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was the hide-on-collapse modal FAB slot: hosts and the HTML catalog still used a +/← collapse toggle instead of Compose always-extended Create on the sliding overlay (rail stays 220 / Start / CornerLarge / SurfaceContainer while offscreen). Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `HIDE_DEMO_HAS_EXTENDED_FAB` + `fab_morph_hide` (always t=1 Create ~188) |
| Catalog | `data-hide-fab-extend`; hide overlay FAB always-extended (16dp inset, label visible); `applyRailHideSlide` keeps FAB at expanded morph |
| Hosts | Desktop + Android hide-on-collapse Create (~188); Menu ☰ / scrim / FAB click still show/hide |
| Inventory | Rail notes hideOnCollapse always-extended Create (~188) |
| Tests | Hide extend attr; demo flag; `fab_morph_hide` 188 / Create; desktop Create 188 |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture hide-on-collapse always-extended Create against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface, ContentPadding 44, Extended FAB Create 56↔188 | OS `PopUp` unopened (n/a for standard) |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface, ContentPadding 44 | Same OS `PopUp` gap |
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer, ContentPadding 44, header-less Extended FAB Create 56↔188 | OS `PopUp` unopened; predictive-back scale skipped |
| Narrow modal | Live 80↔220 + same modal container + ContentPadding 44 + Extended FAB Create (12dp collapsed inset) | Same OS `PopUp` + predictive-back gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape, ContentPadding 44, always-extended Create ~188 | OS `PopUp` unopened; predictive-back scale skipped |

## v48 leftovers

See `docs/qa/v47_leftovers.md`.
