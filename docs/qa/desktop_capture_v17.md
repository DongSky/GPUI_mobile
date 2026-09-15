# v17 Visual QA — Desktop GPUI window

**Date:** 2026-09-15  
**Landing:** `main` (push directly to default branch; no PR)  
**Binary:** `target/debug/material_desktop_demo` after the v17 Inbox snackbar / saved-media tabs / share-sheet / carousel layouts / JNI fnPtr pass.

Live GPUI catalog frames from v12 remain the scene screenshots (`docs/qa/catalog_*_v12.png`). Official vs HTML-catalog strips added this pass (left: headless Chrome on m3.material.io; right: HTML catalog heroes from the same `resolve()` the GPUI hosts paint):

- `compare_desktop_vs_official_snackbar_v17.png`
- `compare_desktop_vs_official_tabs_v17.png`
- `compare_desktop_vs_official_carousel_v17.png`
- `compare_desktop_vs_official_bottom_sheet_v17.png`

v12 text-fields/progress, v13 buttons/search/time-picker, v14 date-picker/nav-rail/slider, v15 dialog/carousel/button-group, and v16 snackbar/tabs/sheet/dialog/carousel/buttons strips still apply.

## Critique vs current m3.material.io Expressive

| Official | Catalog / GPUI | Gap |
|---|---|---|
| Snackbar overview is an Inbox phone with “Email archived”, Action, and close | Inbox + mail rows + **Email archived / Undo / ✕**; timeout + swipe + `close()` | No avatars / timestamps / bottom nav; action label is Undo |
| Tabs overview is “My saved media” Video / Photos / Audio over photos | Same labels + 64dp icon row + role-color tile grid in a phone frame | Stub tiles, no status-bar chrome |
| Carousel: six layouts, photos, phone mask | Hero / multi-browse / uncontained / **centered-hero** / **full-screen**; phone-frame on the last two | No bitmaps; missing uncontained multi-aspect |
| Bottom sheet overview is a share sheet over a photo app | Photo grid + Share actions + handle; standard no-scrim sheet still listed | No people row / real photos |
| Text fields show the system IME | JNI-mangled `Java_dev_gpui_…` exports + `JNINativeMethod.fnPtr` table; Android `RegisterNatives` binds them | Host tests use C-string payloads; no live JVM `jstring` |

Android `component_demo` debug APK **should build** (NDK 25.1, `hasCode=true`, JNI-mangled IME peer). No AVD/device/SDK in this VM, so install/run is leftover.

Host tests: `gpui_material` **26**, `gpui_android` **17**, `material_desktop_demo` **1**.

## What v17 changed (code)

| Area | Shared `gpui_material` | Mapping |
|---|---|---|
| Snackbar | Inbox scene, Undo + close, `SnackbarState::close` | HTML phone + GPUI mail scene (desktop + Android) |
| Tabs | `SCENE_*` My saved media + tile fills | HTML + both hosts |
| Bottom sheet | Share actions + photo grid helpers | HTML phone + Android overlay + desktop scene |
| Carousel | `CenteredHero` / `FullScreen`, `next()`, phone-frame | HTML five rows; hosts cycle layouts |
| IME | unchanged caret | `#[unsafe(no_mangle)] Java_dev_gpui_…` + `jni_native_fn_ptr_table`; Android `RegisterNatives` uses `fnPtr` |

## v18 leftovers

See `docs/qa/v17_leftovers.md`.
