//! Desktop Material 3 / Expressive catalog.
//!
//! Paints `gpui_material::resolve()` appearances with the same GPUI `div`
//! mapping as Android `component_demo`. Tokens live only in `gpui_material`.
//!
//! Desktop type: Roboto when installed (`typography::desktop_font_family()`),
//! else Liberation Sans. Mesa llvmpipe / cosmic-text often collapses space
//! glyphs, so multi-word copy is mapped as separate word elements with
//! `WORD_GAP_DP`. Hardware GL is unavailable when `/dev/dri` is missing
//! (`scripts/desktop.sh` sets `WGPU_BACKEND=gl`).
//!
//! Boot pattern matches upstream Zed GPUI examples (`gpui_platform::application`).

#![cfg(not(target_os = "android"))]

use gpui::prelude::*;
use gpui::{
    black, canvas, div, point, px, size, Animation, AnimationExt, App, Bounds, Context, FillOptions,
    FillRule, FontWeight, IntoElement, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent,
    ParentElement, PathBuilder, PathStyle, Render, ScrollDelta, ScrollWheelEvent, SharedString,
    StrokeOptions, Styled, TextRun, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui_material::components::date_picker::{self, CivilDate, DayKind};
use gpui_material::components::text_field::TextFieldEditor;
use gpui_material::components::time_picker::{self, DayPeriod, DialFace};
use gpui_material::components::{
    badge, button, button_group, carousel, checkbox, dialog, icon_button, navigation_rail, progress,
    radio, search, slider, switch, tabs, text_field, top_app_bar,
};
use gpui_material::theme::Theme;
use gpui_material::typography;
use gpui_material::{Argb, InteractionState};
use gpui_platform::application;
use lyon::tessellation::{LineCap, LineJoin};
use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

fn paint(c: Argb) -> gpui::Rgba {
    gpui::Rgba {
        r: c.r() as f32 / 255.0,
        g: c.g() as f32 / 255.0,
        b: c.b() as f32 / 255.0,
        a: c.a() as f32 / 255.0,
    }
}

fn type_size(style: gpui_material::typography::TypeStyle) -> gpui::Pixels {
    px(style.size_sp)
}

fn type_weight(style: gpui_material::typography::TypeStyle) -> FontWeight {
    match style.weight {
        w if w >= 700 => FontWeight::BOLD,
        w if w >= 500 => FontWeight::MEDIUM,
        _ => FontWeight::NORMAL,
    }
}

fn feed_outline_verbs(
    builder: &mut PathBuilder,
    origin: gpui::Point<gpui::Pixels>,
    verbs: impl IntoIterator<Item = text_field::OutlineVerb>,
) {
    for v in verbs {
        match v {
            text_field::OutlineVerb::Move(x, y) => {
                builder.move_to(point(origin.x + px(x), origin.y + px(y)));
            }
            text_field::OutlineVerb::Line(x, y) => {
                builder.line_to(point(origin.x + px(x), origin.y + px(y)));
            }
            text_field::OutlineVerb::Arc {
                to_x,
                to_y,
                radius,
                clockwise,
            } => {
                builder.arc_to(
                    point(px(radius), px(radius)),
                    px(0.),
                    false,
                    clockwise,
                    point(origin.x + px(to_x), origin.y + px(to_y)),
                );
            }
            text_field::OutlineVerb::Cubic {
                c1_x,
                c1_y,
                c2_x,
                c2_y,
                to_x,
                to_y,
            } => {
                builder.cubic_bezier_to(
                    point(origin.x + px(to_x), origin.y + px(to_y)),
                    point(origin.x + px(c1_x), origin.y + px(c1_y)),
                    point(origin.x + px(c2_x), origin.y + px(c2_y)),
                );
            }
            text_field::OutlineVerb::Close => builder.close(),
        }
    }
}

fn stroke_round(width: f32) -> PathBuilder {
    PathBuilder::stroke(px(width)).with_style(PathStyle::Stroke(
        StrokeOptions::default()
            .with_line_width(width)
            .with_line_cap(gpui_line_cap(progress::STROKE_CAP))
            .with_line_join(LineJoin::Round),
    ))
}

fn gpui_line_cap(cap: progress::StrokeCap) -> LineCap {
    match cap {
        progress::StrokeCap::Round => LineCap::Round,
    }
}

fn measure_label_width_dp(window: &mut Window, label: &str, size_sp: f32) -> f32 {
    if label.is_empty() {
        return 0.0;
    }
    let run = TextRun {
        len: label.len(),
        font: gpui::font(typography::desktop_font_family()),
        color: black(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let layout = window
        .text_system()
        .layout_line(label, px(size_sp), &[run], None);
    f32::from(layout.width)
}

fn paint_search_scaled_fill(
    window: &mut Window,
    bounds: gpui::Bounds<gpui::Pixels>,
    scale: f32,
    radius: f32,
    color: gpui::Rgba,
) {
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let [pre, post] = search::top_center_scale_translates(
        f32::from(bounds.origin.x),
        f32::from(bounds.origin.y),
        w,
    );
    let r = radius.max(0.0).min(w / 2.0).min(h / 2.0);
    let k = gpui_material::shape::CIRCULAR_KAPPA;
    let verbs = if r < 0.5 {
        vec![
            text_field::OutlineVerb::Move(0.0, 0.0),
            text_field::OutlineVerb::Line(w, 0.0),
            text_field::OutlineVerb::Line(w, h),
            text_field::OutlineVerb::Line(0.0, h),
            text_field::OutlineVerb::Close,
        ]
    } else {
        vec![
            text_field::OutlineVerb::Move(r, 0.0),
            text_field::OutlineVerb::Line(w - r, 0.0),
            text_field::OutlineVerb::cubic_quarter((w - r, 0.0), (w, 0.0), (w, r), k),
            text_field::OutlineVerb::Line(w, h - r),
            text_field::OutlineVerb::cubic_quarter((w, h - r), (w, h), (w - r, h), k),
            text_field::OutlineVerb::Line(r, h),
            text_field::OutlineVerb::cubic_quarter((r, h), (0.0, h), (0.0, h - r), k),
            text_field::OutlineVerb::Line(0.0, r),
            text_field::OutlineVerb::cubic_quarter((0.0, r), (0.0, 0.0), (r, 0.0), k),
            text_field::OutlineVerb::Close,
        ]
    };
    let mut builder = PathBuilder::fill();
    builder.translate(point(px(pre.0), px(pre.1)));
    builder.scale(scale);
    builder.translate(point(px(post.0), px(post.1)));
    feed_outline_verbs(&mut builder, bounds.origin, verbs);
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

fn nav_rail_os_popup_options(spec: navigation_rail::OsPopupSpec) -> WindowOptions {
    WindowOptions {
        kind: WindowKind::PopUp,
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: point(px(0.), px(0.)),
            size: size(px(spec.width_dp), px(880.)),
        })),
        titlebar: None,
        focus: true,
        show: true,
        is_movable: false,
        ..Default::default()
    }
}

fn paint_round_polyline(
    window: &mut Window,
    origin: gpui::Point<gpui::Pixels>,
    pts: &[(f32, f32)],
    stroke: f32,
    color: gpui::Rgba,
) {
    if pts.len() < 2 {
        return;
    }
    let mut builder = stroke_round(stroke);
    for (i, (x, y)) in pts.iter().enumerate() {
        let p = point(origin.x + px(*x), origin.y + px(*y));
        if i == 0 {
            builder.move_to(p);
        } else {
            builder.line_to(p);
        }
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

fn paint_filled_polygon(
    window: &mut Window,
    origin: gpui::Point<gpui::Pixels>,
    pts: &[(f32, f32)],
    color: gpui::Rgba,
) {
    if pts.len() < 3 {
        return;
    }
    let mut builder = PathBuilder::fill();
    for (i, (x, y)) in pts.iter().enumerate() {
        let p = point(origin.x + px(*x), origin.y + px(*y));
        if i == 0 {
            builder.move_to(p);
        } else {
            builder.line_to(p);
        }
    }
    builder.close();
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}


#[derive(Clone, Copy, PartialEq, Eq)]
enum Overlay {
    None,
    Dialog,
    ListDialog,
}

struct CatalogView {
    dark: bool,
    taps: usize,
    checked: bool,
    switched: bool,
    overlay: Overlay,
    slider: f32,
    ringtone: usize,
    tab: usize,
    picker_year: i32,
    picker_month: u32,
    selected: CivilDate,
    today: CivilDate,
    filled: TextFieldEditor,
    outlined: TextFieldEditor,
    group_selected: usize,
    range_start: f32,
    range_end: f32,
    range_drag: Option<slider::RangeThumb>,
    range_focus: slider::RangeThumb,
    range_moved: bool,
    range_hit: Rc<Cell<(f32, f32)>>,
    docked_open: bool,
    search_open: bool,
    search: TextFieldEditor,
    rail_selected: usize,
    rail_mode: navigation_rail::RailMode,
    carousel_index: usize,
    carousel_fling: carousel::FlingState,
    carousel_fling_at: Option<Instant>,
    time_hour: u8,
    time_minute: u8,
    time_period: DayPeriod,
    time_dial: DialFace,
    time_hand_from: f32,
    time_hand_gen: u32,
}

impl CatalogView {
    fn theme(&self) -> Theme {
        if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }

    fn bump_time_hand(&mut self) {
        self.time_hand_from = time_picker::hand_angle_deg(
            self.time_dial,
            self.time_hour,
            self.time_minute,
        );
        self.time_hand_gen = self.time_hand_gen.wrapping_add(1);
    }

    fn tick_carousel_fling(&mut self) {
        if self.carousel_fling.resting() {
            self.carousel_fling.selected = self.carousel_index;
            self.carousel_fling_at = None;
            return;
        }
        let now = Instant::now();
        let dt = self
            .carousel_fling_at
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(carousel::FLING_FRAME_DT);
        self.carousel_fling_at = Some(now);
        self.carousel_index = self.carousel_fling.step_live(dt);
    }
}

impl Render for CatalogView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.tick_carousel_fling();
        let theme = self.theme();
        let c = theme.color;
        let bar = top_app_bar::resolve(&theme);

        let chrome = div()
            .h(px(bar.height_dp))
            .w_full()
            .px(px(16.))
            .flex()
            .items_center()
            .justify_between()
            .bg(paint(bar.container))
            .child(
                div()
                    .text_size(px(bar.title_style.size_sp))
                    .text_color(paint(bar.title))
                    .whitespace_nowrap()
                    .font_weight(type_weight(bar.title_style.emphasized()))
                    .child(spaced_line(
                        "Material 3 desktop",
                        bar.title_style.emphasized().size_sp,
                        paint(bar.title),
                    )),
            )
            .child(
                div()
                    .id("theme-toggle")
                    .px(px(12.))
                    .py(px(8.))
                    .rounded(px(20.))
                    .bg(paint(c.secondary_container))
                    .text_color(paint(c.on_secondary_container))
                    .text_size(type_size(theme.typography.label_large))
                    .child(if self.dark { "Dark" } else { "Light" })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.dark = !this.dark;
                        cx.notify();
                    })),
            );

        let body = match self.overlay {
            Overlay::None => catalog_body(self, &theme, cx).into_any_element(),
            Overlay::Dialog => dialog_overlay(&theme, cx).into_any_element(),
            Overlay::ListDialog => list_dialog_overlay(self, &theme, cx).into_any_element(),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(paint(c.background))
            .font_family(typography::desktop_font_family())
            .text_color(paint(c.on_background))
            .child(chrome)
            .child(body)
    }
}

