# Desktop capture v49

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v49 commit after `6d9d23b` (v48).  
**Official:** [m3.material.io text fields](https://m3.material.io/components/text-fields/specs) Expressive `TextFieldDefaults.roundedShape` + `tonalColors()`.

v49 started from v48 leftovers (`docs/qa/v48_leftovers.md` as written on that landing). Rail FAB family and list swipe LazyColumn fling are shipped; highest-impact **implementable** gap vs current Expressive was Compose `TextFieldDefaults.roundedShape` (`ShapeKeyTokens.CornerMedium` 12) + `tonalColors()` with `TextFieldLabelPosition.Inside`. Hosts and the HTML catalog still used baseline extra-small top + indicator / 4dp Cutout notch. Live IME, caret bounds, and search `cx.transform` stay blocked without an NDK/gpui bump. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` stays skipped (unproven on Linux/NativeActivity). Predictive-back scale stays skipped (`cx.transform` unavailable).

## What shipped

| Surface | Change |
|---|---|
| `text_field.rs` | `TextFieldStyle::Expressive`; `roundedShape` CornerMedium 12; filled tonal SurfaceContainer + no indicator; outlined tonal OnPrimary + OutlineVariant; Inside label (no Cutout); baseline resolve() unchanged |
| Catalog | Heroes + editable use `resolve_expressive`; `data-rounded-shape="CornerMedium"` / `data-tonal` / `data-label-position="inside"`; JS skip Cutout swap on Inside |
| Hosts | Desktop + Android empty heroes + live editors use Expressive; `field_block` keys off `variant` (not bottom radius) |
| Inventory | Text field notes roundedShape + tonalColors + Inside |
| Tests | Tonal shape/colors/Inside; catalog attrs; desktop SurfaceContainer |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive rounded + tonal text fields against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| In-flow wide | 96↔220, no scrim, Inbox body, CornerNone / Surface, ContentPadding 44, Extended FAB Create 56↔188 | OS `PopUp` unopened (n/a for standard) |
| Header + FAB + Bottom | Menu/MenuOpen, Extended FAB Create 56↔188, tooltip Above, dests flex-end, Inbox shifts, no scrim, CornerNone / Surface, ContentPadding 44 | Same OS `PopUp` gap |
| Modal rail | 96↔220, 32% scrim, CornerLarge 16 + SurfaceContainer, ContentPadding 44, header-less Extended FAB Create 56↔188 | OS `PopUp` unopened; predictive-back scale skipped |
| Narrow modal | Live 80↔220 + same modal container + ContentPadding 44 + Extended FAB Create (12dp collapsed inset) | Same OS `PopUp` + predictive-back gap |
| Hide-on-collapse modal | Slides offscreen, Menu ☰, Start items, Center arrangement, expanded modal shape, ContentPadding 44, always-extended Create ~188 | OS `PopUp` unopened; predictive-back scale skipped |
| List swipe | Archive/Delete 80dp rails, Team sync notes Open, LazyColumn fling to Closed/Open/primary, 16dp overshoot, growing reveal | Nested Android `LazyColumn` + OS overscroll glow not in GPUI; predictive-back scale skipped |
| Text field | Expressive rounded 12 + tonal filled SurfaceContainer / outlined OnPrimary, Inside label | Live JVM IME / GPUI caret bounds still blocked |

## v50 leftovers

See `docs/qa/v49_leftovers.md`.
