# v15 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Binary:** `target/debug/material_desktop_demo` after the v15 CursorAnchorInfo / NativeInputConnection / search path-scale clock / notch centerline / shared vsync / rail popup-spec / carousel snap pass.

Live GPUI catalog frames from v12 remain the scene screenshots (`docs/qa/catalog_*_v12.png`). Official vs HTML-catalog strips added this pass (left: headless Chrome on m3.material.io; right: HTML catalog heroes from the same `resolve()` the GPUI hosts paint):

- `compare_desktop_vs_official_dialog_v15.png`
- `compare_desktop_vs_official_carousel_v15.png`
- `compare_desktop_vs_official_button_group_v15.png`

v12 text-fields/progress, v13 buttons/search/time-picker, and v14 date-picker/nav-rail/slider strips still apply.

## Critique vs current m3.material.io Expressive

| Official | Catalog / GPUI | Gap |
|---|---|---|
| Search bar → view is a shared-element container scale about top-center | Same tokens; GPUI now interpolates `PathBuilder::scale` on the `with_animation` clock (not only the settled end state) | No GPUI layer transform / clip-path |
| Outlined field is a stroked rounded-rect with a legend gap | Even-odd C-path fill **plus** lyon centerline stroke (`LineCap::Round` at the cutout) | Stroke + fill can read slightly heavier than Compose |
| Time picker dial + analog hand | Same, plus a 16 ms wall-clock second hand (`motion::FRAME_MS`) | Second hand is extra vs the official hero |
| Nav rail expanded is a modal 220 dp column | Overlay + `OsPopupSpec` (titled `PopUp`, 880 dp) — not opened | Linux ignores `WindowKind`; NativeActivity is one window |
| Carousel: photo tiles, parallax, six layouts | Hero/neighbor width lerp + leftover snap-to-nearest | Stub colors; no media / parallax / multi-browse mask |
| Dialogs: basic + full-screen | Reset-settings list + ringtone list | No full-screen dialog |
| Button groups: connected icons + overflow | Day/Week/Month connected text | No icon-row / overflow menu |
| Text fields show the system IME | Live JNI `toggleSoftInput` + `CursorAnchorInfo.Builder` + `NativeInputConnection` (`hasCode=true`) | No `RegisterNatives` bind; catalog caret is computed, not a GPUI `Window` API |

Android `component_demo` debug APK **should build** (NDK 25.1, `hasCode=true` Java peer). No AVD/device/SDK in this VM, so install/run is leftover. NativeActivity stores `JavaVM*` + activity jobject and flushes `FindClass` + `CallVoidMethodA(toggleSoftInput)` + `View` + `NativeInputConnection` (BaseIC fallback) + live `CursorAnchorInfo` / `updateCursorAnchorInfo`.

Host tests: `gpui_material` **26**, `gpui_android` **17**, `material_desktop_demo` **1**.

## What v15 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| IME | `catalog_ime_from_editor` / `CATALOG_FIELD_ORIGIN_DP` | Android `CursorAnchorInfo$Builder` + `CallVoidMethod(updateCursorAnchorInfo)`; Java `dev.gpui.material.NativeInputConnection`; `hasCode=true`; hosts store `last_catalog_ime` |
| Search | `morph_path_scale` / `morph_path_scale_eased`; HTML `data-search-anim-scale` | GPUI canvas fill scale follows the morph `with_animation` clock |
| Notch | `centerline_polyline` / `centerline_svg_d`; HTML `data-notch-centerline` | Hosts stroke the outline centerline with lyon `LineCap::Round` on top of the even-odd C-path |
| Motion | `FRAME_MS` = 16, `FRAME_DT` = 1/60 | Time-picker second hand + carousel fling share one vsync token |
| Nav rail | `OsPopupSpec` title / height / focus / movable | Desktop `WindowOptions` uses the spec (still not `open_window`) |
| Carousel | `settle`, `snap_offset_t`, `item_width_during_fling`, `FLING_SNAP_FRACTION` | Hosts lerp tile widths while flinging; leftover ≥ ½ unit snaps |

## v16 leftovers

See `docs/qa/v15_leftovers.md`.
