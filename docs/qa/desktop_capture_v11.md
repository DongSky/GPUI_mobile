# v11 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Base:** `81beada` (v10 C-path notch + Expressive morph loading) plus this v11 layer.

## What v11 adds on top of the latest `main`

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| IME | `ime_caret_rect_in_window` | `gpui_android::ime::ImeSession` (`commitText` / compose / delete / cursor anchor); `update_ime_position` fills session + `last_ime_bounds` (still no JNI) |
| Search | `MorphFrame` / `morph_frame_eased` / `morph_container` | Spatial-fast height + corners + **shared-element inset/scale**; HTML `data-search-shared` |
| Range | `range_ticks` / `range_tick_count` | Discrete 5% stop ticks on the dual-thumb track (alongside existing snap) |
| Shape | `CORNER_SMOOTHING` / `rounded_polygon_quarter` | RoundedPolygon helpers; live notch remains the v10 C-path even-odd polygon |
| Nav rail | `focus_trapped` / `modal_elevation_dp` / `dismiss_on_scrim` | Catalog `data-rail-focus-trap`; scrim dismiss |
| Carousel | `FlingState::step` | Per-frame integrator next to existing `integrate_fling` |

Live GPUI frames stay the v10 set (`docs/qa/*_v10.png`) until a v11 recapture. HTML catalog (`docs/catalog/material-catalog-*.html`) shares `gpui_material::resolve()`.

Host tests: `gpui_material` 26, `gpui_android` 14 (IME InputConnection session), `material_desktop_demo` 1.

## v12 leftovers

See `docs/qa/v11_leftovers.md`.
