//! Material 3 catalog on Android. Appearances come from `gpui_material`.

#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use gpui::prelude::*;
use gpui::{
    div, px, rgb, App, Application, Context, FontWeight, IntoElement, ParentElement, Render,
    SharedString, Styled, Window,
};
use gpui_android::AndroidPlatform;
use gpui_material::components::date_picker::{self, CivilDate, DayKind};
use gpui_material::components::text_field::TextFieldEditor;
use gpui_material::components::{
    badge, bottom_sheet, button, card, checkbox, chip, dialog, divider, fab, icon_button, list,
    menu, navigation_bar, progress, radio, slider, snackbar, switch, tabs, text_field, top_app_bar,
    Appearance,
};
use gpui_material::theme::Theme;
use gpui_material::{Argb, InteractionState};

fn paint(c: Argb) -> gpui::Rgba {
    rgb(c.rgb_u32())
}

fn type_size(style: gpui_material::typography::TypeStyle) -> gpui::Pixels {
    px(style.size_sp)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Overlay {
    None,
    Dialog,
    Sheet,
    Menu,
}

struct CatalogView {
    dark: bool,
    taps: usize,
    checked: bool,
    radio: usize,
    switched: bool,
    overlay: Overlay,
    menu_selected: usize,
    slider: f32,
    tab_primary: usize,
    tab_secondary: usize,
    badge_count: u32,
    picker_year: i32,
    picker_month: u32,
    selected: CivilDate,
    today: CivilDate,
    filled: TextFieldEditor,
    outlined: TextFieldEditor,
    nav: usize,
}

impl CatalogView {
    fn theme(&self) -> Theme {
        if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }

    fn blur_fields(&mut self) {
        self.filled.set_focus(false);
        self.outlined.set_focus(false);
    }

    fn apply_key(&mut self, key: &str) {
        let ed = if self.filled.focused {
            &mut self.filled
        } else if self.outlined.focused {
            &mut self.outlined
        } else {
            return;
        };
        if key == "⌫" {
            ed.backspace();
        } else if key == " " {
            ed.insert_char(' ');
        } else if let Some(ch) = key.chars().next() {
            ed.insert_char(ch);
        }
        if self.outlined.focused {
            let v = self.outlined.value().to_string();
            self.outlined.error = !v.is_empty() && !text_field::looks_like_email(&v);
        }
    }

    fn field_focused(&self) -> bool {
        self.filled.focused || self.outlined.focused
    }
}

impl Render for CatalogView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme();
        let c = theme.color;
        let bar = top_app_bar::resolve(&theme);
        let nav = navigation_bar::resolve(&theme);

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
            );

        let body = match self.overlay {
            Overlay::None => catalog_body(self, &theme, cx).into_any_element(),
            Overlay::Dialog => dialog_overlay(&theme, cx).into_any_element(),
            Overlay::Sheet => sheet_overlay(&theme, cx).into_any_element(),
            Overlay::Menu => menu_overlay(self, &theme, cx).into_any_element(),
        };

        let keyboard = self.field_focused().then(|| onscreen_keys(&theme, cx));

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(paint(c.background))
            .font_family("Roboto")
            .text_color(paint(c.on_background))
            .child(chrome)
            .child(body)
            .children(keyboard)
            .child(
                div()
                    .h(px(nav.height_dp))
                    .w_full()
                    .bg(paint(nav.container))
                    .flex()
                    .items_center()
                    .justify_around()
                    .children(["Home", "Search", "Profile"].into_iter().enumerate().map(
                        |(i, label)| {
                            let active = self.nav == i;
                            nav_dest(
                                &nav,
                                active,
                                label,
                                cx.listener(move |this, _, _, cx| {
                                    this.nav = i;
                                    cx.notify();
                                }),
                            )
                        },
                    )),
            )
    }
}

