# Desktop capture v110

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v110 after origin v109 DatePickerDialog CrossAxisSpacing (`c117b9b`).  
**Official:** Compose `ClockFaceSizeModifier` sizes the ClockFace from available height: ≥ `TimePickerMaxHeight` 384 → `ClockDialContainerSize` 256; ≥ `TimePickerMidHeight` 330 → `ClockDialMidContainerSize` 238; else `ClockDialMinContainerSize` 200. Selector handle stays `ClockDialSelectorHandleContainerSize` 48.

v110 started from current `main` (origin v109) after this agent's v107–v109 slots were already taken. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **TimePicker ClockFaceSizeModifier responsive clock sizes**. Compact hosts already shrank the dial to unofficial 192dp (`0.75 × 256`); that breakpoint maps to official Min 200. YearPicker `requiredHeight` 335 still assumes 48dp calendar rows — skipped while day cells stay 40dp. Do not reopen DatePickerDialog CrossAxisSpacing, TimePickerCustomLayout, TimePickerDialogDefaults Title, TimeInput SupportLabelTop, YearPicker trailing HorizontalDivider, DateEntryContainer header HorizontalDivider, DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `TIME_PICKER_MAX_HEIGHT_DP` 384 / `TIME_PICKER_MID_HEIGHT_DP` 330 / `CLOCK_DIAL_MID_CONTAINER_SIZE_DP` 238 / `CLOCK_DIAL_MIN_CONTAINER_SIZE_DP` 200 / `CLOCK_DIAL_SIZES` / `ClockDialSize` + `clock_dial_size_for_max_height` / compact hosts `DEMO_HOST_CLOCK_SIZE` Min |
| Catalog | Vertical + horizontal stay Max 256 (`data-clock-dial-size="max"`); mid 238 + min 200 ClockFace heroes (`data-hero="timepicker-clock-mid"` / `timepicker-clock-min`) |
| Hosts | Desktop + Android replace unofficial 192dp (`0.75` / hardcoded) with official Min 200 + 48dp handle |
| Inventory | Time picker notes ClockFaceSizeModifier 384/330 → 256/238/200 |
| Tests | breakpoint 384→Max / 330→Mid / 329→Min + catalog mid/min markers + host 200 |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture TimePicker ClockFaceSizeModifier against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + YearPicker trailing HorizontalDivider + DateEntryContainer header divider + DatePickerDialog 360×568 + HeaderContainerHeight 120 / range 68 + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + CrossAxisSpacing 8 + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + SupportLabelTop + Title 20dp + CustomLayout Cancel/OK + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal + **ClockFaceSizeModifier 256/238/200** | Recapture stills when screenshot works |

## v111 leftovers

See `docs/qa/v110_leftovers.md`.
