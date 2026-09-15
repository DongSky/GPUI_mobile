//! HTML catalog generated from the same resolve() functions the GPUI demo uses.

use crate::components::{
    badge, bottom_sheet, button, button_group, card, carousel, checkbox, chip, date_picker, dialog,
    divider, fab, fab_menu, icon_button, list, menu, navigation_bar, navigation_rail, photo_stub,
    progress, radio, search, side_sheet, slider, snackbar, split_button, switch, tabs, text_field,
    time_picker, toolbar, tooltip, top_app_bar, Appearance,
};
use crate::elevation::ElevationLevels;
use crate::inventory::{self, Parity};
use crate::state::InteractionState;
use crate::theme::Theme;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn paint_photo_class(kind: photo_stub::PhotoKind, extra: &str, class: &str) -> String {
    format!(
        r#"<div class="{class}" data-photo="{label}" data-decoded-jpeg="1" data-licensed-camera="1" data-photo-license="{lic}" data-photo-source="{src}" style="background:{css};{extra}"></div>"#,
        label = kind.label(),
        lic = kind.credit().license,
        src = kind.credit().source,
        css = kind.css_background(),
    )
}

fn paint_avatar(kind: photo_stub::PhotoKind, size: f32) -> String {
    format!(
        r#"<div class="photo-stub av" data-photo="{label}" data-decoded-jpeg="1" data-licensed-camera="1" data-photo-license="{lic}" style="width:{size}px;height:{size}px;border-radius:{r}px;flex:0 0 {size}px;background:{css}"></div>"#,
        label = kind.label(),
        lic = kind.credit().license,
        r = size / 2.0,
        css = kind.css_background(),
    )
}

fn pill(label: &str, bg: &str, fg: &str) -> String {
    format!(
        "<span class=\"pill\" style=\"background:{bg};color:{fg}\">{}</span>",
        esc(label)
    )
}

fn state_row_open(label: &str) -> String {
    format!(
        "<div class=\"state\"><div class=\"state-name\">{}</div><div class=\"state-body\">",
        esc(label)
    )
}