fn catalog_body(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let c = theme.color;
    let snack = snackbar::resolve(theme);
    let lin = progress::linear(theme, ((this.taps % 10) as f32) / 10.0);
    let one = list::resolve(theme, list::ListLines::One, InteractionState::Enabled);
    let two = list::resolve(theme, list::ListLines::Two, InteractionState::Enabled);
    let three = list::resolve(theme, list::ListLines::Three, InteractionState::Enabled);
    let filled_a = this.filled.appearance(theme);
    let outlined_a = this.outlined.appearance(theme);
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
    let fab_reg = fab::resolve(theme, fab::FabVariant::Primary, InteractionState::Enabled);
    let fab_medium = fab::resolve_size(
        theme,
        fab::FabVariant::Primary,
        fab::FabSize::Medium,
        InteractionState::Enabled,
    );
    let fab_large = fab::resolve_size(
        theme,
        fab::FabVariant::Primary,
        fab::FabSize::Large,
        InteractionState::Enabled,
    );
    let fab_ext = fab::resolve_size(
        theme,
        fab::FabVariant::Primary,
        fab::FabSize::Extended,
        InteractionState::Enabled,
    );
    let icon = icon_button::resolve(
        theme,
        icon_button::IconButtonVariant::Standard,
        InteractionState::Enabled,
    );
    let chip_a = chip::resolve(
        theme,
        chip::ChipVariant::Filter,
        true,
        InteractionState::Enabled,
    );
    let card_e = card::resolve(
        theme,
        card::CardVariant::Elevated,
        InteractionState::Enabled,
    );
    let card_o = card::resolve(
        theme,
        card::CardVariant::Outlined,
        InteractionState::Enabled,
    );
    let div_a = divider::resolve(theme, false);
    let slide = slider::resolve(theme, this.slider, InteractionState::Enabled);
    let tabs_p = tabs::resolve(theme, tabs::TabsVariant::Primary);
    let tabs_s = tabs::resolve(theme, tabs::TabsVariant::Secondary);
    let badge_s = badge::resolve(theme, badge::BadgeKind::Small);
    let badge_l = badge::resolve(theme, badge::BadgeKind::Large);
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
        .p(px(16.))
        .gap(px(16.))
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(px(theme.typography.body_medium.size_sp))
                .text_color(paint(c.on_surface_variant))
                .child("Material You baseline. Tokens: androidx v0_210."),
        )
        .child(section_title(theme, "Buttons"))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .child(m_button(
                    "btn-filled",
                    theme,
                    button::ButtonVariant::Filled,
                    InteractionState::Enabled,
                    format!("Filled · {}", this.taps),
                    cx.listener(|this, _, _, cx| {
                        this.taps += 1;
                        log::info!("count incremented to {}", this.taps);
                        cx.notify();
                    }),
                ))
                .child(m_button(
                    "btn-tonal",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Tonal",
                    |_, _, _| {},
                ))
                .child(m_button(
                    "btn-elevated",
                    theme,
                    button::ButtonVariant::Elevated,
                    InteractionState::Enabled,
                    "Elevated",
                    |_, _, _| {},
                ))
                .child(m_button(
                    "btn-outlined",
                    theme,
                    button::ButtonVariant::Outlined,
                    InteractionState::Enabled,
                    "Outlined",
                    |_, _, _| {},
                ))
                .child(m_button(
                    "btn-text",
                    theme,
                    button::ButtonVariant::Text,
                    InteractionState::Enabled,
                    "Text",
                    |_, _, _| {},
                ))
                .child(m_button(
                    "btn-disabled",
                    theme,
                    button::ButtonVariant::Filled,
                    InteractionState::Disabled,
                    "Disabled",
                    |_, _, _| {},
                )),
        )
        .child(section_title(theme, "Text fields"))
        .child(
            div()
                .text_size(px(12.))
                .text_color(paint(c.on_surface_variant))
                .child("Tap a field, then use the on-screen keys. System IME is a NativeActivity stub."),
        )
        .child(field_block(
            "field-filled",
            &filled_a,
            "Label",
            this.filled.display_with_caret(),
            "Supporting text",
            cx.listener(|this, _, _, cx| {
                this.outlined.set_focus(false);
                this.filled.set_focus(true);
                cx.notify();
            }),
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
                cx.notify();
            }),
        ))
        .child(section_title(theme, "Selection"))
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
                                .child(if this.checked { "✓" } else { "" }),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.checked = !this.checked;
                            cx.notify();
                        })),
                )
                .children((0..2).map(|i| {
                    let selected = this.radio == i;
                    let r = radio::resolve(theme, selected, InteractionState::Enabled);
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
        .child(section_title(theme, "Lists"))
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
        .child(section_title(theme, "Chips · cards · FAB"))
        .child(
            div()
                .flex()
                .gap(px(8.))
                .items_center()
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
                .child(fab_box(&fab_medium, "+"))
                .child(fab_box(&fab_reg, "+"))
                .child(fab_box(&fab_large, "+"))
                .child(
                    div()
                        .h(px(fab_ext.height_dp))
                        .px(px(16.))
                        .min_w(px(fab_ext.min_width_dp.unwrap_or(80.)))
                        .rounded(px(fab_ext.corners.top_left))
                        .bg(paint(fab_ext.container))
                        .text_color(paint(fab_ext.content))
                        .flex()
                        .items_center()
                        .child("+ Create"),
                ),
        )
        .child(
            div()
                .flex()
                .gap(px(8.))
                .child(m_block(&card_e, "Elevated card"))
                .child(m_block(&card_o, "Outlined card")),
        )
        .child(section_title(theme, "Slider"))
        .child(
            div()
                .id("slider")
                .w_full()
                .h(px(slide.target_dp))
                .flex()
                .items_center()
                .child(
                    div()
                        .w_full()
                        .h(px(slide.handle_h))
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .h(px(slide.track_h))
                                .w(px(140. * slide.value.max(0.12)))
                                .rounded(px(slide.track_corner))
                                .bg(paint(slide.active)),
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
                                .bg(paint(slide.inactive)),
                        ),
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    this.slider = ((this.slider + 0.1) * 10.0).round() / 10.0;
                    if this.slider > 1.0 {
                        this.slider = 0.0;
                    }
                    cx.notify();
                })),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(paint(c.on_surface_variant))
                .child(format!(
                    "Value {:.0}% · Expressive XS 16dp / 4×44 handle",
                    this.slider * 100.0
                )),
        )
        .child(section_title(theme, "Tabs"))
        .child(tab_row(theme, &tabs_p, this.tab_primary, "p", cx))
        .child(tab_row(theme, &tabs_s, this.tab_secondary, "s", cx))
        .child(section_title(theme, "Badge"))
        .child(
            div()
                .flex()
                .gap(px(24.))
                .items_center()
                .child(badge_icon(&icon, Some((badge_s.container, None))))
                .child(
                    div()
                        .id("badge-plus")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.badge_count = this.badge_count.saturating_add(1);
                            if this.badge_count > 1200 {
                                this.badge_count = 1;
                            }
                            cx.notify();
                        }))
                        .child(badge_icon(
                            &icon,
                            Some((
                                badge_l.container,
                                Some((
                                    badge::label_for_count(this.badge_count),
                                    badge_l.label,
                                )),
                            )),
                        )),
                ),
        )
        .child(section_title(theme, "Date picker"))
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
                            let (y, m) = date_picker::add_months(this.picker_year, this.picker_month, -1);
                            this.picker_year = y;
                            this.picker_month = m;
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .text_size(px(pick.date_style.size_sp.min(22.0)))
                        .child(date_picker::month_title(this.picker_year, this.picker_month)),
                )
                .child(
                    div()
                        .id("month-next")
                        .p(px(8.))
                        .child(">")
                        .on_click(cx.listener(|this, _, _, cx| {
                            let (y, m) = date_picker::add_months(this.picker_year, this.picker_month, 1);
                            this.picker_year = y;
                            this.picker_month = m;
                            cx.notify();
                        })),
                ),
        )
        .child(
            div()
                .text_size(px(pick.year_style.size_sp))
                .text_color(paint(pick.header_year))
                .child(this.selected.year.to_string()),
        )
        .child(
            div()
                .text_size(px(22.))
                .text_color(paint(pick.header_date))
                .child(date_picker::header_date_label(this.selected)),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .children(date_picker::WEEKDAYS.iter().map(|d| {
                    div()
                        .w(px(40.))
                        .h(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(paint(pick.weekday))
                        .child(*d)
                })),
        )
        .child(
            div().flex().flex_wrap().children(cells.into_iter().enumerate().map(
                |(i, (day, kind))| {
                    let (bg, fg) = match kind {
                        DayKind::Selected => (paint(pick.day_selected_container), paint(pick.day_selected)),
                        DayKind::Today => (paint(theme.color.surface), paint(pick.day)),
                        DayKind::InMonth => (paint(theme.color.surface), paint(pick.day)),
                        DayKind::OutOfMonth => (paint(theme.color.surface), paint(pick.day_out)),
                    };
                    let in_month = kind != DayKind::OutOfMonth;
                    let year = this.picker_year;
                    let month = this.picker_month;
                    div()
                        .id(SharedString::from(format!("day-{i}")))
                        .w(px(40.))
                        .h(px(40.))
                        .rounded(px(20.))
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
                                this.selected = CivilDate {
                                    year,
                                    month,
                                    day,
                                };
                                cx.notify();
                            }))
                        })
                },
            )),
        )
        .child(section_title(theme, "Overlays"))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .child(m_button(
                    "open-dialog",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Dialog",
                    cx.listener(|this, _, _, cx| {
                        this.blur_fields();
                        this.overlay = Overlay::Dialog;
                        cx.notify();
                    }),
                ))
                .child(m_button(
                    "open-sheet",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Sheet",
                    cx.listener(|this, _, _, cx| {
                        this.blur_fields();
                        this.overlay = Overlay::Sheet;
                        cx.notify();
                    }),
                ))
                .child(m_button(
                    "open-menu",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Menu",
                    cx.listener(|this, _, _, cx| {
                        this.blur_fields();
                        this.overlay = Overlay::Menu;
                        cx.notify();
                    }),
                )),
        )
        .child(section_title(theme, "Progress"))
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
                .child(div().text_color(paint(snack.action)).child("Action")),
        )
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
        .on_click(cx.listener(|this, _, _, cx| {
            this.overlay = Overlay::None;
            cx.notify();
        }))
        .child(
            div()
                .id("dialog-card")
                .min_w(px(a.min_width_dp))
                .p(px(a.pad_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(a.container))
                .gap(px(16.))
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(type_size(a.headline_style))
                        .text_color(paint(a.headline))
                        .child("Reset settings?"),
                )
                .child(
                    div()
                        .text_size(type_size(a.supporting_style))
                        .text_color(paint(a.supporting))
                        .child("This will restore defaults. You can change them again later."),
                )
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap(px(8.))
                        .child(m_button(
                            "dialog-cancel",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            "Cancel",
                            cx.listener(|this, _, _, cx| {
                                this.overlay = Overlay::None;
                                cx.notify();
                            }),
                        ))
                        .child(m_button(
                            "dialog-ok",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            "Accept",
                            cx.listener(|this, _, _, cx| {
                                this.overlay = Overlay::None;
                                cx.notify();
                            }),
                        )),
                ),
        )
}

