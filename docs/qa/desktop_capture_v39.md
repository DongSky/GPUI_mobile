# Desktop capture v39

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v39 commit after `b109755` (v38).  
**Official:** [m3.material.io menus](https://m3.material.io/components/menus/specs) typeahead + [navigation rail](https://m3.material.io/components/navigation-rail/specs) Expressive `NarrowContainerWidth` 80.

v39 started from v38 leftovers (`docs/qa/v38_leftovers.md` as written on that landing). Highest-impact **implementable** gaps vs current Expressive were in-page submenu typeahead still click-to-focus, and optional narrow 80 remaining token-only. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `menu.rs` | `typeahead_autofocus_in_page` — in-page submenu cascade autofocuses like overflow/split/overlay |
| `navigation_rail.rs` | `RailCollapsedKind` Wide 96 / Narrow 80; `resolve_mode_kind` + `morph_width_eased_kind`; live narrow modal 80↔220 |
| Catalog | `data-menu-cascade` paints `data-typeahead-autofocus` + load/hover `focusTypeahead`; `data-hero="wide-rail-narrow"` 80↔220 + 32% scrim |
| Hosts | Desktop + Android autofocus in-page cascade on first paint / hover; live narrow modal FAB toggle |
| Inventory | Menu notes in-page autofocus; rail notes live narrow 80↔220 |
| Tests | In-page autofocus helper; cascade catalog attrs; narrow kind/morph/hero |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture in-page typeahead + live narrow 80 against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-page submenu | Autofocus + hover `focusTypeahead`; Share/Save/Sort typeahead without a click | OS menu `PopUp` unopened |
| Narrow modal | Live 80↔220 + 32% scrim, FAB toggle, Top→Start | OS `PopUp` unopened |
| Modal rail | 96↔220 + 32% scrim | Same OS `PopUp` gap |
| In-flow wide | 96↔220, no scrim, Inbox body | No hide-when-collapsed arrangement |

## v40 leftovers

See `docs/qa/v39_leftovers.md`.
