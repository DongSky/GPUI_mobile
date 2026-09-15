# Desktop capture v29

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v29 commit after `4efd4d8` (v28).  
**Official:** [m3.material.io chips](https://m3.material.io/components/chips/specs) + Compose `FilterChip` / `InputChip` `ChipShapes` morph (1.5 / I/O 2026 Expressive).

v29 started from v28 leftovers (`docs/qa/v28_leftovers.md` as written on that landing). Highest-impact **implementable** gap vs current Expressive was chips still on the 32dp full-round baseline. Live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `chip.rs` | Compose `Shapes.defaultChipShapes`: Filter/Input morph CornerMedium 12 rest → CornerFull selected → CornerSmall 8 pressed. Selected FilterChip leading check (`✓`); InputChip trailing close (`×`). Assist / Suggestion stay 32dp full-round. Official hero labels Elevator / Washer / Pets + Portland |
| Catalog | `data-hero="chips"` filter row + input row; matrix chips now emit per-chip `border-radius` / pad from `resolve()` |
| Hosts | `material_desktop_demo` + `component_demo` paint both hero rows from `FILTER_HERO` / `INPUT_HERO` |
| Inventory | Chip notes Expressive Filter/Input morph |
| Tests | Token corners/pads/glyphs; catalog evidence; desktop 12 / 16 / 8 |

Host tests (this VM): **32** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive chips against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Filter morph | Elevator 12dp outline, Washer full-round secondary-container + check, Pets 8dp pressed | Catalog is static anatomy, not live press interpolation |
| Input morph | Portland 12dp + close; selected full-round secondary-container + close | No avatar / elevated InputChip |
| Assist / Suggestion | 32dp full-round baseline (no Compose morph overload) | Correct — do not invent morph |
| Spacing | 8dp lead/trail inset + 8dp icon gap | Compact 4dp arrangement is token-only |

## v30 leftovers

See `docs/qa/v29_leftovers.md`.
