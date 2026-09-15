# Desktop capture v19

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v19 commit after `ddc5957` (v18).  
**Official:** [m3.material.io/develop/flutter/overview](https://m3.material.io/develop/flutter/overview) Expressive.

v19 started from v18 leftovers (`docs/qa/v18_leftovers.md` as written on that landing). Highest-impact **implementable** gaps were leftovers **#6–9**: photo chrome for snackbar/tabs/share/carousel, Gmail dest labels + Action copy, FAB menu / split / toolbar in-scene heroes, and carousel uncontained multi-aspect. Live IME/caret/search transform, APK, and wgpu recapture stay blocked (no NDK/SDK/display).

## What shipped

| Surface | Change |
|---|---|
| `photo_stub.rs` | Named landscape/portrait/product stubs; CSS layered gradients + GPUI `fill()`/`accent()`; helpers for carousel/share/tabs |
| Snackbar | Mail/Chat/Rooms/Meet; Sofia Sacchi / Carmen Villanueva photo avatars; “Email archived / Action”; Meet badge; phone 560dp |
| Tabs | Audio selected (`SCENE_SELECTED = 2`); Bloom/Egret tiles; app-bar back + mic/cal/overflow; status `9:30` |
| Bottom sheet | Album hero + horizontal Share/Add to/Trash/Order prints/Move to archive; Send + five named people |
| FAB menu | Woven-basket phone scene; trail-aligned Document/Message/Folder menu |
| Split button | Enamel mugs product card (`$7.49`, five mug colors) |
| Toolbar | Chat thread (Renee Claess + dog stub); vibrant FAB = `FabVariant::Tertiary` (pink) |
| Carousel | Sixth layout `UncontainedMulti` (`uncontained-multi`); per-item heights; photographic media fills; catalog JS widths 168/112 |
| Catalog / hosts | HTML + `material_desktop_demo` + `component_demo` heroes share the same tokens |
| Tests | Catalog evidence for `data-photo`, `data-fab-scene`, `data-split-scene`, `data-toolbar-scene`, Mail/Meet, Alejandro, uncontained-multi |

Host tests (this VM): **27** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

Left = headless Chrome on m3.material.io (reused from v18 strips; carousel left from v17). Right = extracted HTML catalog hero (not live wgpu).

- `docs/qa/compare_desktop_vs_official_snackbar_v19.png`
- `docs/qa/compare_desktop_vs_official_fab_menu_v19.png`
- `docs/qa/compare_desktop_vs_official_split_button_v19.png`
- `docs/qa/compare_desktop_vs_official_toolbar_v19.png`
- `docs/qa/compare_desktop_vs_official_tabs_v19.png`
- `docs/qa/compare_desktop_vs_official_bottom_sheet_v19.png`
- `docs/qa/compare_desktop_vs_official_carousel_v19.png`

## Critique vs Expressive (from those strips)

| Hero | Match | Gap |
|---|---|---|
| Snackbar | Mail/Chat/Rooms/Meet, Action, Sofia/Carmen names + photo-stub avatars, Email archived | Gradients not camera portraits; Inbox title still shown; third row is a full mail item vs official peek; dest icons are BMP glyphs, not filled Gmail icons |
| FAB menu | Document / Message / Folder over a basket-colored phone, trail-aligned close FAB | Basket is a radial gradient, not the woven-basket JPEG |
| Split button | Enamel mugs card, `$7.49`, five mug colors | Mugs are CSS cylinders on a flat fill, not the product photo |
| Toolbar | Chat thread + vibrant tertiary (pink) FAB + four actions | Dog/avatar are stubs; no camera photo of the terrier |
| Tabs | Audio selected, two landscape tiles, back/mic/cal/overflow, `9:30` | Bloom/Egret are gradients, not the flower/egret photos |
| Share sheet | Horizontal actions, Send, named people (Alejandro … Marty) | Album hero is a party gradient; people are radial portraits |
| Carousel | Six layouts including uncontained-multi with varying tile heights | Gradient media, not decoded photos; multi-aspect is a token row not a phone “Your lists” mask |

## v20 leftovers

See `docs/qa/v19_leftovers.md`. Decoded JPEGs, live IME/caret, APK, wgpu recapture.