fn catalog_body(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let c = theme.color;
    let filled_a = this.filled.appearance(theme);
    let outlined_a = this.outlined.appearance(theme);
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
    let check = checkbox::resolve(
        theme,
        if this.checked {
            checkbox::CheckValue::Checked
        } else {
            checkbox::CheckValue::Unchecked
        },
        InteractionState::Enabled,
    );
    let sw = switch::resolve(theme, this.switched, InteractionState::Enabled);
    let tabs_p = tabs::resolve(theme, tabs::TabsVariant::Primary);
    let pick = date_picker::resolve(theme);
    let cells = date_picker::month_grid_classified(
        this.picker_year,
        this.picker_month,
        this.selected,
        this.today,
    );

    div()
        .id("catalog-scroll")
        .flex_1()
        .w_full()
        .overflow_y_scroll()
        .p(px(20.))
        .gap(px(16.))
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(px(theme.typography.body_medium.size_sp))
                .text_color(paint(c.on_surface_variant))
                .whitespace_nowrap()
                .child(spaced_line(
                    "Desktop catalog · same gpui_material::resolve() as Android + HTML.",
                    theme.typography.body_medium.size_sp,
                    paint(c.on_surface_variant),
                )),
        )
        .child(settings_scene(this, theme, cx))
        .child(section_title(theme, "Buttons"))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .children(button::OVERVIEW_ORDER.iter().map(|variant| {
                    m_button(
                        variant.label(),
                        theme,
                        *variant,
                        InteractionState::Enabled,
                        variant.overview_label(),
                        cx.listener(|this, _, _, cx| {
                            this.taps += 1;
                            cx.notify();
                        }),
                    )
                    .into_any_element()
                })),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .items_center()
                .children(button::ButtonSize::ALL.iter().map(|size| {
                    let a = button::resolve_expressive(
                        theme,
                        button::ButtonVariant::Filled,
                        *size,
                        button::ButtonShape::Round,
                        InteractionState::Enabled,
                    );
                    div()
                        .h(px(a.height_dp))
                        .px(px(a.pad_start_dp))
                        .rounded(px(a.corners.top_left))
                        .bg(paint(a.container))
                        .text_color(paint(a.content))
                        .text_size(type_size(a.label_style))
                        .font_weight(FontWeight::MEDIUM)
                        .flex()
                        .items_center()
                        .child("Label")
                })),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(paint(c.on_surface_variant))
                .child(format!(
                    "Filled tap count {} · sizes XS–XL from resolve_expressive()",
                    this.taps
                )),
        )
        .child(connected_button_group(theme, this.group_selected, cx))
        .child(section_title(theme, "Icon buttons"))
        .child(
            div()
                .flex()
                .gap(px(8.))
                .items_center()
                .children(icon_button::IconButtonVariant::ALL.iter().map(|variant| {
                    let a = icon_button::resolve(theme, *variant, InteractionState::Enabled);
                    div()
                        .w(px(a.height_dp))
                        .h(px(a.height_dp))
                        .rounded(px(a.corners.top_left))
                        .bg(paint(a.container))
                        .text_color(paint(a.content))
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(a.outline.is_some(), |el| {
                            el.border_1()
                                .border_color(paint(a.outline.unwrap().0))
                        })
                        .child("★")
                })),
        )
        .child(section_title(theme, "Text fields"))
        .child(field_block(
            "hero-empty-filled",
            &empty_filled,
            "Label",
            "",
            "",
            |_, _, _| {},
        ))
        .child(field_block(
            "hero-empty-outlined",
            &empty_outlined,
            "Label",
            "",
            "",
            |_, _, _| {},
        ))
        .child(field_block(
            "field-outlined",
            &outlined_a,
            "Email",
            this.outlined.display_with_caret(),
            if this.outlined.error {
                "Enter a valid email"
            } else {
                "Supporting text"
            },
            cx.listener(|this, _, _, cx| {
                this.filled.set_focus(false);
                this.outlined.set_focus(true);
                this.outlined.insert_char('a');
                let v = this.outlined.value().to_string();
                this.outlined.error = !v.is_empty() && !text_field::looks_like_email(&v);
                cx.notify();
            }),
        ))
        .child(field_block(
            "field-filled",
            &filled_a,
            "Label",
            this.filled.display_with_caret(),
            "Supporting text",
            cx.listener(|this, _, _, cx| {
                this.outlined.set_focus(false);
                this.filled.set_focus(true);
                this.filled.insert_char('x');
                cx.notify();
            }),
        ))
        .child(section_title(theme, "Slider"))
        .child(volume_slider_scene(theme, this.slider, cx))
        .child(range_slider_hero(this, theme, cx))
        .child(section_title(theme, "Progress"))
        .child(progress_heroes(theme))
        .child(section_title(theme, "Carousel"))
        .child(carousel_hero(this, theme, cx))
        .child(
            div()
                .text_size(px(12.))
                .text_color(paint(c.on_surface_variant))
                .child(spaced_line(
                    format!(
                        "Media {:.0}% · XS track 16 / handle visual 28 (token 44) · Alarm 13 stops",
                        this.slider * 100.0
                    ),
                    12.0,
                    paint(c.on_surface_variant),
                )),
        )
        .child(section_title(theme, "Selection · tabs"))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(16.))
                .child(
                    div()
                        .id("checkbox")
                        .w(px(check.target_dp))
                        .h(px(check.target_dp))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(check.box_size_dp))
                                .h(px(check.box_size_dp))
                                .rounded(px(check.corner_dp))
                                .bg(paint(check.box_fill))
                                .when(check.box_outline.is_some(), |el| {
                                    el.border_1()
                                        .border_color(paint(check.box_outline.unwrap()))
                                })
                                .text_color(paint(check.icon))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(if this.checked { "✓" } else { "" }),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.checked = !this.checked;
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .id("switch")
                        .w(px(sw.track_w))
                        .h(px(sw.track_h))
                        .rounded(px(sw.track_h / 2.0))
                        .bg(paint(sw.track))
                        .when(sw.track_outline.is_some(), |el| {
                            el.border_2().border_color(paint(sw.track_outline.unwrap()))
                        })
                        .flex()
                        .items_center()
                        .px(px(4.))
                        .when(this.switched, |el| el.justify_end())
                        .child(
                            div()
                                .w(px(sw.thumb_dp))
                                .h(px(sw.thumb_dp))
                                .rounded(px(sw.thumb_dp / 2.0))
                                .bg(paint(sw.thumb)),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.switched = !this.switched;
                            cx.notify();
                        })),
                ),
        )
        .child(tab_row(&tabs_p, this.tab, cx))
        .child(section_title(theme, "Navigation rail"))
        .child(nav_rail_hero(this, theme, cx))
        .child(section_title(theme, "Dialog"))
        .child(
            div()
                .flex()
                .gap(px(8.))
                .child(m_button(
                    "open-dialog",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Reset settings",
                    cx.listener(|this, _, _, cx| {
                        this.overlay = Overlay::Dialog;
                        cx.notify();
                    }),
                ))
                .child(m_button(
                    "open-list-dialog",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Phone ringtone",
                    cx.listener(|this, _, _, cx| {
                        this.overlay = Overlay::ListDialog;
                        cx.notify();
                    }),
                )),
        )
        .child(section_title(theme, "Search"))
        .child(search_bar_hero(this, theme, cx))
        .child(section_title(theme, "Time picker"))
        .child(time_picker_hero(this, theme, cx))
        .child(section_title(theme, "Date picker"))
        .child(date_range_hero(theme, &pick))
        .child(docked_date_picker(this, theme, &pick, &cells, cx))
        .child(date_picker_card(this, theme, &pick, &cells, cx))
}

fn connected_button_group(
    theme: &Theme,
    selected: usize,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let count = button_group::DEMO_SEGMENTS.len();
    div()
        .flex()
        .flex_row()
        .gap(px(button_group::CONNECTED_GAP_DP))
        .children(
            button_group::DEMO_SEGMENTS
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let a = button_group::resolve_segment(theme, i, count, i == selected, false);
                    div()
                        .id(SharedString::from(format!("group-{i}")))
                        .h(px(a.height_dp))
                        .px(px(a.pad_start_dp))
                        .rounded_tl(px(a.corners.top_left))
                        .rounded_tr(px(a.corners.top_right))
                        .rounded_br(px(a.corners.bottom_right))
                        .rounded_bl(px(a.corners.bottom_left))
                        .bg(paint(a.container))
                        .text_color(paint(a.content))
                        .text_size(type_size(a.label_style))
                        .font_weight(type_weight(a.label_style))
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(a.outline.is_some(), |el| {
                            let (color, _) = a.outline.unwrap();
                            el.border_1().border_color(paint(color))
                        })
                        .child(*label)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.group_selected = i;
                            cx.notify();
                        }))
                }),
        )
}

