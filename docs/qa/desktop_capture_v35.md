# Desktop capture v35

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v35 commit after `a11026a` (v34).  
**Official:** [m3.material.io menus](https://m3.material.io/components/menus/specs) nested submenu hover-open (`MenuOpenDelay`).

v35 started from v34 leftovers (`docs/qa/v34_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was GPUI hover-open still firing immediately. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `menu.rs` | `hover_parent` highlights immediately but returns `HoverOpenIntent::Delay`; flyout opens only after `confirm_hover_open` (seq-guarded). Click / keyboard still immediate. `HOVER_OPEN_DELAY_MS` 200 |
| Hosts | Desktop + Android `cx.spawn` + `background_executor().timer(200ms)` on overflow / split / overlay / cascade More hover. Leave / sibling hover / click / keys cancel the pending seq |
| Catalog | Note that catalog JS **and** GPUI hosts wait 200ms; inventory `GPUI/catalog hover-open 200ms (MenuOpenDelay)` |
| Tests | Overlay More hover stays closed until confirm; stale seq after leave is ignored; overflow delay + confirm; catalog `GPUI hosts` copy |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture grouped overflow/split + 200ms hover delay against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Standard overflow | Left / Right / Justify group + More, 16/8 corners, 2dp gap, 200ms hover-open | Typeahead still needs cascade `tab_index` focus |
| Connected overflow | Cut / Copy / Paste + More › Share/Save/Sort | Same focus gap |
| Split trailing | Add to cart / Save for later + More | Catalog variants share one open anatomy |
| Overlay / cascade | Unscrimmed + GPUI/catalog 200ms hover delay | Click/keyboard stay immediate (Compose `MenuOpenDelay`) |

## v36 leftovers

See `docs/qa/v35_leftovers.md`.
