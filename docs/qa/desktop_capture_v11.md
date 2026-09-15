# v11 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Binary:** `target/debug/material_desktop_demo` after the v11 IME / shared-element / leftover-polish pass.

Live GPUI capture: `scripts/desktop-screenshot.sh` + wheel-scroll frames, `DISPLAY=:1`, lavapipe Vulkan (`mesa-vulkan-drivers` 25.2.8 + LLVM 20.1), window **720×880** at `+2,85`.

`docs/qa/catalog_*_v11.png` and `desktop_gpui_live_v11.png` are **live GPUI pixels**. Walkthrough copies: `/opt/cursor/artifacts/screenshots/v11_*.png`.

HTML catalog (`docs/catalog/material-catalog-*.html`) shares `gpui_material::resolve()`. `docs/qa/catalog_html_light_v11.png` is a headless-Chrome crop of that HTML (inventory header). Android `component_demo` maps the same tokens (NDK not present; crate typechecks only).

Host tests: `gpui_material` **26**, `gpui_android` **14** (IME InputConnection session), `material_desktop_demo` **1**.

## What the v11 PNGs show

| File | Live GPUI content |
|---|---|
| `desktop_gpui_live_v11.png` / `catalog_settings_scene_v11.png` | Settings: sliders, **notched Email**, Day–Week–Month |
| `catalog_buttons_v11.png` | Quiet hours + **filled / tonal / XL** buttons |
| `catalog_text_fields_v11.png` | Empty filled + empty outlined vs **populated Email notch** |
| `catalog_slider_v11.png` | Dual-thumb **Price range · 20–75% · min span 5%** with painted ticks |
| `catalog_progress_v11.png` / `catalog_progress_carousel_v11.png` | Range ticks + wavy + **contained PTR morph** + **Loading + 65% determinate morph** + circular + carousel |
| `catalog_nav_rail_v11.png` / `catalog_nav_rail_scrim_v11.png` | **Expanded 220dp modal rail over 32% scrim** (`←` FAB, Home/Search/Profile, badges) |
| `catalog_search_v11.png` / `catalog_search_activity_v11.png` | Search **activity** growing-bar (back, caret `\|`, suggestions) under the rail hero |
| `catalog_timepicker_v11.png` / `catalog_timepicker_dial_v11.png` | Polar minute dial at 30 + **ticking second hand** (demo second 12) |
| `catalog_datepicker_v11.png` | Date-range hero Sep 15–21 + docked field + calendar |
| `catalog_html_light_v11.png` | HTML catalog (same `resolve()`), not GPUI pixels |

## What v11 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| IME | `ime_caret_rect_in_window` | `gpui_android::ime::ImeSession` (`commitText` / compose / delete / cursor anchor); `update_ime_position` fills session + `last_ime_bounds` (still no JNI) |
| Search | `MorphFrame` / `morph_frame_eased` / `morph_container` / `morph_back_opacity` | Spatial-fast height + corners + **shared-element inset/scale** + **container color lerp** + **leading icon/back + avatar crossfade**; HTML `data-search-lead` |
| Range | `range_ticks` / `range_tick_count` | Discrete 5% stop ticks on the dual-thumb track (alongside existing snap) |
| Shape | `CORNER_SMOOTHING` / `rounded_polygon_quarter` | RoundedPolygon helpers; live notch remains the v10 C-path even-odd polygon |
| Nav rail | `focus_trapped` / `modal_elevation_dp` / `dismiss_on_scrim` | Catalog `data-rail-focus-trap`; scrim dismiss |
| Loading | `loading_polygon_for_progress` / `loading_phase_for_progress` | Determinate 7-shape morph (no spin) at 65% next to the cycling indicator |
| Carousel | `FlingState::step_until_rest` / `apply_wheel` | Wheel uses 60 Hz integrator (HTML catalog still one-shot JS) |
| Time | `second_hand_angle_deg` / `second_hand_quad` | 60 s repeating second hand on GPUI + HTML |

## v12 leftovers

See `docs/qa/v11_leftovers.md`.
