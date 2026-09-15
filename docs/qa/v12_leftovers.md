# v13 leftovers (after v12 follow-up on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

1. **JNI InputConnection on device** — `ImeJniQueue`, `dry_run_jni_env()`, RegisterNatives table, and `AndroidWindowInner.pending_jni` are ready. NativeActivity still has no live `JNIEnv` to `CallVoidMethod(toggleSoftInput)` or a View-backed `InputConnection`.
2. **Search pixel-perfect** — GPUI applies `morph_scaled_margin_dp` (inset + centered 0.94→1.0 scale stand-in). Remaining: a real GPUI layer transform matching CSS `transform: scale`.
3. **Outlined notch** — Live even-odd path is RoundedPolygon cubics via `cubic_bezier_to`; width includes `NOTCH_WIDTH_SAFETY_DP`. Remaining: measure the painted label instead of glyph heuristics; optional lyon stroked centerline.
4. **gpui `LineCap::Round` re-export** — Wavy hosts import `lyon::tessellation::LineCap` directly. Circular/PTR still use a filled sausage. Switch to gpui's own re-export if/when it lands upstream.
5. **Nav rail OS window** — Popup-*kind* overlay (`RailChrome::Popup`, `data-rail-chrome="popup"`). Not a second `gpui::Window` / Android popup. NativeActivity is single-window.
6. **Android APK run** — Debug APK **builds** (`scripts/build.sh`, NDK 25.1). Remaining: emulator/device install + screenshot when an AVD is available.
7. **Official side-by-side** — `compare_desktop_vs_official_text_fields_v12.png` and `compare_desktop_vs_official_progress_v12.png` landed. Optional: buttons / search / time-picker strips.
