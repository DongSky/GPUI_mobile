# Desktop capture v33

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v33 commit after `f4106c7` (v32).  
**Official:** [m3.material.io chips](https://m3.material.io/components/chips/specs) + Compose `ElevatedFilterChip` / `tonalElevatedFilterChipColors` + [menus](https://m3.material.io/components/menus/specs) unscrimmed dropdown.

v33 started from v32 leftovers (`docs/qa/v32_leftovers.md` as written on that landing). Highest-impact **implementable** gaps vs current Expressive were missing ElevatedFilterChip / tonal leading-icon defaults and overlay menus still sitting on a 32% scrim. Live IME, APK, and recapture stay environment-blocked. Critique vs current Expressive: **do not add banners or nav drawers**.

## What shipped

| Surface | Change |
|---|---|
| `chip.rs` | `ChipColor` Flat / Tonal / Elevated / TonalElevated. Flat FilterChip uses outline-variant + on-surface-variant label. Tonal leading icon `ChipsTokens.UnselectedLeadingIconColor` (on-surface-variant vs Primary). ElevatedFilterChip: surface-container-low, elev 1, no outline. Heroes: Elevator/Washer/Pets + Wifi tonal + ElevatedFilterChip row |
| Catalog | `data-hero="chips"` filter/tonal/input rows; `data-hero="chips-elevated"`; overlay is `data-anchored="1"` `data-scrim="0"` next to a Menu anchor (no 32% scrim fill) |
| Hosts | Android + desktop paint tonal + elevated chip rows; Menu overlay is a transparent host over the catalog with an anchored popup (no scrim) |
| Inventory | Chip notes ElevatedFilterChip + tonal; Menu notes unscrimmed anchored overlay |
| Tests | Elevated/tonal tokens + catalog `data-chip-style` / `data-scrim="0"` / Menu anchor; desktop 1dp / on-surface-variant lead |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. `scripts/test.sh` OK. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture Expressive ElevatedFilterChip and unscrimmed overlay against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Flat filter | Elevator 12dp outline-variant, Washer full-round secondary-container + check, Pets 8dp pressed | Catalog is static anatomy, not live press interpolation |
| Tonal | Wifi + leading ◈ on-surface-variant | Compact 4dp arrangement token-only |
| ElevatedFilterChip | surface-container-low · elev 1 · no outline; selected still secondary-container | No InputChip avatar |
| Overlay | Menu anchor + grouped popup, `data-scrim="0"` | Overflow/split still a single 16dp shell; no hover delay |

## v34 leftovers

See `docs/qa/v33_leftovers.md`.
