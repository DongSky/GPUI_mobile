# Desktop capture v18

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v18 commit after `33a994d` (v17).  
**Official:** [m3.material.io/develop/flutter/overview](https://m3.material.io/develop/flutter/overview) Expressive.

v18 started from v17 leftovers (`docs/qa/v17_leftovers.md` as written on that landing). Highest-impact **implementable** gap was leftover **#9**: FAB menu, split button, and floating toolbar were not in `INVENTORY`. Also closed leftover **#7–8** chrome that does not need photo decode: snackbar avatars/timestamps/nav, tabs status time, share-sheet people row.

## What shipped

| Surface | Change |
|---|---|
| `fab_menu.rs` | androidx `FabMenuBaselineTokens` — 56dp items, 24dp pad, 8dp close gap, 4dp item gap, 20dp close icon; primary/secondary/tertiary color sets; expanded close = solid set color |
| `split_button.rs` | androidx `SplitButtonSmallTokens` — 2dp gap, outer full-round, inner rest 4/4/4/8/12, press 8/12/12/20/20; open trailing = full-round + pressed layer; filled/tonal/elevated/outlined |
| `toolbar.rs` | `FloatingToolbarTokens` — 64dp, 8dp pad, 4dp gap, elev 3 floating / 0 docked; standard vs vibrant; optional trailing FAB |
| Snackbar | Mail rows with initials avatars + timestamps; 24dp status bar + `9:41`; 80dp Inbox/Starred/Profile nav; phone 520dp |
| Tabs | Status chrome on saved-media phone; phone 560dp |
| Bottom sheet | People row (AR/JL/SC/+ Add); status bar; phone 620dp |
| Catalog / hosts | HTML + `material_desktop_demo` + `component_demo` heroes; catalog JS keeps in-phone snack visible |
| Tests | `fab_menu_split_button_toolbar_tokens`; catalog evidence for `data-fab-menu`, `data-split`, `data-toolbar`, avatars, people, status |

Host tests (this VM): **27** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

Left = headless Chrome on m3.material.io. Right = extracted HTML catalog hero (not live wgpu).

- `docs/qa/compare_desktop_vs_official_fab_menu_v18.png`
- `docs/qa/compare_desktop_vs_official_split_button_v18.png`
- `docs/qa/compare_desktop_vs_official_toolbar_v18.png`
- `docs/qa/compare_desktop_vs_official_snackbar_v18.png`
- `docs/qa/compare_desktop_vs_official_tabs_v18.png`
- `docs/qa/compare_desktop_vs_official_bottom_sheet_v18.png`

## Critique vs Expressive (from those strips)

| Hero | Match | Gap |
|---|---|---|
| FAB menu | Document / Message / Folder labels, 56dp pills, close FAB, color sets | Catalog is a token column, not a photo phone frame; items trail-align with the close control |
| Split button | `$7.49` + chevron, small size, inner rest radii | Catalog is a size/variant matrix, not the enamel-mugs **product card**; open example is extra vs official overview (closed) |
| Toolbar | Four actions + trailing FAB, 64dp, full-round floating vs docked | Not in a chat/photo scene; vibrant FAB is primary-container purple vs official pink |
| Snackbar | Avatars, timestamps, status `9:41`, 80dp nav, Email archived | Initials not photos; Inbox/Starred/Profile not Mail/Chat/Rooms/Meet; in-phone snack persists (`data-persist`) and sits in-flow above nav |
| Tabs | Video/Photos/Audio + stub tiles + status | No camera photos; Audio not shown selected in the extract |
| Share sheet | People row + Copy/WhatsApp/Messages/Gmail | Initials not named camera photos |

## v19 leftovers

See `docs/qa/v18_leftovers.md`. Photo assets, official Gmail dest labels, in-scene chrome for the three new components, live IME/caret, APK, wgpu recapture.
