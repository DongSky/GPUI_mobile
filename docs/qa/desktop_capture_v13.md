# v13 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Binary:** `target/debug/material_desktop_demo` after the v13 JNI-sink / measured-notch / LineCap-stroke / MorphLayerTransform pass.

Live GPUI catalog frames from v12 remain the scene screenshots (`docs/qa/catalog_*_v12.png`). v13 visual deltas are the circular/PTR **stroked** round-cap arc (replacing the filled sausage) and HTML `data-progress-stroke="round"`. Official vs GPUI strips added this pass:

- `compare_desktop_vs_official_buttons_v13.png`
- `compare_desktop_vs_official_search_v13.png`
- `compare_desktop_vs_official_timepicker_v13.png`

(Left: headless Chrome on m3.material.io; right: live GPUI v12 frames.) Text-field and progress v12 strips still apply.

Android `component_demo` debug APK **builds** (NDK 25.1). No AVD/device attached, so install/run is leftover. NativeActivity `hasCode=false` — live `JNIEnv` `CallVoidMethod` still needs a VM attach from `ANativeActivity`.

Host tests: `gpui_material` **26**, `gpui_android` **16** (JNIEnv sink + attach slot), `material_desktop_demo` **1**.

## What v13 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| IME | unchanged caret | `JniEnvSink` / `RecordingJniSink` / `flush_ime_jni_queue`, `JniNativeMethodDesc`, `attach_jni_env` + `flush_if_attached` from `update_ime_position`. Dummy non-null env counts calls; no live JVM |
| Search | `MorphLayerTransform`, `TRANSFORM_ORIGIN` (`top center`), `morph_layer_css` | HTML `data-search-transform-origin`; GPUI still uses `morph_scaled_margin_dp` (no div layer transform) |
| Notch | `roboto_advance_em`, `measured_label_width_dp`, `notch_width_from_measured_dp` | Same live cubics; hosts can pass a measured label width |
| LineCap | Circular/PTR HTML is a **stroked** `ptr_arc` with `stroke-linecap=round` | Desktop/Android paint `ptr_arc_polyline` via lyon `LineCap::Round` |
| Nav rail | `OsPopupSpec` (`WindowKind::PopUp` name, 220dp, `supported_on_android: false`) | HTML `data-rail-os-popup="0"` |

## v14 leftovers

See `docs/qa/v13_leftovers.md`.
