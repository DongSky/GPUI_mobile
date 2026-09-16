# Desktop capture v80

**Date:** 2026-09-16  
**Landing:** `main` only (no pull request), continuous mode for [DongSky/GPUI_mobile](https://github.com/DongSky/GPUI_mobile).  
**HEAD at capture:** v80 after v79 DateRangePicker VerticalMonthsList (`451d116`).  
**Official:** [m3.material.io date pickers](https://m3.material.io/components/date-pickers/specs) — docked DatePicker trailing DateRange icon button (spec item 4).

v80 started from v79 leftovers (`docs/qa/v79_leftovers.md`). Recapture still blocked (no wgpu; Chrome `--screenshot` hangs). Highest-impact **implementable** spec delta vs current Expressive (no NDK/gpui bump): **trailing DateRange icon on the docked outlined field**. Do not reopen range-hero VerticalMonthsList, docked live select or YearPicker, DatePicker month nav or Confirm/Cancel, range-hero connectors, Confirm/Cancel, showModeToggle, YearPicker, month nav, or live start→end. Live IME, caret bounds, search `cx.transform`, and `gpui::LineCap` stay blocked. Critique vs current Expressive: **do not add banners or nav drawers**. OS `PopUp` and predictive-back stay skipped.

## What shipped

| Surface | Change |
|---|---|
| `date_picker.rs` | `DOCKED_TRAILING`; `DOCKED_TRAILING_ICON` / `_LABEL` / 48dp target; `apply_docked_toggle` |
| Catalog | `data-docked-trailing`; trailing ▦ on the outlined field; icon toggles popup |
| Hosts | Desktop + Android trailing overlay + `toggle_docked_open` |
| Inventory | Date picker notes trailing DateRange icon |
| Tests | Toggle helper, catalog trailing attr + glyph |

Host tests (this VM): **35** `gpui_material`, **17** `gpui_android`, **1** `material_desktop_demo`. Android crate is `cfg(target_os = "android")`.

## Side-by-side strips

New HTML-vs-official PNGs were **not** written this pass: Chrome `--screenshot` hangs in this VM (`dump-dom` succeeds; no wgpu display). Keep v19 strips for the official left halves. Recapture docked trailing DateRange against `docs/catalog/material-catalog-light.html` when screenshot works.

## Critique vs Expressive

| Hero | Match | Gap |
|---|---|---|
| Date picker | Modal + range VerticalMonthsList + docked YearPicker + live select + **trailing DateRange** | Recapture stills when screenshot works |
| Search | Compact/docked + groups + segmented + two-line + clear-X + 40/20 + no-results + filter chips | `cx.transform` search morph skipped |
| Time picker | TimeScroll + TimeInput + 24h ClockFace + Horizontal | Recapture stills when screenshot works |

## v81 leftovers

See `docs/qa/v80_leftovers.md`.
