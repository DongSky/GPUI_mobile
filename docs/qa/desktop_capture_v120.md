# Desktop capture v120

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v120 after v119 DatePicker DisplayModeToggleButton (`aeed3f4`).  
**Official:** `TimePickerDialogDefaults.ScrollDisplayModeToggle` uses `Icons.Filled.SwipeVertical` while Input (Keyboard while Scroll). `DisplayModeToggle` still uses Schedule for Input → Picker. Hosts/catalog still painted unofficial Schedule ◷ on the scroll toggle.

v120 started from current `main` and `docs/qa/v119_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **ScrollDisplayModeToggle SwipeVertical**. Do not reopen DisplayModeToggle / DatePicker DisplayModeToggleButton a11y strings. Do not apply `periodSelectorShape` CornerFull or TimeSelector 2dp selected outline. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `SWIPE_VERTICAL` / `SWIPE_VERTICAL_ICON` ⇅ / `display_mode_toggle_icon`; Scroll↔Input Keyboard / SwipeVertical |
| Catalog | `data-swipe-vertical` + live ⌨/⇅ paint |
| Hosts | Desktop + Android `toggle_icon()` pick SwipeVertical on Input |
| Inventory | Time picker notes ScrollDisplayModeToggle SwipeVertical |
| Tests | Input icon ⇅ + catalog `data-swipe-vertical` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture ScrollDisplayModeToggle SwipeVertical against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + WeekDays + DisplayModeToggleButton a11y | Recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + PeriodSelector Outline + DisplayModeToggle a11y + **ScrollDisplayModeToggle SwipeVertical** | Recapture stills when screenshot works |

## v121 leftovers

See `docs/qa/v120_leftovers.md`.
