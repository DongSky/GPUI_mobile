# v12 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Binary:** `target/debug/material_desktop_demo` after the v12 JNI / RoundedPolygon / wait-morph / overlay-rail / live-clock pass.

Live GPUI capture: `scripts/desktop-screenshot.sh` + wheel-scroll frames, `DISPLAY=:1`, lavapipe Vulkan (`mesa-vulkan-drivers`) with `WGPU_BACKEND=gl` fallback, window **720×880** at `+2,85`.

`docs/qa/catalog_*_v12.png` and `desktop_gpui_live_v12.png` are **live GPUI pixels**. Walkthrough copies: `/opt/cursor/artifacts/screenshots/v12_*.png`.

HTML catalog (`docs/catalog/material-catalog-*.html`) shares `gpui_material::resolve()`. `docs/qa/catalog_html_light_v12.png` is a headless-Chrome crop of that HTML (inventory header). Android `component_demo` maps the same tokens (NDK not present; crate typechecks only). Official m3.material.io compare strips were not recaptured this pass (headless Chrome timed out); last strips remain `compare_desktop_vs_official_*_v9.png`.

Host tests: `gpui_material` **26**, `gpui_android` **15** (JNI IMM plan + InputConnection native dispatch), `material_desktop_demo` **1**.

## What the v12 PNGs show

| File | Live GPUI content |
|---|---|
| `desktop_gpui_live_v12.png` / `catalog_settings_scene_v12.png` | Settings: sliders, **notched Email** (RoundedPolygon cubics), Day–Week–Month |
| `catalog_buttons_v12.png` | Quiet hours + **filled / tonal / XL** buttons |
| `catalog_text_fields_v12.png` | Empty filled + empty outlined vs **populated Email notch** (C-path from cubics) |
| `catalog_slider_v12.png` | Dual-thumb **Price range · 20–75% · min span 5%** with painted ticks |
| `catalog_progress_v12.png` / `catalog_progress_carousel_v12.png` | Wavy + **contained PTR** + **WaitProgress determinate morph** (live %) + **round-capped** circular sausage + carousel |
| `catalog_nav_rail_v12.png` / `catalog_nav_rail_scrim_v12.png` | **Expanded 220dp overlay-window rail** (`#nav-rail-window`) over 32% scrim |
| `catalog_search_v12.png` / `catalog_search_activity_v12.png` | Search **activity** growing-bar (back, caret `\|`, suggestions) under the rail hero |
| `catalog_timepicker_v12.png` / `catalog_timepicker_dial_v12.png` | Polar minute dial + **wall-clock second hand** |
| `catalog_datepicker_v12.png` | Date-range hero Sep 15–21 + docked field + calendar |
| `catalog_html_light_v12.png` | HTML catalog (same `resolve()`), not GPUI pixels |

## What v12 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| IME | caret + session unchanged | `gpui_android::ime` JNI tables (`toggleSoftInput` SHOW_FORCED / HIDE_IMPLICIT_ONLY), packed `CursorAnchorInfoPayload`, `ImeSession::jni_imm_calls()`, `dispatch_native_input_connection()`; still no live `JNIEnv` / View-backed `InputConnection` |
| Notch | `sample_cubic` / `flatten_rounded_quarter`; live even-odd path uses `OutlineVerb::Cubic` via `rounded_polygon_quarter` + `CIRCULAR_KAPPA`; glyph-advance `notch_width_dp` | GPUI/HTML flatten the same verbs; SVG `C` commands; `data-notch-rounded-polygon="1"` |
| Loading | `WaitProgress` (bytes / elapsed / fraction), `DEMO_WAIT`, `determinate_wait_ms`, `loading_polygon_for_wait` | Hosts drive morph + live `%` from a repeating wait clock; HTML `data-wait-progress` + rAF |
| LineCap | Circular/PTR filled sausage; wavy endpoint discs via `paint_round_capped_polyline` | HTML `data-linecap="round"`; gpui still does not re-export `LineCap::Round` |
| Nav rail | `overlay_window()` / `overlay_window_attr()` | Desktop/Android expanded rail in absolute `#nav-rail-window`; HTML `.rail-window` + `data-rail-window`. Not a second OS `Window` |
| Carousel | `FlingState::step_live`, `FLING_FRAME_DT` | Catalog ticks from `Instant`; wheel **impulses**; HTML rAF integrator |
| Time | `wall_second()` / `second_hand_angle_wall_clock()` | GPUI paints wall-clock angle each 1 s tick; HTML rAF `Date` rotation (`data-second-wall="1"`) |

## v13 leftovers

See `docs/qa/v12_leftovers.md`.