fn range_slider_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let range = slider::resolve_range(
        theme,
        this.range_start,
        this.range_end,
        InteractionState::Enabled,
    );
    let t = range.track;
    let total = slider::RANGE_TRACK_W_DP;
    let paint_r = slider::range_paint(range.start, range.end, total, t.handle_w.max(4.0));
    let y_track = ((t.target_dp - t.track_h) / 2.0).max(0.0);
    let y_handle = ((t.target_dp - t.handle_h_visual) / 2.0).max(0.0);
    let hit = this.range_hit.clone();
    let hit_move = this.range_hit.clone();
    let hit_click = this.range_hit.clone();
    div()
        .id("range-slider")
        .tab_index(0)
        .w_full()
        .flex()
        .flex_col()
        .gap(px(4.))
        .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
            if let Some((s, e, thumb)) = slider::apply_arrow(
                this.range_start,
                this.range_end,
                this.range_focus,
                &ev.keystroke.key,
            ) {
                this.range_start = s;
                this.range_end = e;
                this.range_focus = thumb;
                cx.notify();
            }
        }))
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _, _, cx| {
                this.range_drag = None;
                cx.notify();
            }),
        )
        .child(spaced_line(
            slider::range_value_label(range.start, range.end),
            12.0,
            paint(theme.color.on_surface),
        ))
        .child(
            div()
                .id("range-track")
                .relative()
                .w(px(total))
                .h(px(t.target_dp))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                        let (origin, w) = hit.get();
                        let frac =
                            slider::fraction_from_local_x(f32::from(ev.position.x) - origin, w);
                        this.range_focus =
                            slider::nearest_thumb(this.range_start, this.range_end, frac);
                        this.range_drag = Some(this.range_focus);
                        this.range_moved = false;
                        cx.notify();
                    }),
                )
                .on_mouse_move(cx.listener(move |this, ev: &MouseMoveEvent, _, cx| {
                    if ev.dragging() {
                        if let Some(thumb) = this.range_drag {
                            let (origin, w) = hit_move.get();
                            let frac =
                                slider::fraction_from_local_x(f32::from(ev.position.x) - origin, w);
                            let (s, e) = slider::drag_thumb_snapped(
                                this.range_start,
                                this.range_end,
                                thumb,
                                frac,
                            );
                            this.range_start = s;
                            this.range_end = e;
                            this.range_moved = true;
                            cx.notify();
                        }
                    }
                }))
                .on_click(cx.listener(move |this, ev: &gpui::ClickEvent, _, cx| {
                    if this.range_moved {
                        this.range_drag = None;
                        this.range_moved = false;
                        cx.notify();
                        return;
                    }
                    let (origin, w) = hit_click.get();
                    let frac =
                        slider::fraction_from_local_x(f32::from(ev.position().x) - origin, w);
                    let (s, e) = slider::click_step(this.range_start, this.range_end, frac);
                    this.range_start = s;
                    this.range_end = e;
                    this.range_drag = None;
                    cx.notify();
                }))
                .child(
                    div()
                        .absolute()
                        .left(px(0.))
                        .top(px(y_track))
                        .h(px(t.track_h))
                        .w(px(paint_r.left))
                        .rounded(px(t.track_corner))
                        .bg(paint(t.inactive)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.start_handle))
                        .top(px(y_handle))
                        .w(px(paint_r.handle_w.max(12.0)))
                        .h(px(t.handle_h_visual))
                        .rounded(px(2.))
                        .bg(paint(t.handle)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.start_handle + paint_r.handle_w.max(12.0)))
                        .top(px(y_track))
                        .h(px(t.track_h))
                        .w(px(paint_r.active))
                        .rounded(px(t.inner_corner))
                        .bg(paint(t.active)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.end_handle))
                        .top(px(y_handle))
                        .w(px(paint_r.handle_w.max(12.0)))
                        .h(px(t.handle_h_visual))
                        .rounded(px(2.))
                        .bg(paint(t.handle)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.end_handle + paint_r.handle_w.max(12.0)))
                        .top(px(y_track))
                        .h(px(t.track_h))
                        .w(px(paint_r.right))
                        .rounded(px(t.track_corner))
                        .bg(paint(t.inactive)),
                )
                .children(slider::range_tick_fractions().into_iter().map(|frac| {
                    let x = (frac * total - t.stop_dp / 2.0).max(0.0);
                    let active = slider::range_tick_active(frac, range.start, range.end);
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(y_track + (t.track_h - t.stop_dp) / 2.0))
                        .w(px(t.stop_dp))
                        .h(px(t.stop_dp))
                        .rounded(px(t.stop_dp / 2.0))
                        .bg(paint(if active {
                            t.stop_active
                        } else {
                            t.stop_inactive
                        }))
                }))
                .child({
                    let hit = this.range_hit.clone();
                    canvas(
                        move |bounds, _, _| {
                            hit.set((
                                f32::from(bounds.origin.x),
                                f32::from(bounds.size.width).max(1.0),
                            ));
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top(px(0.))
                    .left(px(0.))
                    .w(px(total))
                    .h(px(t.target_dp))
                }),
        )
}

fn settings_scene(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let title = theme.typography.title_large.emphasized();
    let section = theme.typography.title_medium.emphasized();
    let outlined = this.outlined.appearance(theme);
    div()
        .w_full()
        .p(px(button_group::SETTINGS_PAD_DP))
        .rounded(px(button_group::SETTINGS_CORNER_DP))
        .bg(paint(theme.color.surface_container))
        .flex()
        .flex_col()
        .gap(px(button_group::SETTINGS_GROUP_GAP_DP))
        .child(
            div()
                .font_weight(type_weight(title))
                .child(spaced_line(
                    button_group::SETTINGS_SCENE_TITLE,
                    title.size_sp,
                    paint(theme.color.on_surface),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(button_group::SETTINGS_ROW_GAP_DP))
                .child(
                    div()
                        .font_weight(type_weight(section))
                        .child(spaced_line(
                            button_group::SETTINGS_VOLUME_TITLE,
                            section.size_sp,
                            paint(theme.color.on_surface),
                        )),
                )
                .child(volume_slider_scene(theme, this.slider, cx)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(button_group::SETTINGS_ROW_GAP_DP))
                .child(
                    div()
                        .font_weight(type_weight(section))
                        .child(spaced_line(
                            button_group::SETTINGS_QUIET_HOURS_TITLE,
                            section.size_sp,
                            paint(theme.color.on_surface),
                        )),
                )
                .child(field_block(
                    "settings-email",
                    &outlined,
                    "Email",
                    this.outlined.display_with_caret(),
                    if this.outlined.error {
                        "Enter a valid email"
                    } else {
                        "Supporting text"
                    },
                    cx.listener(|this, _, _, cx| {
                        this.filled.set_focus(false);
                        this.outlined.set_focus(true);
                        cx.notify();
                    }),
                ))
                .child(connected_button_group(theme, this.group_selected, cx)),
        )
        .child(m_button(
            "settings-reset",
            theme,
            button::ButtonVariant::Text,
            InteractionState::Enabled,
            "Reset settings",
            cx.listener(|this, _, _, cx| {
                this.overlay = Overlay::Dialog;
                cx.notify();
            }),
        ))
}

fn docked_date_picker(
    this: &CatalogView,
    theme: &Theme,
    pick: &date_picker::DatePickerAppearance,
    cells: &[(u32, DayKind); 42],
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        if this.docked_open {
            InteractionState::Focused
        } else {
            InteractionState::Enabled
        },
        true,
    );
    let cal_w = pick.day_dp * 7.0;
    let value = date_picker::docked_field_value(this.selected);
    let popup = this.docked_open.then(|| {
        div()
            .w_full()
            .p(px(12.))
            .rounded_tl(px(8.))
            .rounded_tr(px(pick.corners.top_right))
            .rounded_br(px(pick.corners.bottom_right))
            .rounded_bl(px(pick.corners.bottom_left))
            .bg(paint(pick.container))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(4.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .id("docked-month-prev")
                            .p(px(8.))
                            .child("<")
                            .on_click(cx.listener(|this, _, _, cx| {
                                let (y, m) = date_picker::add_months(
                                    this.picker_year,
                                    this.picker_month,
                                    -1,
                                );
                                this.picker_year = y;
                                this.picker_month = m;
                                cx.notify();
                            })),
                    )
                    .child(spaced_line(
                        date_picker::month_nav_label(this.picker_year, this.picker_month),
                        pick.year_style.size_sp,
                        paint(pick.header_year),
                    ))
                    .child(
                        div()
                            .id("docked-month-next")
                            .p(px(8.))
                            .child(">")
                            .on_click(cx.listener(|this, _, _, cx| {
                                let (y, m) = date_picker::add_months(
                                    this.picker_year,
                                    this.picker_month,
                                    1,
                                );
                                this.picker_year = y;
                                this.picker_month = m;
                                cx.notify();
                            })),
                    ),
            )
            .child(weekday_row(pick, cal_w))
            .child(div().w(px(cal_w)).flex().flex_wrap().children(
                cells.iter().copied().enumerate().map(|(i, (day, kind))| {
                    let (bg, fg, radius) = day_colors(pick, kind);
                    let in_month = kind != DayKind::OutOfMonth;
                    let year = this.picker_year;
                    let month = this.picker_month;
                    div()
                        .id(SharedString::from(format!("docked-day-{i}")))
                        .w(px(pick.day_dp))
                        .h(px(pick.day_dp))
                        .rounded(px(radius))
                        .bg(bg)
                        .text_color(fg)
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(kind == DayKind::Today, |el| {
                            el.border_1().border_color(paint(pick.day_today_outline))
                        })
                        .child(day.to_string())
                        .when(in_month, |el| {
                            el.on_click(cx.listener(move |this, _, _, cx| {
                                this.selected = CivilDate { year, month, day };
                                if date_picker::DOCKED_DISMISS_ON_SELECT {
                                    this.docked_open = false;
                                }
                                cx.notify();
                            }))
                        })
                }),
            ))
            .into_any_element()
    });
    div()
        .id("docked-date")
        .flex()
        .flex_col()
        .gap(px(4.))
        .w(px(cal_w + 32.0))
        .on_mouse_down_out(cx.listener(|this, _, _, cx| {
            if date_picker::DOCKED_DISMISS_ON_OUTSIDE && this.docked_open {
                this.docked_open = false;
                cx.notify();
            }
        }))
        .child(field_block(
            "docked-date-field",
            &field,
            date_picker::DOCKED_FIELD_LABEL,
            value,
            "",
            cx.listener(|this, _, _, cx| {
                this.docked_open = !this.docked_open;
                cx.notify();
            }),
        ))
        .children(popup)
}

fn slider_stop(slide: &slider::SliderAppearance, active: bool) -> impl IntoElement {
    div()
        .w(px(slide.stop_dp))
        .h(px(slide.stop_dp))
        .rounded(px(slide.stop_dp / 2.0))
        .bg(paint(if active {
            slide.stop_active
        } else {
            slide.stop_inactive
        }))
}

fn expressive_slider(slide: &slider::SliderAppearance) -> impl IntoElement {
    let fractions = slider::stop_fractions(slide.stop_count);
    let rail = div()
        .w_full()
        .h(px(slide.target_dp))
        .flex()
        .items_center()
        .child(
            div()
                .h(px(slide.track_h))
                .w(px(180. * slide.value.max(0.12)))
                .rounded(px(slide.track_corner))
                .bg(paint(slide.active)),
        )
        .child(
            div()
                .mx(px(slide.gap_dp))
                .w(px(slide.handle_w))
                .h(px(slide.handle_h_visual))
                .rounded(px(2.))
                .bg(paint(slide.handle)),
        )
        .child(
            div()
                .h(px(slide.track_h))
                .flex_1()
                .rounded(px(slide.track_corner))
                .bg(paint(slide.inactive)),
        );
    if slide.stop_count <= 2 {
        return rail.into_any_element();
    }
    // Unified rail overlay: even mid-stops across the full track (official Alarm).
    div()
        .w_full()
        .flex()
        .flex_col()
        .child(rail)
        .child(
            div()
                .w_full()
                .h(px(slide.target_dp))
                .mt(px(-slide.target_dp))
                .px(px(8.))
                .flex()
                .items_center()
                .justify_between()
                .children(fractions.into_iter().map(|frac| {
                    slider_stop(slide, frac <= slide.value + 0.001)
                })),
        )
        .into_any_element()
}

