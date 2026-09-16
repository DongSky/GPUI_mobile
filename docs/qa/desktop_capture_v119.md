# Desktop capture v119

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v119 after origin v118 TimePicker DisplayModeToggle a11y (`726c1fa`). Timer asked v118; origin already landed that slot.  
**Official:** DatePicker `DisplayModeToggleButton` strings are `m3c_date_picker_switch_to_input_mode` “Switch to text input mode” and `m3c_date_picker_switch_to_calendar_mode` “Switch to calendar input mode”. Hosts/catalog still used the shorter “Switch to input mode” / “Switch to calendar mode”.

v119 started from current `main` (`726c1fa`) and `docs/qa/v118_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DatePicker DisplayModeToggleButton a11y + tooltip**. Do not reopen TimePicker DisplayModeToggle / ScrollDisplayModeToggle, PeriodSelector outline, TimeSelector colors, TimeInput field outline, TimePickerDefaults.shapes CornerLarge 16, or the v118 do-not-reopen list. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DISPLAY_MODE_TOGGLE` / `TOGGLE_INPUT` / `TOGGLE_CALENDAR`; `DatePickerDisplayMode.toggle_label` uses official strings |
| Catalog | `data-date-display-mode-toggle` + title / aria-label + live JS official strings |
| Hosts | Desktop + Android hover tooltip on live date / range mode toggles |
| Inventory | Date picker notes DisplayModeToggleButton a11y |
| Tests | official strings + catalog `data-date-display-mode-toggle` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DatePicker DisplayModeToggleButton against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + WeekDays + 360×568 + pads + empty headlines + SelectableDates + **DisplayModeToggleButton a11y** | Recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + CornerLarge 16 + field outline + TimeSelector colors + PeriodSelector Outline + DisplayModeToggle a11y + CustomLayout + ClockFaceSizeModifier | Recapture stills when screenshot works |

## v120 leftovers

See `docs/qa/v119_leftovers.md`.
