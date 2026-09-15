# v35 leftovers — GPUI mobile Material 3 Expressive

Continuing from **v34** (`feat: v34 Expressive overflow/split grouped menus`). Critique vs [m3.material.io](https://m3.material.io/components/menus/specs) Expressive.

**v34 landed on `main` only (no PR).** Start from that tip; do not re-open overflow/split as a single 16dp shell, or ElevatedFilterChip / unscrimmed overlay as missing inventory — they ship. Navigation drawer and **banner** stay **out** of inventory.

Highest-impact remaining gaps:

1. **Live JVM IME (`jstring`)** — host `jni_text_from_arg` uses UTF-8 `GetStringUTFChars` on Android; a live `ImeListener` on a device JVM still needs NDK + `jni-sys`.
2. **GPUI caret bounds** — `crates/gpui` has no `Window` IME-rect API. A real GPUI method needs a crate bump.
3. **Search-view morph** — AndroidX search morphs with a container transform. GPUI still fades/resizes via `PathBuilder::scale`; no `cx.transform`.
4. **`gpui::LineCap`** — `progress.rs` maps `StrokeCap::Round` to lyon until gpui re-exports `LineCap`.
5. **Nav rail `PopUp`** — OS `PopUp` on Linux/Android NativeActivity is still unproven; rail uses an inline GPUI overlay.
6. **Device APK** — no SDK/AVD/adb in this environment; `component_demo` still needs `cargo apk run` on a device/emulator.
7. **Live GPUI + Chrome recapture** — this VM cannot create a wgpu surface; Chrome `--screenshot` hangs (dump-dom works). Official-vs-catalog strips remain v19 left halves. Recapture HTML heroes (including grouped overflow/split + 200ms hover delay) when a display/screenshot pipeline works.
8. **List swipe-as-LazyColumn fling and predictive-back** stay unimplemented. Higher-res stills (current bundle is 128–240px) if official optical match needs print-size photos.
9. **Chip live press interpolation** — catalog chips snap `ChipShapes`; no `rememberAnimatedShape`. InputChip avatar is not painted. Compact 4dp arrangement is token-only.
10. **Wide rail live morph of icon position** — catalog pair is static Top vs Start; the modal 80↔220 overlay swaps layout on toggle but does not interpolate Top→Start (Compose `iconPosition` follows `railExpanded` with a layout animation). Non-modal expanded (no scrim, `WideNavigationRail` in-flow) is the pair only.
11. **GPUI hover delay** — catalog JS waits `HOVER_OPEN_DELAY_MS` (200) before opening the End flyout; desktop/Android hosts still hover-open immediately (no GPUI timer). Typeahead still needs the cascade focused (`tab_index`).

Do **not** reopen as missing: JPEG `PhotoKind::jpeg_bytes` / decode / mosaic / **licensed camera stills**, snackbar peek + dest SVGs, Your lists chrome, split/FAB-menu/toolbar token modules, medium/large **flexible** collapse + search app bar, side sheet **standard/modal/detached**, navigation **drawer** (deprecated), **banner** (not on current site), plain/rich **tooltip** + **caret** + **hover/long-press**, flexible **64dp** nav, **segmented lists**, **standard + connected** button groups, standard **OverflowIndicator**, list **swipe rails** + **drag-handle reorder**, icon-button **narrow/default/wide**, icon-button **toggle selected** colors + round↔square morph, **Expressive menus** (standard/vibrant, grouped, horizontal + icon-only), **Expressive Filter/Input chip morph** (12/16/8 + selected check + input close), **ElevatedFilterChip** + **tonal** leading-icon `onSurfaceVariant`, **WideNavigationRailItem Top/Start** (96/220, 56×32 vs 56dp pill, secondary active label), **submenu flyout** (24/8 morph, End placement, Share/Save/Sort typeahead, catalog hover-open), **overlay submenu** (grouped default, More Stay + End flyout, live typeahead, catalog `menu-overlay`), **unscrimmed anchored overlay** (no 32% scrim, Menu anchor), **overflow/split grouped 2dp + nested More flyout**.

## Suggested v35 order

1. Live IME / caret / search transform if an NDK or gpui API bump becomes available (otherwise skip).
2. GPUI hover-open timer (`HOVER_OPEN_DELAY_MS`) on overflow/split/overlay cascades.
3. APK install + Chrome/wgpu recapture of v21–v34 heroes.

Host tests: `cargo test -p gpui_material` (35), `-p gpui_android` (17), `-p material_desktop_demo` (1). Android GPUI paint is `cfg(target_os = "android")`.
