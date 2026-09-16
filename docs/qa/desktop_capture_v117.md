# Desktop capture v117

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v117 after v116 TimeSelector colors (`05a1bdb`).  
**Official:** `TimePickerTokens.PeriodSelectorOutlineWidth` + `PeriodSelectorOutlineColor` (`Outline`) wrap AM/PM in one CornerSmall shell. Vertical container is 52×80 (two 40dp halves); horizontal is 216×38; TimeInput is 52×72. Unofficial 8dp gap + 36dp buttons retired.

v117 started from current `main` and `docs/qa/v116_leftovers.md`. Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **PeriodSelector outline**. Do not apply `periodSelectorShape` CornerFull — keep CornerSmall 8. Do not add TimeSelector 2dp selected outline. Do not reopen TimeSelector colors, TimeInput field outline, TimePickerDefaults.shapes CornerLarge 16, or the v116 do-not-reopen list. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `time_picker.rs` | `PERIOD_OUTLINE` / `PERIOD_OUTLINE_W_DP` 1 / `PERIOD_CONTAINER_H_DP` 80 / `PERIOD_H_DP` 40 / `PERIOD_GAP_DP` 0 / `period_outline_w_css`; appearance `period_outline` Outline |
| Catalog | `data-period-outline` + 1px Outline shell on dial / TimeScroll / TimeInput |
| Hosts | Desktop + Android period columns bordered CornerSmall 8, no item gap |
| Inventory | Time picker notes PeriodSelector 1dp Outline shell |
| Tests | token 1 / 80 / 0 + catalog `data-period-outline` |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture PeriodSelector outline against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range + docked + WeekDays + 360×568 + pads + empty headlines + SelectableDates | Recapture stills when screenshot works |
| Search | Compact/docked + fullscreen surfaceContainerLow | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + CornerLarge 16 + field outline + TimeSelector colors + **PeriodSelector Outline shell** + CustomLayout + ClockFaceSizeModifier | Recapture stills when screenshot works |

## v118 leftovers

See `docs/qa/v117_leftovers.md`.
