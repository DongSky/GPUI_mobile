# v32 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v31** (`feat: v31 Expressive menu submenu flyout + typeahead`). Critique vs [m3.material.io](https://m3.material.io/components/menus/specs) Expressive.

**v31 landed on `main` only (no PR).** Start from that tip; do not re-open submenu flyout (ActiveContainerShape 24 / InactiveContainerShape 8, MenuAnchorPosition.End, Share/Save/Sort typeahead, hover-open catalog) as missing inventory — they ship. Navigation drawer and **banner** stay **out** of inventory.

Highest-impact remaining gaps:

1. **Live JVM IME (`jstring`)** — `ImeListener` still uses UTF-16 `GetStringChars`. Needs NDK + `jni-sys` on a device JVM.
2. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
3. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes; no `cx.transform`.
4. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
5. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
6. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
7. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including Expressive submenu cascade) when a display/screenshot pipeline works.
8. **List swipe-as-LazyColumn fling and predictive-back** stay unimplemented. Higher-res stills (current bundle is 128–240px) if official optical match needs print-size photos.
9. **Chip live press interpolation + elevated/tonal** — catalog chips snap `ChipShapes`; no `rememberAnimatedShape`. ElevatedFilterChip / tonal leading-icon (`onSurfaceVariant`) color defaults are not a separate hero. InputChip avatar is not painted.
10. **Wide rail live morph of icon position** — catalog pair is static Top vs Start; the modal 80↔220 overlay swaps layout on toggle but does not interpolate Top→Start (Compose `iconPosition` follows `railExpanded` with a layout animation). Non-modal expanded (no scrim, `WideNavigationRail` in-flow) is the pair only.
11. **GPUI overlay submenu + live typeahead** — catalog HTML hover/keyboard drives the cascade; desktop/Android overlay menus are still a single grouped surface (click More dismisses). GPUI hosts paint the static cascade hero but do not hover-open or typeahead.

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic / **licensed camera stills**, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated), **banner** (not on current site), plain/rich **tooltip** + **caret** + **hover/long-press**, flexible **64dp** nav, **segmented lists**, **standard + connected** button groups, standard **OverflowIndicator**, list **swipe rails** + **drag-handle reorder**, icon-button **narrow/default/wide**, icon-button **toggle selected** colors + round↔square morph, **Expressive menus** (standard/vibrant, grouped, horizontal + icon-only), **Expressive Filter/Input chip morph** (12/16/8 + selected check + input close), **WideNavigationRailItem Top/Start** (96/220, 56×32 vs 56dp pill, secondary active label), **submenu flyout** (24/8 morph, End placement, Share/Save/Sort typeahead, catalog hover-open).

## Suggested v32 order

1. ElevatedFilterChip / tonal chip color defaults (implementable) or GPUI overlay submenu.
2. Live IME / caret / search transform (needs NDK + gpui API).
3. APK install + Chrome/wgpu recapture of v21–v31 heroes.

Host tests: `cargo test -p gpui_material` (34), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
