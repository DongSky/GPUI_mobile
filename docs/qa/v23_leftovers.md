# v24 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v23** (`feat: v23 Expressive segmented lists, tooltip carets`). Critique vs [m3.material.io](https://m3.material.io/components/lists/specs) Expressive.

**v23 landed on `main` only (no PR).** Start from that tip; do not re-open segmented lists (2dp gap, 4/16 corners, selected secondary-container, Wi-Fi hero) or tooltip 16×8 carets as missing inventory — they ship. Navigation drawer and **banner** stay **out** of inventory: Expressive replaces the drawer with the expanded rail; banners are not on the current component list. Baseline lists remain available but are not recommended.

Highest-impact remaining gaps:

1. **Camera / licensed stills** — catalog photos are still procedural JPEGs (decode-verified). Official overviews use real photographs. A small bundled asset set (with license) would close the last visual gap.
2. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
3. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
4. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
5. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
6. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
7. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
8. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including segmented lists + tooltip carets + v21–v22 chrome) when a display/screenshot pipeline works.
9. **Tooltip hover / long-press** — caret ships; catalog heroes are still persistent, not hover / long-press. List drag-reorder / swipe remain unimplemented.

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated), **banner** (not on current site), plain/rich **tooltip** + **caret**, flexible **64dp** nav (vertical + horizontal), **segmented lists**.

## Suggested v24 order

1. Licensed camera stills (or list swipe / tooltip hover if photos stay deferred).
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture of v21–v23 heroes.

Host tests: `cargo test -p gpui_material` (28), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
