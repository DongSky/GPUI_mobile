//! HTML catalog generated from the same resolve() functions the GPUI demo uses.

use crate::components::{
    button, card, checkbox, chip, divider, fab, icon_button, list, navigation_bar, progress, radio,
    snackbar, switch, text_field, top_app_bar,
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
    format!("<div class=\"state\"><div class=\"state-name\">{}</div><div class=\"state-body\">", esc(label))
}

pub fn render_html(theme: &Theme) -> String {
    let c = theme.color;
    let mode = if theme.dark { "dark" } else { "light" };
    let mut body = String::new();

    body.push_str(&hero(theme));
    body.push_str(&inventory_section());
    body.push_str(&color_section(theme));
    body.push_str(&type_section(theme));
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

    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="{mode}">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1"/>
<title>GPUI Material 3 catalog ({mode})</title>
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
h1 {{ font-size: 32px; line-height: 40px; font-weight: 400; margin: 0 0 8px; }}
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
  height: 40px; min-width: 64px; padding: 0 24px;
  border-radius: 20px; font-size: 14px; line-height: 20px; font-weight: 500;
  letter-spacing: 0.1px;
}}
.icon-btn {{
  width: 40px; height: 40px; border-radius: 20px;
  display: inline-flex; align-items: center; justify-content: center;
  font-size: 18px;
}}
.fab {{
  width: 56px; height: 56px; border-radius: 16px;
  display: inline-flex; align-items: center; justify-content: center;
  font-size: 22px;
}}
.field {{
  width: 280px; height: 56px; padding: 8px 16px;
  display: flex; flex-direction: column; justify-content: center; align-items: flex-start;
}}
.field .lab {{ font-size: 12px; line-height: 16px; }}
.field .val {{ font-size: 16px; line-height: 24px; }}
.field-wrap {{ display: flex; flex-direction: column; gap: 4px; }}
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
.linear {{ width: 240px; height: 4px; border-radius: 2px; overflow: hidden; }}
.linear i {{ display: block; height: 100%; }}
.circ {{
  width: 48px; height: 48px; border-radius: 24px;
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
table.inv {{ width: 100%; border-collapse: collapse; font-size: 13px; }}
table.inv th, table.inv td {{ text-align: left; padding: 8px 10px; border-bottom: 1px solid {outline_var}; vertical-align: top; }}
table.inv th {{ font-weight: 500; }}
.pill {{ display: inline-block; padding: 2px 8px; border-radius: 8px; font-size: 11px; font-weight: 500; }}
.divider {{ height: 1px; background: {outline_var}; margin: 8px 0; }}
</style>
</head>
<body>
{body}
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
    )
}

fn hero(theme: &Theme) -> String {
    let bar = top_app_bar::resolve(theme);
    format!(
        r#"<header class="bar" style="background:{bg};color:{fg}">
  <div class="title" style="font-size:{sz}px;line-height:{lh}px">{title}</div>
  <div>{mode}</div>
</header>
<h1>Material 3 component catalog</h1>
<p class="lead">Resolved from <code>gpui_material</code> tokens (androidx Material 3 v0_210 baseline / Material You). Same appearances drive the Android GPUI demo.</p>"#,
        bg = bar.container.css_hex(),
        fg = bar.title.css_hex(),
        sz = bar.title_style.size_sp,
        lh = bar.title_style.line_height_sp,
        title = "GPUI Material",
        mode = if theme.dark { "dark theme" } else { "light theme" },
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
        ("primaryContainer", c.primary_container, c.on_primary_container),
        ("onPrimaryContainer", c.on_primary_container, c.primary_container),
        ("secondary", c.secondary, c.on_secondary),
        ("onSecondary", c.on_secondary, c.secondary),
        ("secondaryContainer", c.secondary_container, c.on_secondary_container),
        ("tertiary", c.tertiary, c.on_tertiary),
        ("tertiaryContainer", c.tertiary_container, c.on_tertiary_container),
        ("error", c.error, c.on_error),
        ("errorContainer", c.error_container, c.on_error_container),
        ("surface", c.surface, c.on_surface),
        ("onSurface", c.on_surface, c.surface),
        ("surfaceContainerLow", c.surface_container_low, c.on_surface),
        ("surfaceContainer", c.surface_container, c.on_surface),
        ("surfaceContainerHigh", c.surface_container_high, c.on_surface),
        ("surfaceContainerHighest", c.surface_container_highest, c.on_surface),
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
    let mut out = String::from("<h2>Type scale</h2>");
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
    out
}

fn paint_button(theme: &Theme, variant: button::ButtonVariant, state: InteractionState) -> String {
    let a = button::resolve(theme, variant, state);
    format!(
        "<button class=\"btn\" data-button=\"{v}\" data-state=\"{s}\" style=\"background:{bg};color:{fg};border:{bd};box-shadow:{sh};min-width:{mw}px;height:{h}px;border-radius:{r}\">{label}</button>",
        v = variant.label(),
        s = state.label(),
        bg = a.container.css_hex(),
        fg = a.content.css_hex(),
        bd = a.outline_css(),
        sh = ElevationLevels::css_shadow(a.elevation_dp),
        mw = a.min_width_dp.unwrap_or(64.0),
        h = a.height_dp,
        r = a.corners.top_left,
        label = variant.label(),
    )
}

fn buttons(theme: &Theme) -> String {
    let mut out = String::from("<h2>Buttons</h2><p class=\"note\">Filled, tonal, elevated, outlined, text — 40dp, full corner, labelLarge. <a href=\"https://m3.material.io/components/buttons/specs\">spec</a></p>");
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
    let mut out = String::from("<h2>Icon buttons</h2>");
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
                "<div class=\"icon-btn\" data-icon-button=\"{v}\" data-state=\"{s}\" style=\"background:{bg};color:{fg};border:{bd}\">★</div>",
                v = variant.label(),
                s = state.label(),
                bg = a.container.css_hex(),
                fg = a.content.css_hex(),
                bd = a.outline_css(),
            ));
            out.push_str("</div></div>");
        }
    }
    out
}