pub fn render_html(theme: &Theme) -> String {
    let c = theme.color;
    let mode = if theme.dark { "dark" } else { "light" };
    let mut body = String::new();

    body.push_str(&hero(theme));
    body.push_str(&inventory_section());
    body.push_str(&color_section(theme));
    body.push_str(&type_section(theme));
    body.push_str(&settings_scene(theme));
    body.push_str(&buttons(theme));
    body.push_str(&icon_buttons(theme));
    body.push_str(&fabs(theme));
    body.push_str(&fab_menu_section(theme));
    body.push_str(&split_button_section(theme));
    body.push_str(&toolbar_section(theme));
    body.push_str(&text_fields(theme));
    body.push_str(&selection(theme));
    body.push_str(&lists(theme));
    body.push_str(&chips(theme));
    body.push_str(&cards(theme));
    body.push_str(&chrome(theme));
    body.push_str(&tooltips_section(theme));
    body.push_str(&app_bars(theme));
    body.push_str(&progress_section(theme));
    body.push_str(&dialogs(theme));
    body.push_str(&sheets(theme));
    body.push_str(&side_sheets(theme));
    body.push_str(&menus(theme));
    body.push_str(&sliders(theme));
    body.push_str(&tabs_section(theme));
    body.push_str(&badges(theme));
    body.push_str(&date_pickers(theme));
    body.push_str(&search_section(theme));
    body.push_str(&time_picker_section(theme));
    body.push_str(&carousel_section(theme));
    body.push_str(&motion_section(theme));

    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="{mode}">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1"/>
<title>GPUI Material 3 Expressive catalog ({mode})</title>
<link rel="preconnect" href="https://fonts.googleapis.com"/>
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
<link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap" rel="stylesheet"/>
<style>
:root {{
  --bg: {bg};
  --on-bg: {on_bg};
  --surface: {surface};
  --on-surface: {on_surface};
  --outline: {outline};
  --primary: {primary};
  --on-primary: {on_primary};
  --surface-low: {surface_low};
}}
* {{ box-sizing: border-box; }}
html, body {{
  margin: 0; padding: 0;
  background: var(--bg);
  color: var(--on-bg);
  font-family: Roboto, system-ui, sans-serif;
}}
body {{ max-width: 1100px; margin: 0 auto; padding: 24px 20px 80px; }}
h1 {{ font-size: {h1s}px; line-height: {h1l}px; font-weight: {h1w}; margin: 0 0 8px; letter-spacing: {h1t}px; }}
h2 {{ font-size: 24px; line-height: 32px; font-weight: 400; margin: 40px 0 8px; }}
h3 {{ font-size: 16px; line-height: 24px; font-weight: 500; margin: 20px 0 8px; letter-spacing: 0.15px; }}
p.lead, p.note {{ color: {on_var}; font-size: 14px; line-height: 20px; }}
a {{ color: var(--primary); }}
.bar {{
  display: flex; align-items: center; justify-content: space-between;
  height: 64px; padding: 0 8px 0 16px;
  background: {appbar};
}}
.bar .title {{ font-size: 22px; line-height: 28px; }}
.grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 8px; }}
.swatch {{
  border-radius: 12px; min-height: 88px; padding: 12px;
  display: flex; flex-direction: column; justify-content: flex-end;
  font-size: 11px; line-height: 16px; letter-spacing: 0.5px; font-weight: 500;
}}
.swatch code {{ font-weight: 400; opacity: 0.85; }}
.state {{ display: grid; grid-template-columns: 120px 1fr; gap: 12px; align-items: center; margin: 8px 0; }}
.state-name {{ font-size: 12px; color: {on_var}; }}
.state-body {{ display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }}
.btn, .chip, .card-demo, .field, .list-item, .snack, .nav {{
  display: inline-flex; align-items: center; justify-content: center;
  font-family: Roboto, sans-serif; border: none; text-decoration: none;
}}
.btn {{
  height: 40px; min-width: 64px; padding: 0 16px;
  border-radius: 20px; font-size: 14px; line-height: 20px; font-weight: 500;
  letter-spacing: 0.1px;
}}
.icon-btn {{
  width: 40px; height: 40px; border-radius: 20px;
  display: inline-flex; align-items: center; justify-content: center;
  font-size: 18px; flex: 0 0 auto;
}}
.fab {{
  width: 56px; height: 56px; border-radius: 16px;
  display: inline-flex; align-items: center; justify-content: center;
  font-size: 22px;
}}
.hero-card {{
  background: {surface_low}; border-radius: 28px; padding: 24px; margin: 12px 0 20px;
  display: flex; flex-direction: column; gap: 16px;
}}
.btn:active {{ border-radius: var(--press-r, 8px) !important; }}
.icon-btn:active {{ border-radius: var(--press-r, 8px) !important; }}
.chip[data-chip-morph="1"]:active {{ border-radius: var(--press-r, 8px) !important; }}
.btn-group {{
  display: flex; gap: {gap}px; align-items: stretch; flex-wrap: wrap;
}}
.btn-group[data-button-group="standard"] {{ gap: 12px; }}
.btn-connected {{ min-width: 72px; }}
.btn-connected.selected {{ font-weight: 700; }}
.btn-standard {{ min-width: 0; flex: 0 0 auto; box-sizing: border-box; justify-content: center; }}
.btn-standard.selected {{ font-weight: 700; }}
.docked {{
  display: flex; flex-direction: column; align-items: stretch;
  max-width: 360px; position: relative;
}}
.docked .cal {{ margin-top: 4px; border-top-left-radius: 8px; }}
.docked[data-popup="open"] .cal {{ box-shadow: 0 4px 12px rgba(0,0,0,.22); }}
.settings-scene {{ display: flex; flex-direction: column; gap: 24px; padding: 16px; }}
.settings-scene h3 {{ margin: 0; }}
.settings-block {{ display: flex; flex-direction: column; gap: 8px; }}
.settings-block h4 {{ margin: 0; font-size: 16px; line-height: 24px; font-weight: 700; }}
.search-bar[data-hidden="1"] {{ display: none; }}
.search-morph, .search-bar, .search-view {{
  display: flex; flex-direction: column; max-width: 720px; overflow: hidden;
  min-height: 56px; border-radius: 28px;
  transform-origin: top center;
  transition: min-height 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90),
    border-radius 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90),
    margin 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90),
    transform 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.search-morph .sv-head, .search-bar {{
  display: flex; align-items: center; gap: 16px;
  height: 56px; padding: 0 16px;
}}
.search-morph[data-open="1"], .search-view[data-search-activity="1"] {{
  border-radius: 0; max-width: none; min-height: 320px; margin: 0; transform: scale(1);
}}
.search-morph[data-open="0"] {{
  margin: 0 16px; transform: scale(0.94);
}}
.search-morph[data-search-style="contained"][data-width-class="medium"],
.search-morph[data-search-style="contained"][data-search-expanded="docked"][data-open="1"],
.search-view[data-search-style="contained"][data-search-expanded="docked"] {{
  border-radius: 28px; max-width: 720px; min-height: 280px; margin: 12px; transform: none;
}}
.search-morph[data-search-style="contained"][data-width-class="medium"][data-open="0"] {{
  margin: 24px; min-height: 56px; transform: none;
}}
.search-morph[data-search-style="contained"][data-width-class="compact"][data-open="1"],
.search-morph[data-search-style="contained"][data-search-expanded="fullscreen"] {{
  border-radius: 0; max-width: none; min-height: 320px; margin: 0; transform: none;
}}
.search-morph[data-search-style="contained"][data-width-class="compact"][data-open="0"] {{
  border-radius: 28px; max-width: 720px; margin: 16px; min-height: 56px; transform: none;
}}
.search-morph[data-search-style="contained"] .sv-divider {{ display: none; }}
.search-morph[data-open="0"] .sv-list {{ max-height: 0; opacity: 0; }}
.search-morph .lead {{
  position: relative; width: 24px; height: 24px; flex: 0 0 24px;
}}
.search-morph .lead-docked, .search-morph .lead-activity {{
  position: absolute; inset: 0; display: flex; align-items: center; justify-content: center;
  transition: opacity 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.search-morph[data-open="1"] .lead-docked, .search-morph[data-open="1"] .avatar {{
  opacity: 0; pointer-events: none;
}}
.search-morph[data-open="0"] .lead-activity {{
  opacity: 0; pointer-events: none;
}}
.search-morph .avatar {{
  transition: opacity 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.search-bar .ico {{ width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; font-size: 18px; }}
.search-bar .hint {{ flex: 1; font-size: 16px; line-height: 24px; }}
.search-bar .avatar {{ width: 30px; height: 30px; border-radius: 15px; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 500; }}
.search-view input, .search-morph input {{
  border: none; outline: none; background: transparent; flex: 1;
  font: 400 16px/24px Roboto, sans-serif; color: inherit;
}}
.search-view .sv-head, .search-morph .sv-head {{
  display: flex; align-items: center; gap: 16px; padding: 0 16px;
}}
.search-view .sv-list, .search-morph .sv-list {{
  display: flex; flex-direction: column;
  max-height: 480px; opacity: 1; overflow: hidden;
  transition: max-height 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90),
    opacity 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.search-view .sv-row, .search-morph .sv-row {{
  display: flex; align-items: center; gap: 16px; padding: 0 16px;
  min-height: 56px; font-size: 16px;
}}
.timepicker {{
  display: flex; flex-direction: column; gap: 16px; padding: 24px; max-width: 360px;
}}
.timepicker .time-row {{ display: flex; align-items: center; gap: 12px; }}
.timepicker .time-fields {{ display: flex; align-items: center; gap: 4px; }}
.timepicker .time-field {{
  min-width: 64px; padding: 8px 12px; border-radius: 8px; text-align: center;
  cursor: pointer; border: 2px solid transparent;
}}
.timepicker .clock {{
  position: relative; border-radius: 50%; flex: 0 0 auto;
}}
.timepicker .hour, .timepicker .minute {{
  position: absolute; display: flex; align-items: center; justify-content: center;
  border-radius: 50%;
}}
.timepicker .hand-svg {{
  position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none;
  transform-origin: 50% 50%;
  transition: transform 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.timepicker .second-hand-svg {{
  position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none;
  transform-origin: 50% 50%;
}}
.timepicker .hub {{
  position: absolute; left: 50%; top: 50%; width: 8px; height: 8px;
  margin: -4px 0 0 -4px; border-radius: 50%; pointer-events: none;
}}
.period {{ display: flex; flex-direction: column; gap: 8px; }}
.period button {{
  width: 52px; height: 36px; border: none; border-radius: 8px; font-weight: 700; cursor: pointer;
}}
.time-scroll {{
  display: flex; flex-direction: column; gap: 16px; padding: 24px; max-width: 360px;
}}
.time-scroll .scroll-row {{ display: flex; align-items: center; gap: 8px; }}
.time-scroll .scroll-field {{
  position: relative; overflow: hidden; flex: 0 0 auto; touch-action: none;
}}
.time-scroll .scroll-item {{
  position: absolute; left: 0; right: 0; display: flex; align-items: center; justify-content: center;
  user-select: none; cursor: pointer;
}}
.time-scroll .scroll-colon {{
  display: flex; align-items: center; justify-content: center; pointer-events: none;
}}
.time-expressive {{
  display: flex; flex-direction: column; gap: 16px; padding: 24px; max-width: 360px;
}}
.time-expressive .time-display-head {{
  display: flex; align-items: center; justify-content: space-between; gap: 12px;
}}
.time-expressive .time-display-actions {{
  display: flex; align-items: center; gap: 4px;
}}
.time-expressive .time-display-toggle, .time-expressive .time-format-toggle {{
  width: 48px; height: 48px; border: none; background: transparent; cursor: pointer;
  font-size: 20px; line-height: 24px; border-radius: 24px;
}}
.time-expressive .time-format-toggle {{ font-weight: 700; font-size: 16px; }}
.time-expressive[data-time-display="input"] [data-time-scroll] {{ display: none; }}
.time-expressive[data-time-display="scroll"] [data-time-input] {{ display: none; }}
.time-expressive[data-time-format="24"] .period {{ display: none; }}
.time-input-row {{ display: flex; align-items: center; gap: 8px; }}
.time-input-field {{
  width: 96px; height: 72px; border: none; border-radius: 28px; text-align: center;
  font: 500 57px/64px Roboto, sans-serif; padding: 0;
}}
.time-input-colon {{
  display: flex; align-items: center; justify-content: center; pointer-events: none;
}}
.time-input .period button {{ height: 32px; }}
.field {{
  width: 280px; height: 56px; padding: 8px 16px;
  display: flex; flex-direction: column; justify-content: center; align-items: flex-start;
}}
.field .lab {{ font-size: 12px; line-height: 16px; }}
.field .val {{ font-size: 16px; line-height: 24px; }}
.field-wrap {{ display: flex; flex-direction: column; gap: 4px; }}
.ol {{
  width: 280px; min-height: 56px; margin: 8px 0 0; padding: 0 12px 8px;
  display: flex; align-items: center; gap: 12px; background: transparent;
  position: relative; border: none;
}}
.ol[data-label-position="inside"] {{
  flex-direction: column; justify-content: flex-end; align-items: stretch;
  padding: 6px 16px 8px; box-sizing: border-box; gap: 0;
}}
.ol[data-label-position="inside"][data-floating="0"] {{ justify-content: center; }}
.ol[data-label-position="inside"][data-floating="1"] .lab {{
  font-size: 12px; line-height: 16px;
}}
.filled-hero[data-field-style="expressive"] {{
  box-shadow: none;
}}
.ol legend {{
  padding: 0 4px; margin-left: 8px; font-size: 12px; line-height: 16px;
  position: relative; z-index: 1; background: transparent;
}}
.ol .ol-evenodd {{
  position: absolute; inset: 0; width: 100%; height: 100%;
  pointer-events: none; overflow: visible;
}}
.ol[data-notch="cutout"] {{ border-style: none; }}
.ol input, .filled-hero input {{
  border: none; outline: none; background: transparent; width: 100%;
  font: 400 16px/24px Roboto, sans-serif; color: inherit; padding: 8px 0 4px;
}}
.filled-hero {{
  width: 280px; height: 56px; padding: 6px 16px 8px; position: relative;
  display: flex; flex-direction: column; justify-content: flex-end;
}}
.filled-hero .lab {{ font-size: 12px; line-height: 16px; }}
.filled-hero.empty {{ justify-content: center; }}
.filled-hero.empty .lab {{ font-size: 16px; line-height: 24px; }}
.xslider {{
  position: relative; display: flex; align-items: center; width: 100%; max-width: 420px;
  min-height: 48px; height: auto; gap: 6px;
}}
.xseg {{
  position: relative; display: flex; align-items: center;
  box-sizing: border-box;
}}
.xstops {{
  position: absolute; left: 8px; right: 8px; top: 0; bottom: 0;
  display: flex; align-items: center; justify-content: space-between;
  pointer-events: none;
}}
.xstop {{
  width: 4px; height: 4px; border-radius: 2px; flex: 0 0 auto;
}}
.xtick {{
  position: absolute; width: 4px; height: 4px; border-radius: 2px;
  top: 50%; transform: translate(-50%, -50%); pointer-events: none; z-index: 2;
}}
.xhandle {{ flex: 0 0 auto; border-radius: 2px; position: relative; z-index: 1; }}
.slider-row {{ display: flex; align-items: center; gap: 12px; width: 100%; }}
.slider-meta {{ display: flex; flex-direction: column; flex: 1; gap: 4px; min-width: 0; }}
.slider-label {{ font-size: 12px; line-height: 16px; color: {on_surface}; }}
.slider-icon {{
  width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;
  color: {on_var}; font-size: 18px;
}}
.support {{ font-size: 12px; line-height: 16px; padding: 0 16px; }}
.list-item {{
  width: 100%; max-width: 420px; padding: 8px 16px;
  display: flex; flex-direction: column; justify-content: center; align-items: flex-start;
}}
.list-item .h {{ font-size: 16px; line-height: 24px; }}
.list-item .s {{ font-size: 14px; line-height: 20px; }}
.list-group {{
  display: flex; flex-direction: column; gap: 2px;
  width: 100%; max-width: 420px; margin: 8px 0 16px;
}}
.list-item.segmented {{
  flex-direction: row; align-items: center; gap: 12px;
  box-sizing: border-box; cursor: pointer;
}}
.list-item.segmented .lead {{
  width: 20px; height: 20px; flex: 0 0 20px;
  display: flex; align-items: center; justify-content: center; font-size: 16px;
}}
.list-item.segmented .meta {{ flex: 1; min-width: 0; display: flex; flex-direction: column; }}
.list-item.segmented .trail {{ margin-left: auto; flex: 0 0 auto; }}
.list-swipe {{
  position: relative; overflow: hidden; width: 100%; max-width: 420px;
  margin: 8px 0 16px;
}}
.list-swipe .rails {{
  position: absolute; inset: 0; display: flex; align-items: stretch;
  pointer-events: none;
}}
.list-swipe .rail {{
  width: 80px; display: flex; align-items: center; justify-content: center;
  font-size: 12px; font-weight: 500; flex: 0 0 auto;
}}
.list-swipe .rail.trail {{ margin-left: auto; }}
.list-swipe .sheet {{
  position: relative; z-index: 1; display: flex; flex-direction: column; gap: 0;
  background: var(--surface);
}}
.list-swipe .list-item {{ max-width: none; width: 100%; cursor: grab; touch-action: pan-y; }}
.list-reorder .handle {{
  width: 24px; height: 24px; flex: 0 0 24px; display: flex;
  align-items: center; justify-content: center; letter-spacing: -2px;
  cursor: grab; font-size: 14px; opacity: 0.7;
}}
.tooltip-plain {{ position: relative; }}
.tooltip-caret {{
  width: 0; height: 0;
  border-left: 8px solid transparent;
  border-right: 8px solid transparent;
  border-top-width: 8px; border-top-style: solid;
}}
.tooltip-caret.up {{
  border-top: none;
  border-bottom-width: 8px; border-bottom-style: solid;
}}
.chip {{
  height: 32px; padding: 0 16px; border-radius: 16px; font-size: 14px; font-weight: 500;
  gap: 8px; box-sizing: border-box; cursor: pointer;
  transition: border-radius 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.chip[data-chip-compact="1"] {{ gap: 4px; }}
.chip .chip-ico {{ width: 18px; height: 18px; display: inline-flex; align-items: center; justify-content: center; font-size: 14px; }}
.chip .chip-av {{
  width: 24px; height: 24px; border-radius: 12px; flex: 0 0 24px; overflow: hidden;
}}
.menu-anchor-stage {{
  display: flex; flex-direction: column; align-items: flex-start; gap: 8px;
  position: relative; min-width: 360px; padding: 24px;
}}
.card-demo {{
  width: 220px; min-height: 88px; border-radius: 12px; padding: 16px;
  flex-direction: column; align-items: flex-start; justify-content: center;
}}
.check, .radio {{
  width: 48px; height: 48px; display: inline-flex; align-items: center; justify-content: center;
  border-radius: 24px;
}}
.box {{ width: 18px; height: 18px; border-radius: 2px; display: flex; align-items: center; justify-content: center; font-size: 14px; }}
.dot {{ width: 20px; height: 20px; border-radius: 10px; display: flex; align-items: center; justify-content: center; }}
.dot i {{ width: 10px; height: 10px; border-radius: 5px; display: block; }}
.switch {{
  width: 52px; height: 32px; border-radius: 16px; position: relative;
  display: inline-block;
}}
.switch b {{
  position: absolute; top: 50%; transform: translateY(-50%);
  border-radius: 50%;
}}
.linear {{ width: 240px; height: 4px; border-radius: 2px; overflow: hidden; position: relative; }}
.linear i {{ display: block; height: 100%; }}
.linear.indet i {{
  position: absolute; left: 0; top: 0;
  animation: m3indet 1400ms {ease} infinite;
}}
.circ {{
  width: 48px; height: 48px; border-radius: 24px;
}}
.circ.indet, .ptr .circ {{ animation: m3spin 1200ms linear infinite; }}
.ptr {{ display: flex; flex-direction: column; align-items: center; gap: 8px; margin: 12px 0; }}
.loading-row {{ display: flex; align-items: center; gap: 16px; margin: 12px 0; }}
.loading-contained {{
  width: 48px; height: 48px; border-radius: 24px;
  display: flex; align-items: center; justify-content: center;
}}
.loading-shape {{ display: block; }}
.hand-svg[data-hour-live="1"] {{
  animation: hourHandLive 8s linear infinite;
  transform-origin: 50% 50%;
}}
@keyframes hourHandLive {{
  from {{ transform: rotate(var(--hand-base, 0deg)); }}
  to {{ transform: rotate(calc(var(--hand-base, 0deg) + 15deg)); }}
}}
@keyframes m3indet {{
  0% {{ transform: translateX(-120%); }}
  100% {{ transform: translateX(340%); }}
}}
@keyframes m3spin {{
  to {{ transform: rotate(360deg); }}
}}
.snack {{
  min-height: 48px; padding: 0 16px; border-radius: 4px; gap: 16px;
  justify-content: space-between; min-width: 280px;
}}
.nav {{
  width: 100%; max-width: 420px; height: 64px; border-radius: 0;
  justify-content: space-around; padding: 8px 0 6px;
}}
.nav .dest {{ display: flex; flex-direction: column; align-items: center; gap: 6px; font-size: 12px; font-weight: 500; }}
.nav .ind {{ width: 56px; height: 32px; border-radius: 16px; display: flex; align-items: center; justify-content: center; }}
.nav[data-layout="horizontal"] {{
  max-width: 720px; justify-content: center; gap: 8px; padding: 12px 16px;
}}
.nav[data-layout="horizontal"] .dest {{ flex-direction: row; gap: 0; }}
.nav[data-layout="horizontal"] .ind {{
  width: auto; height: 40px; padding: 0 16px; gap: 4px;
  display: flex; align-items: center; justify-content: center; white-space: nowrap;
}}
.tooltip-stage {{
  display: flex; flex-wrap: wrap; gap: 32px; align-items: flex-end; padding: 16px 0;
}}
.tooltip-anchor {{
  display: flex; flex-direction: column; align-items: center; gap: 4px;
}}
.tooltip-anchor[data-tooltip-trigger="hover"] .tooltip-bubble {{
  opacity: 0; pointer-events: none; transition: opacity 120ms;
}}
.tooltip-anchor[data-tooltip-trigger="hover"]:hover .tooltip-bubble,
.tooltip-anchor[data-tooltip-trigger="hover"]:focus-within .tooltip-bubble,
.tooltip-anchor[data-open="1"] .tooltip-bubble {{
  opacity: 1; pointer-events: auto;
}}
.tooltip-plain {{
  min-height: 24px; min-width: 40px; max-width: 200px;
  padding: 4px 8px; border-radius: 4px;
  display: inline-flex; align-items: center; justify-content: center;
  font-size: 12px; line-height: 16px;
}}
.tooltip-bubble {{ display: flex; flex-direction: column; align-items: center; }}
.tooltip-rich {{
  max-width: 320px; padding: 12px 16px 8px; border-radius: 12px;
  display: flex; flex-direction: column; gap: 4px;
}}
.tooltip-rich .sub {{ font-size: 14px; line-height: 20px; font-weight: 500; }}
.tooltip-rich .body {{ font-size: 14px; line-height: 20px; }}
.tooltip-rich .acts {{ display: flex; gap: 16px; justify-content: flex-end; padding-top: 8px; }}
.tooltip-rich .acts span {{ font-size: 14px; line-height: 20px; font-weight: 500; cursor: pointer; }}
.nav-rail {{
  width: 96px; display: flex; flex-direction: column; align-items: center; gap: 12px;
  padding: {content_pad_v}px {content_pad_h}px; border-radius: 0; position: relative; overflow: hidden;
  box-sizing: border-box;
}}
.nav-rail[data-rail-layout="modal"],
.nav-rail[data-rail-layout="narrow"] {{
  align-items: stretch;
}}
.nav-rail[data-rail-layout="modal"].expanded,
.nav-rail[data-rail-layout="modal"][data-nav-rail-expanded="1"],
.nav-rail[data-rail-layout="narrow"][data-nav-rail-expanded="1"],
.nav-rail[data-hide-on-collapse="1"] {{
  border-radius: 16px;
}}
.nav-rail[data-narrow="1"] {{ width: 80px; }}
.nav-rail .dest {{
  display: flex; flex-direction: column; align-items: center; gap: 4px;
  font-size: 12px; font-weight: 500; width: 96px; position: relative;
  box-sizing: border-box;
}}
.nav-rail[data-narrow="1"] .dest {{ width: 80px; }}
.nav-rail .ind {{
  width: 56px; height: 32px; border-radius: 16px;
  display: flex; align-items: center; justify-content: center; position: relative;
}}
.nav-rail .dest[data-active="1"] .ind {{ background: var(--ind); }}
.nav-rail .fab-slot {{
  width: 56px; height: 56px; border-radius: 16px;
  display: flex; align-items: center; justify-content: center; font-size: 24px;
}}
.nav-rail .fab-slot[data-rail-fab-extend="1"] {{
  box-sizing: border-box; overflow: hidden; white-space: nowrap;
  justify-content: flex-start; gap: 0; padding: 0 16px; margin-left: 20px;
  flex: 0 0 auto; font-size: 24px;
}}
.nav-rail[data-narrow="1"] .fab-slot[data-rail-fab-extend="1"] {{
  margin-left: 12px;
}}
.nav-rail .fab-slot[data-rail-fab-extend="1"] .fab-label {{
  font-size: 14px; line-height: 20px; font-weight: 500; opacity: 0;
}}
.nav-rail.expanded .fab-slot[data-rail-fab-extend="1"],
.nav-rail[data-nav-rail-expanded="1"] .fab-slot[data-rail-fab-extend="1"] {{
  width: calc(100% - 32px); margin-left: 16px; gap: 8px;
}}
.nav-rail.expanded .fab-slot[data-rail-fab-extend="1"] .fab-label,
.nav-rail[data-nav-rail-expanded="1"] .fab-slot[data-rail-fab-extend="1"] .fab-label {{
  opacity: 1;
}}
.nav-rail[data-hide-on-collapse="1"] .fab-slot[data-rail-fab-extend="1"],
.nav-rail[data-hide-on-collapse="1"] .fab-slot[data-hide-fab-extend="1"] {{
  width: calc(100% - 32px); margin-left: 16px; gap: 8px;
}}
.nav-rail[data-hide-on-collapse="1"] .fab-slot[data-rail-fab-extend="1"] .fab-label,
.nav-rail[data-hide-on-collapse="1"] .fab-slot[data-hide-fab-extend="1"] .fab-label {{
  opacity: 1;
}}
.nav-rail .dot {{
  position: absolute; top: 2px; right: 18px; min-width: 16px; height: 16px;
  border-radius: 8px; font-size: 10px; display: flex; align-items: center; justify-content: center;
}}
.nav-rail .dot.small {{ width: 6px; height: 6px; min-width: 6px; right: 22px; top: 6px; }}
.nav-rail {{ transition: width 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90), box-shadow 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90), border-radius 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90), background-color 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90); }}
.nav-rail[data-icon-morphing="1"] {{ transition: none; align-items: stretch; }}
.nav-rail[data-icon-morphing="1"] .dest,
.nav-rail[data-icon-morphing="1"].expanded .dest,
.nav-rail[data-icon-morphing="1"][data-icon-position="start"] .dest {{
  position: relative; display: block; flex-direction: unset; justify-content: unset;
  align-items: unset; box-sizing: border-box;
  width: var(--dest-w); height: var(--dest-h); margin: 0 0 0 var(--dest-ml);
  padding: 0; gap: 0; border-radius: var(--dest-r); background: var(--dest-bg, transparent);
  font-size: var(--lbl-size);
}}
.nav-rail[data-icon-morphing="1"] .ind,
.nav-rail[data-icon-morphing="1"].expanded .ind,
.nav-rail[data-icon-morphing="1"][data-icon-position="start"] .ind {{
  position: absolute; left: var(--icon-left); top: var(--icon-top);
  width: var(--icon-w); height: var(--icon-h); border-radius: calc(var(--icon-h) / 2);
  background: var(--icon-bg, transparent);
}}
.nav-rail[data-icon-morphing="1"] .dest[data-active="1"] .ind,
.nav-rail[data-icon-morphing="1"].expanded .dest[data-active="1"] .ind,
.nav-rail[data-icon-morphing="1"][data-icon-position="start"] .dest[data-active="1"] .ind {{
  background: var(--icon-bg, transparent);
}}
.nav-rail[data-icon-morphing="1"] .lbl {{
  position: absolute; left: var(--lbl-left); top: var(--lbl-top);
  width: var(--lbl-w); text-align: var(--lbl-align); font-size: var(--lbl-size);
  line-height: var(--lbl-line);
}}
.nav-rail.expanded, .nav-rail[data-icon-position="start"] {{ width: 220px; align-items: stretch; }}
.nav-rail[data-wide-collapsed="1"] {{ width: 96px; }}
.nav-rail[data-wide-collapsed="1"] .dest {{ width: 96px; }}
.nav-rail[data-narrow="1"]:not(.expanded):not([data-icon-position="start"]) {{ width: 80px; }}
.nav-rail[data-narrow="1"]:not(.expanded):not([data-icon-position="start"]) .dest {{ width: 80px; }}
.nav-rail.expanded .dest, .nav-rail[data-icon-position="start"] .dest {{
  width: auto; flex-direction: row; justify-content: flex-start; align-items: center;
  margin: 0 16px; padding: 0 16px; gap: 8px; height: 56px; border-radius: 28px;
  font-size: 14px; line-height: 20px;
}}
.nav-rail.expanded .dest[data-active="1"],
.nav-rail[data-icon-position="start"] .dest[data-active="1"] {{
  background: var(--ind);
}}
.nav-rail.expanded .ind, .nav-rail[data-icon-position="start"] .ind {{
  width: 24px; height: 24px; background: transparent; border-radius: 0;
}}
.nav-rail.expanded .dest[data-active="1"] .ind,
.nav-rail[data-icon-position="start"] .dest[data-active="1"] .ind {{
  background: transparent;
}}
.wide-rail-pair {{
  display: flex; gap: 24px; align-items: flex-start; flex-wrap: wrap; max-width: 720px;
}}
.wide-rail-pair .nav-rail {{
  box-shadow: 0 1px 2px rgba(0,0,0,.12);
  min-height: 280px;
}}
.rail-stage {{ position: relative; min-height: 280px; max-width: 720px; }}
.rail-stage.is-standard {{
  display: flex; flex-direction: row; align-items: stretch; overflow: hidden;
}}
.rail-stage.is-standard .nav-rail {{
  flex: 0 0 auto; box-shadow: none; min-height: 280px; z-index: 1;
}}
.rail-inflow-body {{
  flex: 1 1 auto; min-width: 0; padding: 16px; font-size: 14px; line-height: 20px;
}}
.rail-window {{
  position: absolute; left: 0; top: 0; bottom: 0; z-index: 2;
  pointer-events: auto;
}}
.rail-window[data-rail-window="1"] {{
  box-shadow: 0 8px 24px rgba(0,0,0,.28);
}}
.rail-scrim {{
  position: absolute; inset: 0; border-radius: 12px; z-index: 0;
  opacity: 0; pointer-events: none;
  transition: opacity 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.rail-stage.is-modal .rail-scrim, .rail-scrim[data-visible="1"] {{
  opacity: 1; pointer-events: auto;
}}
.rail-stage.is-hide {{
  position: relative; overflow: hidden; min-height: 280px;
  display: flex; flex-direction: row; align-items: stretch;
}}
.rail-stage.is-hide .rail-inflow-body {{
  flex: 1 1 auto; min-width: 0; padding: 16px; font-size: 14px; line-height: 20px;
}}
.rail-menu {{
  width: 56px; height: 56px; border-radius: 16px; border: 0; margin: 16px 0 0 16px;
  display: flex; align-items: center; justify-content: center; font-size: 22px;
  cursor: pointer; flex: 0 0 auto; z-index: 1;
}}
.nav-rail[data-hide-on-collapse="1"] {{
  width: 220px; align-items: stretch;
  transform: translateX(-100%);
  transition: transform 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90);
}}
.nav-rail[data-hide-on-collapse="1"][data-nav-rail-expanded="1"],
.nav-rail[data-hide-on-collapse="1"].expanded {{
  transform: translateX(0);
}}
.nav-rail[data-hide-sliding="1"] {{ transition: none; }}
.nav-rail .rail-dests {{
  display: flex; flex-direction: column; align-items: inherit; gap: 12px;
}}
.nav-rail[data-rail-arrangement="center"] {{
  position: relative;
}}
.nav-rail[data-rail-arrangement="center"] .fab-slot {{
  position: relative; z-index: 1;
}}
.nav-rail[data-rail-arrangement="center"] .rail-dests {{
  position: absolute; inset: 0;
  justify-content: center; align-items: stretch;
}}
.nav-rail[data-rail-layout="standard"],
.nav-rail[data-rail-header="1"] {{
  align-items: stretch; min-height: 280px;
}}
.rail-header {{
  position: relative; flex: 0 0 auto; z-index: 1;
  display: flex; flex-direction: column; align-items: stretch;
  width: 100%; gap: 12px;
}}
.rail-header-menu-row {{
  position: relative; flex: 0 0 auto;
  display: flex; flex-direction: column; align-items: flex-start;
  padding-left: 24px;
}}
.rail-header-btn {{
  width: 40px; height: 40px; border: 0; border-radius: 20px;
  background: transparent; display: flex; align-items: center; justify-content: center;
  font-size: 20px; cursor: pointer; padding: 0;
}}
.rail-header-tip {{
  display: none; position: absolute; bottom: calc(100% + 4px); left: 24px;
  min-height: 24px; max-width: 200px; padding: 4px 8px; border-radius: 4px;
  font-size: 12px; line-height: 16px; white-space: nowrap; pointer-events: none;
}}
.rail-header-menu-row:hover .rail-header-tip, .rail-header-btn:focus + .rail-header-tip,
.rail-header-menu-row[data-open="1"] .rail-header-tip {{ display: flex; align-items: center; }}
.nav-rail[data-rail-arrangement="bottom"] {{
  justify-content: flex-start;
}}
.nav-rail[data-rail-arrangement="bottom"] .rail-dests {{
  flex: 1 1 auto; justify-content: flex-end; align-items: stretch;
}}
.carousel {{ display: flex; gap: 8px; overflow: hidden; max-width: 720px; }}
.carousel .tile {{
  height: 168px; border-radius: 28px; display: flex; align-items: flex-end;
  padding: 16px; font-weight: 700; flex: 0 0 auto;
  background-size: 140% 140%; background-position: center;
}}
.carousel .tile[data-media="1"] {{
  background-image: linear-gradient(135deg, rgba(255,255,255,.18), transparent 55%);
}}
.overflow-menu {{
  position: relative; display: flex; align-items: flex-start; gap: 8px;
}}
.overflow-cascade {{
  display: flex; flex-direction: row; align-items: flex-end; gap: 4px;
  position: relative; min-width: 200px;
}}
.overflow-cascade[data-open="0"] {{ display: none; }}
.overflow-cascade[data-flyout="0"] .menu-flyout {{ display: none; }}
.snack {{
  min-height: 48px; padding: 0 16px; gap: 16px; justify-content: space-between;
  min-width: 280px;
}}
.wave {{ width: 240px; height: 16px; overflow: hidden; margin: 12px 0; }}
.wave svg {{ display: block; width: 240px; height: 16px; }}
.wave .wave-path {{ animation: m3wave 1200ms linear infinite; }}
@keyframes m3wave {{
  from {{ transform: translateX(0); }}
  to {{ transform: translateX(-20px); }}
}}
.month-nav {{ display: flex; align-items: center; justify-content: space-between; padding: 4px 8px; font-weight: 500; }}
.month-nav button {{ border: none; background: transparent; cursor: pointer; font-size: 18px; padding: 4px 8px; color: inherit; }}
table.inv {{ width: 100%; border-collapse: collapse; font-size: 13px; }}
table.inv th, table.inv td {{ text-align: left; padding: 8px 10px; border-bottom: 1px solid {outline_var}; vertical-align: top; }}
table.inv th {{ font-weight: 500; }}
.pill {{ display: inline-block; padding: 2px 8px; border-radius: 8px; font-size: 11px; font-weight: 500; }}
.divider {{ height: 1px; background: {outline_var}; margin: 8px 0; }}
.btn, .chip, .icon-btn, .fab, .field, .switch, .switch b, .nav .ind, .tab, .tab-ind, .slider-thumb, .day, .menu-item, .dialog, .sheet {{
  transition: {motion};
}}
.field input {{
  border: none; outline: none; background: transparent; width: 100%;
  font: inherit; color: inherit; padding: 0; margin: 0;
}}
.dialog {{
  min-width: 280px; max-width: 420px; padding: 24px;
  display: flex; flex-direction: column; gap: 16px;
}}
.dialog .actions {{ display: flex; justify-content: flex-end; gap: 8px; width: 100%; }}
.dialog .actions .btn {{ background: transparent; box-shadow: none; min-width: 64px; }}
.dialog-list {{ align-items: stretch; text-align: left; }}
.dialog-fullscreen {{
  min-width: 0; max-width: none; width: 100%; padding: 0; gap: 0;
  min-height: 320px; border-radius: 0;
}}
.dialog-fullscreen .fs-head {{
  display: flex; align-items: center; justify-content: space-between;
  height: 64px; padding: 0 8px 0 16px; gap: 12px;
}}
.dialog-fullscreen .fs-fields {{
  display: flex; flex-direction: column; gap: 12px; padding: 16px 24px 24px;
}}
.dialog-fullscreen .fs-field {{
  min-height: 56px; border-radius: 4px 4px 0 0; padding: 8px 16px;
  display: flex; align-items: center;
}}
.dialog-list .ringtone {{
  display: flex; align-items: center; justify-content: space-between;
  min-height: 48px; width: 100%;
}}
.accounts {{ width: 100%; display: flex; flex-direction: column; gap: 4px; text-align: left; }}
.account {{
  display: flex; align-items: center; gap: 12px;
  min-height: 48px; width: 100%;
}}
.avatar {{
  width: 40px; height: 40px; border-radius: 20px;
  display: flex; align-items: center; justify-content: center;
  font-size: 14px; font-weight: 500; flex: 0 0 auto;
}}
.scrim {{
  border-radius: 12px; padding: 24px; display: flex; justify-content: center;
}}
.sheet {{
  width: 100%; max-width: 480px; padding: 0 0 16px;
  display: flex; flex-direction: column; align-items: center;
}}
.sheet .handle {{ border-radius: 2px; margin: 16px 0 12px; }}
.menu {{
  min-width: 112px; max-width: 280px; padding: 4px; display: flex; flex-direction: column;
}}
.menu-item {{
  height: 44px; padding: 0 16px; display: flex; align-items: center; gap: 12px;
  box-sizing: border-box;
}}
.menu-item .lead {{
  width: 20px; height: 20px; flex: 0 0 20px; display: flex; align-items: center; justify-content: center;
  font-size: 14px; font-weight: 500;
}}
.menu-item .lbl {{ flex: 1; min-width: 0; }}
.menu-item .trail {{ margin-left: auto; font-size: 11px; letter-spacing: 0.5px; }}
.menu-stack {{
  display: flex; flex-direction: column; gap: 2px; min-width: 200px; max-width: 280px;
}}
.menu-group {{
  display: flex; flex-direction: column; padding: 4px;
}}
.menu-row {{
  display: flex; flex-direction: row; align-items: stretch; gap: 8px; flex-wrap: wrap; margin: 8px 0 16px;
}}
.menu-horizontal {{
  display: flex; flex-direction: row; align-items: center; gap: 2px; padding: 4px;
}}
.menu-horizontal .menu-item {{
  padding: 0 12px;
}}
.menu-icons {{
  display: flex; flex-direction: row; align-items: center; gap: 4px; padding: 4px;
}}
.menu-icons .menu-item {{
  width: 52px; height: 52px; padding: 0; justify-content: center;
}}
.menu-cascade {{
  display: flex; flex-direction: row; align-items: flex-end; gap: 4px;
  position: relative; min-width: 360px;
}}
.menu-cascade[data-open="0"] .menu-flyout {{ display: none; }}
.menu-overlay {{
  display: flex; flex-direction: row; align-items: flex-end; gap: 4px;
  position: relative; min-width: 280px;
}}
.menu-overlay[data-open="0"] .menu-flyout {{ display: none; }}
.menu-flyout {{
  min-width: 160px; max-width: 280px;
}}
.menu-item[data-menu-hi="1"] {{
  outline: 2px solid var(--primary); outline-offset: -2px;
}}
.slider {{
  position: relative; width: 240px; height: 48px; display: flex; align-items: center;
}}
.slider .track {{ position: relative; width: 100%; border-radius: 2px; overflow: visible; }}
.slider-thumb {{
  position: absolute; top: 50%; transform: translate(-50%, -50%);
  border-radius: 50%;
}}
.tabs {{ display: flex; position: relative; width: 100%; max-width: 420px; }}
.tab {{
  flex: 1; height: 48px; display: flex; flex-direction: column;
  align-items: center; justify-content: flex-end; padding-bottom: 8px;
  position: relative; font-weight: 500;
}}
.tab-ind {{ position: absolute; bottom: 0; border-radius: 3px 3px 0 0; }}
.phone {{
  width: 360px; max-width: 100%; border: 12px solid {on_surface};
  border-radius: 36px; overflow: hidden; position: relative;
  background: {surface}; display: flex; flex-direction: column;
}}
.phone .phone-bar {{
  height: 56px; display: flex; align-items: center; padding: 0 16px;
  font-size: 22px; line-height: 28px; font-weight: 500;
}}
.status-bar {{
  height: 24px; display: flex; align-items: center; justify-content: space-between;
  padding: 0 20px; font-size: 12px; line-height: 16px; font-weight: 500;
}}
.mail-list {{ display: flex; flex-direction: column; flex: 1; }}
.mail-row {{
  display: flex; flex-direction: row; align-items: center; gap: 12px;
  min-height: 72px; padding: 8px 16px; border-bottom: 1px solid {outline_var};
}}
.mail-avatar {{
  width: 40px; height: 40px; border-radius: 20px; flex: 0 0 40px;
  display: flex; align-items: center; justify-content: center;
  font-size: 14px; font-weight: 500;
}}
.mail-meta {{ flex: 1; min-width: 0; display: flex; flex-direction: column; }}
.mail-row .from {{ font-size: 16px; line-height: 24px; }}
.mail-row .subj {{ font-size: 14px; line-height: 20px; opacity: 0.8; }}
.mail-time {{ font-size: 12px; line-height: 16px; opacity: 0.7; flex: 0 0 auto; }}
.inbox-nav {{
  height: 64px; display: flex; align-items: flex-start; justify-content: space-around;
  flex: 0 0 auto; flex-wrap: nowrap; overflow: hidden; padding-top: 6px;
}}
.inbox-nav .dest {{
  display: flex; flex-direction: column; align-items: center; gap: 6px;
  font-size: 12px; flex: 1 1 0; min-width: 0; max-width: 80px;
}}
.inbox-nav .ind {{
  width: 56px; height: 32px; border-radius: 16px;
  display: flex; align-items: center; justify-content: center;
  font-size: 16px; line-height: 16px;
}}
.inbox-nav .nav-ico {{ display: block; }}
.mail-row[data-mail-peek="1"] {{
  max-height: 28px; min-height: 28px; padding-top: 0; padding-bottom: 0;
  overflow: hidden; opacity: 0.92; border-bottom: none;
}}
.mail-list {{ overflow: hidden; }}
.phone .snack {{
  position: relative; margin: 8px 16px; min-width: 0; z-index: 2;
  flex: 0 0 auto; flex-wrap: nowrap; white-space: nowrap;
}}
.snack .snack-close {{
  width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;
  cursor: pointer; font-size: 16px; flex: 0 0 auto;
}}
.media-grid {{
  display: grid; grid-template-columns: 1fr 1fr; gap: 8px; padding: 12px;
}}
.media-tile {{
  min-height: 96px; border-radius: 16px; padding: 12px;
  display: flex; align-items: flex-end; font-weight: 700;
}}
.appbar {{
  display: flex; flex-direction: column; width: 100%;
  transition: height 350ms cubic-bezier(0.42, 1.67, 0.21, 0.90),
    background 200ms ease, box-shadow 200ms ease;
  overflow: hidden;
}}
.appbar-row {{
  height: 64px; display: flex; align-items: center; gap: 8px;
  padding: 0 4px 0 8px; flex: 0 0 64px;
}}
.appbar-row .ico {{
  width: 48px; height: 48px; display: flex; align-items: center; justify-content: center;
  font-size: 20px; flex: 0 0 48px;
}}
.appbar-titles {{
  padding: 0 16px 16px; display: flex; flex-direction: column; gap: 2px;
  min-width: 0;
}}
.appbar[data-collapse="1"] .appbar-titles {{ display: none; }}
.appbar[data-collapse="1"] .appbar-inline {{ display: flex; flex-direction: column; flex: 1; min-width: 0; }}
.appbar-inline {{ display: none; min-width: 0; }}
.appbar-search {{
  flex: 1; height: 56px; border-radius: 28px; display: flex; align-items: center;
  gap: 12px; padding: 0 16px; margin: 4px 8px;
}}
.side-stage {{ position: relative; overflow: hidden; }}
.side-scrim {{ position: absolute; inset: 0; z-index: 1; }}
.side-sheet {{
  position: absolute; top: 0; bottom: 0; right: 0; z-index: 2;
  display: flex; flex-direction: column;
}}
.side-head {{
  display: flex; align-items: center; justify-content: space-between;
  padding: 16px 16px 12px 24px;
}}
.side-filters {{ display: flex; flex-direction: column; padding: 0 8px; flex: 1; }}
.side-filter {{
  display: flex; align-items: center; justify-content: space-between;
  min-height: 56px; padding: 8px 16px;
}}
.side-actions {{
  height: 72px; display: flex; align-items: center; padding: 16px 24px 24px;
}}
.share-stage {{ position: relative; min-height: 360px; justify-content: flex-end; }}
.share-grid {{
  display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 4px; padding: 8px;
  flex: 1;
}}
.share-tile {{
  min-height: 88px; border-radius: 12px; padding: 8px;
  display: flex; align-items: flex-end; font-size: 12px; font-weight: 500;
}}
.people-row {{
  display: flex; gap: 4px; padding: 4px 8px 12px; overflow-x: hidden;
  justify-content: space-between; width: 100%;
}}
.people {{
  width: 60px; display: flex; flex-direction: column; align-items: center; gap: 2px;
  font-size: 10px; line-height: 12px; text-align: center; flex: 1 1 0; min-width: 0;
}}
.people .pn {{
  display: block; width: 100%; white-space: nowrap; overflow: hidden;
  text-overflow: ellipsis;
}}
.people .av {{
  width: 40px; height: 40px; border-radius: 20px;
  display: flex; align-items: center; justify-content: center; font-weight: 500;
}}
.photo-stub {{
  background-size: cover; background-position: center; overflow: hidden;
  background-repeat: no-repeat;
}}
.mail-avatar.photo-stub, .people .av.photo-stub {{
  font-size: 0; color: transparent; background-repeat: no-repeat;
}}
.lists-scene {{ padding: 8px 16px 16px; }}
.lists-scene .lists-title {{ font-size: 16px; font-weight: 500; margin: 12px 0 8px; }}
.lists-row {{
  display: flex; align-items: center; gap: 12px; min-height: 56px;
}}
.lists-row .lists-meta {{ display: flex; flex-direction: column; flex: 1; }}
.lists-row .lists-sub {{ font-size: 12px; opacity: 0.7; }}
.fab-scene, .product-card, .chat-scene {{
  position: relative; width: 360px; max-width: 100%;
  border: 12px solid {on_surface}; border-radius: 36px; overflow: hidden;
  background: {surface};
}}
.fab-scene .photo-hero {{ min-height: 360px; height: 360px; }}
.fab-scene .fab-menu {{ position: absolute; right: 16px; bottom: 16px; }}
.product-card .photo-hero {{ height: 200px; width: 100%; }}
.product-card .copy {{ padding: 8px 20px 16px; background: {surface}; }}
.product-card .copy .h {{ font-size: 22px; line-height: 28px; font-weight: 500; }}
.product-card .copy .s {{ font-size: 14px; line-height: 20px; opacity: 0.75; margin: 4px 0 16px; }}
.chat-scene .chat-head {{
  display: flex; align-items: center; gap: 12px; padding: 12px 16px;
}}
.chat-scene .chat-meta {{ flex: 1; display: flex; flex-direction: column; }}
.chat-scene .bubble {{
  margin: 8px 16px; padding: 12px 16px; border-radius: 16px; max-width: 80%;
  background: {surface_low}; font-size: 14px; line-height: 20px;
}}
.chat-scene .dog {{ margin: 8px 16px; height: 140px; border-radius: 16px; }}
.chat-scene .toolbar-fab {{ position: absolute; left: 16px; right: 16px; bottom: 16px; justify-content: space-between; }}
.sheet-actions {{
  display: flex; justify-content: space-around; width: 100%; padding: 4px 4px 12px;
}}
.sheet-action {{
  display: flex; flex-direction: column; align-items: center; gap: 4px;
  font-size: 11px; line-height: 14px; width: 64px; text-align: center;
}}
.share-hero {{ flex: 1; min-height: 180px; }}
.album-bar {{
  display: flex; justify-content: space-around; padding: 8px 12px; font-size: 12px; font-weight: 500;
}}
.meet-dot {{
  width: 6px; height: 6px; border-radius: 3px; background: #B3261E; position: absolute; top: 4px; right: 18px;
}}
.inbox-nav .dest {{ position: relative; }}
.fab-menu {{
  display: inline-flex; flex-direction: column; align-items: flex-end; gap: 4px;
}}
.fab-menu .fab-item {{
  display: inline-flex; align-items: center; height: 56px; min-width: 56px;
  padding: 0 24px; border-radius: 28px; gap: 8px; font-weight: 500;
}}
.fab-menu[data-expanded="0"] .fab-item {{
  width: 0; min-width: 0; padding: 0; opacity: 0; overflow: hidden;
  pointer-events: none;
}}
.split {{
  display: inline-flex; align-items: stretch; gap: 2px; position: relative;
}}
.split .split-lead, .split .split-trail {{
  display: inline-flex; align-items: center; justify-content: center;
  border: none; font-family: Roboto, sans-serif; cursor: pointer;
}}
.split-menu {{
  position: absolute; top: calc(100% + 4px); right: 0; z-index: 3;
  display: flex; flex-direction: row; align-items: flex-end; gap: 4px;
  min-width: 200px;
}}
.split-menu[data-flyout="0"] .menu-flyout {{ display: none; }}
.split[data-open="0"] .split-menu {{ display: none; }}
.toolbar {{
  display: inline-flex; align-items: center; height: 64px; padding: 0 8px; gap: 4px;
}}
.toolbar[data-axis="vertical"] {{
  flex-direction: column; width: 64px; height: auto; padding: 8px 0;
}}
.toolbar-fab {{
  display: inline-flex; align-items: center; gap: 8px;
}}
.share-stage .sheet {{
  position: relative; left: auto; right: auto; bottom: auto; max-width: none;
}}
.carousel[data-carousel-axis="vertical"] {{
  flex-direction: column; max-height: 420px;
}}
.carousel[data-carousel-centered="1"] {{ justify-content: center; }}
.carousel[data-carousel-layout="uncontained-multi"] {{ align-items: flex-end; }}
.phone-frame {{
  width: 360px; max-width: 100%; border: 12px solid {on_surface};
  border-radius: 36px; overflow: hidden; background: {surface};
}}
.phone-frame .carousel {{ max-width: none; }}
.badge-wrap {{ position: relative; display: inline-flex; width: 40px; height: 40px; align-items: center; justify-content: center; }}
.badge {{
  position: absolute; top: 2px; right: 2px;
  border-radius: 8px; min-width: 16px; height: 16px; padding: 0 4px;
  display: flex; align-items: center; justify-content: center;
  font-size: 11px; font-weight: 500;
}}
.badge.small {{ width: 6px; height: 6px; min-width: 6px; padding: 0; border-radius: 3px; top: 6px; right: 6px; }}
.cal {{ width: 360px; padding: 16px 12px 12px; }}
.cal .head {{ padding: 8px 12px 16px; }}
.cal .week, .cal .grid {{ display: grid; grid-template-columns: repeat(7, 40px); justify-content: center; }}
.day {{
  width: 40px; height: 40px; border-radius: 20px;
  display: flex; align-items: center; justify-content: center;
}}
.motion-box {{
  width: 48px; height: 48px; border-radius: 12px;
  animation: m3slide 1200ms {ease} infinite alternate;
}}
@keyframes m3slide {{
  from {{ transform: translateX(0); }}
  to {{ transform: translateX(160px); }}
}}
</style>
</head>
<body>
{body}
<script>
function focusTypeahead(menu) {{
  if (!menu) return;
  menu.setAttribute("tabindex", "0");
  menu.setAttribute("data-typeahead-autofocus", "1");
  try {{ menu.focus({{ preventScroll: true }}); }} catch (e) {{ menu.focus(); }}
}}
document.querySelectorAll(".chip[data-chip-morph='1']").forEach(function (chip) {{
  chip.addEventListener("click", function () {{
    var on = chip.getAttribute("data-selected") === "1";
    var next = !on;
    chip.setAttribute("data-selected", next ? "1" : "0");
    var r = chip.getAttribute(next ? "data-chip-sel-r" : "data-chip-rest-r");
    if (r) chip.style.borderRadius = r + "px";
    var bg = chip.getAttribute(next ? "data-sel-bg" : "data-idle-bg");
    var fg = chip.getAttribute(next ? "data-sel-fg" : "data-idle-fg");
    var bd = chip.getAttribute(next ? "data-sel-bd" : "data-idle-bd");
    if (bg) chip.style.background = bg;
    if (fg) chip.style.color = fg;
    if (bd) chip.style.border = bd;
    chip.setAttribute("data-chip-press", "1");
  }});
}});
document.querySelectorAll("[data-editor] input").forEach(function (input) {{
  function sync() {{
    var wrap = input.closest("[data-editor]");
    if (!wrap) return;
    var lab = wrap.querySelector(".lab");
    if (lab) lab.style.fontSize = (input.value || document.activeElement === input) ? "12px" : "16px";
    var box = wrap.querySelector("[data-field='outlined-edit']");
    if (box && box.tagName !== "FIELDSET" && box.getAttribute("data-label-position") !== "inside" && (input.value || document.activeElement === input)) {{
      var fs = document.createElement("fieldset");
      fs.className = box.className;
      fs.setAttribute("data-field", "outlined-edit");
      fs.setAttribute("data-notched", "1");
      fs.setAttribute("data-notch", "cutout");
      fs.setAttribute("data-notch-evenodd", "1");
      fs.setAttribute("data-notch-cpath", "1");
      fs.setAttribute("style", "border:none;position:relative;" + (box.getAttribute("style") || ""));
      var evenD = box.getAttribute("data-evenodd-d") || "";
      if (evenD) {{
        var svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("class", "ol-evenodd");
        svg.setAttribute("viewBox", "0 0 280 56");
        svg.setAttribute("preserveAspectRatio", "none");
        var path = document.createElementNS("http://www.w3.org/2000/svg", "path");
        path.setAttribute("fill-rule", "evenodd");
        path.setAttribute("data-notch-evenodd-path", "1");
        path.setAttribute("d", evenD);
        var oc = (box.getAttribute("style") || "").match(/border:[^;]*solid\\s+([^;]+)/);
        path.setAttribute("fill", oc ? oc[1].trim() : "currentColor");
        svg.appendChild(path);
        fs.appendChild(svg);
      }}
      var legend = document.createElement("legend");
      legend.textContent = (box.querySelector(".lab") || {{textContent: "Email"}}).textContent || "Email";
      legend.style.padding = "0 4px";
      while (box.firstChild) fs.appendChild(box.firstChild);
      var innerLab = fs.querySelector(".lab");
      if (innerLab) innerLab.remove();
      fs.insertBefore(legend, fs.firstChild);
      box.replaceWith(fs);
    }}
  }}
  input.addEventListener("input", sync);
  input.addEventListener("focus", sync);
  input.addEventListener("blur", sync);
}});
document.querySelectorAll("[data-button-group='standard']").forEach(function (group) {{
  var btns = group.querySelectorAll("[data-standard-i]");
  function apply(sel) {{
    var n = btns.length;
    var base = Number(group.getAttribute("data-base-w") || "88");
    var ratio = Number(group.getAttribute("data-expanded-ratio") || "0.15");
    var extra = base * ratio;
    var neighbors = [];
    if (sel > 0) neighbors.push(sel - 1);
    if (sel + 1 < n) neighbors.push(sel + 1);
    var share = neighbors.length ? extra / neighbors.length : 0;
    btns.forEach(function (btn, i) {{
      var w = i === sel ? base + extra : (neighbors.indexOf(i) >= 0 ? base - share : base);
      btn.style.width = w + "px";
      var on = i === sel;
      btn.classList.toggle("selected", on);
      btn.style.background = btn.getAttribute(on ? "data-sel-bg" : "data-idle-bg") || btn.style.background;
      btn.style.color = btn.getAttribute(on ? "data-sel-fg" : "data-idle-fg") || btn.style.color;
      btn.style.borderRadius = btn.getAttribute(on ? "data-sel-r" : "data-idle-r") || btn.style.borderRadius;
      btn.style.fontWeight = on ? "700" : "500";
    }});
    group.setAttribute("data-selected", String(sel));
  }}
  btns.forEach(function (btn) {{
    btn.addEventListener("click", function () {{
      apply(Number(btn.getAttribute("data-standard-i") || "0"));
    }});
  }});
  var overflowBtn = group.querySelector("[data-standard-overflow-btn]");
  if (overflowBtn) {{
    overflowBtn.addEventListener("click", function () {{
      var wrap = group.parentElement;
      var menu = wrap && wrap.querySelector("[data-standard-overflow-menu]");
      if (menu) {{
        var open = menu.getAttribute("data-open") === "1";
        menu.setAttribute("data-open", open ? "0" : "1");
        menu.style.display = open ? "none" : "flex";
        if (!open) focusTypeahead(menu);
      }}
    }});
  }}
}});
document.querySelectorAll("[data-button-group]").forEach(function (group) {{
  group.querySelectorAll(".btn-connected").forEach(function (btn) {{
    btn.addEventListener("click", function () {{
      if (btn.getAttribute("data-overflow") === "1") {{
        var menu = group.parentElement && group.parentElement.querySelector("[data-overflow-menu]");
        if (menu) {{
          var open = menu.getAttribute("data-open") === "1";
          menu.setAttribute("data-open", open ? "0" : "1");
          menu.style.display = open ? "none" : "flex";
          if (!open) focusTypeahead(menu);
        }}
        return;
      }}
      group.querySelectorAll(".btn-connected").forEach(function (other) {{
        if (other.getAttribute("data-overflow") === "1") return;
        other.classList.remove("selected");
        other.style.background = other.getAttribute("data-idle-bg") || other.style.background;
        other.style.color = other.getAttribute("data-idle-fg") || other.style.color;
        other.style.borderRadius = other.getAttribute("data-idle-r") || other.style.borderRadius;
        other.style.border = other.getAttribute("data-idle-bd") || other.style.border;
        other.style.fontWeight = "500";
      }});
      btn.classList.add("selected");
      btn.style.background = btn.getAttribute("data-sel-bg") || btn.style.background;
      btn.style.color = btn.getAttribute("data-sel-fg") || btn.style.color;
      btn.style.borderRadius = btn.getAttribute("data-sel-r") || btn.style.borderRadius;
      btn.style.border = "none";
      btn.style.fontWeight = "700";
    }});
  }});
}});
document.querySelectorAll("[data-list-swipe]").forEach(function (wrap) {{
  var sheet = wrap.querySelector(".sheet");
  var lead = wrap.querySelector(".rail.lead");
  var trail = wrap.querySelector(".rail.trail");
  var reveal = Number(wrap.getAttribute("data-swipe-reveal") || "80");
  var thresh = Number(wrap.getAttribute("data-swipe-threshold") || "56");
  var primary = Number(wrap.getAttribute("data-swipe-primary-dp") || "360");
  var overshoot = Number(wrap.getAttribute("data-swipe-overshoot") || "16");
  var pthresh = Number(wrap.getAttribute("data-swipe-primary-threshold") || "180");
  var decay = Number(wrap.getAttribute("data-swipe-fling-decay") || "2");
  var rest = Number(wrap.getAttribute("data-swipe-fling-rest") || "24");
  var max = primary + overshoot;
  var x0 = 0, cur = Number(wrap.getAttribute("data-swipe-offset") || "0");
  var vel = 0, dragging = false, raf = 0, last = 0, lastMove = 0;
  function snapTarget() {{
    if (Math.abs(cur) >= pthresh) return cur > 0 ? primary : -primary;
    if (Math.abs(cur) >= thresh) return cur > 0 ? reveal : -reveal;
    return 0;
  }}
  function snapState(t) {{
    if (Math.abs(t) >= primary) return "primary";
    if (Math.abs(t) >= thresh) return "open";
    return "closed";
  }}
  function growRails() {{
    if (lead) {{
      var lw = cur > reveal ? Math.min(primary, cur) : reveal;
      lead.style.width = lw + "px";
      lead.style.flexBasis = lw + "px";
    }}
    if (trail) {{
      var tw = cur < -reveal ? Math.min(primary, -cur) : reveal;
      trail.style.width = tw + "px";
      trail.style.flexBasis = tw + "px";
    }}
  }}
  function setOff(x, state) {{
    cur = Math.max(-max, Math.min(max, x));
    if (sheet) sheet.style.transform = "translateX(" + cur + "px)";
    wrap.setAttribute("data-swipe-offset", String(cur));
    wrap.setAttribute("data-swipe-leading", cur >= thresh ? "1" : "0");
    wrap.setAttribute("data-swipe-trailing", cur <= -thresh ? "1" : "0");
    wrap.setAttribute("data-swipe-dismissed", Math.abs(cur) >= primary ? "1" : "0");
    growRails();
    if (state) wrap.setAttribute("data-swipe-state", state);
  }}
  function loop(now) {{
    if (last) {{
      var dt = Math.min(0.05, (now - last) / 1000);
      cur += vel * dt;
      vel *= Math.exp(-decay * dt);
      if (Math.abs(cur) >= primary) {{
        vel = 0;
        setOff(cur > 0 ? primary : -primary, "primary");
        raf = 0; last = 0;
        return;
      }}
      if (Math.abs(vel) < rest) {{
        vel = 0;
        var t = snapTarget();
        setOff(t, snapState(t));
        raf = 0; last = 0;
        return;
      }}
      setOff(cur, "settling");
    }}
    last = now;
    raf = requestAnimationFrame(loop);
  }}
  function startFling() {{
    if (!raf) raf = requestAnimationFrame(loop);
  }}
  wrap.addEventListener("pointerdown", function (ev) {{
    dragging = true; vel = 0; x0 = ev.clientX; lastMove = ev.timeStamp;
    if (raf) {{ cancelAnimationFrame(raf); raf = 0; last = 0; }}
    wrap.setPointerCapture(ev.pointerId);
    wrap.setAttribute("data-swipe-state", "dragging");
  }});
  wrap.addEventListener("pointermove", function (ev) {{
    if (!dragging) return;
    var dx = ev.clientX - x0;
    var dt = Math.max(0.001, (ev.timeStamp - lastMove) / 1000);
    vel = dx / dt;
    lastMove = ev.timeStamp;
    x0 = ev.clientX;
    setOff(cur + dx, "dragging");
  }});
  function end() {{
    if (!dragging) return;
    dragging = false;
    wrap.setAttribute("data-swipe-state", "settling");
    startFling();
  }}
  wrap.addEventListener("pointerup", end);
  wrap.addEventListener("pointercancel", end);
  wrap.addEventListener("wheel", function (ev) {{
    var dx = ev.deltaX;
    if (Math.abs(dx) < 0.5) return;
    ev.preventDefault();
    vel += dx;
    wrap.setAttribute("data-swipe-state", "settling");
    startFling();
  }}, {{ passive: false }});
  setOff(cur);
}});
document.querySelectorAll("[data-list-reorder]").forEach(function (group) {{
  group.querySelectorAll("[data-list-handle]").forEach(function (handle) {{
    handle.addEventListener("click", function (ev) {{
      ev.stopPropagation();
      var item = handle.closest("[data-list-item]");
      if (!item || !item.previousElementSibling) return;
      group.insertBefore(item, item.previousElementSibling);
    }});
  }});
}});
document.querySelectorAll("[data-tooltip-trigger='hover']").forEach(function (anchor) {{
  var hold = Number(anchor.getAttribute("data-tooltip-longpress-ms") || "500");
  var timer = null;
  function open() {{ anchor.setAttribute("data-open", "1"); }}
  function close() {{ anchor.setAttribute("data-open", "0"); }}
  anchor.addEventListener("mouseenter", open);
  anchor.addEventListener("mouseleave", close);
  anchor.addEventListener("focusin", open);
  anchor.addEventListener("focusout", close);
  anchor.addEventListener("pointerdown", function () {{
    timer = setTimeout(open, hold);
  }});
  ["pointerup", "pointerleave", "pointercancel"].forEach(function (ev) {{
    anchor.addEventListener(ev, function () {{ if (timer) {{ clearTimeout(timer); timer = null; }} }});
  }});
}});
document.querySelectorAll("[data-list-style='segmented']").forEach(function (group) {{
  group.querySelectorAll("[data-list-item]").forEach(function (item) {{
    item.addEventListener("click", function () {{
      group.querySelectorAll("[data-list-item]").forEach(function (other) {{
        other.setAttribute("data-list-selected", "0");
        other.style.background = other.getAttribute("data-idle-bg") || other.style.background;
        other.style.color = other.getAttribute("data-idle-fg") || other.style.color;
        other.style.borderRadius = other.getAttribute("data-idle-r") || other.style.borderRadius;
        var sub = other.querySelector(".s");
        if (sub) sub.style.color = other.getAttribute("data-idle-fg") || sub.style.color;
      }});
      item.setAttribute("data-list-selected", "1");
      item.style.background = item.getAttribute("data-on-bg") || item.style.background;
      item.style.color = item.getAttribute("data-on-fg") || item.style.color;
      item.style.borderRadius = item.getAttribute("data-on-r") || item.style.borderRadius;
      var onSub = item.querySelector(".s");
      if (onSub) onSub.style.color = item.getAttribute("data-on-fg") || onSub.style.color;
    }});
  }});
}});
document.querySelectorAll("[data-appbar-scene]").forEach(function (scene) {{
  var bar = scene.querySelector("[data-appbar]");
  if (!bar) return;
  scene.addEventListener("click", function () {{
    var cur = Number(bar.getAttribute("data-collapse") || "0");
    var next = cur < 0.25 ? 0.5 : cur < 0.75 ? 1 : 0;
    bar.setAttribute("data-collapse", String(next));
    var exp = Number(bar.getAttribute("data-expanded-h") || "152");
    var col = Number(bar.getAttribute("data-collapsed-h") || "64");
    var h = exp + (col - exp) * next;
    bar.style.height = h + "px";
    var fill = next > 0.001;
    bar.style.background = fill
      ? (bar.getAttribute("data-scrolled-bg") || bar.style.background)
      : (bar.getAttribute("data-rest-bg") || bar.style.background);
    bar.style.boxShadow = fill
      ? (bar.getAttribute("data-scrolled-sh") || "none")
      : "none";
    var title = bar.querySelector("[data-appbar-title]");
    var sub = bar.querySelector("[data-appbar-sub]");
    var t0 = Number(bar.getAttribute("data-title-exp") || "36");
    var t1 = Number(bar.getAttribute("data-title-col") || "22");
    if (title) title.style.fontSize = (t0 + (t1 - t0) * next) + "px";
    if (sub) sub.style.opacity = String(1 - next);
  }});
}});
document.querySelectorAll("[data-media-scene]").forEach(function (scene) {{
  scene.querySelectorAll("[data-media-tab]").forEach(function (tab) {{
    tab.addEventListener("click", function () {{
      scene.querySelectorAll("[data-media-tab]").forEach(function (other) {{
        other.setAttribute("data-active", "false");
        other.style.color = other.getAttribute("data-idle-fg") || other.style.color;
        var ind = other.querySelector(".tab-ind");
        if (ind) ind.style.display = "none";
      }});
      tab.setAttribute("data-active", "true");
      tab.style.color = tab.getAttribute("data-sel-fg") || tab.style.color;
      var on = tab.querySelector(".tab-ind");
      if (on) on.style.display = "block";
    }});
  }});
}});
document.querySelectorAll("[data-snackbar]").forEach(function (bar) {{
  var remain = Number(bar.getAttribute("data-timeout-ms") || "4000");
  var persist = bar.getAttribute("data-persist") === "1";
  var start = performance.now();
  var ox = 0;
  function paint() {{
    bar.style.transform = "translateX(" + ox + "px)";
    bar.style.opacity = String(Math.max(0, 1 - Math.abs(ox) / 72));
  }}
  function hide() {{
    bar.setAttribute("data-dismissed", "1");
    bar.style.display = "none";
  }}
  function tick(now) {{
    if (bar.getAttribute("data-dismissed") === "1") return;
    if (now - start >= remain) {{ hide(); return; }}
    requestAnimationFrame(tick);
  }}
  // Catalog snapshots keep the in-phone snack visible (official overview always shows it).
  if (!persist) requestAnimationFrame(tick);
  var sx = null;
  bar.addEventListener("pointerdown", function (ev) {{ sx = ev.clientX; }});
  bar.addEventListener("pointermove", function (ev) {{
    if (sx == null) return;
    ox = ev.clientX - sx;
    paint();
  }});
  bar.addEventListener("pointerup", function () {{
    if (Math.abs(ox) >= 72) hide();
    else {{ ox = 0; paint(); }}
    sx = null;
  }});
  var closeBtn = bar.querySelector("[data-snackbar-close]");
  if (closeBtn) {{
    closeBtn.addEventListener("click", function (ev) {{
      ev.stopPropagation();
      hide();
    }});
  }}
}});
document.querySelectorAll("[data-slider-range]").forEach(function (row) {{
  var slider = row.querySelector(".xslider");
  if (!slider) return;
  row.tabIndex = 0;
  row.setAttribute("data-range-focus", "start");
  var dragging = null;
  function clampRange(start, end) {{
    start = Math.max(0, Math.min(1 - 0.05, start));
    end = Math.max(start + 0.05, Math.min(1, end));
    return [start, end];
  }}
  function fracFromEvent(ev) {{
    var r = slider.getBoundingClientRect();
    if (!r.width) return 0;
    return Math.max(0, Math.min(1, (ev.clientX - r.left) / r.width));
  }}
  function paint(start, end) {{
    row.setAttribute("data-start", start.toFixed(2));
    row.setAttribute("data-end", end.toFixed(2));
    var segs = slider.querySelectorAll(".xseg");
    if (segs[0]) segs[0].style.width = (Math.max(6, start * 42)).toFixed(1) + "%";
    if (segs[1]) segs[1].style.width = (Math.max(8, (end - start) * 42)).toFixed(1) + "%";
    var lab = row.querySelector(".slider-label");
    if (lab) lab.textContent = "Price range · " + Math.round(start * 100) + "–" + Math.round(end * 100) + "% · min span 5%";
  }}
  function clickStep(start, end, fraction) {{
    var snapped = Math.round(fraction / 0.05) * 0.05;
    if (Math.abs(snapped - start) <= Math.abs(snapped - end)) return clampRange(snapped, end);
    return clampRange(start, snapped);
  }}
  slider.addEventListener("pointerdown", function (ev) {{
    var start = parseFloat(row.getAttribute("data-start") || "0.2");
    var end = parseFloat(row.getAttribute("data-end") || "0.75");
    var f = fracFromEvent(ev);
    dragging = Math.abs(f - start) <= Math.abs(f - end) ? "start" : "end";
    row.setAttribute("data-range-focus", dragging);
    try {{ slider.setPointerCapture(ev.pointerId); }} catch (e) {{}}
    ev.preventDefault();
  }});
  slider.addEventListener("pointermove", function (ev) {{
    if (!dragging) return;
    var start = parseFloat(row.getAttribute("data-start") || "0.2");
    var end = parseFloat(row.getAttribute("data-end") || "0.75");
    var f = fracFromEvent(ev);
    if (true) {{ f = Math.round(f / 0.05) * 0.05; }}
    var next = dragging === "start" ? clampRange(f, end) : clampRange(start, f);
    paint(next[0], next[1]);
  }});
  slider.addEventListener("pointerup", function () {{ dragging = null; }});
  slider.addEventListener("click", function (ev) {{
    if (dragging) return;
    var start = parseFloat(row.getAttribute("data-start") || "0.2");
    var end = parseFloat(row.getAttribute("data-end") || "0.75");
    var next = clickStep(start, end, fracFromEvent(ev));
    paint(next[0], next[1]);
  }});
  row.addEventListener("keydown", function (ev) {{
    var start = parseFloat(row.getAttribute("data-start") || "0.2");
    var end = parseFloat(row.getAttribute("data-end") || "0.75");
    var focus = row.getAttribute("data-range-focus") || "start";
    var delta = 0;
    if (ev.key === "ArrowLeft" || ev.key === "h") delta = -0.05;
    if (ev.key === "ArrowRight" || ev.key === "l") delta = 0.05;
    if (!delta) return;
    var next = focus === "end" ? clampRange(start, end + delta) : clampRange(start + delta, end);
    paint(next[0], next[1]);
    ev.preventDefault();
  }});
}});
document.querySelectorAll("[data-timepicker]").forEach(function (picker) {{
  function setDial(face) {{
    picker.setAttribute("data-dial", face);
    picker.querySelectorAll("[data-hour]").forEach(function (el) {{
      el.style.display = face === "hour" ? "flex" : "none";
    }});
    picker.querySelectorAll("[data-minute]").forEach(function (el) {{
      el.style.display = face === "minute" ? "flex" : "none";
    }});
    var hour = parseInt(picker.getAttribute("data-hour") || "6", 10);
    var minute = parseInt(picker.getAttribute("data-minute") || "30", 10);
    var deg = face === "minute" ? minute * 6 : hour * 30 + minute * 0.5;
    var hand = picker.querySelector(".hand-svg");
    if (hand) {{
      hand.style.setProperty("--hand-base", deg + "deg");
      hand.style.transform = "rotate(" + deg + "deg)";
      if (face === "hour") {{
        hand.setAttribute("data-hour-live", "1");
      }} else {{
        hand.removeAttribute("data-hour-live");
        hand.style.animation = "none";
      }}
    }}
    picker.querySelectorAll("[data-time-field]").forEach(function (el) {{
      el.setAttribute("data-active", el.getAttribute("data-time-field") === face ? "1" : "0");
    }});
  }}
  picker.querySelectorAll("[data-hour]").forEach(function (el) {{
    el.style.cursor = "pointer";
    el.addEventListener("click", function () {{
      picker.setAttribute("data-hour", el.getAttribute("data-hour"));
      picker.querySelectorAll("[data-hour]").forEach(function (other) {{
        other.setAttribute("data-selected", other === el ? "1" : "0");
      }});
      setDial("minute");
    }});
  }});
  picker.querySelectorAll("[data-minute]").forEach(function (el) {{
    el.style.cursor = "pointer";
    el.addEventListener("click", function () {{
      picker.setAttribute("data-minute", el.getAttribute("data-minute"));
      picker.querySelectorAll("[data-minute]").forEach(function (other) {{
        other.setAttribute("data-selected", other === el ? "1" : "0");
      }});
      setDial("minute");
    }});
  }});
  picker.querySelectorAll("[data-time-field]").forEach(function (el) {{
    el.addEventListener("click", function () {{
      setDial(el.getAttribute("data-time-field"));
    }});
  }});
  picker.querySelectorAll("[data-period]").forEach(function (btn) {{
    btn.addEventListener("click", function () {{
      picker.setAttribute("data-period", btn.getAttribute("data-period"));
    }});
  }});
  setDial(picker.getAttribute("data-dial") || "minute");
  var secondEl = picker.querySelector("[data-second-hand]");
  if (secondEl) {{
    secondEl.setAttribute("data-second-wall", "1");
    function tickSecond() {{
      var now = new Date();
      var deg = (now.getSeconds() + now.getMilliseconds() / 1000) * 6;
      secondEl.style.transform = "rotate(" + deg + "deg)";
      requestAnimationFrame(tickSecond);
    }}
    tickSecond();
  }}
}});
document.querySelectorAll("[data-time-scroll]").forEach(function (hero) {{
  var itemH = Number(hero.getAttribute("data-scroll-item-h") || "66.666");
  var decay = Number(hero.getAttribute("data-scroll-fling-decay") || "2");
  var rest = Number(hero.getAttribute("data-scroll-fling-rest") || "0.35");
  var stiff = Number(hero.getAttribute("data-scroll-snap") || "14");
  function wrap(off, count) {{
    var o = off % count;
    return o < 0 ? o + count : o;
  }}
  function shortest(from, to, count) {{
    var d = to - from;
    if (d > count / 2) d -= count;
    else if (d < -count / 2) d += count;
    return d;
  }}
  function hourValue(idx, count) {{
    return count === 24 ? (idx % 24) : (idx % 12) + 1;
  }}
  function paintField(field) {{
    var count = Number(field.getAttribute("data-count") || "12");
    var off = Number(field.getAttribute("data-offset") || "0");
    var kind = field.getAttribute("data-scroll-field");
    var center = (Number(field.getAttribute("data-field-h") || "200") - itemH) / 2;
    var base = Math.floor(off);
    field.querySelectorAll(".scroll-item").forEach(function (el) {{
      var rel = Number(el.getAttribute("data-rel") || "0");
      var logical = base + rel;
      var idx = ((logical % count) + count) % count;
      var value = kind === "hour" ? hourValue(idx, count) : idx % 60;
      var y = (logical - off) * itemH + center;
      var dist = Math.abs(logical - off);
      var selected = dist < 0.5;
      var opacity = Math.max(0.28, Math.min(1, 1 - dist * 0.42));
      var label = String(value).padStart(2, "0");
      el.style.top = y + "px";
      el.style.opacity = String(opacity);
      el.style.fontWeight = selected ? "500" : "400";
      el.style.fontSize = selected ? "57px" : "45px";
      el.setAttribute("data-index", String(idx));
      el.setAttribute("data-value", String(value));
      el.setAttribute("data-selected", selected ? "1" : "0");
      el.textContent = label;
    }});
    field.setAttribute("data-offset", String(off));
    var selectedIdx = Math.round(off);
    selectedIdx = ((selectedIdx % count) + count) % count;
    var selectedVal = kind === "hour" ? hourValue(selectedIdx, count) : selectedIdx % 60;
    hero.setAttribute(kind === "hour" ? "data-hour" : "data-minute", String(selectedVal));
  }}
  hero.querySelectorAll("[data-scroll-field]").forEach(function (field) {{
    var vel = 0, dragging = false, lastY = 0, lastT = 0, raf = 0, last = 0;
    function currentCount() {{
      return Number(field.getAttribute("data-count") || "12");
    }}
    function setOff(next) {{
      field.setAttribute("data-offset", String(wrap(next, currentCount())));
      paintField(field);
    }}
    function loop(now) {{
      var count = currentCount();
      if (last) {{
        var dt = Math.min(0.05, (now - last) / 1000);
        var off = Number(field.getAttribute("data-offset") || "0");
        if (Math.abs(vel) >= rest) {{
          off = wrap(off + vel * dt, count);
          vel *= Math.exp(-decay * dt);
          if (Math.abs(vel) < rest) vel = 0;
        }} else {{
          vel = 0;
          var target = Math.round(off);
          var d = shortest(off, target, count);
          if (Math.abs(d) < 0.002) off = wrap(target, count);
          else off = wrap(off + d * (1 - Math.exp(-stiff * dt)), count);
        }}
        setOff(off);
      }}
      last = now;
      var still = Math.abs(vel) >= rest;
      var offNow = Number(field.getAttribute("data-offset") || "0");
      var dNow = shortest(offNow, Math.round(offNow), count);
      if (still || Math.abs(dNow) >= 0.002) raf = requestAnimationFrame(loop);
      else raf = 0;
    }}
    function kick() {{
      if (!raf) {{
        last = 0;
        raf = requestAnimationFrame(loop);
      }}
    }}
    field.addEventListener("wheel", function (ev) {{
      ev.preventDefault();
      vel += ev.deltaY / itemH * 8;
      setOff(Number(field.getAttribute("data-offset") || "0") + ev.deltaY * 0.15 / itemH);
      kick();
    }}, {{ passive: false }});
    field.addEventListener("pointerdown", function (ev) {{
      dragging = true;
      lastY = ev.clientY;
      lastT = performance.now();
      vel = 0;
      field.setPointerCapture(ev.pointerId);
    }});
    field.addEventListener("pointermove", function (ev) {{
      if (!dragging) return;
      var now = performance.now();
      var dy = ev.clientY - lastY;
      var dt = Math.max(0.008, (now - lastT) / 1000);
      vel = -dy / itemH / dt * 0.35;
      setOff(Number(field.getAttribute("data-offset") || "0") - dy / itemH);
      lastY = ev.clientY;
      lastT = now;
    }});
    function endDrag() {{
      if (!dragging) return;
      dragging = false;
      kick();
    }}
    field.addEventListener("pointerup", endDrag);
    field.addEventListener("pointercancel", endDrag);
    field.querySelectorAll(".scroll-item").forEach(function (el) {{
      el.addEventListener("click", function () {{
        if (Math.abs(vel) > 1) return;
        var idx = Number(el.getAttribute("data-index") || "0");
        var off = Number(field.getAttribute("data-offset") || "0");
        setOff(off + shortest(off, idx, currentCount()));
        vel = 0;
        kick();
      }});
    }});
    field.addEventListener("repaint-scroll", function () {{
      paintField(field);
    }});
    paintField(field);
  }});
  hero.querySelectorAll("[data-period]").forEach(function (btn) {{
    btn.addEventListener("click", function () {{
      hero.setAttribute("data-period", btn.getAttribute("data-period"));
    }});
  }});
}});
document.querySelectorAll("[data-scroll-display-mode-toggle]").forEach(function (btn) {{
  btn.addEventListener("click", function (ev) {{
    ev.stopPropagation();
    var host = btn.closest("[data-time-display]");
    if (!host) return;
    var mode = host.getAttribute("data-time-display") === "scroll" ? "input" : "scroll";
    host.setAttribute("data-time-display", mode);
    btn.setAttribute("data-display-mode", mode);
    btn.textContent = mode === "scroll" ? "⌨" : "◷";
    btn.setAttribute("title", mode === "scroll" ? "Switch to input mode" : "Switch to scroll mode");
    var scroll = host.querySelector("[data-time-scroll]");
    var input = host.querySelector("[data-time-input]");
    if (scroll) scroll.style.display = mode === "scroll" ? "" : "none";
    if (input) input.style.display = mode === "input" ? "" : "none";
  }});
}});
document.querySelectorAll("[data-time-format-toggle]").forEach(function (btn) {{
  btn.addEventListener("click", function (ev) {{
    ev.stopPropagation();
    var host = btn.closest("[data-time-format]");
    if (!host) return;
    var next = host.getAttribute("data-time-format") === "24" ? "12" : "24";
    var scroll = host.querySelector("[data-time-scroll]");
    var input = host.querySelector("[data-time-input]");
    var src = scroll || input;
    var hour = Number((src && src.getAttribute("data-hour")) || "18");
    var period = (src && src.getAttribute("data-period")) || "PM";
    if (next === "24") {{
      if (period === "AM") hour = hour === 12 ? 0 : hour;
      else hour = hour === 12 ? 12 : hour + 12;
    }} else {{
      if (hour === 0) {{ hour = 12; period = "AM"; }}
      else if (hour < 12) {{ period = "AM"; }}
      else if (hour === 12) {{ period = "PM"; }}
      else {{ hour = hour - 12; period = "PM"; }}
    }}
    host.setAttribute("data-time-format", next);
    btn.setAttribute("data-time-format", next);
    btn.textContent = next === "24" ? "12" : "24";
    btn.setAttribute("title", next === "24" ? "Switch to 12-hour" : "Switch to 24-hour");
    function applyHour(el) {{
      if (!el) return;
      el.setAttribute("data-hour", String(hour));
      el.setAttribute("data-period", period);
    }}
    applyHour(scroll);
    applyHour(input);
    if (scroll) {{
      var hf = scroll.querySelector("[data-scroll-field='hour']");
      if (hf) {{
        hf.setAttribute("data-count", next === "24" ? "24" : "12");
        hf.setAttribute("data-offset", next === "24" ? String(hour) : String(Math.max(0, hour - 1)));
        hf.dispatchEvent(new Event("repaint-scroll"));
      }}
    }}
    if (input) {{
      var inH = input.querySelector("[data-time-input-field='hour']");
      if (inH) inH.value = String(hour).padStart(2, "0");
    }}
  }});
}});
document.querySelectorAll("[data-time-input-field]").forEach(function (field) {{
  field.addEventListener("input", function () {{
    var raw = (field.value || "").replace(/\\D/g, "").slice(0, 2);
    var kind = field.getAttribute("data-time-input-field");
    var n = parseInt(raw || "0", 10);
    var host = field.closest("[data-time-format]") || field.closest("[data-time-input]") || field.closest("[data-time-display]");
    var maxHour = (host && host.getAttribute("data-time-format") === "24") ? 23 : 12;
    if (kind === "hour" && n > maxHour) raw = raw.slice(0, 1);
    if (kind === "minute" && n > 59) raw = raw.slice(0, 1);
    field.value = raw;
    if (host && raw.length === 2) host.setAttribute("data-" + kind, String(n));
    var box = field.closest("[data-time-input]");
    if (box && raw.length === 2) box.setAttribute("data-" + kind, String(parseInt(raw || "0", 10)));
  }});
}});
document.querySelectorAll("[data-datepicker-docked]").forEach(function (dock) {{
  var cal = dock.querySelector("[data-datepicker-popup]");
  var field = dock.querySelector("[data-field-hero='docked-date']");
  function setOpen(open) {{
    dock.setAttribute("data-popup", open ? "open" : "closed");
    if (cal) cal.style.display = open ? "" : "none";
  }}
  if (field) field.addEventListener("click", function (ev) {{
    ev.stopPropagation();
    setOpen(dock.getAttribute("data-popup") !== "open");
  }});
  if (cal) cal.addEventListener("click", function (ev) {{ ev.stopPropagation(); }});
  function daysInMonth(y, m) {{
    return [31, ((y%4===0 && y%100!==0)||y%400===0)?29:28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][m-1];
  }}
  function weekdaySunday0(y, m, d) {{
    var t = [0,3,2,5,0,3,5,1,4,6,2,4];
    var yy = y;
    if (m < 3) yy -= 1;
    return (yy + Math.floor(yy/4) - Math.floor(yy/100) + Math.floor(yy/400) + t[m-1] + d) % 7;
  }}
  function monthGrid(y, m) {{
    var first = weekdaySunday0(y, m, 1);
    var dim = daysInMonth(y, m);
    var pm = m === 1 ? 12 : m - 1;
    var py = m === 1 ? y - 1 : y;
    var pdim = daysInMonth(py, pm);
    var cells = [];
    for (var i = 0; i < first; i++) cells.push([pdim - first + 1 + i, "OutOfMonth"]);
    for (var d = 1; d <= dim; d++) cells.push([d, "InMonth"]);
    while (cells.length < 42) cells.push([cells.length - first - dim + 1, "OutOfMonth"]);
    return cells;
  }}
  function classify(y, m, day, kind) {{
    var sy = parseInt(dock.getAttribute("data-selected-year") || "2026", 10);
    var sm = parseInt(dock.getAttribute("data-selected-month") || "9", 10);
    var sd = parseInt(dock.getAttribute("data-selected-day") || "15", 10);
    var ty = parseInt(dock.getAttribute("data-today-year") || "2026", 10);
    var tm = parseInt(dock.getAttribute("data-today-month") || "9", 10);
    var td = parseInt(dock.getAttribute("data-today-day") || "11", 10);
    if (kind !== "InMonth") return kind;
    if (y === sy && m === sm && day === sd) return "Selected";
    if (y === ty && m === tm && day === td) return "Today";
    return "InMonth";
  }}
  function paintDockedGrid(y, m) {{
    var grid = dock.querySelector("[data-docked-grid]");
    if (!grid) return;
    var selBg = dock.getAttribute("data-day-sel-bg") || "#6750A4";
    var selFg = dock.getAttribute("data-day-sel-fg") || "#fff";
    var todayBd = dock.getAttribute("data-day-today") || "#6750A4";
    var inFg = dock.getAttribute("data-day-in") || "#1C1B1F";
    var outFg = dock.getAttribute("data-day-out") || "#9a9a9a";
    var html = "";
    monthGrid(y, m).forEach(function (cell) {{
      var day = cell[0];
      var kind = classify(y, m, day, cell[1]);
      var bg = "transparent", fg = inFg, outline = "none", radius = "20px";
      if (kind === "Selected") {{ bg = selBg; fg = selFg; }}
      else if (kind === "Today") {{ outline = "1px solid " + todayBd; }}
      else if (kind === "OutOfMonth") {{ fg = outFg; }}
      html += '<div class="day" data-day="'+day+'" data-kind="'+kind+'" style="background:'+bg+';color:'+fg+';border:'+outline+';border-radius:'+radius+'">'+day+'</div>';
    }});
    grid.innerHTML = html;
    dock.setAttribute("data-year", String(y));
    dock.setAttribute("data-month", String(m));
  }}
  dock.querySelectorAll("[data-docked-month]").forEach(function (btn) {{
    btn.addEventListener("click", function (ev) {{
      ev.stopPropagation();
      var label = dock.querySelector("[data-docked-month-label]");
      if (!label) return;
      var delta = parseInt(btn.getAttribute("data-docked-month") || "0", 10);
      var months = ["January","February","March","April","May","June","July","August","September","October","November","December"];
      var year = parseInt(dock.getAttribute("data-year") || "2026", 10);
      var mi = parseInt(dock.getAttribute("data-month") || "9", 10) - 1;
      mi += delta;
      while (mi < 0) {{ mi += 12; year -= 1; }}
      while (mi > 11) {{ mi -= 12; year += 1; }}
      label.textContent = months[mi] + " " + year + " ▾";
      paintDockedGrid(year, mi + 1);
    }});
  }});
  document.addEventListener("click", function () {{
    if (dock.getAttribute("data-dismiss-outside") === "1") setOpen(false);
  }});
}});
document.querySelectorAll("[data-search='1']").forEach(function (bar) {{
  bar.style.cursor = "pointer";
  bar.addEventListener("click", function (ev) {{
    if (ev.target && ev.target.closest && ev.target.closest("[data-search-input]")) return;
    var view = bar.matches("[data-search-view]") ? bar : document.querySelector("[data-search-view]");
    if (!view) return;
    var open = view.getAttribute("data-open") !== "1";
    view.setAttribute("data-open", open ? "1" : "0");
    view.setAttribute("data-search-activity", open ? "1" : "0");
    view.setAttribute("data-search-morph", open ? "1" : "0");
    view.setAttribute("data-search-shared", "1");
    view.setAttribute("data-search-scale", open ? "1" : "0.94");
    view.setAttribute("data-search-path-scale", "1");
    view.setAttribute("data-search-layer-box", "1");
    view.setAttribute("data-search-anim-scale", open ? "1" : "0.94");
    view.setAttribute("data-search-transform-origin", "top center");
    if (view.classList.contains("search-morph")) {{
      var contained = view.getAttribute("data-search-style") === "contained";
      if (contained) {{
        var compact = view.getAttribute("data-width-class") === "compact"
          || view.getAttribute("data-search-expanded") === "fullscreen";
        if (compact) {{
          view.style.minHeight = open ? "320px" : "56px";
          view.style.borderRadius = open ? "0" : "28px";
          view.style.marginLeft = open ? "0" : "16px";
          view.style.marginRight = open ? "0" : "16px";
        }} else {{
          view.style.minHeight = open ? "280px" : "56px";
          view.style.borderRadius = "28px";
          view.style.marginLeft = open ? "12px" : "24px";
          view.style.marginRight = open ? "12px" : "24px";
        }}
        view.style.transform = "none";
        view.setAttribute("data-search-scale", "1");
        view.setAttribute("data-search-anim-scale", "1");
      }} else {{
        view.style.minHeight = open ? "320px" : "56px";
        view.style.borderRadius = open ? "0" : "28px";
        view.style.marginLeft = open ? "0" : "16px";
        view.style.marginRight = open ? "0" : "16px";
        view.style.transform = open ? "scale(1)" : "scale(0.94)";
      }}
    }} else {{
      view.style.display = open ? "flex" : "none";
      bar.setAttribute("data-hidden", open ? "1" : "0");
    }}
  }});
}});
document.querySelectorAll("[data-search-input]").forEach(function (input) {{
  input.addEventListener("input", function () {{
    var view = input.closest("[data-search-view]");
    if (!view) return;
    var q = (input.value || "").trim().toLowerCase();
    view.querySelectorAll("[data-search-suggestion]").forEach(function (row) {{
      var label = (row.getAttribute("data-search-suggestion") || "").toLowerCase();
      row.style.display = !q || label.indexOf(q) >= 0 ? "flex" : "none";
    }});
  }});
  input.addEventListener("click", function (ev) {{ ev.stopPropagation(); }});
}});
document.querySelectorAll("[data-search-suggestion]").forEach(function (row) {{
  row.style.cursor = "pointer";
  row.addEventListener("click", function () {{
    var view = row.closest("[data-search-view]");
    if (!view) return;
    var input = view.querySelector("[data-search-input]");
    if (input) {{
      input.value = row.getAttribute("data-search-suggestion") || "";
      input.dispatchEvent(new Event("input"));
    }}
  }});
}});
document.querySelectorAll("[data-carousel]").forEach(function (car) {{
  car.setAttribute("data-carousel-fling", "1");
  car.setAttribute("data-carousel-live", "1");
  car.setAttribute("data-carousel-snap", "1");
  var fling = {{ selected: 0, velocity: 0, leftover: 0, raf: 0, last: 0 }};
  function applyWidths(sel, offsetT) {{
    var n = car.querySelectorAll("[data-carousel-item]").length;
    var next = offsetT >= 0 ? sel + 1 : sel - 1;
    next = ((next % n) + n) % n;
    var at = Math.min(1, Math.abs(offsetT));
    var layout = car.getAttribute("data-carousel-layout") || "hero";
    var large = layout === "multi-browse" ? 186 : layout === "uncontained-multi" ? 168 : layout === "uncontained" ? 220 : layout === "centered-hero" ? 200 : layout === "full-screen" ? 336 : 256;
    var small = layout === "multi-browse" ? 56 : layout === "uncontained-multi" ? 112 : layout === "uncontained" ? 140 : layout === "centered-hero" ? 72 : layout === "full-screen" ? 336 : 120;
    car.querySelectorAll("[data-carousel-item]").forEach(function (t) {{
      var i = Number(t.getAttribute("data-carousel-item"));
      t.style.transform = "translateX(" + (offsetT * 12) + "px)";
      var w = small;
      if (i === sel) w = large + (small - large) * at;
      else if (i === next) w = small + (large - small) * at;
      t.style.width = w + "px";
    }});
  }}
  function applySel(sel) {{
    var n = car.querySelectorAll("[data-carousel-item]").length;
    sel = ((sel % n) + n) % n;
    fling.selected = sel;
    car.setAttribute("data-carousel-selected", String(sel));
    applyWidths(sel, 0);
  }}
  function stepLive(dt) {{
    fling.leftover += fling.velocity * dt;
    fling.velocity *= Math.exp(-{fling_decay} * dt);
    if (Math.abs(fling.velocity) < 0.5) fling.velocity = 0;
    while (Math.abs(fling.leftover) >= {fling_unit}) {{
      var dir = fling.leftover > 0 ? 1 : -1;
      applySel(fling.selected + dir);
      fling.leftover -= dir * {fling_unit};
    }}
    if (Math.abs(fling.velocity) < 0.5) {{
      if (Math.abs(fling.leftover) >= {fling_unit} * {fling_snap}) {{
        var dir = fling.leftover > 0 ? 1 : -1;
        applySel(fling.selected + dir);
      }}
      fling.leftover = 0;
      fling.velocity = 0;
    }}
    applyWidths(fling.selected, fling.leftover / {fling_unit});
  }}
  function loop(now) {{
    if (fling.last) {{
      var dt = Math.min(0.05, (now - fling.last) / 1000);
      stepLive(dt);
    }}
    fling.last = now;
    if (Math.abs(fling.velocity) >= 0.5 || Math.abs(fling.leftover) >= {fling_unit} * {fling_snap}) {{
      fling.raf = requestAnimationFrame(loop);
    }} else {{
      fling.raf = 0;
      fling.last = 0;
    }}
  }}
  car.addEventListener("click", function (ev) {{
    var tile = ev.target.closest("[data-carousel-item]");
    if (!tile) return;
    fling.velocity = 0;
    fling.leftover = 0;
    applySel(Number(tile.getAttribute("data-carousel-item")));
  }});
  car.addEventListener("wheel", function (ev) {{
    var dx = ev.deltaX, dy = ev.deltaY;
    var dominant = Math.abs(dx) >= Math.abs(dy) ? dx : dy;
    if (Math.abs(dominant) < 0.5) return;
    fling.velocity += dominant;
    if (!fling.raf) fling.raf = requestAnimationFrame(loop);
    ev.preventDefault();
  }}, {{ passive: false }});
}});
document.querySelectorAll("[data-wait-morph]").forEach(function (path) {{
  var frames = (path.getAttribute("data-wait-frames") || "").split(";").filter(Boolean);
  var row = path.closest("[data-wait-progress]");
  var ms = Number((row && row.getAttribute("data-wait-ms")) || "{wait_ms}");
  var label = row && row.querySelector("[data-wait-label]");
  var t0 = performance.now();
  function tickWait(now) {{
    var p = ((now - t0) % ms) / ms;
    if (frames.length) {{
      var i = Math.min(frames.length - 1, Math.floor(p * frames.length));
      path.setAttribute("d", frames[i]);
    }}
    if (label) label.textContent = Math.round(p * 100) + "%";
    requestAnimationFrame(tickWait);
  }}
  requestAnimationFrame(tickWait);
}});
function spatialFastAt(t) {{
  t = Math.max(0, Math.min(1, t));
  if (t === 0 || t === 1) return t;
  var x1 = 0.42, y1 = 1.67, x2 = 0.21, y2 = 0.90;
  var s = t;
  function coord(p, a, b) {{
    var u = 1 - p;
    return 3 * u * u * p * a + 3 * u * p * p * b + p * p * p;
  }}
  function deriv(p, a, b) {{
    var u = 1 - p;
    return 3 * u * u * a + 6 * u * p * (b - a) + 3 * p * p * (1 - b);
  }}
  for (var i = 0; i < 10; i++) {{
    var x = coord(s, x1, x2);
    var dx = deriv(s, x1, x2);
    if (Math.abs(dx) < 1e-6) break;
    s = Math.max(0, Math.min(1, s - (x - t) / dx));
  }}
  return Math.max(0, Math.min(1, coord(s, y1, y2)));
}}
function hexRgba(hex, a) {{
  if (!hex) return "transparent";
  hex = hex.replace("#", "");
  if (hex.length === 3) hex = hex[0]+hex[0]+hex[1]+hex[1]+hex[2]+hex[2];
  var n = parseInt(hex, 16);
  if (isNaN(n)) return "transparent";
  return "rgba(" + ((n >> 16) & 255) + ", " + ((n >> 8) & 255) + ", " + (n & 255) + ", " + a + ")";
}}
function hexMix(a, b, t) {{
  function rgb(hex) {{
    hex = (hex || "#000000").replace("#", "");
    if (hex.length === 3) hex = hex[0]+hex[0]+hex[1]+hex[1]+hex[2]+hex[2];
    var n = parseInt(hex, 16);
    if (isNaN(n)) return [0, 0, 0];
    return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  }}
  var A = rgb(a), B = rgb(b);
  var r = Math.round(A[0] + (B[0] - A[0]) * t);
  var g = Math.round(A[1] + (B[1] - A[1]) * t);
  var bl = Math.round(A[2] + (B[2] - A[2]) * t);
  function h(n) {{ return ("0" + n.toString(16)).slice(-2); }}
  return "#" + h(r) + h(g) + h(bl);
}}
function applyRailContainerMorph(rail, e) {{
  var hide = rail.getAttribute("data-hide-on-collapse") === "1";
  var c0 = rail.getAttribute("data-container-collapsed") || "";
  var c1 = rail.getAttribute("data-container-expanded") || c0;
  var r0 = Number(rail.getAttribute("data-collapsed-shape") || 0);
  var r1 = Number(rail.getAttribute("data-expanded-shape") || 0);
  if (hide) {{
    if (c1) rail.style.background = c1;
    rail.style.borderRadius = r1 + "px";
    return;
  }}
  if (c0 && c1) rail.style.background = hexMix(c0, c1, e);
  rail.style.borderRadius = (r0 + (r1 - r0) * e) + "px";
}}
function applyRailHideSlide(rail, t) {{
  var e = spatialFastAt(t);
  var expanded = Number(rail.getAttribute("data-expanded-width") || {rail_expanded});
  rail.style.width = expanded + "px";
  rail.style.transform = "translateX(" + (-expanded * (1 - e)) + "px)";
  applyRailContainerMorph(rail, 1);
  applyRailFabMorph(rail, 1, expanded);
}}
function applyRailFabMorph(rail, e, railW) {{
  var fab = rail.querySelector("[data-rail-fab-extend]");
  if (!fab) return;
  var collapsed = Number(rail.getAttribute("data-collapsed-width") || {rail_collapsed});
  var ml0 = Math.max(0, (collapsed - {fab_slot}) / 2);
  var ml1 = {fab_pad};
  var ml = ml0 + (ml1 - ml0) * e;
  var w = Math.max({fab_slot}, railW - 2 * ml);
  fab.style.width = w + "px";
  fab.style.marginLeft = ml + "px";
  fab.style.gap = ({fab_gap} * e) + "px";
  var label = fab.querySelector("[data-rail-fab-label]");
  if (label) label.style.opacity = String(e);
}}
function applyRailIconMorph(rail, t) {{
  if (rail.getAttribute("data-hide-on-collapse") === "1") {{
    applyRailHideSlide(rail, t);
    return;
  }}
  var e = spatialFastAt(t);
  var collapsed = Number(rail.getAttribute("data-collapsed-width") || {rail_collapsed});
  var expanded = {rail_expanded};
  var railW = collapsed + (expanded - collapsed) * e;
  applyRailFabMorph(rail, e, railW);
  applyRailContainerMorph(rail, e);
  rail.style.width = railW + "px";
  var destMl = 16 * e;
  var destW = Math.max(0, railW - destMl * 2);
  var destH = 52 + (56 - 52) * e;
  var destPad = 16 * e;
  var iconW = 56 + (24 - 56) * e;
  var iconH = 32 + (24 - 32) * e;
  var iconLeft = ((destW - 56) / 2) * (1 - e) + destPad * e;
  var iconTop = (destH - 24) / 2 * e;
  var lblSize = 12 + (14 - 12) * e;
  var lblLine = 16 + (20 - 16) * e;
  var lblLeft = (destPad + 24 + 8) * e;
  var lblTop = (32 + 4) * (1 - e) + ((destH - lblLine) / 2) * e;
  var lblW = Math.max(0, destW - lblLeft - destPad);
  rail.querySelectorAll(".dest").forEach(function (dest) {{
    var active = dest.getAttribute("data-active") === "1";
    var ind = dest.style.getPropertyValue("--ind") || "#E8DEF8";
    dest.style.setProperty("--dest-w", destW + "px");
    dest.style.setProperty("--dest-h", destH + "px");
    dest.style.setProperty("--dest-ml", destMl + "px");
    dest.style.setProperty("--dest-r", (destH / 2) + "px");
    dest.style.setProperty("--dest-bg", active ? hexRgba(ind, e) : "transparent");
    dest.style.setProperty("--icon-left", iconLeft + "px");
    dest.style.setProperty("--icon-top", iconTop + "px");
    dest.style.setProperty("--icon-w", iconW + "px");
    dest.style.setProperty("--icon-h", iconH + "px");
    dest.style.setProperty("--icon-bg", active ? hexRgba(ind, 1 - e) : "transparent");
    dest.style.setProperty("--lbl-left", lblLeft + "px");
    dest.style.setProperty("--lbl-top", lblTop + "px");
    dest.style.setProperty("--lbl-w", lblW + "px");
    dest.style.setProperty("--lbl-size", lblSize + "px");
    dest.style.setProperty("--lbl-line", lblLine + "px");
    dest.style.setProperty("--lbl-align", e < 0.5 ? "center" : "start");
    dest.setAttribute("data-icon-position", e >= 0.5 ? "start" : "top");
  }});
}}
function clearRailIconMorph(rail) {{
  if (rail.getAttribute("data-hide-on-collapse") === "1") {{
    var expanded = Number(rail.getAttribute("data-expanded-width") || {rail_expanded});
    var shown = rail.getAttribute("data-nav-rail-expanded") === "1";
    rail.style.width = expanded + "px";
    rail.style.transform = shown ? "translateX(0)" : "translateX(-100%)";
    applyRailContainerMorph(rail, 1);
    applyRailFabMorph(rail, 1, expanded);
    return;
  }}
  var shown = rail.getAttribute("data-nav-rail-expanded") === "1";
  applyRailContainerMorph(rail, shown ? 1 : 0);
  rail.style.width = "";
  var fab = rail.querySelector("[data-rail-fab-extend]");
  if (fab) {{
    fab.style.width = "";
    fab.style.marginLeft = "";
    fab.style.gap = "";
    var label = fab.querySelector("[data-rail-fab-label]");
    if (label) label.style.opacity = "";
  }}
  rail.querySelectorAll(".dest").forEach(function (dest) {{
    ["--dest-w","--dest-h","--dest-ml","--dest-r","--dest-bg","--icon-left","--icon-top","--icon-w","--icon-h","--icon-bg","--lbl-left","--lbl-top","--lbl-w","--lbl-size","--lbl-line","--lbl-align"].forEach(function (k) {{
      dest.style.removeProperty(k);
    }});
  }});
}}
document.querySelectorAll("[data-nav-rail]").forEach(function (rail) {{
  rail.querySelectorAll(".dest").forEach(function (dest, i) {{
    dest.style.cursor = "pointer";
    dest.addEventListener("click", function () {{
      rail.setAttribute("data-rail-selected", String(i));
      rail.querySelectorAll(".dest").forEach(function (d, j) {{
        d.setAttribute("data-active", j === i ? "1" : "0");
      }});
    }});
  }});
  var toggles = rail.querySelectorAll("[data-rail-fab], [data-rail-header-menu]");
  toggles.forEach(function (fab) {{
    fab.style.cursor = "pointer";
    fab.addEventListener("click", function () {{
      if (rail._iconMorphRaf) cancelAnimationFrame(rail._iconMorphRaf);
      var exp = rail.getAttribute("data-nav-rail-expanded") !== "1";
      var ms = Number(rail.getAttribute("data-icon-morph-ms") || "{morph_ms}");
      var from = exp ? 0 : 1;
      var to = exp ? 1 : 0;
      var hide = rail.getAttribute("data-hide-on-collapse") === "1";
      if (hide) rail.setAttribute("data-hide-sliding", "1");
      rail.setAttribute("data-icon-morphing", hide ? "0" : "1");
      rail.setAttribute("data-icon-morph", hide ? "0" : "1");
      var t0 = performance.now();
      function tick(now) {{
        var p = Math.min(1, (now - t0) / ms);
        applyRailIconMorph(rail, from + (to - from) * p);
        if (p < 1) {{
          rail._iconMorphRaf = requestAnimationFrame(tick);
          return;
        }}
        rail._iconMorphRaf = 0;
        rail.classList.toggle("expanded", exp);
        rail.setAttribute("data-nav-rail-expanded", exp ? "1" : "0");
        rail.setAttribute("data-rail-mode", exp ? "expanded" : "collapsed");
        rail.setAttribute("data-icon-position", hide || exp ? "start" : "top");
        rail.removeAttribute("data-icon-morphing");
        rail.removeAttribute("data-hide-sliding");
        clearRailIconMorph(rail);
        var layout = rail.getAttribute("data-rail-layout") || "modal";
        rail.setAttribute("data-wide-collapsed", !hide && !exp && layout !== "narrow" && layout !== "header" ? "1" : "0");
        if (layout === "narrow") {{
          rail.setAttribute("data-narrow", "1");
        }}
        rail.querySelectorAll("[data-rail-fab]").forEach(function (slot) {{
          if (slot.getAttribute("data-rail-fab-extend") === "1") return;
          slot.textContent = hide || exp ? "←" : "+";
        }});
        rail.querySelectorAll("[data-rail-header-menu]").forEach(function (btn) {{
          btn.textContent = exp ? "{header_open}" : "{header_menu}";
          var label = exp ? "{header_collapse}" : "{header_expand}";
          var state = exp ? "{header_state_exp}" : "{header_state_col}";
          btn.setAttribute("data-rail-header-label", label);
          btn.setAttribute("data-rail-header-state", state);
          btn.setAttribute("aria-label", label);
          var tip = rail.querySelector("[data-rail-header-tooltip]");
          if (tip) {{
            tip.textContent = label;
            tip.setAttribute("data-tooltip-text", label);
          }}
        }});
        var stage = rail.closest(".rail-stage");
        if (stage && (layout === "standard" || layout === "header")) {{
          stage.classList.remove("is-modal");
          var inflowScrim = stage.querySelector("[data-rail-scrim]");
          if (inflowScrim) inflowScrim.setAttribute("data-visible", "0");
        }} else if (stage) {{
          stage.classList.toggle("is-modal", exp);
          var scrim = stage.querySelector("[data-rail-scrim]");
          if (scrim) scrim.setAttribute("data-visible", exp ? "1" : "0");
          var win = stage.querySelector("[data-rail-window]");
          if (win) win.setAttribute("data-rail-window", exp ? "1" : "0");
        }}
      }}
      rail._iconMorphRaf = requestAnimationFrame(tick);
    }});
  }});
}});
document.querySelectorAll("[data-rail-menu]").forEach(function (btn) {{
  btn.style.cursor = "pointer";
  btn.addEventListener("click", function () {{
    var stage = btn.closest(".rail-stage");
    var rail = stage && stage.querySelector("[data-hide-on-collapse]");
    var fab = rail && rail.querySelector("[data-rail-fab]");
    if (fab) fab.click();
  }});
}});
document.querySelectorAll("[data-fab-menu]").forEach(function (menu) {{
  var btn = menu.querySelector("[data-fab-close]");
  if (!btn) return;
  btn.style.cursor = "pointer";
  btn.addEventListener("click", function () {{
    var open = menu.getAttribute("data-expanded") !== "1";
    menu.setAttribute("data-expanded", open ? "1" : "0");
    btn.textContent = open ? menu.getAttribute("data-close-glyph") : menu.getAttribute("data-open-glyph");
    var r = open ? menu.getAttribute("data-open-r") : menu.getAttribute("data-closed-r");
    var bg = open ? menu.getAttribute("data-open-bg") : menu.getAttribute("data-closed-bg");
    var fg = open ? menu.getAttribute("data-open-fg") : menu.getAttribute("data-closed-fg");
    btn.style.borderRadius = r + "px";
    btn.style.background = bg;
    btn.style.color = fg;
  }});
}});
document.querySelectorAll("[data-split]").forEach(function (split) {{
  var trail = split.querySelector("[data-split-trail]");
  if (!trail) return;
  var menu = split.querySelector("[data-split-menu]");
  if (menu) menu.addEventListener("click", function (ev) {{ ev.stopPropagation(); }});
  trail.addEventListener("click", function (ev) {{
    ev.stopPropagation();
    var open = split.getAttribute("data-open") !== "1";
    split.setAttribute("data-open", open ? "1" : "0");
    trail.textContent = open ? "▴" : "▾";
    if (open) focusTypeahead(split.querySelector("[data-overflow-cascade], [data-split-cascade]"));
  }});
}});
document.addEventListener("click", function () {{
  document.querySelectorAll("[data-split]").forEach(function (split) {{
    split.setAttribute("data-open", "0");
    var trail = split.querySelector("[data-split-trail]");
    if (trail) trail.textContent = "▾";
  }});
}});
document.querySelectorAll("[data-menu-keyboard]").forEach(function (root) {{
  var flyout = root.querySelector("[data-menu-submenu]");
  var trigger = root.querySelector("[data-submenu-trigger]");
  var delay = Number(root.getAttribute("data-hover-delay") || "0");
  var hoverTimer = null;
  var flyAttr = root.hasAttribute("data-overflow-cascade") ? "data-flyout" : "data-open";
  function itemList(node) {{
    return node ? Array.prototype.slice.call(node.querySelectorAll("[data-menu-item]")) : [];
  }}
  function setHi(items, idx) {{
    items.forEach(function (el, i) {{
      el.setAttribute("data-menu-hi", i === idx ? "1" : "0");
    }});
  }}
  function applyOpen(open) {{
    root.setAttribute(flyAttr, open ? "1" : "0");
    root.setAttribute("data-menu-focus-parent", open ? "inactive" : "rest");
    if (flyout) flyout.style.display = open ? "flex" : "none";
    if (trigger) trigger.setAttribute("aria-expanded", open ? "true" : "false");
    root.querySelectorAll("[data-r-rest]").forEach(function (g) {{
      var r = open ? g.getAttribute("data-r-inactive") : g.getAttribute("data-r-rest");
      if (r) g.style.borderRadius = r;
    }});
  }}
  function typeahead(items, from, ch) {{
    if (!items.length) return from;
    var needle = ch.toLowerCase();
    var start = ((from < 0 ? -1 : from) + 1) % items.length;
    for (var step = 0; step < items.length; step++) {{
      var i = (start + step) % items.length;
      var label = (items[i].getAttribute("data-menu-item") || "").charAt(0).toLowerCase();
      if (label === needle) return i;
    }}
    return from;
  }}
  function openSoon() {{
    if (delay <= 0) {{ applyOpen(true); return; }}
    clearTimeout(hoverTimer);
    hoverTimer = setTimeout(function () {{ applyOpen(true); }}, delay);
  }}
  function cancelHover() {{
    clearTimeout(hoverTimer);
    hoverTimer = null;
  }}
  var hoverWhole = root.hasAttribute("data-menu-cascade") && !root.hasAttribute("data-menu-overlay");
  if (hoverWhole) {{
    root.addEventListener("mouseenter", function () {{
      openSoon();
      focusTypeahead(root);
    }});
  }} else if (trigger) {{
    trigger.addEventListener("mouseenter", openSoon);
  }}
  root.addEventListener("mouseleave", function () {{
    cancelHover();
    applyOpen(false);
  }});
  if (trigger) {{
    trigger.style.cursor = "pointer";
    trigger.addEventListener("click", function (ev) {{
      ev.stopPropagation();
      cancelHover();
      applyOpen(root.getAttribute(flyAttr) !== "1");
    }});
  }}
  root.querySelectorAll("[data-menu-item]:not([data-submenu-trigger])").forEach(function (el) {{
    el.addEventListener("click", function (ev) {{
      ev.stopPropagation();
      if (root.hasAttribute("data-overflow-cascade")) {{
        root.setAttribute("data-open", "0");
        root.style.display = "none";
      }}
      var split = root.closest("[data-split]");
      if (split) {{
        split.setAttribute("data-open", "0");
        var trail = split.querySelector("[data-split-trail]");
        if (trail) trail.textContent = "▾";
      }}
    }});
  }});
  root.setAttribute("tabindex", "0");
  if (hoverWhole) focusTypeahead(root);
  root.addEventListener("keydown", function (ev) {{
    var open = root.getAttribute(flyAttr) === "1";
    var parentItems = itemList(root.querySelector("[data-menu-parent]"));
    var subItems = itemList(flyout);
    var items = open ? subItems : parentItems;
    var cur = items.findIndex(function (el) {{ return el.getAttribute("data-menu-hi") === "1"; }});
    if (ev.key === "ArrowRight") {{
      applyOpen(true);
      setHi(subItems, 0);
      ev.preventDefault();
    }} else if (ev.key === "ArrowLeft" || ev.key === "Escape") {{
      applyOpen(false);
      setHi(parentItems, parentItems.length - 1);
      ev.preventDefault();
    }} else if (ev.key === "ArrowDown") {{
      var next = cur < 0 ? 0 : (cur + 1) % items.length;
      setHi(items, next);
      ev.preventDefault();
    }} else if (ev.key === "ArrowUp") {{
      var prev = cur < 0 ? items.length - 1 : (cur - 1 + items.length) % items.length;
      setHi(items, prev);
      ev.preventDefault();
    }} else if (ev.key.length === 1 && !ev.ctrlKey && !ev.metaKey && !ev.altKey) {{
      var hit = typeahead(items, cur < 0 ? items.length - 1 : cur, ev.key);
      if (hit >= 0) setHi(items, hit);
      ev.preventDefault();
    }}
  }});
}});
</script>
</body>
</html>
"##,
        bg = c.background.css_hex(),
        on_bg = c.on_background.css_hex(),
        surface = c.surface.css_hex(),
        on_surface = c.on_surface.css_hex(),
        outline = c.outline.css_hex(),
        primary = c.primary.css_hex(),
        on_primary = c.on_primary.css_hex(),
        surface_low = c.surface_container_low.css_hex(),
        on_var = c.on_surface_variant.css_hex(),
        appbar = c.surface.css_hex(),
        outline_var = c.outline_variant.css_hex(),
        body = body,
        motion = theme.motion.css_state_transition(),
        ease = theme.motion.emphasized,
        fling_unit = carousel::FLING_UNIT,
        fling_decay = carousel::FLING_DECAY,
        fling_snap = carousel::FLING_SNAP_FRACTION,
        wait_ms = progress::determinate_wait_ms(theme),
        rail_collapsed = navigation_rail::WIDE_COLLAPSED_WIDTH_DP,
        rail_expanded = navigation_rail::EXPANDED_WIDTH_DP,
        content_pad_v = navigation_rail::CONTENT_PAD_VERTICAL_DP,
        content_pad_h = navigation_rail::CONTENT_PAD_HORIZONTAL_DP,
        fab_slot = navigation_rail::FAB_SLOT_DP,
        fab_pad = navigation_rail::FAB_PAD_EXPANDED_DP,
        fab_gap = navigation_rail::FAB_ICON_LABEL_GAP_DP,
        morph_ms = navigation_rail::morph_ms(theme),
        header_menu = navigation_rail::HEADER_MENU_GLYPH,
        header_open = navigation_rail::HEADER_MENU_OPEN_GLYPH,
        header_expand = navigation_rail::HEADER_EXPAND_LABEL,
        header_collapse = navigation_rail::HEADER_COLLAPSE_LABEL,
        header_state_col = navigation_rail::HEADER_STATE_COLLAPSED,
        header_state_exp = navigation_rail::HEADER_STATE_EXPANDED,
        gap = button_group::CONNECTED_GAP_DP,
        h1s = theme.typography.display_small.emphasized().size_sp,
        h1l = theme.typography.display_small.emphasized().line_height_sp,
        h1w = theme.typography.display_small.emphasized().weight,
        h1t = theme.typography.display_small.emphasized().tracking_sp,
    )
}

fn hero(theme: &Theme) -> String {
    let bar = top_app_bar::resolve(theme);
    format!(
        r#"<header class="bar" style="background:{bg};color:{fg}">
  <div class="title" style="font-size:{sz}px;line-height:{lh}px">{title}</div>
  <div>{mode}</div>
</header>
<h1 data-type="displaySmallEmphasized">Material 3 component catalog</h1>
<p class="lead">Current <a href="https://m3.material.io">Material 3 / Expressive</a> (m3.material.io). Heroes match official overview scenes; state matrices follow. Same <code>resolve()</code> drives the Android demo.</p>"#,
        bg = bar.container.css_hex(),
        fg = bar.title.css_hex(),
        sz = bar.title_style.size_sp,
        lh = bar.title_style.line_height_sp,
        title = "GPUI Material",
        mode = if theme.dark {
            "dark theme"
        } else {
            "light theme"
        },
    )
}

fn inventory_section() -> String {
    let mut rows = String::new();
    for e in inventory::INVENTORY {
        let (bg, fg) = match e.parity {
            Parity::Done => ("#146C2E", "#FFFFFF"),
            Parity::Partial => ("#7A5900", "#FFFFFF"),
            Parity::NotStarted => ("#6B6B6B", "#FFFFFF"),
        };
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><a href=\"{}\">spec</a></td></tr>",
            esc(e.name),
            esc(e.material),
            pill(e.parity.label(), bg, fg),
            esc(e.notes),
            e.docs
        ));
    }
    format!(
        "<h2>Inventory</h2><table class=\"inv\"><tr><th>Component</th><th>Material</th><th>Parity</th><th>Notes</th><th>Docs</th></tr>{rows}</table>"
    )
}

fn color_section(theme: &Theme) -> String {
    let c = theme.color;
    let roles = [
        ("primary", c.primary, c.on_primary),
        ("onPrimary", c.on_primary, c.primary),
        (
            "primaryContainer",
            c.primary_container,
            c.on_primary_container,
        ),
        (
            "onPrimaryContainer",
            c.on_primary_container,
            c.primary_container,
        ),
        ("secondary", c.secondary, c.on_secondary),
        ("onSecondary", c.on_secondary, c.secondary),
        (
            "secondaryContainer",
            c.secondary_container,
            c.on_secondary_container,
        ),
        ("tertiary", c.tertiary, c.on_tertiary),
        (
            "tertiaryContainer",
            c.tertiary_container,
            c.on_tertiary_container,
        ),
        ("error", c.error, c.on_error),
        ("errorContainer", c.error_container, c.on_error_container),
        ("surface", c.surface, c.on_surface),
        ("onSurface", c.on_surface, c.surface),
        ("surfaceContainerLow", c.surface_container_low, c.on_surface),
        ("surfaceContainer", c.surface_container, c.on_surface),
        (
            "surfaceContainerHigh",
            c.surface_container_high,
            c.on_surface,
        ),
        (
            "surfaceContainerHighest",
            c.surface_container_highest,
            c.on_surface,
        ),
        ("onSurfaceVariant", c.on_surface_variant, c.surface),
        ("outline", c.outline, c.surface),
        ("outlineVariant", c.outline_variant, c.on_surface),
        ("inverseSurface", c.inverse_surface, c.inverse_on_surface),
        ("inversePrimary", c.inverse_primary, c.inverse_surface),
    ];
    let mut cells = String::new();
    for (name, bg, fg) in roles {
        cells.push_str(&format!(
            "<div class=\"swatch\" data-role=\"{name}\" style=\"background:{bg};color:{fg}\">{name}<code>{hex}</code></div>",
            bg = bg.css_hex(),
            fg = fg.css_hex(),
            hex = bg.css_hex(),
        ));
    }
    format!("<h2>Color roles</h2><div class=\"grid\">{cells}</div>")
}

fn type_section(theme: &Theme) -> String {
    let mut out = String::from("<h2>Type scale</h2><h3>baseline</h3>");
    for style in theme.typography.all() {
        out.push_str(&format!(
            "<div data-type=\"{name}\" style=\"font-size:{sz}px;line-height:{lh}px;letter-spacing:{tr}px;font-weight:{w};color:{c}\">{name} · {sz}/{lh} · {w}</div>",
            name = style.name,
            sz = style.size_sp,
            lh = style.line_height_sp,
            tr = style.tracking_sp,
            w = style.weight,
            c = theme.color.on_surface.css_hex(),
        ));
    }
    out.push_str("<h3>emphasized</h3><p class=\"note\">Expressive hero moments: same size/line-height, heavier weight. Wired on catalog display headline, dialog headlines, date-picker large dates, time-picker clock, settings section titles.</p>");
    for style in theme.typography.emphasized().all() {
        out.push_str(&format!(
            "<div data-type=\"{name}\" style=\"font-size:{sz}px;line-height:{lh}px;letter-spacing:{tr}px;font-weight:{w};color:{c}\">{name} · {sz}/{lh} · {w}</div>",
            name = style.name,
            sz = style.size_sp,
            lh = style.line_height_sp,
            tr = style.tracking_sp,
            w = style.weight,
            c = theme.color.on_surface.css_hex(),
        ));
    }
    out
}

fn paint_standard_group(theme: &Theme, selected: usize, overflow_open: bool) -> String {
    let count = button_group::STANDARD_SEGMENTS.len();
    let ov = button_group::resolve_standard_overflow(theme, false);
    let mut parts = String::from(
        r#"<div class="overflow-menu" data-standard-overflow="1" data-hero="button-group-standard">"#,
    );
    parts.push_str(&format!(
        r#"<div class="btn-group" data-button-group="standard" data-hero="button-group-standard" data-selected="{sel}" data-base-w="{base}" data-expanded-ratio="{ratio}" data-standard-gap="{gap}">"#,
        sel = selected,
        base = button_group::STANDARD_BASE_W_DP,
        ratio = format!("{:.2}", button_group::EXPANDED_RATIO),
        gap = button_group::STANDARD_GAP_DP,
    ));
    for (i, label) in button_group::STANDARD_SEGMENTS.iter().enumerate() {
        let a = button_group::resolve_standard_scene(theme, i, selected);
        let idle = button_group::resolve_standard(theme, i, count, false, false, None);
        let on = button_group::resolve_standard(theme, i, count, true, false, Some(i));
        let class = if i == selected {
            "btn btn-standard selected"
        } else {
            "btn btn-standard"
        };
        let w = a.width_dp.unwrap_or(button_group::STANDARD_BASE_W_DP);
        parts.push_str(&format!(
            r#"<button class="{class}" data-standard-i="{i}" data-idle-bg="{ibg}" data-idle-fg="{ifg}" data-idle-r="{ir}" data-sel-bg="{sbg}" data-sel-fg="{sfg}" data-sel-r="{sr}" style="width:{w}px;--press-r:{pr}px;background:{bg};color:{fg};border-radius:{r};height:{h}px;padding:0 {pad}px;font-size:{fs}px;font-weight:{fw}">{label}</button>"#,
            class = class,
            i = i,
            ibg = idle.container.css_hex(),
            ifg = idle.content.css_hex(),
            ir = idle.corners.css(),
            sbg = on.container.css_hex(),
            sfg = on.content.css_hex(),
            sr = on.corners.css(),
            w = w,
            pr = button::ButtonSize::Small.pressed_corner_dp(),
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            r = a.corners.css(),
            h = a.height_dp,
            pad = a.pad_start_dp,
            fs = a.label_style.size_sp,
            fw = a.label_style.weight,
            label = label,
        ));
    }
    parts.push_str(&format!(
        r#"<button class="btn btn-standard-overflow" data-overflow="1" data-standard-overflow-btn="1" style="--press-r:{pr}px;background:{bg};color:{fg};border-radius:{r};height:{h}px;min-width:{mw}px;padding:0;font-size:{fs}px">{glyph}</button>"#,
        pr = button::ButtonSize::Small.pressed_corner_dp(),
        bg = ov.container.css_hex(),
        fg = ov.content.css_hex(),
        r = ov.corners.css(),
        h = ov.height_dp,
        mw = ov.width_dp.unwrap_or(icon_button::container_width_dp(
            button::ButtonSize::Small,
            icon_button::IconButtonWidth::Default,
        )),
        fs = ov.label_style.size_sp,
        glyph = button_group::STANDARD_OVERFLOW_GLYPH,
    ));
    parts.push_str("</div>");
    parts.push_str(&paint_grouped_overflow(
        theme,
        menu::GroupedPopupKind::StandardOverflow,
        overflow_open,
        "overflow-cascade",
        r#" data-overflow-menu="1" data-standard-overflow-menu="1""#,
    ));
    parts.push_str("</div>");
    parts
}

fn paint_connected_group(theme: &Theme, selected: usize) -> String {
    let count = button_group::DEMO_SEGMENTS.len();
    let mut parts = String::from(
        r#"<div class="btn-group" data-button-group="connected" data-hero="button-group">"#,
    );
    for (i, label) in button_group::DEMO_SEGMENTS.iter().enumerate() {
        let sel = i == selected;
        let a = button_group::resolve_segment(theme, i, count, sel, false);
        let idle = button_group::resolve_segment(theme, i, count, false, false);
        let on = button_group::resolve_segment(theme, i, count, true, false);
        let role = match button_group::segment_role(i, count) {
            button_group::SegmentRole::Leading => "leading",
            button_group::SegmentRole::Middle => "middle",
            button_group::SegmentRole::Trailing => "trailing",
        };
        let class = if sel {
            format!("btn btn-connected {role} selected")
        } else {
            format!("btn btn-connected {role}")
        };
        parts.push_str(&format!(
            r#"<button class="{class}" data-segment="{i}" data-role="{role}" data-idle-bg="{ibg}" data-idle-fg="{ifg}" data-idle-r="{ir}" data-idle-bd="{ibd}" data-sel-bg="{sbg}" data-sel-fg="{sfg}" data-sel-r="{sr}" style="--press-r:{pr}px;background:{bg};color:{fg};border:{bd};border-radius:{r};height:{h}px;padding:0 {pad}px;font-size:{fs}px;font-weight:{fw}">{label}</button>"#,
            class = class,
            i = i,
            role = role,
            ibg = idle.container.css_hex(),
            ifg = idle.content.css_hex(),
            ir = idle.corners.css(),
            ibd = idle.outline_css(),
            sbg = on.container.css_hex(),
            sfg = on.content.css_hex(),
            sr = on.corners.css(),
            pr = button::ButtonSize::Small.pressed_corner_dp(),
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            bd = a.outline_css(),
            r = a.corners.css(),
            h = a.height_dp,
            pad = a.pad_start_dp,
            fs = a.label_style.size_sp,
            fw = a.label_style.weight,
            label = label,
        ));
    }
    parts.push_str("</div>");
    parts
}

fn paint_icon_group(theme: &Theme, selected: usize, overflow_open: bool) -> String {
    let count = button_group::icon_group_count();
    let mut parts = String::from(r#"<div class="overflow-menu" data-hero="button-group-icons">"#);
    parts.push_str(r#"<div class="btn-group" data-button-group="icons" data-icon-group="1">"#);
    for i in 0..count {
        let glyph = button_group::icon_glyph(i);
        let overflow = i == button_group::overflow_index();
        let sel = !overflow && i == selected;
        let a = button_group::resolve_icon_segment(theme, i, count, sel, false);
        let idle = button_group::resolve_icon_segment(theme, i, count, false, false);
        let on = button_group::resolve_icon_segment(theme, i, count, true, false);
        let role = match button_group::segment_role(i, count) {
            button_group::SegmentRole::Leading => "leading",
            button_group::SegmentRole::Middle => "middle",
            button_group::SegmentRole::Trailing => "trailing",
        };
        let class = if sel {
            format!("btn btn-connected {role} selected")
        } else {
            format!("btn btn-connected {role}")
        };
        parts.push_str(&format!(
            r#"<button class="{class}" data-segment="{i}" data-role="{role}" data-overflow="{ov}" data-idle-bg="{ibg}" data-idle-fg="{ifg}" data-idle-r="{ir}" data-idle-bd="{ibd}" data-sel-bg="{sbg}" data-sel-fg="{sfg}" data-sel-r="{sr}" style="--press-r:{pr}px;background:{bg};color:{fg};border:{bd};border-radius:{r};height:{h}px;min-width:{mw}px;padding:0 {pad}px;font-size:{fs}px">{glyph}</button>"#,
            class = class,
            i = i,
            role = role,
            ov = if overflow { "1" } else { "0" },
            ibg = idle.container.css_hex(),
            ifg = idle.content.css_hex(),
            ir = idle.corners.css(),
            ibd = idle.outline_css(),
            sbg = on.container.css_hex(),
            sfg = on.content.css_hex(),
            sr = on.corners.css(),
            pr = button::ButtonSize::Small.pressed_corner_dp(),
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            bd = a.outline_css(),
            r = a.corners.css(),
            h = a.height_dp,
            mw = a.min_width_dp.unwrap_or(button_group::ICON_MIN_W_DP),
            pad = a.pad_start_dp,
            fs = a.label_style.size_sp,
            glyph = glyph,
        ));
    }
    parts.push_str("</div>");
    parts.push_str(&paint_grouped_overflow(
        theme,
        menu::GroupedPopupKind::ConnectedOverflow,
        overflow_open,
        "overflow-cascade",
        r#" data-overflow-menu="1""#,
    ));
    parts.push_str("</div>");
    parts
}

fn settings_scene(theme: &Theme) -> String {
    let title = theme.typography.title_large.emphasized();
    let section = theme.typography.title_medium.emphasized();
    let outlined = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    let mut rows = String::new();
    for row in slider::OVERVIEW_ROWS.iter().take(2) {
        let a =
            slider::resolve_with_stops(theme, row.value, InteractionState::Enabled, row.stop_count);
        rows.push_str(&format!(
            r#"<div class="slider-row" data-slider-row="{label}"><div class="slider-icon" aria-hidden="true">{icon}</div><div class="slider-meta"><div class="slider-label">{label}</div>{}</div></div>"#,
            paint_expressive_slider(&a, row.label),
            label = row.label,
            icon = row.icon,
        ));
    }
    let text_btn = button::resolve(
        theme,
        button::ButtonVariant::Text,
        InteractionState::Enabled,
    );
    format!(
        r#"<h2>Settings scene</h2>
<p class="note">Containment / hierarchy: surface-container, 16dp pad, 24dp groups, emphasized section titles.</p>
<div class="hero-card settings-scene" data-settings-scene="1" style="background:{bg};border-radius:{r}px;padding:{pad}px;gap:{gap}px">
  <h3 data-type="titleLargeEmphasized" style="font-size:{ts}px;line-height:{tl}px;font-weight:{tw};color:{on}">{title}</h3>
  <div class="settings-block" data-settings-block="volume">
    <h4 data-type="titleMediumEmphasized" style="font-size:{ss}px;line-height:{sl}px;font-weight:{sw};color:{on}">{volume}</h4>
    {rows}
  </div>
  <div class="settings-block" data-settings-block="quiet-hours">
    <h4 data-type="titleMediumEmphasized" style="font-size:{ss}px;line-height:{sl}px;font-weight:{sw};color:{on}">{quiet}</h4>
    {field}
    {group}
  </div>
  <div class="actions"><button class="btn" data-settings-dialog="1" style="background:{abg};color:{act}">{reset}</button></div>
</div>"#,
        bg = theme.color.surface_container.css_hex(),
        r = button_group::SETTINGS_CORNER_DP,
        pad = button_group::SETTINGS_PAD_DP,
        gap = button_group::SETTINGS_GROUP_GAP_DP,
        ts = title.size_sp,
        tl = title.line_height_sp,
        tw = title.weight,
        ss = section.size_sp,
        sl = section.line_height_sp,
        sw = section.weight,
        on = theme.color.on_surface.css_hex(),
        title = button_group::SETTINGS_SCENE_TITLE,
        volume = button_group::SETTINGS_VOLUME_TITLE,
        quiet = button_group::SETTINGS_QUIET_HOURS_TITLE,
        rows = rows,
        field = paint_outlined_field(
            &outlined,
            "data-field-hero=\"settings-email\"",
            "Email",
            r#"<input class="val" value="you@domain.com" data-editor="settings"/>"#,
        ),
        group = paint_connected_group(theme, button_group::DEMO_SELECTED),
        abg = text_btn.container.css_hex(),
        act = theme.color.primary.css_hex(),
        reset = dialog::RESET_HEADLINE.trim_end_matches('?'),
    )
}

fn paint_button(theme: &Theme, variant: button::ButtonVariant, state: InteractionState) -> String {
    let a = button::resolve(theme, variant, state);
    format!(
        "<button class=\"btn\" data-button=\"{v}\" data-state=\"{s}\" style=\"--press-r:{pr}px;background:{bg};color:{fg};border:{bd};box-shadow:{sh};min-width:{mw}px;height:{h}px;padding:0 {pad}px;border-radius:{r}px;font-size:{fs}px\">{label}</button>",
        v = variant.label(),
        s = state.label(),
        pr = button::ButtonSize::Small.pressed_corner_dp(),
        bg = a.container.css_hex(),
        fg = a.content.css_hex(),
        bd = a.outline_css(),
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        mw = a.min_width_dp.unwrap_or(64.0),
        h = a.height_dp,
        pad = a.pad_start_dp,
        r = a.corners.top_left,
        fs = a.label_style.size_sp,
        label = variant.overview_label(),
    )
}

fn buttons(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Buttons</h2><p class=\"note\">M3 Expressive: five colors, XS–XL, round/square, press morph. Default S is 40×16. <a href=\"https://m3.material.io/components/buttons/specs\">spec</a></p>",
    );
    out.push_str("<div class=\"hero-card\" data-hero=\"buttons\"><div class=\"state-body\">");
    for variant in button::OVERVIEW_ORDER {
        out.push_str(&paint_button(theme, variant, InteractionState::Enabled));
    }
    out.push_str("</div><h3>sizes</h3><div class=\"state-body\">");
    for size in button::ButtonSize::ALL {
        let a = button::resolve_expressive(
            theme,
            button::ButtonVariant::Filled,
            size,
            button::ButtonShape::Round,
            InteractionState::Enabled,
        );
        out.push_str(&format!(
            "<button class=\"btn\" data-button-size=\"{s}\" style=\"--press-r:{pr}px;background:{bg};color:{fg};height:{h}px;padding:0 {pad}px;border-radius:{r}px;font-size:{fs}px;min-width:{mw}px\">Label</button>",
            s = size.label(),
            pr = size.pressed_corner_dp(),
            mw = a.min_width_dp.unwrap_or(64.0).max(a.height_dp * 0.6),
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            h = a.height_dp,
            pad = a.pad_start_dp,
            r = a.corners.top_left,
            fs = a.label_style.size_sp,
        ));
    }
    out.push_str("</div><h3>shapes</h3><div class=\"state-body\">");
    for shape in [button::ButtonShape::Round, button::ButtonShape::Square] {
        let a = button::resolve_expressive(
            theme,
            button::ButtonVariant::Filled,
            button::ButtonSize::Small,
            shape,
            InteractionState::Enabled,
        );
        out.push_str(&format!(
            "<button class=\"btn\" data-button-shape=\"{s}\" style=\"--press-r:8px;background:{bg};color:{fg};height:{h}px;padding:0 {pad}px;border-radius:{r}px\">{label}</button>",
            s = shape.label(),
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            h = a.height_dp,
            pad = a.pad_start_dp,
            r = a.corners.top_left,
            label = if matches!(shape, button::ButtonShape::Round) {
                "Round"
            } else {
                "Square"
            },
        ));
    }
    out.push_str("</div><p class=\"note\">Press any button — corners morph to the Expressive pressed radius (S → 8dp, M → 12dp, L/XL → 16dp).</p>");
    out.push_str("<h3>standard button group</h3>");
    out.push_str(&paint_standard_group(
        theme,
        button_group::STANDARD_SELECTED,
        button_group::STANDARD_OVERFLOW_OPEN,
    ));
    out.push_str("<p class=\"note\">Standard group: 12dp gap, ExpandedRatio 0.15 — the selected child grows and neighbors compress. Tonal round → filled square. Trailing filled OverflowIndicator opens a grouped 2dp menu (Left / Right / Justify + More › Share/Save/Sort flyout).</p>");
    out.push_str("<h3>connected button group</h3>");
    out.push_str(&paint_connected_group(theme, button_group::DEMO_SELECTED));
    out.push_str("<h3>connected icon row + overflow</h3>");
    out.push_str(&paint_icon_group(
        theme,
        button_group::ICON_SELECTED,
        button_group::OVERFLOW_OPEN,
    ));
    out.push_str("<p class=\"note\">Expressive connected group: 2dp gap, 8dp inner corners, full-round outer. Selected segment morphs toward square (checkedShape). Overflow opens Cut / Copy / Paste + More › grouped flyout. Click to restyle.</p></div>");
    for variant in button::ButtonVariant::ALL {
        out.push_str(&format!("<h3>{}</h3>", variant.label()));
        for state in InteractionState::ALL_COMMON {
            out.push_str(&state_row_open(state.label()));
            out.push_str(&paint_button(theme, variant, state));
            out.push_str("</div></div>");
        }
    }
    out
}

fn paint_icon_btn(a: &Appearance, attrs: &str, press_r: f32, font_px: f32, glyph: &str) -> String {
    format!(
        "<div class=\"icon-btn\" {attrs} style=\"--press-r:{press_r}px;width:{w}px;height:{h}px;background:{bg};color:{fg};border:{bd};border-radius:{r}px;font-size:{font_px}px\">{glyph}</div>",
        w = a.width_dp.unwrap_or(a.height_dp),
        h = a.height_dp,
        bg = a.container.css_hex(),
        fg = a.content.css_hex(),
        bd = a.outline_css(),
        r = a.corners.top_left,
    )
}

fn paint_icon_width_row(theme: &Theme, size: button::ButtonSize) -> String {
    let mut out = format!(
        "<div class=\"state-body\" data-icon-width-size=\"{}\">",
        size.label()
    );
    for width in icon_button::IconButtonWidth::ALL {
        let a = icon_button::resolve_width(
            theme,
            icon_button::IconButtonVariant::Filled,
            size,
            button::ButtonShape::Round,
            width,
            InteractionState::Enabled,
        );
        out.push_str(&paint_icon_btn(
            &a,
            &format!(
                "data-icon-width=\"{w}\" data-icon-width-w=\"{px}\"",
                w = width.label(),
                px = a.width_dp.unwrap_or(a.height_dp)
            ),
            size.pressed_corner_dp(),
            icon_button::icon_dp(size) * 0.75,
            "★",
        ));
    }
    out.push_str("</div>");
    out
}

fn paint_icon_toggle_row(
    theme: &Theme,
    rest: button::ButtonShape,
    variants: &[icon_button::IconButtonVariant],
) -> String {
    let size = icon_button::TOGGLE_HERO_SIZE;
    let mut out = format!(
        "<div class=\"state-body\" data-icon-toggle-rest=\"{}\">",
        rest.label()
    );
    for variant in variants {
        for selection in icon_button::IconButtonSelection::TOGGLE {
            let a = icon_button::resolve_selection(
                theme,
                *variant,
                size,
                rest,
                icon_button::IconButtonWidth::Default,
                selection,
                InteractionState::Enabled,
            );
            let paint_shape = icon_button::resting_shape(rest, selection);
            out.push_str(&paint_icon_btn(
                &a,
                &format!(
                    "data-icon-toggle=\"{sel}\" data-icon-toggle-variant=\"{v}\" data-icon-toggle-shape=\"{sh}\" data-icon-toggle-r=\"{r}\"",
                    sel = selection.label(),
                    v = variant.label(),
                    sh = paint_shape.label(),
                    r = a.corners.top_left,
                ),
                size.pressed_corner_dp(),
                icon_button::icon_dp(size) * 0.75,
                selection.glyph(),
            ));
        }
    }
    out.push_str("</div>");
    out
}

fn icon_buttons(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Icon buttons</h2><p class=\"note\">Expressive: filled / tonal / outlined / standard, XS–XL, round/square, press morph, narrow/default/wide, toggle selected round↔square. Default S is 40×24. <a href=\"https://m3.material.io/components/icon-buttons/specs\">spec</a></p>",
    );
    out.push_str("<div class=\"hero-card\" data-hero=\"icon-buttons\"><div class=\"state-body\">");
    for variant in icon_button::IconButtonVariant::ALL {
        let a = icon_button::resolve(theme, variant, InteractionState::Enabled);
        out.push_str(&paint_icon_btn(
            &a,
            &format!("data-icon-button=\"{}\"", variant.label()),
            8.0,
            18.0,
            "★",
        ));
    }
    out.push_str("</div><h3>sizes</h3><div class=\"state-body\">");
    for size in button::ButtonSize::ALL {
        let a = icon_button::resolve_expressive(
            theme,
            icon_button::IconButtonVariant::Filled,
            size,
            button::ButtonShape::Round,
            InteractionState::Enabled,
        );
        out.push_str(&paint_icon_btn(
            &a,
            &format!("data-icon-size=\"{}\"", size.label()),
            size.pressed_corner_dp(),
            icon_button::icon_dp(size) * 0.75,
            "★",
        ));
    }
    out.push_str("</div></div>");
    out.push_str("<h3>widths</h3><p class=\"note\">MDC leading/trailing: S 4/8/14 · M 12/16/24. Narrow S is 32×40; wide S is 52×40. Extra-small and small keep a 48dp target.</p>");
    out.push_str("<div class=\"hero-card\" data-hero=\"icon-buttons-width\">");
    out.push_str(&paint_icon_width_row(theme, icon_button::WIDTH_HERO_SIZE));
    out.push_str(&paint_icon_width_row(
        theme,
        icon_button::WIDTH_HERO_SIZE_MEDIUM,
    ));
    out.push_str("</div>");
    out.push_str("<h3>toggle</h3><p class=\"note\">Compose <code>IconToggleButton</code>: unselected outlined glyph, selected filled glyph. Rest round morphs to square when selected (and the reverse). Colors: filled surface-container→primary; tonal secondary-container→secondary; outlined outline→inverse-surface; standard on-surface-variant→primary.</p>");
    out.push_str("<div class=\"hero-card\" data-hero=\"icon-buttons-toggle\">");
    out.push_str(&paint_icon_toggle_row(
        theme,
        button::ButtonShape::Round,
        &icon_button::IconButtonVariant::TOGGLE_OVERVIEW,
    ));
    out.push_str(&paint_icon_toggle_row(
        theme,
        button::ButtonShape::Square,
        &[icon_button::IconButtonVariant::Filled],
    ));
    out.push_str("</div>");
    for variant in icon_button::IconButtonVariant::ALL {
        out.push_str(&format!("<h3>{}</h3>", variant.label()));
        for state in [
            InteractionState::Enabled,
            InteractionState::Disabled,
            InteractionState::Pressed,
        ] {
            let a = icon_button::resolve(theme, variant, state);
            out.push_str(&state_row_open(state.label()));
            out.push_str(&format!(
                "<div class=\"icon-btn\" data-icon-button=\"{v}\" data-state=\"{s}\" style=\"--press-r:8px;background:{bg};color:{fg};border:{bd};border-radius:{r}px\">★</div>",
                v = variant.label(),
                s = state.label(),
                bg = a.container.css_hex(),
                fg = a.content.css_hex(),
                bd = a.outline_css(),
                r = a.corners.top_left,
            ));
            out.push_str("</div></div>");
        }
    }
    out
}

fn fabs(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>FAB</h2><p class=\"note\">Expressive: regular 56 / medium 80 / large 96 / small-extended. Baseline 40dp small FAB is deprecated.</p><div class=\"hero-card\" data-hero=\"fab\"><div class=\"state-body\">",
    );
    for variant in fab::FabVariant::ALL {
        let a = fab::resolve(theme, variant, InteractionState::Enabled);
        out.push_str(&format!(
            "<div class=\"fab\" data-fab=\"{v}\" style=\"width:{w}px;height:{h}px;background:{bg};color:{fg};box-shadow:{sh};border-radius:{r}px\">+</div>",
            v = variant.label(),
            w = a.width_dp.unwrap_or(a.height_dp),
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            sh = ElevationLevels::css_shadow(a.elevation_dp),
            r = a.corners.top_left,
        ));
    }
    out.push_str("</div><h3>sizes</h3><div class=\"state-body\">");
    for size in fab::FabSize::ALL {
        let a = fab::resolve_size(
            theme,
            fab::FabVariant::Primary,
            size,
            InteractionState::Enabled,
        );
        let label = if matches!(size, fab::FabSize::Extended) {
            "+ Create"
        } else {
            "+"
        };
        let w = a
            .width_dp
            .map(|w| format!("width:{w}px;"))
            .unwrap_or_else(|| {
                format!(
                    "min-width:{}px;padding:0 16px;",
                    a.min_width_dp.unwrap_or(80.0)
                )
            });
        out.push_str(&format!(
            "<div class=\"fab\" data-fab-size=\"{s}\" style=\"{w}height:{h}px;background:{bg};color:{fg};box-shadow:{sh};border-radius:{r}px;font-size:{fs}px\">{label}</div>",
            s = size.label(),
            w = w,
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            sh = ElevationLevels::css_shadow(a.elevation_dp),
            r = a.corners.top_left,
            fs = if matches!(size, fab::FabSize::Large) { 28 } else { 18 },
        ));
    }
    out.push_str("</div></div>");
    out
}

fn paint_fab_menu(theme: &Theme, color: fab_menu::FabMenuColor, expanded: bool) -> String {
    let item = fab_menu::resolve_item(theme, color);
    let close = fab_menu::resolve_close(theme, color, true);
    let closed = fab_menu::resolve_close(theme, color, false);
    let trigger = if expanded { &close } else { &closed };
    let mut items = String::new();
    for (icon, label) in fab_menu::DEMO_ITEMS {
        items.push_str(&format!(
            r#"<div class="fab-item" data-fab-item="{label}" style="background:{bg};color:{fg};box-shadow:{sh};font-size:{fs}px"><span>{icon}</span><span>{label}</span></div>"#,
            bg = item.container.css_hex(),
            fg = item.content.css_hex(),
            sh = ElevationLevels::css_shadow(item.elevation_dp),
            fs = item.label_style.size_sp,
        ));
    }
    format!(
        r#"<div class="fab-menu" data-fab-menu="{color}" data-hero="fab-menu" data-expanded="{exp}" data-open-glyph="{og}" data-close-glyph="{cg}" data-open-r="{or}" data-closed-r="{cr}" data-open-bg="{obg}" data-closed-bg="{cbg}" data-open-fg="{ofg}" data-closed-fg="{cfg}">
  {items}
  <div class="fab" data-fab-close="1" style="width:{sz}px;height:{sz}px;background:{bg};color:{fg};box-shadow:{sh};border-radius:{r}px;font-size:{fs}px">{glyph}</div>
</div>"#,
        color = color.label(),
        exp = if expanded { "1" } else { "0" },
        og = fab_menu::OPEN_GLYPH,
        cg = fab_menu::CLOSE_GLYPH,
        or = close.corners.top_left,
        cr = closed.corners.top_left,
        obg = close.container.css_hex(),
        cbg = closed.container.css_hex(),
        ofg = close.content.css_hex(),
        cfg = closed.content.css_hex(),
        items = items,
        sz = trigger.size_dp,
        bg = trigger.container.css_hex(),
        fg = trigger.content.css_hex(),
        sh = ElevationLevels::css_shadow(trigger.elevation_dp),
        r = trigger.corners.top_left,
        fs = trigger.icon_dp,
        glyph = trigger.glyph,
    )
}

fn fab_menu_section(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>FAB menu</h2><p class=\"note\">M3 Expressive: 2–6 related actions from a FAB. One 56dp menu size; close FAB morphs full-round to the solid set color. Official overview sits over a woven-basket photo. <a href=\"https://m3.material.io/components/fab-menu/specs\">spec</a></p><div class=\"hero-card\" data-hero=\"fab-menu\">",
    );
    out.push_str(&format!(
        r#"<div class="fab-scene" data-fab-scene="1" data-hero="fab-menu">
  {photo}
  {menu}
</div>"#,
        photo = paint_photo_class(
            fab_menu::SCENE_PHOTO,
            "height:360px;width:100%;",
            "photo-stub photo-hero",
        ),
        menu = paint_fab_menu(
            theme,
            fab_menu::FabMenuColor::Primary,
            fab_menu::DEMO_EXPANDED,
        ),
    ));
    out.push_str(
        "<h3>color sets</h3><div class=\"state-body\" style=\"align-items:flex-end;gap:32px\">",
    );
    out.push_str(&paint_fab_menu(
        theme,
        fab_menu::FabMenuColor::Secondary,
        false,
    ));
    out.push_str(&paint_fab_menu(
        theme,
        fab_menu::FabMenuColor::Tertiary,
        false,
    ));
    out.push_str("</div></div>");
    out
}

