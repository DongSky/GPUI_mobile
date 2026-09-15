# v16 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Binary:** `target/debug/material_desktop_demo` after the v16 full-screen dialog / icon-row overflow / carousel layouts / snackbar dismiss / RegisterNatives peer pass.

Live GPUI catalog frames from v12 remain the scene screenshots (`docs/qa/catalog_*_v12.png`). Official vs HTML-catalog strips added this pass (left: headless Chrome on m3.material.io; right: HTML catalog heroes from the same `resolve()` the GPUI hosts paint):

- `compare_desktop_vs_official_snackbar_v16.png`
- `compare_desktop_vs_official_bottom_sheet_v16.png`
- `compare_desktop_vs_official_tabs_v16.png`
- `compare_desktop_vs_official_dialog_v16.png`
- `compare_desktop_vs_official_carousel_v16.png`
- `compare_desktop_vs_official_buttons_v16.png`

v12 text-fields/progress, v13 buttons/search/time-picker, v14 date-picker/nav-rail/slider, and v15 dialog/carousel/button-group strips still apply.

## Critique vs current m3.material.io Expressive

| Official | Catalog / GPUI | Gap |
|---|---|---|
| Dialogs overview is basic + full-screen Event editor | Reset-settings + ringtone **plus** full-screen Event (0dp, 64dp header, close/Save, divider, filled fields) | Catalog full-screen is a section, not a route-sized takeover |
| Button groups: connected icon row + overflow on a photo | Day/Week/Month **plus** ✎/🖼/＋/⋮ icon row with Cut/Copy/Paste menu | Official icons are filled-tonal circles; catalog uses connected outlined/filled morph |
| Carousel: photo tiles in a phone frame | Hero 256/120, multi-browse 186/56, uncontained 220/140; role-color media + parallax | No bitmaps / phone mask / extra layouts |
| Snackbar: “Email archived” over a mail scene, Action + close | Inverse-surface Retry bar; 4s timeout + 72dp swipe | Not in-scene; no close affordance |
| Bottom sheet: share sheet over photos | Modal list + standard (no-scrim) sheet | Official is a photo-grid share surface |
| Tabs: phone “My saved media” icon+label | Primary / secondary + 64dp icon row (News/Video/Photos) | No in-app media scene |
| Text fields show the system IME | `native*` methods + session handle + `RegisterNatives` plan | No live `fnPtr` bind; catalog caret is computed |

Android `component_demo` debug APK **should build** (NDK 25.1, `hasCode=true`, native IME peer). No AVD/device/SDK in this VM, so install/run is leftover.

Host tests: `gpui_material` **26**, `gpui_android` **17**, `material_desktop_demo` **1**.

## What v16 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| Dialog | `resolve_fullscreen` / Event fields / 64dp header | HTML + GPUI full-screen overlay (desktop + Android) |
| Button group | `ICON_SEGMENTS` / overflow menu / `resolve_icon_segment` | Connected icon row + menu in HTML + both hosts |
| Carousel | `CarouselLayout` hero/multi-browse/uncontained; `media_fill`; `parallax_offset_dp` | Hosts lerp widths per layout; HTML paints three rows |
| Snackbar | `SnackbarState` 4s/10s + 72dp swipe | HTML rAF timeout/pointer swipe; Android wall-clock + wheel swipe |
| Tabs / sheet | `resolve_with_icons` 64dp; standard sheet | HTML icon row + `data-sheet="standard"` |
| IME | unchanged caret | Java `native*` + `(View,Z,J)` ctor; Rust peer table + session handle + `RegisterNatives` queue step |

## v17 leftovers

See `docs/qa/v16_leftovers.md`.
