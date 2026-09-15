# v10 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (user asked to stop updating PR #4; work continues as commits on the default branch)  
**Binary:** `target/debug/material_desktop_demo` rebuilt after v10.

## What v10 PNGs show

Live GPUI capture (`720×880` lavapipe). Scroll frames are labeled by capture order; content is listed honestly.

| File | Live GPUI content |
|---|---|
| `desktop_gpui_live_v10.png` / `catalog_settings_scene_v10.png` | Settings scene, notched Email, Day–Week–Month |
| `catalog_text_fields_v10.png` | Empty filled/outlined + **even-odd notched Email**; **Price range · 20–75% · min span 5%** |
| `catalog_slider_v10.png` / `catalog_progress_carousel_v10.png` | Range min-span label; wavy + **round-capped PTR** + **Loading** circular; carousel |
| `catalog_progress_v10.png` / `catalog_nav_rail_scrim_v10.png` | Carousel **Three** selected (wheel fling skipped past ±1); **expanded rail over 32% scrim** (`←` FAB, Home/Search/Profile) |
| `catalog_nav_rail_v10.png` / `catalog_search_activity_v10.png` | Search activity **growing-bar** with caret `\|` + suggestions; time picker header |
| `catalog_search_v10.png` / `catalog_timepicker_dial_v10.png` | Polar minute dial + analog hand at 30; date range |
| `catalog_timepicker_v10.png` / `catalog_datepicker_v10.png` | Docked date field + calendars |

Walkthrough copies also live under `/opt/cursor/artifacts/screenshots/v10_*`.

Same lavapipe recipe as v8/v9 (`scripts/desktop-screenshot.sh`, `DISPLAY=:1`, `720×880`).

`docs/qa/catalog_*_v10.png`, `desktop_gpui_live_v10.png` are **live GPUI pixels** when capture succeeds.

HTML catalog (`docs/catalog/material-catalog-*.html`) shares `gpui_material::resolve()`. Android `component_demo` maps the same morph/search, snap range, round-capped PTR, even-odd notch, modal rail, fling carousel, and hour-hand lerp (NDK not present in this VM so Android was not rebuilt here).

Host tests: `gpui_material` 26, `gpui_android` 11 (IME stub), `material_desktop_demo` 1.

## What v10 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| Search | `morph_t` / `morph_height_dp` / `morph_corner_dp_at` / `morph_ms` | One growing-bar tree; `with_animation` spatial-fast; caret editor kept |
| Range | `snap_to_step`, `drag_thumb_snapped`, documented `RANGE_MIN_SPAN` (5%) | Tick-snap while dragging; label shows min span |
| Progress | `clock_ms`, `ptr_cap_centers`, `loading_circular` | Round-capped PTR/loading stroke (PathBuilder + cap discs); shared clock |
| Notch | `evenodd_verbs` / `evenodd_svg_d` | GPUI even-odd fill + stroke; HTML **fieldset** + hidden evenodd path |
| Nav rail | `scrim()`, `is_modal` | Collapsed 80dp; expanded 220dp **modal + 32% scrim**; dest selection kept |
| Carousel | `fling_steps`, `decay_velocity` | Wheel fling can skip more than ±1 item |
| IME | `ime_caret_rect_dp` | `gpui_android::ime` records bounds from `update_ime_position` (still no InputConnection) |
| Time | `lerp_angle_deg`, `hand_quad_at_angle` | Hour-face / value change interpolates the analog hand |

## v11 leftovers

- Real system IME / JNI InputConnection (NativeActivity still has none).
- Search morph is height/corner lerp, not a pixel-perfect M3 container transform.
- Range has no live stop ticks painted on the dual-thumb track.
- PTR round caps are tessellated discs (gpui does not re-export `LineCap`).
- Rail modal is an in-catalog overlay, not a separate `Window`.
- Carousel decay is a one-shot `e^{-kt}` helper, not a physics integrator.
- Hour hand animation is one-shot per change, not a continuous ticking second hand.