fn volume_slider_scene(
    theme: &Theme,
    media_value: f32,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(12.))
        .children(slider::OVERVIEW_ROWS.iter().copied().map(|row| {
            let value = if row.label.starts_with("Media") {
                media_value
            } else {
                row.value
            };
            let slide = slider::resolve_with_stops(
                theme,
                value,
                InteractionState::Enabled,
                row.stop_count,
            );
            let interactive = row.label.starts_with("Media");
            div()
                .id(SharedString::from(row.label))
                .w_full()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(
                    div()
                        .w(px(24.))
                        .h(px(24.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(paint(theme.color.on_surface_variant))
                        .child(row.icon),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .gap(px(4.))
                        .child(spaced_line(
                            row.label,
                            12.0,
                            paint(theme.color.on_surface),
                        ))
                        .child(expressive_slider(&slide)),
                )
                .when(interactive, |el| {
                    el.on_click(cx.listener(|this, _, _, cx| {
                        this.slider = ((this.slider + 0.1) * 10.0).round() / 10.0;
                        if this.slider > 1.0 {
                            this.slider = 0.0;
                        }
                        cx.notify();
                    }))
                })
        }))
}

fn dialog_account_row(theme: &Theme, email: &'static str) -> impl IntoElement {
    let c = theme.color;
    div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.))
        .py(px(4.))
        .child(
            div()
                .w(px(40.))
                .h(px(40.))
                .rounded(px(20.))
                .bg(paint(c.secondary_container))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_size(px(14.))
                        .text_color(paint(c.on_secondary_container))
                        .child(dialog::account_initials(email)),
                ),
        )
        .child(
            div()
                .text_size(px(14.))
                .text_color(paint(c.on_surface))
                .child(email.to_string()),
        )
}

fn dismiss_overlay(
    this: &mut CatalogView,
    _: &gpui::ClickEvent,
    _: &mut Window,
    cx: &mut Context<CatalogView>,
) {
    this.overlay = Overlay::None;
    cx.notify();
}

fn dialog_overlay(theme: &Theme, cx: &mut Context<CatalogView>) -> impl IntoElement {
    let a = dialog::resolve(theme);
    div()
        .id("dialog-scrim")
        .flex_1()
        .w_full()
        .bg(paint(a.scrim))
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.))
        .on_click(cx.listener(dismiss_overlay))
        .child(
            div()
                .id("dialog-card")
                .w(px(a.min_width_dp + 40.0))
                .min_w(px(a.min_width_dp))
                .p(px(a.pad_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(a.container))
                .gap(px(16.))
                .flex()
                .flex_col()
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_center()
                        .text_size(px(dialog::ICON_DP))
                        .text_color(paint(a.icon))
                        .child(dialog::RESET_ICON),
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_center()
                        .font_weight(type_weight(a.headline_style))
                        .child(spaced_line(
                            dialog::RESET_HEADLINE,
                            a.headline_style.size_sp,
                            paint(a.headline),
                        )),
                )
                .child(
                    div()
                        .w_full()
                        .child(spaced_line(
                            dialog::RESET_SUPPORTING,
                            a.supporting_style.size_sp,
                            paint(a.supporting),
                        )),
                )
                .children(dialog::RESET_ACCOUNTS.iter().map(|email| {
                    dialog_account_row(theme, email)
                }))
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_end()
                        .gap(px(dialog::ACTION_GAP_DP))
                        .child(m_button(
                            "dialog-cancel",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RESET_CANCEL,
                            cx.listener(dismiss_overlay),
                        ))
                        .child(m_button(
                            "dialog-ok",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RESET_ACCEPT,
                            cx.listener(dismiss_overlay),
                        )),
                ),
        )
}

fn list_dialog_overlay(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = dialog::resolve(theme);
    div()
        .id("dialog-list-scrim")
        .flex_1()
        .w_full()
        .bg(paint(a.scrim))
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.))
        .on_click(cx.listener(dismiss_overlay))
        .child(
            div()
                .id("dialog-list-card")
                .w(px(a.min_width_dp + 40.0))
                .min_w(px(a.min_width_dp))
                .p(px(a.pad_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(a.container))
                .gap(px(8.))
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(type_size(a.headline_style))
                        .font_weight(type_weight(a.headline_style))
                        .text_color(paint(a.headline))
                        .child(dialog::RINGTONE_HEADLINE),
                )
                .children(dialog::RINGTONE_OPTIONS.iter().enumerate().map(|(i, label)| {
                    let selected = this.ringtone == i;
                    let r = radio::resolve(theme, selected, InteractionState::Enabled);
                    div()
                        .id(SharedString::from(format!("ringtone-{i}")))
                        .w_full()
                        .h(px(r.target_dp))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(theme.typography.body_large.size_sp))
                                .text_color(paint(r.label))
                                .child(*label),
                        )
                        .child(
                            div()
                                .w(px(r.outer_dp))
                                .h(px(r.outer_dp))
                                .rounded(px(r.outer_dp / 2.0))
                                .border_2()
                                .border_color(paint(r.ring))
                                .flex()
                                .items_center()
                                .justify_center()
                                .when(r.inner.is_some(), |el| {
                                    el.child(
                                        div()
                                            .w(px(r.inner_dp))
                                            .h(px(r.inner_dp))
                                            .rounded(px(r.inner_dp / 2.0))
                                            .bg(paint(r.inner.unwrap())),
                                    )
                                }),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.ringtone = i;
                            cx.notify();
                        }))
                }))
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_end()
                        .gap(px(dialog::ACTION_GAP_DP))
                        .child(m_button(
                            "list-cancel",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RINGTONE_CANCEL,
                            cx.listener(dismiss_overlay),
                        ))
                        .child(m_button(
                            "list-ok",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RINGTONE_OK,
                            cx.listener(dismiss_overlay),
                        )),
                ),
        )
}

fn day_colors(
    pick: &date_picker::DatePickerAppearance,
    kind: DayKind,
) -> (gpui::Rgba, gpui::Rgba, f32) {
    match kind {
        DayKind::Selected => (
            paint(pick.day_selected_container),
            paint(pick.day_selected),
            pick.day_dp / 2.0,
        ),
        DayKind::InRange => (
            paint(pick.day_range_container),
            paint(pick.day_range),
            0.0,
        ),
        DayKind::Today => (paint(pick.container), paint(pick.day), pick.day_dp / 2.0),
        DayKind::InMonth => (paint(pick.container), paint(pick.day), pick.day_dp / 2.0),
        DayKind::OutOfMonth => (paint(pick.container), paint(pick.day_out), pick.day_dp / 2.0),
    }
}

fn weekday_row(pick: &date_picker::DatePickerAppearance, cal_w: f32) -> impl IntoElement {
    div().w(px(cal_w)).flex().flex_wrap().children(
        date_picker::WEEKDAYS.iter().map(|d| {
            div()
                .w(px(pick.day_dp))
                .h(px(32.))
                .flex()
                .items_center()
                .justify_center()
                .text_color(paint(pick.weekday))
                .child(*d)
        }),
    )
}

