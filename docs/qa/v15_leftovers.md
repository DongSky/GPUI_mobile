# v16 leftovers (after v15 follow-up on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

v15 on `main` now includes live `CursorAnchorInfo.Builder` + `CallVoidMethod(updateCursorAnchorInfo)`, `NativeInputConnection` Java peer (`hasCode=true`), catalog caret → IMM queue, search `with_animation` PathBuilder scale, notch lyon centerline stroke, shared `FRAME_MS`/`FRAME_DT` clocks, richer rail `OsPopupSpec`, carousel leftover snap + width interpolation, and official dialog/carousel/button-group strips.

1. **RegisterNatives live bind** — `dev.gpui.material.NativeInputConnection` is in the APK and constructed on a live JVM. Binding `commitText` / `setComposingText` still needs `native` methods (or an AAR) plus a session handle. Do not `RegisterNatives` onto `BaseInputConnection` methods that are not `native`.
2. **GPUI public caret bounds** — Catalog computes `catalog_ime_from_editor` and hosts store `last_catalog_ime`. `Window` still does not expose caret bounds; wire `update_ime_position` from a real GPUI text input when that API exists.
3. **Search shared-element layer transform** — PathBuilder scale now interpolates with `with_animation`. GPUI `div` still has no CSS `transform` / clip-path; HTML uses a real CSS scale. Optional: upstream element transform.
4. **gpui `LineCap::Round` re-export** — Hosts map `progress::StrokeCap` to lyon `LineCap` until upstream gpui re-exports the enum.
5. **Nav rail OS window** — `WindowOptions { kind: WindowKind::PopUp, title, 880dp }` is constructed and unit-tested. Do not `open_window` it on Linux (`WindowParams.kind` unused) or NativeActivity (single window). Screenshot both windows on a compositor that honors `PopUp`.
6. **Carousel media / multi-browse** — Physics now snap leftover and lerp hero/neighbor widths. Official m3.material.io still uses photo tiles, parallax, and six layouts (multi-browse, uncontained, hero, …). Stub remains solid-color items.
7. **Dialog full-screen + button-group icon row** — Official dialogs show a basic + full-screen pair; catalog is Reset-settings / ringtone lists. Official button groups show a connected icon row + overflow; catalog is Day/Week/Month text.
8. **Android APK run** — Debug APK should still build (NDK 25.1 + `hasCode=true` Java). Remaining: emulator/device install + screenshot (no AVD / no SDK in this VM).
9. **Live GPUI recapture** — This VM cannot create a wgpu surface. Keep v12 live frames until lavapipe/Vulkan ICD works. v15 official strips use HTML catalog heroes on the right.
10. **Official side-by-side** — v12 text-fields/progress, v13 buttons/search/time-picker, v14 date-picker/nav-rail/slider, v15 dialog/carousel/button-group. Optional: snackbar / bottom sheet / tabs.
