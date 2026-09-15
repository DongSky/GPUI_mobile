# v23 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v22** (`feat: v22 Expressive tooltips, flexible nav bar`). Critique vs [m3.material.io Expressive](https://m3.material.io/develop/flutter/overview).

**v22 landed on `main` only (no PR).** Start from that tip; do not re-open plain/rich tooltips, flexible 64dp nav (vertical + horizontal), medium/large flexible collapse, search app bar, Bloom album hero, or standard/modal/detached side sheets as missing inventory — they ship. Navigation drawer and **banner** stay **out** of inventory: Expressive replaces the drawer with the expanded rail; banners are not on the current component list.

Highest-impact remaining gaps:

1. **Camera / licensed stills** — catalog photos are still procedural JPEGs (decode-verified). Official overviews use real photographs. A small bundled asset set (with license) would close the last visual gap.
2. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
3. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
4. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
5. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
6. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
7. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
8. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including tooltip + flexible nav + v21 app-bar / side-sheet) when a display/screenshot pipeline works.
9. **Tooltip caret / hover** — rich tooltip has no caret triangle; catalog heroes are persistent, not hover / long-press. Next polish if photos and IME stay deferred.

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated), **banner** (not on current site), plain/rich **tooltip**, flexible **64dp** nav (vertical + horizontal).

## Suggested v23 order

1. Licensed camera stills (or tooltip caret / hover if photos stay deferred).
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture of v21–v22 heroes.

Host tests: `cargo test -p gpui_material` (28), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
