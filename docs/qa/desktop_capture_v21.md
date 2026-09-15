# Desktop capture v21

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v21 commit after `8df6e95` (v20).  
**Official:** [m3.material.io/develop/flutter/overview](https://m3.material.io/develop/flutter/overview) Expressive.

v21 started from v20 leftovers (`docs/qa/v20_leftovers.md` as written on that landing). Highest-impact **implementable** gap was leftover **#9**: collapsing app bars / nav drawer. Critique vs current Expressive: navigation drawers are **no longer recommended** (use the expanded rail). Side sheets remain. Implemented medium/large **flexible** collapsing app bars + search app bar + standard/modal/detached side sheets.

## What shipped

| Surface | Change |
|---|---|
| `top_app_bar.rs` | Small 64 + medium flexible 112/136 + large flexible 120/152 compress to 64; title/subtitle type lerp; scrolled `surface-container` elev 2; search 56-in-64 |
| Catalog | Bloom album phone (`data-appbar-scene`, click cycles collapse); medium “Saved / 12 albums”; search bar; collapsed token row |
| `side_sheet.rs` | Standard surface elev 0; modal `surface-container-low` elev 1 + 16dp start corners + 32% scrim; detached 16dp margin |
| Side sheet hero | Filters pane (Date / People / Places + Apply) over Bloom/Egret/Lake/Grove grid |
| Hosts | `material_desktop_demo` + `component_demo` paint collapsing Bloom bar + modal Filters sheet |
| Inventory | Top app bar notes flexible variants; Side sheet added; drawer explicitly not added |
| Tests | Flexible heights + collapse lerp; side-sheet corners/scrim; catalog evidence for `data-appbar`, Bloom subtitle, Filters |

Host tests (this VM): **28** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Large flexible | 152dp + subtitle, displaySmall → titleLarge compress, scrolled container/elev 2, Bloom JPEG | Procedural flower, not the camera still; collapse is click-stepped not nested-scroll |
| Medium flexible | 136dp + “Saved / 12 albums”, headlineMedium | Token row, not a second phone scene |
| Search app bar | 56dp contained field in 64dp bar, bodyLarge hint | Not the full search-app-bar trailing-actions set |
| Side sheet | Modal 256, start 16, elev 1, Filters + Apply over photo grid | Overlay is catalog-absolute, not a drag-to-dismiss sheet |
| Nav drawer | Correctly omitted | Use expanded rail (already shipped) |

## v22 leftovers

See `docs/qa/v21_leftovers.md`. Licensed stills, live IME/caret, APK, recapture, tooltip / banner.