fn sheet_overlay(theme: &Theme, cx: &mut Context<CatalogView>) -> impl IntoElement {
    let a = bottom_sheet::resolve(theme, true);
    div()
        .id("sheet-scrim")
        .flex_1()
        .w_full()
        .bg(paint(a.scrim))
        .flex()
        .flex_col()
        .justify_end()
        .on_click(cx.listener(|this, _, _, cx| {
            this.overlay = Overlay::None;
            cx.notify();
        }))
        .child(
            div()
                .id("sheet-card")
                .w_full()
                .bg(paint(a.container))
                .rounded_tl(px(a.corners.top_left))
                .rounded_tr(px(a.corners.top_right))
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .mt(px(16.))
                        .mb(px(8.))
                        .w(px(a.handle_w))
                        .h(px(a.handle_h))
                        .rounded(px(2.))
                        .bg(paint(a.handle)),
                )
                .children(
                    ["Share", "Add to favorites", "Delete"]
                        .into_iter()
                        .map(|label| {
                            div()
                                .w_full()
                                .h(px(56.))
                                .px(px(16.))
                                .flex()
                                .items_center()
                                .text_color(paint(a.content))
                                .child(label)
                        }),
                ),
        )
}

fn menu_overlay(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let shell = menu::resolve_menu(theme);
    let labels = ["Item one", "Item two", "Item three"];
    div()
        .id("menu-scrim")
        .flex_1()
        .w_full()
        .p(px(24.))
        .bg(paint(
            theme
                .color
                .scrim
                .with_alpha(0.32)
                .composite_over(theme.color.surface),
        ))
        .on_click(cx.listener(|this, _, _, cx| {
            this.overlay = Overlay::None;
            cx.notify();
        }))
        .child(
            div()
                .id("menu-card")
                .min_w(px(200.))
                .py(px(8.))
                .rounded(px(shell.corners.top_left))
                .bg(paint(shell.container))
                .flex()
                .flex_col()
                .children(labels.into_iter().enumerate().map(|(i, label)| {
                    let a = menu::resolve_item(
                        theme,
                        this.menu_selected == i,
                        InteractionState::Enabled,
                    );
                    div()
                        .id(SharedString::from(format!("menu-item-{i}")))
                        .h(px(a.height_dp))
                        .px(px(12.))
                        .flex()
                        .items_center()
                        .bg(paint(a.container))
                        .text_color(paint(a.label))
                        .child(label)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.menu_selected = i;
                            this.overlay = Overlay::None;
                            cx.notify();
                        }))
                })),
        )
}

