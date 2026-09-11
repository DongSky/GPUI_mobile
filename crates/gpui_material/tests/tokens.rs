//! Golden tests against androidx Material 3 token values (v0_210 palette / type scale).

use gpui_material::components::{
    button, card, checkbox, chip, divider, fab, icon_button, list, navigation_bar, progress, radio,
    snackbar, switch, text_field, top_app_bar,
};
use gpui_material::inventory::{Parity, INVENTORY};
use gpui_material::palette;
use gpui_material::state::{
    InteractionState, DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY, FOCUS_OPACITY,
    HOVER_OPACITY, PRESSED_OPACITY,
};
use gpui_material::theme::Theme;
use gpui_material::Argb;

fn hex(c: Argb) -> String {
    c.css_hex()
}

#[test]
fn palette_primary_40_is_baseline_purple() {
    assert_eq!(hex(palette::PRIMARY.tone40), "#6750A4");
    assert_eq!(palette::PRIMARY.tone40, Argb::rgb(103, 80, 164));
}

#[test]
fn light_scheme_matches_androidx_color_light_tokens() {
    let c = Theme::light().color;
    assert_eq!(hex(c.primary), "#6750A4");
    assert_eq!(hex(c.on_primary), "#FFFFFF");
    assert_eq!(hex(c.primary_container), "#EADDFF");
    assert_eq!(hex(c.on_primary_container), "#21005D");
    assert_eq!(hex(c.secondary), "#625B71");
    assert_eq!(hex(c.secondary_container), "#E8DEF8");
    assert_eq!(hex(c.tertiary), "#7D5260");
    assert_eq!(hex(c.error), "#B3261E");
    assert_eq!(hex(c.error_container), "#F9DEDC");
    assert_eq!(hex(c.surface), "#FEF7FF");
    assert_eq!(hex(c.on_surface), "#1D1B20");
    assert_eq!(hex(c.surface_container_lowest), "#FFFFFF");
    assert_eq!(hex(c.surface_container_low), "#F7F2FA");
    assert_eq!(hex(c.surface_container), "#F3EDF7");
    assert_eq!(hex(c.surface_container_high), "#ECE6F0");
    assert_eq!(hex(c.surface_container_highest), "#E6E0E9");
    assert_eq!(hex(c.outline), "#79747E");
    assert_eq!(hex(c.outline_variant), "#CAC4D0");
    assert_eq!(hex(c.inverse_primary), "#D0BCFF");
}

#[test]
fn dark_scheme_matches_androidx_color_dark_tokens() {
    let c = Theme::dark().color;
    assert_eq!(hex(c.primary), "#D0BCFF");
    assert_eq!(hex(c.on_primary), "#381E72");
    assert_eq!(hex(c.primary_container), "#4F378B");
    assert_eq!(hex(c.surface), "#141218");
    assert_eq!(hex(c.on_surface), "#E6E0E9");
    assert_eq!(hex(c.surface_container_lowest), "#0F0D13");
    assert_eq!(hex(c.surface_container), "#211F26");
    assert_eq!(hex(c.outline), "#938F99");
    assert_eq!(hex(c.error), "#F2B8B5");
}

#[test]
fn type_scale_matches_androidx_type_scale_tokens() {
    let t = Theme::light().typography;
    assert_eq!((t.display_large.size_sp, t.display_large.line_height_sp), (57.0, 64.0));
    assert_eq!(t.display_large.tracking_sp, -0.2);
    assert_eq!((t.body_large.size_sp, t.body_large.line_height_sp, t.body_large.tracking_sp, t.body_large.weight), (16.0, 24.0, 0.5, 400));
    assert_eq!((t.label_large.size_sp, t.label_large.weight), (14.0, 500));
    assert_eq!((t.title_medium.size_sp, t.title_medium.weight), (16.0, 500));
    assert_eq!(t.label_small.size_sp, 11.0);
}

#[test]
fn shape_and_elevation_and_motion_tokens() {
    let th = Theme::light();
    assert_eq!(th.shapes.extra_small, 4.0);
    assert_eq!(th.shapes.medium, 12.0);
    assert_eq!(th.shapes.large, 16.0);
    assert_eq!(th.shapes.extra_large, 28.0);
    assert_eq!(th.elevation.level1, 1.0);
    assert_eq!(th.elevation.level3, 6.0);
    assert_eq!(th.elevation.level5, 12.0);
    assert_eq!(th.motion.short4_ms, 200);
    assert_eq!(th.motion.medium2_ms, 300);
    assert_eq!(th.motion.emphasized, "cubic-bezier(0.2, 0.0, 0.0, 1.0)");
}