fn paint_split(
    theme: &Theme,
    variant: split_button::SplitButtonVariant,
    size: button::ButtonSize,
    open: bool,
) -> String {
    let lead = split_button::resolve_leading(theme, variant, size, false);
    let trail = split_button::resolve_trailing(theme, variant, size, open);
    let items = paint_grouped_overflow(
        theme,
        menu::GroupedPopupKind::Split,
        true,
        "split-menu overflow-cascade",
        r#" data-split-menu="1" data-split-cascade="1""#,
    );
    format!(
        r#"<div class="split" data-split="{v}" data-split-size="{s}" data-open="{open}" data-hero="split-button">
  <button class="split-lead" data-split-lead="1" style="background:{lbg};color:{lfg};border:{lbd};border-radius:{lr};height:{lh}px;padding:0 {lpad}px;font-size:{lfs}px;box-shadow:{lsh}">{icon} {label}</button>
  <button class="split-trail" data-split-trail="1" style="background:{tbg};color:{tfg};border:{tbd};border-radius:{tr};height:{th}px;min-width:{tw}px;padding:0 {tpad}px;font-size:{tfs}px;box-shadow:{tsh}">{caret}</button>
  {items}
</div>"#,
        v = variant.label(),
        s = size.label(),
        open = if open { "1" } else { "0" },
        lbg = lead.container.css_hex(),
        lfg = lead.content.css_hex(),
        lbd = lead.outline_css(),
        lr = lead.corners.css(),
        lh = lead.height_dp,
        lpad = lead.pad_start_dp,
        lfs = lead.label_style.size_sp,
        lsh = ElevationLevels::css_shadow(lead.elevation_dp),
        icon = split_button::DEMO_LEADING_ICON,
        label = split_button::DEMO_LABEL,
        tbg = trail.container.css_hex(),
        tfg = trail.content.css_hex(),
        tbd = trail.outline_css(),
        tr = trail.corners.css(),
        th = trail.height_dp,
        tw = trail
            .min_width_dp
            .unwrap_or(split_button::TRAILING_MIN_W_DP),
        tpad = trail.pad_start_dp,
        tfs = split_button::trailing_icon_dp(size),
        tsh = ElevationLevels::css_shadow(trail.elevation_dp),
        caret = split_button::caret(open),
        items = items,
    )
}

