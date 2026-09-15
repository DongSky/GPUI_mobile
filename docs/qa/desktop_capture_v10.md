# v10 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (standing order: no iteration PRs; PR #4 is already merged)  
**Binary:** `target/debug/material_desktop_demo` after the v10 C-path / Expressive pass.

Live GPUI capture: `scripts/desktop-screenshot.sh` + wheel-scroll frames, `DISPLAY=:1`, lavapipe Vulkan (`mesa-vulkan-drivers` 25.2.8 + LLVM 20.1), window **720×880** at `+2,85`.

`docs/qa/catalog_*_v10.png` and `desktop_gpui_live_v10.png` are **live GPUI pixels**. Walkthrough copies: `/opt/cursor/artifacts/screenshots/v10_*.png`.

HTML catalog (`docs/catalog/material-catalog-*.html`) shares `gpui_material::resolve()`. `docs/qa/catalog_html_light_v10.png` is a headless-Chrome crop of that HTML (inventory header). Android `component_demo` maps the same tokens (NDK not present; crate typechecks only).

Host tests: `gpui_material` **26**, `gpui_android` **11** (IME caret stub), `material_desktop_demo` **1**.

## What the v10 PNGs show

| File | Live GPUI content |
|---|---|
| `desktop_gpui_live_v10.png` / `catalog_settings_scene_v10.png` | Settings: sliders, **notched Email** (label sits in a top-stroke gap), Day–Week–Month |
| `catalog_text_fields_v10.png` | Empty filled + empty outlined vs **populated Email notch** |
| `catalog_slider_v10.png` | Dual-thumb **Price range · 20–75% · min span 5%** with painted ticks; wavy; **contained morph PTR** (cookie/burst) + uncontained morph + **round-capped** circular |
| `catalog_progress_v10.png` / `catalog_progress_carousel_v10.png` | Progress + **hero carousel** One–Four |
| `catalog_nav_rail_v10.png` / `catalog_nav_rail_scrim_v10.png` | **Expanded 220dp modal rail over 32% scrim** (`←` FAB, Home/Search/Profile, badges) |
| `catalog_search_v10.png` / `catalog_search_activity_v10.png` | Search **activity** growing-bar (back, caret `\|`, suggestions) |
| `catalog_timepicker_v10.png` / `catalog_timepicker_dial_v10.png` | Polar minute dial + analog hand at 30; date-range hero |
| `catalog_datepicker_v10.png` | Docked date field + calendars |
| `catalog_html_light_v10.png` | HTML catalog (same `resolve()`), not GPUI pixels |

## What v10 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| Notch (P0) | Single C-path `evenodd_verbs` (outer CW + inner CCW, **one** contour, no gap-chip). `evenodd_polygon` uses **explicit quarter-circle corners** (not inferred SVG-arc centers). | **GPUI + HTML even-odd fill** of that C so the floating label cuts the stroke on any background. |
| Loading + PTR (P0) | `loading_polygon` 7-shape morph; `contained_loading_indicator` = primary-container / on-primary-container; `round_capped_arc_polygon`; `clock_ms`. | PTR = contained morph. Circular = filled sausage + **disc-stamped** round caps (not a stroked polyline crescent). |
| Nav rail (P1) | `morph_width_dp`, `scrim_opacity_at`, `elevation_dp_at`, `morph_ms` | Width + scrim opacity + shadow; expanded is a modal overlay, not a plain width swap. |
| Search (P1) | `morph_height_dp` / `morph_corner_dp_at` / `morph_list_opacity` | One growing-bar tree, overflow clip, list fade. HTML max-height/opacity transition. |
| Range (P1) | `snap_to_step` while dragging; `click_step` snaps nearest thumb to the 5% grid; `range_tick_fractions` (21) | Painted ticks; min-span stays one tick (5%). |
| Carousel (P2) | `inertial_steps` (`v₀ e^{-kt}`, `k=2`) can skip **beyond ±1** (~96dp → 2 items) | Wheel uses inertial steps; `integrate_fling` kept for a later per-frame loop. |
| IME (P2) | `ime_caret_rect_dp` | `gpui_android::ime::record_caret_rect` from NativeActivity `update_ime_position` (still no InputConnection). |
| Time (P2) | `hour_face_live_angle_deg` | Repeating hour-face hand motion while the hour dial is showing. |

Official refs used: [text fields](https://m3.material.io/components/text-fields/specs), [loading indicator](https://m3.material.io/components/loading-indicator/overview) (7-shape morph; contained for PTR), [progress](https://m3.material.io/components/progress-indicators/specs), [navigation rail](https://m3.material.io/components/navigation-rail/guidelines).

## v11 leftovers

See `docs/qa/v11_leftovers.md`.
