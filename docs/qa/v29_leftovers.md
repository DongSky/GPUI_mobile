# v30 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v29** (`feat: v29 Expressive chip Filter/Input shape morph`). Critique vs [m3.material.io](https://m3.material.io/components/chips/specs) Expressive.

**v29 landed on `main` only (no PR).** Start from that tip; do not re-open Expressive Filter/Input chip morph (medium 12 / full selected / small 8 pressed, selected check, input close) as missing inventory — they ship. Navigation drawer and **banner** stay **out** of inventory.

Highest-impact remaining gaps:

1. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
2. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
3. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
4. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
5. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
6. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
7. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including Expressive chips) when a display/screenshot pipeline works.
8. **List swipe-as-LazyColumn fling and predictive-back** stay unimplemented. Higher-res stills (current bundle is 128–240px) if official optical match needs print-size photos.
9. **Wide navigation rail icon-position** — collapsed top-icon / expanded start-icon (`WideNavigationRailItem`) is not a separate hero from the 80↔220 overlay rail.
10. **Menu submenu flyout + keyboard** — More › is a trailing chevron only; no nested surface, hover-open, or WAI-ARIA typeahead.
11. **Chip live press interpolation + elevated/tonal** — catalog chips snap `ChipShapes`; no `rememberAnimatedShape`. ElevatedFilterChip / tonal leading-icon (`onSurfaceVariant`) color defaults are not a separate hero. InputChip avatar is not painted.

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic / **licensed camera stills**, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated), **banner** (not on current site), plain/rich **tooltip** + **caret** + **hover/long-press**, flexible **64dp** nav, **segmented lists**, **standard + connected** button groups, standard **OverflowIndicator**, list **swipe rails** + **drag-handle reorder**, icon-button **narrow/default/wide**, icon-button **toggle selected** colors + round↔square morph, **Expressive menus** (standard/vibrant, grouped, horizontal + icon-only), **Expressive Filter/Input chip morph** (12/16/8 + selected check + input close).

## Suggested v30 order

1. Wide-rail icon-position hero (implementable) or elevated/tonal chip color defaults.
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture of v21–v29 heroes.

Host tests: `cargo test -p gpui_material` (32), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