fn date_range_hero(_theme: &Theme, pick: &date_picker::DatePickerAppearance) -> impl IntoElement {
    let start = date_picker::RANGE_DEMO_START;
    let end = date_picker::RANGE_DEMO_END;
    let today = CivilDate {
        year: 2026,
        month: 9,
        day: 11,
    };
    let cells = date_picker::month_grid_range(start.year, start.month, start, end, today);
    let cal_w = pick.day_dp * 7.0;
    div()
        .w(px(cal_w + 32.0))
        .p(px(16.))
        .rounded(px(pick.corners.top_left))
        .bg(paint(pick.container))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(spaced_line(
            date_picker::RANGE_HERO_TITLE,
            pick.year_style.size_sp,
            paint(pick.header_year),
        ))
        .child(
            div()
                .font_weight(type_weight(pick.date_style))
                .child(spaced_line(
                    date_picker::header_range_label(start, end),
                    pick.date_style.size_sp.min(28.0),
                    paint(pick.header_date),
                )),
        )
        .child(spaced_line(
            date_picker::month_nav_label(start.year, start.month),
            pick.year_style.size_sp,
            paint(pick.header_year),
        ))
        .child(weekday_row(pick, cal_w))
        .child(div().w(px(cal_w)).flex().flex_wrap().children(
            cells.iter().copied().map(|(day, kind)| {
                let (bg, fg, radius) = day_colors(pick, kind);
                div()
                    .w(px(pick.day_dp))
                    .h(px(pick.day_dp))
                    .rounded(px(radius))
                    .bg(bg)
                    .text_color(fg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(kind == DayKind::Today, |el| {
                        el.border_1().border_color(paint(pick.day_today_outline))
                    })
                    .child(day.to_string())
            }),
        ))
}

fn date_picker_card(
    this: &CatalogView,
    theme: &Theme,
    pick: &date_picker::DatePickerAppearance,
    cells: &[(u32, DayKind); 42],
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let cal_w = pick.day_dp * 7.0;
    div()
        .w(px(cal_w + 32.0))
        .p(px(16.))
        .rounded(px(pick.corners.top_left))
        .bg(paint(pick.container))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(spaced_line(
            "Select date",
            pick.year_style.size_sp,
            paint(pick.header_year),
        ))
        .child(
            div()
                .font_weight(type_weight(pick.date_style))
                .child(spaced_line(
                    date_picker::header_date_label(this.selected),
                    pick.date_style.size_sp,
                    paint(pick.header_date),
                )),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .id("month-prev")
                        .p(px(8.))
                        .child("<")
                        .on_click(cx.listener(|this, _, _, cx| {
                            let (y, m) = date_picker::add_months(
                                this.picker_year,
                                this.picker_month,
                                -1,
                            );
                            this.picker_year = y;
                            this.picker_month = m;
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .id("year-control")
                        .child(spaced_line(
                            date_picker::month_nav_label(this.picker_year, this.picker_month),
                            pick.year_style.size_sp,
                            paint(pick.header_year),
                        ))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.picker_year += 1;
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .id("month-next")
                        .p(px(8.))
                        .child(">")
                        .on_click(cx.listener(|this, _, _, cx| {
                            let (y, m) = date_picker::add_months(
                                this.picker_year,
                                this.picker_month,
                                1,
                            );
                            this.picker_year = y;
                            this.picker_month = m;
                            cx.notify();
                        })),
                ),
        )
        .child(weekday_row(pick, cal_w))
        .child(div().w(px(cal_w)).flex().flex_wrap().children(
            cells.iter().copied().enumerate().map(|(i, (day, kind))| {
                let (bg, fg, radius) = day_colors(pick, kind);
                let in_month = kind != DayKind::OutOfMonth;
                let year = this.picker_year;
                let month = this.picker_month;
                div()
                    .id(SharedString::from(format!("day-{i}")))
                    .w(px(pick.day_dp))
                    .h(px(pick.day_dp))
                    .rounded(px(radius))
                    .bg(bg)
                    .text_color(fg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(kind == DayKind::Today, |el| {
                        el.border_1().border_color(paint(pick.day_today_outline))
                    })
                    .child(day.to_string())
                    .when(in_month, |el| {
                        el.on_click(cx.listener(move |this, _, _, cx| {
                            this.selected = CivilDate { year, month, day };
                            cx.notify();
                        }))
                    })
            }),
        ))
        .child(
            div()
                .w_full()
                .flex()
                .justify_end()
                .gap(px(dialog::ACTION_GAP_DP))
                .child(m_button(
                    "date-cancel",
                    theme,
                    button::ButtonVariant::Text,
                    InteractionState::Enabled,
                    "Cancel",
                    |_, _, _| {},
                ))
                .child(m_button(
                    "date-ok",
                    theme,
                    button::ButtonVariant::Text,
                    InteractionState::Enabled,
                    "OK",
                    |_, _, _| {},
                )),
        )
}

fn tab_row(
    a: &tabs::TabsAppearance,
    selected: usize,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let labels = ["One", "Two", "Three"];
    div()
        .w_full()
        .h(px(a.height_dp))
        .bg(paint(a.container))
        .flex()
        .children(labels.into_iter().enumerate().map(|(i, label)| {
            let active = selected == i;
            div()
                .id(SharedString::from(format!("tab-{i}")))
                .flex_1()
                .h_full()
                .flex()
                .flex_col()
                .items_center()
                .justify_end()
                .pb(px(8.))
                .text_color(paint(if active {
                    a.active_label
                } else {
                    a.inactive_label
                }))
                .child(label)
                .when(active, |el| {
                    el.child(
                        div()
                            .mt(px(4.))
                            .h(px(a.indicator_h))
                            .w(px(if a.indicator_full_width { 96. } else { 48. }))
                            .rounded_tl(px(3.))
                            .rounded_tr(px(3.))
                            .bg(paint(a.indicator)),
                    )
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.tab = i;
                    cx.notify();
                }))
        }))
}

fn m_button(
    id: &'static str,
    theme: &Theme,
    variant: button::ButtonVariant,
    state: InteractionState,
    label: impl Into<SharedString>,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let a = button::resolve(theme, variant, state);
    let disabled = state.is_disabled();
    div()
        .id(id)
        .h(px(a.height_dp))
        .px(px(a.pad_start_dp))
        .flex()
        .items_center()
        .justify_center()
        .when(a.container.a() > 0, |el| {
            el.rounded(px(a.corners.top_left))
                .bg(paint(a.container))
        })
        .text_color(paint(a.content))
        .text_size(type_size(a.label_style))
        .font_weight(FontWeight::MEDIUM)
        .when(a.outline.is_some(), |el| {
            let (color, _) = a.outline.unwrap();
            el.border_1().border_color(paint(color))
        })
        .child(spaced_line(
            label.into().to_string(),
            a.label_style.size_sp,
            paint(a.content),
        ))
        .when(!disabled, |el| el.on_click(on_click))
}

fn search_bar_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let open = this.search_open;
    let a = search::resolve(theme);
    let view = if open {
        search::resolve_activity(theme)
    } else {
        search::resolve_view(theme)
    };
    let suggestions = search::filter_suggestions(this.search.value());
    let query_label = if this.search.focused {
        this.search.display_with_caret()
    } else {
        search::query_display(this.search.value()).to_string()
    };
    let morph_ms = search::morph_ms(theme) as u64;
    let docked_bg = a.bar.container;
    let activity_bg = view.container;
    let query_color = paint(if open && !this.search.value().is_empty() {
        view.input
    } else if open {
        view.placeholder
    } else {
        a.placeholder
    });
    let lead_color = paint(a.leading_icon);
    let back_color = paint(view.header);
    let avatar_bg = paint(a.avatar);
    let avatar_fg = paint(a.avatar_label);
    let mic_color = paint(if open { view.header } else { a.trailing_icon });
    let header = div()
        .id(if open { "search-activity" } else { "search-bar" })
        .w_full()
        .px(px(16.))
        .flex()
        .items_center()
        .gap(px(search::GAP_DP))
        .when(!open, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                this.search_open = true;
                this.search.set_focus(true);
                cx.notify();
            }))
        })
        .with_animation(
            if open {
                "search-header-in"
            } else {
                "search-header-out"
            },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let linear = if open { delta } else { 1.0 - delta };
                let frame = search::morph_frame_eased(linear);
                this.h(px(frame.header_h_dp))
            },
        )
        .child(
            div()
                .relative()
                .w(px(search::ICON_DP))
                .h(px(search::ICON_DP))
                .child(
                    div()
                        .id("search-lead-docked")
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(search::ICON_DP))
                        .h(px(search::ICON_DP))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(lead_color)
                        .child(search::LEADING_ICON)
                        .with_animation(
                            if open {
                                "search-lead-out"
                            } else {
                                "search-lead-in"
                            },
                            Animation::new(Duration::from_millis(morph_ms)),
                            move |this, delta| {
                                let linear = if open { delta } else { 1.0 - delta };
                                this.opacity(search::morph_avatar_opacity(
                                    search::morph_eased_t(linear),
                                ))
                            },
                        ),
                )
                .child(
                    div()
                        .id("search-back")
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(search::ICON_DP))
                        .h(px(search::ICON_DP))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(back_color)
                        .child(search::VIEW_BACK)
                        .when(open, |el| {
                            el.on_click(cx.listener(|this, _, _, cx| {
                                this.search_open = false;
                                this.search.set_value("");
                                this.search.set_focus(false);
                                cx.notify();
                            }))
                        })
                        .with_animation(
                            if open {
                                "search-back-in"
                            } else {
                                "search-back-out"
                            },
                            Animation::new(Duration::from_millis(morph_ms)),
                            move |this, delta| {
                                let linear = if open { delta } else { 1.0 - delta };
                                this.opacity(search::morph_back_opacity(
                                    search::morph_eased_t(linear),
                                ))
                            },
                        ),
                ),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(a.placeholder_style.size_sp))
                .text_color(query_color)
                .child(query_label),
        )
        .child(
            div()
                .w(px(search::ICON_DP))
                .text_color(mic_color)
                .child(search::TRAILING_MIC),
        )
        .child(
            div()
                .w(px(search::AVATAR_DP))
                .h(px(search::AVATAR_DP))
                .rounded(px(search::AVATAR_DP / 2.0))
                .bg(avatar_bg)
                .text_color(avatar_fg)
                .flex()
                .items_center()
                .justify_center()
                .child("A")
                .with_animation(
                    if open {
                        "search-avatar-out"
                    } else {
                        "search-avatar-in"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let linear = if open { delta } else { 1.0 - delta };
                        this.opacity(search::morph_avatar_opacity(search::morph_eased_t(linear)))
                    },
                ),
        )
        .into_any_element();
    let rows: Vec<_> = if suggestions.is_empty() {
        vec![search::EMPTY_SUGGESTIONS]
    } else {
        suggestions
    };
    let list = div()
        .flex()
        .flex_col()
        .child(div().h(px(1.)).w_full().bg(paint(view.divider)))
        .children(rows.into_iter().enumerate().map(|(i, label)| {
            div()
                .id(SharedString::from(format!("search-sug-{i}")))
                .h(px(view.suggestion_h_dp))
                .px(px(16.))
                .flex()
                .items_center()
                .gap(px(16.))
                .text_color(paint(view.suggestion))
                .child(
                    div()
                        .text_color(paint(view.suggestion_icon))
                        .child(if i == 0 { "⌕" } else { "◌" }),
                )
                .child(label)
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(picked) = search::pick_suggestion(this.search.value(), i) {
                        this.search.set_value(picked);
                        cx.notify();
                    }
                }))
        }))
        .into_any_element();
    div()
        .id("search-morph")
        .relative()
        .w_full()
        .flex()
        .flex_col()
        .overflow_hidden()
        .tab_index(0)
        .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
            if this.search_open {
                search::apply_key_to_editor(&mut this.search, &ev.keystroke.key);
                cx.notify();
            }
        }))
        .with_animation(
            if open { "search-grow" } else { "search-shrink" },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let linear = if open { delta } else { 1.0 - delta };
                let frame = search::morph_frame_eased(linear);
                this.min_h(px(frame.height_dp))
                    .rounded(px(frame.corner_dp))
                    .ml(px(frame.inset_h_dp))
                    .mr(px(frame.inset_h_dp))
            },
        )
        .child({
            let settled = search::morph_frame_at(search::morph_t(open));
            let fill = paint(docked_bg.lerp(activity_bg, settled.t));
            let scale = search::morph_layer_transform(settled).scale;
            let corner = settled.corner_dp;
            canvas(
                move |_, _, _| scale,
                move |bounds, scale, window, _| {
                    paint_search_scaled_fill(window, bounds, scale, corner, fill);
                },
            )
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .size_full()
        })
        .child(header)
        .child(
            div()
                .with_animation(
                    if open {
                        "search-list-in"
                    } else {
                        "search-list-out"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if open { delta } else { 1.0 - delta };
                        this.opacity(search::morph_list_opacity(t))
                    },
                )
                .child(list),
        )
}

