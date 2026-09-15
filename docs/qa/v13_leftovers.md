# v14 leftovers (after v13 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

1. **Live JNIEnv** — `JniEnvSink`, `flush_ime_jni_queue`, `attach_jni_env`, and `flush_if_attached` are ready. NativeActivity still needs the real `ANativeActivity.env` / `clazz` wired to `FindClass` + `CallVoidMethod(toggleSoftInput)` and a View-backed `InputConnection`.
2. **Search GPUI layer transform** — `MorphLayerTransform` + CSS `top center` origin exist. GPUI `div` still has no element scale; hosts keep `morph_scaled_margin_dp`.
3. **Notch measured from layout** — `notch_width_from_measured_dp` is the API. Hosts still estimate with `roboto_advance_em` instead of GPUI text-layout width. Optional lyon stroked centerline.
4. **gpui `LineCap::Round` re-export** — Hosts import `lyon::tessellation::LineCap` directly. Upstream gpui still does not re-export the enum.
5. **Nav rail OS window** — `OsPopupSpec` records `WindowKind::PopUp`. Desktop catalog does not call `cx.open_window` for the rail; NativeActivity cannot.
6. **Android APK run** — Debug APK builds. Remaining: emulator/device install + screenshot (no AVD in this VM).
7. **Official side-by-side** — v12 text-fields/progress + v13 buttons/search/time-picker strips exist. Optional: date-picker / nav-rail / slider.
