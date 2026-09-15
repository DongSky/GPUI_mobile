# v15 leftovers (after v14 follow-up on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

v14 on `main` now includes NativeActivity `CallVoidMethodA(toggleSoftInput)`, dummy `View` + `BaseInputConnection` `NewObject`, search `PathBuilder::scale` + `morph_layer_box` layout, GPUI `layout_line` notch, 16ms second hand, fling `needs_frame` notify, and official datepicker/nav-rail/slider strips.

1. **RegisterNatives Java peer + CursorAnchorInfo objects** — Dummy `View` / `BaseInputConnection` are constructed on a live JVM. Binding `dev.gpui.material.NativeInputConnection` still needs `hasCode=true` (or an AAR). `updateCursorAnchorInfo` still has no real `CursorAnchorInfo` jobject.
2. **Catalog `update_ime_position` from focused caret** — `Window` still does not expose caret bounds. Wire when GPUI adds a public API, or compute from `TextFieldState` + editor layout.
3. **Search `with_animation` PathBuilder scale** — Layout uses `morph_layer_box`; container fill uses settled `PathBuilder::scale`. Interpolating the path scale with `with_animation` is still leftover.
4. **Notch lyon centerline** — Optional: stroke the outline centerline with lyon instead of (or in addition to) the even-odd C-path fill.
5. **gpui `LineCap::Round` re-export** — Hosts map `progress::StrokeCap` to lyon `LineCap` until upstream gpui re-exports the enum.
6. **Nav rail OS window** — `WindowOptions { kind: WindowKind::PopUp }` is constructed and unit-tested. Do not `open_window` it on Linux (`WindowParams.kind` unused) or NativeActivity (single window). Screenshot both windows on a compositor that honors `PopUp`.
7. **Android APK run** — Debug APK builds. Remaining: emulator/device install + screenshot (no AVD in this VM).
8. **Live GPUI recapture** — This VM cannot create a wgpu surface. Keep v12 live frames until lavapipe/Vulkan ICD works.
9. **Official side-by-side** — v12 text-fields/progress, v13 buttons/search/time-picker, v14 date-picker/nav-rail/slider. Optional: dialog / carousel / button-group.
