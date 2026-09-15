# Desktop capture v58

**Date:** 2026-09-15  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v58 commit after `7f8201e` (v57 docked search scrim).  
**Official:** [m3.material.io time picker specs](https://m3.material.io/components/time-pickers/specs) — horizontal (landscape) puts time selectors beside the 256dp ClockFace; period selector is 216×38. Compose `TimePickerLayoutType.Horizontal`.

v58 started from v57 leftovers (`docs/qa/v57_leftovers.md`). Highest-impact **implementable** gap vs current Expressive (no NDK/gpui bump): **TimePickerLayoutType.Horizontal**. Do not reopen ClockFace rings, TimeInput, 24-hour format, docked search scrim, Results/Quick results, groups, compact→docked. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TimePickerLayoutType` Vertical / Horizontal; 216×38 period; 24dp selector–dial gap |
| Catalog | Vertical dial keeps `data-time-layout="vertical"`; new horizontal sibling `data-hero="timepicker-horizontal"` |
| Hosts | Desktop dial hero is landscape (selectors left, ClockFace right); compact Android stays vertical |
| Inventory | Time picker notes TimePickerLayoutType Horizontal |
| Tests | Layout tokens, catalog attrs |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture horizontal TimePicker against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Time picker | TimeScroll + TimeInput 96×72 + toggle; 24-hour 00–23; ClockFace dual rings; Horizontal landscape sibling | Segmented search result rows still open |
| Search | Compact fullscreen; medium docked + 32% scrim; Quick results / Results | `cx.transform` search morph skipped |
| Text field | Expressive rounded 12 + tonal Inside | Live JVM IME / GPUI caret bounds still blocked |

## v59 leftovers

See `docs/qa/v58_leftovers.md`.
