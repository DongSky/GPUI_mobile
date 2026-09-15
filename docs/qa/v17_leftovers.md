# v18 leftovers (after v17 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

v17 on `main` now includes Inbox snackbar scene (Email archived + Undo + close), My saved media tabs phone scene, share sheet over a photo grid, carousel centered-hero + full-screen + phone-frame mask, and live `JNINativeMethod.fnPtr` / JNI-mangled `Java_dev_gpui_material_NativeInputConnection_native*` exports (`RegisterNatives` binds the table on device).

1. **Live JVM keystroke path** — Host tests call the mangled C ABI with a C-string payload. A device JVM still has to `GetStringUTFChars` a real `jstring` and feed IME keystrokes into the bound `ImeSession`. Confirm on an emulator after `RegisterNatives`.
2. **GPUI public caret bounds** — Catalog computes `catalog_ime_from_editor` and hosts store `last_catalog_ime`. `Window` still does not expose caret bounds; wire `update_ime_position` from a real GPUI text input when that API exists.
3. **Search shared-element layer transform** — PathBuilder scale interpolates with `with_animation`. GPUI `div` still has no CSS `transform` / clip-path; HTML uses a real CSS scale. Optional: upstream element transform.
4. **gpui `LineCap::Round` re-export** — Hosts map `progress::StrokeCap` to lyon `LineCap` until upstream gpui re-exports the enum.
5. **Nav rail OS window** — `WindowOptions { kind: WindowKind::PopUp }` is constructed and unit-tested. Do not `open_window` it on Linux (`WindowParams.kind` unused) or NativeActivity (single window).
6. **Carousel bitmaps / uncontained multi-aspect** — Five layouts + phone-frame mask + role-color media. Official still uses real photos and an uncontained multi-aspect-ratio layout. Stub is not bitmap media.
7. **Snackbar mail chrome** — Inbox + Undo + close exists. Official still has avatars, timestamps, and a bottom nav on the phone. Catalog mail rows are text-only.
8. **Tabs / sheet photo media** — Saved-media tiles and share-grid use role-color stubs. Official uses camera photos, status-bar chrome, and a people row on the share sheet.
9. **New Expressive components** — FAB menu, split button, and floating toolbars are on m3.material.io and not in `INVENTORY`.
10. **Android APK run** — Debug APK should still build (NDK 25.1 + `hasCode=true` + JNI-mangled IME peer). Remaining: emulator/device install + screenshot (no AVD / no SDK in this VM).
11. **Live GPUI recapture** — This VM cannot create a wgpu surface. Keep v12 live frames until lavapipe/Vulkan ICD works. v17 official strips use HTML catalog heroes on the right.