fn tab_row(
    _theme: &Theme,
    a: &tabs::TabsAppearance,
    selected: usize,
    prefix: &'static str,
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
            let is_primary = prefix == "p";
            div()
                .id(SharedString::from(format!("tab-{prefix}-{i}")))
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
                    if is_primary {
                        this.tab_primary = i;
                    } else {
                        this.tab_secondary = i;
                    }
                    cx.notify();
                }))
        }))
}

fn badge_icon(
    icon: &Appearance,
    badge: Option<(Argb, Option<(String, Argb)>)>,
) -> impl IntoElement {
    let (dot, label) = match badge {
        Some((bg, Some((text, fg)))) => (None, Some((bg, text, fg))),
        Some((bg, None)) => (Some(bg), None),
        None => (None, None),
    };
    div()
        .w(px(40.))
        .h(px(40.))
        .rounded(px(20.))
        .bg(paint(icon.container))
        .text_color(paint(icon.content))
        .flex()
        .items_center()
        .justify_center()
        .child("★")
        .when_some(dot, |el, bg| {
            el.child(div().w(px(6.)).h(px(6.)).rounded(px(3.)).bg(paint(bg)))
        })
        .when_some(label, |el, (bg, text, fg)| {
            el.child(
                div()
                    .px(px(4.))
                    .h(px(16.))
                    .rounded(px(8.))
                    .bg(paint(bg))
                    .text_color(paint(fg))
                    .text_size(px(11.))
                    .child(text),
            )
        })
}

