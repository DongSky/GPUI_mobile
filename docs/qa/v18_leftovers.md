# v19 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v18** (`feat: v18 FAB menu, split button, floating toolbar, inbox/tabs/sheet chrome`). Critique vs [m3.material.io Expressive](https://m3.material.io/develop/flutter/overview).

**v18 landed on `main` only (no PR).** Start from that tip; do not re-open FAB menu / split button / floating toolbar as missing inventory — they ship with androidx small/full-round tokens.

Highest-impact remaining gaps:

1. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. `docs/qa/ime_jstring_spike.md` is the next JNI change (needs NDK + `jni-sys`).
2. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. `gpui-component/input.rs` still uses a local `fnPtr` workaround for `set_ime_exclusion_area`; a real GPUI method needs a crate bump.
3. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
4. **`gpui::LineCap`** — `progress.rs` documents `LineCap::Round` as a gpui re-export. Verify after the next gpui bump.
5. **Nav rail `PopUpMenu`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
6. **Carousel bitmaps** — still `primary_container` color blocks, not decoded photo tiles (need image crate + assets).
7. **Snackbar destination chrome vs official Gmail** — v18 added initials avatars, timestamps, status bar, and 80dp nav. Catalog in-phone snack uses `data-persist="1"` so snapshots keep “Email archived”. Official still uses **photo avatars**, **Mail / Chat / Rooms / Meet** destinations (not Inbox/Starred/Profile), and a real status-bar cluster.
8. **Tabs / share-sheet photo chrome** — v18 added status time and a people row (AR/JL/SC/+ Add). Official uses **camera photos**, **named people**, and a selected **Audio** tab with real thumbnails. Role-color stubs remain.
9. **FAB menu / split button / toolbar scene chrome** — tokens + catalog/desktop/Android heroes land in v18. Official wraps them in **photo product / chat / map** scenes (enamel mugs card, chat thread, city map). Catalog is a token matrix, not those photos. Toolbar vibrant FAB is `primary_container` purple vs official pink tertiary-ish fill.
10. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
11. **Live GPUI recapture** — `docs/qa/compare_desktop_vs_official_*_v18.png` is **headless Chrome catalog HTML vs m3.material.io**, not a wgpu window. Recapture the `material_desktop_demo` window when a display is available.

Do **not** reopen as missing: small FAB 56/80, nav-rail overlay, search-view tokens, loading indicator, split button / FAB menu / floating toolbar **token modules**.

## Suggested v19 order

1. Photo assets for snackbar avatars, tabs tiles, share-sheet people, carousel (or a tiny bundled JPEG set).
2. Snackbar destinations Mail/Chat/Rooms/Meet + “Email archived / Action” copy already in the phone; match official nav labels.
3. Put FAB menu / split / toolbar heroes in a phone+photo frame closer to the overview.
4. Live IME / caret / search transform (needs NDK + gpui API).
5. APK install + live desktop wgpu recapture.

Host tests: `cargo test -p gpui_material` (27), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
