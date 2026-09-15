//! HTML catalog generated from the same resolve() functions the GPUI demo uses.

use crate::components::{
    badge, bottom_sheet, button, button_group, card, checkbox, chip, date_picker, dialog, divider,
    fab, icon_button, list, menu, navigation_bar, navigation_rail, progress, radio, search, slider, snackbar, switch,
    tabs, text_field, time_picker, top_app_bar,
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
    body.push_str(&text_fields(theme));
    body.push_str(&selection(theme));
    body.push_str(&lists(theme));
    body.push_str(&chips(theme));
    body.push_str(&cards(theme));
    body.push_str(&chrome(theme));
    body.push_str(&progress_section(theme));
    body.push_str(&dialogs(theme));
    body.push_str(&sheets(theme));
    body.push_str(&menus(theme));
    body.push_str(&sliders(theme));
    body.push_str(&tabs_section(theme));
    body.push_str(&badges(theme));
    body.push_str(&date_pickers(theme));
    body.push_str(&search_section(theme));
    body.push_str(&time_picker_section(theme));
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
.btn-group {{
  display: flex; gap: {gap}px; align-items: stretch; flex-wrap: wrap;
}}
.btn-connected {{ min-width: 72px; }}
.btn-connected.selected {{ font-weight: 700; }}
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
.search-bar {{
  display: flex; align-items: center; gap: 16px;
  height: 56px; padding: 0 16px; border-radius: 28px; max-width: 720px;
}}
.search-bar .ico {{ width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; font-size: 18px; }}
.search-bar .hint {{ flex: 1; font-size: 16px; line-height: 24px; }}
.search-bar .avatar {{ width: 30px; height: 30px; border-radius: 15px; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 500; }}
.search-view {{
  display: flex; flex-direction: column; max-width: 720px; overflow: hidden;
}}
.search-view .sv-head {{
  display: flex; align-items: center; gap: 16px; padding: 0 16px;
}}
.search-view .sv-list {{ display: flex; flex-direction: column; }}
.search-view .sv-row {{
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
.timepicker .hand {{
  position: absolute; left: 50%; top: 50%; width: 2px; height: 38%;
  margin-left: -1px; margin-top: -38%;
  transform-origin: 50% 100%; border-radius: 1px; pointer-events: none;
}}
.timepicker .hub {{
  position: absolute; left: 50%; top: 50%; width: 8px; height: 8px;
  margin: -4px 0 0 -4px; border-radius: 50%; pointer-events: none;
}}
.period {{ display: flex; flex-direction: column; gap: 8px; }}
.period button {{
  width: 52px; height: 36px; border: none; border-radius: 8px; font-weight: 700; cursor: pointer;
}}
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
}}
.ol legend {{
  padding: 0 4px; margin-left: 8px; font-size: 12px; line-height: 16px;
}}
.ol[data-notch="cutout"] {{ border-style: solid; }}
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
.chip {{ height: 32px; padding: 0 16px; border-radius: 16px; font-size: 14px; font-weight: 500; }}
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
.circ.indet {{ animation: m3spin 1200ms linear infinite; }}
.ptr {{ display: flex; flex-direction: column; align-items: center; gap: 8px; margin: 12px 0; }}
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
  width: 100%; max-width: 420px; height: 80px; border-radius: 0;
  justify-content: space-around; padding: 12px 0 16px;
}}
.nav .dest {{ display: flex; flex-direction: column; align-items: center; gap: 4px; font-size: 12px; font-weight: 500; }}
.nav .ind {{ width: 64px; height: 32px; border-radius: 16px; display: flex; align-items: center; justify-content: center; }}
.nav-rail {{
  width: 80px; display: flex; flex-direction: column; align-items: center; gap: 12px;
  padding: 16px 0; border-radius: 0;
}}
.nav-rail .dest {{ display: flex; flex-direction: column; align-items: center; gap: 4px; font-size: 12px; font-weight: 500; width: 80px; }}
.nav-rail .ind {{ width: 56px; height: 32px; border-radius: 16px; display: flex; align-items: center; justify-content: center; }}
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
  min-width: 200px; padding: 8px 0; display: flex; flex-direction: column;
}}
.menu-item {{
  height: 48px; padding: 0 12px; display: flex; align-items: center; gap: 12px;
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
document.querySelectorAll("[data-editor] input").forEach(function (input) {{
  function sync() {{
    var wrap = input.closest("[data-editor]");
    if (!wrap) return;
    var lab = wrap.querySelector(".lab");
    if (lab) lab.style.fontSize = (input.value || document.activeElement === input) ? "12px" : "16px";
    var box = wrap.querySelector("[data-field='outlined-edit']");
    if (box && box.tagName !== "FIELDSET" && (input.value || document.activeElement === input)) {{
      var fs = document.createElement("fieldset");
      fs.className = box.className;
      fs.setAttribute("data-field", "outlined-edit");
      fs.setAttribute("data-notched", "1");
      fs.setAttribute("data-notch", "cutout");
      fs.setAttribute("style", box.getAttribute("style") || "");
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
document.querySelectorAll("[data-button-group]").forEach(function (group) {{
  group.querySelectorAll(".btn-connected").forEach(function (btn) {{
    btn.addEventListener("click", function () {{
      group.querySelectorAll(".btn-connected").forEach(function (other) {{
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
    if (lab) lab.textContent = "Price range · " + Math.round(start * 100) + "–" + Math.round(end * 100) + "%";
  }}
  function clickStep(start, end, fraction) {{
    var pad = 0.04;
    if (Math.abs(fraction - start) <= pad) return clampRange(start + 0.05, end);
    if (Math.abs(fraction - end) <= pad) return clampRange(start, end + 0.05);
    if (fraction < start) return clampRange(start - 0.05, end);
    if (fraction > end) return clampRange(start, end - 0.05);
    var mid = (start + end) / 2;
    if (Math.abs(fraction - start) <= Math.abs(fraction - end)) return clampRange(mid, end);
    return clampRange(start, mid);
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
    var hand = picker.querySelector(".hand");
    if (hand) hand.style.transform = "rotate(" + deg + "deg)";
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
  dock.querySelectorAll("[data-docked-month]").forEach(function (btn) {{
    btn.addEventListener("click", function (ev) {{
      ev.stopPropagation();
      var label = dock.querySelector("[data-docked-month-label]");
      if (!label) return;
      var delta = parseInt(btn.getAttribute("data-docked-month") || "0", 10);
      var months = ["January","February","March","April","May","June","July","August","September","October","November","December"];
      var parts = (label.textContent || "September 2026").replace(" ▾","").split(" ");
      var mi = months.indexOf(parts[0]);
      var year = parseInt(parts[1] || "2026", 10);
      if (mi < 0) mi = 8;
      mi += delta;
      while (mi < 0) {{ mi += 12; year -= 1; }}
      while (mi > 11) {{ mi -= 12; year += 1; }}
      label.textContent = months[mi] + " " + year + " ▾";
    }});
  }});
  document.addEventListener("click", function () {{
    if (dock.getAttribute("data-dismiss-outside") === "1") setOpen(false);
  }});
}});
document.querySelectorAll("[data-search='1']").forEach(function (bar) {{
  bar.style.cursor = "pointer";
  bar.addEventListener("click", function () {{
    var view = document.querySelector("[data-search-view]");
    if (!view) return;
    var open = view.getAttribute("data-open") !== "1";
    view.setAttribute("data-open", open ? "1" : "0");
    view.style.display = open ? "flex" : "none";
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
        let a = slider::resolve_with_stops(
            theme,
            row.value,
            InteractionState::Enabled,
            row.stop_count,
        );
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
    let mut out = String::from("<h2>Buttons</h2><p class=\"note\">M3 Expressive: five colors, XS–XL, round/square, press morph. Default S is 40×16. <a href=\"https://m3.material.io/components/buttons/specs\">spec</a></p>");
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
    out.push_str("<h3>connected button group</h3>");
    out.push_str(&paint_connected_group(theme, button_group::DEMO_SELECTED));
    out.push_str("<p class=\"note\">Expressive connected group: 2dp gap, 8dp inner corners, full-round outer. Selected segment morphs toward square (checkedShape). Click to restyle.</p></div>");
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

fn icon_buttons(theme: &Theme) -> String {
    let mut out = String::from("<h2>Icon buttons</h2><p class=\"note\">Expressive: filled / tonal / outlined / standard, XS–XL, round/square, press morph. Default S is 40×24. <a href=\"https://m3.material.io/components/icon-buttons/specs\">spec</a></p>");
    out.push_str("<div class=\"hero-card\" data-hero=\"icon-buttons\"><div class=\"state-body\">");
    for variant in icon_button::IconButtonVariant::ALL {
        let a = icon_button::resolve(theme, variant, InteractionState::Enabled);
        out.push_str(&format!(
            "<div class=\"icon-btn\" data-icon-button=\"{v}\" style=\"--press-r:8px;width:{w}px;height:{h}px;background:{bg};color:{fg};border:{bd};border-radius:{r}px;font-size:18px\">★</div>",
            v = variant.label(),
            w = a.width_dp.unwrap_or(a.height_dp),
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            bd = a.outline_css(),
            r = a.corners.top_left,
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
        let fs = icon_button::icon_dp(size);
        out.push_str(&format!(
            "<div class=\"icon-btn\" data-icon-size=\"{s}\" style=\"--press-r:{pr}px;width:{w}px;height:{h}px;background:{bg};color:{fg};border-radius:{r}px;font-size:{fs}px\">★</div>",
            s = size.label(),
            pr = size.pressed_corner_dp(),
            w = a.width_dp.unwrap_or(a.height_dp),
            h = a.height_dp,
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            r = a.corners.top_left,
            fs = fs * 0.75,
        ));
    }
    out.push_str("</div></div>");
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
    let mut out = String::from("<h2>FAB</h2><p class=\"note\">Expressive: regular 56 / medium 80 / large 96 / small-extended. Baseline 40dp small FAB is deprecated.</p><div class=\"hero-card\" data-hero=\"fab\"><div class=\"state-body\">");
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
        .unwrap_or_else(|| ("transparent".into(), 1.0));
    if !a.floating {
        return format!(
            r#"<div class="filled-hero empty" {attrs} data-floating="0" style="background:{bg};border-radius:{r}px {r}px 0 0;box-shadow:inset 0 -{ow}px 0 {oc};color:{lab}">
  <div class="lab" style="color:{lab};font-size:{ls}px;line-height:{lh}px">{label}</div>
</div>"#,
            bg = a.field.container.css_hex(),
            r = a.field.corners.top_left,
            lab = a.label.css_hex(),
            ls = a.label_style.size_sp,
            lh = a.label_style.line_height_sp,
        );
    }
    format!(
        r#"<div class="filled-hero" {attrs} data-floating="1" style="background:{bg};border-radius:{r}px {r}px 0 0;box-shadow:inset 0 -{ow}px 0 {oc};color:{inp}">
  <div class="lab" style="color:{lab};font-size:{ls}px;line-height:{lh}px">{label}</div>
  {value_html}
</div>"#,
        bg = a.field.container.css_hex(),
        r = a.field.corners.top_left,
        inp = a.input.css_hex(),
        lab = a.label.css_hex(),
        ls = a.label_style.size_sp,
        lh = a.label_style.line_height_sp,
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
    if a.notched {
        format!(
            r#"<fieldset class="ol" data-notched="1" data-notch="cutout" {attrs} style="border:{ow}px solid {oc};border-radius:{r}px;color:{inp}">
  <legend style="color:{lab};padding:0 {pad}px">{label}</legend>
  {inner_html}
</fieldset>"#,
            r = a.field.corners.top_left,
            inp = a.input.css_hex(),
            lab = a.label.css_hex(),
            pad = text_field::NOTCH_PAD_DP,
        )
    } else {
        format!(
            r#"<div class="ol" data-notched="0" {attrs} style="border:{ow}px solid {oc};border-radius:{r}px;color:{lab};min-height:56px">
  <span class="lab" style="font-size:{ls}px;line-height:{lh}px">{label}</span>
  {inner_html}
</div>"#,
            r = a.field.corners.top_left,
            lab = a.label.css_hex(),
            ls = a.label_style.size_sp,
            lh = a.label_style.line_height_sp,
        )
    }
}

fn text_fields(theme: &Theme) -> String {
    let mut out = String::from("<h2>Text fields</h2><p class=\"note\">Official overview heroes are <em>empty</em> filled + outlined with the label inside the box. Populated/focused outlined uses a <em>notched floating label</em> (4dp gap). Focus/error outline is 2dp. <a href=\"https://m3.material.io/components/text-fields/specs\">spec</a></p>");
    let empty_filled = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        false,
    );
    let empty_outlined = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Enabled,
        false,
    );
    let filled = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Focused,
        true,
    );
    let outlined = text_field::resolve(
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
    out.push_str("<p class=\"note\">Empty pair matches the official overview; Email is the populated/notched example. System IME remains a NativeActivity stub.</p></div>");

    let filled_edit = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        true,
    );
    let outlined_empty = text_field::resolve(
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
    let mut out = String::from("<h2>Lists</h2>");
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

fn chips(theme: &Theme) -> String {
    let mut out = String::from("<h2>Chips</h2><div class=\"state-body\">");
    for variant in chip::ChipVariant::ALL {
        for selected in [false, true] {
            if matches!(variant, chip::ChipVariant::Assist) && selected {
                continue;
            }
            let a = chip::resolve(theme, variant, selected, InteractionState::Enabled);
            out.push_str(&format!(
                "<div class=\"chip\" data-chip=\"{v}\" data-selected=\"{sel}\" style=\"background:{bg};color:{fg};border:{bd}\">{label}</div>",
                v = variant.label(),
                sel = selected,
                bg = a.container.css_hex(),
                fg = a.content.css_hex(),
                bd = a.outline_css(),
                label = if selected {
                    format!("{} · selected", variant.label())
                } else {
                    variant.label().into()
                },
            ));
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

fn chrome(theme: &Theme) -> String {
    let snack = snackbar::resolve(theme);
    let nav = navigation_bar::resolve(theme);
    let rail = navigation_rail::resolve(theme);
    let mut rail_dests = String::new();
    for (i, (label, icon)) in navigation_rail::DESTINATIONS
        .iter()
        .zip(navigation_rail::DESTINATION_ICONS.iter())
        .enumerate()
    {
        if i == 0 {
            rail_dests.push_str(&format!(
                r#"<div class="dest" style="color:{fg}"><div class="ind" style="background:{ind}">{icon}</div>{label}</div>"#,
                fg = rail.active_label.css_hex(),
                ind = rail.active_indicator.css_hex(),
            ));
        } else {
            rail_dests.push_str(&format!(
                r#"<div class="dest" style="color:{fg}">{icon}<span>{label}</span></div>"#,
                fg = rail.inactive_label.css_hex(),
            ));
        }
    }
    format!(
        r#"<h2>Snackbar</h2>
<div class="snack" data-snackbar="1" style="background:{sbg};color:{sfg};border-radius:{sr}px">
  <span>Message sent</span>
  <span style="color:{act};font-weight:500">Action</span>
</div>
<h2>Navigation bar</h2>
<div class="nav" data-navbar="1" style="background:{nbg}">
  <div class="dest" style="color:{nact}"><div class="ind" style="background:{ind}">●</div>Home</div>
  <div class="dest" style="color:{nin}">○<span>Search</span></div>
  <div class="dest" style="color:{nin}">○<span>Profile</span></div>
</div>
<h2>Navigation rail</h2>
<p class="note">80dp vertical destinations, 56×32 active indicator. Catalog stub — no collapsed/expanded modal. <a href="https://m3.material.io/components/navigation-rail/specs">spec</a></p>
<div class="nav-rail" data-nav-rail="1" data-hero="nav-rail" style="background:{rbg}">{rail_dests}</div>"#,
        sbg = snack.container.css_hex(),
        sfg = snack.supporting.css_hex(),
        sr = snack.corners.top_left,
        act = snack.action.css_hex(),
        nbg = nav.container.css_hex(),
        nact = nav.active_label.css_hex(),
        ind = nav.active_indicator.css_hex(),
        nin = nav.inactive_label.css_hex(),
        rbg = rail.container.css_hex(),
        rail_dests = rail_dests,
    )
}

fn progress_section(theme: &Theme) -> String {
    let lin = progress::linear(theme, 0.6);
    let circ = progress::circular(theme, 0.6);
    let indet = progress::linear_indeterminate(theme);
    let circ_i = progress::circular_indeterminate(theme);
    let ptr = progress::pull_to_refresh(theme);
    format!(
        r#"<h2>Progress</h2>
<p class="note">Determinate plus indeterminate / pull-to-refresh. HTML animates with spatial/effects springs; GPUI paints a static busy frame. <a href="https://m3.material.io/components/progress-indicators/specs">spec</a></p>
<div class="linear" data-progress="linear" style="background:{track}"><i style="width:{p}%;background:{ind}"></i></div>
<div class="circ" data-progress="circular" style="background:conic-gradient({cind} {ang}deg, {ctrack} 0deg)"></div>
<h3>indeterminate</h3>
<div class="linear indet" data-progress="indeterminate" data-hero="progress-indet" style="background:{itrack};margin:12px 0"><i style="width:{span}%;background:{iind}"></i></div>
<div class="circ indet" data-progress="circular-indet" style="background:conic-gradient({ciind} {arc}deg, {citrack} 0deg)"></div>
<div class="ptr" data-progress="ptr" data-hero="progress-ptr">
  <div class="circ indet" style="width:{psz}px;height:{psz}px;border-radius:{pr}px;background:conic-gradient({pind} {parc}deg, {ptrack} 0deg)"></div>
  <div class="note">{plabel}</div>
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
        psz = ptr.size_dp,
        pr = ptr.size_dp / 2.0,
        pind = ptr.indicator.css_hex(),
        parc = ptr.arc_deg,
        ptrack = ptr.track.css_hex(),
        plabel = progress::PTR_LABEL,
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
<p class="note">Guidelines Reset settings (icon, headline, supporting, text Cancel/Accept) plus overview Phone ringtone list dialog. 28dp · elev 3 · 32% scrim. Actions use the Text button resolve (transparent container). <a href="https://m3.material.io/components/dialogs/overview">overview</a></p>
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
</div>"#,
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
    )
}

fn sheets(theme: &Theme) -> String {
    let modal = bottom_sheet::resolve(theme, true);
    format!(
        r#"<h2>Bottom sheet</h2>
<p class="note">Modal sheet · extra-large top 28 · 32×4 handle · elevation 1.</p>
<div class="scrim" data-sheet="scrim" style="background:{scrim};align-items:flex-end">
  <div class="sheet" data-sheet="modal" style="background:{bg};border-radius:{css};box-shadow:{sh};color:{fg}">
    <div class="handle" style="width:{hw}px;height:{hh}px;background:{handle}"></div>
    <div class="list-item" style="width:100%"><div class="h">Share</div></div>
    <div class="list-item" style="width:100%"><div class="h">Add to favorites</div></div>
    <div class="list-item" style="width:100%"><div class="h">Delete</div></div>
  </div>
</div>"#,
        scrim = modal.scrim.css_hex(),
        bg = modal.container.css_hex(),
        css = modal.corners.css(),
        sh = ElevationLevels::css_shadow(modal.elevation_dp),
        fg = modal.content.css_hex(),
        hw = modal.handle_w,
        hh = modal.handle_h,
        handle = modal.handle.css_hex(),
    )
}

fn menus(theme: &Theme) -> String {
    let shell = menu::resolve_menu(theme);
    let items = [
        ("Item one", true, InteractionState::Enabled),
        ("Item two", false, InteractionState::Enabled),
        ("Disabled", false, InteractionState::Disabled),
    ];
    let mut rows = String::new();
    for (label, selected, state) in items {
        let a = menu::resolve_item(theme, selected, state);
        rows.push_str(&format!(
            "<div class=\"menu-item\" data-menu-item=\"{label}\" data-selected=\"{sel}\" style=\"background:{bg};color:{fg};height:{h}px\">{label}</div>",
            sel = selected,
            bg = a.container.css_hex(),
            fg = a.label.css_hex(),
            h = a.height_dp,
        ));
    }
    format!(
        r#"<h2>Menu</h2>
<p class="note">4dp corners · elevation 2 · 48dp items. Selected uses secondary container.</p>
<div class="menu" data-menu="1" style="background:{bg};border-radius:{r}px;box-shadow:{sh}">{rows}</div>"#,
        bg = shell.container.css_hex(),
        r = shell.corners.top_left,
        sh = ElevationLevels::css_shadow(shell.elevation_dp),
        rows = rows,
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
    format!(
        r#"<div class="slider-row" data-slider-range="1" data-start="{start}" data-end="{end}" data-range-interactive="1">
  <div class="slider-meta"><div class="slider-label">{label}</div>
  <div class="xslider" data-slider="{label}" data-handle-visual="{hv}">
    <div class="xseg inactive" style="width:{lw}%;height:{th}px;background:{inactive};border-radius:{oc}px {ic}px {ic}px {oc}px"></div>
    <div class="xhandle" style="width:{hw}px;height:{hh}px;background:{handle};margin:0 {gap}px"></div>
    <div class="xseg active" style="width:{mw}%;height:{th}px;background:{active};border-radius:{ic}px"></div>
    <div class="xhandle" style="width:{hw}px;height:{hh}px;background:{handle};margin:0 {gap}px"></div>
    <div class="xseg inactive" style="flex:1;height:{th}px;background:{inactive};border-radius:{ic}px {oc}px {oc}px {ic}px"></div>
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
    let mut out = String::from("<h2>Slider</h2><p class=\"note\">M3 Expressive (current site): thick track + 4×44 vertical handle, 6dp gap, 4dp stops. Overview scene is volume rows; Alarm has mid-track stops. <a href=\"https://m3.material.io/components/sliders/specs\">spec</a></p>");
    out.push_str("<div class=\"hero-card\" data-hero=\"slider\">");
    for row in slider::OVERVIEW_ROWS {
        let a = slider::resolve_with_stops(
            theme,
            row.value,
            InteractionState::Enabled,
            row.stop_count,
        );
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

fn paint_date_grid(a: &date_picker::DatePickerAppearance, cells: [(u32, date_picker::DayKind); 42]) -> String {
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
<div class="docked" data-datepicker-docked="1" data-hero="datepicker-docked" data-popup="open" data-dismiss-outside="1">
  {docked_field}
  <div class="cal dialog" data-datepicker-popup="open" style="background:{bg};border-radius:8px {r}px {r}px {r}px;box-shadow:{sh};margin-top:4px;width:100%">
    <div class="month-nav">
      <button type="button" data-docked-month="-1" aria-label="Previous month">&lt;</button>
      <div data-docked-month-label="1">{month}</div>
      <button type="button" data-docked-month="1" aria-label="Next month">&gt;</button>
    </div>
    <div class="week">{week}</div>
    <div class="grid">{grid}</div>
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

fn search_section(theme: &Theme) -> String {
    let a = search::resolve(theme);
    let view = search::resolve_view(theme);
    let mut rows = String::new();
    for (i, label) in search::SUGGESTIONS.iter().enumerate() {
        rows.push_str(&format!(
            r#"<div class="sv-row" data-search-suggestion="{label}" style="color:{fg};height:{h}px"><span style="color:{ico}">{icon}</span><span>{label}</span></div>"#,
            fg = view.suggestion.css_hex(),
            h = view.suggestion_h_dp,
            ico = view.suggestion_icon.css_hex(),
            icon = if i == 0 { "⌕" } else { "◌" },
        ));
    }
    format!(
        r#"<h2>Search</h2>
<p class="note">Docked 56dp full-round bar plus expanded search view/sheet (back, input, suggestions). Tap the bar to toggle. <a href="https://m3.material.io/components/search/specs">spec</a></p>
<div class="search-bar" data-search="1" data-hero="search" style="background:{bg};color:{hint};height:{h}px;border-radius:{r}px">
  <div class="ico" aria-hidden="true" style="color:{lead}">{lead_ico}</div>
  <div class="hint">{placeholder}</div>
  <div class="ico" aria-hidden="true">{mic}</div>
  <div class="avatar" style="background:{abg};color:{afg}">A</div>
</div>
<div class="search-view" data-search-view="1" data-open="1" data-hero="search-view" style="background:{vbg};border-radius:{vr}px;box-shadow:{vsh};margin-top:12px">
  <div class="sv-head" style="height:{vh}px;color:{vfg}">
    <div class="ico" aria-hidden="true">{back}</div>
    <div class="hint" style="flex:1;color:{vph}">{placeholder}</div>
    <div class="ico">{mic}</div>
  </div>
  <div style="height:1px;background:{vdiv}"></div>
  <div class="sv-list">{rows}</div>
</div>"#,
        bg = a.bar.container.css_hex(),
        hint = a.placeholder.css_hex(),
        h = a.bar.height_dp,
        r = a.bar.corners.top_left,
        lead = a.leading_icon.css_hex(),
        lead_ico = search::LEADING_ICON,
        placeholder = search::PLACEHOLDER,
        mic = search::TRAILING_MIC,
        abg = a.avatar.css_hex(),
        afg = a.avatar_label.css_hex(),
        vbg = view.container.css_hex(),
        vr = view.corners.top_left,
        vsh = ElevationLevels::css_shadow(view.elevation_dp),
        vh = view.header_h_dp,
        vfg = view.header.css_hex(),
        vph = view.placeholder.css_hex(),
        back = search::VIEW_BACK,
        vdiv = view.divider.css_hex(),
        rows = rows,
    )
}

fn time_picker_section(theme: &Theme) -> String {
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
            ("transparent".into(), a.number.css_hex(), a.number_style.weight)
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
            ("transparent".into(), a.number.css_hex(), a.number_style.weight)
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
        (a.period_selected_container.css_hex(), a.period_selected.css_hex())
    } else {
        (a.period_idle_container.css_hex(), a.period_idle.css_hex())
    };
    let (pm_bg, pm_fg) = if time_picker::DEMO_PERIOD == pm {
        (a.period_selected_container.css_hex(), a.period_selected.css_hex())
    } else {
        (a.period_idle_container.css_hex(), a.period_idle.css_hex())
    };
    let hand_deg = time_picker::hand_angle_deg(
        time_picker::DEMO_DIAL,
        time_picker::DEMO_HOUR,
        time_picker::DEMO_MINUTE,
    );
    let hour_active = time_picker::DEMO_DIAL == time_picker::DialFace::Hour;
    format!(
        r#"<h2>Time picker</h2>
<p class="note">12-hour + minute dial, analog selector hand, displaySmallEmphasized header, AM/PM. Header fields toggle the face. <a href="https://m3.material.io/components/time-pickers/specs">spec</a></p>
<div class="timepicker dialog" data-timepicker="1" data-hero="timepicker" data-dial="minute" data-hour="{hour}" data-minute="{minute}" data-period="{period}" style="background:{bg};border-radius:{r}px;box-shadow:{sh}">
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
    <div class="hand" style="background:{hand};transform:rotate({deg}deg)"></div>
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
        deg = hand_deg,
    )
}

fn motion_section(theme: &Theme) -> String {
    format!(
        r#"<h2>Motion</h2>
<p class="note">Expressive spatial <code>{ease}</code> ({sd}ms) + effects springs. Button / connected-group press morph uses the same CSS transition. GPUI has no shared animation clock.</p>
<div class="motion-box" data-motion="emphasized" style="background:{p}"></div>"#,
        ease = theme.motion.spatial_fast,
        sd = theme.motion.spatial_fast_ms,
        p = theme.color.primary.css_hex(),
    )
}
