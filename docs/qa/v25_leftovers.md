# v26 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v25** (`feat: v25 Expressive icon-button narrow/default/wide`). Critique vs [m3.material.io](https://m3.material.io/components/icon-buttons/specs) Expressive.

**v25 landed on `main` only (no PR).** Start from that tip; do not re-open icon-button narrow/default/wide (MDC leading/trailing S 4/8/14, M 12/16/24; containers 32/40/52 and 48/56/72) as missing inventory — they ship. Navigation drawer and **banner** stay **out** of inventory.

Highest-impact remaining gaps:

1. **Camera / licensed stills** — catalog photos are still procedural JPEGs (decode-verified). Official overviews use real photographs. A small bundled asset set (with license) would close the last visual gap.
2. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
3. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
4. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
5. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
6. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
7. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
8. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including icon-button widths) when a display/screenshot pipeline works.
9. **Toggle icon buttons** — selected color + round↔square morph (Compose `IconToggleButton`) is still missing. Standard button-group overflow on the 12dp row, list swipe-as-LazyColumn fling, and predictive-back stay unimplemented.

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated), **banner** (not on current site), plain/rich **tooltip** + **caret** + **hover/long-press**, flexible **64dp** nav, **segmented lists**, **standard + connected** button groups, list **swipe rails** + **drag-handle reorder**, icon-button **narrow/default/wide**.

## Suggested v26 order

1. Licensed camera stills (or toggle icon buttons if photos stay deferred).
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture of v21–v25 heroes.

Host tests: `cargo test -p gpui_material` (29), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