fn time_picker_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = time_picker::resolve(theme);
    let clock = a.clock_dp * 0.75;
    let number = 32.0;
    let hour_on = this.time_dial == DialFace::Hour;
    let labels: Vec<(u8, String, f32, f32, bool)> = match this.time_dial {
        DialFace::Hour => (1u8..=12)
            .map(|hour| {
                let (x, y) = time_picker::hour_offset(hour, clock, number);
                (hour, hour.to_string(), x, y, hour == this.time_hour)
            })
            .collect(),
        DialFace::Minute => time_picker::minute_labels()
            .map(|m| {
                let (x, y) = time_picker::minute_offset(m, clock, number);
                (m, format!("{m:02}"), x, y, m == this.time_minute)
            })
            .collect(),
    };
    let from_angle = this.time_hand_from;
    let to_angle = time_picker::hand_angle_deg(
        this.time_dial,
        this.time_hour,
        this.time_minute,
    );
    let hand_gen = this.time_hand_gen;
    let hand_ms = time_picker::hand_motion_ms(theme) as u64;
    let hour_live = hour_on;
    let live_hour = this.time_hour;
    let live_minute = this.time_minute;
    let hub = (clock / 2.0, clock / 2.0);
    let hand_color = paint(a.hand);
    div()
        .w_full()
        .p(px(time_picker::CONTAINER_PAD_DP))
        .rounded(px(a.corners.top_left))
        .bg(paint(a.container))
        .shadow_md()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(spaced_line(
            time_picker::TITLE,
            a.title_style.size_sp,
            paint(a.header),
        ))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.))
                        .child(
                            div()
                                .id("time-hour-field")
                                .px(px(8.))
                                .p(px(4.))
                                .rounded(px(8.))
                                .bg(paint(if hour_on {
                                    a.number_selected_container
                                } else {
                                    a.clock
                                }))
                                .text_color(paint(if hour_on {
                                    a.number_selected
                                } else {
                                    a.header
                                }))
                                .font_weight(type_weight(a.time_style))
                                .child(time_picker::format_hour_field(this.time_hour))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.bump_time_hand();
                                    this.time_dial = DialFace::Hour;
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .font_weight(type_weight(a.time_style))
                                .text_color(paint(a.header))
                                .child(":"),
                        )
                        .child(
                            div()
                                .id("time-minute-field")
                                .px(px(8.))
                                .p(px(4.))
                                .rounded(px(8.))
                                .bg(paint(if !hour_on {
                                    a.number_selected_container
                                } else {
                                    a.clock
                                }))
                                .text_color(paint(if !hour_on {
                                    a.number_selected
                                } else {
                                    a.header
                                }))
                                .font_weight(type_weight(a.time_style))
                                .child(time_picker::format_minute_field(this.time_minute))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.bump_time_hand();
                                    this.time_dial = DialFace::Minute;
                                    cx.notify();
                                })),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(time_picker::PERIOD_GAP_DP))
                        .children([DayPeriod::Am, DayPeriod::Pm].into_iter().map(|period| {
                            let selected = this.time_period == period;
                            div()
                                .id(SharedString::from(period.label()))
                                .w(px(time_picker::PERIOD_W_DP))
                                .h(px(time_picker::PERIOD_H_DP))
                                .rounded(px(8.))
                                .bg(paint(if selected {
                                    a.period_selected_container
                                } else {
                                    a.period_idle_container
                                }))
                                .text_color(paint(if selected {
                                    a.period_selected
                                } else {
                                    a.period_idle
                                }))
                                .font_weight(type_weight(a.period_style))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(period.label())
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.time_period = period;
                                    cx.notify();
                                }))
                        })),
                ),
        )
        .child(
            div()
                .relative()
                .w(px(clock))
                .h(px(clock))
                .rounded(px(clock / 2.0))
                .bg(paint(a.clock))
                .child(
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(clock))
                        .h(px(clock))
                        .with_animation(
                            SharedString::from(format!(
                                "time-hand-{hand_gen}-{}",
                                if hour_live { "live" } else { "once" }
                            )),
                            if hour_live {
                                Animation::new(Duration::from_millis(hand_ms.saturating_mul(12)))
                                    .repeat()
                            } else {
                                Animation::new(Duration::from_millis(hand_ms))
                            },
                            move |this, delta| {
                                let angle = if hour_live {
                                    time_picker::hour_face_live_angle_deg(
                                        live_hour,
                                        live_minute,
                                        delta,
                                    )
                                } else {
                                    time_picker::lerp_angle_deg(from_angle, to_angle, delta)
                                };
                                let quad = time_picker::hand_quad_at_angle(clock, angle, number);
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let mut builder = PathBuilder::fill();
                                            for (i, (x, y)) in quad.iter().enumerate() {
                                                let p = point(
                                                    bounds.origin.x + px(*x),
                                                    bounds.origin.y + px(*y),
                                                );
                                                if i == 0 {
                                                    builder.move_to(p);
                                                } else {
                                                    builder.line_to(p);
                                                }
                                            }
                                            builder.close();
                                            if let Ok(path) = builder.build() {
                                                window.paint_path(path, hand_color);
                                            }
                                            let mut hub_b = PathBuilder::fill();
                                            let r = time_picker::HAND_HUB_DP / 2.0;
                                            let n = 12u32;
                                            for i in 0..n {
                                                let ang = i as f32 / n as f32 * std::f32::consts::TAU;
                                                let p = point(
                                                    bounds.origin.x + px(hub.0 + r * ang.cos()),
                                                    bounds.origin.y + px(hub.1 + r * ang.sin()),
                                                );
                                                if i == 0 {
                                                    hub_b.move_to(p);
                                                } else {
                                                    hub_b.line_to(p);
                                                }
                                            }
                                            hub_b.close();
                                            if let Ok(path) = hub_b.build() {
                                                window.paint_path(path, hand_color);
                                            }
                                        },
                                    )
                                    .w(px(clock))
                                    .h(px(clock)),
                                )
                            },
                        ),
                )
                .child({
                    let second_color = hand_color;
                    let period = 1_000u64;
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(clock))
                        .h(px(clock))
                        .with_animation(
                            "time-second-hand",
                            Animation::new(Duration::from_millis(period)).repeat(),
                            move |this, _delta| {
                                let angle = time_picker::second_hand_angle_wall_clock();
                                let quad = time_picker::second_hand_quad(clock, angle, number);
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let mut builder = PathBuilder::fill();
                                            for (i, (x, y)) in quad.iter().enumerate() {
                                                let p = point(
                                                    bounds.origin.x + px(*x),
                                                    bounds.origin.y + px(*y),
                                                );
                                                if i == 0 {
                                                    builder.move_to(p);
                                                } else {
                                                    builder.line_to(p);
                                                }
                                            }
                                            builder.close();
                                            if let Ok(path) = builder.build() {
                                                window.paint_path(path, second_color);
                                            }
                                        },
                                    )
                                    .w(px(clock))
                                    .h(px(clock)),
                                )
                            },
                        )
                })
                .children(labels.into_iter().map(|(value, label, x, y, selected)| {
                    let face = this.time_dial;
                    div()
                        .id(SharedString::from(format!("dial-{value}")))
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(number))
                        .h(px(number))
                        .rounded(px(number / 2.0))
                        .bg(paint(if selected {
                            a.number_selected_container
                        } else {
                            a.clock
                        }))
                        .text_color(paint(if selected {
                            a.number_selected
                        } else {
                            a.number
                        }))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(label)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.bump_time_hand();
                            match face {
                                DialFace::Hour => {
                                    this.time_hour = time_picker::select_hour(this.time_hour, value);
                                    this.time_dial = DialFace::Minute;
                                }
                                DialFace::Minute => {
                                    this.time_minute =
                                        time_picker::select_minute(this.time_minute, value);
                                }
                            }
                            cx.notify();
                        }))
                })),
        )
}

fn nav_rail_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let expanded = this.rail_mode == navigation_rail::RailMode::Expanded;
    let scrim_c = paint(navigation_rail::scrim(theme));
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    div()
        .id("nav-rail-stage")
        .relative()
        .w_full()
        .min_h(px(280.))
        .overflow_hidden()
        .child(
            div()
                .id("rail-scrim")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .bg(scrim_c)
                .when(expanded, |el| {
                    el.on_click(cx.listener(|this, _, _, cx| {
                        this.rail_mode = navigation_rail::RailMode::Collapsed;
                        cx.notify();
                    }))
                })
                .with_animation(
                    if expanded {
                        "rail-scrim-in"
                    } else {
                        "rail-scrim-out"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if expanded { delta } else { 1.0 - delta };
                        this.opacity(t)
                    },
                ),
        )
        .child(
            div()
                .id("nav-rail-window")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .h_full()
                .when(expanded, |el| el.shadow_lg())
                .child(nav_rail_column(this, theme, this.rail_mode, cx)),
        )
}

fn nav_rail_column(
    this: &CatalogView,
    theme: &Theme,
    mode: navigation_rail::RailMode,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let rail = navigation_rail::resolve_mode(theme, mode);
    let expanded = mode == navigation_rail::RailMode::Expanded;
    let selected = this.rail_selected;
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    div()
        .id("nav-rail")
        .overflow_hidden()
        .pt(px(navigation_rail::PAD_TOP_DP))
        .bg(paint(rail.container))
        .flex()
        .flex_col()
        .items_center()
        .gap(px(navigation_rail::DEST_GAP_DP))
        .with_animation(
            if expanded { "rail-expand" } else { "rail-collapse" },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let t = if expanded { delta } else { 1.0 - delta };
                this.w(px(navigation_rail::morph_width_dp(t)))
            },
        )
        .child(
            div()
                .id("rail-fab")
                .w(px(navigation_rail::FAB_SLOT_DP))
                .h(px(navigation_rail::FAB_SLOT_DP))
                .rounded(px(16.))
                .bg(paint(rail.fab))
                .text_color(paint(rail.fab_icon))
                .flex()
                .items_center()
                .justify_center()
                .child(if expanded { "←" } else { "+" })
                .on_click(cx.listener(|this, _, _, cx| {
                    this.rail_mode = navigation_rail::toggle_mode(this.rail_mode);
                    cx.notify();
                })),
        )
        .children(
            navigation_rail::DESTINATIONS
                .iter()
                .zip(navigation_rail::DESTINATION_ICONS.iter())
                .zip(navigation_rail::DESTINATION_BADGES.iter())
                .enumerate()
                .map(|(i, ((label, icon), badge))| {
                    let active = navigation_rail::is_active(selected, i);
                    let mut dest = div()
                        .id(SharedString::from(format!("rail-dest-{i}")))
                        .w(px(rail.width_dp))
                        .relative()
                        .flex()
                        .gap(px(4.))
                        .items_center()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.rail_selected = navigation_rail::select_destination(
                                this.rail_selected,
                                i,
                            );
                            cx.notify();
                        }));
                    dest = if expanded {
                        dest.flex_row().justify_start().px(px(12.))
                    } else {
                        dest.flex_col().justify_center()
                    };
                    dest = dest.child(
                        div()
                            .w(px(navigation_rail::INDICATOR_W_DP))
                            .h(px(navigation_rail::INDICATOR_H_DP))
                            .rounded(px(navigation_rail::INDICATOR_H_DP / 2.0))
                            .bg(paint(if active {
                                rail.active_indicator
                            } else {
                                rail.container
                            }))
                            .text_color(paint(if active {
                                rail.active_icon
                            } else {
                                rail.inactive_icon
                            }))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(*icon),
                    );
                    dest = dest.child(
                        div()
                            .text_size(px(rail.label_style.size_sp))
                            .text_color(paint(if active {
                                rail.active_label
                            } else {
                                rail.inactive_label
                            }))
                            .child(*label),
                    );
                    match badge {
                        Some(0) => dest.child(
                            div()
                                .absolute()
                                .top(px(2.))
                                .right(px(18.))
                                .w(px(6.))
                                .h(px(6.))
                                .rounded(px(3.))
                                .bg(paint(rail.badge)),
                        ),
                        Some(n) => dest.child(
                            div()
                                .absolute()
                                .top(px(2.))
                                .right(px(12.))
                                .min_w(px(16.))
                                .h(px(16.))
                                .px(px(4.))
                                .rounded(px(8.))
                                .bg(paint(rail.badge))
                                .text_color(paint(rail.badge_label))
                                .text_size(px(10.))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(badge::label_for_count(*n)),
                        ),
                        None => dest,
                    }
                }),
        )
}