fn fabs(theme: &Theme) -> String {
    let mut out = String::from("<h2>FAB</h2>");
    for variant in fab::FabVariant::ALL {
        let a = fab::resolve(theme, variant, InteractionState::Enabled);
        out.push_str(&format!(
            "<div class=\"fab\" data-fab=\"{v}\" style=\"background:{bg};color:{fg};box-shadow:{sh};border-radius:{r}px\">+</div>",
            v = variant.label(),
            bg = a.container.css_hex(),
            fg = a.content.css_hex(),
            sh = ElevationLevels::css_shadow(a.elevation_dp),
            r = a.corners.top_left,
        ));
    }
    out
}

fn text_fields(theme: &Theme) -> String {
    let mut out = String::from("<h2>Text fields</h2><p class=\"note\">Filled / outlined · 56dp. Error and focus indicator 2dp. IME not wired on Android.</p>");
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
            let border = match variant {
                text_field::TextFieldVariant::Filled => format!(
                    "border:none;border-bottom:{};border-radius:{} {} 0 0",
                    a.field.outline_css(),
                    a.field.corners.top_left,
                    a.field.corners.top_right
                ),
                text_field::TextFieldVariant::Outlined => format!(
                    "border:{};border-radius:{}",
                    a.field.outline_css(),
                    a.field.corners.top_left
                ),
            };
            out.push_str(&state_row_open(state.label()));
            out.push_str(&format!(
                r#"<div class="field-wrap"><div class="field" data-field="{v}" data-state="{s}" style="background:{bg};{border}">
  <div class="lab" style="color:{lab}">Label</div>
  <div class="val" style="color:{inp}">Input text</div>
</div><div class="support" style="color:{sup}">{support}</div></div>"#,
                v = variant.label(),
                s = state.label(),
                bg = a.field.container.css_hex(),
                border = border,
                lab = a.label.css_hex(),
                inp = a.input.css_hex(),
                sup = a.supporting.css_hex(),
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
            out.push_str(&state_row_open(&format!("{} / {}", value.label(), state.label())));
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
</div>"#,
        sbg = snack.container.css_hex(),
        sfg = snack.supporting.css_hex(),
        sr = snack.corners.top_left,
        act = snack.action.css_hex(),
        nbg = nav.container.css_hex(),
        nact = nav.active_label.css_hex(),
        ind = nav.active_indicator.css_hex(),
        nin = nav.inactive_label.css_hex(),
    )
}

fn progress_section(theme: &Theme) -> String {
    let lin = progress::linear(theme, 0.6);
    let circ = progress::circular(theme, 0.6);
    format!(
        r#"<h2>Progress</h2>
<div class="linear" data-progress="linear" style="background:{track}"><i style="width:{p}%;background:{ind}"></i></div>
<div class="circ" data-progress="circular" style="background:conic-gradient({cind} {ang}deg, {ctrack} 0deg)"></div>"#,
        track = lin.track.css_hex(),
        ind = lin.indicator.css_hex(),
        p = lin.progress * 100.0,
        cind = circ.indicator.css_hex(),
        ctrack = circ.track.css_hex(),
        ang = circ.progress * 360.0,
    )
}
