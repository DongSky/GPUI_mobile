# Desktop capture v118

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v118 after origin v117 PeriodSelector outline (`7d866ce`). Timer asked v116 from `d74b850` while origin landed TimeSelector as v116 and PeriodSelector as v117 — this slot is **v118**.  
**Official:** Compose `TimePickerDialogDefaults.DisplayModeToggle` / `ScrollDisplayModeToggle` a11y + tooltip strings are `m3c_time_picker_toggle_keyboard` “Switch to text input mode”, `m3c_time_picker_toggle_scroll` “Switch to scroll mode”, and `m3c_time_picker_toggle_touch` “Switch to clock mode”. Hosts still used the shorter “Switch to input mode” demo title.

v118 started from current `main` (`7d866ce`) and `docs/qa/v117_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePickerDialogDefaults.DisplayModeToggle a11y strings + tooltip**. Do not reopen PeriodSelector 1dp Outline shell, TimeSelector selected / idle colors, TimeInput vibrant field outline, TimePickerDefaults.shapes / ScrollFieldDefaults.shape CornerLarge 16, fullScreenContainedSearchBarColor, DatePicker WeekDays, VibrantTimePickerDialog, TimePicker ClockFaceSizeModifier, DatePickerDialog CrossAxisSpacing, TimePickerCustomLayout paddings / Cancel/OK, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped. Do not apply VibrantTimeField 100/132×120, VibrantHorizontalGap 52, or VibrantTimePickerCustomLayout 12. Do not restore ExtraLarge 28 on TimeScroll / TimeInput fields. Do not add TimeSelector 2dp primary outline or `periodSelectorShape` CornerFull. Do not move the title-row 12/24 + ScrollDisplayModeToggle into the action row.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TOGGLE_KEYBOARD` / `TOGGLE_SCROLL` / `TOGGLE_TOUCH` / `DISPLAY_MODE_TOGGLE` / `display_mode_toggle_label`; `TimePickerDisplayMode.toggle_label` uses official ScrollDisplayModeToggle strings |
| Catalog | `data-display-mode-toggle` + `data-toggle-keyboard` / `scroll` / `touch`; title + aria-label; JS writes official strings |
| Hosts | Desktop + Android hover tooltip with `toggle_label()` (Compose DisplayModeToggle tooltip) |
| Inventory | Time picker notes DisplayModeToggle a11y |
| Tests | official strings + catalog `data-display-mode-toggle` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimePicker DisplayModeToggle a11y against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + dividers + 360×568 + HeaderContainerHeight + MonthsNavigation + range-header close + DialogButtonsPadding + CrossAxisSpacing 8 + pads + empty headlines + SelectableDates + WeekDays BodyLarge / OnSurface / 48 | Recapture stills when screenshot works; DatePicker mode-toggle still uses shorter “Switch to input mode” / “Switch to calendar mode” |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + CornerLarge 16 + vibrant field outline + TimeSelector PrimaryContainer + PeriodSelector Outline shell + SupportLabelTop + Title + CustomLayout + ClockFaceSizeModifier + VibrantTimePickerDialog + **DisplayModeToggle a11y + tooltip** + Cancel/OK + 24h ClockFace | Recapture stills when screenshot works |

## v119 leftovers

See `docs/qa/v118_leftovers.md`.