fn fab_box(a: &Appearance, label: &'static str) -> impl IntoElement {
    div()
        .w(px(a.width_dp.unwrap_or(a.height_dp)))
        .h(px(a.height_dp))
        .rounded(px(a.corners.top_left))
        .bg(paint(a.container))
        .text_color(paint(a.content))
        .flex()
        .items_center()
        .justify_center()
        .child(label)
}

fn onscreen_keys(theme: &Theme, cx: &mut Context<CatalogView>) -> impl IntoElement {
    let rows: [&[&str]; 4] = [
        &["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
        &["a", "s", "d", "f", "g", "h", "j", "k", "l"],
        &["z", "x", "c", "v", "b", "n", "m", "⌫"],
        &["@", ".", "-", "_", " "],
    ];
    let mut row_els = Vec::new();
    for (ri, row) in rows.iter().enumerate() {
        let mut keys = Vec::new();
        for (ci, key) in row.iter().enumerate() {
            let key = (*key).to_string();
            let key_click = key.clone();
            let label = if key == " " {
                "space".to_string()
            } else {
                key.clone()
            };
            keys.push(
                div()
                    .id(SharedString::from(format!("key-{ri}-{ci}")))
                    .px(px(if key == " " { 28. } else { 10. }))
                    .py(px(10.))
                    .rounded(px(8.))
                    .bg(paint(theme.color.surface_container_high))
                    .text_color(paint(theme.color.on_surface))
                    .text_size(px(14.))
                    .child(label)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.apply_key(&key_click);
                        cx.notify();
                    })),
            );
        }
        row_els.push(div().flex().justify_center().gap(px(4.)).children(keys));
    }
    div()
        .w_full()
        .p(px(6.))
        .gap(px(4.))
        .flex()
        .flex_col()
        .bg(paint(theme.color.surface_container))
        .children(row_els)
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

fn section_title(theme: &Theme, title: &'static str) -> impl IntoElement {
    div()
        .mt(px(8.))
        .text_size(px(theme.typography.title_medium.size_sp))
        .font_weight(FontWeight::MEDIUM)
        .text_color(paint(theme.color.on_surface))
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
    div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(
            div()
                .id(id)
                .h(px(field.field.height_dp))
                .px(px(16.))
                .rounded(px(field.field.corners.top_left))
                .bg(paint(field.field.container))
                .border_1()
                .border_color(paint(field.field.outline.unwrap().0))
                .flex()
                .flex_col()
                .justify_center()
                .on_click(on_click)
                .child(
                    div()
                        .text_size(px(field.label_style.size_sp))
                        .text_color(paint(field.label))
                        .child(label.into()),
                )
                .child(
                    div()
                        .text_size(px(field.input_style.size_sp))
                        .text_color(paint(field.input))
                        .child(value.into()),
                ),
        )
        .child(
            div()
                .px(px(16.))
                .text_size(px(field.supporting_style.size_sp))
                .text_color(paint(field.supporting))
                .child(supporting.into()),
        )
}

fn list_row(
    a: &Appearance,
    title: &'static str,
    support: Option<&'static str>,
) -> impl IntoElement {
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
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(SharedString::from(format!("nav-{label}")))
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
        .on_click(on_click)
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
                overlay: Overlay::None,
                menu_selected: 0,
                slider: 0.4,
                tab_primary: 0,
                tab_secondary: 0,
                badge_count: 8,
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
                filled: TextFieldEditor::new(text_field::TextFieldVariant::Filled, "hello"),
                outlined: {
                    let mut ed = TextFieldEditor::new(
                        text_field::TextFieldVariant::Outlined,
                        "not-an-email",
                    );
                    ed.error = true;
                    ed
                },
                nav: 0,
            })
        })
        .expect("failed to open window");
    });
}
