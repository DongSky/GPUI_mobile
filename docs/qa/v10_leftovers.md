# v11 leftovers (after v10)

Start here if continuous mode continues.

1. NativeActivity InputConnection — consume `ime_caret_rect_dp` / `last_ime_bounds` for a real IME.
2. Search docked→activity: shared-element container transform closer to Compose SearchBar.
3. Range dual-thumb: paint discrete stop ticks while snapping.
4. PTR: use lyon `LineCap::Round` if gpui re-exports it later (today: cap discs).
5. Nav rail: elevated modal window / focus trap, not just a scrim sibling.
6. Carousel: inertial fling with per-frame decay instead of one wheel event.
7. Time picker: continuous hour-face motion while the dial is showing hours.
8. Loading indicator: determinate circular with gap + rotation like M3 Expressive.
