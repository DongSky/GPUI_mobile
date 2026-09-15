# Desktop capture v20

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v20 commit after `33014cf` (v19).  
**Official:** [m3.material.io/develop/flutter/overview](https://m3.material.io/develop/flutter/overview) Expressive.

v20 started from v19 leftovers (`docs/qa/v19_leftovers.md` as written on that landing). Highest-impact **implementable** gap was leftover **#1**: bundled JPEG decode for catalog + GPUI hosts. Also closed v19 critique chrome that does not need a camera: snackbar peek (no Inbox title), filled dest SVGs, carousel Your lists phone.

## What shipped

| Surface | Change |
|---|---|
| `photo_stub.rs` | Procedural RGB → JPEG encode → **decode** (`image` jpeg); `data_uri` / `css_background` for HTML; `mosaic()` for GPUI hosts |
| Catalog | `data-decoded-jpeg="1"` + `url('data:image/jpeg;base64,…')` on scene photos |
| Snackbar | Peeking “Shows lined up” row (`PEEK_H_DP`); no Inbox title; filled Mail/Chat/Rooms/Meet SVGs |
| Split button | Enamel-mugs **JPEG** hero (mugs baked into the photo); CSS cylinder overlay removed |
| FAB menu / toolbar / tabs / sheet / carousel | Decoded JPEG fills instead of CSS gradients |
| Carousel | Uncontained-multi wrapped in official **Your lists** phone (Starred places / Labeled) |
| Hosts | `material_desktop_demo` + `component_demo` paint JPEG mosaics |
| Tests | JPEG SOI + decode + mosaic; catalog evidence for peek, dest SVG, Your lists, `url('data:image/jpeg` |

Host tests (this VM): **28** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves:

- `docs/qa/compare_desktop_vs_official_snackbar_v19.png`
- `docs/qa/compare_desktop_vs_official_fab_menu_v19.png`
- `docs/qa/compare_desktop_vs_official_split_button_v19.png`
- `docs/qa/compare_desktop_vs_official_toolbar_v19.png`
- `docs/qa/compare_desktop_vs_official_tabs_v19.png`
- `docs/qa/compare_desktop_vs_official_bottom_sheet_v19.png`
- `docs/qa/compare_desktop_vs_official_carousel_v19.png`

Right panels of those strips are **pre-JPEG** HTML. Recapture against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Snackbar | Peeking thread, Sofia/Carmen, Action, Mail/Chat/Rooms/Meet SVGs, no Inbox title | Procedural JPEG portraits, not camera faces |
| FAB menu | Document / Message / Folder over a woven-basket JPEG | Basket is generated weave, not the product photo |
| Split button | Enamel mugs JPEG + `$7.49` | Mugs are painted cylinders in the JPEG, not the still life |
| Toolbar | Chat thread + vibrant tertiary FAB | Dog/avatar are decoded procedurals |
| Tabs | Audio selected, Bloom/Egret JPEG tiles | Not the flower/egret photographs |
| Share sheet | Horizontal actions, Send, named people | Album/people are decoded procedurals |
| Carousel | Your lists phone + uncontained-multi heights | JPEG tiles, not the three camera photos |

## v21 leftovers

See `docs/qa/v20_leftovers.md`. Licensed stills, live IME/caret, APK, recapture, collapsing app bars / nav drawer.
