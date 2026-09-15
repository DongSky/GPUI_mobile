# v14 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Binary:** `target/debug/material_desktop_demo` after the v14 NativeActivity JNI / PathBuilder search scale / layout-line notch / StrokeCap / PopUp-options pass.

Live GPUI catalog frames from v12 remain the scene screenshots (`docs/qa/catalog_*_v12.png`). Official vs GPUI strips added this pass:

- `compare_desktop_vs_official_datepicker_v14.png`
- `compare_desktop_vs_official_nav_rail_v14.png`
- `compare_desktop_vs_official_slider_v14.png`

(Left: headless Chrome on m3.material.io; right: live GPUI v12 frames.) v12 text-fields/progress and v13 buttons/search/time-picker strips still apply.

Android `component_demo` debug APK **builds** (NDK 25.1). No AVD/device attached, so install/run is leftover. NativeActivity stores `JavaVM*` + activity jobject on `InitWindow` / `open_window` and flushes `FindClass` + `CallVoidMethodA(toggleSoftInput)` plus dummy `View` / `BaseInputConnection` `NewObject`. There is still no `hasCode=true` `RegisterNatives` peer.

Host tests: `gpui_material` **26**, `gpui_android` **17** (NativeActivity attach dry-run + View/IC plan), `material_desktop_demo` **1**.

## What v14 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| IME | unchanged caret | `attach_native_activity` / `flush_native_activity_imm`; Android `FindClass(Context/IMM/View)` + `getSystemService("input_method")` + `toggleSoftInput` + dummy `new View` / `new BaseInputConnection`; host dry-run when both pointers are set |
| Search | `top_center_scale_translates` / `path_scale_origin_dp` / `morph_layer_box`; HTML `data-search-path-scale` + `data-search-layer-box` | GPUI paints container fill with `PathBuilder::scale` about `top center`; layout uses `morph_layer_box` (height + x inset) |
| Notch | `notch_frame_from_layout` | Outlined-field canvas **prepaint** calls `text_system().layout_line`; Roboto-advance fallback if width ≤ 1 |
| StrokeCap | `progress::StrokeCap::Round` / `STROKE_CAP` | Hosts map to lyon `LineCap::Round`; HTML `data-stroke-cap="round"` |
| Nav rail | `GPUI_WINDOW_KIND` (`PopUp`), `OS_POPUP_OPENED=false`, `os_popup_window_options` | Desktop builds `WindowOptions { kind: WindowKind::PopUp }` in tests only — not opened (Linux ignores `kind`; NativeActivity is one window) |
| Time / carousel | `SECOND_HAND_FRAME_MS` = 16; `FlingState::needs_frame` | Second-hand animation ~60 Hz; hosts `cx.notify()` while flinging |

## v15 leftovers

See `docs/qa/v14_leftovers.md`.
