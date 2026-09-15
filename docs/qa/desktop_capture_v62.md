# Desktop capture v62

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v62 commit after `c6c1bf2` (v61 trailing clear-X).  
**Official:** [m3.material.io lists specs](https://m3.material.io/components/lists/specs) — Expressive two-line items use `ItemLeadingAvatarSize` 40dp and `ItemLeadingIconExpressiveSize` 20dp, with `ItemBetweenSpace` 12dp.

v62 started from v61 leftovers (`docs/qa/v61_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **leading 40dp avatar / 20dp icon on two-line search results**. Do not reopen trailing clear-X, two-line supporting/open, segmented rows, TimePickerLayoutType Horizontal, docked scrim, Results/Quick results, ClockFace, groups, compact→docked, TimeInput, contained visual tokens. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `search.rs` | `ROW_LEADING_AVATAR_DP` 40 / `ROW_LEADING_ICON_DP` 20 / 12dp gap; App → avatar, other two-line → icon |
| Catalog | `data-search-leading="avatar"|"icon"`; 40dp circle or 20dp glyph on queried rows |
| Hosts | Desktop + Android paint sized leading; Android regains the missing leading slot |
| Inventory | Search notes 40dp avatar / 20dp icon |
| Tests | Kind/size tokens, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture 40/20 leading against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Search | Compact fullscreen; medium docked + 32% scrim; Quick results / Results; groups; segmented 2/4/16; two-line 72dp; trailing clear-X; 40/20 leading | `cx.transform` search morph skipped; catalog no-results sibling still open |
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings; Horizontal landscape sibling | Recapture stills when screenshot works |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v63 leftovers

See `docs/qa/v62_leftovers.md`.
