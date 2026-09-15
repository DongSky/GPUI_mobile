# v21 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v20** (`feat: v20 decoded JPEG media, snackbar peek, Your lists`). Critique vs [m3.material.io Expressive](https://m3.material.io/develop/flutter/overview).

**v20 landed on `main` only (no PR).** Start from that tip; do not re-open JPEG encode/decode, `data-decoded-jpeg`, snackbar peek row, filled dest SVGs, or carousel `Your lists` phone chrome as missing inventory — they ship. Catalog photos are **procedural JPEGs** (decode-verified), not licensed camera stills.

Highest-impact remaining gaps:

1. **Camera / licensed stills** — `photo_stub` encodes and decodes tiny procedural JPEGs (grain, portraits, landscapes, basket, mugs, dog). Official overviews still use real photographs. A small bundled asset set (with license) would close the last visual gap.
2. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
3. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
4. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
5. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
6. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
7. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
8. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes and `material_desktop_demo` when a display/screenshot pipeline works.
9. **Collapsing app bars / navigation drawer** — inventory top app bar is small-only; nav drawer / side sheet are not in `INVENTORY`. Next Expressive surfaces if scene photos stay deferred.

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / `decode_rgb` / `data_uri` / mosaic, snackbar peek + dest SVGs, uncontained-multi **layout**, Your lists **chrome**, split/FAB-menu/toolbar token modules, Gmail dest labels, Audio-selected saved-media.

## Suggested v21 order

1. Licensed camera stills (or collapsing app bar / nav drawer if photos stay deferred).
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture.

Host tests: `cargo test -p gpui_material` (28), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
