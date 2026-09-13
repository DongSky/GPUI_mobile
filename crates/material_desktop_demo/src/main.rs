//! Desktop Material 3 / Expressive catalog.
//!
//! Paints `gpui_material::resolve()` appearances with the same GPUI `div`
//! mapping as Android `component_demo`. Tokens live only in `gpui_material`.
//!
//! Boot pattern matches upstream Zed GPUI examples (`gpui_platform::application`).

#![cfg(not(target_os = "android"))]

use gpui::prelude::*;
use gpui::{
    div, px, size, App, Bounds, Context, FontWeight, IntoElement, ParentElement, Render,
    SharedString, Styled, TitlebarOptions, Window, WindowBounds, WindowOptions,
};
use gpui_material::components::date_picker::{self, CivilDate, DayKind};
use gpui_material::components::text_field::TextFieldEditor;
use gpui_material::components::{
    button, checkbox, dialog, icon_button, radio, slider, switch, tabs, text_field, top_app_bar,
};
use gpui_material::theme::Theme;
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
                    .child("Material 3 desktop"),
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
            .font_family("Noto Sans")
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
                .child(
                    "Desktop catalog · same gpui_material::resolve() as Android + HTML.",
                ),
        )
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
        .child(
            div()
                .text_size(px(12.))
                .text_color(paint(c.on_surface_variant))
                .child(format!(
                    "Media {:.0}% · Expressive XS 16dp track / 4×44 handle · Alarm mid-stops",
                    this.slider * 100.0
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
        .child(section_title(theme, "Date picker"))
        .child(date_picker_card(this, theme, &pick, &cells, cx))
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
    let (active_n, inactive_n) = slider::segmented_stop_counts(slide.value, slide.stop_count);
    div()
        .w_full()
        .h(px(slide.handle_h))
        .flex()
        .items_center()
        .child(
            div()
                .h(px(slide.track_h))
                .w(px(180. * slide.value.max(0.12)))
                .rounded(px(slide.track_corner))
                .bg(paint(slide.active))
                .flex()
                .items_center()
                .justify_between()
                .px(px(8.))
                .children((0..active_n).map(|_| slider_stop(slide, true))),
        )
        .child(
            div()
                .mx(px(slide.gap_dp))
                .w(px(slide.handle_w))
                .h(px(slide.handle_h))
                .rounded(px(2.))
                .bg(paint(slide.handle)),
        )
        .child(
            div()
                .h(px(slide.track_h))
                .flex_1()
                .rounded(px(slide.track_corner))
                .bg(paint(slide.inactive))
                .flex()
                .items_center()
                .justify_between()
                .px(px(8.))
                .children((0..inactive_n).map(|_| slider_stop(slide, false))),
        )
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
        .gap(px(8.))
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
                .child(expressive_slider(&slide))
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
                        .text_size(type_size(a.headline_style))
                        .text_color(paint(a.headline))
                        .child(dialog::RESET_HEADLINE),
                )
                .child(
                    div()
                        .w_full()
                        .text_size(type_size(a.supporting_style))
                        .text_color(paint(a.supporting))
                        .child(dialog::RESET_SUPPORTING),
                )
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
        .child(
            div()
                .text_size(px(pick.year_style.size_sp))
                .text_color(paint(pick.header_year))
                .child("Select date"),
        )
        .child(
            div()
                .text_size(px(pick.date_style.size_sp))
                .text_color(paint(pick.header_date))
                .child(date_picker::header_date_label(this.selected)),
        )
        .child(
            div()
                .id("year-control")
                .text_size(px(pick.year_style.size_sp))
                .text_color(paint(pick.header_year))
                .child(date_picker::header_year_label(this.picker_year))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.picker_year += 1;
                    cx.notify();
                })),
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
                .child(date_picker::month_title(this.picker_year, this.picker_month))
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
        .child(
            div()
                .w(px(cal_w))
                .flex()
                .flex_wrap()
                .children(date_picker::WEEKDAYS.iter().map(|d| {
                    div()
                        .w(px(pick.day_dp))
                        .h(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(paint(pick.weekday))
                        .child(*d)
                })),
        )
        .child(div().w(px(cal_w)).flex().flex_wrap().children(
            cells.iter().copied().enumerate().map(|(i, (day, kind))| {
                let (bg, fg) = match kind {
                    DayKind::Selected => {
                        (paint(pick.day_selected_container), paint(pick.day_selected))
                    }
                    DayKind::Today => (paint(pick.container), paint(pick.day)),
                    DayKind::InMonth => (paint(pick.container), paint(pick.day)),
                    DayKind::OutOfMonth => (paint(pick.container), paint(pick.day_out)),
                };
                let in_month = kind != DayKind::OutOfMonth;
                let year = this.picker_year;
                let month = this.picker_month;
                div()
                    .id(SharedString::from(format!("day-{i}")))
                    .w(px(pick.day_dp))
                    .h(px(pick.day_dp))
                    .rounded(px(pick.day_dp / 2.0))
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
        .child(label.into())
        .when(!disabled, |el| el.on_click(on_click))
}

fn section_title(theme: &Theme, title: &'static str) -> impl IntoElement {
    div()
        .mt(px(8.))
        .text_size(px(theme.typography.title_medium.size_sp))
        .font_weight(FontWeight::MEDIUM)
        .text_color(paint(theme.color.on_surface))
        .whitespace_nowrap()
        .child(title)
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
        let label_h = field.label_style.line_height_sp;
        div()
            .id(id)
            .flex()
            .flex_col()
            .on_click(on_click)
            .child(
                div().h(px(label_h)).pl(px(12.)).child(
                    div()
                        .px(px(text_field::NOTCH_PAD_DP))
                        .bg(paint(field.field.container))
                        .text_size(px(field.label_style.size_sp))
                        .text_color(paint(field.label))
                        .child(label),
                ),
            )
            .child(
                div()
                    .mt(px(-label_h * 0.5))
                    .h(px(field.field.height_dp))
                    .px(px(16.))
                    .pt(px(8.))
                    .rounded(px(field.field.corners.top_left))
                    .bg(paint(field.field.container))
                    .when(outline.1 >= 2.0, |el| {
                        el.border_2().border_color(paint(outline.0))
                    })
                    .when(outline.1 < 2.0, |el| {
                        el.border_1().border_color(paint(outline.0))
                    })
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(field.input_style.size_sp))
                            .text_color(paint(field.input))
                            .child(value),
                    ),
            )
            .into_any_element()
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
                })
            },
        )
        .expect("failed to open Material desktop catalog window");
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use gpui_material::components::{button, slider, text_field};
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
        assert_eq!(s.inactive, theme.color.surface_container_highest);
        assert_eq!(s.stop_count, 2);
        let alarm = slider::resolve_with_stops(&theme, 0.52, InteractionState::Enabled, 11);
        assert_eq!(alarm.stop_count, 11);

        let empty_outlined = text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Outlined,
            InteractionState::Enabled,
            false,
        );
        assert!(!empty_outlined.notched);
        assert!(!empty_outlined.floating);
        assert_eq!(empty_outlined.label_style.name, "bodyLarge");
    }
}