#[test]
fn state_layer_opacities_match_m3_guidance() {
    assert_eq!(HOVER_OPACITY, 0.08);
    assert_eq!(FOCUS_OPACITY, 0.10);
    assert_eq!(PRESSED_OPACITY, 0.10);
    assert_eq!(DISABLED_CONTENT_OPACITY, 0.38);
    assert_eq!(DISABLED_CONTAINER_OPACITY, 0.12);
}

#[test]
fn filled_button_enabled_uses_primary_on_primary() {
    let theme = Theme::light();
    let a = button::resolve(&theme, button::ButtonVariant::Filled, InteractionState::Enabled);
    assert_eq!(a.height_dp, 40.0);
    assert_eq!(a.min_width_dp, Some(64.0));
    assert_eq!(a.corners.top_left, 20.0);
    assert_eq!(a.pad_start_dp, 24.0);
    assert_eq!(a.label_style.name, "labelLarge");
    assert_eq!(a.container, theme.color.primary);
    assert_eq!(a.content, theme.color.on_primary);
    assert_eq!(a.elevation_dp, 0.0);
}

#[test]
fn filled_button_disabled_composites_on_surface() {
    let theme = Theme::light();
    let a = button::resolve(&theme, button::ButtonVariant::Filled, InteractionState::Disabled);
    let expected_container = theme
        .color
        .on_surface
        .with_alpha(0.12)
        .composite_over(theme.color.surface);
    let expected_label = theme
        .color
        .on_surface
        .with_alpha(0.38)
        .composite_over(theme.color.surface);
    assert_eq!(a.container, expected_container);
    assert_eq!(a.content, expected_label);
    assert_eq!(a.elevation_dp, 0.0);
}

#[test]
fn outlined_and_text_buttons_keep_transparent_container() {
    let theme = Theme::light();
    let outlined = button::resolve(&theme, button::ButtonVariant::Outlined, InteractionState::Enabled);
    assert_eq!(outlined.container, theme.color.surface);
    assert_eq!(outlined.outline, Some((theme.color.outline, 1.0)));
    assert_eq!(outlined.content, theme.color.primary);
    let text = button::resolve(&theme, button::ButtonVariant::Text, InteractionState::Enabled);
    assert_eq!(text.outline, None);
    assert_eq!(text.content, theme.color.primary);
}

#[test]
fn tonal_and_elevated_button_roles() {
    let theme = Theme::light();
    let tonal = button::resolve(&theme, button::ButtonVariant::Tonal, InteractionState::Enabled);
    assert_eq!(tonal.container, theme.color.secondary_container);
    assert_eq!(tonal.content, theme.color.on_secondary_container);
    let elevated = button::resolve(&theme, button::ButtonVariant::Elevated, InteractionState::Enabled);
    assert_eq!(elevated.container, theme.color.surface_container_low);
    assert_eq!(elevated.content, theme.color.primary);
    assert_eq!(elevated.elevation_dp, 1.0);
}

#[test]
fn pressed_state_layer_tints_filled_button() {
    let theme = Theme::light();
    let enabled = button::resolve(&theme, button::ButtonVariant::Filled, InteractionState::Enabled);
    let pressed = button::resolve(&theme, button::ButtonVariant::Filled, InteractionState::Pressed);
    assert_ne!(pressed.container, enabled.container);
    let expected = theme
        .color
        .on_primary
        .with_alpha(0.10)
        .composite_over(theme.color.primary);
    assert_eq!(pressed.container, expected);
}

