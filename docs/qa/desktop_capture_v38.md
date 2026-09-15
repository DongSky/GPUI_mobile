# Desktop capture v38

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v38 commit after `505661f` (v37).  
**Official:** [m3.material.io navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive collapsed/expanded layout (standard vs modal).

v38 started from v37 leftovers (`docs/qa/v37_leftovers.md` as written on that landing). Highest-impact **implementable** gaps vs current Expressive were the static-only in-flow wide pair and modal collapsed width 80. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `navigation_rail.rs` | `RailExpandedLayout` Standard (in-flow, no scrim, elev 0) vs Modal; Wide collapsed default 96; `morph_width_dp` 96↔220; `morph_width_narrow_dp` keeps 80; `is_modal_for` / `scrim_opacity_for` / `elevation_dp_for` |
| Catalog | `data-hero="wide-rail-inflow"` 96↔220 no scrim + Inbox body; modal `data-collapsed-width=96`; default `.nav-rail` 96; JS skips scrim/overlay for `data-rail-layout="standard"` |
| Hosts | Desktop + Android in-flow column (FAB toggle, Inbox shifts); modal overlay collapsed 96 |
| Inventory | Notes in-flow standard + modal 96 |
| Tests | Standard vs modal helpers; morph 96 endpoints; catalog inflow attrs; desktop mid-lerp + layout |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture live in-flow 96↔220 + modal 96 against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-flow wide | 96↔220, no scrim, Inbox body in row, live Top→Start | OS `PopUp` unopened; no hide-when-collapsed arrangement |
| Modal rail | 96↔220 + 32% scrim, FAB toggle, live Top→Start | Narrow 80 is token-only; OS `PopUp` unopened |
| Wide pair | Static 96 Top + 220 Start | Settled-state reference only |
| Typeahead | Overflow / split / overlay autofocus | In-page submenu cascade still click-to-focus |

## v39 leftovers

See `docs/qa/v38_leftovers.md`.
