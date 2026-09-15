# v12 leftovers (after v11 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

1. **JNI InputConnection** — `ImeSession` mirrors `commitText` / compose / delete / cursor-anchor and `update_ime_position` fills it. NativeActivity still has no JNI `InputMethodManager`.
2. **Search header chrome** — container lerp + inset/scale shared-element is in. Header still swaps avatar vs back (no crossfade of leading chrome).
3. **Outlined notch** — GPUI/HTML share the explicit C-path even-odd polygon. Remaining: lyon `LineCap` if hosts want a stroked centerline; label width vs glyph metrics can leave a sliver under wide letters. `RoundedPolygon` cubics exist in `shape` but are not the live notch path.
4. **Loading determinate** — `loading_polygon_for_progress` is shown at 65%. Hosts do not yet drive morph-by-progress from a real download / determinate wait.
5. **Circular / PTR stroke** — disc-stamped round caps (gpui does not re-export `LineCap::Round`).
6. **Nav rail** — in-catalog modal + scrim + level-2 elevation + `focus_trapped` / scrim dismiss. Not a separate `Window`.
7. **Carousel live clock** — GPUI wheel now runs `FlingState::step_until_rest` (60 Hz batch). HTML catalog JS is still one-shot. No per-frame animation loop while a fling is in flight.
8. **Time picker wall clock** — 60 s ticking second hand uses `DEMO_SECOND`, not a host wall-clock. Hour-face live tick is still a repeating lerp.
9. **Android APK** — NDK not in this VM; `component_demo` typechecks only.
10. **Official side-by-side** — last compare strips are `compare_desktop_vs_official_*_v9.png`.
