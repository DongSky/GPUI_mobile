# v15 leftovers (after v14 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

1. **View-backed InputConnection** — NativeActivity now attaches `JavaVM*` + activity and can `FindClass` / `CallVoidMethodA(toggleSoftInput)`. Remaining: a Java `View` + `InputConnection` (NativeActivity is `hasCode=false`), `updateCursorAnchorInfo` object construction, and a real device IME round-trip.
2. **Search GPUI layer transform** — Container fill uses `PathBuilder::scale` about `top center` at the settled morph `t`. `div` still has no element scale; morph color/height animate on the parent; PathBuilder scale does not interpolate with `with_animation`.
3. **Notch lyon centerline** — Notch width comes from GPUI `layout_line` (Roboto-advance fallback). Optional: stroke the outline centerline with lyon instead of (or in addition to) the even-odd C-path fill.
4. **gpui `LineCap::Round` re-export** — Hosts map `progress::StrokeCap` to lyon `LineCap` until upstream gpui re-exports the enum.
5. **Nav rail OS window** — `WindowOptions { kind: WindowKind::PopUp }` is constructed and unit-tested. Do not `open_window` it on Linux (`WindowParams.kind` unused) or NativeActivity (single window).
6. **Android APK run** — Debug APK builds. Remaining: emulator/device install + screenshot (no AVD in this VM).
7. **Official side-by-side** — v12 text-fields/progress, v13 buttons/search/time-picker, v14 date-picker/nav-rail/slider strips exist. Optional: dialog / carousel / button-group.
