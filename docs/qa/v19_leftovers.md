# v20 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v19** (`feat: v19 Expressive scene chrome, photo stubs, uncontained-multi carousel`). Critique vs [m3.material.io Expressive](https://m3.material.io/develop/flutter/overview).

**v19 landed on `main` only (no PR).** Start from that tip; do not re-open photo-stub scene chrome, Mail/Chat/Rooms/Meet dest labels, enamel-mugs / basket / chat heroes, Audio-selected tabs, share-sheet named people, or carousel `uncontained-multi` as missing inventory — they ship as CSS/GPUI gradient stubs (no JPEG decoder).

Highest-impact remaining gaps:

1. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. `docs/qa/ime_jstring_spike.md` is the next JNI change (needs NDK + `jni-sys`).
2. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. `gpui-component/input.rs` still uses a local `fnPtr` workaround for `set_ime_exclusion_area`; a real GPUI method needs a crate bump.
3. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
4. **`gpui::LineCap`** — `progress.rs` documents `LineCap::Round` as a gpui re-export. Verify after the next gpui bump.
5. **Nav rail `PopUpMenu`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
6. **Decoded photo assets** — snackbar avatars, tabs Bloom/Egret, share-sheet people, carousel tiles, basket/mugs/dog scenes are layered CSS/GPUI gradients (`photo_stub.rs`), not camera JPEGs. Official overviews use real photographs. Needs an image crate + a tiny bundled asset set.
7. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
8. **Live GPUI recapture** — `docs/qa/compare_desktop_vs_official_*_v19.png` is **headless Chrome catalog HTML vs m3.material.io** (official left halves reused from v18/v17 strips). Recapture the `material_desktop_demo` window when a display is available.

Do **not** reopen as missing: small FAB 56/80, nav-rail overlay, search-view tokens, loading indicator, split button / FAB menu / floating toolbar **token modules**, Gmail dest labels, Audio-selected saved-media, share-sheet Send people, uncontained-multi carousel **layout**.

## Suggested v20 order

1. Bundled JPEG decode for catalog + GPUI hosts (replace `PhotoKind::css_background` / `fill()` stubs).
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + live desktop wgpu recapture.

Host tests: `cargo test -p gpui_material` (27), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
