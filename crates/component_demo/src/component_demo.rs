//! Material 3 catalog on Android. Appearances come from `gpui_material`.

#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use gpui::prelude::*;
use gpui::{
    div, px, rgb, App, Application, Context, FontWeight, IntoElement, ParentElement, Render,
    SharedString, Styled, Window,
};
use gpui_android::AndroidPlatform;
use gpui_material::components::{
    button, card, checkbox, chip, divider, fab, icon_button, list, navigation_bar, progress, radio,
    snackbar, switch, text_field, top_app_bar, Appearance,
};
use gpui_material::theme::Theme;
use gpui_material::{Argb, InteractionState};

fn paint(c: Argb) -> gpui::Rgba {
    rgb(c.rgb_u32())
}

fn type_size(style: gpui_material::typography::TypeStyle) -> gpui::Pixels {
    px(style.size_sp)
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
        .rounded(px(a.corners.top_left))
        .bg(paint(a.container))
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

fn m_block(a: &Appearance, child: impl IntoElement) -> impl IntoElement {
    div()
        .h(px(a.height_dp))
        .px(px(a.pad_start_dp))
        .py(px(a.pad_top_dp))
        .rounded(px(a.corners.top_left))
        .bg(paint(a.container))
        .text_color(paint(a.content))
        .when(a.outline.is_some(), |el| {
            let (color, _) = a.outline.unwrap();
            el.border_1().border_color(paint(color))
        })
        .child(child)
}

struct CatalogView {
    dark: bool,
    taps: usize,
    checked: bool,
    radio: usize,
    switched: bool,
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
        let snack = snackbar::resolve(&theme);
        let nav = navigation_bar::resolve(&theme);
        let lin = progress::linear(&theme, ((self.taps % 10) as f32) / 10.0);
        let one = list::resolve(&theme, list::ListLines::One, InteractionState::Enabled);
        let two = list::resolve(&theme, list::ListLines::Two, InteractionState::Enabled);
        let three = list::resolve(&theme, list::ListLines::Three, InteractionState::Enabled);
        let filled_field = text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Filled,
            InteractionState::Enabled,
            true,
        );
        let outlined_error = text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Outlined,
            InteractionState::Error,
            true,
        );
        let check = checkbox::resolve(
            &theme,
            if self.checked {
                checkbox::CheckValue::Checked
            } else {
                checkbox::CheckValue::Unchecked
            },
            InteractionState::Enabled,
        );
        let sw = switch::resolve(&theme, self.switched, InteractionState::Enabled);
        let fab_a = fab::resolve(&theme, fab::FabVariant::Primary, InteractionState::Enabled);
        let icon = icon_button::resolve(
            &theme,
            icon_button::IconButtonVariant::Standard,
            InteractionState::Enabled,
        );
        let chip_a = chip::resolve(
            &theme,
            chip::ChipVariant::Filter,
            true,
            InteractionState::Enabled,
        );
        let card_e = card::resolve(&theme, card::CardVariant::Elevated, InteractionState::Enabled);
        let card_o = card::resolve(&theme, card::CardVariant::Outlined, InteractionState::Enabled);
        let div_a = divider::resolve(&theme, false);

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(paint(c.background))
            .font_family("Roboto")
            .text_color(paint(c.on_background))
            .child(
                div()
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
                            .child("Material 3"),
                    )
                    .child(
                        div()
                            .id("theme-toggle")
                            .px(px(12.))
                            .py(px(8.))
                            .rounded(px(20.))
                            .bg(paint(c.secondary_container))
                            .text_color(paint(c.on_secondary_container))
                            .text_size(px(theme.typography.label_large.size_sp))
                            .child(if self.dark { "Dark" } else { "Light" })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.dark = !this.dark;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .id("catalog-scroll")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .p(px(16.))
                    .gap(px(16.))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(theme.typography.body_medium.size_sp))
                            .text_color(paint(c.on_surface_variant))
                            .child("Canonical Material You states. Tokens: androidx v0_210."),
                    )
                    .child(section_title(&theme, "Buttons"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.))
                            .child(m_button(
                                "btn-filled",
                                &theme,
                                button::ButtonVariant::Filled,
                                InteractionState::Enabled,
                                format!("Filled · {}", self.taps),
                                cx.listener(|this, _, _, cx| {
                                    this.taps += 1;
                                    log::info!("count incremented to {}", this.taps);
                                    cx.notify();
                                }),
                            ))
                            .child(m_button(
                                "btn-tonal",
                                &theme,
                                button::ButtonVariant::Tonal,
                                InteractionState::Enabled,
                                "Tonal",
                                |_, _, _| {},
                            ))
                            .child(m_button(
                                "btn-elevated",
                                &theme,
                                button::ButtonVariant::Elevated,
                                InteractionState::Enabled,
                                "Elevated",
                                |_, _, _| {},
                            ))
                            .child(m_button(
                                "btn-outlined",
                                &theme,
                                button::ButtonVariant::Outlined,
                                InteractionState::Enabled,
                                "Outlined",
                                |_, _, _| {},
                            ))
                            .child(m_button(
                                "btn-text",
                                &theme,
                                button::ButtonVariant::Text,
                                InteractionState::Enabled,
                                "Text",
                                |_, _, _| {},
                            ))
                            .child(m_button(
                                "btn-disabled",
                                &theme,
                                button::ButtonVariant::Filled,
                                InteractionState::Disabled,
                                "Disabled",
                                |_, _, _| {},
                            )),
                    )
                    .child(section_title(&theme, "Text fields"))
                    .child(field_block(&filled_field, "Label", "gpui-component → Material"))
                    .child(field_block(&outlined_error, "Email", "not-an-email"))
                    .child(section_title(&theme, "Selection"))
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
                                    .rounded(px(check.state_layer_dp / 2.0))
                                    .bg(paint(check.state_layer))
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
                                            .child(if self.checked { "✓" } else { "" }),
                                    )
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.checked = !this.checked;
                                        cx.notify();
                                    })),
                            )
                            .children((0..2).map(|i| {
                                let selected = self.radio == i;
                                let r = radio::resolve(&theme, selected, InteractionState::Enabled);
                                div()
                                    .id(SharedString::from(format!("radio-{i}")))
                                    .w(px(r.target_dp))
                                    .h(px(r.target_dp))
                                    .flex()
                                    .items_center()
                                    .justify_center()
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
                                            .when(selected, |el| {
                                                el.child(
                                                    div()
                                                        .w(px(r.inner_dp))
                                                        .h(px(r.inner_dp))
                                                        .rounded(px(r.inner_dp / 2.0))
                                                        .bg(paint(r.inner.unwrap_or(r.ring))),
                                                )
                                            }),
                                    )
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.radio = i;
                                        cx.notify();
                                    }))
                            }))
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
                                    .when(self.switched, |el| el.justify_end())
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
                    .child(section_title(&theme, "Lists"))
                    .child(list_row(&one, "One-line item", None))
                    .child(
                        div()
                            .h(px(div_a.thickness_dp))
                            .w_full()
                            .bg(paint(div_a.color)),
                    )
                    .child(list_row(&two, "Two-line item", Some("Supporting text")))
                    .child(list_row(
                        &three,
                        "Three-line item",
                        Some("Supporting text that can wrap onto a second line"),
                    ))
                    .child(section_title(&theme, "Chips · cards · FAB"))
                    .child(
                        div()
                            .flex()
                            .gap(px(8.))
                            .child(
                                div()
                                    .h(px(chip_a.height_dp))
                                    .px(px(chip_a.pad_start_dp))
                                    .rounded(px(chip_a.corners.top_left))
                                    .bg(paint(chip_a.container))
                                    .text_color(paint(chip_a.content))
                                    .flex()
                                    .items_center()
                                    .child("Filter"),
                            )
                            .child(
                                div()
                                    .w(px(icon.height_dp))
                                    .h(px(icon.height_dp))
                                    .rounded(px(icon.corners.top_left))
                                    .bg(paint(icon.container))
                                    .text_color(paint(icon.content))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child("★"),
                            )
                            .child(
                                div()
                                    .w(px(fab_a.height_dp))
                                    .h(px(fab_a.height_dp))
                                    .rounded(px(fab_a.corners.top_left))
                                    .bg(paint(fab_a.container))
                                    .text_color(paint(fab_a.content))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child("+"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.))
                            .child(m_block(&card_e, "Elevated card"))
                            .child(m_block(&card_o, "Outlined card")),
                    )
                    .child(section_title(&theme, "Progress"))
                    .child(
                        div()
                            .w_full()
                            .h(px(lin.height_dp))
                            .rounded(px(2.))
                            .bg(paint(lin.track))
                            .child(
                                div()
                                    .h_full()
                                    .w(px(120. * lin.progress.max(0.08)))
                                    .bg(paint(lin.indicator)),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h(px(snack.min_height_dp))
                            .px(px(16.))
                            .rounded(px(snack.corners.top_left))
                            .bg(paint(snack.container))
                            .text_color(paint(snack.supporting))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child("Message sent")
                            .child(
                                div()
                                    .text_color(paint(snack.action))
                                    .child("Action"),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(paint(c.on_surface_variant))
                            .child("Dialog, sheets, menus, sliders, tabs: not started."),
                    ),
            )
            .child(
                div()
                    .h(px(nav.height_dp))
                    .w_full()
                    .bg(paint(nav.container))
                    .flex()
                    .items_center()
                    .justify_around()
                    .child(nav_dest(&nav, true, "Home"))
                    .child(nav_dest(&nav, false, "Search"))
                    .child(nav_dest(&nav, false, "Profile")),
            )
    }
}

