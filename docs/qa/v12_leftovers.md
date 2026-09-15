# v13 leftovers (after v12 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

1. **JNI InputConnection on device** — Host tables, `ImeSession::jni_imm_calls()`, packed `CursorAnchorInfoPayload`, and `dispatch_native_input_connection()` are ready. NativeActivity still has no live `JNIEnv` `toggleSoftInput` / `RegisterNatives` / View-backed `InputConnection`.
2. **Search pixel-perfect** — leading icon/back + avatar crossfade with `MorphFrame`. Remaining: Compose SearchBar container transform is still an approximation (scale is not applied as a GPUI layer transform).
3. **Outlined notch** — Live even-odd path is RoundedPolygon cubics (`CIRCULAR_KAPPA`). Remaining: lyon `LineCap` if hosts want a stroked centerline; glyph-advance width can still leave a sliver under wide letters.
4. **gpui `LineCap::Round`** — Circular/PTR use a filled sausage; wavy uses endpoint discs. Switch to a real stroke cap if/when gpui re-exports `LineCap::Round`.
5. **Nav rail OS window** — Expanded rail is an in-window overlay (`#nav-rail-window` + scrim), not a second GPUI `Window` / Android popup. NativeActivity is single-window.
6. **Android APK** — NDK not in this VM; `component_demo` typechecks only. Build + install when `ANDROID_HOME` / NDK 25.1 is present.
7. **Official side-by-side** — last compare strips are `compare_desktop_vs_official_*_v9.png`. Headless Chrome timed out on m3.material.io this pass; recapture `compare_desktop_vs_official_*_v12.png` (or v13) when capture works.
