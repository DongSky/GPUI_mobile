# v7 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Branch:** `cursor/material3-component-parity-d068`  
**Binary:** `target/debug/material_desktop_demo` rebuilt after v7 (`cargo build -p material_desktop_demo`).

## Live window — **succeeded**

A real Desktop GPUI window **opened** on this VM using Mesa **lavapipe** (software Vulkan).

- `/dev/dri` is still missing (no GPU render node).
- `WGPU_BACKEND=vulkan` + `VK_ICD_FILENAMES` pointing at Ubuntu noble `mesa-vulkan-drivers` 25.2.8 (`libvulkan_lvp.so`) linked against system `libLLVM.so.20.1` and `libxml2.so.2`.
- **Critical:** `LD_LIBRARY_PATH` must *prepend* the lavapipe dir and keep `/usr/lib/x86_64-linux-gnu` (and extracted `libxkbcommon-x11`) so the binary still finds XKB. Setting `LD_LIBRARY_PATH` to *only* the Mesa folder hides `libxkbcommon-x11.so.0` and the process never starts.
- Window title: `Material 3 desktop catalog` · `720×880` · `DISPLAY=:1`.
- Capture helper: `scripts/desktop-screenshot.sh` (ffmpeg x11grab). Do **not** `pkill -f material_desktop_demo` (kills the wrapper). Kill by `readlink /proc/pid/exe`.

`docs/qa/catalog_*_v7.png` and `desktop_gpui_live_v7.png` on the right of `compare_desktop_vs_official_*_v7.png` are **live GPUI pixels**, not the HTML catalog.

The HTML catalog (`docs/catalog/material-catalog-*.html`) remains the token-accurate fallback and matches the same `gpui_material::resolve()` recipes.

## What v7 PNGs show

| File | Content |
|---|---|
| `desktop_gpui_live_v7.png` / `catalog_settings_scene_v7.png` | Live GPUI: Sound & notifications, volume rows, notched Email, Day–Week–Month |
| `catalog_slider_v7.png` | Dual-thumb Price range 20–75% + indeterminate / pull-to-refresh progress + nav rail start |
| `catalog_timepicker_v7.png` | Minute dial 00–55, analog selector hand at 30, header `06 : 30`, AM/PM |
| `catalog_search_v7.png` | Expanded search view rows + time picker (docked bar toggles the sheet) |
| `catalog_nav_rail_v7.png` | 80dp rail: Home (active 56×32) / Search / Profile |
| `catalog_datepicker_v7.png` | Range + modal calendars with month `<` `>` |
| `catalog_text_fields_v7.png` | Empty filled/outlined + notched Email (legend on the top stroke) |
| `catalog_buttons_v7.png` | Expressive overview buttons + XS–XL sizes |
| `compare_desktop_vs_official_{timepicker,slider,text_fields,datepicker,buttons,dialog,search}_v7.png` | Official m3.material.io (left) vs **live GPUI** (right) |

Host tests: `gpui_material` 26, `gpui_android` 10, `material_desktop_demo` 1.
