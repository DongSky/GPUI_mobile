//! Golden tests against androidx Material 3 token values (v0_210 palette / type scale).

use gpui_material::components::{
    badge, bottom_sheet, button, button_group, card, carousel, checkbox, chip, date_picker, dialog, divider,
    fab, fab_menu, icon_button, list, menu, navigation_bar, navigation_rail, progress, radio, search, slider, snackbar, split_button, switch,
    tabs, text_field, time_picker, toolbar, top_app_bar,
};
use gpui_material::inventory::{Parity, INVENTORY};
use gpui_material::motion;
use gpui_material::palette;
use gpui_material::state::{
    InteractionState, DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY, FOCUS_OPACITY,
    HOVER_OPACITY, PRESSED_OPACITY,
};
use gpui_material::theme::Theme;
use gpui_material::typography;
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
    assert_eq!(
        (t.display_large.size_sp, t.display_large.line_height_sp),
        (57.0, 64.0)
    );
    assert_eq!(t.display_large.tracking_sp, -0.2);
    assert_eq!(
        (
            t.body_large.size_sp,
            t.body_large.line_height_sp,
            t.body_large.tracking_sp,
            t.body_large.weight
        ),
        (16.0, 24.0, 0.5, 400)
    );
    assert_eq!((t.label_large.size_sp, t.label_large.weight), (14.0, 500));
    assert_eq!((t.title_medium.size_sp, t.title_medium.weight), (16.0, 500));
    assert_eq!(t.label_small.size_sp, 11.0);
    let emp = t.emphasized();
    assert_eq!(emp.headline_small.name, "headlineSmallEmphasized");
    assert_eq!(emp.headline_small.size_sp, 24.0);
    assert_eq!(emp.headline_small.weight, 500);
    assert_eq!(emp.headline_large.weight, 500);
    assert_eq!(emp.title_medium.weight, 700);
    assert_eq!(emp.label_large.weight, 700);
    assert_eq!(emp.body_large.weight, 500);
}

#[test]
fn shape_and_elevation_and_motion_tokens() {
    let th = Theme::light();
    assert_eq!(th.shapes.extra_small, 4.0);
    assert_eq!(th.shapes.medium, 12.0);
    assert_eq!(th.shapes.large, 16.0);
    assert_eq!(th.shapes.large_increased, 20.0);
    assert_eq!(th.shapes.extra_large, 28.0);
    assert_eq!(th.shapes.extra_extra_large, 48.0);
    assert_eq!(th.elevation.level1, 1.0);
    assert_eq!(th.elevation.level3, 6.0);
    assert_eq!(th.elevation.level5, 12.0);
    assert_eq!(th.motion.short4_ms, 200);
    assert_eq!(th.motion.medium2_ms, 300);
    assert_eq!(th.motion.emphasized, "cubic-bezier(0.2, 0.0, 0.0, 1.0)");
    assert_eq!(
        th.motion.spatial_fast,
        "cubic-bezier(0.42, 1.67, 0.21, 0.90)"
    );
    assert_eq!(th.motion.spatial_fast_ms, 350);
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
    let a = button::resolve(
        &theme,
        button::ButtonVariant::Filled,
        InteractionState::Enabled,
    );
    assert_eq!(a.height_dp, 40.0);
    assert_eq!(a.min_width_dp, Some(64.0));
    assert_eq!(a.corners.top_left, 20.0);
    assert_eq!(a.pad_start_dp, 16.0);
    assert_eq!(a.label_style.name, "labelLarge");
    assert_eq!(a.container, theme.color.primary);
    assert_eq!(a.content, theme.color.on_primary);
    assert_eq!(a.elevation_dp, 0.0);
    let xl = button::resolve_expressive(
        &theme,
        button::ButtonVariant::Filled,
        button::ButtonSize::ExtraLarge,
        button::ButtonShape::Round,
        InteractionState::Enabled,
    );
    assert_eq!(xl.height_dp, 136.0);
    let pressed = button::resolve(
        &theme,
        button::ButtonVariant::Filled,
        InteractionState::Pressed,
    );
    assert_eq!(pressed.corners.top_left, 8.0);
}

#[test]
fn filled_button_disabled_composites_on_surface() {
    let theme = Theme::light();
    let a = button::resolve(
        &theme,
        button::ButtonVariant::Filled,
        InteractionState::Disabled,
    );
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
    let outlined = button::resolve(
        &theme,
        button::ButtonVariant::Outlined,
        InteractionState::Enabled,
    );
    assert_eq!(outlined.container, theme.color.surface);
    assert_eq!(outlined.outline, Some((theme.color.outline_variant, 1.0)));
    assert_eq!(outlined.content, theme.color.on_surface_variant);
    let text = button::resolve(
        &theme,
        button::ButtonVariant::Text,
        InteractionState::Enabled,
    );
    assert_eq!(text.outline, None);
    assert_eq!(text.content, theme.color.primary);
    assert_eq!(text.container, Argb::TRANSPARENT);
}

#[test]
fn tonal_and_elevated_button_roles() {
    let theme = Theme::light();
    let tonal = button::resolve(
        &theme,
        button::ButtonVariant::Tonal,
        InteractionState::Enabled,
    );
    assert_eq!(tonal.container, theme.color.secondary_container);
    assert_eq!(tonal.content, theme.color.on_secondary_container);
    let elevated = button::resolve(
        &theme,
        button::ButtonVariant::Elevated,
        InteractionState::Enabled,
    );
    assert_eq!(elevated.container, theme.color.surface_container_low);
    assert_eq!(elevated.content, theme.color.primary);
    assert_eq!(elevated.elevation_dp, 1.0);
}

