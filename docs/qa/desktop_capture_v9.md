# v9 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Branch:** `cursor/material3-component-parity-d068`  
**Binary:** `target/debug/material_desktop_demo` rebuilt after v9.

## Live window — **succeeded**

Same lavapipe recipe as v8 (`scripts/desktop-screenshot.sh`, `DISPLAY=:1`, `720×880` at `1200,85`).

`docs/qa/catalog_*_v9.png`, `desktop_gpui_live_v9.png`, and the **right** panels of `compare_desktop_vs_official_*_v9.png` are **live GPUI pixels**.

HTML catalog (`docs/catalog/material-catalog-*.html`) shares `gpui_material::resolve()`. Android `component_demo` maps the same polar clock, hole-notch path, search editor, range paint, PTR arc, rail selection, and carousel snap (NDK not present in this VM so Android was not rebuilt here).

## What v9 PNGs show

| File | Content (live GPUI) |
|---|---|
| `desktop_gpui_live_v9.png` / `catalog_settings_scene_v9.png` | Settings scene, notched Email, Day–Week–Month |
| `catalog_buttons_v9.png` | Expressive buttons + empty fields start |
| `catalog_text_fields_v9.png` / `catalog_slider_v9.png` | Hole-notched Email; **Price range 20–75%** absolute thumbs |
| `catalog_progress_v9.png` / `catalog_carousel_v9.png` | Wavy + **spinning PTR arc**; carousel One–Four (click/wheel snap) |
| `catalog_nav_rail_v9.png` / `catalog_search_v9.png` | **Expanded rail** (`←` FAB, selectable dests, badges); search activity with **caret `\|`** |
| `catalog_timepicker_v9.png` | Polar minute dial + path-drawn hand at 30 |
| `catalog_datepicker_v9.png` / `catalog_datepicker_modal_v9.png` | Range calendar + docked field + modal |
| `compare_desktop_vs_official_{timepicker,slider,text_fields,datepicker,buttons,search}_v9.png` | Official m3.material.io (left) vs live GPUI v9 (right) |

Host tests: `gpui_material` 26, `gpui_android` 10, `material_desktop_demo` 1.

## What v9 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| Android time | existing polar offsets + `hand_quad` | Android catalog uses the same clock as desktop (no wrap-grid) |
| Notch | `hole_rect` / `notch_gap_rect` + `ime_cursor_origin_dp` | Outer outline fill + inner hole + legend gap + path stroke |
| Search | `apply_key_to_editor`, caret, `pick_suggestion` | GPUI `TextFieldEditor`; HTML `<input>` + suggestion tap |
| Range | `range_paint` absolute boxes | No flex min-width quantization during drag; 5% click/keys kept |
| Progress | `ptr_arc_polyline` | GPUI `Animation` spins the PTR arc; HTML CSS `m3spin` |
| Nav rail | `select_destination` / `toggle_mode` | One interactive rail; FAB morphs collapsed↔expanded |
| Carousel | `advance` / `snap_to` / `fling_step` | Click tile + `on_scroll_wheel` |