fn progress_heroes(theme: &Theme) -> impl IntoElement {
    let lin = progress::linear(theme, 0.6);
    let indet = progress::linear_indeterminate(theme);
    let wave = progress::wavy(theme, progress::WAVE_DEMO_PROGRESS);
    let span = indet.head_span;
    let ind_color = paint(indet.indicator);
    let wave_color = paint(wave.indicator);
    let wave_w = wave.width_dp;
    let wave_h = wave.height_dp;
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            div()
                .w(px(240.))
                .h(px(lin.height_dp))
                .rounded(px(2.))
                .bg(paint(lin.track))
                .child(
                    div()
                        .h_full()
                        .w(px(240. * lin.progress))
                        .bg(paint(lin.indicator)),
                ),
        )
        .child(
            div()
                .w(px(wave_w))
                .h(px(wave_h))
                .bg(paint(wave.track))
                .with_animation(
                    "wavy-progress",
                    Animation::new(Duration::from_millis(wave.duration_ms as u64)).repeat(),
                    move |this, delta| {
                        this.child(
                            canvas(
                                move |_, _, _| {},
                                move |bounds, _, window, _| {
                                    let pts = progress::wave_polyline(
                                        wave_w,
                                        wave_h,
                                        progress::WAVE_DEMO_PROGRESS,
                                        delta,
                                    );
                                    let mut builder = stroke_round(progress::WAVE_STROKE_DP);
                                    for (i, (x, y)) in pts.iter().enumerate() {
                                        let p = point(
                                            bounds.origin.x + px(*x),
                                            bounds.origin.y + px(*y),
                                        );
                                        if i == 0 {
                                            builder.move_to(p);
                                        } else {
                                            builder.line_to(p);
                                        }
                                    }
                                    if let Ok(path) = builder.build() {
                                        window.paint_path(path, wave_color);
                                    }
                                },
                            )
                            .w(px(wave_w))
                            .h(px(wave_h)),
                        )
                    },
                ),
        )
        .child(
            div()
                .relative()
                .w(px(240.))
                .h(px(indet.height_dp))
                .rounded(px(2.))
                .bg(paint(indet.track))
                .with_animation(
                    "indet-head",
                    Animation::new(Duration::from_millis(indet.duration_ms as u64)).repeat(),
                    move |this, delta| {
                        this.child(
                            div()
                                .absolute()
                                .left(px(-80.0 + delta * 320.0))
                                .h_full()
                                .w(px(240.0 * span))
                                .bg(ind_color),
                        )
                    },
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(8.))
                .child({
                    let load = progress::contained_loading_indicator(theme);
                    let box_s = load.contained_dp;
                    let shape_s = load.size_dp;
                    let ptr_color = paint(load.indicator);
                    let box_bg = paint(load.container);
                    let dur = load.duration_ms as u64;
                    div()
                        .w(px(box_s))
                        .h(px(box_s))
                        .rounded(px(box_s / 2.0))
                        .bg(box_bg)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(shape_s))
                                .h(px(shape_s))
                                .with_animation(
                                    "ptr-morph",
                                    Animation::new(Duration::from_millis(dur)).repeat(),
                                    move |this, delta| {
                                        this.child(
                                            canvas(
                                                move |_, _, _| {},
                                                move |bounds, _, window, _| {
                                                    let pts = progress::loading_polygon(shape_s, delta);
                                                    paint_filled_polygon(
                                                        window,
                                                        bounds.origin,
                                                        &pts,
                                                        ptr_color,
                                                    );
                                                },
                                            )
                                            .w(px(shape_s))
                                            .h(px(shape_s)),
                                        )
                                    },
                                ),
                        )
                })
                .child(spaced_line(
                    progress::PTR_LABEL,
                    12.0,
                    paint(theme.color.on_surface_variant),
                )),
        )
        .child({
            let load = progress::loading_indicator(theme);
            let size = load.size_dp;
            let load_color = paint(load.indicator);
            let dur = load.duration_ms as u64;
            let circ = progress::circular_indeterminate(theme);
            let cap_size = circ.size_dp;
            let cap_stroke = circ.stroke_dp;
            let cap_arc = circ.arc_deg;
            let cap_color = paint(circ.indicator);
            let cap_dur = circ.duration_ms as u64;
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(8.))
                        .child(
                            div()
                                .w(px(size))
                                .h(px(size))
                                .with_animation(
                                    "loading-morph",
                                    Animation::new(Duration::from_millis(dur)).repeat(),
                                    move |this, delta| {
                                        this.child(
                                            canvas(
                                                move |_, _, _| {},
                                                move |bounds, _, window, _| {
                                                    let pts = progress::loading_polygon(size, delta);
                                                    paint_filled_polygon(
                                                        window,
                                                        bounds.origin,
                                                        &pts,
                                                        load_color,
                                                    );
                                                },
                                            )
                                            .w(px(size))
                                            .h(px(size)),
                                        )
                                    },
                                ),
                        )
                        .child(spaced_line(
                            progress::LOADING_LABEL,
                            12.0,
                            paint(theme.color.on_surface_variant),
                        )),
                )
                .child({
                    let det_size = load.size_dp;
                    let det_color = paint(load.indicator);
                    let wait_ms = progress::determinate_wait_ms(theme) as u64;
                    let wait_label = paint(theme.color.on_surface_variant);
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(8.))
                        .child(
                            div()
                                .w(px(det_size))
                                .h(px(det_size))
                                .with_animation(
                                    "loading-det-wait",
                                    Animation::new(Duration::from_millis(wait_ms)).repeat(),
                                    move |this, delta| {
                                        let wait = progress::WaitProgress::from_fraction(delta);
                                        let pts = progress::loading_polygon_for_wait(det_size, wait);
                                        this.child(
                                            canvas(
                                                move |_, _, _| {},
                                                move |bounds, _, window, _| {
                                                    paint_filled_polygon(
                                                        window,
                                                        bounds.origin,
                                                        &pts,
                                                        det_color,
                                                    );
                                                },
                                            )
                                            .w(px(det_size))
                                            .h(px(det_size)),
                                        )
                                    },
                                ),
                        )
                        .child(
                            div()
                                .w(px(det_size))
                                .with_animation(
                                    "loading-det-label",
                                    Animation::new(Duration::from_millis(wait_ms)).repeat(),
                                    move |this, delta| {
                                        this.child(spaced_line(
                                            format!("{:.0}%", delta * 100.0),
                                            12.0,
                                            wait_label,
                                        ))
                                    },
                                ),
                        )
                })
                .child(
                    div()
                        .w(px(cap_size))
                        .h(px(cap_size))
                        .with_animation(
                            "circ-indet-cap",
                            Animation::new(Duration::from_millis(cap_dur)).repeat(),
                            move |this, delta| {
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let pts = progress::ptr_arc_polyline(
                                                cap_size, cap_stroke, cap_arc, delta,
                                            );
                                            paint_round_polyline(
                                                window,
                                                bounds.origin,
                                                &pts,
                                                cap_stroke,
                                                cap_color,
                                            );
                                        },
                                    )
                                    .w(px(cap_size))
                                    .h(px(cap_size)),
                                )
                            },
                        ),
                )
        })
}

fn carousel_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = carousel::resolve(theme);
    let selected = this.carousel_index;
    div()
        .id("carousel")
        .relative()
        .w_full()
        .flex()
        .gap(px(a.gap_dp))
        .child(
            div()
                .absolute()
                .w(px(1.))
                .h(px(1.))
                .with_animation(
                    "carousel-live-clock",
                    Animation::new(Duration::from_millis(16)).repeat(),
                    |el, _| el,
                ),
        )
        .on_scroll_wheel(cx.listener(|this, ev: &ScrollWheelEvent, _, cx| {
            let (dx, dy) = match ev.delta {
                ScrollDelta::Pixels(p) => (f32::from(p.x), f32::from(p.y)),
                ScrollDelta::Lines(p) => (p.x, p.y),
            };
            this.carousel_fling.selected = this.carousel_index;
            this.carousel_fling.impulse(dx, dy);
            this.carousel_fling_at = None;
            cx.notify();
        }))
        .children(carousel::ITEMS.iter().enumerate().map(|(i, label)| {
            let w = carousel::item_width_dp(i, selected);
            let (bg, fg) = if i == selected {
                (a.container, a.label)
            } else {
                (a.neighbor, theme.color.on_secondary_container)
            };
            div()
                .id(SharedString::from(format!("carousel-{i}")))
                .w(px(w))
                .h(px(a.height_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(bg))
                .p(px(16.))
                .flex()
                .items_end()
                .text_color(paint(fg))
                .font_weight(type_weight(a.label_style))
                .child(*label)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.carousel_index = carousel::snap_to(i);
                    this.carousel_fling = carousel::FlingState::new(i);
                    this.carousel_fling_at = None;
                    cx.notify();
                }))
        }))
}

fn section_title(theme: &Theme, title: &'static str) -> impl IntoElement {
    let style = theme.typography.title_medium.emphasized();
    div()
        .mt(px(8.))
        .text_size(px(style.size_sp))
        .font_weight(type_weight(style))
        .text_color(paint(theme.color.on_surface))
        .whitespace_nowrap()
        .child(spaced_line(title, style.size_sp, paint(theme.color.on_surface)))
}

fn spaced_line(text: impl AsRef<str>, size: f32, color: gpui::Rgba) -> gpui::AnyElement {
    let text = text.as_ref();
    let words = typography::words(text);
    if words.len() <= 1 {
        return div()
            .text_size(px(size))
            .text_color(color)
            .line_height(px(size + 6.0))
            .child(text.to_string())
            .into_any_element();
    }
    let mut row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_end()
        .gap(px(typography::WORD_GAP_DP));
    for w in words {
        row = row.child(
            div()
                .text_size(px(size))
                .text_color(color)
                .line_height(px(size + 6.0))
                .child(w.to_string()),
        );
    }
    row.into_any_element()
}