#[test]
fn pressed_state_layer_tints_filled_button() {
    let theme = Theme::light();
    let enabled = button::resolve(
        &theme,
        button::ButtonVariant::Filled,
        InteractionState::Enabled,
    );
    let pressed = button::resolve(
        &theme,
        button::ButtonVariant::Filled,
        InteractionState::Pressed,
    );
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
    assert_eq!(
        filled.field.container,
        theme.color.surface_container_highest
    );
    assert_eq!(filled.label_style.name, "bodyLarge");
    assert!(!filled.floating);
    assert!(!filled.notched);

    let focused = text_field::resolve(
        &theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    assert_eq!(focused.field.outline, Some((theme.color.primary, 2.0)));
    assert!(focused.notched);
    assert!(focused.floating);
    assert_eq!(focused.label, theme.color.primary);
    assert_eq!(focused.label_style.name, "bodySmall");
    let cut = text_field::notch_cutout("Email", &focused);
    assert_eq!(cut.start_dp, 8.0);
    assert_eq!(cut.stroke_dp, 2.0);
    assert!(cut.width_dp >= 28.0);
    let frame = text_field::notch_frame("Email", &focused);
    assert_eq!(frame.radius_dp, 4.0);
    assert_eq!(frame.stroke_dp, 2.0);
    assert_eq!(frame.top_lead_dp(), 4.0);
    assert_eq!(frame.notch_gap_h_dp(), 2.0);
    assert_eq!(frame.inner_radius_dp(), 2.0);
    let d = frame.outline_svg_d(280.0);
    assert!(d.starts_with('M'));
    assert!(d.contains(" A"));
    assert_eq!(focused.cutout_fill, theme.color.background);
    assert!(text_field::notch_width_dp("Email", 12.0) >= 28.0);
    assert!(
        text_field::notch_width_dp("WWW", 12.0)
            > text_field::notch_width_dp("iii", 12.0)
    );
    assert!(text_field::notch_width_dp("@gmail", 12.0) >= 28.0 + text_field::NOTCH_WIDTH_SAFETY_DP);
    assert!(
        (text_field::notch_width_from_measured_dp(40.0)
            - (40.0 + text_field::NOTCH_PAD_DP * 2.0 + text_field::NOTCH_WIDTH_SAFETY_DP))
            .abs()
            < 1e-5
    );
    let from_layout = text_field::notch_frame_from_layout("Email", &focused, 40.0);
    assert!((from_layout.width_dp - text_field::notch_width_from_measured_dp(40.0)).abs() < 1e-5);
    let fallback = text_field::notch_frame_from_layout("Email", &focused, 0.0);
    assert_eq!(fallback.width_dp, text_field::notch_frame("Email", &focused).width_dp);
    assert!(text_field::roboto_advance_em('W') > text_field::roboto_advance_em('i'));

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
    let box_on = checkbox::resolve(
        &theme,
        checkbox::CheckValue::Checked,
        InteractionState::Enabled,
    );
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
    let elevated = card::resolve(
        &theme,
        card::CardVariant::Elevated,
        InteractionState::Enabled,
    );
    assert_eq!(elevated.corners.top_left, 12.0);
    assert_eq!(elevated.container, theme.color.surface_container_low);
    assert_eq!(elevated.elevation_dp, 1.0);

    let filter = chip::resolve(
        &theme,
        chip::ChipVariant::Filter,
        true,
        InteractionState::Enabled,
    );
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
    let icon_xl = icon_button::resolve_expressive(
        &theme,
        icon_button::IconButtonVariant::Filled,
        button::ButtonSize::ExtraLarge,
        button::ButtonShape::Round,
        InteractionState::Enabled,
    );
    assert_eq!(icon_xl.height_dp, 136.0);
    assert_eq!(icon_button::icon_dp(button::ButtonSize::ExtraLarge), 40.0);

    let bar = top_app_bar::resolve(&theme);
    assert_eq!(bar.height_dp, 64.0);
    assert_eq!(bar.container, theme.color.surface);

    let snack = snackbar::resolve(&theme);
    assert_eq!(snack.container, theme.color.inverse_surface);
    assert_eq!(snack.action, theme.color.inverse_primary);
    assert_eq!(snack.close, theme.color.inverse_on_surface);
    assert_eq!(snackbar::TIMEOUT_SHORT_MS, 4000);
    assert_eq!(snackbar::SWIPE_DISMISS_DP, 72.0);
    assert_eq!(snackbar::SCENE_MESSAGE, "Email archived");
    assert_eq!(snackbar::SCENE_ACTION, "Action");
    assert!(snackbar::HAS_CLOSE);
    assert_eq!(snackbar::MAIL_ROWS.len(), 3);
    assert!(snackbar::MAIL_ROWS[0].peek);
    assert_eq!(snackbar::MAIL_ROWS[0].from, "Shows lined up");
    assert_eq!(snackbar::MAIL_ROWS[1].from, "Sofia Sacchi");
    assert_eq!(snackbar::MAIL_ROWS[1].time, "1 hr ago");
    assert!(!snackbar::SCENE_SHOW_TITLE);
    assert_eq!(snackbar::INBOX_NAV.len(), 4);
    assert_eq!(snackbar::INBOX_NAV[0].label, "Mail");
    assert_eq!(snackbar::INBOX_NAV[3].label, "Meet");
    assert!(snackbar::INBOX_NAV[0].svg.contains("svg"));
    assert_eq!(snackbar::STATUS_TIME, "9:41");
    let mut snack_state = snackbar::SnackbarState::short();
    assert!(snack_state.visible);
    assert!(!snack_state.tick(1000.0));
    assert!(snack_state.tick(4000.0));
    assert!(snack_state.dismissed());
    let mut swipe = snackbar::SnackbarState::short();
    swipe.swipe(80.0);
    assert!(swipe.dismissed());
    assert_eq!(swipe.opacity(), 0.0);
    let mut closed = snackbar::SnackbarState::short();
    closed.close();
    assert!(closed.dismissed());

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
    let indet = progress::linear_indeterminate(&theme);
    assert_eq!(indet.head_span, progress::INDETERMINATE_SPAN);
    assert_eq!(indet.indicator, theme.color.primary);
    assert_eq!(progress::pull_to_refresh(&theme).size_dp, 40.0);
    let wave = progress::wavy(&theme, 0.6);
    assert_eq!(wave.height_dp, 16.0);
    assert_eq!(wave.indicator, theme.color.primary);
    assert!(progress::wave_polyline(240.0, 16.0, 0.6, 0.0).len() > 8);
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
    assert!(html.contains("data-dialog=\"basic\""));
    assert!(html.contains("data-dialog=\"list\""));
    assert!(html.contains("data-dialog=\"fullscreen\""));
    assert!(html.contains("data-fs-header"));
    assert!(html.contains("Event title"));
    assert!(html.contains("Phone ringtone"));
    assert!(html.contains("data-icon-group=\"1\""));
    assert!(html.contains("data-overflow-menu"));
    assert!(html.contains("data-carousel-layout=\"multi-browse\""));
    assert!(html.contains("data-carousel-layout=\"centered-hero\""));
    assert!(html.contains("data-carousel-layout=\"full-screen\""));
    assert!(html.contains("data-carousel-phone"));
    assert!(html.contains("data-carousel-centered"));
    assert!(html.contains("data-carousel-media"));
    assert!(html.contains("data-timeout-ms"));
    assert!(html.contains("data-swipe-dismiss"));
    assert!(html.contains("data-snackbar-close"));
    assert!(html.contains("data-persist=\"1\""));
    assert!(html.contains("Email archived"));
    assert!(html.contains("data-mail-avatar"));
    assert!(html.contains("data-mail-time"));
    assert!(html.contains("data-inbox-nav=\"Mail\""));
    assert!(html.contains("data-inbox-nav=\"Meet\""));
    assert!(
        html.contains(r#"style="background:#E8DEF8;""#),
        "inbox-nav active indicator must close the style attribute"
    );
    assert!(html.contains("data-meet-badge"));
    assert!(html.contains("Sofia Sacchi"));
    assert!(html.contains("data-fab-scene"));
    assert!(html.contains("data-split-scene"));
    assert!(html.contains("Enamel mugs"));
    assert!(html.contains("data-toolbar-scene"));
    assert!(html.contains("Renee Claess"));
    assert!(html.contains("data-photo=\"sofia\""));
    assert!(html.contains("data-photo=\"basket\""));
    assert!(html.contains("data-decoded-jpeg=\"1\""));
    assert!(html.contains("url('data:image/jpeg;base64,"));
    assert!(html.contains("data-mail-peek=\"1\""));
    assert!(html.contains("data-carousel-lists"));
    assert!(html.contains("Your lists"));
    assert!(html.contains("Starred places"));
    assert!(html.contains("class=\"nav-ico\""));
    assert!(html.contains("data-carousel-layout=\"uncontained-multi\""));
    assert!(html.contains(r#"layout === "uncontained-multi" ? 168"#));
    assert!(html.contains("photo-hero"));
    assert!(html.contains("Alejandro"));
    assert!(html.contains("Order prints"));
    assert!(html.contains("data-media-back"));
    assert!(html.contains("data-status-bar"));
    assert!(html.contains("data-share-people"));
    assert!(html.contains("data-share-person"));
    assert!(html.contains("data-fab-menu"));
    assert!(html.contains("data-fab-item=\"Document\""));
    assert!(html.contains("data-hero=\"fab-menu\""));
    assert!(html.contains("data-split=\"filled\""));
    assert!(html.contains("data-split-trail"));
    assert!(html.contains("Add to cart"));
    assert!(html.contains("data-hero=\"split-button\""));
    assert!(html.contains("data-toolbar=\"floating\""));
    assert!(html.contains("data-toolbar-color=\"vibrant\""));
    assert!(html.contains("data-toolbar-fab"));
    assert!(html.contains("data-hero=\"toolbar\""));
    assert!(html.contains("data-tabs=\"primary-icons\""));
    assert!(html.contains("data-media-scene"));
    assert!(html.contains("My saved media"));
    assert!(html.contains("data-sheet=\"standard\""));
    assert!(html.contains("data-sheet-share"));
    assert!(html.contains("data-share-photo"));
    assert!(html.contains("data-hero=\"snackbar\""));
    assert!(html.contains("data-hero=\"dialog-fullscreen\""));
    assert!(html.contains("data-hero=\"button-group-icons\""));
    assert!(html.contains("data-field-hero=\"empty-filled\""));
    assert!(html.contains("data-field-hero=\"empty-outlined\""));
    assert!(html.contains("data-week-start=\"sunday\""));
    assert!(html.contains("data-stops=\"13\""));
    assert!(html.contains("Alarm volume"));
    assert!(html.contains("Call volume"));
    assert!(html.contains("data-notch=\"cutout\""));
    assert!(html.contains("leevilanuevanotes@google.com"));
    assert!(html.contains("data-dialog-accounts=\"1\""));
    assert!(html.contains("data-datepicker-range=\"1\""));
    assert!(html.contains("Depart – Return dates"));
    assert!(html.contains("data-handle-visual=\"28\""));
    assert!(html.contains("September 2026 ▾"));
    assert!(html.contains("headlineSmallEmphasized"));
    assert!(html.contains("headlineLargeEmphasized"));
    assert!(html.contains("data-button-group=\"connected\""));
    assert!(html.contains("data-slider-range=\"1\""));
    assert!(html.contains("data-datepicker-docked=\"1\""));
    assert!(html.contains("data-settings-scene=\"1\""));
    assert!(html.contains("data-settings-block=\"volume\""));
    assert!(html.contains("data-search=\"1\""));
    assert!(html.contains("Hinted search text"));
    assert!(html.contains("data-search-view=\"1\""));
    assert!(html.contains("data-timepicker=\"1\""));
    assert!(html.contains("data-dial=\"minute\""));
    assert!(html.contains("data-progress=\"indeterminate\""));
    assert!(html.contains("data-progress=\"ptr\""));
    assert!(html.contains("data-nav-rail=\"1\""));
    assert!(html.contains("data-range-interactive=\"1\""));
    assert!(html.contains("data-dismiss-outside=\"1\""));
    assert!(html.contains("data-docked-month"));
    assert!(html.contains("displaySmallEmphasized"));
    assert!(html.contains("data-datepicker-popup=\"open\""));
    assert!(html.contains("titleMediumEmphasized"));
    assert!(html.contains("Sound &amp; notifications") || html.contains("Sound & notifications"));
    assert!(html.contains("Date of birth"));
    assert!(html.contains("Price range"));
    assert!(html.contains("data-role=\"leading\""));
    assert!(html.contains("data-sheet=\"modal\""));
    assert!(html.contains("data-menu=\"1\""));
    assert!(html.contains("data-slider=\"0.3 enabled\""));
    assert!(html.contains("data-hero=\"slider\""));
    assert!(html.contains("data-hero=\"buttons\""));
    assert!(html.contains("data-field-hero=\"outlined\""));
    assert!(html.contains("data-notched=\"1\""));
    assert!(html.contains("<legend"));
    assert!(html.contains("data-button-size=\"xl\""));
    assert!(html.contains("data-button-shape=\"square\""));
    assert!(html.contains("data-icon-size=\"xl\""));
    assert!(html.contains("data-hero=\"icon-buttons\""));
    assert!(html.contains("Filled tonal"));
    assert!(html.contains("data-tabs=\"primary\""));
    assert!(html.contains("data-badge=\"large\""));
    assert!(html.contains("data-datepicker=\"1\""));
    assert!(html.contains("data-motion=\"emphasized\""));
    assert!(html.contains("data-fab-size=\"extended\""));
    assert!(html.contains("data-field=\"filled-edit\""));
    assert!(html.contains("data-carousel=\"1\""));
    assert!(html.contains("data-progress=\"wavy\""));
    assert!(html.contains("data-search-activity=\"1\""));
    assert!(html.contains("data-search-input"));
    assert!(html.contains("data-hand-path=\"1\""));
    assert!(html.contains("data-notch-hole=\"1\""));
    assert!(html.contains("data-carousel-selected"));
    assert!(html.contains("data-rail-selected"));
    assert!(html.contains("data-search-morph"));
    assert!(html.contains("data-search-shared"));
    assert!(html.contains("data-search-lead"));
    assert!(html.contains("data-search-avatar"));
    assert!(html.contains("data-search-scale"));
    assert!(html.contains("data-search-transform-origin"));
    assert!(html.contains("data-search-path-scale=\"1\""));
    assert!(html.contains("data-search-layer-box"));
    assert!(html.contains("data-search-anim-scale"));
    assert!(html.contains("data-notch-centerline"));
    assert!(html.contains("data-carousel-snap"));
    assert!(html.contains("data-rail-popup-title"));
    assert!(html.contains("data-rail-frame-ms"));
    assert!(html.contains("data-stroke-cap=\"round\""));
    assert!(html.contains("data-rail-chrome=\"popup\""));
    assert!(html.contains("data-rail-os-popup=\"0\""));
    assert!(html.contains("data-progress-stroke=\"round\""));
    assert!(html.contains("data-rail-scrim"));
    assert!(html.contains("data-notch-evenodd"));
    assert!(html.contains("data-notch-cpath"));
    assert!(html.contains("data-notch-rounded-polygon"));
    assert!(html.contains("data-loading-morph"));
    assert!(html.contains("data-progress=\"loading-determinate\""));
    assert!(html.contains("data-wait-progress"));
    assert!(html.contains("data-second-hand"));
    assert!(html.contains("data-second-wall"));
    assert!(html.contains("data-range-ticks"));
    assert!(html.contains("fill-rule=\"evenodd\""));
    assert!(html.contains("data-carousel-fling"));
    assert!(html.contains("data-carousel-live"));
    assert!(html.contains("Loading"));
    assert!(html.contains("min span 5%") || html.contains("Price range"));
    assert!(html.contains("stroke-linecap=\"round\""));
    assert!(html.contains("data-linecap=\"round\""));
    assert!(html.contains("data-nav-rail-expanded=\"1\""));
    assert!(html.contains("data-rail-focus-trap"));
    assert!(html.contains("data-rail-fab=\"1\""));
    assert!(html.contains("data-rail-window"));
    assert!(html.contains("data-docked-grid=\"1\""));
}

#[test]
fn inventory_covers_claimed_and_followups() {
    let names: Vec<_> = INVENTORY.iter().map(|e| e.name).collect();
    for required in ["Button", "Text field", "List", "Checkbox", "Switch", "Card"] {
        assert!(names.contains(&required), "missing {required}");
    }
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Button" && e.parity == Parity::Done && e.notes.contains("Expressive")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Slider" && e.notes.contains("4×44") && e.notes.contains("surface-container-highest")
    }));
    assert!(!INVENTORY
        .iter()
        .any(|e| e.notes.contains("intentionally not") || e.notes.contains("skip Expressive")));
    assert!(INVENTORY
        .iter()
        .any(|e| e.name == "Button" && e.parity == Parity::Done));
    assert!(INVENTORY
        .iter()
        .any(|e| e.name == "Text field" && e.parity == Parity::Done));
    for required in [
        "Dialog",
        "Bottom sheet",
        "Menu",
        "Slider",
        "Tabs",
        "Badge",
        "Date picker",
        "Motion tokens",
        "Button group",
        "Typography",
        "Search",
        "Time picker",
        "Navigation rail",
        "Carousel",
        "FAB menu",
        "Split button",
        "Toolbar",
    ] {
        assert!(
            INVENTORY
                .iter()
                .any(|e| e.name == required && e.parity == Parity::Done),
            "{required} must be done"
        );
    }
    assert!(!INVENTORY.iter().any(|e| e.parity == Parity::NotStarted));
}

#[test]
fn dialog_sheet_menu_tokens() {
    let theme = Theme::light();
    let d = dialog::resolve(&theme);
    assert_eq!(d.corners.top_left, 28.0);
    assert_eq!(d.container, theme.color.surface_container_high);
    assert_eq!(d.headline_style.name, "headlineSmallEmphasized");
    assert_eq!(d.headline_style.weight, 500);
    assert_eq!(d.supporting_style.name, "bodyMedium");
    assert_eq!(d.action, theme.color.primary);
    assert_eq!(d.elevation_dp, 6.0);
    assert_eq!(d.min_width_dp, 280.0);
    assert_eq!(dialog::RESET_ACCOUNTS.len(), 3);
    assert!(dialog::RESET_SUPPORTING.contains("The following accounts"));
    assert_eq!(dialog::account_initials("leevilanuevanotes@google.com"), "L");
    let fs = dialog::resolve_fullscreen(&theme);
    assert_eq!(fs.corners.top_left, 0.0);
    assert_eq!(fs.header_h_dp, 64.0);
    assert_eq!(fs.container, theme.color.surface);
    assert_eq!(fs.headline_style.name, "titleLargeEmphasized");
    assert_eq!(dialog::FULLSCREEN_FIELDS.len(), 4);
    assert_eq!(dialog::FULLSCREEN_SAVE, "Save");

    let sheet = bottom_sheet::resolve(&theme, true);
    assert_eq!(sheet.corners.top_left, 28.0);
    assert_eq!(sheet.corners.bottom_left, 0.0);
    assert_eq!(sheet.container, theme.color.surface_container_low);
    assert_eq!((sheet.handle_w, sheet.handle_h), (32.0, 4.0));
    assert_eq!(sheet.elevation_dp, 1.0);
    assert_eq!(bottom_sheet::SHARE_TITLE, "Share");
    assert_eq!(bottom_sheet::SHARE_ACTIONS.len(), 5);
    assert_eq!(bottom_sheet::PHOTO_GRID.len(), 6);
    assert_eq!(bottom_sheet::PEOPLE.len(), 5);
    assert_eq!(bottom_sheet::PEOPLE[0].first, "Alejandro");
    assert_eq!(bottom_sheet::SEND_TITLE, "Send");
    assert_eq!(
        bottom_sheet::photo_fill(&theme, 0),
        gpui_material::components::photo_stub::PhotoKind::Party.fill()
    );

    let menu = menu::resolve_menu(&theme);
    assert_eq!(menu.corners.top_left, 4.0);
    assert_eq!(menu.container, theme.color.surface_container);
    assert_eq!(menu.elevation_dp, 3.0);
    let selected = menu::resolve_item(&theme, true, InteractionState::Enabled);
    assert_eq!(selected.container, theme.color.secondary_container);
    assert_eq!(selected.height_dp, 48.0);
}

#[test]
fn slider_tabs_badge_tokens() {
    let theme = Theme::light();
    let s = slider::resolve(&theme, 0.4, InteractionState::Enabled);
    assert_eq!(s.track_h, 16.0);
    assert_eq!(s.handle_w, 4.0);
    assert_eq!(s.handle_h, 44.0);
    assert_eq!(s.handle_h_visual, 28.0);
    assert_eq!(s.gap_dp, 6.0);
    assert_eq!(s.stop_dp, 4.0);
    assert_eq!(s.active, theme.color.primary);
    assert_eq!(s.inactive, theme.color.surface_container_highest);
    assert_eq!(s.value, 0.4);
    assert_eq!(s.stop_count, 2);
    assert_eq!(slider::stop_fractions(5), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    let alarm = slider::resolve_with_stops(&theme, 0.52, InteractionState::Enabled, 13);
    assert_eq!(alarm.stop_count, 13);
    assert_eq!(slider::OVERVIEW_ROWS[1].stop_count, 13);
    assert_eq!(slider::OVERVIEW_ROWS[0].label, "Call volume");
    assert_eq!(slider::segmented_stop_counts(0.52, 13).0, 7);
    let range = slider::resolve_range(
        &theme,
        slider::RANGE_DEMO_START,
        slider::RANGE_DEMO_END,
        InteractionState::Enabled,
    );
    assert_eq!(range.start, 0.20);
    assert_eq!(range.end, 0.75);
    assert_eq!(range.track.track_h, 16.0);
    assert_eq!(slider::RANGE_HERO_LABEL, "Price range");
    let (s, e) = slider::nudge_start(0.20, 0.75, slider::RANGE_STEP);
    assert!((s - 0.25).abs() < 1e-5 && (e - 0.75).abs() < 1e-5);
    let (s, e) = slider::nudge_end(0.20, 0.75, -slider::RANGE_STEP);
    assert!((s - 0.20).abs() < 1e-5 && (e - 0.70).abs() < 1e-5);
    let (s, e) = slider::move_nearest(0.20, 0.75, 0.10);
    assert!((s - 0.10).abs() < 1e-5 && (e - 0.75).abs() < 1e-5);
    assert!(slider::range_value_label(0.2, 0.75).contains("20"));
    let pressed = slider::resolve(&theme, 0.4, InteractionState::Pressed);
    assert_eq!(pressed.handle_w, 2.0);

    let primary = tabs::resolve(&theme, tabs::TabsVariant::Primary);
    assert_eq!(primary.height_dp, 48.0);
    assert_eq!(primary.indicator_h, 3.0);
    assert!(!primary.indicator_full_width);
    assert_eq!(primary.active_label, theme.color.primary);
    let secondary = tabs::resolve(&theme, tabs::TabsVariant::Secondary);
    assert_eq!(secondary.indicator_h, 2.0);
    assert!(secondary.indicator_full_width);
    let icons = tabs::resolve_with_icons(&theme, tabs::TabsVariant::Primary);
    assert_eq!(icons.height_dp, tabs::HEIGHT_WITH_ICON_DP);
    assert_eq!(tabs::DEMO_ICON_LABELS.len(), 3);
    assert_eq!(tabs::SCENE_TITLE, "My saved media");
    assert_eq!(tabs::SCENE_LABELS, ["Video", "Photos", "Audio"]);
    assert_eq!(tabs::SCENE_TILES.len(), 2);
    assert_eq!(tabs::SCENE_SELECTED, 2);
    assert_eq!(tabs::STATUS_TIME, "9:30");

    let small = badge::resolve(&theme, badge::BadgeKind::Small);
    assert_eq!(small.size_dp, 6.0);
    assert_eq!(small.container, theme.color.error);
    let large = badge::resolve(&theme, badge::BadgeKind::Large);
    assert_eq!(large.size_dp, 16.0);
    assert_eq!(badge::label_for_count(8), "8");
    assert_eq!(badge::label_for_count(1000), "999+");
}

#[test]
fn date_picker_grid_and_weekday() {
    let theme = Theme::light();
    let a = date_picker::resolve(&theme);
    assert_eq!(a.day_dp, 40.0);
    assert_eq!(a.corners.top_left, 28.0);
    assert_eq!(a.container, theme.color.surface_container_high);
    assert_eq!(a.date_style.name, "headlineLargeEmphasized");
    assert_eq!(a.date_style.weight, 500);
    // 2026-09-01 is Tuesday → Sunday=0 → 2; Monday=0 → 1
    assert!(date_picker::WEEK_STARTS_ON_SUNDAY);
    assert_eq!(date_picker::WEEKDAYS[0], "S");
    assert_eq!(date_picker::WEEKDAYS[1], "M");
    assert_eq!(date_picker::weekday_sunday0(2026, 9, 1), 2);
    assert_eq!(date_picker::weekday_monday0(2026, 9, 1), 1);
    assert_eq!(
        date_picker::header_date_label(date_picker::CivilDate {
            year: 2026,
            month: 9,
            day: 15,
        }),
        "Tue, Sep 15"
    );
    assert_eq!(date_picker::days_in_month(2024, 2), 29);
    assert_eq!(date_picker::add_months(2026, 1, -1), (2025, 12));
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
    assert_eq!(cells.len(), 42);
    assert_eq!(cells[0], (30, date_picker::DayKind::OutOfMonth));
    assert_eq!(cells[2], (1, date_picker::DayKind::InMonth));
    let fifteenth = cells
        .iter()
        .find(|(d, k)| *d == 15 && *k == date_picker::DayKind::Selected);
    assert!(fifteenth.is_some());
    assert!(cells.iter().any(|(_, k)| *k == date_picker::DayKind::Today));
    assert_eq!(date_picker::month_nav_label(2026, 9), "September 2026 ▾");
    assert_eq!(date_picker::DOCKED_FIELD_LABEL, "Date of birth");
    assert!(date_picker::DOCKED_OPEN_BY_DEFAULT);
    assert!(date_picker::DOCKED_DISMISS_ON_SELECT);
    assert!(date_picker::DOCKED_DISMISS_ON_OUTSIDE);
    assert_eq!(
        date_picker::docked_field_value(date_picker::RANGE_DEMO_START),
        "Sep 15, 2026"
    );
    assert_eq!(
        date_picker::header_range_label(date_picker::RANGE_DEMO_START, date_picker::RANGE_DEMO_END),
        "Sep 15 – Sep 21"
    );
    let range = date_picker::month_grid_range(
        2026,
        9,
        date_picker::RANGE_DEMO_START,
        date_picker::RANGE_DEMO_END,
        today,
    );
    assert!(
        range
            .iter()
            .any(|(d, k)| *d == 18 && *k == date_picker::DayKind::InRange)
    );
    assert!(
        range
            .iter()
            .any(|(d, k)| *d == 15 && *k == date_picker::DayKind::Selected)
    );
}

#[test]
fn text_field_editor_insert_backspace_caret() {
    let mut ed = text_field::TextFieldEditor::new(text_field::TextFieldVariant::Filled, "ab");
    assert_eq!(ed.caret(), 2);
    ed.move_caret(-1);
    ed.insert_char('X');
    assert_eq!(ed.value(), "aXb");
    assert_eq!(ed.caret(), 2);
    ed.backspace();
    assert_eq!(ed.value(), "ab");
    ed.set_focus(true);
    assert_eq!(ed.interaction_state(), InteractionState::Focused);
    assert!(ed.display_with_caret().contains('|'));
    ed.error = true;
    assert_eq!(ed.interaction_state(), InteractionState::ErrorFocused);
    assert!(text_field::looks_like_email("a@b.c"));
    assert!(!text_field::looks_like_email("not-an-email"));
}

#[test]
fn desktop_type_fallbacks_keep_word_gaps() {
    assert_eq!(typography::FONT_FAMILY, "Roboto");
    assert_eq!(typography::FONT_FAMILY_DESKTOP, "Liberation Sans");
    assert_eq!(typography::words("Call volume"), vec!["Call", "volume"]);
    assert_eq!(typography::WORD_GAP_DP, 6.0);
    assert_eq!(typography::words("Reset settings?"), vec!["Reset", "settings?"]);
    let family = typography::desktop_font_family();
    assert!(family == "Roboto" || family == "Liberation Sans");
}

#[test]
fn motion_emphasized_easing_bounds() {
    let m = Theme::light().motion;
    assert_eq!(m.emphasized_at(0.0), 0.0);
    assert_eq!(m.emphasized_at(1.0), 1.0);
    let mid = m.emphasized_at(0.5);
    assert!(mid > 0.5, "emphasized should lead (mid={mid})");
    assert_eq!(motion::cubic_bezier(0.0, 0.0, 1.0, 1.0, 0.3), 0.3);
    assert!((m.lerp(0.0, 10.0, 1.0) - 10.0).abs() < 1e-5);
}

#[test]
fn fab_baseline_sizes() {
    let theme = Theme::light();
    let medium = fab::resolve_size(
        &theme,
        fab::FabVariant::Primary,
        fab::FabSize::Medium,
        InteractionState::Enabled,
    );
    assert_eq!(medium.height_dp, 80.0);
    assert_eq!(medium.corners.top_left, 20.0);
    let large = fab::resolve_size(
        &theme,
        fab::FabVariant::Primary,
        fab::FabSize::Large,
        InteractionState::Enabled,
    );
    assert_eq!(large.height_dp, 96.0);
    assert_eq!(large.corners.top_left, 28.0);
    let ext = fab::resolve_size(
        &theme,
        fab::FabVariant::Primary,
        fab::FabSize::Extended,
        InteractionState::Enabled,
    );
    assert_eq!(ext.height_dp, 56.0);
    assert_eq!(ext.min_width_dp, Some(80.0));
    assert!(ext.width_dp.is_none());
}

#[test]
fn fab_menu_split_button_toolbar_tokens() {
    let theme = Theme::light();
    let item = fab_menu::resolve_item(&theme, fab_menu::FabMenuColor::Primary);
    assert_eq!(item.height_dp, 56.0);
    assert_eq!(item.pad_h_dp, 24.0);
    assert_eq!(item.icon_dp, 24.0);
    assert_eq!(item.corners.top_left, 28.0);
    assert_eq!(item.container, theme.color.primary_container);
    assert_eq!(item.content, theme.color.on_primary_container);
    assert_eq!(item.elevation_dp, 6.0);
    assert_eq!(item.label_style.name, "titleMedium");
    assert_eq!(fab_menu::ITEM_GAP_DP, 4.0);
    assert_eq!(fab_menu::CLOSE_GAP_DP, 8.0);
    assert_eq!(fab_menu::DEMO_ITEMS.len(), 3);
    assert_eq!(fab_menu::DEMO_ITEMS[0].1, "Document");
    let close = fab_menu::resolve_close(&theme, fab_menu::FabMenuColor::Primary, true);
    assert_eq!(close.size_dp, 56.0);
    assert_eq!(close.corners.top_left, 28.0);
    assert_eq!(close.container, theme.color.primary);
    assert_eq!(close.content, theme.color.on_primary);
    assert_eq!(close.icon_dp, 20.0);
    assert_eq!(close.glyph, fab_menu::CLOSE_GLYPH);
    let closed = fab_menu::resolve_close(&theme, fab_menu::FabMenuColor::Primary, false);
    assert_eq!(closed.container, theme.color.primary_container);
    assert_eq!(closed.glyph, fab_menu::OPEN_GLYPH);
    let sec = fab_menu::resolve_item(&theme, fab_menu::FabMenuColor::Secondary);
    assert_eq!(sec.container, theme.color.secondary_container);

    assert_eq!(split_button::GAP_DP, 2.0);
    assert_eq!(split_button::inner_rest_dp(button::ButtonSize::Small), 4.0);
    assert_eq!(split_button::inner_pressed_dp(button::ButtonSize::Small), 12.0);
    assert_eq!(split_button::trailing_icon_dp(button::ButtonSize::Small), 22.0);
    let lead = split_button::resolve_leading(
        &theme,
        split_button::SplitButtonVariant::Filled,
        button::ButtonSize::Small,
        false,
    );
    assert_eq!(lead.height_dp, 40.0);
    assert_eq!(lead.corners.top_left, 20.0);
    assert_eq!(lead.corners.top_right, 4.0);
    assert_eq!(lead.container, theme.color.primary);
    let trail = split_button::resolve_trailing(
        &theme,
        split_button::SplitButtonVariant::Filled,
        button::ButtonSize::Small,
        true,
    );
    assert_eq!(trail.corners.top_left, 20.0);
    assert_eq!(trail.corners.top_right, 20.0);
    assert_eq!(split_button::caret(true), split_button::CARET_OPEN);
    assert_eq!(split_button::DEMO_MENU.len(), 2);
    let outlined = split_button::resolve_leading(
        &theme,
        split_button::SplitButtonVariant::Outlined,
        button::ButtonSize::Small,
        false,
    );
    assert!(outlined.outline.is_some());

    let bar = toolbar::resolve(
        &theme,
        toolbar::ToolbarKind::Floating,
        toolbar::ToolbarColor::Vibrant,
        toolbar::ToolbarAxis::Horizontal,
    );
    assert_eq!(bar.height_dp, 64.0);
    assert_eq!(bar.pad_h_dp, 8.0);
    assert_eq!(bar.item_gap_dp, 4.0);
    assert_eq!(bar.corners.top_left, 32.0);
    assert_eq!(bar.container, theme.color.primary_container);
    assert_eq!(bar.icon, theme.color.on_primary_container);
    assert_eq!(bar.elevation_dp, 6.0);
    let docked = toolbar::resolve(
        &theme,
        toolbar::ToolbarKind::Docked,
        toolbar::ToolbarColor::Standard,
        toolbar::ToolbarAxis::Horizontal,
    );
    assert_eq!(docked.container, theme.color.surface_container);
    assert_eq!(docked.corners.top_left, 0.0);
    assert_eq!(docked.elevation_dp, 0.0);
    assert_eq!(toolbar::DEMO_ICONS.len(), 4);
    let fab = toolbar::resolve_fab(&theme, toolbar::ToolbarColor::Vibrant);
    assert_eq!(fab.height_dp, 56.0);
    assert_eq!(fab.container, theme.color.tertiary_container);
    assert_eq!(toolbar::SCENE_FROM, "Renee Claess");
    assert_eq!(fab_menu::SCENE_PHOTO.label(), "basket");
    assert_eq!(split_button::SCENE_TITLE, "Enamel mugs");
}

#[test]
fn connected_button_group_tokens() {
    let theme = Theme::light();
    assert_eq!(button_group::CONNECTED_GAP_DP, 2.0);
    assert_eq!(button_group::INNER_CORNER_DP, 8.0);
    assert_eq!(button_group::DEMO_SEGMENTS.len(), 3);
    let leading = button_group::resolve_segment(&theme, 0, 3, false, false);
    assert_eq!(leading.corners.top_left, 20.0);
    assert_eq!(leading.corners.top_right, 8.0);
    let selected = button_group::resolve_segment(&theme, 1, 3, true, false);
    assert_eq!(selected.corners.top_left, 8.0);
    assert_eq!(selected.container, theme.color.primary);
    assert_eq!(selected.label_style.name, "labelLargeEmphasized");
    let pressed = button_group::resolve_segment(&theme, 0, 3, true, true);
    assert_eq!(pressed.corners.top_left, 8.0);
    assert_eq!(button_group::SETTINGS_SCENE_TITLE, "Sound & notifications");
    assert_eq!(button_group::SETTINGS_VOLUME_TITLE, "Volume");
    assert_eq!(button_group::SETTINGS_QUIET_HOURS_TITLE, "Quiet hours");
    assert_eq!(button_group::SETTINGS_GROUP_GAP_DP, 24.0);
    assert_eq!(button_group::SETTINGS_PAD_DP, 16.0);
    assert_eq!(button_group::ICON_SEGMENTS.len(), 3);
    assert_eq!(button_group::icon_group_count(), 4);
    assert_eq!(button_group::overflow_index(), 3);
    assert_eq!(button_group::icon_glyph(3), button_group::OVERFLOW_GLYPH);
    let icon = button_group::resolve_icon_segment(&theme, 0, 4, true, false);
    assert_eq!(icon.min_width_dp, Some(button_group::ICON_MIN_W_DP));
    assert_eq!(button_group::OVERFLOW_ITEMS.len(), 3);
}

#[test]
fn search_bar_and_time_picker_tokens() {
    let theme = Theme::light();
    let search = search::resolve(&theme);
    assert_eq!(search.bar.height_dp, 56.0);
    assert_eq!(search.bar.corners.top_left, 28.0);
    assert_eq!(search.bar.container, theme.color.surface_container_high);
    assert_eq!(search::PLACEHOLDER, "Hinted search text");
    let view = search::resolve_view(&theme);
    assert_eq!(view.header_h_dp, 72.0);
    assert_eq!(search::SUGGESTIONS.len(), 4);
    assert!(search::VIEW_OPEN_BY_DEFAULT);
    assert_eq!(search::filter_suggestions("").len(), 4);
    assert_eq!(search::filter_suggestions("app"), vec!["App"]);
    assert_eq!(search::apply_search_key("", "a"), "a");
    assert_eq!(search::apply_search_key("ab", "backspace"), "a");
    assert_eq!(search::pick_suggestion("app", 0), Some("App"));
    assert_eq!(search::morph_corner_dp(true), 0.0);
    assert!((search::morph_height_dp(0.0) - search::HEIGHT_DP).abs() < 0.01);
    assert!((search::morph_height_dp(1.0) - search::ACTIVITY_MIN_H_DP).abs() < 0.01);
    assert!((search::morph_corner_dp_at(0.5) - 14.0).abs() < 0.01);
    let mut ed = text_field::TextFieldEditor::new(text_field::TextFieldVariant::Filled, "");
    search::apply_key_to_editor(&mut ed, "a");
    search::apply_key_to_editor(&mut ed, "p");
    search::apply_key_to_editor(&mut ed, "p");
    assert_eq!(ed.value(), "app");
    search::apply_key_to_editor(&mut ed, "enter");
    assert_eq!(ed.value(), "App");
    let (ix, iy) = text_field::ime_cursor_origin_dp(3, 16.0);
    assert!(ix > 16.0 && iy > 0.0);
    let rect = text_field::ime_caret_rect_dp(3, 16.0);
    assert_eq!(rect.2, text_field::IME_CARET_W_DP);
    assert_eq!(rect.3, text_field::IME_CARET_H_DP);
    assert_eq!(search::resolve_activity(&theme).corners.top_left, 0.0);
    let time = time_picker::resolve(&theme);
    assert_eq!(time.clock_dp, 256.0);
    assert_eq!(time.number_dp, 48.0);
    assert_eq!(time.time_style.name, "displaySmallEmphasized");
    assert_eq!(time.time_style.weight, 500);
    assert_eq!(time.container, theme.color.surface_container_high);
    assert_eq!(
        time_picker::header_label(6, 30, time_picker::DayPeriod::Pm),
        "6:30 PM"
    );
    assert_eq!(time_picker::select_hour(6, 9), 9);
    assert_eq!(time_picker::select_minute(30, 17), 15);
    assert_eq!(time_picker::DEMO_PERIOD.toggle(), time_picker::DayPeriod::Am);
    assert_eq!(time_picker::DEMO_DIAL, time_picker::DialFace::Minute);
    let (x, y) = time_picker::hour_offset(12, 256.0, 48.0);
    assert!(x > 80.0 && x < 130.0, "12 should sit near top center, x={x}");
    assert!(y < 20.0, "12 should sit near top, y={y}");
    let (mx, my) = time_picker::minute_offset(30, 256.0, 48.0);
    assert!(mx > 80.0 && mx < 130.0, "30 sits bottom-center-ish x={mx}");
    assert!(my > 180.0, "30 sits near bottom, y={my}");
    assert!((time_picker::hand_angle_deg(time_picker::DialFace::Minute, 6, 30) - 180.0).abs() < 0.01);
    let quad = time_picker::hand_quad(256.0, time_picker::DialFace::Minute, 6, 30, 48.0);
    assert_eq!(quad.len(), 4);
    assert!(time_picker::hand_svg_d(256.0, time_picker::DialFace::Minute, 6, 30, 48.0).starts_with('M'));
    let (s, _e) = slider::drag_thumb_snapped(0.2, 0.75, slider::RangeThumb::Start, 0.33);
    assert!((s - 0.35).abs() < 1e-5);
    assert!(slider::RANGE_SNAP_WHILE_DRAG);
    assert!((slider::snap_to_step(0.22) - 0.20).abs() < 1e-5);
    let (cs, ce) = slider::click_step(0.20, 0.75, 0.40);
    assert!((cs - 0.40).abs() < 1e-5);
    assert!((ce - 0.75).abs() < 1e-5);
    assert_eq!(slider::range_tick_fractions().len(), 21);
    assert!(slider::range_value_label(0.2, 0.75).contains("min span"));
    let (s, e, thumb) = slider::apply_arrow(0.2, 0.75, slider::RangeThumb::End, "left").unwrap();
    assert!((e - 0.70).abs() < 1e-5);
    assert_eq!(s, 0.2);
    assert_eq!(thumb, slider::RangeThumb::End);
    assert!((slider::fraction_from_local_x(70.0, 280.0) - 0.25).abs() < 1e-5);
    let rail = navigation_rail::resolve(&theme);
    assert_eq!(rail.width_dp, 80.0);
    assert_eq!(navigation_rail::INDICATOR_W_DP, 56.0);
    assert_eq!(navigation_rail::DESTINATIONS.len(), 3);
    assert_eq!(navigation_rail::EXPANDED_WIDTH_DP, 220.0);
    assert_eq!(
        navigation_rail::resolve_mode(&theme, navigation_rail::RailMode::Expanded).width_dp,
        220.0
    );
    assert!(navigation_rail::is_modal(navigation_rail::RailMode::Expanded));
    assert!((search::morph_list_opacity(1.0) - 1.0).abs() < 1e-5);
    assert!((search::morph_back_opacity(1.0) - 1.0).abs() < 1e-5);
    assert!(search::morph_avatar_opacity(1.0).abs() < 1e-5);
    let grown = search::morph_frame_eased(1.0);
    assert!((grown.height_dp - search::ACTIVITY_MIN_H_DP).abs() < 0.01);
    assert!(grown.inset_h_dp.abs() < 0.01);
    assert!((grown.scale - 1.0).abs() < 0.01);
    let docked = search::morph_frame_at(0.0);
    assert!((docked.scale - search::SHARED_SCALE_DOCKED).abs() < 0.01);
    let docked_m = search::morph_scaled_margin_dp(docked, search::MORPH_STAGE_W_DP);
    assert!(docked_m > docked.inset_h_dp);
    assert!(search::morph_scaled_margin_dp(grown, search::MORPH_STAGE_W_DP).abs() < 0.01);
    let layer = search::morph_layer_transform(docked);
    assert!((layer.scale - search::SHARED_SCALE_DOCKED).abs() < 0.01);
    assert_eq!(layer.origin_x_frac, 0.5);
    assert_eq!(search::TRANSFORM_ORIGIN, "top center");
    assert!(search::morph_layer_css(docked).contains("0.94"));
    let [pre, post] = search::top_center_scale_translates(0.0, 0.0, 100.0);
    assert!((pre.0 + 50.0).abs() < 1e-5);
    assert_eq!(post.0, 50.0);
    let layer_box = search::morph_layer_box(
        search::MORPH_STAGE_W_DP,
        docked.height_dp,
        layer,
    );
    assert!((layer_box.height_dp - docked.height_dp * search::SHARED_SCALE_DOCKED).abs() < 0.02);
    assert!(layer_box.x_dp > 0.0);
    assert!((search::morph_layer_height_dp(grown) - grown.height_dp).abs() < 0.02);
    let (mx, my) = search::morph_layer_map_point(0.0, 10.0, 640.0, 56.0, layer);
    assert!(mx > 0.0);
    assert!((my - 10.0 * search::SHARED_SCALE_DOCKED).abs() < 0.05);
    assert!((search::morph_path_scale(docked) - search::SHARED_SCALE_DOCKED).abs() < 0.01);
    assert!((search::morph_path_scale_eased(1.0) - 1.0).abs() < 0.01);
    assert!(search::morph_path_scale_attr(grown).contains("1"));
    assert_eq!(progress::STROKE_CAP, progress::StrokeCap::Round);
    assert!(progress::STROKE_CAP.is_round());
    assert_eq!(progress::STROKE_CAP.css(), progress::LINE_CAP);
    assert!((grown.leading_activity_opacity - 1.0).abs() < 1e-5);
    assert!(grown.leading_docked_opacity.abs() < 1e-5);
    assert_eq!(search::morph_container(&theme, 1.0), theme.color.surface);
    assert!(navigation_rail::focus_trapped(navigation_rail::RailMode::Expanded));
    assert!(navigation_rail::dismiss_on_scrim());
    assert_eq!(
        navigation_rail::modal_elevation_dp(&theme),
        theme.elevation.level2
    );
    let win = text_field::ime_caret_rect_in_window(8.0, 16.0, 3, 16.0);
    assert_eq!(win.2, text_field::IME_CARET_W_DP);
    let mut focused_ed = text_field::TextFieldEditor::new(
        text_field::TextFieldVariant::Outlined,
        "ab",
    );
    focused_ed.set_focus(true);
    let cat = text_field::catalog_ime_from_focused(&focused_ed, 16.0).expect("focused caret");
    assert_eq!(cat.2, text_field::IME_CARET_W_DP);
    assert!(cat.0 > text_field::CATALOG_FIELD_ORIGIN_DP.0);
    let idle = text_field::TextFieldEditor::new(text_field::TextFieldVariant::Filled, "");
    assert!(text_field::catalog_ime_from_focused(&idle, 16.0).is_none());
    assert_eq!(slider::range_tick_count(), 21);
    let ticks = slider::range_ticks(0.20, 0.75, 280.0, 4.0);
    assert_eq!(ticks.len(), 21);
    let mut fling = carousel::FlingState::new(0);
    fling.impulse(80.0, 0.0);
    let _ = fling.step(0.25);
    assert!(!fling.resting() || fling.selected != 0);
    assert_eq!(navigation_rail::DESTINATION_BADGES[1], Some(3));
    let car = carousel::resolve(&theme);
    assert_eq!(car.large_w_dp, 256.0);
    assert_eq!(car.small_w_dp, 120.0);
    assert_eq!(car.corners.top_left, 28.0);
    assert_eq!(carousel::item_width_dp(0, 0), 256.0);
    assert_eq!(carousel::item_width_dp(1, 0), 120.0);
    assert_eq!(
        carousel::item_width_for(carousel::CarouselLayout::MultiBrowse, 0, 0),
        carousel::MULTI_LARGE_W_DP
    );
    assert_eq!(
        carousel::item_width_for(carousel::CarouselLayout::MultiBrowse, 1, 0),
        carousel::MULTI_SMALL_W_DP
    );
    assert_eq!(
        carousel::item_width_for(carousel::CarouselLayout::Uncontained, 0, 0),
        carousel::UNCONTAINED_W_DP
    );
    assert_eq!(
        carousel::item_width_for(carousel::CarouselLayout::CenteredHero, 0, 0),
        carousel::CENTERED_LARGE_W_DP
    );
    assert_eq!(
        carousel::item_width_for(carousel::CarouselLayout::CenteredHero, 1, 0),
        carousel::CENTERED_SMALL_W_DP
    );
    assert_eq!(
        carousel::item_width_for(carousel::CarouselLayout::FullScreen, 0, 0),
        carousel::FULLSCREEN_W_DP
    );
    assert_eq!(
        carousel::item_height_for(carousel::CarouselLayout::FullScreen),
        carousel::FULLSCREEN_H_DP
    );
    assert!(carousel::CarouselLayout::CenteredHero.center_aligned());
    assert!(carousel::CarouselLayout::CenteredHero.uses_phone_frame());
    assert_eq!(
        carousel::CarouselLayout::FullScreen.axis(),
        carousel::CarouselAxis::Vertical
    );
    assert_eq!(
        carousel::CarouselLayout::Hero.next(),
        carousel::CarouselLayout::MultiBrowse
    );
    assert_eq!(
        carousel::CarouselLayout::FullScreen.next(),
        carousel::CarouselLayout::Hero
    );
    assert_eq!(carousel::CarouselLayout::ALL.len(), 6);
    assert_eq!(
        carousel::CarouselLayout::Uncontained.next(),
        carousel::CarouselLayout::UncontainedMulti
    );
    assert_eq!(
        carousel::media_fill(&theme, 0),
        gpui_material::components::photo_stub::PhotoKind::Lake.fill()
    );
    assert_eq!(
        carousel::item_height_for_index(carousel::CarouselLayout::UncontainedMulti, 0),
        200.0
    );
    assert_eq!(
        carousel::item_height_for_index(carousel::CarouselLayout::UncontainedMulti, 1),
        112.0
    );
    assert_eq!(carousel::parallax_offset_dp(0.5), 6.0);
    assert_eq!(carousel::LISTS_TITLE, "Your lists");
    assert!(carousel::CarouselLayout::UncontainedMulti.uses_lists_scene());
    assert!(!carousel::CarouselLayout::Hero.uses_lists_scene());
    assert_eq!(carousel::advance(0, 1), 1);
    assert_eq!(carousel::advance(0, -1), 3);
    assert_eq!(carousel::fling_step(12.0, 0.0), 1);
    assert_eq!(carousel::fling_steps(80.0, 0.0), 3);
    assert_eq!(carousel::inertial_steps(48.0, 0.0), 1);
    assert_eq!(carousel::inertial_steps(96.0, 0.0), 2);
    let (v, _r, _s) = carousel::integrate_fling(8.0, 0.0, 0.25);
    assert!(v < 8.0);
    assert!(carousel::decay_velocity(10.0, 0.25) < 10.0);
    let paint = slider::range_paint(0.20, 0.75, 280.0, 4.0);
    assert!(paint.left < paint.end_handle);
    assert!((paint.left + paint.handle_w + paint.active + paint.handle_w + paint.right - 280.0).abs() < 1.0);
    let pts = progress::ptr_arc_polyline(40.0, 4.0, 90.0, 0.25);
    assert!(pts.len() > 4);
    let caps = progress::ptr_cap_centers(40.0, 4.0, 90.0, 0.25);
    assert_eq!(caps.len(), 2);
    assert_eq!(progress::clock_ms(&theme), theme.motion.effects_default_ms * 6);
    assert_eq!(progress::LOADING_LABEL, "Loading");
    let morph = progress::loading_polygon(progress::LOADING_SIZE_DP, 0.3);
    assert_eq!(morph.len(), progress::LOADING_SAMPLES);
    let det0 = progress::loading_polygon_for_progress(progress::LOADING_SIZE_DP, 0.0);
    let det1 = progress::loading_polygon_for_progress(progress::LOADING_SIZE_DP, 1.0);
    assert_eq!(det0.len(), progress::LOADING_SAMPLES);
    assert_ne!(det0, det1);
    assert!((progress::loading_phase_for_progress(1.0) - 6.0 / 7.0).abs() < 1e-5);
    assert!(progress::loading_svg_d_for_progress(38.0, 0.65).starts_with('M'));
    assert!((time_picker::second_hand_angle_deg(0, 0.0) - 0.0).abs() < 0.01);
    assert!((time_picker::second_hand_angle_deg(30, 0.0) - 180.0).abs() < 0.01);
    let squad = time_picker::second_hand_quad(256.0, 90.0, 48.0);
    assert_eq!(squad.len(), 4);
    assert!(time_picker::second_hand_svg_d(256.0, 0.0, 48.0).starts_with('M'));
    let mut physics = carousel::FlingState::new(0);
    physics.impulse_items(2);
    let landed = physics.step_until_rest(1.0 / 60.0, 180);
    assert_eq!(landed, 2);
    assert_eq!(carousel::apply_wheel(0, 96.0, 0.0), carousel::advance(0, carousel::inertial_steps(96.0, 0.0)));
    assert!((carousel::item_width_during_fling(0, 0, 0.0) - carousel::LARGE_W_DP).abs() < 0.01);
    assert!(carousel::item_width_during_fling(0, 0, 0.5) < carousel::LARGE_W_DP);
    assert!(carousel::item_width_during_fling(1, 0, 0.5) > carousel::SMALL_W_DP);
    let mut snap = carousel::FlingState::new(0);
    snap.leftover = carousel::FLING_UNIT * 0.6;
    snap.settle();
    assert_eq!(snap.selected, 1);
    assert!(snap.resting());
    assert_eq!(carousel::FLING_FRAME_DT, gpui_material::motion::FRAME_DT);
    assert_eq!(time_picker::SECOND_HAND_FRAME_MS, gpui_material::motion::FRAME_MS);
    assert!(progress::loading_svg_d(38.0, 0.0).starts_with('M'));
    assert!(progress::loading_svg_values(38.0, 4).contains(';'));
    let sausage = progress::round_capped_arc_polygon(48.0, 4.0, -90.0, 90.0);
    assert!(sausage.len() > 20);
    assert_eq!(progress::contained_loading_indicator(&theme).contained, true);
    assert_eq!(
        progress::contained_loading_indicator(&theme).container,
        theme.color.primary_container
    );
    assert_eq!(
        progress::contained_loading_indicator(&theme).indicator,
        theme.color.on_primary_container
    );
    assert_eq!(
        progress::loading_indicator(&theme).indicator,
        theme.color.primary
    );
    assert!((navigation_rail::morph_width_dp(0.0) - 80.0).abs() < 0.01);
    assert!((navigation_rail::morph_width_dp(1.0) - 220.0).abs() < 0.01);
    assert!((time_picker::hour_face_live_angle_deg(6, 30, 0.0) - 195.0).abs() < 0.01);
    assert_eq!(navigation_rail::select_destination(0, 2), 2);
    assert_eq!(
        navigation_rail::toggle_mode(navigation_rail::RailMode::Expanded),
        navigation_rail::RailMode::Collapsed
    );
    let frame = text_field::notch_frame(
        "Email",
        &text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Outlined,
            InteractionState::Focused,
            true,
        ),
    );
    let hole = frame.hole_rect(280.0);
    assert!(hole.2 > 200.0);
    let center = frame.centerline_polyline(280.0);
    assert!(center.len() > 8);
    assert!(frame.centerline_svg_d(280.0).starts_with('M'));
    assert!(!frame.centerline_svg_d(280.0).contains('Z'));
    assert!(frame.evenodd_svg_d(280.0).contains('Z'));
    assert_eq!(frame.evenodd_subpath_count(280.0), 1);
    assert!(frame.evenodd_svg_d(280.0).contains('C'));
    let poly = frame.evenodd_polygon(280.0);
    assert!(poly.len() > 16);
    assert!(poly.iter().any(|(x, _)| *x < 10.0));
    let notch_l = frame.start_dp;
    let notch_r = frame.start_dp + frame.width_dp;
    assert!(poly.iter().any(|(x, _)| (*x - notch_l).abs() < 2.0));
    assert!(poly.iter().any(|(x, _)| (*x - notch_r).abs() < 2.0));
    let gap_mid = frame.start_dp + frame.width_dp * 0.5;
    assert!(
        !poly.iter().any(|(x, y)| (*x - gap_mid).abs() < 6.0 && *y < 0.4),
        "C-path must leave the legend gap open on the top edge"
    );
    let [c1, _, to] = gpui_material::shape::rounded_polygon_quarter(
        (0.0, 0.0),
        (4.0, 0.0),
        (4.0, 4.0),
        gpui_material::shape::CIRCULAR_KAPPA,
    );
    assert!(c1.0 > 0.0);
    assert!((to.0 - 4.0).abs() < 1e-5);
    let wait = progress::WaitProgress::bytes(650_000, 1_000_000);
    assert!((wait.fraction() - 0.65).abs() < 1e-5);
    assert_eq!(progress::DEMO_WAIT.fraction(), wait.fraction());
    assert_eq!(progress::LINE_CAP, "round");
    assert!(navigation_rail::overlay_window(navigation_rail::RailMode::Expanded));
    assert_eq!(
        navigation_rail::rail_chrome(navigation_rail::RailMode::Expanded),
        navigation_rail::RailChrome::Popup
    );
    assert_eq!(
        navigation_rail::rail_chrome_attr(navigation_rail::RailMode::Expanded),
        "popup"
    );
    assert_eq!(navigation_rail::POPUP_WINDOW_KIND, "popup");
    assert_eq!(navigation_rail::GPUI_WINDOW_KIND, "PopUp");
    assert!(!navigation_rail::OS_POPUP_OPENED);
    let popup = navigation_rail::os_popup_spec(navigation_rail::RailMode::Expanded);
    assert_eq!(popup.kind, "popup");
    assert_eq!(popup.gpui_kind, "PopUp");
    assert_eq!(popup.height_dp, navigation_rail::OS_POPUP_HEIGHT_DP);
    assert_eq!(popup.title, navigation_rail::OS_POPUP_TITLE);
    assert!(popup.focus);
    assert!(!popup.movable);
    assert!(!popup.supported_on_android);
    assert_eq!(navigation_rail::os_popup_attr(), "0");
    let (kind, width, android) =
        navigation_rail::os_popup_window_options(navigation_rail::RailMode::Expanded);
    assert_eq!(kind, "PopUp");
    assert_eq!(width, 220.0);
    assert!(!android);
    assert_eq!(
        navigation_rail::overlay_window_attr(navigation_rail::RailMode::Collapsed),
        "0"
    );
    let (sec, tick) = time_picker::wall_second();
    assert!(sec <= 59);
    assert!(tick >= 0.0 && tick < 1.0);
    let _ = time_picker::second_hand_angle_wall_clock();
    assert_eq!(time_picker::SECOND_HAND_FRAME_MS, 16);
    let mut live = carousel::FlingState::new(0);
    live.impulse(80.0, 0.0);
    assert!(live.needs_frame());
    let _ = live.step_live(1.0 / 60.0);
    assert!(progress::loading_svg_values_for_wait(38.0, 4).contains(';'));
    let lerped = time_picker::lerp_angle_deg(180.0, 0.0, 0.5);
    assert!(lerped.abs() > 80.0);
}

#[test]
fn catalog_jpeg_decodes_for_scene_photos() {
    use gpui_material::components::photo_stub::PhotoKind;
    let jpeg = PhotoKind::Basket.jpeg_bytes();
    assert!(jpeg.len() > 32);
    assert_eq!(&jpeg[0..2], &[0xFF, 0xD8]);
    let (w, h, rgb) = PhotoKind::Basket.decode_rgb();
    assert!(w >= 64 && h >= 64);
    assert_eq!(rgb.len(), (w * h * 3) as usize);
    let uri = PhotoKind::PortraitSofia.data_uri();
    assert!(uri.starts_with("data:image/jpeg;base64,"));
    let mosaic = PhotoKind::Bloom.mosaic(8, 6);
    assert_eq!(mosaic.len(), 48);
    assert!(PhotoKind::Mugs.css_background().contains("url('data:image/jpeg"));
}
