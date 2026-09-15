# v29 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v28** (`feat: v28 Expressive menus, grouped + vibrant + horizontal`). Critique vs [m3.material.io](https://m3.material.io/components/menus/specs) Expressive.

**v28 landed on `main` only (no PR).** Start from that tip; do not re-open Expressive menus (standard/vibrant, grouped 2dp, 44dp items, horizontal pills + icon-only) as missing inventory — they ship. Navigation drawer and **banner** stay **out** of inventory.

Highest-impact remaining gaps:

1. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
2. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
3. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
4. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
5. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
6. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
7. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including Expressive menus) when a display/screenshot pipeline works.
8. **List swipe-as-LazyColumn fling and predictive-back** stay unimplemented. Higher-res stills (current bundle is 128–240px) if official optical match needs print-size photos.
9. **Chips Expressive shape morph** — catalog chips are still 32dp full-round baseline; Compose FilterChip/InputChip now morph corners on select. Next component-level gap vs current Expressive.
10. **Wide navigation rail icon-position** — collapsed top-icon / expanded start-icon (`WideNavigationRailItem`) is not a separate hero from the 80↔220 overlay rail.
11. **Menu submenu flyout + keyboard** — More › is a trailing chevron only; no nested surface, hover-open, or WAI-ARIA typeahead.

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic / **licensed camera stills**, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated), **banner** (not on current site), plain/rich **tooltip** + **caret** + **hover/long-press**, flexible **64dp** nav, **segmented lists**, **standard + connected** button groups, standard **OverflowIndicator**, list **swipe rails** + **drag-handle reorder**, icon-button **narrow/default/wide**, icon-button **toggle selected** colors + round↔square morph, **Expressive menus** (standard/vibrant, grouped, horizontal + icon-only).

## Suggested v29 order

1. Chips Expressive select morph (implementable) or wide-rail icon-position hero.
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture of v21–v28 heroes.

Host tests: `cargo test -p gpui_material` (31), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
