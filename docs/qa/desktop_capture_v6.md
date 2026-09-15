# v6 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Branch:** `cursor/material3-component-parity-d068`  
**Binary:** `target/debug/material_desktop_demo` rebuilt after v6 (`cargo build -p material_desktop_demo`).

## Live window

A real Desktop GPUI window still **does not open** on this VM.

- `/dev/dri` is missing (no GPU / render node).
- `WGPU_BACKEND=gl` + `LIBGL_ALWAYS_SOFTWARE=1` + Mesa `swrast` / `llvmpipe` still fail in Zed GPUI’s `create_surface_unsafe` with `Failed to create surface for any enabled backend: {}`.
- System `libEGL` is absent (extracted `/tmp/egl` is not enough for wgpu/GLX).
- Lavapipe (`mesa-vulkan-drivers` + `libLLVM.so.21.1` + `libdisplay-info.so.3`) then failed on `libxml2.so.16` — the extracted Mesa 26 ICD is newer than this Ubuntu userland. No root/`apt` to install the chain.

Do **not** treat the right-hand `compare_desktop_vs_official_*_v6.png` panels as GPUI pixels. They are the **HTML catalog** generated from the same `gpui_material::resolve()` recipes the desktop/Android demos paint.

Roboto TTFs were installed to `~/.local/share/fonts` (`fc-cache`); `typography::desktop_font_family()` will return `Roboto` when a window can open.

## What v6 PNGs show

| File | Content |
|---|---|
| `compare_desktop_vs_official_{text_fields,slider,dialog,datepicker,buttons,search,timepicker}_v6.png` | Official m3.material.io (left) vs HTML catalog (right) |
| `catalog_settings_scene_v6.png` | Sound & notifications: Volume / Quiet hours, notched Email, Day–Week–Month |
| `catalog_search_v6.png` | Docked 56dp search bar + time picker |
| `catalog_timepicker_v6.png` | 12-hour dial, 6:30 PM, selected hour 6 |
| `catalog_datepicker_v6.png` | Docked DOB field + anchored calendar |
| `catalog_slider_v6.png` | Volume rows + dual-thumb Price range |

Host tests: `gpui_material` 26, `gpui_android` 10, `material_desktop_demo` 1.