fn split_button_section(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Split button</h2><p class=\"note\">M3 Expressive: leading action + trailing menu, 2dp gap, outer full-round, inner 4dp rest / 12dp press (S). Trailing menu is a grouped 2dp popup (Add to cart / Save for later + More ›). Official overview is an enamel-mugs product card. <a href=\"https://m3.material.io/components/split-button/specs\">spec</a></p><div class=\"hero-card\" data-hero=\"split-button\">",
    );
    out.push_str(&format!(
        r#"<div class="product-card" data-split-scene="1" data-hero="split-button">
  {photo}
  <div class="copy">
    <div class="h">{title}</div>
    <div class="s">{sub}</div>
    {split}
  </div>
</div>"#,
        photo = paint_photo_class(
            split_button::SCENE_PHOTO,
            "height:200px;width:100%;",
            "photo-stub photo-hero",
        ),
        title = split_button::SCENE_TITLE,
        sub = split_button::SCENE_SUBTITLE,
        split = paint_split(
            theme,
            split_button::SplitButtonVariant::Filled,
            button::ButtonSize::Small,
            false,
        ),
    ));
    out.push_str("<h3>variants</h3><div class=\"state-body\" style=\"align-items:flex-start\">");
    out.push_str(&paint_split(
        theme,
        split_button::SplitButtonVariant::Filled,
        button::ButtonSize::Small,
        true,
    ));
    for variant in [
        split_button::SplitButtonVariant::Tonal,
        split_button::SplitButtonVariant::Elevated,
        split_button::SplitButtonVariant::Outlined,
    ] {
        out.push_str(&paint_split(
            theme,
            variant,
            button::ButtonSize::Small,
            false,
        ));
    }
    out.push_str("</div><h3>sizes</h3><div class=\"state-body\">");
    for size in button::ButtonSize::ALL {
        out.push_str(&paint_split(
            theme,
            split_button::SplitButtonVariant::Filled,
            size,
            false,
        ));
    }
    out.push_str("</div></div>");
    out
}

