# v12 leftovers (after v11 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

1. **JNI InputConnection** — `ImeSession` now mirrors `commitText` / compose / delete / cursor-anchor and `update_ime_position` fills it. NativeActivity still has no JNI `InputMethodManager`.
2. **Search shared-element** — `MorphFrame` eases height/corners/inset/scale + container lerp. Still not a pixel-perfect Compose SearchBar container transform; header chrome still swaps (avatar vs back).
3. **Outlined notch** — GPUI/HTML share the explicit C-path even-odd polygon. Remaining: lyon `LineCap` if hosts want a stroked centerline; label width vs glyph metrics can leave a sliver under wide letters. `RoundedPolygon` cubics exist in `shape` but are not the live notch path.
4. **Loading indicator** — 7-shape polar morph (SoftBurst / Cookie9 / Pentagon / Pill / Sunny / Cookie4 / Oval). Determinate morph-by-progress not shown.
5. **Circular / PTR stroke** — disc-stamped round caps (gpui does not re-export `LineCap::Round`).
6. **Nav rail** — in-catalog modal + scrim + level-2 elevation + `focus_trapped` / scrim dismiss. Not a separate `Window`.
7. **Carousel** — `FlingState::step` + `integrate_fling` exist; catalogs still one-shot wheel, not a per-frame physics loop.
8. **Time picker** — hour-face live tick is a repeating lerp, not a continuous ticking second hand.
9. **Android APK** — NDK not in this VM; `component_demo` typechecks only.
10. **Official side-by-side** — last compare strips are `compare_desktop_vs_official_*_v9.png`.
