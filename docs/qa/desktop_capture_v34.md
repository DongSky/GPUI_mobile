# Desktop capture v34

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v34 commit after `53f66e8` (v33).  
**Official:** [m3.material.io menus](https://m3.material.io/components/menus/specs) grouped vertical + [button groups](https://m3.material.io/components/button-groups/specs) OverflowIndicator + [split buttons](https://m3.material.io/components/split-button/specs) trailing menu.

v34 started from v33 leftovers (`docs/qa/v33_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was overflow/split still a single 16dp shell. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `menu.rs` | `GroupedPopupKind` trees for overlay / standard overflow / connected overflow / split. `OverlayMenuSession` drives each tree (2dp groups + More › Share/Save/Sort). `HOVER_OPEN_DELAY_MS` 200 |
| Catalog | Overflow + split paint `data-overflow-cascade` grouped stacks (`data-popup-kind`, `data-grouped="1"`) instead of one 16dp `.menu`. Catalog JS waits 200ms before hover-open; click/keyboard stay immediate |
| Hosts | Android + desktop overflow/split open the live cascade (`tab_index` + typeahead). Leaf Commit dismisses the popup; More Stay + End flyout |
| Inventory | Button group / split / menu notes grouped overflow/split + 200ms hover |
| Tests | Overflow session Left→More Stay + `s` Share→Save; catalog `data-overflow-cascade` / popup-kind / hover-delay; desktop 2-group overflow |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture grouped overflow/split against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Standard overflow | Left / Right / Justify group + More, 16/8 corners, 2dp gap | GPUI hover-open has no 200ms delay |
| Connected overflow | Cut / Copy / Paste + More › Share/Save/Sort | Same hover-delay gap |
| Split trailing | Add to cart / Save for later + More | Catalog variants share one open anatomy; GPUI typeahead needs focus |
| Overlay / cascade | Unscrimmed + 200ms catalog hover delay | Hosts still open on immediate hover |

## v35 leftovers

See `docs/qa/v34_leftovers.md`.