fn paint_toolbar(
    theme: &Theme,
    kind: toolbar::ToolbarKind,
    color: toolbar::ToolbarColor,
    axis: toolbar::ToolbarAxis,
    with_fab: bool,
) -> String {
    let a = toolbar::resolve(theme, kind, color, axis);
    let icon = toolbar::resolve_icon(theme, color);
    let mut icons = String::new();
    for glyph in toolbar::DEMO_ICONS {
        icons.push_str(&format!(
            r#"<div class="icon-btn" data-toolbar-icon="{glyph}" style="width:{w}px;height:{h}px;background:{bg};color:{fg};border-radius:{r}px">{glyph}</div>"#,
            w = icon.width_dp.unwrap_or(icon.height_dp),
            h = icon.height_dp,
            bg = icon.container.css_hex(),
            fg = icon.content.css_hex(),
            r = icon.corners.top_left,
        ));
    }
    let bar = format!(
        r#"<div class="toolbar" data-toolbar="{kind}" data-toolbar-color="{color}" data-axis="{axis}" style="background:{bg};color:{fg};border-radius:{r};height:{h};padding:{pad};gap:{gap}px;box-shadow:{sh};flex-direction:{dir}">{icons}</div>"#,
        kind = kind.label(),
        color = color.label(),
        axis = axis.label(),
        bg = a.container.css_hex(),
        fg = a.icon.css_hex(),
        r = a.corners.css(),
        h = if axis == toolbar::ToolbarAxis::Vertical {
            "auto".into()
        } else if kind == toolbar::ToolbarKind::Docked {
            format!("{}px;width:100%;max-width:360px", a.height_dp)
        } else {
            format!("{}px", a.height_dp)
        },
        pad = if axis == toolbar::ToolbarAxis::Vertical {
            format!("8px 0")
        } else {
            format!("0 {}px", a.pad_h_dp)
        },
        gap = a.item_gap_dp,
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        dir = if axis == toolbar::ToolbarAxis::Vertical {
            "column"
        } else {
            "row"
        },
        icons = icons,
    );
    if !with_fab {
        return bar;
    }
    let fab = toolbar::resolve_fab(theme, color);
    format!(
        r#"<div class="toolbar-fab" data-toolbar-fab="1" data-hero="toolbar">
  {bar}
  <div class="fab" style="width:{w}px;height:{h}px;background:{bg};color:{fg};border-radius:{r}px;box-shadow:{sh}">{glyph}</div>
</div>"#,
        bar = bar,
        w = fab.width_dp.unwrap_or(fab.height_dp),
        h = fab.height_dp,
        bg = fab.container.css_hex(),
        fg = fab.content.css_hex(),
        r = fab.corners.top_left,
        sh = ElevationLevels::css_shadow(fab.elevation_dp),
        glyph = toolbar::DEMO_FAB,
    )
}

fn toolbar_section(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Toolbars</h2><p class=\"note\">M3 Expressive: floating 64dp full-round over content, or docked full-width. Pair a FAB for the highest-priority action. Vibrant paired FAB uses tertiary (pink) like the official chat overview. <a href=\"https://m3.material.io/components/toolbars/specs\">spec</a></p><div class=\"hero-card\" data-hero=\"toolbar\">",
    );
    let bubbles: String = toolbar::SCENE_BUBBLES
        .iter()
        .map(|b| format!(r#"<div class="bubble">{b}</div>"#))
        .collect();
    out.push_str(&format!(
        r#"<div class="chat-scene" data-toolbar-scene="1" data-hero="toolbar" style="min-height:{ph}px">
  <div class="chat-head">
    {avatar}
    <div class="chat-meta"><span>{from}</span><span style="font-size:12px;opacity:.7">{time}</span></div>
    <span>{star}</span>
  </div>
  {bubbles}
  {dog}
  {bar}
</div>"#,
        ph = toolbar::PHONE_H_DP,
        avatar = paint_avatar(toolbar::SCENE_AVATAR, 40.0),
        from = toolbar::SCENE_FROM,
        time = toolbar::SCENE_TIME,
        star = toolbar::SCENE_STAR,
        bubbles = bubbles,
        dog = paint_photo_class(
            toolbar::SCENE_PHOTO,
            "height:140px;margin:8px 16px;border-radius:16px;",
            "photo-stub dog",
        ),
        bar = paint_toolbar(
            theme,
            toolbar::ToolbarKind::Floating,
            toolbar::ToolbarColor::Vibrant,
            toolbar::ToolbarAxis::Horizontal,
            true,
        ),
    ));
    out.push_str("<h3>standard + vertical</h3><div class=\"state-body\">");
    out.push_str(&paint_toolbar(
        theme,
        toolbar::ToolbarKind::Floating,
        toolbar::ToolbarColor::Standard,
        toolbar::ToolbarAxis::Horizontal,
        false,
    ));
    out.push_str(&paint_toolbar(
        theme,
        toolbar::ToolbarKind::Floating,
        toolbar::ToolbarColor::Standard,
        toolbar::ToolbarAxis::Vertical,
        true,
    ));
    out.push_str("</div><h3>docked</h3>");
    out.push_str(&paint_toolbar(
        theme,
        toolbar::ToolbarKind::Docked,
        toolbar::ToolbarColor::Standard,
        toolbar::ToolbarAxis::Horizontal,
        false,
    ));
    out.push_str("</div>");
    out
}

fn paint_filled_field(
    a: &text_field::TextFieldAppearance,
    attrs: &str,
    label: &str,
    value_html: &str,
) -> String {
    let (oc, ow) = a
        .field
        .outline
        .map(|(c, w)| (c.css_hex(), w))
        .unwrap_or_else(|| ("transparent".into(), 0.0));
    let shadow = if a.shows_indicator() {
        format!("inset 0 -{ow}px 0 {oc}")
    } else {
        "none".into()
    };
    let empty = if a.floating { "" } else { " empty" };
    let style_attrs = a.catalog_style_attrs();
    format!(
        r#"<div class="filled-hero{empty}" {attrs} {style_attrs} data-floating="{float}" style="background:{bg};border-radius:{rad};box-shadow:{shadow};color:{fg}">
  <div class="lab" style="color:{lab};font-size:{ls}px;line-height:{lh}px">{label}</div>
  {value}
</div>"#,
        empty = empty,
        style_attrs = style_attrs,
        float = if a.floating { "1" } else { "0" },
        bg = a.field.container.css_hex(),
        rad = a.field.corners.css(),
        shadow = shadow,
        fg = if a.floating {
            a.input.css_hex()
        } else {
            a.label.css_hex()
        },
        lab = a.label.css_hex(),
        ls = a.label_style.size_sp,
        lh = a.label_style.line_height_sp,
        value = if a.floating { value_html } else { "" },
    )
}

fn paint_outlined_field(
    a: &text_field::TextFieldAppearance,
    attrs: &str,
    label: &str,
    inner_html: &str,
) -> String {
    let (oc, ow) = a
        .field
        .outline
        .map(|(c, w)| (c.css_hex(), w))
        .unwrap_or_else(|| ("transparent".into(), 1.0));
    let style_attrs = a.catalog_style_attrs();
    if a.notched {
        let frame = text_field::notch_frame(label, a);
        let d = frame.outline_svg_d(280.0);
        let even = frame.evenodd_svg_d(280.0);
        format!(
            r#"<fieldset class="ol" data-notched="1" data-notch="cutout" data-notch-hole="1" data-notch-evenodd="1" data-notch-cpath="1" data-notch-rounded-polygon="1" data-notch-centerline="1" data-notch-path="{d}" data-stroke="{ow}" {attrs} {style_attrs} style="border:none;position:relative;border-radius:{r}px;color:{inp}">
  <svg class="ol-evenodd" viewBox="0 0 280 56" preserveAspectRatio="none" aria-hidden="true"><path data-notch-evenodd-path="1" fill-rule="evenodd" fill="{oc}" d="{even}"/><path data-notch-centerline-path="1" fill="none" stroke="{oc}" stroke-width="{ow}" stroke-linecap="round" d="{center}"/></svg>
  <legend style="color:{lab};padding:0 {pad}px">{label}</legend>
  {inner_html}
</fieldset>"#,
            r = a.field.corners.top_left,
            inp = a.input.css_hex(),
            lab = a.label.css_hex(),
            pad = text_field::NOTCH_PAD_DP,
            d = d,
            center = frame.centerline_svg_d(280.0),
        )
    } else {
        let even = text_field::notch_frame(label, a).evenodd_svg_d(280.0);
        format!(
            r#"<div class="ol" data-notched="0" data-floating="{float}" data-evenodd-d="{even}" {attrs} {style_attrs} style="border:{ow}px solid {oc};border-radius:{r}px;background:{bg};color:{fg};min-height:56px">
  <span class="lab" style="color:{lab};font-size:{ls}px;line-height:{lh}px">{label}</span>
  {inner_html}
</div>"#,
            float = if a.floating { "1" } else { "0" },
            r = a.field.corners.top_left,
            bg = a.field.container.css_hex(),
            fg = if a.floating {
                a.input.css_hex()
            } else {
                a.label.css_hex()
            },
            lab = a.label.css_hex(),
            ls = a.label_style.size_sp,
            lh = a.label_style.line_height_sp,
        )
    }
}

fn text_fields(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Text fields</h2><p class=\"note\">Expressive (recommended): Compose <code>roundedShape</code> CornerMedium 12 + <code>tonalColors()</code> + <code>TextFieldLabelPosition.Inside</code>. Filled tonal is SurfaceContainer with no indicator; outlined tonal is OnPrimary + OutlineVariant. Baseline extra-small + Cutout notch still available. <a href=\"https://m3.material.io/components/text-fields/specs\">spec</a></p>",
    );
    let empty_filled = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        false,
    );
    let empty_outlined = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Enabled,
        false,
    );
    let filled = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Focused,
        true,
    );
    let outlined = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    let hero_outlined_inner = format!(
        r#"<span style="color:{lead};font-size:20px">⌕</span>
  <input class="val" value="Input" data-editor="outlined"/>
  <span style="color:{trail};font-size:18px">✕</span>"#,
        lead = outlined.leading_icon.css_hex(),
        trail = outlined.trailing_icon.css_hex(),
    );
    out.push_str("<div class=\"hero-card\" data-hero=\"text-fields\">");
    out.push_str(&paint_filled_field(
        &empty_filled,
        "data-field-hero=\"empty-filled\"",
        "Label",
        "",
    ));
    out.push_str(&paint_outlined_field(
        &empty_outlined,
        "data-field-hero=\"empty-outlined\"",
        "Label",
        "",
    ));
    out.push_str(&paint_outlined_field(
        &outlined,
        "data-field-hero=\"outlined\"",
        "Email",
        &hero_outlined_inner,
    ));
    out.push_str(&paint_filled_field(
        &filled,
        "data-field-hero=\"filled\"",
        "Label",
        &format!(
            r#"<input class="val" style="color:{}" value="Input text" data-editor="filled"/>"#,
            filled.input.css_hex()
        ),
    ));
    out.push_str("<p class=\"note\">Empty pair + populated Email use Expressive Inside (no Cutout). System IME remains a NativeActivity stub.</p></div>");

    let filled_edit = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        true,
    );
    let outlined_empty = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Enabled,
        false,
    );
    out.push_str("<h3>editable</h3><div class=\"state-body\">");
    out.push_str(&format!(
        r#"<div class="field-wrap" data-editor="filled">{}<div class="support" style="color:{}">Supporting text</div></div>"#,
        paint_filled_field(
            &filled_edit,
            "data-field=\"filled-edit\"",
            "Label",
            &format!(
                r#"<input class="val" style="color:{}" value="Editable filled"/>"#,
                filled_edit.input.css_hex()
            ),
        ),
        filled_edit.supporting.css_hex()
    ));
    out.push_str(&format!(
        r#"<div class="field-wrap" data-editor="outlined">{}<div class="support" style="color:{}">Supporting text</div></div>"#,
        paint_outlined_field(
            &outlined_empty,
            "data-field=\"outlined-edit\"",
            "Email",
            r#"<input class="val" value="" placeholder="you@domain"/>"#,
        ),
        outlined_empty.supporting.css_hex()
    ));
    out.push_str("</div>");

    let states = [
        InteractionState::Enabled,
        InteractionState::Disabled,
        InteractionState::Hovered,
        InteractionState::Focused,
        InteractionState::Error,
        InteractionState::ErrorFocused,
    ];
    for variant in text_field::TextFieldVariant::ALL {
        out.push_str(&format!("<h3>{}</h3>", variant.label()));
        for state in states {
            let a = text_field::resolve(theme, variant, state, true);
            let support = if state.is_error() {
                "Enter a valid value"
            } else {
                "Supporting text"
            };
            let value = format!(
                r#"<div class="val" style="color:{}">Input text</div>"#,
                a.input.css_hex()
            );
            let attrs = format!(
                "data-field=\"{}\" data-state=\"{}\"",
                variant.label(),
                state.label()
            );
            out.push_str(&state_row_open(state.label()));
            out.push_str("<div class=\"field-wrap\">");
            match variant {
                text_field::TextFieldVariant::Filled => {
                    out.push_str(&paint_filled_field(&a, &attrs, "Label", &value));
                }
                text_field::TextFieldVariant::Outlined => {
                    out.push_str(&paint_outlined_field(&a, &attrs, "Label", &value));
                }
            }
            out.push_str(&format!(
                r#"<div class="support" style="color:{}">{support}</div></div>"#,
                a.supporting.css_hex()
            ));
            out.push_str("</div></div>");
        }
    }
    out
}

fn selection(theme: &Theme) -> String {
    let mut out = String::from("<h2>Checkbox, radio, switch</h2>");
    out.push_str("<h3>Checkbox</h3>");
    for value in checkbox::CheckValue::ALL {
        for state in [
            InteractionState::Enabled,
            InteractionState::Disabled,
            InteractionState::Pressed,
        ] {
            let a = checkbox::resolve(theme, value, state);
            let mark = match value {
                checkbox::CheckValue::Checked => "✓",
                checkbox::CheckValue::Indeterminate => "–",
                checkbox::CheckValue::Unchecked => "",
            };
            let border = a
                .box_outline
                .map(|o| format!("2px solid {}", o.css_hex()))
                .unwrap_or_else(|| "none".into());
            out.push_str(&state_row_open(&format!(
                "{} / {}",
                value.label(),
                state.label()
            )));
            out.push_str(&format!(
                "<div class=\"check\" data-checkbox=\"{v}\" data-state=\"{s}\" style=\"background:{layer}\"><div class=\"box\" style=\"background:{fill};color:{icon};border:{border}\">{mark}</div></div>",
                v = value.label(),
                s = state.label(),
                layer = a.state_layer.css_hex(),
                fill = a.box_fill.css_hex(),
                icon = a.icon.css_hex(),
            ));
            out.push_str("</div></div>");
        }
    }
    out.push_str("<h3>Radio</h3>");
    for selected in [false, true] {
        let a = radio::resolve(theme, selected, InteractionState::Enabled);
        let inner = a
            .inner
            .map(|i| format!("<i style=\"background:{}\"></i>", i.css_hex()))
            .unwrap_or_default();
        out.push_str(&format!(
            "<div class=\"radio\" data-radio=\"{sel}\"><div class=\"dot\" style=\"border:2px solid {ring}\">{inner}</div></div>",
            sel = selected,
            ring = a.ring.css_hex(),
        ));
    }
    out.push_str("<h3>Switch</h3>");
    for selected in [false, true] {
        for state in [InteractionState::Enabled, InteractionState::Disabled] {
            let a = switch::resolve(theme, selected, state);
            let left = if selected { 24.0 } else { 8.0 };
            let outline = a
                .track_outline
                .map(|o| format!("2px solid {}", o.css_hex()))
                .unwrap_or_else(|| "none".into());
            out.push_str(&state_row_open(&format!(
                "{} / {}",
                if selected { "selected" } else { "unselected" },
                state.label()
            )));
            out.push_str(&format!(
                "<div class=\"switch\" data-switch=\"{sel}\" data-state=\"{s}\" style=\"background:{track};border:{outline}\"><b style=\"width:{th}px;height:{th}px;left:{left}px;background:{thumb}\"></b></div>",
                sel = selected,
                s = state.label(),
                track = a.track.css_hex(),
                th = a.thumb_dp,
                thumb = a.thumb.css_hex(),
            ));
            out.push_str("</div></div>");
        }
    }
    out
}

fn lists(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Lists</h2><p class=\"note\">Expressive segmented lists (recommended): 2dp gap, 4dp inner / 16dp outer, selected 16dp + secondary-container. Swipe Archive/Delete rails with LazyColumn fling (Closed / Open / primary action, 16dp overshoot, growing reveal). Drag-handle reorder. Baseline 56/72/88 still available. <a href=\"https://m3.material.io/components/lists/specs\">spec</a></p>",
    );
    out.push_str(&format!(
        r#"<div class="list-group" data-hero="list" data-list-style="segmented" data-list-gap="{gap}">"#,
        gap = list::SEGMENTED_GAP_DP,
    ));
    for i in 0..list::SCENE_COUNT {
        let selected = i == list::SCENE_SELECTED;
        let a = list::resolve_scene(theme, i, list::SCENE_SELECTED);
        let idle = list::resolve_segmented(
            theme,
            list::ListLines::Two,
            i,
            list::SCENE_COUNT,
            false,
            InteractionState::Enabled,
        );
        let on = list::resolve_segmented(
            theme,
            list::ListLines::Two,
            i,
            list::SCENE_COUNT,
            true,
            InteractionState::Enabled,
        );
        let sw = switch::resolve(theme, list::SCENE_TRAILING_ON[i], InteractionState::Enabled);
        let left = if list::SCENE_TRAILING_ON[i] {
            24.0
        } else {
            8.0
        };
        let outline = sw
            .track_outline
            .map(|o| format!("2px solid {}", o.css_hex()))
            .unwrap_or_else(|| "none".into());
        out.push_str(&format!(
            r#"<div class="list-item segmented" data-list="segmented" data-list-item="{key}" data-list-selected="{sel}" data-idle-bg="{ibg}" data-idle-fg="{ifg}" data-idle-r="{ir}" data-on-bg="{obg}" data-on-fg="{ofg}" data-on-r="{orad}" style="height:{h}px;background:{bg};color:{fg};border-radius:{r};padding:{pt}px {ph}px">
  <div class="lead">{icon}</div>
  <div class="meta"><div class="h">{head}</div><div class="s" style="color:{sfg}">{sub}</div></div>
  <div class="trail"><div class="switch" data-switch="{on}" style="background:{track};border:{outline}"><b style="width:{th}px;height:{th}px;left:{left}px;background:{thumb}"></b></div></div>
</div>"#,
            key = list::SCENE_KEYS[i],
            sel = selected as u8,
            ibg = idle.container.css_hex(),
            ifg = idle.content.css_hex(),
            ir = idle.corners.css(),
            obg = on.container.css_hex(),
            ofg = on.content.css_hex(),
            orad = on.corners.css(),
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            r = a.corners.css(),
            pt = a.pad_top_dp,
            ph = a.pad_start_dp,
            icon = list::SCENE_ICONS[i],
            head = list::SCENE_HEADLINES[i],
            sfg = a.secondary_content.unwrap().css_hex(),
            sub = list::SCENE_SUPPORTING[i],
            on = list::SCENE_TRAILING_ON[i],
            track = sw.track.css_hex(),
            outline = outline,
            th = sw.thumb_dp,
            left = left,
            thumb = sw.thumb.css_hex(),
        ));
    }
    out.push_str("</div>");
    out.push_str(&paint_list_swipe(theme));
    out.push_str(&paint_list_reorder(theme));
    for lines in list::ListLines::ALL {
        let a = list::resolve(theme, lines, InteractionState::Enabled);
        let support = if matches!(lines, list::ListLines::One) {
            String::new()
        } else {
            format!(
                "<div class=\"s\" style=\"color:{}\">Supporting text</div>",
                a.secondary_content.unwrap().css_hex()
            )
        };
        out.push_str(&format!(
            "<div class=\"list-item\" data-list=\"{l}\" style=\"height:{h}px;background:{bg}\"><div class=\"h\" style=\"color:{fg}\">{title}</div>{support}</div>",
            l = lines.label(),
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            title = lines.label(),
        ));
        let d = divider::resolve(theme, true);
        out.push_str(&format!(
            "<div class=\"divider\" data-divider=\"inset\" style=\"height:{}px;background:{};margin-left:{}px\"></div>",
            d.thickness_dp,
            d.color.css_hex(),
            d.inset_dp
        ));
    }
    let pressed = list::resolve(theme, list::ListLines::One, InteractionState::Pressed);
    out.push_str(&format!(
        "<div class=\"list-item\" data-list=\"pressed\" style=\"height:{}px;background:{}\"><div class=\"h\" style=\"color:{}\">pressed one-line</div></div>",
        pressed.height_dp,
        pressed.container.css_hex(),
        pressed.content.css_hex()
    ));
    out
}

fn paint_list_swipe(theme: &Theme) -> String {
    let lead_bg = list::leading_action_container(theme).css_hex();
    let lead_fg = list::leading_action_content(theme).css_hex();
    let trail_bg = list::trailing_action_container(theme).css_hex();
    let trail_fg = list::trailing_action_content(theme).css_hex();
    let mut items = String::new();
    for i in 0..list::SWIPE_COUNT {
        let a = list::resolve_swipe_item(theme, i, list::SWIPE_COUNT);
        items.push_str(&format!(
            r#"<div class="list-item" data-list="swipe" data-list-item="{key}" style="height:{h}px;background:{bg};color:{fg};padding:{pt}px {ph}px">
  <div class="h">{head}</div>
  <div class="s" style="color:{sfg}">{sub}</div>
</div>"#,
            key = list::SWIPE_KEYS[i],
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            pt = a.pad_top_dp,
            ph = a.pad_start_dp,
            head = list::SWIPE_HEADLINES[i],
            sfg = a.secondary_content.unwrap().css_hex(),
            sub = list::SWIPE_SUPPORTING[i],
        ));
    }
    format!(
        r#"<div class="list-swipe" data-hero="list-swipe" data-list-swipe="1" data-swipe-fling="1" data-swipe-reveal="{reveal}" data-swipe-threshold="{thresh}" data-swipe-offset="{off}" data-swipe-overshoot="{over}" data-swipe-primary-dp="{primary}" data-swipe-primary-threshold="{pthresh}" data-swipe-fling-decay="{decay}" data-swipe-fling-rest="{rest}" data-swipe-state="open" data-swipe-leading="1">
  <div class="rails">
    <div class="rail lead" data-swipe-action="archive" style="background:{lbg};color:{lfg}">{archive}</div>
    <div class="rail trail" data-swipe-action="delete" style="background:{tbg};color:{tfg}">{delete}</div>
  </div>
  <div class="sheet" style="transform:translateX({off}px)">{items}</div>
</div>"#,
        reveal = list::SWIPE_REVEAL_DP,
        thresh = list::SWIPE_THRESHOLD_DP,
        off = list::SWIPE_DEMO_OFFSET_DP,
        over = list::SWIPE_OVERSHOOT_DP,
        primary = list::SWIPE_PRIMARY_ACTION_DP,
        pthresh = list::SWIPE_PRIMARY_THRESHOLD_DP,
        decay = list::SWIPE_FLING_DECAY,
        rest = list::SWIPE_FLING_REST_DP,
        lbg = lead_bg,
        lfg = lead_fg,
        archive = list::SWIPE_LEADING_LABEL,
        tbg = trail_bg,
        tfg = trail_fg,
        delete = list::SWIPE_TRAILING_LABEL,
        items = items,
    )
}

fn paint_list_reorder(theme: &Theme) -> String {
    let mut out = format!(
        r#"<div class="list-group list-reorder" data-hero="list-reorder" data-list-reorder="1" data-list-style="segmented" data-list-gap="{gap}" data-drag-handle="{h}">"#,
        gap = list::SEGMENTED_GAP_DP,
        h = list::DRAG_HANDLE_DP,
    );
    for (pos, &id) in list::REORDER_DEMO.iter().enumerate() {
        let a = list::resolve_reorder_item(theme, pos, list::REORDER_COUNT, pos == 0);
        out.push_str(&format!(
            r#"<div class="list-item segmented" data-list="reorder" data-list-item="{key}" data-list-selected="{sel}" style="height:{h}px;background:{bg};color:{fg};border-radius:{r};padding:{pt}px {ph}px">
  <div class="meta"><div class="h">{head}</div><div class="s" style="color:{sfg}">{sub}</div></div>
  <div class="trail handle" data-list-handle="1">{glyph}</div>
</div>"#,
            key = list::REORDER_KEYS[id],
            sel = (pos == 0) as u8,
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            r = a.corners.css(),
            pt = a.pad_top_dp,
            ph = a.pad_start_dp,
            head = list::REORDER_HEADLINES[id],
            sfg = a.secondary_content.unwrap().css_hex(),
            sub = list::REORDER_SUPPORTING[id],
            glyph = list::DRAG_HANDLE_GLYPH,
        ));
    }
    out.push_str("</div>");
    out
}

