# v22 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v21** (`feat: v21 Expressive flexible app bars, side sheets`). Critique vs [m3.material.io Expressive](https://m3.material.io/develop/flutter/overview).

**v21 landed on `main` only (no PR).** Start from that tip; do not re-open medium/large flexible collapse, search app bar, Bloom album hero, or standard/modal/detached side sheets as missing inventory — they ship. Navigation drawer stays **out** of inventory: Expressive replaces it with the expanded rail.

Highest-impact remaining gaps:

1. **Camera / licensed stills** — catalog photos are still procedural JPEGs (decode-verified). Official overviews use real photographs. A small bundled asset set (with license) would close the last visual gap.
2. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
3. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
4. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
5. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
6. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
7. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
8. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including new app-bar / side-sheet) when a display/screenshot pipeline works.
9. **Tooltip / banner** — not in `INVENTORY`. Next Expressive surfaces if photos and IME stay deferred. Flexible bottom app bar is specified as docked toolbar (already shipped).

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated).

## Suggested v22 order

1. Licensed camera stills (or tooltip / banner if photos stay deferred).
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture of v21 heroes.

Host tests: `cargo test -p gpui_material` (28), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
