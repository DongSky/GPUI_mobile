# v17 leftovers (after v16 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

v16 on `main` now includes full-screen Event dialog, connected icon row + overflow menu, carousel hero/multi-browse/uncontained media tiles + parallax, snackbar 4s/10s timeout + 72dp swipe dismiss, icon+label primary tabs, standard (non-modal) sheet, `NativeInputConnection` `native*` methods + session handle (`RegisterNatives` plan, not BaseIC overrides), and official snackbar / bottom-sheet / tabs / dialog / carousel / button-group strips.

1. **RegisterNatives function pointers** — Java `native*` methods and a session handle exist. Host dry-run walks `FindClass` + `GetStaticMethodID` + `RegisterNatives`. A live JVM still needs `JNINativeMethod.fnPtr` (or JNI-mangled `Java_dev_gpui_…` exports) bound before IME keystrokes reach `ImeSession`.
2. **GPUI public caret bounds** — Catalog computes `catalog_ime_from_editor` and hosts store `last_catalog_ime`. `Window` still does not expose caret bounds; wire `update_ime_position` from a real GPUI text input when that API exists.
3. **Search shared-element layer transform** — PathBuilder scale interpolates with `with_animation`. GPUI `div` still has no CSS `transform` / clip-path; HTML uses a real CSS scale. Optional: upstream element transform.
4. **gpui `LineCap::Round` re-export** — Hosts map `progress::StrokeCap` to lyon `LineCap` until upstream gpui re-exports the enum.
5. **Nav rail OS window** — `WindowOptions { kind: WindowKind::PopUp }` is constructed and unit-tested. Do not `open_window` it on Linux (`WindowParams.kind` unused) or NativeActivity (single window).
6. **Carousel photos / remaining layouts** — Three layouts + role-color media + parallax offset. Official still uses real photos, phone-frame mask, and additional layouts (centered / full-screen). Stub is not bitmap media.
7. **Snackbar in-scene + close** — Timeout/swipe runtime exists. Official hero is an in-app “Email archived” bar with Action + close over a mail list. Catalog is a standalone Retry bar.
8. **Tabs in-app media scene** — Icon+label 64dp row exists. Official overview is a phone “My saved media” scene (Video / Photos / Audio) over photo tiles.
9. **Android APK run** — Debug APK should still build (NDK 25.1 + `hasCode=true` + native IME peer). Remaining: emulator/device install + screenshot (no AVD / no SDK in this VM).
10. **Live GPUI recapture** — This VM cannot create a wgpu surface. Keep v12 live frames until lavapipe/Vulkan ICD works. v16 official strips use HTML catalog heroes on the right.