fn paint_chip(theme: &Theme, demo: chip::ChipDemo, attrs: &str) -> String {
    let a = chip::resolve_demo(theme, demo);
    let idle = chip::resolve_style(
        theme,
        demo.variant,
        demo.color,
        false,
        InteractionState::Enabled,
        demo.leading,
        demo.avatar,
    );
    let sel_a = chip::resolve_style(
        theme,
        demo.variant,
        demo.color,
        true,
        InteractionState::Enabled,
        demo.leading,
        demo.avatar,
    );
    let ico = a.secondary_content.unwrap_or(a.content).css_hex();
    let lead = chip::demo_leading_icon(demo)
        .map(|g| {
            format!(r#"<span class="chip-ico" data-chip-lead="1" style="color:{ico}">{g}</span>"#)
        })
        .unwrap_or_default();
    let avatar = chip::demo_avatar(demo)
        .map(|kind| {
            format!(
                r#"<div class="chip-av" data-chip-avatar="1" data-chip-avatar-size="{size}" data-photo="{label}">{inner}</div>"#,
                size = chip::AVATAR_DP,
                label = kind.label(),
                inner = paint_avatar(kind, chip::AVATAR_DP),
            )
        })
        .unwrap_or_default();
    let trail = chip::trailing_icon(demo.variant)
        .map(|g| {
            format!(r#"<span class="chip-ico" data-chip-trail="1" style="color:{ico}">{g}</span>"#)
        })
        .unwrap_or_default();
    let rest_r = chip::rest_corner_dp(theme, demo.variant, false);
    let sel_r = chip::rest_corner_dp(theme, demo.variant, true);
    let press_r = chip::PRESSED_CORNER_DP;
    format!(
        r#"<div class="chip" data-chip="{v}" data-chip-label="{label}" data-chip-style="{style}" data-chip-elev="{elev}" data-selected="{sel}" data-chip-state="{st}" data-chip-r="{r}" data-chip-morph="{morph}" data-chip-press="{press}" data-chip-rest-r="{rest_r}" data-chip-sel-r="{sel_r}" data-chip-compact="{compact}" data-idle-bg="{ibg}" data-idle-fg="{ifg}" data-idle-bd="{ibd}" data-sel-bg="{sbg}" data-sel-fg="{sfg}" data-sel-bd="{sbd}" {attrs} style="--press-r:{press_r}px;background:{bg};color:{fg};border:{bd};border-radius:{rad};padding-left:{ps}px;padding-right:{pe}px;gap:{gap}px;box-shadow:{sh}">{avatar}{lead}{label}{trail}</div>"#,
        v = demo.variant.label(),
        style = demo.color.label(),
        elev = a.elevation_dp,
        label = demo.label,
        sel = if demo.selected { "1" } else { "0" },
        st = demo.state.label(),
        r = a.corners.top_left,
        morph = if demo.variant.morphs() { "1" } else { "0" },
        press = if demo.variant.morphs() { "1" } else { "0" },
        compact = if demo.has_avatar() { "1" } else { "0" },
        bg = a.container.css_hex(),
        fg = a.content.css_hex(),
        bd = a.outline_css(),
        rad = a.corners.css(),
        ps = a.pad_start_dp,
        pe = a.pad_end_dp,
        gap = chip::demo_icon_gap_dp(demo),
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        ibg = idle.container.css_hex(),
        ifg = idle.content.css_hex(),
        ibd = idle.outline_css(),
        sbg = sel_a.container.css_hex(),
        sfg = sel_a.content.css_hex(),
        sbd = sel_a.outline_css(),
    )
}

fn chips(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Chips</h2><p class=\"note\">Expressive FilterChip / InputChip morph Compose <code>ChipShapes</code>: CornerMedium 12 rest, CornerFull selected, CornerSmall 8 pressed. Live press interpolates those corners (<code>rememberAnimatedShape</code> / spatial-fast). Flat filter is outlined (outline-variant). Tonal leading icons use on-surface-variant (<code>ChipsTokens.UnselectedLeadingIconColor</code>). ElevatedFilterChip is surface-container-low · elev 1 · no outline (<code>tonalElevatedFilterChipColors</code>). Selected filter shows a leading check; input keeps a trailing close. InputChip avatar is 24dp + compact 4dp arrangement. Assist / suggestion stay 32dp full-round baseline. <a href=\"https://m3.material.io/components/chips/specs\">spec</a></p>",
    );
    out.push_str("<div class=\"hero-card\" data-hero=\"chips\">");
    out.push_str("<div class=\"state-body\" data-chip-row=\"filter\">");
    for demo in chip::FILTER_HERO {
        out.push_str(&paint_chip(theme, demo, r#"data-hero-chip="filter""#));
    }
    out.push_str("</div><div class=\"state-body\" data-chip-row=\"tonal\">");
    for demo in chip::TONAL_FILTER_HERO {
        out.push_str(&paint_chip(theme, demo, r#"data-hero-chip="tonal""#));
    }
    out.push_str("</div><div class=\"state-body\" data-chip-row=\"input\">");
    for demo in chip::INPUT_HERO {
        out.push_str(&paint_chip(theme, demo, r#"data-hero-chip="input""#));
    }
    out.push_str("</div><div class=\"state-body\" data-chip-row=\"input-avatar\">");
    for demo in chip::INPUT_AVATAR_HERO {
        out.push_str(&paint_chip(
            theme,
            demo,
            r#"data-hero-chip="input-avatar" data-chip-avatar-row="1""#,
        ));
    }
    out.push_str("</div></div>");
    out.push_str("<div class=\"hero-card\" data-hero=\"chips-elevated\">");
    out.push_str("<div class=\"state-body\" data-chip-row=\"elevated-hero\">");
    for demo in chip::ELEVATED_FILTER_HERO {
        out.push_str(&paint_chip(
            theme,
            demo,
            r#"data-hero-chip="elevated" data-chip-elevated="1""#,
        ));
    }
    out.push_str("</div></div>");
    out.push_str("<div class=\"state-body\">");
    for variant in chip::ChipVariant::ALL {
        for selected in [false, true] {
            if matches!(variant, chip::ChipVariant::Assist) && selected {
                continue;
            }
            let demo = chip::ChipDemo {
                variant,
                color: chip::ChipColor::Flat,
                label: if selected {
                    match variant {
                        chip::ChipVariant::Filter => "filter · selected",
                        chip::ChipVariant::Input => "input · selected",
                        chip::ChipVariant::Suggestion => "suggestion · selected",
                        chip::ChipVariant::Assist => "assist",
                    }
                } else {
                    variant.label()
                },
                selected,
                state: InteractionState::Enabled,
                leading: None,
                avatar: None,
            };
            out.push_str(&paint_chip(theme, demo, r#"data-chip-matrix="1""#));
        }
    }
    out.push_str("</div>");
    out
}

fn cards(theme: &Theme) -> String {
    let mut out = String::from("<h2>Cards</h2><div class=\"state-body\">");
    for variant in card::CardVariant::ALL {
        let a = card::resolve(theme, variant, InteractionState::Enabled);
        out.push_str(&format!(
            "<div class=\"card-demo\" data-card=\"{v}\" style=\"background:{bg};color:{fg};border:{bd};box-shadow:{sh}\"><strong>{}</strong><span style=\"color:{sec};font-size:14px\">16dp padding · 12dp corners</span></div>",
            variant.label(),
            v = variant.label(),
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            sec = a.secondary_content.unwrap().css_hex(),
            bd = a.outline_css(),
            sh = ElevationLevels::css_shadow(a.elevation_dp),
        ));
    }
    out.push_str("</div>");
    out
}

fn rail_dests_html(
    theme: &Theme,
    rail: &navigation_rail::NavRailAppearance,
    position: navigation_rail::IconPosition,
    selected: usize,
) -> String {
    let metrics = navigation_rail::item_metrics(theme, position);
    let mut out = String::new();
    for (i, ((label, icon), badge)) in navigation_rail::DESTINATIONS
        .iter()
        .zip(navigation_rail::DESTINATION_ICONS.iter())
        .zip(navigation_rail::DESTINATION_BADGES.iter())
        .enumerate()
    {
        let badge_html = match badge {
            Some(0) => format!(
                r#"<span class="dot small" style="background:{bg}"></span>"#,
                bg = rail.badge.css_hex()
            ),
            Some(n) => format!(
                r#"<span class="dot" style="background:{bg};color:{fg}">{n}</span>"#,
                bg = rail.badge.css_hex(),
                fg = rail.badge_label.css_hex(),
            ),
            None => String::new(),
        };
        let active = navigation_rail::is_active(selected, i);
        let fg = if active {
            rail.active_label
        } else {
            rail.inactive_label
        };
        let icon_fg = if active {
            rail.active_icon
        } else {
            rail.inactive_icon
        };
        out.push_str(&format!(
            r#"<div class="dest" data-icon-position="{pos}" data-active="{on}" style="color:{fg};--ind:{ind}"><div class="ind" style="color:{icon_fg}">{icon}{badge}</div><span class="lbl" style="font-size:{fs}px">{label}</span></div>"#,
            pos = position.label(),
            on = if active { "1" } else { "0" },
            fg = fg.css_hex(),
            ind = rail.active_indicator.css_hex(),
            icon_fg = icon_fg.css_hex(),
            fs = metrics.label_style.size_sp,
            badge = badge_html,
        ));
    }
    out
}

fn chrome(theme: &Theme) -> String {
    let snack = snackbar::resolve(theme);
    let nav = navigation_bar::resolve(theme);
    let rail = navigation_rail::resolve(theme);
    let rail_dests = rail_dests_html(theme, &rail, navigation_rail::IconPosition::Start, 0);
    let top_dests = rail_dests_html(theme, &rail, navigation_rail::IconPosition::Top, 0);
    let start_dests = rail_dests_html(theme, &rail, navigation_rail::IconPosition::Start, 0);
    let pad = navigation_rail::content_padding();
    let content_pad_attrs = format!(
        r#"data-content-padding="1" data-content-pad-v="{v}" data-content-pad-h="{h}""#,
        v = pad.top_dp,
        h = pad.start_dp,
    );
    let inflow_extended_fab = format!(
        r#"<div class="fab-slot" data-rail-fab="1" data-rail-fab-extend="1" data-inflow-fab-extend="1" style="background:{bg};color:{fg}"><span class="fab-icon">{glyph}</span><span class="fab-label" data-rail-fab-label="1">{label}</span></div>"#,
        bg = rail.fab.css_hex(),
        fg = rail.fab_icon.css_hex(),
        glyph = navigation_rail::FAB_GLYPH,
        label = navigation_rail::FAB_LABEL,
    );
    let modal_extended_fab = format!(
        r#"<div class="fab-slot" data-rail-fab="1" data-rail-fab-extend="1" data-modal-fab-extend="1" style="background:{bg};color:{fg}"><span class="fab-icon">{glyph}</span><span class="fab-label" data-rail-fab-label="1">{label}</span></div>"#,
        bg = rail.fab.css_hex(),
        fg = rail.fab_icon.css_hex(),
        glyph = navigation_rail::FAB_GLYPH,
        label = navigation_rail::FAB_LABEL,
    );
    let narrow_extended_fab = format!(
        r#"<div class="fab-slot" data-rail-fab="1" data-rail-fab-extend="1" data-narrow-fab-extend="1" style="background:{bg};color:{fg}"><span class="fab-icon">{glyph}</span><span class="fab-label" data-rail-fab-label="1">{label}</span></div>"#,
        bg = rail.fab.css_hex(),
        fg = rail.fab_icon.css_hex(),
        glyph = navigation_rail::FAB_GLYPH,
        label = navigation_rail::FAB_LABEL,
    );
    let hide_extended_fab = format!(
        r#"<div class="fab-slot" data-rail-fab="1" data-rail-fab-extend="1" data-hide-fab-extend="1" style="background:{bg};color:{fg}"><span class="fab-icon">{glyph}</span><span class="fab-label" data-rail-fab-label="1">{label}</span></div>"#,
        bg = rail.fab.css_hex(),
        fg = rail.fab_icon.css_hex(),
        glyph = navigation_rail::FAB_GLYPH,
        label = navigation_rail::FAB_LABEL,
    );
    let expanded = navigation_rail::resolve_mode(theme, navigation_rail::RailMode::Expanded);
    let mut mail_rows = String::new();
    for row in snackbar::MAIL_ROWS {
        let peek = if row.peek { "1" } else { "0" };
        mail_rows.push_str(&format!(
            r#"<div class="mail-row" data-mail-row="{from}" data-mail-avatar="1" data-mail-time="{time}" data-mail-peek="{peek}">
  {avatar}
  <span class="mail-meta"><span class="from">{from}</span><span class="subj" style="color:{sec}">{subj}</span></span>
  <span class="mail-time" style="color:{sec}">{time}</span>
</div>"#,
            from = row.from,
            time = row.time,
            avatar = paint_avatar(row.photo, snackbar::AVATAR_DP),
            subj = row.subject,
            sec = theme.color.on_surface_variant.css_hex(),
            peek = peek,
        ));
    }
    let nav_bar = navigation_bar::resolve(theme);
    let nav_h = navigation_bar::resolve_horizontal(theme);
    let mut horizontal = String::new();
    for (i, label) in navigation_bar::MEDIUM_DESTS.iter().enumerate() {
        let active = i == 0;
        let fg = if active {
            nav_h.active_label.css_hex()
        } else {
            nav_h.inactive_label.css_hex()
        };
        let icon = if active { "●" } else { "○" };
        let ind = if active {
            format!("background:{};", nav_h.active_indicator.css_hex())
        } else {
            String::new()
        };
        horizontal.push_str(&format!(
            r#"<div class="dest" data-nav-item="{label}" style="color:{fg}"><div class="ind" style="{ind}height:{h}px;padding:0 {pad}px">{icon} {label}</div></div>"#,
            h = nav_h.indicator_h_dp,
            pad = nav_h.indicator_pad_h_dp,
        ));
    }
    let mut inbox_nav = String::new();
    for (i, dest) in snackbar::INBOX_NAV.iter().enumerate() {
        let active = i == 0;
        let fg = if active {
            nav_bar.active_label.css_hex()
        } else {
            nav_bar.inactive_label.css_hex()
        };
        let ind = if active {
            format!(
                r#"style="background:{};""#,
                nav_bar.active_indicator.css_hex()
            )
        } else {
            String::new()
        };
        let badge = if i == snackbar::MEET_BADGE_INDEX {
            r#"<span class="meet-dot" data-meet-badge="1"></span>"#
        } else {
            ""
        };
        inbox_nav.push_str(&format!(
            r#"<div class="dest" data-inbox-nav="{label}" style="color:{fg}"><div class="ind" {ind}>{icon}</div>{label}{badge}</div>"#,
            label = dest.label,
            icon = dest.svg,
        ));
    }
    let title_bar = if snackbar::SCENE_SHOW_TITLE {
        format!(r#"<div class="phone-bar">{}</div>"#, snackbar::SCENE_TITLE)
    } else {
        String::new()
    };
    format!(
        r#"<h2>Snackbar</h2>
<p class="note">Official overview is a Gmail phone with “Email archived”, Action, close, and Mail / Chat / Rooms / Meet. Inverse surface · 4s timeout · swipe 72dp. <a href="https://m3.material.io/components/snackbar/specs">spec</a></p>
<div class="phone" data-mail-scene="1" data-hero="snackbar" style="height:{ph}px">
  <div class="status-bar" data-status-bar="1"><span>{stime}</span><span>5G · 100%</span></div>
  {title_bar}
  <div class="mail-list">{mail}</div>
  <div class="snack" data-snackbar="1" data-timeout-ms="{timeout}" data-swipe-dismiss="{swipe}" data-snackbar-close-affordance="1" data-persist="1" style="background:{sbg};color:{sfg};border-radius:{sr}px">
    <span>{scene_msg}</span>
    <span style="color:{act};font-weight:500">{scene_act}</span>
    <span class="snack-close" data-snackbar-close="1" style="color:{close}">{x}</span>
  </div>
  <div class="inbox-nav" data-inbox-nav="1" data-nav-flexible="1" style="background:{nbg};height:{nh}px">{inbox_nav}</div>
</div>
<div class="snack" data-snackbar="token" data-timeout-ms="{timeout}" data-swipe-dismiss="{swipe}" style="background:{sbg};color:{sfg};border-radius:{sr}px;margin-top:16px">
  <span>{msg}</span>
  <span style="color:{act};font-weight:500">{action}</span>
</div>
<h2>Navigation bar</h2>
<p class="note">Expressive flexible / short bar is 64dp. Baseline 80dp is not recommended. Compact uses vertical items (56×32 indicator); medium uses a 40dp horizontal pill. <a href="https://m3.material.io/components/navigation-bar/specs">spec</a></p>
<div class="nav" data-navbar="1" data-layout="vertical" data-hero="nav-bar" data-nav-height="{nh}" style="background:{nbg};height:{nh}px">
  <div class="dest" data-nav-item="Home" style="color:{nact}"><div class="ind" style="background:{ind};width:{iw}px;height:{ih}px">●</div>Home</div>
  <div class="dest" data-nav-item="Search" style="color:{nin}"><div class="ind" style="width:{iw}px;height:{ih}px">○</div>Search</div>
  <div class="dest" data-nav-item="Profile" style="color:{nin}"><div class="ind" style="width:{iw}px;height:{ih}px">○</div>Profile</div>
</div>
<div class="nav" data-navbar="horizontal" data-layout="horizontal" data-hero="nav-bar-horizontal" data-nav-height="{nh}" style="background:{hnbg};height:{nh}px">
  {horizontal}
</div>
<h2>Navigation rail</h2>
<p class="note">WideNavigationRailItem: collapsed Top icon (96dp, 56×32) / expanded Start icon (220dp, 56dp full-width pill). Active label is secondary. Interactive <strong>standard</strong> WideNavigationRail interpolates Top→Start in-flow (96↔220, no scrim, CornerNone / Surface) with Compose <code>ExtendedFloatingActionButton</code> (Create, 56↔188). Modal overlay uses the same 96 collapsed width over a 32% scrim, then <code>modalExpandedShape</code> CornerLarge 16 and <code>ModalContainerColor</code> SurfaceContainer, with the same header-less Extended FAB. Optional live <strong>narrow</strong> modal uses <code>NarrowContainerWidth</code> 80↔220 with the same modal container morph and Extended FAB (12dp collapsed inset). Dismissible modal <code>hideOnCollapse</code> slides offscreen (Menu ☰) with Start items, <code>Arrangement.Center</code>, expanded modal shape, and an always-extended <code>ExtendedFloatingActionButton</code> Create (~188). Header slot stays top: Menu / MenuOpen + plain tooltip Above + the same Extended FAB. Live <code>Arrangement.Bottom</code> packs destinations below the menu+FAB. Compose <code>WideNavigationRailDefaults.ContentPadding</code> is 0 / 44 / 0 / 44 (<code>WNRVerticalPadding</code> = TopSpace). <a href="https://m3.material.io/components/navigation-rail/specs">spec</a></p>
<div class="wide-rail-pair" data-hero="wide-rail">
  <div class="nav-rail" data-wide-collapsed="1" data-icon-position="top" data-nav-rail-wide="1" {content_pad} style="background:{rbg};width:{ww}px">{top_dests}</div>
  <div class="nav-rail" data-icon-position="start" data-nav-rail-wide="1" {content_pad} style="background:{rbg};width:{ew}px">{start_dests}</div>
</div>
<div class="rail-stage is-standard" data-hero="wide-rail-inflow" data-rail-layout="standard">
  <div class="nav-rail" data-nav-rail="1" data-nav-rail-wide="1" data-rail-layout="standard" data-wide-collapsed="1" data-nav-rail-expanded="0" data-icon-position="top" data-icon-morph="1" data-icon-morph-ms="{morph_ms}" data-collapsed-width="{ww}" data-container-collapsed="{rbg}" data-container-expanded="{rbg}" data-collapsed-shape="{shape0}" data-expanded-shape="{shape0}" data-rail-mode="collapsed" data-rail-selected="0" data-rail-focus-trap="0" {content_pad} style="background:{rbg};width:{ww}px">{inflow_extended_fab}{top_dests}</div>
  <div class="rail-inflow-body" data-rail-inflow-body="1">{inflow_body}</div>
</div>
<div class="rail-stage is-modal" data-hero="nav-rail" data-rail-layout="modal">
  <div class="rail-scrim" data-rail-scrim="1" data-visible="1" style="background:{scrim}"></div>
  <div class="rail-window" data-rail-window="1" data-rail-chrome="popup" data-rail-window-kind="popup" data-rail-os-popup="0" data-rail-popup-title="Navigation rail" data-rail-popup-h="880" data-rail-frame-ms="{frame_ms}">
  <div class="nav-rail expanded" data-nav-rail="1" data-nav-rail-expanded="1" data-rail-layout="modal" data-icon-position="start" data-icon-morph="1" data-icon-morph-ms="{morph_ms}" data-collapsed-width="{ww}" data-container-collapsed="{rbg}" data-container-expanded="{mbg}" data-collapsed-shape="{shape0}" data-expanded-shape="{shape1}" data-modal-expanded-shape="{shape_token}" data-rail-mode="expanded" data-rail-selected="0" data-rail-focus-trap="1" {content_pad} style="background:{mbg};width:{ew}px;border-radius:{shape1}px">{modal_extended_fab}{rail_dests}</div>
  </div>
</div>
<div class="rail-stage" data-hero="wide-rail-narrow" data-rail-layout="narrow">
  <div class="rail-scrim" data-rail-scrim="1" data-visible="0" style="background:{scrim}"></div>
  <div class="rail-window" data-rail-window="1" data-rail-chrome="popup" data-rail-window-kind="popup" data-rail-os-popup="0" data-rail-popup-title="Navigation rail" data-rail-popup-h="880" data-rail-frame-ms="{frame_ms}">
  <div class="nav-rail" data-nav-rail="1" data-nav-rail-wide="1" data-rail-layout="narrow" data-narrow="1" data-nav-rail-expanded="0" data-icon-position="top" data-icon-morph="1" data-icon-morph-ms="{morph_ms}" data-collapsed-width="{nw}" data-container-collapsed="{rbg}" data-container-expanded="{mbg}" data-collapsed-shape="{shape0}" data-expanded-shape="{shape1}" data-modal-expanded-shape="{shape_token}" data-rail-mode="collapsed" data-rail-selected="0" data-rail-focus-trap="0" {content_pad} style="background:{rbg};width:{nw}px">{narrow_extended_fab}{top_dests}</div>
  </div>
</div>
<div class="rail-stage is-hide" data-hero="wide-rail-hide" data-rail-layout="hide" data-hide-on-collapse="1">
  <div class="rail-scrim" data-rail-scrim="1" data-visible="0" style="background:{scrim}"></div>
  <button type="button" class="rail-menu" data-rail-menu="1" data-rail-menu-label="{hide_menu_label}" style="background:{fab_bg};color:{fab_fg}">{hide_menu}</button>
  <div class="rail-inflow-body" data-rail-inflow-body="1" data-hide-body="1">{inflow_body}</div>
  <div class="rail-window" data-rail-window="0" data-rail-chrome="popup" data-rail-window-kind="popup" data-rail-os-popup="0" data-rail-popup-title="Navigation rail" data-rail-popup-h="880" data-rail-frame-ms="{frame_ms}">
  <div class="nav-rail" data-nav-rail="1" data-nav-rail-wide="1" data-rail-layout="hide" data-hide-on-collapse="1" data-nav-rail-expanded="0" data-icon-position="start" data-icon-morph="0" data-icon-morph-ms="{morph_ms}" data-collapsed-width="0" data-expanded-width="{ew}" data-container-collapsed="{mbg}" data-container-expanded="{mbg}" data-collapsed-shape="{shape1}" data-expanded-shape="{shape1}" data-modal-expanded-shape="{shape_token}" data-rail-mode="collapsed" data-rail-arrangement="{arr}" data-rail-selected="0" data-rail-focus-trap="0" {content_pad} style="background:{mbg};width:{ew}px;transform:translateX(-100%);border-radius:{shape1}px">{hide_fab}<div class="rail-dests" data-rail-dests="1" data-rail-arrangement="{arr}">{start_dests}</div></div>
  </div>
</div>
<div class="rail-stage is-standard" data-hero="wide-rail-header" data-rail-layout="header">
  <div class="nav-rail" data-nav-rail="1" data-nav-rail-wide="1" data-rail-layout="header" data-rail-header="1" data-wide-collapsed="1" data-nav-rail-expanded="0" data-icon-position="top" data-icon-morph="1" data-icon-morph-ms="{morph_ms}" data-collapsed-width="{ww}" data-container-collapsed="{rbg}" data-container-expanded="{rbg}" data-collapsed-shape="{shape0}" data-expanded-shape="{shape0}" data-rail-mode="collapsed" data-rail-arrangement="{harr}" data-rail-selected="0" data-rail-focus-trap="0" {content_pad} style="background:{rbg};width:{ww}px">
    <div class="rail-header" data-rail-header-slot="1" data-header-space="{hspace}" data-tooltip-anchor="{htip}" data-header-fab="1">
      <div class="rail-header-menu-row">
        <button type="button" class="rail-header-btn" data-rail-header-menu="1" data-rail-header-label="{hlabel}" data-rail-header-state="{hstate}" aria-label="{hlabel}" style="color:{hbtn}">{hglyph}</button>
        <div class="rail-header-tip" data-rail-header-tooltip="1" data-tooltip-plain="header" data-tooltip-text="{hlabel}" style="background:{htbg};color:{htfg}">{hlabel}</div>
      </div>
      <div class="fab-slot" data-rail-fab-extend="1" style="background:{fab_bg};color:{fab_fg}"><span class="fab-icon">{fglyph}</span><span class="fab-label" data-rail-fab-label="1">{flabel}</span></div>
    </div>
    <div class="rail-dests" data-rail-dests="1" data-rail-arrangement="{harr}">{top_dests}</div>
  </div>
  <div class="rail-inflow-body" data-rail-inflow-body="1" data-header-body="1">{inflow_body}</div>
</div>"#,
        sbg = snack.container.css_hex(),
        sfg = snack.supporting.css_hex(),
        sr = snack.corners.top_left,
        act = snack.action.css_hex(),
        close = snack.close.css_hex(),
        timeout = snackbar::TIMEOUT_SHORT_MS,
        swipe = snackbar::SWIPE_DISMISS_DP,
        msg = snackbar::DEMO_MESSAGE,
        action = snackbar::DEMO_ACTION,
        scene_msg = snackbar::SCENE_MESSAGE,
        scene_act = snackbar::SCENE_ACTION,
        title_bar = title_bar,
        x = snackbar::CLOSE_GLYPH,
        mail = mail_rows,
        inbox_nav = inbox_nav,
        stime = snackbar::STATUS_TIME,
        ph = snackbar::PHONE_H_DP,
        nbg = nav.container.css_hex(),
        nact = nav.active_label.css_hex(),
        ind = nav.active_indicator.css_hex(),
        nin = nav.inactive_label.css_hex(),
        nh = nav.height_dp,
        iw = nav.indicator_w_dp,
        ih = nav.indicator_h_dp,
        hnbg = nav_h.container.css_hex(),
        horizontal = horizontal,
        rbg = rail.container.css_hex(),
        mbg = navigation_rail::modal_container_color(theme).css_hex(),
        shape0 = navigation_rail::SHAPE_DP,
        shape1 = navigation_rail::MODAL_EXPANDED_SHAPE_DP,
        shape_token = navigation_rail::MODAL_EXPANDED_SHAPE_TOKEN,
        ew = expanded.width_dp,
        ww = navigation_rail::WIDE_COLLAPSED_WIDTH_DP,
        nw = navigation_rail::WIDTH_DP,
        rail_dests = rail_dests,
        top_dests = top_dests,
        start_dests = start_dests,
        inflow_extended_fab = inflow_extended_fab,
        modal_extended_fab = modal_extended_fab,
        narrow_extended_fab = narrow_extended_fab,
        inflow_body = navigation_rail::IN_FLOW_BODY,
        hide_menu = navigation_rail::HIDE_MENU_GLYPH,
        hide_menu_label = navigation_rail::HIDE_MENU_LABEL,
        content_pad = content_pad_attrs,
        hide_fab = hide_extended_fab,
        fab_bg = rail.fab.css_hex(),
        fab_fg = rail.fab_icon.css_hex(),
        arr = navigation_rail::HIDE_DEMO_ARRANGEMENT.label(),
        harr = navigation_rail::HEADER_DEMO_ARRANGEMENT.label(),
        hspace = navigation_rail::header_space_dp(
            navigation_rail::HEADER_DEMO_HAS_HEADER,
            navigation_rail::HEADER_DEMO_ARRANGEMENT,
        ),
        htip = navigation_rail::HEADER_TOOLTIP_ABOVE,
        hlabel = navigation_rail::header_menu_label(false),
        hstate = navigation_rail::header_state_description(false),
        hglyph = navigation_rail::header_menu_glyph(false),
        fglyph = navigation_rail::FAB_GLYPH,
        flabel = navigation_rail::FAB_LABEL,
        htbg = tooltip::resolve_plain(theme).container.css_hex(),
        htfg = tooltip::resolve_plain(theme).supporting.css_hex(),
        hbtn = icon_button::resolve(
            theme,
            icon_button::IconButtonVariant::Standard,
            InteractionState::Enabled,
        )
        .content
        .css_hex(),
        scrim = navigation_rail::scrim(theme).css_hex(),
        frame_ms = crate::motion::FRAME_MS,
        morph_ms = navigation_rail::morph_ms(theme),
    )
}

fn tooltips_section(theme: &Theme) -> String {
    let plain = tooltip::resolve_plain(theme);
    let rich = tooltip::resolve_rich(theme);
    let anchor = icon_button::resolve(
        theme,
        icon_button::IconButtonVariant::Tonal,
        crate::state::InteractionState::Enabled,
    );
    format!(
        r#"<h2>Tooltip</h2>
<p class="note">Plain labels icon-only controls (inverse surface, 24dp, 16×8 caret). Hover or 500ms long-press to show. Rich adds a subhead, supporting text, two text buttons, and a caret. <a href="https://m3.material.io/components/tooltips/specs">spec</a></p>
<div class="tooltip-stage" data-hero="tooltip">
  <div class="tooltip-anchor" data-tooltip-plain="1" data-tooltip-trigger="{trig}" data-tooltip-longpress-ms="{hold}" data-open="0">
    <div class="tooltip-bubble">
      <div class="tooltip-plain" data-tooltip="plain" data-tooltip-text="{plain_text}" style="background:{pbg};color:{pfg};min-height:{ph}px;max-width:{pmw}px;padding:{ppv}px {pph}px;border-radius:{pr}px">{plain_text}</div>
      <div class="tooltip-caret" data-tooltip-caret="plain" style="border-top-color:{pbg}"></div>
    </div>
    <button class="icon-btn" data-tooltip-anchor="1" style="background:{abg};color:{afg};width:{asz}px;height:{asz}px;border-radius:{ar}px">{ag}</button>
  </div>
  <div class="tooltip-anchor" data-tooltip-rich="1" data-tooltip-trigger="{trig}" data-tooltip-longpress-ms="{hold}" data-open="0">
    <button class="icon-btn" data-tooltip-anchor="rich" style="background:{abg};color:{afg};width:{asz}px;height:{asz}px;border-radius:{ar}px">?</button>
    <div class="tooltip-bubble">
      <div class="tooltip-caret up" data-tooltip-caret="rich" style="border-bottom-color:{rbg}"></div>
      <div class="tooltip-rich" data-tooltip="rich" data-tooltip-subhead="{sub}" style="background:{rbg};color:{rfg};max-width:{rmw}px;padding:{rpt}px {rph}px {rpb}px;border-radius:{rr}px;box-shadow:{rsh}">
        <div class="sub" data-tooltip-sub="1" style="color:{rsub}">{sub}</div>
        <div class="body" data-tooltip-body="1">{body}</div>
        <div class="acts">
          <span data-tooltip-action="learn" style="color:{ract}">{learn}</span>
          <span data-tooltip-action="dismiss" style="color:{ract}">{dismiss}</span>
        </div>
      </div>
    </div>
  </div>
</div>"#,
        trig = tooltip::HOVER_TRIGGER,
        hold = tooltip::LONG_PRESS_MS,
        plain_text = tooltip::PLAIN_TEXT,
        pbg = plain.container.css_hex(),
        pfg = plain.supporting.css_hex(),
        ph = plain.min_height_dp,
        pmw = plain.max_width_dp,
        ppv = plain.pad_top_dp,
        pph = plain.pad_start_dp,
        pr = plain.corners.top_left,
        abg = anchor.container.css_hex(),
        afg = anchor.content.css_hex(),
        asz = anchor.height_dp,
        ar = anchor.corners.top_left,
        ag = tooltip::PLAIN_ANCHOR,
        sub = tooltip::RICH_SUBHEAD,
        body = tooltip::RICH_SUPPORTING,
        learn = tooltip::RICH_ACTION_PRIMARY,
        dismiss = tooltip::RICH_ACTION_SECONDARY,
        rbg = rich.container.css_hex(),
        rfg = rich.supporting.css_hex(),
        rmw = rich.max_width_dp,
        rpt = rich.pad_top_dp,
        rph = rich.pad_start_dp,
        rpb = rich.pad_bottom_dp,
        rr = rich.corners.top_left,
        rsh = ElevationLevels::css_shadow(rich.elevation_dp),
        rsub = rich.subhead.unwrap_or(rich.supporting).css_hex(),
        ract = rich.action.unwrap_or(theme.color.primary).css_hex(),
    )
}

fn paint_app_bar(
    theme: &Theme,
    variant: top_app_bar::AppBarVariant,
    subtitle: Option<&str>,
    collapse: f32,
    title: &str,
) -> String {
    let a = top_app_bar::resolve_variant(theme, variant, subtitle, collapse, collapse > 0.001);
    let rest = top_app_bar::resolve_variant(theme, variant, subtitle, 0.0, false);
    let scrolled = top_app_bar::resolve_variant(theme, variant, subtitle, 1.0, true);
    let (row_mid, below) = if variant == top_app_bar::AppBarVariant::Search {
        let search = search::resolve(theme);
        (
            format!(
                r#"<div class="appbar-search" data-appbar-search="1" style="background:{bg};color:{fg};height:{h}px">
  <span>{lead}</span><span style="flex:1;color:{hint}">{ph}</span>
  <span class="avatar" style="width:30px;height:30px;border-radius:15px;background:{av};color:{avon};display:flex;align-items:center;justify-content:center">A</span>
</div>"#,
                bg = search.bar.container.css_hex(),
                fg = search.bar.content.css_hex(),
                h = a.search_field_h_dp,
                lead = search::LEADING_ICON,
                hint = search.placeholder.css_hex(),
                ph = top_app_bar::SEARCH_PLACEHOLDER,
                av = search.avatar.css_hex(),
                avon = search.avatar_label.css_hex(),
            ),
            String::new(),
        )
    } else {
        let sub = subtitle
            .filter(|s| !s.is_empty())
            .map(|s| {
                format!(
                    r#"<div class="appbar-sub" data-appbar-sub="1" style="color:{c};font-size:{sz}px;line-height:{lh}px;font-weight:{w}">{s}</div>"#,
                    c = a.subtitle.css_hex(),
                    sz = a.subtitle_style.size_sp,
                    lh = a.subtitle_style.line_height_sp,
                    w = a.subtitle_style.weight,
                    s = esc(s),
                )
            })
            .unwrap_or_default();
        (
            format!(
                r#"<div class="appbar-inline" data-appbar-inline="1">
  <div data-appbar-title="1" style="color:{c};font-size:{sz}px;line-height:{lh}px;font-weight:{w}">{title}</div>
</div>"#,
                c = a.title.css_hex(),
                sz = a.title_style.size_sp,
                lh = a.title_style.line_height_sp,
                w = a.title_style.weight,
                title = esc(title),
            ),
            format!(
                r#"<div class="appbar-titles">
  <div data-appbar-title="1" style="color:{c};font-size:{sz}px;line-height:{lh}px;font-weight:{w}">{title}</div>
  {sub}
</div>"#,
                c = a.title.css_hex(),
                sz = a.title_style.size_sp,
                lh = a.title_style.line_height_sp,
                w = a.title_style.weight,
                title = esc(title),
                sub = sub,
            ),
        )
    };
    let trailing = if variant == top_app_bar::AppBarVariant::Search {
        String::new()
    } else {
        top_app_bar::SCENE_TRAILING
            .iter()
            .map(|g| format!(r#"<span class="ico" data-appbar-action="{g}">{g}</span>"#))
            .collect()
    };
    format!(
        r#"<div class="appbar" data-appbar="{v}" data-collapse="{col}" data-expanded-h="{eh}" data-collapsed-h="{ch}" data-title-exp="{texp}" data-title-col="{tcol}" data-rest-bg="{rbg}" data-scrolled-bg="{sbg}" data-scrolled-sh="{ssh}" style="height:{h}px;background:{bg};color:{fg};box-shadow:{sh}">
  <div class="appbar-row">
    <span class="ico" data-appbar-back="1">{back}</span>
    {row_mid}
    {trailing}
  </div>
  {below}
</div>"#,
        v = variant.label(),
        col = top_app_bar::collapse_attr(collapse),
        eh = a.expanded_height_dp,
        ch = a.collapsed_height_dp,
        texp = rest.title_style.size_sp,
        tcol = scrolled.title_style.size_sp,
        rbg = rest.container.css_hex(),
        sbg = scrolled.container.css_hex(),
        ssh = ElevationLevels::css_shadow(scrolled.elevation_dp),
        h = a.height_dp,
        bg = a.container.css_hex(),
        fg = a.title.css_hex(),
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        back = top_app_bar::SCENE_LEADING,
        row_mid = row_mid,
        trailing = trailing,
        below = below,
    )
}

fn app_bars(theme: &Theme) -> String {
    let large = paint_app_bar(
        theme,
        top_app_bar::AppBarVariant::LargeFlexible,
        Some(top_app_bar::SCENE_SUBTITLE),
        0.0,
        top_app_bar::SCENE_TITLE,
    );
    let medium = paint_app_bar(
        theme,
        top_app_bar::AppBarVariant::MediumFlexible,
        Some(top_app_bar::MEDIUM_SCENE_SUBTITLE),
        0.0,
        top_app_bar::MEDIUM_SCENE_TITLE,
    );
    let search_bar = paint_app_bar(
        theme,
        top_app_bar::AppBarVariant::Search,
        None,
        0.0,
        top_app_bar::SEARCH_PLACEHOLDER,
    );
    let collapsed = paint_app_bar(
        theme,
        top_app_bar::AppBarVariant::LargeFlexible,
        Some(top_app_bar::SCENE_SUBTITLE),
        1.0,
        top_app_bar::SCENE_TITLE,
    );
    format!(
        r#"<h2>Top app bar</h2>
<p class="note">Expressive flexible bars compress into the 64dp small bar. Medium 112/136 · large 120/152 · search 56-in-64. Scrolled fill is surface-container at elevation 2. Click the Bloom phone to cycle collapse. <a href="https://m3.material.io/components/app-bars/specs">spec</a></p>
<div class="phone" data-appbar-scene="1" data-hero="app-bar" style="height:{ph}px">
  <div class="status-bar" data-status-bar="1"><span>{stime}</span><span>5G · 100%</span></div>
  {large}
  <div class="photo-hero photo-stub" data-appbar-photo="1" data-photo="{photo}" data-decoded-jpeg="1" data-licensed-camera="1" data-photo-license="{lic}" style="background:{css};height:{ih}px"></div>
</div>
<h3>medium flexible</h3>
<div data-hero="app-bar-medium">{medium}</div>
<h3>search app bar</h3>
<div data-hero="app-bar-search">{search}</div>
<h3>collapsed</h3>
<div data-hero="app-bar-collapsed">{collapsed}</div>"#,
        ph = top_app_bar::PHONE_H_DP,
        stime = top_app_bar::STATUS_TIME,
        large = large,
        photo = top_app_bar::SCENE_PHOTO.label(),
        lic = top_app_bar::SCENE_PHOTO.credit().license,
        css = top_app_bar::SCENE_PHOTO.css_background(),
        ih = top_app_bar::PHOTO_H_DP,
        medium = medium,
        search = search_bar,
        collapsed = collapsed,
    )
}

fn progress_section(theme: &Theme) -> String {
    let lin = progress::linear(theme, 0.6);
    let circ = progress::circular(theme, 0.6);
    let indet = progress::linear_indeterminate(theme);
    let circ_i = progress::circular_indeterminate(theme);
    let wave = progress::wavy(theme, progress::WAVE_DEMO_PROGRESS);
    let wave_d = progress::wave_svg_d(wave.width_dp, wave.height_dp, wave.progress, 0.0);
    let load = progress::contained_loading_indicator(theme);
    let morph = progress::loading_indicator(theme);
    let lsz = progress::LOADING_SIZE_DP;
    let loadd0 = progress::loading_svg_d(lsz, 0.0);
    let lvals = progress::loading_svg_values(lsz, 8);
    let detd = progress::loading_svg_d_for_wait(lsz, progress::DEMO_WAIT);
    let wait_frames = progress::loading_svg_values_for_wait(lsz, 8);
    let wait_ms = progress::determinate_wait_ms(theme);
    let capd = progress::ptr_arc_svg_d(circ_i.size_dp, circ_i.stroke_dp, circ_i.arc_deg, 0.0);
    format!(
        r#"<h2>Progress</h2>
<p class="note">Determinate, wavy determinate, indeterminate, plus M3 Expressive morphing loading indicator (contained for PTR). HTML CSS + GPUI Animation clock. <a href="https://m3.material.io/components/loading-indicator/overview">loading</a> · <a href="https://m3.material.io/components/progress-indicators/specs">progress</a></p>
<div class="linear" data-progress="linear" style="background:{track}"><i style="width:{p}%;background:{ind}"></i></div>
<div class="circ" data-progress="circular" style="background:conic-gradient({cind} {ang}deg, {ctrack} 0deg)"></div>
<div class="wave" data-progress="wavy" data-hero="progress-wavy">
  <svg viewBox="0 0 {ww} {wh}" width="{ww}" height="{wh}" aria-hidden="true">
    <path d="M0,{mid} H{ww}" stroke="{wtrack}" stroke-width="{wsw}" fill="none"/>
    <path class="wave-path" d="{wd}" stroke="{wind}" stroke-width="{wsw}" fill="none" stroke-linecap="round"/>
  </svg>
</div>
<h3>indeterminate</h3>
<div class="linear indet" data-progress="indeterminate" data-hero="progress-indet" style="background:{itrack};margin:12px 0"><i style="width:{span}%;background:{iind}"></i></div>
<div class="circ indet" data-progress="circular-indet" style="background:conic-gradient({ciind} {arc}deg, {citrack} 0deg)"></div>
<div class="ptr" data-progress="ptr" data-hero="progress-ptr" data-loading-morph="1">
  <div class="loading-contained" style="background:{lbox}">
    <svg class="loading-shape" width="{lsz}" height="{lsz}" viewBox="0 0 {lsz} {lsz}" aria-hidden="true">
      <path fill="{lind}" d="{loadd0}">
        <animate attributeName="d" dur="{ldur}ms" repeatCount="indefinite" values="{lvals}"/>
      </path>
    </svg>
  </div>
  <div class="note">{plabel}</div>
</div>
<div class="loading-row" data-progress="loading" data-hero="progress-loading" data-loading-morph="1">
  <svg class="loading-shape" width="{lsz}" height="{lsz}" viewBox="0 0 {lsz} {lsz}" aria-hidden="true">
    <path fill="{mind}" d="{loadd0}">
      <animate attributeName="d" dur="{ldur}ms" repeatCount="indefinite" values="{lvals}"/>
    </path>
  </svg>
  <svg width="{csz}" height="{csz}" viewBox="0 0 {csz} {csz}" aria-hidden="true">
    <path d="{capd}" fill="none" stroke="{cind_fill}" stroke-width="{csw}" stroke-linecap="round" data-linecap="{lcap}" data-stroke-cap="{lcap}" data-progress-stroke="round"/>
  </svg>
  <div class="note">{llabel}</div>
</div>
<div class="loading-row" data-progress="loading-determinate" data-hero="progress-loading-det" data-wait-progress="1" data-wait-ms="{wms}">
  <svg class="loading-shape" width="{lsz}" height="{lsz}" viewBox="0 0 {lsz} {lsz}" aria-hidden="true">
    <path data-wait-morph="1" data-wait-frames="{wframes}" fill="{mind}" d="{detd}"/>
  </svg>
  <div class="note" data-wait-label="1">{detlabel}</div>
</div>"#,
        track = lin.track.css_hex(),
        ind = lin.indicator.css_hex(),
        p = lin.progress * 100.0,
        cind = circ.indicator.css_hex(),
        ctrack = circ.track.css_hex(),
        ang = circ.progress * 360.0,
        itrack = indet.track.css_hex(),
        iind = indet.indicator.css_hex(),
        span = indet.head_span * 100.0,
        ciind = circ_i.indicator.css_hex(),
        citrack = circ_i.track.css_hex(),
        arc = circ_i.arc_deg,
        lbox = load.container.css_hex(),
        lsz = lsz,
        lind = load.indicator.css_hex(),
        mind = morph.indicator.css_hex(),
        cind_fill = circ_i.indicator.css_hex(),
        loadd0 = loadd0,
        lvals = lvals,
        ldur = load.duration_ms,
        plabel = progress::PTR_LABEL,
        csz = circ_i.size_dp,
        capd = capd,
        csw = circ_i.stroke_dp,
        llabel = progress::LOADING_LABEL,
        detd = detd,
        detlabel = format!("{:.0}%", progress::DEMO_WAIT.fraction() * 100.0),
        wms = wait_ms,
        wframes = wait_frames,
        lcap = progress::LINE_CAP,
        ww = wave.width_dp,
        wh = wave.height_dp,
        mid = wave.height_dp / 2.0,
        wtrack = wave.track.css_hex(),
        wind = wave.indicator.css_hex(),
        wsw = wave.stroke_dp,
        wd = wave_d,
    )
}

fn dialogs(theme: &Theme) -> String {
    let a = dialog::resolve(theme);
    let text_btn = button::resolve(
        theme,
        button::ButtonVariant::Text,
        InteractionState::Enabled,
    );
    let selected = radio::resolve(theme, true, InteractionState::Enabled);
    let idle = radio::resolve(theme, false, InteractionState::Enabled);
    let mut accounts = String::from(r#"<div class="accounts" data-dialog-accounts="1">"#);
    for email in dialog::RESET_ACCOUNTS {
        accounts.push_str(&format!(
            r#"<div class="account" data-account="{email}"><div class="avatar" style="background:{abg};color:{afg}">{initial}</div><span>{email}</span></div>"#,
            abg = theme.color.secondary_container.css_hex(),
            afg = theme.color.on_secondary_container.css_hex(),
            initial = dialog::account_initials(email),
        ));
    }
    accounts.push_str("</div>");
    let mut rows = String::new();
    for (i, label) in dialog::RINGTONE_OPTIONS.iter().enumerate() {
        let r = if i == 2 { &selected } else { &idle };
        let inner = r
            .inner
            .map(|c| format!("<i style=\"background:{}\"></i>", c.css_hex()))
            .unwrap_or_default();
        rows.push_str(&format!(
            r#"<div class="ringtone" data-ringtone="{label}"><span>{label}</span><div class="radio"><div class="dot" style="border:2px solid {ring}">{inner}</div></div></div>"#,
            ring = r.ring.css_hex(),
        ));
    }
    format!(
        r#"<h2>Dialog</h2>
<p class="note">Official pair: basic Reset settings + full-screen Event editor, plus Phone ringtone list. 28dp · elev 3 · 32% scrim for basic; full-screen is surface / 0 corners / 64dp header. <a href="https://m3.material.io/components/dialogs/overview">overview</a></p>
<div class="scrim" data-dialog="scrim" style="background:{scrim}">
  <div class="dialog" data-dialog="basic" data-hero="dialog" style="background:{bg};color:{fg};border-radius:{r}px;box-shadow:{sh};min-width:{mw}px;text-align:center;align-items:center">
    <div style="font-size:{icon_dp}px;color:{icon}">{reset_icon}</div>
    <div style="font-size:{hs}px;line-height:{hl}px;font-weight:{hw};color:{head}">{reset_h}</div>
    <div style="font-size:{bs}px;line-height:{bl}px;color:{sup};text-align:center">{reset_s}</div>
    {accounts}
    <div class="actions">
      <button class="btn" style="background:{abg};color:{act}">{cancel}</button>
      <button class="btn" style="background:{abg};color:{act}">{accept}</button>
    </div>
  </div>
</div>
<div class="scrim" data-dialog="list-scrim" style="background:{scrim};margin-top:16px">
  <div class="dialog dialog-list" data-dialog="list" data-hero="dialog-list" style="background:{bg};color:{fg};border-radius:{r}px;box-shadow:{sh};min-width:{mw}px">
    <div style="font-size:{hs}px;line-height:{hl}px;font-weight:{hw};color:{head};text-align:left">{list_h}</div>
    {rows}
    <div class="actions">
      <button class="btn" style="background:{abg};color:{act}">{list_cancel}</button>
      <button class="btn" style="background:{abg};color:{act}">{list_ok}</button>
    </div>
  </div>
</div>
<div data-dialog="fullscreen-stage" style="margin-top:16px">{fullscreen}</div>"#,
        scrim = a.scrim.css_hex(),
        bg = a.container.css_hex(),
        fg = a.headline.css_hex(),
        r = a.corners.top_left,
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        mw = a.min_width_dp,
        hs = a.headline_style.size_sp,
        hl = a.headline_style.line_height_sp,
        hw = a.headline_style.weight,
        head = a.headline.css_hex(),
        bs = a.supporting_style.size_sp,
        bl = a.supporting_style.line_height_sp,
        sup = a.supporting.css_hex(),
        act = a.action.css_hex(),
        abg = text_btn.container.css_hex(),
        icon = a.icon.css_hex(),
        icon_dp = dialog::ICON_DP,
        reset_icon = dialog::RESET_ICON,
        reset_h = dialog::RESET_HEADLINE,
        reset_s = dialog::RESET_SUPPORTING,
        cancel = dialog::RESET_CANCEL,
        accept = dialog::RESET_ACCEPT,
        list_h = dialog::RINGTONE_HEADLINE,
        list_cancel = dialog::RINGTONE_CANCEL,
        list_ok = dialog::RINGTONE_OK,
        rows = rows,
        accounts = accounts,
        fullscreen = fullscreen_dialog(theme),
    )
}

fn fullscreen_dialog(theme: &Theme) -> String {
    let a = dialog::resolve_fullscreen(theme);
    let text_btn = button::resolve(
        theme,
        button::ButtonVariant::Text,
        InteractionState::Enabled,
    );
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        false,
    );
    let mut fields = String::new();
    for label in dialog::FULLSCREEN_FIELDS {
        fields.push_str(&format!(
            r#"<div class="fs-field" data-fs-field="{label}" style="background:{bg};color:{fg}">{label}</div>"#,
            bg = field.field.container.css_hex(),
            fg = a.supporting.css_hex(),
        ));
    }
    let divider = if dialog::FULLSCREEN_HAS_DIVIDER {
        format!(
            r#"<div class="divider" data-fs-divider="1" style="background:{}"></div>"#,
            a.divider.css_hex()
        )
    } else {
        String::new()
    };
    format!(
        r#"<div class="dialog dialog-fullscreen" data-dialog="fullscreen" data-hero="dialog-fullscreen" style="background:{bg};color:{fg};border-radius:{r}px;box-shadow:{sh}">
  <div class="fs-head" data-fs-header="1" style="height:{hh}px">
    <span data-fs-close="1" style="font-size:{icon}px;color:{ic}">{close}</span>
    <span style="flex:1;font-size:{hs}px;line-height:{hl}px;font-weight:{hw}">{head}</span>
    <button class="btn" style="background:{abg};color:{act}">{save}</button>
  </div>
  {divider}
  <div class="fs-fields">{fields}</div>
</div>"#,
        bg = a.container.css_hex(),
        fg = a.headline.css_hex(),
        r = a.corners.top_left,
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        hh = a.header_h_dp,
        icon = dialog::ICON_DP,
        ic = a.icon.css_hex(),
        close = dialog::FULLSCREEN_CLOSE,
        hs = a.headline_style.size_sp,
        hl = a.headline_style.line_height_sp,
        hw = a.headline_style.weight,
        head = dialog::FULLSCREEN_HEADLINE,
        abg = text_btn.container.css_hex(),
        act = a.action.css_hex(),
        save = dialog::FULLSCREEN_SAVE,
        divider = divider,
        fields = fields,
    )
}

fn sheets(theme: &Theme) -> String {
    let modal = bottom_sheet::resolve(theme, true);
    let standard = bottom_sheet::resolve(theme, false);
    let mut album = String::new();
    for label in bottom_sheet::ALBUM_ACTIONS {
        album.push_str(&format!(
            r#"<span data-album-action="{label}">{label}</span>"#,
        ));
    }
    let mut actions = String::new();
    for (icon, label) in bottom_sheet::SHARE_ACTIONS {
        actions.push_str(&format!(
            r#"<div class="sheet-action" data-share-action="{label}"><span style="font-size:20px">{icon}</span>{label}</div>"#,
        ));
    }
    let mut people = String::new();
    for person in bottom_sheet::PEOPLE {
        people.push_str(&format!(
            r#"<div class="people" data-share-person="{first}"><div class="av photo-stub" data-photo="{photo}" data-decoded-jpeg="1" data-licensed-camera="1" style="background:{css}"></div><span class="pn">{first}</span><span class="pn">{last}</span></div>"#,
            first = person.first,
            last = person.last,
            photo = person.photo.label(),
            css = person.photo.css_background(),
        ));
    }
    format!(
        r#"<h2>Bottom sheet</h2>
<p class="note">Official overview is a share sheet over a photo album: horizontal Share / Add to / Trash, then Send + named people. Modal extra-large top 28 · 32×4 handle · elevation 1. <a href="https://m3.material.io/components/bottom-sheets/specs">spec</a></p>
<div class="phone share-stage" data-sheet-scene="1" data-hero="bottom-sheet" style="height:{ph}px;background:{surface}">
  <div class="status-bar" data-status-bar="1"><span>{stime}</span><span>5G · 100%</span></div>
  <div class="share-hero photo-stub" data-share-photo="0" data-share-grid="1" data-decoded-jpeg="1" data-licensed-camera="1" style="background:{album_css}">
    <div class="album-bar">{album}</div>
  </div>
  <div class="sheet" data-sheet="modal" data-sheet-share="1" style="background:{bg};border-radius:{css};box-shadow:{sh};color:{fg};position:relative">
    <div class="handle" style="width:{hw}px;height:{hh}px;background:{handle}"></div>
    <div class="sheet-actions">{actions}</div>
    <div class="list-item" style="width:100%"><div class="h">{send}</div></div>
    <div class="people-row" data-share-people="1">{people}</div>
  </div>
</div>
<div class="sheet" data-sheet="standard" style="background:{sbg};border-radius:{scss};box-shadow:{ssh};color:{sfg};margin-top:16px">
  <div class="handle" style="width:{hw}px;height:{hh}px;background:{shandle}"></div>
  <div class="list-item" style="width:100%"><div class="h">Open in Maps</div></div>
</div>"#,
        ph = bottom_sheet::PHONE_H_DP,
        surface = theme.color.surface.css_hex(),
        album = album,
        album_css = photo_stub::PhotoKind::Party.css_background(),
        people = people,
        stime = bottom_sheet::STATUS_TIME,
        bg = modal.container.css_hex(),
        css = modal.corners.css(),
        sh = ElevationLevels::css_shadow(modal.elevation_dp),
        fg = modal.content.css_hex(),
        hw = modal.handle_w,
        hh = modal.handle_h,
        handle = modal.handle.css_hex(),
        send = bottom_sheet::SEND_TITLE,
        actions = actions,
        sbg = standard.container.css_hex(),
        scss = standard.corners.css(),
        ssh = ElevationLevels::css_shadow(standard.elevation_dp),
        sfg = standard.content.css_hex(),
        shandle = standard.handle.css_hex(),
    )
}

fn side_sheets(theme: &Theme) -> String {
    let modal = side_sheet::resolve_scene(theme);
    let standard = side_sheet::resolve(theme, side_sheet::SideSheetVariant::Standard);
    let detached = side_sheet::resolve(theme, side_sheet::SideSheetVariant::Detached);
    let mut filters = String::new();
    for (label, value) in side_sheet::FILTERS {
        filters.push_str(&format!(
            r#"<div class="side-filter" data-side-filter="{label}">
  <span style="color:{fg}">{label}</span>
  <span style="color:{sec}">{value}</span>
</div>"#,
            fg = modal.content.css_hex(),
            sec = modal.supporting.css_hex(),
        ));
    }
    let mut tiles = String::new();
    for (i, kind) in side_sheet::SCENE_PHOTOS.iter().enumerate() {
        tiles.push_str(&format!(
            r#"<div class="media-tile photo-stub" data-side-photo="{i}" data-photo="{photo}" data-decoded-jpeg="1" data-licensed-camera="1" style="background:{css};height:{h}px"></div>"#,
            photo = kind.label(),
            css = kind.css_background(),
            h = side_sheet::PHOTO_TILE_H_DP,
        ));
    }
    format!(
        r#"<h2>Side sheet</h2>
<p class="note">Expressive keeps side sheets; navigation drawers are deprecated in favor of the expanded rail. Modal 256 · surface-container-low · 16dp start corners · elev 1 · 32% scrim. Official-style Filters pane over a photo grid. <a href="https://m3.material.io/components/side-sheets/specs">spec</a></p>
<div class="phone side-stage" data-side-scene="1" data-hero="side-sheet" style="height:{ph}px;background:{surface}">
  <div class="status-bar" data-status-bar="1"><span>{stime}</span><span>5G · 100%</span></div>
  <div class="media-grid">{tiles}</div>
  <div class="side-scrim" data-side-scrim="1" style="background:{scrim}"></div>
  <div class="side-sheet" data-side-sheet="modal" data-side-filters="1" style="width:{w}px;background:{bg};border-radius:{css};box-shadow:{sh};color:{fg}">
    <div class="side-head">
      <div data-side-headline="1" style="color:{hl};font-size:{hsz}px;line-height:{hlh}px;font-weight:{hw}">{headline}</div>
      <span data-side-close="1" style="color:{close}">{x}</span>
    </div>
    <div class="side-filters">{filters}</div>
    <div class="side-actions"><span data-side-apply="1" style="color:{act};font-weight:500">{apply}</span></div>
  </div>
</div>
<div class="state-body" style="margin-top:16px">
  <div class="side-sheet" data-side-sheet="standard" style="position:relative;width:{sw}px;height:160px;background:{sbg};border-radius:{scss}">standard 256</div>
  <div class="side-sheet" data-side-sheet="detached" style="position:relative;width:{dw}px;height:160px;background:{dbg};border-radius:{dcss};margin:{dm}px;box-shadow:{dsh}">detached 16</div>
</div>"#,
        ph = side_sheet::PHONE_H_DP,
        surface = theme.color.surface.css_hex(),
        stime = side_sheet::STATUS_TIME,
        tiles = tiles,
        scrim = modal.scrim.css_hex(),
        w = modal.width_dp,
        bg = modal.container.css_hex(),
        css = modal.corners.css(),
        sh = ElevationLevels::css_shadow(modal.elevation_dp),
        fg = modal.content.css_hex(),
        hl = modal.headline.css_hex(),
        hsz = modal.headline_style.size_sp,
        hlh = modal.headline_style.line_height_sp,
        hw = modal.headline_style.weight,
        headline = side_sheet::HEADLINE,
        close = modal.close.css_hex(),
        x = side_sheet::CLOSE_GLYPH,
        filters = filters,
        act = modal.action.css_hex(),
        apply = side_sheet::APPLY_LABEL,
        sw = standard.width_dp,
        sbg = standard.container.css_hex(),
        scss = standard.corners.css(),
        dw = detached.width_dp,
        dbg = detached.container.css_hex(),
        dcss = detached.corners.css(),
        dm = detached.margin_dp,
        dsh = ElevationLevels::css_shadow(detached.elevation_dp),
    )
}

fn paint_menu_item_row(
    item: &menu::MenuDemoItem,
    a: &menu::MenuItemAppearance,
    selected: bool,
    extra: &str,
) -> String {
    let trail = menu::trailing_text(item);
    let trail_html = if trail.is_empty() {
        String::new()
    } else {
        format!(
            r#"<span class="trail" style="color:{c}">{t}</span>"#,
            c = a.shortcut.css_hex(),
            t = esc(trail),
        )
    };
    format!(
        r#"<div class="menu-item" role="menuitem" data-menu-item="{label}" data-selected="{sel}"{extra} style="background:{bg};color:{fg};height:{h}px;border-radius:{r};padding:0 {pad}px">
  <span class="lead" style="color:{ic}">{icon}</span>
  <span class="lbl">{label}</span>
  {trail}
</div>"#,
        label = esc(item.label),
        sel = selected,
        extra = extra,
        bg = a.container.css_hex(),
        fg = a.label.css_hex(),
        h = a.height_dp,
        r = a.corners.css(),
        pad = a.pad_h_dp,
        ic = a.icon.css_hex(),
        icon = esc(item.icon),
        trail = trail_html,
    )
}

fn overflow_leaf_extra(kind: menu::GroupedPopupKind, item: &menu::MenuDemoItem) -> String {
    if item.submenu {
        return String::new();
    }
    let label = esc(item.label);
    match kind {
        menu::GroupedPopupKind::StandardOverflow => {
            format!(r#" data-overflow-item="{label}" data-standard-overflow-item="{label}""#)
        }
        menu::GroupedPopupKind::ConnectedOverflow => {
            format!(r#" data-overflow-item="{label}""#)
        }
        menu::GroupedPopupKind::Split => format!(r#" data-split-item="{label}""#),
        menu::GroupedPopupKind::Overlay => String::new(),
    }
}

fn paint_vertical_menu(theme: &Theme, scheme: menu::MenuScheme) -> String {
    paint_vertical_menu_focus(
        theme,
        scheme,
        menu::GroupedPopupKind::Overlay,
        menu::MenuFocus::Rest,
        false,
        r#" data-menu="1" data-hero="menu""#,
    )
}

fn paint_vertical_menu_focus(
    theme: &Theme,
    scheme: menu::MenuScheme,
    kind: menu::GroupedPopupKind,
    focus: menu::MenuFocus,
    submenu_open: bool,
    stack_attrs: &str,
) -> String {
    let groups = kind.groups();
    let selected_index = kind.default_parent_hi();
    let mut stack = format!(
        r#"<div class="menu-stack" role="menu"{attrs} data-menu-scheme="{scheme}" data-menu-axis="vertical" data-menu-gap="{gap}" data-menu-focus="{focus}" data-menu-parent="1" data-popup-kind="{kind}">"#,
        attrs = stack_attrs,
        scheme = scheme.label(),
        gap = menu::GROUP_GAP_DP,
        focus = focus.label(),
        kind = kind.label(),
    );
    let mut flat = 0usize;
    for (gi, group) in groups.iter().enumerate() {
        let rest = menu::resolve_group(theme, scheme, gi, groups.len());
        let shell = menu::resolve_group_focus(theme, scheme, gi, groups.len(), focus);
        stack.push_str(&format!(
            r#"<div class="menu-group menu" data-menu-group="{gi}" data-menu-scheme="{scheme}" data-r-rest="{rest_r}" data-r-inactive="{inact}" style="background:{bg};border-radius:{r};box-shadow:{sh};padding:{pad}px">"#,
            scheme = scheme.label(),
            rest_r = rest.corners.css(),
            inact = crate::shape::Corners::all(menu::INACTIVE_CONTAINER_CORNER_DP).css(),
            bg = shell.container.css_hex(),
            r = shell.corners.css(),
            sh = ElevationLevels::css_shadow(shell.elevation_dp),
            pad = shell.pad_dp,
        ));
        for (i, item) in group.iter().enumerate() {
            let selected = flat == selected_index && !item.submenu;
            let state = if item.submenu && submenu_open {
                InteractionState::Hovered
            } else {
                InteractionState::Enabled
            };
            let a = menu::resolve_item_at(
                theme,
                scheme,
                menu::MenuAxis::Vertical,
                i,
                group.len(),
                selected,
                state,
            );
            let mut extra = if item.submenu {
                format!(
                    r#" data-submenu-trigger="1" aria-haspopup="menu" aria-expanded="{exp}""#,
                    exp = if submenu_open { "true" } else { "false" },
                )
            } else {
                overflow_leaf_extra(kind, item)
            };
            extra.push_str(&format!(r#" data-menu-flat="{flat}""#));
            stack.push_str(&paint_menu_item_row(item, &a, selected, &extra));
            flat += 1;
        }
        stack.push_str("</div>");
    }
    stack.push_str("</div>");
    stack
}

fn paint_grouped_overflow(
    theme: &Theme,
    kind: menu::GroupedPopupKind,
    visible: bool,
    class_name: &str,
    extra_attrs: &str,
) -> String {
    let scheme = menu::MenuScheme::Standard;
    let display = if visible { "flex" } else { "none" };
    format!(
        r#"<div class="{class}" role="menu" data-overflow-cascade="1" data-popup-kind="{kind}" data-open="{open}" data-flyout="0" data-typeahead="1" data-typeahead-autofocus="{af}" data-menu-keyboard="1" data-menu-gap="{gap}" data-hover-delay="{delay}" data-grouped="1"{extra} style="display:{display};gap:{gap}px">
  {parent}
  {flyout}
</div>"#,
        class = class_name,
        kind = kind.label(),
        open = if visible { "1" } else { "0" },
        af = if visible && menu::TYPEAHEAD_AUTOFOCUS {
            "1"
        } else {
            "0"
        },
        gap = menu::SUBMENU_GAP_DP,
        delay = menu::HOVER_OPEN_DELAY_MS,
        extra = extra_attrs,
        display = display,
        parent = paint_vertical_menu_focus(
            theme,
            scheme,
            kind,
            menu::MenuFocus::Rest,
            false,
            r#" data-menu-parent="1""#,
        ),
        flyout = paint_submenu_flyout(theme, scheme),
    )
}

fn paint_submenu_flyout(theme: &Theme, scheme: menu::MenuScheme) -> String {
    let shell = menu::resolve_submenu(theme, scheme);
    let count = menu::SUBMENU_ITEMS.len();
    let mut out = format!(
        r#"<div class="menu menu-flyout" role="menu" data-menu-submenu="1" data-menu-focus="active" data-menu-scheme="{scheme}" style="background:{bg};border-radius:{r};box-shadow:{sh};padding:{pad}px">"#,
        scheme = scheme.label(),
        bg = shell.container.css_hex(),
        r = shell.corners.css(),
        sh = ElevationLevels::css_shadow(shell.elevation_dp),
        pad = shell.pad_dp,
    );
    for (i, item) in menu::SUBMENU_ITEMS.iter().enumerate() {
        let selected = i == menu::SUBMENU_SELECTED;
        let a = menu::resolve_item_at(
            theme,
            scheme,
            menu::MenuAxis::Vertical,
            i,
            count,
            selected,
            InteractionState::Enabled,
        );
        out.push_str(&paint_menu_item_row(item, &a, selected, ""));
    }
    out.push_str("</div>");
    out
}

fn paint_submenu_cascade(theme: &Theme) -> String {
    let scheme = menu::MenuScheme::Standard;
    format!(
        r#"<div class="menu-cascade" data-hero="menu-submenu" data-menu-cascade="1" data-open="{open}" data-typeahead="1" data-typeahead-autofocus="{af}" data-menu-keyboard="1" data-menu-gap="{gap}" data-hover-delay="{delay}" tabindex="0" style="gap:{gap}px">
  {parent}
  {flyout}
</div>"#,
        open = if menu::CASCADE_OPEN { "1" } else { "0" },
        af = if menu::typeahead_autofocus_in_page(menu::CASCADE_OPEN) {
            "1"
        } else {
            "0"
        },
        gap = menu::SUBMENU_GAP_DP,
        delay = menu::HOVER_OPEN_DELAY_MS,
        parent = paint_vertical_menu_focus(
            theme,
            scheme,
            menu::GroupedPopupKind::Overlay,
            menu::MenuFocus::Inactive,
            true,
            "",
        ),
        flyout = paint_submenu_flyout(theme, scheme),
    )
}

fn paint_overlay_menu(theme: &Theme) -> String {
    let scheme = menu::MenuScheme::Standard;
    let anchor = button::resolve(
        theme,
        button::ButtonVariant::Tonal,
        InteractionState::Enabled,
    );
    format!(
        r#"<div class="menu-anchor-stage" data-hero="menu-overlay" data-menu-overlay-stage="1" data-scrim="{scrim}" data-anchored="1">
  <button class="btn" data-menu-anchor="1" data-menu-anchor-label="{label}" style="background:{abg};color:{afg};border:none;min-width:64px;height:{ah}px;padding:0 16px;border-radius:{ar}px;font-size:14px">{label}</button>
  <div class="menu-overlay" data-hero="menu-overlay" data-menu-overlay="1" data-open="{open}" data-typeahead="1" data-menu-keyboard="1" data-menu-gap="{gap}" data-hover-delay="{delay}" data-overlay-flyout="1" data-anchored="1" data-scrim="{scrim}" style="gap:{gap}px">
  {parent}
  {flyout}
</div>
</div>"#,
        open = if menu::OVERLAY_FLYOUT_OPEN { "1" } else { "0" },
        gap = menu::SUBMENU_GAP_DP,
        delay = menu::HOVER_OPEN_DELAY_MS,
        scrim = if menu::OVERLAY_USES_SCRIM { "1" } else { "0" },
        label = menu::OVERLAY_ANCHOR_LABEL,
        abg = anchor.container.css_hex(),
        afg = anchor.content.css_hex(),
        ah = anchor.height_dp,
        ar = anchor.corners.top_left,
        parent = paint_vertical_menu_focus(
            theme,
            scheme,
            menu::GroupedPopupKind::Overlay,
            menu::MenuFocus::Rest,
            false,
            r#" data-menu-parent="1""#,
        ),
        flyout = paint_submenu_flyout(theme, scheme),
    )
}

fn paint_horizontal_menu(theme: &Theme) -> String {
    let scheme = menu::MenuScheme::Standard;
    let shell = menu::resolve_container(theme, scheme);
    let count = menu::HORIZONTAL_LABELS.len();
    let mut out = format!(
        r#"<div class="menu menu-horizontal" data-menu-axis="horizontal" data-hero="menu-horizontal" data-menu-scheme="standard" style="background:{bg};border-radius:{r};box-shadow:{sh}">"#,
        bg = shell.container.css_hex(),
        r = shell.corners.css(),
        sh = ElevationLevels::css_shadow(shell.elevation_dp),
    );
    for (i, label) in menu::HORIZONTAL_LABELS.iter().enumerate() {
        let selected = i == menu::HORIZONTAL_SELECTED;
        let a =
            menu::resolve_horizontal(theme, scheme, i, count, selected, InteractionState::Enabled);
        out.push_str(&format!(
            r#"<div class="menu-item" data-menu-h="{label}" data-selected="{sel}" style="background:{bg};color:{fg};height:{h}px;border-radius:{r};padding:0 {pad}px">{label}</div>"#,
            sel = selected,
            bg = a.container.css_hex(),
            fg = a.label.css_hex(),
            h = a.height_dp,
            r = a.corners.css(),
            pad = a.pad_h_dp,
        ));
    }
    out.push_str("</div>");
    out
}

fn paint_horizontal_icons(theme: &Theme) -> String {
    let scheme = menu::MenuScheme::Standard;
    let shell = menu::resolve_container(theme, scheme);
    let count = menu::HORIZONTAL_ICONS.len();
    let mut out = format!(
        r#"<div class="menu menu-icons" data-menu-axis="horizontal-icon" data-hero="menu-icons" data-menu-icon-gap="{gap}" style="background:{bg};border-radius:{r};box-shadow:{sh}">"#,
        gap = menu::HORIZONTAL_ICON_GAP_DP,
        bg = shell.container.css_hex(),
        r = shell.corners.css(),
        sh = ElevationLevels::css_shadow(shell.elevation_dp),
    );
    for (i, glyph) in menu::HORIZONTAL_ICONS.iter().enumerate() {
        let selected = i == menu::HORIZONTAL_ICON_SELECTED;
        let a = menu::resolve_horizontal_icon(theme, scheme, i, count, selected);
        out.push_str(&format!(
            r#"<div class="menu-item" data-menu-icon="{glyph}" data-selected="{sel}" style="background:{bg};color:{fg};width:{s}px;height:{s}px;border-radius:{r}">{glyph}</div>"#,
            sel = selected,
            bg = a.container.css_hex(),
            fg = a.label.css_hex(),
            s = a.height_dp,
            r = a.corners.css(),
        ));
    }
    out.push_str("</div>");
    out
}

fn menus(theme: &Theme) -> String {
    format!(
        r#"<h2>Menu</h2>
<p class="note">M3 Expressive vertical menus (I/O 2026): standard surface-container-low / vibrant tertiary-container, corner-large 16, elev 2, 44dp items, grouped 2dp gap. Selected uses tertiary-container (standard) or tertiary (vibrant) + corner-medium. Nested submenu flies out at MenuAnchorPosition.End; focused ActiveContainerShape 24, parent InactiveContainerShape 8. Overlay menus are unscrimmed popups next to a Menu anchor; overflow and split trailing menus use the same grouped 2dp shell + More flyout (not a single 16dp surface). More hover opens the End flyout after 200ms on catalog JS and GPUI hosts (click/keyboard stay immediate; More does not dismiss). Hover-open + WAI-ARIA typeahead (cascade autofocus when overflow/split/overlay opens and on the in-page submenu). Horizontal 2dp pills go full-round when selected. <a href="https://m3.material.io/components/menus/specs">spec</a></p>
<div class="hero-card" data-hero="menu">
  <div class="menu-row">
    {standard}
    {vibrant}
  </div>
  <div class="menu-row">
    {horizontal}
    {icons}
  </div>
</div>
<div class="hero-card" data-hero="menu-submenu">
  {cascade}
</div>
<div class="hero-card">
  {overlay}
</div>"#,
        standard = paint_vertical_menu(theme, menu::MenuScheme::Standard),
        vibrant = paint_vertical_menu(theme, menu::MenuScheme::Vibrant),
        horizontal = paint_horizontal_menu(theme),
        icons = paint_horizontal_icons(theme),
        cascade = paint_submenu_cascade(theme),
        overlay = paint_overlay_menu(theme),
    )
}

fn paint_slider_stops(a: &slider::SliderAppearance) -> String {
    slider::stop_fractions(a.stop_count)
        .into_iter()
        .map(|frac| {
            let color = if frac <= a.value + 0.001 {
                a.stop_active.css_hex()
            } else {
                a.stop_inactive.css_hex()
            };
            format!(r#"<span class="xstop" style="background:{color}"></span>"#)
        })
        .collect()
}

fn paint_expressive_slider(a: &slider::SliderAppearance, label: &str) -> String {
    let active_pct = (a.value * 42.0).max(8.0);
    let stops = if a.stop_count > 2 {
        format!(r#"<div class="xstops">{}</div>"#, paint_slider_stops(a))
    } else {
        String::new()
    };
    format!(
        r#"<div class="xslider" data-slider="{label}" data-value="{value}" data-stops="{n}" data-handle-visual="{hv}" data-hero-slider="1">
  <div class="xseg active" style="width:{aw}%;height:{th}px;background:{active};border-radius:{oc}px {ic}px {ic}px {oc}px"></div>
  <div class="xhandle" style="width:{hw}px;height:{hh}px;background:{handle};margin:0 {gap}px"></div>
  <div class="xseg inactive" style="flex:1;height:{th}px;background:{inactive};border-radius:{ic}px {oc}px {oc}px {ic}px"></div>
  {stops}
</div>"#,
        value = a.value,
        n = a.stop_count,
        hv = a.handle_h_visual,
        aw = active_pct,
        th = a.track_h,
        active = a.active.css_hex(),
        oc = a.track_corner,
        ic = a.inner_corner,
        hw = a.handle_w,
        hh = a.handle_h_visual,
        handle = a.handle.css_hex(),
        gap = a.gap_dp,
        inactive = a.inactive.css_hex(),
        stops = stops,
    )
}

fn paint_range_slider(a: &slider::RangeSliderAppearance, label: &str) -> String {
    let t = &a.track;
    let left = (a.start * 42.0).max(6.0);
    let mid = ((a.end - a.start) * 42.0).max(8.0);
    let ticks: String = slider::range_tick_fractions()
        .into_iter()
        .map(|frac| {
            let active = slider::range_tick_active(frac, a.start, a.end);
            let color = if active {
                t.stop_active.css_hex()
            } else {
                t.stop_inactive.css_hex()
            };
            format!(
                r#"<i class="xtick" data-range-tick="{frac:.2}" style="left:{left}%;background:{color}"></i>"#,
                left = frac * 100.0,
                color = color,
                frac = frac,
            )
        })
        .collect();
    format!(
        r#"<div class="slider-row" data-slider-range="1" data-start="{start}" data-end="{end}" data-range-interactive="1">
  <div class="slider-meta"><div class="slider-label">{label}</div>
  <div class="xslider" data-slider="{label}" data-handle-visual="{hv}">
    <div class="xseg inactive" style="width:{lw}%;height:{th}px;background:{inactive};border-radius:{oc}px {ic}px {ic}px {oc}px"></div>
    <div class="xhandle" style="width:{hw}px;height:{hh}px;background:{handle};margin:0 {gap}px"></div>
    <div class="xseg active" style="width:{mw}%;height:{th}px;background:{active};border-radius:{ic}px"></div>
    <div class="xhandle" style="width:{hw}px;height:{hh}px;background:{handle};margin:0 {gap}px"></div>
    <div class="xseg inactive" style="flex:1;height:{th}px;background:{inactive};border-radius:{ic}px {oc}px {oc}px {ic}px"></div>
    <div class="xstops" data-range-ticks="1">{ticks}</div>
  </div></div>
</div>"#,
        start = a.start,
        end = a.end,
        label = label,
        hv = t.handle_h_visual,
        lw = left,
        mw = mid,
        th = t.track_h,
        inactive = t.inactive.css_hex(),
        active = t.active.css_hex(),
        oc = t.track_corner,
        ic = t.inner_corner,
        hw = t.handle_w,
        hh = t.handle_h_visual,
        handle = t.handle.css_hex(),
        gap = t.gap_dp,
    )
}

fn sliders(theme: &Theme) -> String {
    let mut out = String::from(
        "<h2>Slider</h2><p class=\"note\">M3 Expressive (current site): thick track + 4×44 vertical handle, 6dp gap, 4dp stops. Overview scene is volume rows; Alarm has mid-track stops. <a href=\"https://m3.material.io/components/sliders/specs\">spec</a></p>",
    );
    out.push_str("<div class=\"hero-card\" data-hero=\"slider\">");
    for row in slider::OVERVIEW_ROWS {
        let a =
            slider::resolve_with_stops(theme, row.value, InteractionState::Enabled, row.stop_count);
        out.push_str(&format!(
            r#"<div class="slider-row" data-slider-row="{label}"><div class="slider-icon" aria-hidden="true">{icon}</div><div class="slider-meta"><div class="slider-label">{label}</div>{}</div></div>"#,
            paint_expressive_slider(&a, row.label),
            label = row.label,
            icon = row.icon,
        ));
    }
    out.push_str("<p class=\"note\">XS 16dp track · painted handle track+12 (~28) · token handle 44 · Alarm unified mid-stops from resolve_with_stops(). Range: pointer-drag thumbs, keyboard arrows, 5% click-step.</p></div>");
    let range = slider::resolve_range(
        theme,
        slider::RANGE_DEMO_START,
        slider::RANGE_DEMO_END,
        InteractionState::Enabled,
    );
    out.push_str("<h3>range</h3>");
    out.push_str(&paint_range_slider(&range, slider::RANGE_HERO_LABEL));
    for (label, value, state) in [
        ("0.3 enabled", 0.3, InteractionState::Enabled),
        ("0.7 pressed", 0.7, InteractionState::Pressed),
        ("0.5 disabled", 0.5, InteractionState::Disabled),
    ] {
        let a = slider::resolve(theme, value, state);
        out.push_str(&state_row_open(label));
        out.push_str(&paint_expressive_slider(&a, label));
        out.push_str("</div></div>");
    }
    out.push_str("<h3>sizes</h3><div class=\"state-body\" style=\"flex-direction:column;align-items:stretch\">");
    for size in slider::SliderSize::ALL {
        let a = slider::resolve_size(theme, size, 0.45, InteractionState::Enabled);
        out.push_str(&format!(
            "<div><div class=\"state-name\">{}</div>{}</div>",
            size.label(),
            paint_expressive_slider(&a, size.label())
        ));
    }
    out.push_str("</div>");
    out
}

fn tabs_section(theme: &Theme) -> String {
    let mut out = String::from("<h2>Tabs</h2>");
    for variant in [tabs::TabsVariant::Primary, tabs::TabsVariant::Secondary] {
        let a = tabs::resolve(theme, variant);
        let labels = ["Tab one", "Tab two", "Tab three"];
        let mut row = String::new();
        for (i, label) in labels.iter().enumerate() {
            let active = i == 0;
            let color = if active {
                a.active_label
            } else {
                a.inactive_label
            };
            let ind_w = if a.indicator_full_width {
                "100%"
            } else {
                "48px"
            };
            let ind = if active {
                format!(
                    "<div class=\"tab-ind\" style=\"width:{ind_w};height:{h}px;background:{c}\"></div>",
                    h = a.indicator_h,
                    c = a.indicator.css_hex(),
                )
            } else {
                String::new()
            };
            row.push_str(&format!(
                "<div class=\"tab\" data-tab=\"{v}\" data-active=\"{active}\" style=\"color:{color};height:{h}px\">{label}{ind}</div>",
                v = variant.label(),
                color = color.css_hex(),
                h = a.height_dp,
            ));
        }
        out.push_str(&format!(
            "<h3>{}</h3><div class=\"tabs\" data-tabs=\"{}\" style=\"background:{}\">{row}</div>",
            variant.label(),
            variant.label(),
            a.container.css_hex(),
        ));
    }
    let icons = tabs::resolve_with_icons(theme, tabs::TabsVariant::Primary);
    let mut icon_row = String::new();
    for (i, (icon, label)) in tabs::DEMO_ICONS
        .iter()
        .zip(tabs::DEMO_ICON_LABELS.iter())
        .enumerate()
    {
        let active = i == 0;
        let color = if active {
            icons.active_label
        } else {
            icons.inactive_label
        };
        let ind = if active {
            format!(
                "<div class=\"tab-ind\" style=\"width:48px;height:{h}px;background:{c}\"></div>",
                h = icons.indicator_h,
                c = icons.indicator.css_hex(),
            )
        } else {
            String::new()
        };
        icon_row.push_str(&format!(
            "<div class=\"tab\" data-tab=\"primary-icon\" data-active=\"{active}\" style=\"color:{color};height:{h}px;flex-direction:column\"><span>{icon}</span><span>{label}</span>{ind}</div>",
            color = color.css_hex(),
            h = icons.height_dp,
        ));
    }
    out.push_str(&format!(
        "<h3>primary with icons</h3><div class=\"tabs\" data-tabs=\"primary-icons\" style=\"background:{}\">{icon_row}</div>",
        icons.container.css_hex(),
    ));
    let scene = tabs::resolve_with_icons(theme, tabs::TabsVariant::Primary);
    let mut scene_tabs = String::new();
    for (i, (icon, label)) in tabs::SCENE_ICONS
        .iter()
        .zip(tabs::SCENE_LABELS.iter())
        .enumerate()
    {
        let active = i == tabs::SCENE_SELECTED;
        let color = if active {
            scene.active_label
        } else {
            scene.inactive_label
        };
        let ind = format!(
            "<div class=\"tab-ind\" style=\"width:48px;height:{h}px;background:{c};display:{d}\"></div>",
            h = scene.indicator_h,
            c = scene.indicator.css_hex(),
            d = if active { "block" } else { "none" },
        );
        scene_tabs.push_str(&format!(
            r#"<div class="tab" data-media-tab="{i}" data-active="{active}" data-idle-fg="{idle}" data-sel-fg="{sel}" style="color:{color};height:{h}px;flex-direction:column"><span>{icon}</span><span>{label}</span>{ind}</div>"#,
            idle = scene.inactive_label.css_hex(),
            sel = scene.active_label.css_hex(),
            color = color.css_hex(),
            h = scene.height_dp,
        ));
    }
    let mut tiles = String::new();
    for (i, caption) in tabs::SCENE_TILES.iter().enumerate() {
        let kind = tabs::scene_tile_kind(i);
        tiles.push_str(&format!(
            r#"<div class="media-tile photo-stub" data-media-tile="{i}" data-photo="{photo}" data-caption="{caption}" data-decoded-jpeg="1" data-licensed-camera="1" style="background:{css};height:{h}px;border-radius:{r}px"></div>"#,
            photo = kind.label(),
            css = kind.css_background(),
            h = tabs::SCENE_TILE_H_DP,
            r = tabs::SCENE_TILE_CORNER_DP,
        ));
    }
    let trailing: String = tabs::SCENE_TRAILING
        .iter()
        .map(|g| format!(r#"<span data-media-action="{g}">{g}</span>"#))
        .collect();
    out.push_str(&format!(
        r#"<h3>saved media</h3>
<div class="phone" data-media-scene="1" data-hero="tabs" style="min-height:{ph}px">
  <div class="status-bar" data-status-bar="1"><span>{stime}</span><span>5G · 100%</span></div>
  <div class="phone-bar" style="display:flex;align-items:center;gap:12px">
    <span data-media-back="1">{back}</span>
    <span style="flex:1">{title}</span>
    {trailing}
  </div>
  <div class="tabs" data-tabs="saved-media" style="background:{bg};max-width:none">{tabs}</div>
  <div class="media-grid">{tiles}</div>
</div>"#,
        ph = tabs::PHONE_H_DP,
        stime = tabs::STATUS_TIME,
        back = tabs::SCENE_LEADING,
        title = tabs::SCENE_TITLE,
        trailing = trailing,
        bg = scene.container.css_hex(),
        tabs = scene_tabs,
        tiles = tiles,
    ));
    out
}

fn badges(theme: &Theme) -> String {
    let small = badge::resolve(theme, badge::BadgeKind::Small);
    let large = badge::resolve(theme, badge::BadgeKind::Large);
    let icon_bg = theme.color.surface_container_highest.css_hex();
    format!(
        r#"<h2>Badge</h2>
<p class="note">Small 6dp / large 16dp · error / on-error · 999+.</p>
<div class="state-body">
  <div class="badge-wrap" data-badge="small" style="background:{icon_bg};border-radius:20px">★<div class="badge small" style="background:{sbg}"></div></div>
  <div class="badge-wrap" data-badge="large" style="background:{icon_bg};border-radius:20px">★<div class="badge" style="background:{lbg};color:{lfg}">{count}</div></div>
  <div class="badge-wrap" data-badge="overflow" style="background:{icon_bg};border-radius:20px">★<div class="badge" style="background:{lbg};color:{lfg}">{overflow}</div></div>
</div>"#,
        sbg = small.container.css_hex(),
        lbg = large.container.css_hex(),
        lfg = large.label.css_hex(),
        count = badge::label_for_count(8),
        overflow = badge::label_for_count(1200),
    )
}

fn paint_date_grid(
    a: &date_picker::DatePickerAppearance,
    cells: [(u32, date_picker::DayKind); 42],
) -> String {
    let mut grid = String::new();
    for (day, kind) in cells {
        let radius = if kind == date_picker::DayKind::InRange {
            "0"
        } else {
            "20px"
        };
        let (bg, fg, outline) = match kind {
            date_picker::DayKind::Selected => (
                a.day_selected_container.css_hex(),
                a.day_selected.css_hex(),
                "none".into(),
            ),
            date_picker::DayKind::InRange => (
                a.day_range_container.css_hex(),
                a.day_range.css_hex(),
                "none".into(),
            ),
            date_picker::DayKind::Today => (
                "transparent".into(),
                a.day.css_hex(),
                format!("1px solid {}", a.day_today_outline.css_hex()),
            ),
            date_picker::DayKind::InMonth => ("transparent".into(), a.day.css_hex(), "none".into()),
            date_picker::DayKind::OutOfMonth => {
                ("transparent".into(), a.day_out.css_hex(), "none".into())
            }
        };
        grid.push_str(&format!(
            "<div class=\"day\" data-day=\"{day}\" data-kind=\"{kind:?}\" style=\"background:{bg};color:{fg};border:{outline};border-radius:{radius}\">{day}</div>"
        ));
    }
    grid
}

fn date_pickers(theme: &Theme) -> String {
    let a = date_picker::resolve(theme);
    let today = date_picker::CivilDate {
        year: 2026,
        month: 9,
        day: 11,
    };
    let selected = date_picker::CivilDate {
        year: 2026,
        month: 9,
        day: 15,
    };
    let cells = date_picker::month_grid_classified(2026, 9, selected, today);
    let range_cells = date_picker::month_grid_range(
        date_picker::RANGE_DEMO_START.year,
        date_picker::RANGE_DEMO_START.month,
        date_picker::RANGE_DEMO_START,
        date_picker::RANGE_DEMO_END,
        today,
    );
    let mut week = String::new();
    for d in date_picker::WEEKDAYS {
        week.push_str(&format!(
            "<div class=\"day\" style=\"color:{}\">{}</div>",
            a.weekday.css_hex(),
            d
        ));
    }
    let grid = paint_date_grid(&a, cells);
    let range_grid = paint_date_grid(&a, range_cells);
    format!(
        r#"<h2>Date picker</h2>
<p class="note">Official modal: “Select date” + headlineLargeEmphasized + Sunday-first 7-column grid (matches live m3.material.io modal, not ISO Monday-first). Overview range hero uses InRange fill. Docked popup anchors under the outlined field with elevation shadow, month navigation, and outside-click dismiss. 40dp cells. <a href="https://m3.material.io/components/date-pickers/overview">overview</a></p>
<div class="cal dialog" data-datepicker-range="1" data-hero="datepicker-range" data-week-start="sunday" style="background:{bg};border-radius:{r}px;box-shadow:{sh};margin-bottom:16px">
  <div class="head">
    <div style="color:{hy};font-size:{ys}px">{range_title}</div>
    <div style="color:{hd};font-size:{ds}px;font-weight:{dw}">{range_headline}</div>
    <div style="color:{hy};font-size:{ys}px;margin-top:8px">{range_month}</div>
  </div>
  <div class="week">{week}</div>
  <div class="grid">{range_grid}</div>
</div>
<div class="cal dialog" data-datepicker="1" data-hero="datepicker" data-week-start="sunday" style="background:{bg};border-radius:{r}px;box-shadow:{sh};margin-bottom:16px">
  <div class="head">
    <div style="color:{hy};font-size:{ys}px">Select date</div>
    <div style="color:{hd};font-size:{ds}px;font-weight:{dw}">{headline}</div>
  </div>
  <div style="text-align:center;padding:8px;font-weight:500">{month}</div>
  <div class="week">{week}</div>
  <div class="grid">{grid}</div>
  <div class="actions" style="padding:8px 12px 0">
    <button class="btn" style="background:transparent;color:{act}">Cancel</button>
    <button class="btn" style="background:transparent;color:{act}">OK</button>
  </div>
</div>
<div class="docked" data-datepicker-docked="1" data-hero="datepicker-docked" data-popup="open" data-dismiss-outside="1" data-year="2026" data-month="9" data-selected-year="2026" data-selected-month="9" data-selected-day="15" data-today-year="2026" data-today-month="9" data-today-day="11" data-day-sel-bg="{selbg}" data-day-sel-fg="{selfg}" data-day-today="{todaybd}" data-day-in="{infg}" data-day-out="{outfg}">
  {docked_field}
  <div class="cal dialog" data-datepicker-popup="open" style="background:{bg};border-radius:8px {r}px {r}px {r}px;box-shadow:{sh};margin-top:4px;width:100%">
    <div class="month-nav">
      <button type="button" data-docked-month="-1" aria-label="Previous month">&lt;</button>
      <div data-docked-month-label="1">{month}</div>
      <button type="button" data-docked-month="1" aria-label="Next month">&gt;</button>
    </div>
    <div class="week">{week}</div>
    <div class="grid" data-docked-grid="1">{grid}</div>
  </div>
</div>"#,
        bg = a.container.css_hex(),
        r = a.corners.top_left,
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        hy = a.header_year.css_hex(),
        ys = a.year_style.size_sp,
        hd = a.header_date.css_hex(),
        ds = a.date_style.size_sp,
        dw = a.date_style.weight,
        headline = date_picker::header_date_label(selected),
        month = date_picker::month_nav_label(2026, 9),
        range_title = date_picker::RANGE_HERO_TITLE,
        range_headline = date_picker::header_range_label(
            date_picker::RANGE_DEMO_START,
            date_picker::RANGE_DEMO_END
        ),
        range_month = date_picker::month_nav_label(
            date_picker::RANGE_DEMO_START.year,
            date_picker::RANGE_DEMO_START.month
        ),
        act = theme.color.primary.css_hex(),
        selbg = a.day_selected_container.css_hex(),
        selfg = a.day_selected.css_hex(),
        todaybd = a.day_today_outline.css_hex(),
        infg = a.day.css_hex(),
        outfg = a.day_out.css_hex(),
        docked_field = paint_outlined_field(
            &text_field::resolve(
                theme,
                text_field::TextFieldVariant::Outlined,
                InteractionState::Enabled,
                true,
            ),
            "data-field-hero=\"docked-date\"",
            date_picker::DOCKED_FIELD_LABEL,
            &format!(
                r#"<div class="val">{}</div>"#,
                date_picker::docked_field_value(selected)
            ),
        ),
    )
}

fn paint_contained_search(
    theme: &Theme,
    width_class: search::WindowWidthClass,
    hero: bool,
) -> String {
    let bar = search::resolve(theme);
    let contained = search::resolve_view(theme);
    let layout = width_class.expanded_search();
    let focused =
        search::contained_frame_at_layout(layout, 1.0, search::contained_suggestion_count());
    let mut rows = String::new();
    for (i, label) in search::SUGGESTIONS.iter().enumerate() {
        rows.push_str(&format!(
            r#"<div class="sv-row" data-search-suggestion="{label}" style="color:{fg};height:{h}px"><span style="color:{ico}">{icon}</span><span>{label}</span></div>"#,
            fg = contained.suggestion.css_hex(),
            h = contained.suggestion_h_dp,
            ico = contained.suggestion_icon.css_hex(),
            icon = if i == 0 { "⌕" } else { "◌" },
        ));
    }
    format!(
        r#"<div class="search-morph" data-search="1" data-search-view="1" data-search-style="contained" data-width-class="{wc}" data-search-expanded="{layout}" data-search-activity="1" data-search-morph="1" data-search-shared="1" data-open="1" data-search-scale="1" data-search-path-scale="1" data-search-layer-box="1" data-search-anim-scale="1" data-search-transform-origin="top center"{hero_attr} style="background:{cbg};border-radius:{cr}px;min-height:{mh}px;margin:{mg}px">
  <div class="sv-head" style="height:{vh}px;color:{vfg}">
    <div class="lead" data-search-lead="1">
      <span class="lead-docked" aria-hidden="true">{lead}</span>
      <span class="lead-activity" aria-hidden="true">{back}</span>
    </div>
    <input class="hint" data-search-input="1" placeholder="{placeholder}" style="color:{vph}"/>
    <div class="ico">{mic}</div>
    <div class="avatar" data-search-avatar="1" style="background:{abg};color:{afg}">A</div>
  </div>
  <div class="sv-list">{rows}</div>
</div>"#,
        wc = width_class.label(),
        layout = layout.label(),
        hero_attr = if hero { r#" data-hero="search""# } else { "" },
        cbg = search::contained_container(theme).css_hex(),
        cr = focused.corner_dp,
        mh = focused.height_dp,
        mg = focused.margin_dp,
        vh = focused.header_h_dp,
        vfg = contained.header.css_hex(),
        vph = contained.placeholder.css_hex(),
        back = search::VIEW_BACK,
        lead = search::LEADING_ICON,
        abg = bar.avatar.css_hex(),
        afg = bar.avatar_label.css_hex(),
        placeholder = search::PLACEHOLDER,
        mic = search::TRAILING_MIC,
        rows = rows,
    )
}

fn search_section(theme: &Theme) -> String {
    let activity = search::resolve_activity(theme);
    let compact = paint_contained_search(theme, search::WindowWidthClass::Compact, true);
    let docked = paint_contained_search(theme, search::WindowWidthClass::Medium, false);
    format!(
        r#"<h2>Search</h2>
<p class="note">Expressive (recommended): contained search. Compact (<code>&lt; 600dp</code>) expands to full-screen (0 margin / 0 corner). Medium+ docked keeps Corner 28 + 24→12dp margin, no divider. Divided activity remains below. Type to filter suggestions. <a href="https://m3.material.io/components/search/specs">spec</a></p>
{compact}
<h3>medium docked (≥600dp)</h3>
<p class="note">Compose <code>ExpandedDockedSearchBar</code>: persistent filled container, Corner 28 stays, 24→12dp margin.</p>
{docked}
<h3>divided (baseline)</h3>
<p class="note">Not recommended. Divider + full-screen activity flatten (0dp corners).</p>
<div class="search-morph" data-search-style="divided" data-search-view="1" data-search-activity="1" data-open="1" style="background:{abg2};border-radius:{ar}px;min-height:{amh}px">
  <div class="sv-head" style="height:{ah}px;color:{afg2}">
    <div class="lead"><span class="lead-activity">{back}</span></div>
    <input class="hint" placeholder="{placeholder}" style="color:{aph}"/>
  </div>
  <div class="sv-divider" style="height:1px;background:{vdiv}"></div>
</div>"#,
        compact = compact,
        docked = docked,
        back = search::VIEW_BACK,
        placeholder = search::PLACEHOLDER,
        abg2 = activity.container.css_hex(),
        ar = activity.corners.top_left,
        amh = search::ACTIVITY_MIN_H_DP,
        ah = activity.header_h_dp,
        afg2 = activity.header.css_hex(),
        aph = activity.placeholder.css_hex(),
        vdiv = activity.divider.css_hex(),
    )
}

fn paint_scroll_field(
    field: time_picker::ScrollField,
    a: &time_picker::TimeScrollAppearance,
) -> String {
    let mut items = String::new();
    for (i, slot) in field.slots().into_iter().enumerate() {
        let rel = i as i32 - time_picker::SCROLL_SLOT_SPAN;
        let (color, size, weight) = if slot.selected {
            (
                a.selected.css_hex(),
                a.selected_style.size_sp,
                a.selected_style.weight,
            )
        } else {
            (
                a.unselected.css_hex(),
                a.unselected_style.size_sp,
                a.unselected_style.weight,
            )
        };
        items.push_str(&format!(
            r#"<div class="scroll-item" data-rel="{rel}" data-index="{idx}" data-value="{val}" data-selected="{sel}" style="top:{y}px;height:{ih}px;color:{color};font-size:{size}px;font-weight:{weight};opacity:{op}">{label}</div>"#,
            rel = rel,
            idx = slot.index,
            val = slot.value,
            sel = slot.selected as u8,
            y = slot.y_dp,
            ih = a.item_h_dp,
            color = color,
            size = size,
            weight = weight,
            op = slot.opacity,
            label = slot.label,
        ));
    }
    format!(
        r#"<div class="scroll-field" data-scroll-field="{kind}" data-count="{count}" data-offset="{off}" data-field-h="{h}" style="width:{w}px;height:{h}px;background:{bg};border-radius:{r}px">{items}</div>"#,
        kind = field.kind.label(),
        count = field.count(),
        off = field.offset,
        w = a.field_w_dp,
        h = a.field_h_dp,
        bg = a.field_container.css_hex(),
        r = a.field_corners.top_left,
        items = items,
    )
}

fn time_picker_section(theme: &Theme) -> String {
    let scroll = time_picker::resolve_scroll(theme);
    let input = time_picker::resolve_input(theme);
    let state = time_picker::TimeScrollState::demo();
    let hour_field = paint_scroll_field(state.hour, &scroll);
    let minute_field = paint_scroll_field(state.minute, &scroll);
    let input_state = time_picker::TimeInputState::demo();
    let (sam_bg, sam_fg) = if time_picker::DEMO_PERIOD == time_picker::DayPeriod::Am {
        (
            scroll.period_selected_container.css_hex(),
            scroll.period_selected.css_hex(),
        )
    } else {
        (
            scroll.period_idle_container.css_hex(),
            scroll.period_idle.css_hex(),
        )
    };
    let (spm_bg, spm_fg) = if time_picker::DEMO_PERIOD == time_picker::DayPeriod::Pm {
        (
            scroll.period_selected_container.css_hex(),
            scroll.period_selected.css_hex(),
        )
    } else {
        (
            scroll.period_idle_container.css_hex(),
            scroll.period_idle.css_hex(),
        )
    };
    let (iam_bg, iam_fg) = if time_picker::DEMO_PERIOD == time_picker::DayPeriod::Am {
        (
            input.period_selected_container.css_hex(),
            input.period_selected.css_hex(),
        )
    } else {
        (
            input.period_idle_container.css_hex(),
            input.period_idle.css_hex(),
        )
    };
    let (ipm_bg, ipm_fg) = if time_picker::DEMO_PERIOD == time_picker::DayPeriod::Pm {
        (
            input.period_selected_container.css_hex(),
            input.period_selected.css_hex(),
        )
    } else {
        (
            input.period_idle_container.css_hex(),
            input.period_idle.css_hex(),
        )
    };
    let mode = time_picker::DEMO_DISPLAY_MODE;
    let format = time_picker::DEMO_FORMAT;
    let mut out = format!(
        r#"<h2>Time picker</h2>
<p class="note">Expressive (recommended): Compose <code>TimeScroll</code> + two <code>ScrollField</code>s (200dp / 3-item wrap, Corner 28) + <code>vibrantColors()</code> primaryContainer. <code>TimeInput</code> 96×72 + <code>ScrollDisplayModeToggle</code> (⌨/◷). 24-hour (<code>is24Hour</code>) uses 00–23 and hides AM/PM. Dial remains below. <a href="https://m3.material.io/components/time-pickers/specs">spec</a></p>
<div class="time-expressive dialog" data-time-display="{mode}" data-time-format="{fmt}" data-hero="timepicker" style="background:{bg};border-radius:{r}px;box-shadow:{sh}">
  <div class="time-display-head">
    <div style="color:{hy};font-size:{ys}px">{title}</div>
    <div class="time-display-actions">
      <button class="time-format-toggle" data-time-format-toggle="1" data-time-format="{fmt}" title="{flabel}" style="color:{tg}">{ftext}</button>
      <button class="time-display-toggle" data-scroll-display-mode-toggle="1" data-display-mode="{mode}" title="{tlabel}" style="color:{tg}">{ticon}</button>
    </div>
  </div>
  <div class="time-scroll" data-time-scroll="1" data-time-picker-style="scroll" data-scroll-item-h="{ih}" data-scroll-fling-decay="{decay}" data-scroll-fling-rest="{rest}" data-scroll-snap="{snap}" data-hour="{hour}" data-minute="{minute}" data-period="{period}">
    <div class="scroll-row">
      {hour_field}
      <div class="scroll-colon" style="color:{colon};font-size:{cs}px;font-weight:{cw};margin-top:{cy}px">:</div>
      {minute_field}
      <div class="period">
        <button data-period="AM" style="background:{amb};color:{amf}">{am}</button>
        <button data-period="PM" style="background:{pmb};color:{pmf}">{pm}</button>
      </div>
    </div>
  </div>
  <div class="time-input" data-time-input="1" data-time-picker-style="input" data-hour="{hour}" data-minute="{minute}" data-period="{period}">
    <div class="time-input-row">
      <input class="time-input-field" data-time-input-field="hour" data-focused="1" maxlength="2" inputmode="numeric" value="{ihh}" style="background:{ifbg};color:{iffg};width:{ifw}px;height:{ifh}px;border-radius:{ifr}px"/>
      <div class="time-input-colon" style="color:{icolon};font-size:{ics}px;font-weight:{icw}">:</div>
      <input class="time-input-field" data-time-input-field="minute" maxlength="2" inputmode="numeric" value="{imm}" style="background:{imbg};color:{imfg};width:{ifw}px;height:{ifh}px;border-radius:{ifr}px"/>
      <div class="period">
        <button data-period="AM" style="background:{iamb};color:{iamf}">{am}</button>
        <button data-period="PM" style="background:{ipmb};color:{ipmf}">{pm}</button>
      </div>
    </div>
  </div>
</div>
<h3>dial</h3>"#,
        ih = time_picker::SCROLL_ITEM_H_DP,
        decay = time_picker::SCROLL_FLING_DECAY,
        rest = time_picker::SCROLL_FLING_REST,
        snap = time_picker::SCROLL_SNAP_STIFFNESS,
        hour = time_picker::demo_hour(format),
        minute = time_picker::DEMO_MINUTE,
        period = time_picker::DEMO_PERIOD.label(),
        fmt = format.label(),
        flabel = format.toggle_label(),
        ftext = format.toggle_text(),
        bg = scroll.container.css_hex(),
        r = scroll.corners.top_left,
        sh = ElevationLevels::css_shadow(scroll.elevation_dp),
        hy = scroll.header.css_hex(),
        ys = scroll.title_style.size_sp,
        title = time_picker::TITLE,
        hour_field = hour_field,
        minute_field = minute_field,
        colon = scroll.colon.css_hex(),
        cs = scroll.colon_style.size_sp,
        cw = scroll.colon_style.weight,
        cy = time_picker::SCROLL_COLON_OFFSET_Y_DP,
        amb = sam_bg,
        amf = sam_fg,
        pmb = spm_bg,
        pmf = spm_fg,
        am = time_picker::DayPeriod::Am.label(),
        pm = time_picker::DayPeriod::Pm.label(),
        mode = mode.label(),
        tlabel = mode.toggle_label(),
        tg = input.toggle.css_hex(),
        ticon = mode.toggle_icon(),
        ihh = input_state.hour.display(),
        imm = input_state.minute.display(),
        ifbg = input.field_focused.css_hex(),
        iffg = input.field_focused_content.css_hex(),
        imbg = input.field_container.css_hex(),
        imfg = input.field_content.css_hex(),
        ifw = input.field_w_dp,
        ifh = input.field_h_dp,
        ifr = input.field_corners.top_left,
        icolon = input.colon.css_hex(),
        ics = input.colon_style.size_sp,
        icw = input.colon_style.weight,
        iamb = iam_bg,
        iamf = iam_fg,
        ipmb = ipm_bg,
        ipmf = ipm_fg,
    );

    let a = time_picker::resolve(theme);
    let mut hours = String::new();
    for h in 1u8..=12 {
        let (x, y) = time_picker::hour_offset(h, a.clock_dp, a.number_dp);
        let selected = h == time_picker::DEMO_HOUR;
        let (bg, fg, fw) = if selected {
            (
                a.number_selected_container.css_hex(),
                a.number_selected.css_hex(),
                a.time_style.weight,
            )
        } else {
            (
                "transparent".into(),
                a.number.css_hex(),
                a.number_style.weight,
            )
        };
        hours.push_str(&format!(
            r#"<div class="hour" data-hour="{h}" data-selected="{sel}" style="display:none;left:{x}px;top:{y}px;width:{n}px;height:{n}px;background:{bg};color:{fg};font-weight:{fw}">{h}</div>"#,
            h = h,
            sel = selected as u8,
            x = x,
            y = y,
            n = a.number_dp,
            bg = bg,
            fg = fg,
            fw = fw,
        ));
    }
    let mut minutes = String::new();
    for m in time_picker::minute_labels() {
        let (x, y) = time_picker::minute_offset(m, a.clock_dp, a.number_dp);
        let selected = m == time_picker::DEMO_MINUTE;
        let (bg, fg, fw) = if selected {
            (
                a.number_selected_container.css_hex(),
                a.number_selected.css_hex(),
                a.time_style.weight,
            )
        } else {
            (
                "transparent".into(),
                a.number.css_hex(),
                a.number_style.weight,
            )
        };
        minutes.push_str(&format!(
            r#"<div class="minute" data-minute="{m}" data-selected="{sel}" style="left:{x}px;top:{y}px;width:{n}px;height:{n}px;background:{bg};color:{fg};font-weight:{fw}">{label}</div>"#,
            m = m,
            sel = selected as u8,
            x = x,
            y = y,
            n = a.number_dp,
            bg = bg,
            fg = fg,
            fw = fw,
            label = format!("{:02}", m),
        ));
    }
    let am = time_picker::DayPeriod::Am;
    let pm = time_picker::DayPeriod::Pm;
    let (am_bg, am_fg) = if time_picker::DEMO_PERIOD == am {
        (
            a.period_selected_container.css_hex(),
            a.period_selected.css_hex(),
        )
    } else {
        (a.period_idle_container.css_hex(), a.period_idle.css_hex())
    };
    let (pm_bg, pm_fg) = if time_picker::DEMO_PERIOD == pm {
        (
            a.period_selected_container.css_hex(),
            a.period_selected.css_hex(),
        )
    } else {
        (a.period_idle_container.css_hex(), a.period_idle.css_hex())
    };
    let hand_d = time_picker::hand_svg_d_at_angle(a.clock_dp, 0.0, a.number_dp);
    let second_d = time_picker::second_hand_svg_d(a.clock_dp, 0.0, a.number_dp);
    let hand_deg = time_picker::hand_angle_deg(
        time_picker::DEMO_DIAL,
        time_picker::DEMO_HOUR,
        time_picker::DEMO_MINUTE,
    );
    let hour_active = time_picker::DEMO_DIAL == time_picker::DialFace::Hour;
    out.push_str(&format!(
        r#"<p class="note">Baseline 12-hour + minute dial, analog selector hand, displaySmallEmphasized header, AM/PM. Header fields toggle the face.</p>
<div class="timepicker dialog" data-timepicker="1" data-dial="minute" data-hour="{hour}" data-minute="{minute}" data-period="{period}" style="background:{bg};border-radius:{r}px;box-shadow:{sh}">
  <div style="color:{hy};font-size:{ys}px">{title}</div>
  <div class="time-row">
    <div class="time-fields">
      <div class="time-field" data-time-field="hour" data-active="{ha}" style="font-size:{ds}px;font-weight:{dw};color:{hd};background:{clk}">{hh}</div>
      <div style="font-size:{ds}px;font-weight:{dw};color:{hd}">:</div>
      <div class="time-field" data-time-field="minute" data-active="{ma}" style="font-size:{ds}px;font-weight:{dw};color:{hd};background:{clk}">{mm}</div>
    </div>
    <div class="period">
      <button data-period="AM" style="background:{amb};color:{amf}">{am}</button>
      <button data-period="PM" style="background:{pmb};color:{pmf}">{pm}</button>
    </div>
  </div>
  <div class="clock" style="width:{clock}px;height:{clock}px;background:{clk}">
    <svg class="hand-svg" data-hand-path="1" viewBox="0 0 {clock} {clock}" aria-hidden="true" style="transform:rotate({hdeg}deg)">
      <path d="{handd}" fill="{hand}"/>
    </svg>
    <svg class="second-hand-svg" data-second-hand="1" data-second-wall="1" viewBox="0 0 {clock} {clock}" aria-hidden="true">
      <path d="{secondd}" fill="{hand}"/>
    </svg>
    <div class="hub" style="background:{hand}"></div>
    {hours}{minutes}
  </div>
</div>"#,
        hour = time_picker::DEMO_HOUR,
        minute = time_picker::DEMO_MINUTE,
        period = time_picker::DEMO_PERIOD.label(),
        bg = a.container.css_hex(),
        r = a.corners.top_left,
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        hy = a.header.css_hex(),
        ys = a.title_style.size_sp,
        title = time_picker::TITLE,
        hd = a.header.css_hex(),
        ds = a.time_style.size_sp,
        dw = a.time_style.weight,
        hh = time_picker::format_hour_field(time_picker::DEMO_HOUR),
        mm = time_picker::format_minute_field(time_picker::DEMO_MINUTE),
        ha = hour_active as u8,
        ma = (!hour_active) as u8,
        amb = am_bg,
        amf = am_fg,
        pmb = pm_bg,
        pmf = pm_fg,
        am = am.label(),
        pm = pm.label(),
        clock = a.clock_dp,
        clk = a.clock.css_hex(),
        hours = hours,
        minutes = minutes,
        hand = a.hand.css_hex(),
        handd = hand_d,
        secondd = second_d,
        hdeg = hand_deg,
    ));
    out
}

fn motion_section(theme: &Theme) -> String {
    format!(
        r#"<h2>Motion</h2>
<p class="note">Expressive spatial <code>{ease}</code> ({sd}ms) + effects springs. Button / connected-group press morph uses the same CSS transition. GPUI Animation clock drives indeterminate + wavy progress.</p>
<div class="motion-box" data-motion="emphasized" style="background:{p}"></div>"#,
        ease = theme.motion.spatial_fast,
        sd = theme.motion.spatial_fast_ms,
        p = theme.color.primary.css_hex(),
    )
}

fn paint_carousel_row(theme: &Theme, layout: carousel::CarouselLayout) -> String {
    let a = carousel::resolve(theme);
    let mut tiles = String::new();
    for (i, caption) in carousel::MEDIA_CAPTIONS.iter().enumerate() {
        let w = carousel::item_width_for(layout, i, carousel::DEMO_INDEX);
        let h = carousel::item_height_for_index(layout, i);
        let kind = carousel::media_kind(i);
        tiles.push_str(&format!(
            r#"<div class="tile photo-stub" data-carousel-item="{i}" data-media="1" data-photo="{photo}" data-caption="{caption}" data-decoded-jpeg="1" data-licensed-camera="1" data-parallax="{px}" style="width:{w}px;height:{h}px;background:{css};border-radius:{r}px"></div>"#,
            photo = kind.label(),
            px = carousel::PARALLAX_MAX_DP,
            css = kind.css_background(),
            r = a.corners.top_left,
        ));
    }
    let axis = match layout.axis() {
        carousel::CarouselAxis::Vertical => "vertical",
        carousel::CarouselAxis::Horizontal => "horizontal",
    };
    let centered = layout.center_aligned() as u8;
    let is_lists = layout.uses_lists_scene();
    let row = format!(
        r#"<div class="carousel" data-carousel="1" data-carousel-layout="{layout}" data-carousel-axis="{axis}" data-carousel-centered="{centered}" data-carousel-fling="1" data-carousel-live="1" data-carousel-snap="1" data-carousel-selected="0" data-carousel-media="1" data-hero="{hero}">{tiles}</div>"#,
        layout = layout.label(),
        hero = if is_lists { "carousel" } else { layout.label() },
        tiles = tiles,
        axis = axis,
        centered = centered,
    );
    if layout.uses_lists_scene() {
        let mut lists = String::new();
        for (icon, title, sub) in carousel::LISTS_ROWS {
            lists.push_str(&format!(
                r#"<div class="lists-row" data-lists-row="{title}"><span>{icon}</span><span class="lists-meta"><span>{title}</span><span class="lists-sub">{sub}</span></span></div>"#,
            ));
        }
        format!(
            r#"<div class="phone lists-scene" data-carousel-lists="1" data-carousel-phone="1" data-hero="carousel" style="height:{fh}px">
  <div class="status-bar" data-status-bar="1"><span>{stime}</span><span>5G · 100%</span></div>
  {row}
  <div class="lists-title">{title}</div>
  {lists}
</div>"#,
            fh = carousel::LISTS_PHONE_H_DP,
            stime = carousel::LISTS_STATUS,
            title = carousel::LISTS_TITLE,
            lists = lists,
        )
    } else if layout.uses_phone_frame() {
        format!(
            r#"<div class="phone-frame" data-carousel-phone="1" data-carousel-layout="{layout}" style="height:{fh}px">{row}</div>"#,
            layout = layout.label(),
            fh = carousel::phone_frame_h(layout),
            row = row,
        )
    } else {
        row
    }
}

fn carousel_section(theme: &Theme) -> String {
    let mut out = String::from(
        r#"<h2>Carousel</h2>
<p class="note">Hero, multi-browse, uncontained, uncontained-multi, centered-hero, and full-screen. Uncontained-multi uses the official Your lists phone. Decoded JPEG media + parallax while flinging. <a href="https://m3.material.io/components/carousel/specs">spec</a></p>"#,
    );
    for layout in carousel::CarouselLayout::ALL {
        out.push_str(&format!("<h3>{}</h3>", layout.label()));
        out.push_str(&paint_carousel_row(theme, layout));
    }
    out
}
