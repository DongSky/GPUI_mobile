# v8 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Branch:** `cursor/material3-component-parity-d068`  
**Binary:** `target/debug/material_desktop_demo` rebuilt after v8 (`cargo build -p material_desktop_demo`).

## Live window — **succeeded**

Same lavapipe recipe as v7. `scripts/desktop-screenshot.sh` now passes ffmpeg a real grab origin (`:1+X,Y`) instead of concatenating the geometry string (that bug captured the full desktop).

- `/dev/dri` is still missing (no GPU render node).
- `WGPU_BACKEND=vulkan` + Ubuntu noble `mesa-vulkan-drivers` lavapipe (`libvulkan_lvp.so` from `/tmp/mesa-noble`).
- `VK_ICD_FILENAMES=/tmp/gpui-lvp-icd/lvp_icd.json`
- `LD_LIBRARY_PATH` prepends Mesa **and** extracted xkb, and keeps `/usr/lib/x86_64-linux-gnu`.
- Window title `Material 3 desktop catalog` · `720×880` · `DISPLAY=:1` · xdotool id `52428801` at `1200,85`.
- Scroll: `xdotool mousemove` into the content, then `click 5` / `click 4`. Do **not** `pkill -f material_desktop_demo`.

`docs/qa/catalog_*_v8.png`, `desktop_gpui_live_v8.png`, and the **right** panels of `compare_desktop_vs_official_*_v8.png` are **live GPUI pixels**.

The HTML catalog (`docs/catalog/material-catalog-*.html`) is the same `gpui_material::resolve()` recipes (carousel, wavy SVG, search activity, hand path, notch path, rail FAB/badges, docked month re-grid).

## What v8 PNGs show

| File | Content (live GPUI) |
|---|---|
| `desktop_gpui_live_v8.png` / `catalog_settings_scene_v8.png` | Sound & notifications, volume rows, notched Email, Day–Week–Month, Buttons row start |
| `catalog_buttons_v8.png` | Expressive overview + XS–XL filled sizes, connected Day/Week/Month, icon buttons, empty text fields start |
| `catalog_text_fields_v8.png` / `catalog_slider_v8.png` | Empty filled/outlined, **path-stroked notched Email**, filled+value; volume rows; dual-thumb **Price range 20–75%** |
| `catalog_progress_v8.png` / `catalog_carousel_v8.png` | Determinate + **wavy** track + PTR circle; hero carousel One/Two/Three/Four (256dp + 120dp) |
| `catalog_nav_rail_v8.png` | Collapsed 80dp + expanded 220dp rails, FAB `+`, badge `3` + dot |
| `catalog_search_v8.png` / `catalog_timepicker_v8.png` | Search **activity** (back + suggestions); minute dial + **path-drawn hand at 30** |
| `catalog_datepicker_v8.png` | Range calendar Sep 15–21 + docked **Date of birth** field |
| `catalog_datepicker_modal_v8.png` | Modal “Select date” with month `<` `>` |
| `catalog_datepicker_docked_v8.png` | Docked field focused (purple outline) + popup calendar under the field |
| `catalog_datepicker_docked_nov_v8.png` | Same popup after month `>` — **November 2026** grid (1 on Sunday); modal header follows shared month |
| `compare_desktop_vs_official_{timepicker,slider,text_fields,datepicker,buttons,search}_v8.png` | Official m3.material.io (left, reused from v7 chrome) vs **live GPUI v8** (right) |

Host tests: `gpui_material` 26, `gpui_android` 10, `material_desktop_demo` 1.

## What v8 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| Analog time hand | `hand_quad` / `hand_svg_d` | HTML SVG path + GPUI `PathBuilder` fill |
| Range slider | `fraction_from_local_x` | Continuous local-X drag; 5% click-step kept |
| Search | `resolve_activity`, `filter_suggestions` | Docked bar hides; 0-corner activity; keydown filter |
| Progress | `wavy` / `wave_polyline` | HTML CSS wave + GPUI `with_animation` clock |
| Docked date | existing `month_grid` | HTML month `<` `>` re-grids cells (Sakamoto); desktop popup + outside click |
| Notch | `NotchFrame::outline_verbs` / `outline_svg_d` | GPUI stroke path; HTML fieldset + `data-notch-path` |
| Navigation rail | expanded 220dp, FAB slot, badges | Collapsed + expanded heroes |
| Carousel | new `carousel::resolve()` | Hero 256dp + 120dp neighbors |