fn section_title(theme: &Theme, title: &'static str) -> impl IntoElement {
    div()
        .mt(px(8.))
        .text_size(px(theme.typography.title_medium.size_sp))
        .font_weight(FontWeight::MEDIUM)
        .text_color(paint(theme.color.on_surface))
        .child(title)
}

fn field_block(
    field: &text_field::TextFieldAppearance,
    label: &'static str,
    value: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(
            div()
                .h(px(field.field.height_dp))
                .px(px(16.))
                .rounded(px(field.field.corners.top_left))
                .bg(paint(field.field.container))
                .border_1()
                .border_color(paint(field.field.outline.unwrap().0))
                .flex()
                .flex_col()
                .justify_center()
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
                ),
        )
        .child(
            div()
                .px(px(16.))
                .text_size(px(field.supporting_style.size_sp))
                .text_color(paint(field.supporting))
                .child(if field.field.outline.unwrap().0 == field.supporting {
                    "Enter a valid value"
                } else {
                    "Supporting text"
                }),
        )
}

fn list_row(a: &Appearance, title: &'static str, support: Option<&'static str>) -> impl IntoElement {
    div()
        .h(px(a.height_dp))
        .px(px(a.pad_start_dp))
        .w_full()
        .bg(paint(a.container))
        .flex()
        .flex_col()
        .justify_center()
        .child(
            div()
                .text_size(px(a.label_style.size_sp))
                .text_color(paint(a.content))
                .child(title),
        )
        .when_some(support, |el, text| {
            el.child(
                div()
                    .text_size(px(a.supporting_style.unwrap().size_sp))
                    .text_color(paint(a.secondary_content.unwrap()))
                    .child(text),
            )
        })
}

fn nav_dest(
    nav: &navigation_bar::NavBarAppearance,
    active: bool,
    label: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(4.))
        .child(
            div()
                .w(px(navigation_bar::INDICATOR_W_DP))
                .h(px(navigation_bar::INDICATOR_H_DP))
                .rounded(px(16.))
                .when(active, |el| el.bg(paint(nav.active_indicator)))
                .flex()
                .items_center()
                .justify_center()
                .text_color(paint(if active {
                    nav.active_icon
                } else {
                    nav.inactive_icon
                }))
                .child(if active { "●" } else { "○" }),
        )
        .child(
            div()
                .text_size(px(nav.label_style.size_sp))
                .text_color(paint(if active {
                    nav.active_label
                } else {
                    nav.inactive_label
                }))
                .child(label),
        )
}

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("component_demo"),
    );
    std::panic::set_hook(Box::new(|info| {
        log::error!("PANIC: {info}");
    }));
    log::info!("android_main starting");

    let platform = AndroidPlatform::new(app);
    Application::with_platform(platform).run(|cx: &mut App| {
        cx.open_window(gpui::WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| CatalogView {
                dark: false,
                taps: 0,
                checked: true,
                radio: 0,
                switched: true,
            })
        })
        .expect("failed to open window");
    });
}
