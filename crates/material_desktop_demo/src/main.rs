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
    div, px, size, App, Bounds, Context, FontWeight, IntoElement, KeyDownEvent, MouseButton,
    MouseMoveEvent, ParentElement, Render, SharedString, Styled, TitlebarOptions, Window,
    WindowBounds, WindowOptions,
};
use gpui_material::components::date_picker::{self, CivilDate, DayKind};
use gpui_material::components::text_field::TextFieldEditor;
use gpui_material::components::time_picker::{self, DayPeriod, DialFace};
use gpui_material::components::{
    button, button_group, checkbox, dialog, icon_button, navigation_rail, progress, radio, search,
    slider, switch, tabs, text_field, top_app_bar,
};
use gpui_material::theme::Theme;
use gpui_material::typography;
use gpui_material::{Argb, InteractionState};
use gpui_platform::application;

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
    docked_open: bool,
    search_open: bool,
    time_hour: u8,
    time_minute: u8,
    time_period: DayPeriod,
    time_dial: DialFace,
}

impl CatalogView {
    fn theme(&self) -> Theme {
        if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }
}

impl Render for CatalogView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
        .child(nav_rail_hero(theme))
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
    let total = 280.0;
    let left = (total * range.start).max(12.0);
    let mid = (total * (range.end - range.start)).max(16.0);
    let cells = (0..slider::RANGE_DRAG_CELLS).map(|i| {
        let frac = slider::drag_cell_fraction(i);
        div()
            .id(SharedString::from(format!("range-cell-{i}")))
            .flex_1()
            .h_full()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
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
                        let (s, e) =
                            slider::drag_thumb(this.range_start, this.range_end, thumb, frac);
                        this.range_start = s;
                        this.range_end = e;
                        this.range_moved = true;
                        cx.notify();
                    }
                }
            }))
            .on_click(cx.listener(move |this, _, _, cx| {
                if this.range_moved {
                    this.range_drag = None;
                    this.range_moved = false;
                    cx.notify();
                    return;
                }
                let (s, e) = slider::click_step(this.range_start, this.range_end, frac);
                this.range_start = s;
                this.range_end = e;
                this.range_drag = None;
                cx.notify();
            }))
    });
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
                .relative()
                .w(px(total))
                .h(px(t.target_dp))
                .child(
                    div()
                        .w(px(total))
                        .h(px(t.target_dp))
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .h(px(t.track_h))
                                .w(px(left))
                                .rounded(px(t.track_corner))
                                .bg(paint(t.inactive)),
                        )
                        .child(
                            div()
                                .mx(px(t.gap_dp))
                                .w(px(t.handle_w.max(12.0)))
                                .h(px(t.handle_h_visual))
                                .rounded(px(2.))
                                .bg(paint(t.handle)),
                        )
                        .child(
                            div()
                                .h(px(t.track_h))
                                .w(px(mid))
                                .rounded(px(t.inner_corner))
                                .bg(paint(t.active)),
                        )
                        .child(
                            div()
                                .mx(px(t.gap_dp))
                                .w(px(t.handle_w.max(12.0)))
                                .h(px(t.handle_h_visual))
                                .rounded(px(2.))
                                .bg(paint(t.handle)),
                        )
                        .child(
                            div()
                                .h(px(t.track_h))
                                .flex_1()
                                .rounded(px(t.track_corner))
                                .bg(paint(t.inactive)),
                        ),
                )
                .child(
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(total))
                        .h(px(t.target_dp))
                        .flex()
                        .children(cells),
                ),
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
    let a = search::resolve(theme);
    let view = search::resolve_view(theme);
    let bar = div()
        .id("search-bar")
        .w_full()
        .h(px(a.bar.height_dp))
        .px(px(a.bar.pad_start_dp))
        .rounded(px(a.bar.corners.top_left))
        .bg(paint(a.bar.container))
        .flex()
        .items_center()
        .gap(px(search::GAP_DP))
        .child(
            div()
                .w(px(search::ICON_DP))
                .h(px(search::ICON_DP))
                .flex()
                .items_center()
                .justify_center()
                .text_color(paint(a.leading_icon))
                .child(search::LEADING_ICON),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(a.placeholder_style.size_sp))
                .text_color(paint(a.placeholder))
                .child(search::PLACEHOLDER),
        )
        .child(
            div()
                .w(px(search::ICON_DP))
                .text_color(paint(a.trailing_icon))
                .child(search::TRAILING_MIC),
        )
        .child(
            div()
                .w(px(search::AVATAR_DP))
                .h(px(search::AVATAR_DP))
                .rounded(px(search::AVATAR_DP / 2.0))
                .bg(paint(a.avatar))
                .text_color(paint(a.avatar_label))
                .flex()
                .items_center()
                .justify_center()
                .child("A"),
        )
        .on_click(cx.listener(|this, _, _, cx| {
            this.search_open = !this.search_open;
            cx.notify();
        }));
    let sheet = this.search_open.then(|| {
        div()
            .w_full()
            .mt(px(8.))
            .rounded(px(view.corners.top_left))
            .bg(paint(view.container))
            .shadow_md()
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(view.header_h_dp))
                    .px(px(16.))
                    .flex()
                    .items_center()
                    .gap(px(16.))
                    .child(
                        div()
                            .id("search-back")
                            .text_color(paint(view.header))
                            .child(search::VIEW_BACK)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.search_open = false;
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_color(paint(view.placeholder))
                            .child(search::PLACEHOLDER),
                    )
                    .child(div().text_color(paint(view.header)).child(search::TRAILING_MIC)),
            )
            .child(div().h(px(1.)).w_full().bg(paint(view.divider)))
            .children(search::SUGGESTIONS.iter().enumerate().map(|(i, label)| {
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
                    .child(*label)
            }))
            .into_any_element()
    });
    div()
        .w_full()
        .flex()
        .flex_col()
        .child(bar)
        .children(sheet)
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
    let dots = time_picker::hand_dots(clock, this.time_dial, this.time_hour, this.time_minute, number);
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
                        .left(px(clock / 2.0 - time_picker::HAND_HUB_DP / 2.0))
                        .top(px(clock / 2.0 - time_picker::HAND_HUB_DP / 2.0))
                        .w(px(time_picker::HAND_HUB_DP))
                        .h(px(time_picker::HAND_HUB_DP))
                        .rounded(px(time_picker::HAND_HUB_DP / 2.0))
                        .bg(paint(a.hand)),
                )
                .children(dots.into_iter().enumerate().map(|(i, (x, y))| {
                    div()
                        .id(SharedString::from(format!("hand-dot-{i}")))
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(time_picker::HAND_THICKNESS_DP * 2.0))
                        .h(px(time_picker::HAND_THICKNESS_DP * 2.0))
                        .rounded(px(time_picker::HAND_THICKNESS_DP))
                        .bg(paint(a.hand))
                }))
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

