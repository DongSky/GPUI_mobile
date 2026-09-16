# Desktop capture v104

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v104 after origin v103 DatePickerDialog ContainerWidth Height (`e614c1e`).  
**Official:** Compose `DateEntryContainer` draws `HorizontalDivider` under title / headline / mode toggle (`DatePickerColors.dividerColor` = `DividerTokens.Color`, `DividerDefaults.Thickness` 1). Docked DatePicker has no header chrome, so no divider.

v104 started from current `main` (origin v103) and `docs/qa/v103_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **DateEntryContainer header HorizontalDivider**. Do not reopen DatePickerDialog ContainerWidth/Height, HeaderContainerHeight, TimePicker DisplaySeparator / PeriodToggleMargin, ClockFace margins, MonthsNavigation MonthYearHeight, DateRangePicker range-header chrome, DatePickerDialog DialogButtonsPadding, paddings, empty headlines, SelectableDates, DateInputValidator, docked / range-hero / modal chrome already shipped. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DATE_ENTRY_DIVIDER` / `DATE_ENTRY_DIVIDER_H_DP` 1 / `date_entry_divider_visible` / `date_entry_divider_height_css` |
| Catalog | `.dp-entry-divider` full-bleed under modal / range / input / year headers (`data-date-entry-divider`); docked omitted |
| Hosts | Desktop + Android modal date / range / input cards draw outline-variant 1dp under header chrome; range-input `header_year` 1px lines use the official token + color |
| Inventory | Date picker notes DateEntryContainer HorizontalDivider |
| Tests | tokens + catalog `data-date-entry-divider` / `.dp-entry-divider`; visibility true with title/headline/toggle, false when all absent |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture DateEntryContainer header HorizontalDivider against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + DatePickerDialog 360×568 + HeaderContainerHeight 120 / range 68 + **DateEntryContainer HorizontalDivider** + MonthsNavigation 56/48 + range-header close + DialogButtonsPadding + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + ClockFace 36/24 + DisplaySeparator 24 + PeriodToggleMargin 12 + Horizontal | Recapture stills when screenshot works |

## v105 leftovers

See `docs/qa/v104_leftovers.md`.
