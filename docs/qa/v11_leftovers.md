# v11 leftovers (after v10 on `main`)

Start here if continuous Material / Expressive work continues. Land on `main` (no iteration PR).

1. **NativeActivity InputConnection** — `update_ime_position` now records `ime_caret_rect_dp` / `last_ime_bounds`. Wire a real IME / JNI `InputConnection`.
2. **Search docked→activity** — height/corner/list-opacity growing-bar, not a Compose shared-element container transform. Header chrome still swaps (avatar vs back) instead of a single morphing header.
3. **Outlined notch on GPUI** — open-path stroke cuts the label; HTML uses even-odd C fill. GPUI even-odd fill of the tessellated C still leaves corner slivers, so hosts stroke the centerline. A lyon `LineCap` / robust even-odd host would let both paint the same filled C.
4. **Loading indicator** — 7-shape polar morph (SoftBurst / Cookie9 / Pentagon / Pill / Sunny / Cookie4 / Oval approximation), not Material `RoundedPolygon` corner rounding. Determinate morph-by-progress not shown.
5. **Circular / PTR stroke** — disc-stamped round caps (gpui does not re-export `LineCap::Round`). Sausage polygon fill is extra, not a true annular sector.
6. **Nav rail** — in-catalog modal + scrim, not a separate `Window` / focus trap.
7. **Carousel** — one-shot `inertial_steps`; `integrate_fling` exists but wheel is not a per-frame physics loop.
8. **Time picker** — hour-face live tick is a repeating lerp, not a continuous ticking second hand.
9. **Android APK** — NDK not in this VM; `component_demo` typechecks only.
10. **Official side-by-side** — live GPUI vs m3.material.io compare strips were not regenerated this pass (`compare_desktop_vs_official_*_v9.png` is the last set).