#[test]
fn text_field_metrics_and_error_focus() {
    let theme = Theme::light();
    let filled = text_field::resolve(
        &theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        false,
    );
    assert_eq!(filled.field.height_dp, 56.0);
    assert_eq!(filled.field.corners.top_left, 4.0);
    assert_eq!(filled.field.corners.bottom_left, 0.0);
    assert_eq!(filled.field.container, theme.color.surface_container_highest);
    assert_eq!(filled.label_style.name, "bodyLarge");

    let focused = text_field::resolve(
        &theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    assert_eq!(focused.field.outline, Some((theme.color.primary, 2.0)));
    assert_eq!(focused.label, theme.color.primary);
    assert_eq!(focused.label_style.name, "bodySmall");

    let error = text_field::resolve(
        &theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Error,
        true,
    );
    assert_eq!(error.field.outline.unwrap().0, theme.color.error);
    assert_eq!(error.supporting, theme.color.error);
}

#[test]
fn list_heights_match_m3() {
    let theme = Theme::light();
    assert_eq!(
        list::resolve(&theme, list::ListLines::One, InteractionState::Enabled).height_dp,
        56.0
    );
    assert_eq!(
        list::resolve(&theme, list::ListLines::Two, InteractionState::Enabled).height_dp,
        72.0
    );
    assert_eq!(
        list::resolve(&theme, list::ListLines::Three, InteractionState::Enabled).height_dp,
        88.0
    );
}

#[test]
fn checkbox_radio_switch_metrics() {
    let theme = Theme::light();
    let box_on = checkbox::resolve(&theme, checkbox::CheckValue::Checked, InteractionState::Enabled);
    assert_eq!(box_on.box_size_dp, 18.0);
    assert_eq!(box_on.corner_dp, 2.0);
    assert_eq!(box_on.target_dp, 48.0);
    assert_eq!(box_on.box_fill, theme.color.primary);
    assert_eq!(box_on.icon, theme.color.on_primary);

    let radio_on = radio::resolve(&theme, true, InteractionState::Enabled);
    assert_eq!(radio_on.outer_dp, 20.0);
    assert_eq!(radio_on.inner, Some(theme.color.primary));

    let sw = switch::resolve(&theme, true, InteractionState::Enabled);
    assert_eq!((sw.track_w, sw.track_h), (52.0, 32.0));
    assert_eq!(sw.thumb_dp, 24.0);
    assert_eq!(sw.track, theme.color.primary);
    let sw_off = switch::resolve(&theme, false, InteractionState::Enabled);
    assert_eq!(sw_off.thumb_dp, 16.0);
}

#[test]
fn card_chip_fab_chrome_tokens() {
    let theme = Theme::light();
    let elevated = card::resolve(&theme, card::CardVariant::Elevated, InteractionState::Enabled);
    assert_eq!(elevated.corners.top_left, 12.0);
    assert_eq!(elevated.container, theme.color.surface_container_low);
    assert_eq!(elevated.elevation_dp, 1.0);

    let filter = chip::resolve(&theme, chip::ChipVariant::Filter, true, InteractionState::Enabled);
    assert_eq!(filter.height_dp, 32.0);
    assert_eq!(filter.container, theme.color.secondary_container);

    let fab = fab::resolve(&theme, fab::FabVariant::Primary, InteractionState::Enabled);
    assert_eq!(fab.height_dp, 56.0);
    assert_eq!(fab.corners.top_left, 16.0);
    assert_eq!(fab.container, theme.color.primary_container);

    let icon = icon_button::resolve(
        &theme,
        icon_button::IconButtonVariant::Standard,
        InteractionState::Enabled,
    );
    assert_eq!(icon.height_dp, 40.0);

    let bar = top_app_bar::resolve(&theme);
    assert_eq!(bar.height_dp, 64.0);
    assert_eq!(bar.container, theme.color.surface);

    let snack = snackbar::resolve(&theme);
    assert_eq!(snack.container, theme.color.inverse_surface);
    assert_eq!(snack.action, theme.color.inverse_primary);

    let nav = navigation_bar::resolve(&theme);
    assert_eq!(nav.height_dp, 80.0);
    assert_eq!(nav.container, theme.color.surface_container);
    assert_eq!(nav.active_indicator, theme.color.secondary_container);

    let d = divider::resolve(&theme, true);
    assert_eq!(d.thickness_dp, 1.0);
    assert_eq!(d.color, theme.color.outline_variant);
    assert_eq!(d.inset_dp, 16.0);

    let lin = progress::linear(&theme, 0.5);
    assert_eq!(lin.height_dp, 4.0);
    assert_eq!(lin.indicator, theme.color.primary);
}

#[test]
fn catalog_html_embeds_token_evidence() {
    let html = gpui_material::catalog::render_html(&Theme::light());
    assert!(html.contains("#6750A4"));
    assert!(html.contains("data-button=\"filled\""));
    assert!(html.contains("data-state=\"disabled\""));
    assert!(html.contains("data-field=\"outlined\""));
    assert!(html.contains("data-list=\"two-line\""));
    assert!(html.contains("data-checkbox=\"checked\""));
    assert!(html.contains("data-switch=\"true\""));
    assert!(html.contains("data-card=\"elevated\""));
    assert!(html.contains("data-navbar=\"1\""));
    assert!(html.contains("labelLarge"));
}

#[test]
fn inventory_covers_claimed_and_followups() {
    let names: Vec<_> = INVENTORY.iter().map(|e| e.name).collect();
    for required in ["Button", "Text field", "List", "Checkbox", "Switch", "Card"] {
        assert!(names.contains(&required), "missing {required}");
    }
    assert!(INVENTORY.iter().any(|e| e.name == "Button" && e.parity == Parity::Done));
    assert!(INVENTORY.iter().any(|e| e.name == "Text field" && e.parity == Parity::Partial));
    assert!(INVENTORY.iter().any(|e| e.name == "Dialog" && e.parity == Parity::NotStarted));
}