fn outlined_notched_field(
    id: &'static str,
    field: &text_field::TextFieldAppearance,
    label: SharedString,
    value: SharedString,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let outline = field.field.outline.unwrap_or((field.label, 1.0));
    let frame = text_field::notch_frame(label.as_ref(), field);
    let fill = field.field.container;
    let (lx, ly) = frame.label_origin_dp();
    let stroke_color = paint(outline.0);
    let h = field.field.height_dp;
    let radius = frame.radius_dp;
    let field_m = field.clone();
    let label_m = label.clone();
    div()
        .id(id)
        .relative()
        .w_full()
        .h(px(h))
        .on_click(on_click)
        .child(
            div()
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .rounded(px(radius))
                .bg(paint(fill)),
        )
        .child(
            canvas(
                move |_, window, _| {
                    let measured = measure_label_width_dp(
                        window,
                        label_m.as_ref(),
                        field_m.label_style.size_sp,
                    );
                    text_field::notch_frame_from_layout(label_m.as_ref(), &field_m, measured)
                },
                move |bounds, frame, window, _| {
                    let w = f32::from(bounds.size.width);
                    let verbs_w = frame.evenodd_verbs(w);
                    let mut fill_b = PathBuilder::fill().with_style(PathStyle::Fill(
                        FillOptions::default().with_fill_rule(FillRule::EvenOdd),
                    ));
                    feed_outline_verbs(&mut fill_b, bounds.origin, verbs_w);
                    if let Ok(path) = fill_b.build() {
                        window.paint_path(path, stroke_color);
                    }
                },
            )
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .size_full(),
        )
        .child(
            div()
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .px(px(16.))
                .flex()
                .items_center()
                .child(
                    div()
                        .text_size(px(field.input_style.size_sp))
                        .text_color(paint(field.input))
                        .child(value),
                ),
        )
        .child(
            div()
                .absolute()
                .left(px(lx))
                .top(px(ly))
                .w(px(frame.width_dp))
                .h(px(frame.label_h_dp))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_size(px(field.label_style.size_sp))
                        .text_color(paint(field.label))
                        .line_height(px(field.label_style.line_height_sp))
                        .child(label),
                ),
        )
}

fn field_block(
    id: &'static str,
    field: &text_field::TextFieldAppearance,
    label: impl Into<SharedString>,
    value: impl Into<SharedString>,
    supporting: impl Into<SharedString>,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let outline = field.field.outline.unwrap_or((field.label, 1.0));
    let outlined = field.field.corners.bottom_left > 0.0;
    let label = label.into();
    let value = value.into();
    let box_el = if outlined && field.notched {
        outlined_notched_field(id, field, label, value, on_click).into_any_element()
    } else if outlined {
        div()
            .id(id)
            .h(px(field.field.height_dp))
            .px(px(16.))
            .rounded(px(field.field.corners.top_left))
            .bg(paint(field.field.container))
            .border_1()
            .border_color(paint(outline.0))
            .flex()
            .items_center()
            .on_click(on_click)
            .child(
                div()
                    .text_size(px(field.label_style.size_sp))
                    .text_color(paint(field.label))
                    .child(label),
            )
            .into_any_element()
    } else if field.floating {
        div()
            .id(id)
            .h(px(field.field.height_dp))
            .px(px(16.))
            .rounded(px(field.field.corners.top_left))
            .bg(paint(field.field.container))
            .flex()
            .flex_col()
            .justify_end()
            .pb(px(8.))
            .on_click(on_click)
            .child(
                div()
                    .text_size(px(field.label_style.size_sp))
                    .text_color(paint(field.label))
                    .child(label),
            )
            .child(
                div()
                    .text_size(px(field.input_style.size_sp))
                    .text_color(paint(field.input))
                    .child(value),
            )
            .child(
                div()
                    .h(px(outline.1.max(1.0)))
                    .w_full()
                    .bg(paint(outline.0)),
            )
            .into_any_element()
    } else {
        div()
            .id(id)
            .h(px(field.field.height_dp))
            .px(px(16.))
            .rounded(px(field.field.corners.top_left))
            .bg(paint(field.field.container))
            .flex()
            .flex_col()
            .on_click(on_click)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(field.label_style.size_sp))
                            .text_color(paint(field.label))
                            .child(label),
                    ),
            )
            .child(
                div()
                    .h(px(outline.1.max(1.0)))
                    .w_full()
                    .bg(paint(outline.0)),
            )
            .into_any_element()
    };
    div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(box_el)
        .child(
            div()
                .px(px(16.))
                .text_size(px(field.supporting_style.size_sp))
                .text_color(paint(field.supporting))
                .child(supporting.into()),
        )
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(720.), px(880.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Material 3 desktop catalog".into()),
                    ..Default::default()
                }),
                app_id: Some("dev.gpui.material_desktop_demo".into()),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| CatalogView {
                    dark: false,
                    taps: 0,
                    checked: true,
                    switched: true,
                    overlay: Overlay::None,
                    slider: slider::OVERVIEW_ROWS[3].value,
                    ringtone: 2,
                    tab: 0,
                    picker_year: 2026,
                    picker_month: 9,
                    selected: CivilDate {
                        year: 2026,
                        month: 9,
                        day: 15,
                    },
                    today: CivilDate {
                        year: 2026,
                        month: 9,
                        day: 11,
                    },
                    filled: TextFieldEditor::new(text_field::TextFieldVariant::Filled, "Input text"),
                    outlined: TextFieldEditor::new(
                        text_field::TextFieldVariant::Outlined,
                        "you@domain.com",
                    ),
                    group_selected: button_group::DEMO_SELECTED,
                    range_start: slider::RANGE_DEMO_START,
                    range_end: slider::RANGE_DEMO_END,
                    range_drag: None,
                    range_focus: slider::RangeThumb::Start,
                    range_moved: false,
                    range_hit: Rc::new(Cell::new((0.0, slider::RANGE_TRACK_W_DP))),
                    docked_open: date_picker::DOCKED_OPEN_BY_DEFAULT,
                    search_open: search::VIEW_OPEN_BY_DEFAULT,
                    search: {
                        let mut ed = TextFieldEditor::new(
                            text_field::TextFieldVariant::Filled,
                            "",
                        );
                        ed.set_focus(search::VIEW_OPEN_BY_DEFAULT);
                        ed
                    },
                    rail_selected: navigation_rail::DEMO_SELECTED,
                    rail_mode: navigation_rail::DEMO_MODE,
                    carousel_index: carousel::DEMO_INDEX,
                    carousel_fling: carousel::FlingState::new(carousel::DEMO_INDEX),
                    carousel_fling_at: None,
                    time_hour: time_picker::DEMO_HOUR,
                    time_minute: time_picker::DEMO_MINUTE,
                    time_period: time_picker::DEMO_PERIOD,
                    time_dial: time_picker::DEMO_DIAL,
                    time_hand_from: time_picker::hand_angle_deg(
                        time_picker::DEMO_DIAL,
                        time_picker::DEMO_HOUR,
                        time_picker::DEMO_MINUTE,
                    ),
                    time_hand_gen: 0,
                })
            },
        )
        .expect("failed to open Material desktop catalog window");
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use super::{nav_rail_os_popup_options, WindowKind};
    use gpui_material::components::{
                button, button_group, carousel, dialog, navigation_rail, progress, search, slider,
                text_field, time_picker,
    };
    use gpui_material::theme::Theme;
    use gpui_material::InteractionState;

    #[test]
    fn desktop_heroes_use_shared_resolve_not_local_constants() {
        let theme = Theme::light();
        let btn = button::resolve(
            &theme,
            button::ButtonVariant::Filled,
            InteractionState::Enabled,
        );
        assert_eq!(btn.height_dp, 40.0);
        assert_eq!(btn.pad_start_dp, 16.0);
        assert_eq!(btn.container, theme.color.primary);

        let field = text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Outlined,
            InteractionState::Focused,
            true,
        );
        assert!(field.notched);
        assert_eq!(field.field.outline, Some((theme.color.primary, 2.0)));

        let s = slider::resolve(&theme, 0.55, InteractionState::Enabled);
        assert_eq!(s.track_h, 16.0);
        assert_eq!(s.handle_w, 4.0);
        assert_eq!(s.handle_h, 44.0);
        assert_eq!(s.handle_h_visual, 28.0);
        assert_eq!(s.inactive, theme.color.surface_container_highest);
        assert_eq!(s.stop_count, 2);
        let alarm = slider::resolve_with_stops(&theme, 0.52, InteractionState::Enabled, 13);
        assert_eq!(alarm.stop_count, 13);
        assert_eq!(slider::OVERVIEW_ROWS[1].stop_count, 13);
        assert_eq!(slider::OVERVIEW_ROWS[0].label, "Call volume");

        let empty_outlined = text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Outlined,
            InteractionState::Enabled,
            false,
        );
        assert!(!empty_outlined.notched);
        assert!(!empty_outlined.floating);
        assert_eq!(empty_outlined.label_style.name, "bodyLarge");

        let group = button_group::resolve_segment(&theme, 0, 3, false, false);
        assert_eq!(group.corners.top_left, 20.0);
        assert_eq!(button_group::CONNECTED_GAP_DP, 2.0);
        let range = slider::resolve_range(&theme, 0.2, 0.75, InteractionState::Enabled);
        assert_eq!(range.start, 0.2);
        let (s, e) = slider::nudge_start(0.20, 0.75, slider::RANGE_STEP);
        assert!((s - 0.25).abs() < 1e-5);
        assert_eq!(e, 0.75);
        assert_eq!(
            dialog::resolve(&theme).headline_style.name,
            "headlineSmallEmphasized"
        );
        assert_eq!(search::resolve(&theme).bar.height_dp, 56.0);
        assert_eq!(search::resolve_view(&theme).header_h_dp, 72.0);
        assert_eq!(search::SUGGESTIONS.len(), 4);
        assert_eq!(
            time_picker::resolve(&theme).time_style.name,
            "displaySmallEmphasized"
        );
        assert_eq!(time_picker::DEMO_DIAL, time_picker::DialFace::Minute);
        let (s, e, thumb) = slider::apply_arrow(0.2, 0.75, slider::RangeThumb::Start, "right")
            .expect("arrow");
        assert!((s - 0.25).abs() < 1e-5);
        assert_eq!(e, 0.75);
        assert_eq!(thumb, slider::RangeThumb::Start);
        let frame = text_field::notch_frame("Email", &field);
        assert_eq!(frame.radius_dp, 4.0);
        assert_eq!(frame.stroke_dp, 2.0);
        assert_eq!(frame.top_lead_dp(), 4.0);
        assert_eq!(frame.notch_gap_h_dp(), 2.0);
        assert!(frame.outline_svg_d(280.0).starts_with('M'));
        assert!(frame.evenodd_svg_d(280.0).contains('Z'));
        assert_eq!(frame.evenodd_subpath_count(280.0), 1);
        assert_eq!(carousel::LARGE_W_DP, 256.0);
        let popup_opts = nav_rail_os_popup_options(navigation_rail::os_popup_spec(
            navigation_rail::RailMode::Expanded,
        ));
        assert!(matches!(popup_opts.kind, WindowKind::PopUp));
        assert!(!navigation_rail::OS_POPUP_OPENED);
        assert_eq!(progress::STROKE_CAP, progress::StrokeCap::Round);
        assert_eq!(search::resolve_activity(&theme).corners.top_left, 0.0);
        assert!((slider::fraction_from_local_x(140.0, 280.0) - 0.5).abs() < 1e-5);
    }
}