fn nav_rail_hero(theme: &Theme) -> impl IntoElement {
    let rail = navigation_rail::resolve(theme);
    div()
        .w(px(rail.width_dp))
        .pt(px(navigation_rail::PAD_TOP_DP))
        .bg(paint(rail.container))
        .flex()
        .flex_col()
        .items_center()
        .gap(px(navigation_rail::DEST_GAP_DP))
        .children(
            navigation_rail::DESTINATIONS
                .iter()
                .zip(navigation_rail::DESTINATION_ICONS.iter())
                .enumerate()
                .map(|(i, (label, icon))| {
                    let active = i == 0;
                    div()
                        .w(px(rail.width_dp))
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(4.))
                        .child(
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
                        )
                        .child(
                            div()
                                .text_size(px(rail.label_style.size_sp))
                                .text_color(paint(if active {
                                    rail.active_label
                                } else {
                                    rail.inactive_label
                                }))
                                .child(*label),
                        )
                }),
        )
}

fn progress_heroes(theme: &Theme) -> impl IntoElement {
    let lin = progress::linear(theme, 0.6);
    let indet = progress::linear_indeterminate(theme);
    let ptr = progress::pull_to_refresh(theme);
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
                .relative()
                .w(px(240.))
                .h(px(indet.height_dp))
                .rounded(px(2.))
                .bg(paint(indet.track))
                .child(
                    div()
                        .absolute()
                        .left(px(240. * 0.3))
                        .h_full()
                        .w(px(240. * indet.head_span))
                        .bg(paint(indet.indicator)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(8.))
                .child(
                    div()
                        .w(px(ptr.size_dp))
                        .h(px(ptr.size_dp))
                        .rounded(px(ptr.size_dp / 2.0))
                        .border_2()
                        .border_color(paint(ptr.track))
                        .flex()
                        .items_start()
                        .justify_center()
                        .child(
                            div()
                                .w(px(ptr.stroke_dp))
                                .h(px(ptr.size_dp * 0.28))
                                .rounded(px(2.))
                                .bg(paint(ptr.indicator)),
                        ),
                )
                .child(spaced_line(
                    progress::PTR_LABEL,
                    12.0,
                    paint(theme.color.on_surface_variant),
                )),
        )
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

fn notch_corner(
    tl: bool,
    tr: bool,
    br: bool,
    bl: bool,
    frame: text_field::NotchFrame,
    outline: gpui_material::Argb,
    fill: gpui_material::Argb,
) -> impl IntoElement {
    let r = frame.radius_dp;
    let s = frame.stroke_dp;
    let inner = frame.inner_radius_dp();
    let inner_sz = (r - s).max(0.0);
    let mut tile = div().w(px(r)).h(px(r)).bg(paint(outline));
    if tl {
        tile = tile.rounded_tl(px(r)).pt(px(s)).pl(px(s));
    } else if tr {
        tile = tile.rounded_tr(px(r)).pt(px(s)).pr(px(s));
    } else if bl {
        tile = tile.rounded_bl(px(r)).pb(px(s)).pl(px(s));
    } else if br {
        tile = tile.rounded_br(px(r)).pb(px(s)).pr(px(s));
    }
    let mut hole = div().w(px(inner_sz)).h(px(inner_sz)).bg(paint(fill));
    if tl {
        hole = hole.rounded_tl(px(inner));
    } else if tr {
        hole = hole.rounded_tr(px(inner));
    } else if bl {
        hole = hole.rounded_bl(px(inner));
    } else if br {
        hole = hole.rounded_br(px(inner));
    }
    tile.child(hole)
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
    let stroke = frame.stroke_dp;
    let radius = frame.radius_dp;
    let fill = field.field.container;
    let middle_h = frame.middle_h_dp();
    let gap_h = frame.notch_gap_h_dp();
    let (lx, ly) = frame.label_origin_dp();
    div()
        .id(id)
        .relative()
        .flex()
        .flex_col()
        .w_full()
        .on_click(on_click)
        .child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .h(px(radius))
                .child(notch_corner(true, false, false, false, frame, outline.0, fill))
                .child(
                    div()
                        .w(px(frame.top_lead_dp()))
                        .h(px(radius))
                        .flex()
                        .flex_col()
                        .child(div().w_full().h(px(stroke)).bg(paint(outline.0))),
                )
                .child(
                    div()
                        .w(px(frame.width_dp))
                        .h(px(gap_h))
                        .bg(paint(fill)),
                )
                .child(
                    div()
                        .flex_1()
                        .h(px(radius))
                        .flex()
                        .flex_col()
                        .child(div().w_full().h(px(stroke)).bg(paint(outline.0))),
                )
                .child(notch_corner(false, true, false, false, frame, outline.0, fill)),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .h(px(middle_h))
                .child(div().w(px(stroke)).h(px(middle_h)).bg(paint(outline.0)))
                .child(
                    div()
                        .flex_1()
                        .h(px(middle_h))
                        .px(px(16.))
                        .flex()
                        .items_center()
                        .bg(paint(fill))
                        .child(
                            div()
                                .text_size(px(field.input_style.size_sp))
                                .text_color(paint(field.input))
                                .child(value),
                        ),
                )
                .child(div().w(px(stroke)).h(px(middle_h)).bg(paint(outline.0))),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_end()
                .h(px(radius))
                .child(notch_corner(false, false, false, true, frame, outline.0, fill))
                .child(
                    div()
                        .flex_1()
                        .h(px(stroke))
                        .bg(paint(outline.0)),
                )
                .child(notch_corner(false, false, true, false, frame, outline.0, fill)),
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
                .bg(paint(fill))
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
                    docked_open: date_picker::DOCKED_OPEN_BY_DEFAULT,
                    search_open: search::VIEW_OPEN_BY_DEFAULT,
                    time_hour: time_picker::DEMO_HOUR,
                    time_minute: time_picker::DEMO_MINUTE,
                    time_period: time_picker::DEMO_PERIOD,
                    time_dial: time_picker::DEMO_DIAL,
                })
            },
        )
        .expect("failed to open Material desktop catalog window");
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use gpui_material::components::{
        button, button_group, dialog, search, slider, text_field, time_picker,
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
    }
}
