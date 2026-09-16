//! Golden tests against androidx Material 3 token values (v0_210 palette / type scale).

use gpui_material::components::{
    badge, bottom_sheet, button, button_group, card, carousel, checkbox, chip, date_picker, dialog,
    divider, fab, fab_menu, icon_button, list, menu, navigation_bar, navigation_rail, progress,
    radio, search, side_sheet, slider, snackbar, split_button, switch, tabs, text_field,
    time_picker, toolbar, tooltip, top_app_bar,
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
    assert!(text_field::notch_width_dp("WWW", 12.0) > text_field::notch_width_dp("iii", 12.0));
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
    assert_eq!(
        fallback.width_dp,
        text_field::notch_frame("Email", &focused).width_dp
    );
    assert!(text_field::roboto_advance_em('W') > text_field::roboto_advance_em('i'));

    let error = text_field::resolve(
        &theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Error,
        true,
    );
    assert_eq!(error.field.outline.unwrap().0, theme.color.error);
    assert_eq!(error.supporting, theme.color.error);

    let tonal_empty = text_field::resolve_expressive(
        &theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        false,
    );
    assert_eq!(tonal_empty.style, text_field::TextFieldStyle::Expressive);
    assert_eq!(
        tonal_empty.field.corners.top_left,
        text_field::ROUNDED_SHAPE_DP
    );
    assert_eq!(
        tonal_empty.field.corners.bottom_left,
        text_field::ROUNDED_SHAPE_DP
    );
    assert_eq!(text_field::ROUNDED_SHAPE_TOKEN, "CornerMedium");
    assert_eq!(tonal_empty.field.container, theme.color.surface_container);
    assert!(tonal_empty.field.outline.is_none());
    assert!(!tonal_empty.notched);
    assert_eq!(
        tonal_empty.label_position,
        text_field::LabelPosition::Inside
    );
    assert_eq!(
        tonal_empty.field.min_width_dp,
        Some(text_field::TONAL_MIN_WIDTH_DP)
    );

    let tonal_outlined = text_field::resolve_expressive(
        &theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    assert!(tonal_outlined.floating);
    assert!(!tonal_outlined.notched);
    assert_eq!(tonal_outlined.field.corners.top_left, 12.0);
    assert_eq!(tonal_outlined.field.container, theme.color.on_primary);
    assert_eq!(
        tonal_outlined.field.outline,
        Some((theme.color.outline_variant, 1.0))
    );
    assert_eq!(tonal_outlined.label, theme.color.on_surface_variant);
    assert_eq!(tonal_outlined.input, theme.color.on_background);
    assert_eq!(
        tonal_outlined.label_position,
        text_field::LabelPosition::Inside
    );

    let tonal_error = text_field::resolve_expressive(
        &theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Error,
        true,
    );
    assert_eq!(tonal_error.field.container, theme.color.error_container);
    assert_eq!(tonal_error.label, theme.color.error);
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
    assert_eq!(
        list::resolve(&theme, list::ListLines::One, InteractionState::Enabled)
            .corners
            .top_left,
        0.0
    );
    assert_eq!(list::SEGMENTED_GAP_DP, 2.0);
    assert_eq!(list::INNER_CORNER_DP, 4.0);
    assert_eq!(list::OUTER_CORNER_DP, 16.0);
    assert_eq!(list::LEADING_ICON_DP, 20.0);
    let first = list::resolve_segmented(
        &theme,
        list::ListLines::Two,
        0,
        3,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(first.corners.top_left, 16.0);
    assert_eq!(first.corners.bottom_left, 4.0);
    let mid = list::resolve_segmented(
        &theme,
        list::ListLines::Two,
        1,
        3,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(mid.corners.top_left, 4.0);
    let last = list::resolve_segmented(
        &theme,
        list::ListLines::Two,
        2,
        3,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(last.corners.bottom_left, 16.0);
    let sel = list::resolve_scene(&theme, 0, 0);
    assert_eq!(sel.container, theme.color.secondary_container);
    assert_eq!(sel.content, theme.color.on_secondary_container);
    assert_eq!(sel.corners.top_left, 16.0);
    assert_eq!(sel.height_dp, 72.0);
    assert_eq!(list::SCENE_HEADLINES[0], "Wi-Fi");
    let press = list::segmented_corners(1, 3, false, true);
    assert_eq!(press.top_left, 16.0);
    assert_eq!(list::SWIPE_REVEAL_DP, 80.0);
    assert_eq!(list::SWIPE_THRESHOLD_DP, 56.0);
    assert_eq!(list::SWIPE_OVERSHOOT_DP, 16.0);
    assert_eq!(list::SWIPE_PRIMARY_ACTION_DP, 360.0);
    assert_eq!(list::SWIPE_PRIMARY_THRESHOLD_DP, 180.0);
    assert_eq!(list::SWIPE_FLING_DECAY, 2.0);
    assert_eq!(list::SWIPE_FRAME_DT, gpui_material::motion::FRAME_DT);
    assert_eq!(list::SWIPE_HEADLINES[0], "Team sync notes");
    assert_eq!(list::DRAG_HANDLE_DP, 24.0);
    assert_eq!(list::leading_rail_width_dp(80.0), 80.0);
    assert_eq!(list::leading_rail_width_dp(200.0), 200.0);
    assert_eq!(list::trailing_rail_width_dp(-200.0), 200.0);
    let mut swipe = list::ListSwipeState::settled();
    swipe.swipe(80.0);
    assert!(swipe.leading_revealed());
    swipe.settle();
    assert_eq!(swipe.offset_x_dp, 80.0);
    assert_eq!(swipe.phase, list::SwipePhase::Open);
    let mut back = list::ListSwipeState::settled();
    back.swipe(-80.0);
    assert!(back.trailing_revealed());
    let mut fling = list::ListSwipeState::settled();
    fling.impulse(960.0);
    fling.step_until_rest(list::SWIPE_FRAME_DT, 180);
    assert!(fling.primary_action());
    assert_eq!(fling.phase, list::SwipePhase::SwipePrimaryAction);
    assert_eq!(fling.offset_x_dp, 360.0);
    let mut open_fling = list::ListSwipeState::settled();
    open_fling.impulse(160.0);
    open_fling.step_until_rest(list::SWIPE_FRAME_DT, 180);
    assert!(open_fling.leading_revealed());
    assert!(!open_fling.primary_action());
    assert_eq!(open_fling.phase, list::SwipePhase::Open);
    let mut order = list::REORDER_DEMO;
    list::move_item(&mut order, 0, 2);
    assert_eq!(order, [1, 2, 0]);
    list::move_item(&mut order, 2, 0);
    assert_eq!(order, [0, 1, 2]);
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
    assert_eq!(filter.corners.top_left, chip::SELECTED_CORNER_DP);
    assert_eq!(filter.pad_start_dp, chip::LEADING_PAD_START_DP);

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
    assert_eq!(icon.width_dp, Some(40.0));
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
    let large = top_app_bar::resolve_scene(&theme, 0.0);
    assert_eq!(large.variant, top_app_bar::AppBarVariant::LargeFlexible);
    assert_eq!(large.expanded_height_dp, 152.0);
    assert_eq!(large.height_dp, 152.0);
    assert_eq!(large.title_style.name, "displaySmall");
    assert_eq!(large.subtitle_style.name, "titleMedium");
    assert_eq!(large.elevation_dp, 0.0);
    let collapsed = top_app_bar::resolve_scene(&theme, 1.0);
    assert_eq!(collapsed.height_dp, 64.0);
    assert_eq!(collapsed.container, theme.color.surface_container);
    assert_eq!(collapsed.elevation_dp, 3.0);
    assert_eq!(collapsed.title_style.name, "titleLarge");
    let mid = top_app_bar::resolve_scene(&theme, 0.5);
    assert!((mid.height_dp - 108.0).abs() < 0.01);
    let medium = top_app_bar::resolve_medium_scene(&theme, 0.0);
    assert_eq!(medium.expanded_height_dp, 136.0);
    assert_eq!(medium.title_style.name, "headlineMedium");
    let search_bar = top_app_bar::resolve_search(&theme);
    assert_eq!(search_bar.height_dp, 64.0);
    assert_eq!(search_bar.search_field_h_dp, 56.0);
    assert_eq!(top_app_bar::next_collapse(0.0), 0.5);
    assert_eq!(top_app_bar::next_collapse(0.5), 1.0);
    assert_eq!(top_app_bar::next_collapse(1.0), 0.0);
    let sheet = side_sheet::resolve_scene(&theme);
    assert_eq!(sheet.width_dp, 256.0);
    assert_eq!(sheet.container, theme.color.surface_container_low);
    assert_eq!(sheet.elevation_dp, 1.0);
    assert_eq!(sheet.corners.top_left, 16.0);
    assert_eq!(sheet.corners.top_right, 0.0);
    assert_eq!(side_sheet::FILTERS[0].0, "Date");
    let std_sheet = side_sheet::resolve(&theme, side_sheet::SideSheetVariant::Standard);
    assert_eq!(std_sheet.container, theme.color.surface);
    assert_eq!(std_sheet.elevation_dp, 0.0);
    let det = side_sheet::resolve(&theme, side_sheet::SideSheetVariant::Detached);
    assert_eq!(det.margin_dp, 16.0);
    assert_eq!(det.corners.top_left, 16.0);

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
    assert_eq!(nav.height_dp, 64.0);
    assert_eq!(nav.layout, navigation_bar::NavBarLayout::Vertical);
    assert_eq!(nav.indicator_w_dp, 56.0);
    assert_eq!(nav.indicator_h_dp, 32.0);
    assert_eq!(nav.container, theme.color.surface_container);
    assert_eq!(nav.active_indicator, theme.color.secondary_container);
    assert_eq!(nav.active_label, theme.color.secondary);
    assert_eq!(nav.elevation_dp, 3.0);
    assert_eq!(navigation_bar::TALL_HEIGHT_DP, 80.0);
    let nav_h = navigation_bar::resolve_horizontal(&theme);
    assert_eq!(nav_h.layout, navigation_bar::NavBarLayout::Horizontal);
    assert_eq!(nav_h.indicator_h_dp, 40.0);
    assert_eq!(nav_h.indicator_pad_h_dp, 16.0);
    assert_eq!(nav_h.active_label, theme.color.on_secondary_container);
    assert_eq!(navigation_bar::MEDIUM_DESTS.len(), 4);
    assert!(navigation_bar::is_flexible_height(nav.height_dp));

    let plain = tooltip::resolve_plain(&theme);
    assert_eq!(plain.min_height_dp, 24.0);
    assert_eq!(plain.max_width_dp, 200.0);
    assert_eq!(plain.container, theme.color.inverse_surface);
    assert_eq!(plain.supporting, theme.color.inverse_on_surface);
    assert_eq!(plain.supporting_style.name, "bodySmall");
    assert_eq!(plain.corners.top_left, 4.0);
    let rich = tooltip::resolve_rich(&theme);
    assert_eq!(rich.max_width_dp, 320.0);
    assert_eq!(rich.pad_top_dp, 12.0);
    assert_eq!(rich.pad_bottom_dp, 8.0);
    assert_eq!(rich.pad_start_dp, 16.0);
    assert_eq!(rich.container, theme.color.surface_container);
    assert_eq!(rich.supporting, theme.color.on_surface_variant);
    assert_eq!(rich.subhead, Some(theme.color.on_surface_variant));
    assert_eq!(rich.action, Some(theme.color.primary));
    assert_eq!(rich.elevation_dp, 3.0);
    assert_eq!(rich.corners.top_left, 12.0);
    assert_eq!(rich.subhead_style.unwrap().name, "titleSmall");
    assert_eq!(rich.action_style.unwrap().name, "labelLarge");
    assert_eq!(tooltip::PLAIN_TEXT, "Add to library");
    assert_eq!(tooltip::RICH_ACTION_PRIMARY, "Learn more");
    assert!(tooltip::has_actions(tooltip::TooltipKind::Rich));
    assert!(!tooltip::has_actions(tooltip::TooltipKind::Plain));
    assert_eq!(tooltip::CARET_W_DP, 16.0);
    assert_eq!(tooltip::CARET_H_DP, 8.0);
    assert_eq!(tooltip::caret_down_points()[2], (8.0, 8.0));
    assert_eq!(tooltip::caret_up_points()[2], (8.0, 0.0));
    assert_eq!(tooltip::LONG_PRESS_MS, 500);
    assert_eq!(tooltip::HOVER_TRIGGER, "hover");

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
    assert!(html.contains("data-navbar=\"horizontal\""));
    assert!(html.contains("data-layout=\"vertical\""));
    assert!(html.contains("data-layout=\"horizontal\""));
    assert!(html.contains("data-hero=\"nav-bar\""));
    assert!(html.contains("data-hero=\"nav-bar-horizontal\""));
    assert!(html.contains("data-nav-flexible=\"1\""));
    assert!(html.contains("data-nav-height=\"64\""));
    assert!(html.contains("data-hero=\"tooltip\""));
    assert!(html.contains("data-tooltip=\"plain\""));
    assert!(html.contains("data-tooltip=\"rich\""));
    assert!(html.contains("data-tooltip-text=\"Add to library\""));
    assert!(html.contains("data-tooltip-subhead=\"Rich tooltip\""));
    assert!(html.contains("data-tooltip-action=\"learn\""));
    assert!(html.contains("data-tooltip-caret=\"plain\""));
    assert!(html.contains("data-tooltip-caret=\"rich\""));
    assert!(html.contains("data-hero=\"list\""));
    assert!(html.contains("data-list-style=\"segmented\""));
    assert!(html.contains("data-list-item=\"wifi\""));
    assert!(html.contains("data-list-item=\"bluetooth\""));
    assert!(html.contains("data-list-gap=\"2\""));
    assert!(html.contains("Wi-Fi"));
    assert!(html.contains("Airplane mode"));
    assert!(html.contains("data-hero=\"list-swipe\""));
    assert!(html.contains("data-list-swipe=\"1\""));
    assert!(html.contains("data-swipe-fling=\"1\""));
    assert!(html.contains("data-swipe-overshoot=\"16\""));
    assert!(html.contains("data-swipe-primary-dp=\"360\""));
    assert!(html.contains("data-swipe-state=\"open\""));
    assert!(html.contains("data-swipe-action=\"archive\""));
    assert!(html.contains("data-swipe-action=\"delete\""));
    assert!(html.contains("Team sync notes"));
    assert!(html.contains("data-hero=\"list-reorder\""));
    assert!(html.contains("data-list-reorder=\"1\""));
    assert!(html.contains("data-list-handle"));
    assert!(html.contains("Morning briefing"));
    assert!(html.contains("data-hero=\"button-group-standard\""));
    assert!(html.contains("data-button-group=\"standard\""));
    assert!(html.contains("data-expanded-ratio=\"0.15\""));
    assert!(html.contains("data-standard-i"));
    assert!(html.contains(">Start<"));
    assert!(html.contains(">Center<"));
    assert!(html.contains("data-standard-overflow=\"1\""));
    assert!(html.contains("data-standard-overflow-btn=\"1\""));
    assert!(html.contains("data-standard-overflow-item=\"Left\""));
    assert!(html.contains("data-standard-overflow-item=\"Justify\""));
    assert!(html.contains("data-overflow-cascade"));
    assert!(html.contains("data-popup-kind=\"standard-overflow\""));
    assert!(html.contains("data-popup-kind=\"connected-overflow\""));
    assert!(html.contains("data-popup-kind=\"split\""));
    assert!(html.contains("data-hover-delay=\"200\""));
    assert!(html.contains("GPUI hosts"));
    assert!(html.contains("data-grouped=\"1\""));
    assert!(html.contains("data-licensed-camera=\"1\""));
    assert!(html.contains("data-photo-license=\"CC0\""));
    assert!(html.contains("data-photo-license=\"Unsplash License\""));
    assert!(html.contains("data-tooltip-trigger=\"hover\""));
    assert!(html.contains("data-tooltip-longpress-ms=\"500\""));
    assert!(html.contains("Learn more"));
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
    assert!(html.contains("data-appbar=\"large-flexible\""));
    assert!(html.contains("data-appbar=\"medium-flexible\""));
    assert!(html.contains("data-appbar=\"search\""));
    assert!(html.contains("data-appbar-scene"));
    assert!(html.contains("data-hero=\"app-bar\""));
    assert!(html.contains("data-appbar-photo"));
    assert!(html.contains("April 12 – 16") || html.contains("April 12"));
    assert!(html.contains("12 albums"));
    assert!(html.contains("data-side-sheet=\"modal\""));
    assert!(html.contains("data-side-sheet=\"standard\""));
    assert!(html.contains("data-side-sheet=\"detached\""));
    assert!(html.contains("data-side-filters"));
    assert!(html.contains("data-side-filter=\"Date\""));
    assert!(html.contains("data-hero=\"side-sheet\""));
    assert!(html.contains("Filters"));
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
    assert!(html.contains(r#"data-date-range-live="1""#));
    assert!(html.contains(r#"data-range-grid="1""#));
    assert!(html.contains(r#"data-range-headline="1""#));
    assert!(html.contains(r#"data-range-month-nav="1""#));
    assert!(html.contains(r#"data-range-month-delta="-1""#));
    assert!(html.contains(r#"data-range-month-label="1""#));
    assert!(html.contains(r#"data-range-year-toggle="1""#));
    assert!(html.contains(r#"data-range-years="1""#));
    assert!(html.contains(r#"data-range-display-live="1""#));
    assert!(html.contains(r#"data-range-display-toggle="1""#));
    assert!(html.contains(r#"data-range-live-fields="1""#));
    assert!(html.contains(r#"data-range-live-start="1""#));
    assert!(html.contains(r#"data-range-divider="1""#));
    assert!(html.contains(r#"data-range-actions="1""#));
    assert!(html.contains(r#"data-range-cancel="1""#));
    assert!(html.contains(r#"data-range-ok="1""#));
    assert!(html.contains(r#"data-range-connector="1""#));
    assert!(html.contains(r#"data-range-vertical-months="1""#));
    assert!(html.contains(r#"data-range-months="1""#));
    assert!(html.contains(r#"data-range-subhead="1""#));
    assert!(html.contains("October 2026"));
    assert!(html.contains(r#"data-range-fill="start-half""#));
    assert!(html.contains(r#"data-range-fill="end-half""#));
    assert!(html.contains(r#"data-range-fill="full""#));
    assert!(html.contains(r#"data-date-actions-live="1""#));
    assert!(html.contains(r#"data-date-grid="1""#));
    assert!(html.contains(r#"data-date-headline="1""#));
    assert!(html.contains(r#"data-date-header-paddings="1""#));
    assert!(html.contains(r#"data-date-title-pad="1""#));
    assert!(html.contains(r#"data-date-headline-pad="1""#));
    assert!(html.contains(&format!("padding: {}", date_picker::title_padding_css())));
    assert!(html.contains(&format!("padding: {}", date_picker::headline_padding_css())));
    assert!(html.contains(r#"data-date-actions-divider="1""#));
    assert!(html.contains(r#"data-date-actions="1""#));
    assert!(html.contains(r#"data-date-cancel="1""#));
    assert!(html.contains(r#"data-date-ok="1""#));
    assert!(html.contains(r#"data-date-commit-year="2026""#));
    assert!(html.contains(r#"data-date-month-nav="1""#));
    assert!(html.contains(r#"data-date-month-delta="-1""#));
    assert!(html.contains(r#"data-date-month="1""#));
    assert!(html.contains(r#"data-hero="datepicker-range""#));
    assert!(html.contains(r#"data-hero="datepicker-range-picker-empty""#));
    assert!(html.contains(r#"data-range-picker-empty="1""#));
    assert!(html.contains("Select dates"));
    assert!(html.contains(r#"data-date-pane="calendar""#));
    assert!(html.contains("Depart – Return dates"));
    assert!(html.contains("data-handle-visual=\"28\""));
    assert!(html.contains("September 2026 ▾"));
    assert!(html.contains("headlineSmallEmphasized"));
    assert!(html.contains("headlineLargeEmphasized"));
    assert!(html.contains("data-button-group=\"connected\""));
    assert!(html.contains("data-slider-range=\"1\""));
    assert!(html.contains("data-datepicker-docked=\"1\""));
    assert!(html.contains(r#"data-selectable-dates="1""#));
    assert!(html.contains(r#"data-kind="Disabled""#));
    assert!(html.contains(r#"data-docked-pane="calendar""#));
    assert!(html.contains(r#"data-docked-select-live="1""#));
    assert!(html.contains(r#"data-docked-dismiss-select="1""#));
    assert!(html.contains(r#"data-docked-trailing="1""#));
    assert!(html.contains(date_picker::DOCKED_TRAILING_ICON));
    assert!(html.contains(r#"data-docked-year-toggle="1""#));
    assert!(html.contains(r#"data-docked-years="1""#));
    assert!(html.contains(r#"data-docked-year="2026""#));
    assert!(html.contains(r#"data-hero="datepicker-input""#));
    assert!(html.contains(r#"data-datepicker-input="1""#));
    assert!(html.contains(r#"data-date-input-pad="1""#));
    assert!(html.contains(r#"data-date-toggle-pad="1""#));
    assert!(html.contains(r#"data-date-range-title-pad="1""#));
    assert!(html.contains(r#"data-range-header-chrome="1""#));
    assert!(html.contains(r#"data-range-header-close="1""#));
    assert!(html.contains(date_picker::RANGE_HEADER_CLOSE_GLYPH));
    assert!(html.contains(r#"aria-label="Close""#));
    assert!(html.contains(r#"data-date-range-headline-pad="1""#));
    assert!(html.contains(r#"data-date-range-header-paddings="1""#));
    assert!(html.contains(r#"data-date-month-year="1""#));
    assert!(html.contains("month-nav-arrows"));
    assert!(html.contains("height: 56px; min-height: 56px"));
    assert!(html.contains(r#"data-date-header-min="1""#));
    assert!(html.contains(r#"data-date-container="1""#));
    assert!(html.contains("max-width: 360px; max-height: 568px"));
    assert!(html.contains(r#"data-date-entry-divider="1""#));
    assert!(html.contains(r#"class="dp-entry-divider" data-date-entry-divider="1""#));
    assert!(html.contains(".cal .dp-entry-divider { height: 1px; margin: 0 -12px; }"));
    assert!(html.contains(r#"data-date-year-divider="1""#));
    assert_eq!(html.matches(r#"data-date-year-divider="1""#).count(), 4);
    assert!(html.contains(".cal .dp-year-divider"));
    assert!(html.contains(r#"data-date-range-header-min="1""#));
    assert!(html.contains(&format!(
        "min-height: {};",
        date_picker::header_min_height_css()
    )));
    assert!(html.contains(&format!(
        "min-height: {};",
        date_picker::range_header_min_height_css()
    )));
    assert!(html.contains(r#"data-date-dialog-buttons="1""#));
    assert!(html.contains(&format!(
        "padding: {};",
        date_picker::dialog_buttons_padding_css()
    )));
    assert!(html.contains(&format!(
        "padding: {}",
        date_picker::input_field_padding_css()
    )));
    assert!(html.contains(&format!("padding: {};", date_picker::toggle_padding_css())));
    assert!(html.contains(&format!(
        "padding: {};",
        date_picker::range_title_padding_css()
    )));
    assert!(html.contains(&format!(
        "padding: {};",
        date_picker::range_headline_padding_css()
    )));
    assert!(html.contains(r#"data-hero="datepicker-picker-empty""#));
    assert!(html.contains(r#"data-picker-empty="1""#));
    assert!(html.contains("Selected date"));
    assert!(html.contains(r#"data-hero="datepicker-input-empty""#));
    assert!(html.contains(r#"data-date-input-empty="1""#));
    assert!(html.contains(r#"data-date-headline-empty="1""#));
    assert!(html.contains("Entered date"));
    assert!(html.contains(r#"data-hero="datepicker-input-errors""#));
    assert!(html.contains(r#"data-date-input-errors="1""#));
    assert!(html.contains(r#"data-date-error="year""#));
    assert!(html.contains(r#"data-date-error="allowed""#));
    assert!(html.contains("Date not allowed: Sat, Sep 12"));
    assert!(html.contains(date_picker::INPUT_ERROR_NOT_ALLOWED_SAMPLE));
    assert!(html.contains(r#"data-date-display="input""#));
    assert!(html.contains(r#"data-date-display="picker""#));
    assert!(html.contains(r#"data-date-display-live="1""#));
    assert!(html.contains(r#"data-date-display-toggle="1""#));
    assert!(html.contains(r#"data-hero="datepicker-year""#));
    assert!(html.contains(r#"data-hero="datepicker-range-input""#));
    assert!(html.contains(r#"data-datepicker-range-input="1""#));
    assert!(html.contains(r#"data-hero="datepicker-range-input-empty""#));
    assert!(html.contains(r#"data-range-input-empty="1""#));
    assert!(html.contains(r#"data-range-headline-empty="1""#));
    assert!(html.contains("Start date – End date"));
    assert!(html.contains(r#"data-hero="datepicker-range-start-only""#));
    assert!(html.contains(r#"data-datepicker-range-start-only="1""#));
    assert!(html.contains(r#"data-range-start-only="1""#));
    assert!(html.contains(r#"data-range-start-only-headline="1""#));
    assert!(html.contains("Sep 15 – End date"));
    assert!(html.contains(r#"data-hero="datepicker-range-end-only""#));
    assert!(html.contains(r#"data-datepicker-range-end-only="1""#));
    assert!(html.contains(r#"data-range-end-only="1""#));
    assert!(html.contains(r#"data-range-end-only-headline="1""#));
    assert!(html.contains("Start date – Sep 21"));
    assert!(html.contains(r#"data-hero="datepicker-range-input-errors""#));
    assert!(html.contains(r#"data-range-input-errors="1""#));
    assert!(html.contains(r#"data-range-error="year""#));
    assert!(html.contains(r#"data-range-error="allowed""#));
    assert!(html.contains(r#"data-date-range-allowed-start="1""#));
    assert!(html.contains("Date format not recognized"));
    assert!(html.contains("Date out of expected year range 1900 - 2100"));
    assert!(html.contains("End date can't be before start date"));
    assert!(html.contains(date_picker::INPUT_ERROR_YEAR_SAMPLE));
    assert!(html.contains(r#"data-date-range-start="1""#));
    assert!(html.contains(r#"data-date-range-end="1""#));
    assert!(html.contains("Enter dates"));
    assert!(html.contains("Start date"));
    assert!(html.contains("End date"));
    assert!(html.contains(r#"data-datepicker-year="1""#));
    assert!(html.contains(r#"data-date-pane="year""#));
    assert!(html.contains(r#"data-date-years="1""#));
    assert!(html.contains(r#"data-date-year="2026""#));
    assert!(html.contains("MM/DD/YYYY"));
    assert!(html.contains("09/15/2026"));
    assert!(html.contains("data-settings-scene=\"1\""));
    assert!(html.contains("data-settings-block=\"volume\""));
    assert!(html.contains("data-search=\"1\""));
    assert!(html.contains("Hinted search text"));
    assert!(html.contains("data-search-view=\"1\""));
    assert!(html.contains(r#"data-search-style="contained""#));
    assert!(html.contains(r#"data-width-class="compact""#));
    assert!(html.contains(r#"data-width-class="medium""#));
    assert!(html.contains(r#"data-search-expanded="fullscreen""#));
    assert!(html.contains(r#"data-search-expanded="docked""#));
    assert!(html.contains(r#"data-search-group="Recent""#));
    assert!(html.contains(r#"data-search-group="Suggestions""#));
    assert!(html.contains(r#"data-search-status="quick-results""#));
    assert!(html.contains(r#"data-search-status="results""#));
    assert!(html.contains("Quick results"));
    assert!(html.contains(r#"data-search-status-label="1""#));
    assert!(html.contains(r#"aria-live="polite""#));
    assert!(html.contains(r#"data-search-docked-stage="1""#));
    assert!(html.contains(r#"data-search-scrim="1""#));
    assert!(html.contains(r#"data-search-scrim-layer="1""#));
    assert!(html.contains(r#"data-docked-min-h="240""#));
    assert!(html.contains(r#"data-search-row="segmented""#));
    assert!(html.contains(r#"data-search-lines="two""#));
    assert!(html.contains(r#"data-search-open="1""#));
    assert!(html.contains(r#"data-search-open-affordance="1""#));
    assert!(html.contains("Installed application"));
    assert!(html.contains(r#"data-search-list="segmented""#));
    assert!(html.contains(r#"data-search-leading="avatar""#));
    assert!(html.contains(r#"data-search-leading="icon""#));
    assert!(html.contains(r#"data-search-clear="1""#));
    assert!(html.contains(r#"data-search-trailing="1""#));
    assert!(html.contains(search::TRAILING_CLEAR));
    assert!(html.contains(r#"data-hero="search-empty""#));
    assert!(html.contains(r#"data-search-empty="1""#));
    assert!(html.contains(r#"data-search-filters="1""#));
    assert!(html.contains(r#"data-search-filter="all""#));
    assert!(html.contains(r#"data-search-filter="settings""#));
    assert!(html.contains(r#"data-hero="search-filters""#));
    assert!(html.contains(r#"data-search-filter-chip="apps""#));
    assert!(html.contains(r#"data-search-category="apps""#));
    assert!(html.contains(search::EMPTY_SUGGESTIONS));
    assert!(html.contains("0 results"));
    assert!(html.contains("data-timepicker=\"1\""));
    assert!(html.contains(r#"data-time-layout="vertical""#));
    assert!(html.contains(r#"data-clock-face-margins="1""#));
    assert!(html.contains("margin-top: 36px; margin-bottom: 24px"));
    assert!(html.contains(r#"data-display-separator="1""#));
    assert!(html.contains("width: 24px; height: 80px"));
    assert!(html.contains(r#"data-period-toggle-margin="1""#));
    assert!(html.contains(r#"data-time-layout="horizontal""#));
    assert!(html.contains(r#"data-hero="timepicker-horizontal""#));
    assert!(html.contains("data-time-scroll=\"1\""));
    assert!(html.contains(r#"data-time-picker-style="scroll""#));
    assert!(html.contains(r#"data-time-picker-style="input""#));
    assert!(html.contains("data-time-input=\"1\""));
    assert!(html.contains(r#"data-time-support-label="hour""#));
    assert!(html.contains(r#"data-time-support-label="minute""#));
    assert!(html.contains("time-input-support"));
    assert!(html.contains(&format!(
        "margin-top:{}px",
        time_picker::SUPPORT_LABEL_TOP_DP
    )));
    assert!(html.contains("data-scroll-display-mode-toggle=\"1\""));
    assert!(html.contains(r#"data-time-display="input""#));
    assert!(html.contains(r#"data-time-format="24""#));
    assert!(html.contains("data-time-format-toggle=\"1\""));
    assert!(html.contains(r#"data-scroll-field="hour""#));
    assert!(html.contains(r#"data-scroll-field="minute""#));
    assert!(html.contains("data-dial=\"hour\""));
    assert!(html.contains(r#"data-ring="inner""#));
    assert!(html.contains(r#"data-ring="outer""#));
    assert!(html.contains(r#"data-format="24""#));
    assert!(html.contains("data-progress=\"indeterminate\""));
    assert!(html.contains("data-progress=\"ptr\""));
    assert!(html.contains("data-nav-rail=\"1\""));
    assert!(html.contains("data-hero=\"wide-rail\""));
    assert!(html.contains("data-hero=\"wide-rail-inflow\""));
    assert!(html.contains("data-hero=\"wide-rail-narrow\""));
    assert!(html.contains("data-hero=\"wide-rail-hide\""));
    assert!(html.contains("data-hero=\"wide-rail-header\""));
    assert!(html.contains("data-rail-layout=\"standard\""));
    assert!(html.contains("data-rail-layout=\"narrow\""));
    assert!(html.contains("data-rail-layout=\"hide\""));
    assert!(html.contains("data-rail-layout=\"header\""));
    assert!(html.contains("data-hide-on-collapse=\"1\""));
    assert!(html.contains("data-rail-arrangement=\"center\""));
    assert!(html.contains("data-rail-arrangement=\"bottom\""));
    assert!(html.contains("data-rail-header=\"1\""));
    assert!(html.contains("data-rail-header-menu=\"1\""));
    assert!(html.contains("data-rail-header-tooltip=\"1\""));
    assert!(html.contains("data-header-fab=\"1\""));
    assert!(html.contains("data-rail-fab-extend=\"1\""));
    assert!(html.contains("data-inflow-fab-extend=\"1\""));
    assert!(html.contains("data-modal-fab-extend=\"1\""));
    assert!(html.contains("data-narrow-fab-extend=\"1\""));
    assert!(html.contains("data-hide-fab-extend=\"1\""));
    assert!(html.contains("data-rail-fab-label=\"1\""));
    assert!(html.contains("function applyRailFabMorph"));
    assert!(html.contains("function applyRailContainerMorph"));
    assert!(html.contains("function hexMix"));
    assert!(html.contains("data-modal-expanded-shape=\"CornerLarge\""));
    assert!(html.contains("data-expanded-shape=\"16\""));
    assert!(html.contains("data-collapsed-shape=\"0\""));
    assert!(html.contains("data-content-padding=\"1\""));
    assert!(html.contains("data-content-pad-v=\"44\""));
    assert!(html.contains("data-content-pad-h=\"0\""));
    assert!(html.contains("ContentPadding"));
    assert!(html.contains("data-container-expanded"));
    assert!(html.contains("Expand rail"));
    assert!(html.contains(">Create<"));
    assert!(html.contains("function applyRailHideSlide"));
    assert!(html.contains("data-rail-menu=\"1\""));
    assert!(html.contains("data-narrow=\"1\""));
    assert!(html.contains("data-collapsed-width=\"80\""));
    assert!(html.contains("data-rail-inflow-body=\"1\""));
    assert!(html.contains("data-icon-position=\"top\""));
    assert!(html.contains("data-icon-position=\"start\""));
    assert!(html.contains("data-icon-morph=\"1\""));
    assert!(html.contains("data-icon-morph-ms"));
    assert!(html.contains("function applyRailIconMorph"));
    assert!(html.contains("iconPosition"));
    assert!(html.contains("data-wide-collapsed=\"1\""));
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
    assert!(html.contains("data-hero=\"menu\""));
    assert!(html.contains("data-menu-scheme=\"standard\""));
    assert!(html.contains("data-menu-scheme=\"vibrant\""));
    assert!(html.contains("data-menu-axis=\"vertical\""));
    assert!(html.contains("data-menu-axis=\"horizontal\""));
    assert!(html.contains("data-hero=\"menu-horizontal\""));
    assert!(html.contains("data-hero=\"menu-icons\""));
    assert!(html.contains("data-menu-group=\"0\""));
    assert!(html.contains("data-menu-h=\"Week\""));
    assert!(html.contains("data-menu-icon=\"I\""));
    assert!(html.contains("data-hero=\"menu-submenu\""));
    assert!(html.contains("data-menu-cascade=\"1\""));
    assert!(html.contains(
        r#"data-menu-cascade="1" data-open="1" data-typeahead="1" data-typeahead-autofocus="1""#
    ));
    assert!(html.contains("data-menu-submenu=\"1\""));
    assert!(html.contains("data-submenu-trigger=\"1\""));
    assert!(html.contains("data-typeahead=\"1\""));
    assert!(html.contains("data-menu-keyboard=\"1\""));
    assert!(html.contains("data-menu-focus=\"active\""));
    assert!(html.contains("data-menu-focus=\"inactive\""));
    assert!(html.contains("data-menu-item=\"Share\""));
    assert!(html.contains("data-menu-item=\"Save\""));
    assert!(html.contains("data-menu-item=\"Sort\""));
    assert!(html.contains("data-hero=\"menu-overlay\""));
    assert!(html.contains("data-menu-overlay=\"1\""));
    assert!(html.contains("data-overlay-flyout=\"1\""));
    assert!(html.contains("data-menu-anchor-label=\"Menu\""));
    assert!(html.contains("data-slider=\"0.3 enabled\""));
    assert!(html.contains("data-hero=\"slider\""));
    assert!(html.contains("data-hero=\"buttons\""));
    assert!(html.contains("data-field-hero=\"outlined\""));
    assert!(html.contains("data-notched=\"1\""));
    assert!(html.contains("data-field-style=\"expressive\""));
    assert!(html.contains("data-rounded-shape=\"CornerMedium\""));
    assert!(html.contains("data-tonal=\"1\""));
    assert!(html.contains("data-label-position=\"inside\""));
    assert!(html.contains("<legend"));
    assert!(html.contains("data-button-size=\"xl\""));
    assert!(html.contains("data-button-shape=\"square\""));
    assert!(html.contains("data-icon-size=\"xl\""));
    assert!(html.contains("data-hero=\"icon-buttons\""));
    assert!(html.contains("data-hero=\"icon-buttons-width\""));
    assert!(html.contains("data-icon-width=\"narrow\""));
    assert!(html.contains("data-icon-width=\"default\""));
    assert!(html.contains("data-icon-width=\"wide\""));
    assert!(html.contains("data-icon-width-size=\"s\""));
    assert!(html.contains("data-icon-width-size=\"m\""));
    assert!(html.contains("data-icon-width-w=\"32\""));
    assert!(html.contains("data-icon-width-w=\"52\""));
    assert!(html.contains("data-icon-width-w=\"48\""));
    assert!(html.contains("data-icon-width-w=\"72\""));
    assert!(html.contains("data-hero=\"chips\""));
    assert!(html.contains("data-hero=\"chips-elevated\""));
    assert!(html.contains("data-chip-morph=\"1\""));
    assert!(html.contains("data-hero-chip=\"filter\""));
    assert!(html.contains("data-hero-chip=\"tonal\""));
    assert!(html.contains("data-hero-chip=\"elevated\""));
    assert!(html.contains("data-hero-chip=\"input\""));
    assert!(html.contains("data-chip-style=\"flat\""));
    assert!(html.contains("data-chip-style=\"tonal\""));
    assert!(html.contains("data-chip-style=\"tonal-elevated\""));
    assert!(html.contains("data-chip-elev=\"1\""));
    assert!(html.contains("data-chip-label=\"Elevator\""));
    assert!(html.contains("data-chip-label=\"Washer\""));
    assert!(html.contains("data-chip-label=\"Pets\""));
    assert!(html.contains("data-chip-label=\"Wifi\""));
    assert!(html.contains("data-chip-label=\"Portland\""));
    assert!(html.contains("data-chip-label=\"Sofia\""));
    assert!(html.contains("data-chip-avatar=\"1\""));
    assert!(html.contains("data-chip-avatar-size=\"24\""));
    assert!(html.contains("data-chip-compact=\"1\""));
    assert!(html.contains("data-chip-press=\"1\""));
    assert!(html.contains("data-chip-rest-r=\"12\""));
    assert!(html.contains("data-chip-sel-r=\"16\""));
    assert!(html.contains("--press-r:8px"));
    assert!(html.contains("rememberAnimatedShape"));
    assert!(html.contains("data-hero-chip=\"input-avatar\""));
    assert!(html.contains("data-typeahead-autofocus=\"1\""));
    assert!(html.contains("function focusTypeahead"));
    assert!(html.contains("data-chip-r=\"12\""));
    assert!(html.contains("data-chip-r=\"16\""));
    assert!(html.contains("data-chip-r=\"8\""));
    assert!(html.contains("data-chip-lead=\"1\""));
    assert!(html.contains("data-chip-trail=\"1\""));
    assert!(html.contains("data-menu-anchor=\"1\""));
    assert!(html.contains("data-anchored=\"1\""));
    assert!(html.contains("data-scrim=\"0\""));
    assert!(html.contains("data-menu-overlay-stage=\"1\""));
    assert!(html.contains("data-hero=\"icon-buttons-toggle\""));
    assert!(html.contains("data-icon-toggle=\"unselected\""));
    assert!(html.contains("data-icon-toggle=\"selected\""));
    assert!(html.contains("data-icon-toggle-rest=\"round\""));
    assert!(html.contains("data-icon-toggle-rest=\"square\""));
    assert!(html.contains("data-icon-toggle-variant=\"filled\""));
    assert!(html.contains("data-icon-toggle-variant=\"tonal\""));
    assert!(html.contains("data-icon-toggle-variant=\"outlined\""));
    assert!(html.contains("data-icon-toggle-variant=\"standard\""));
    assert!(html.contains("data-icon-toggle-shape=\"round\""));
    assert!(html.contains("data-icon-toggle-shape=\"square\""));
    assert!(html.contains("data-icon-toggle-r=\"20\""));
    assert!(html.contains("data-icon-toggle-r=\"12\""));
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
        e.name == "List" && e.parity == Parity::Done && e.notes.contains("segmented")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "List"
            && e.notes.contains("swipe")
            && e.notes.contains("LazyColumn")
            && e.notes.contains("reorder")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Text field"
            && e.notes.contains("roundedShape")
            && e.notes.contains("CornerMedium")
            && e.notes.contains("tonalColors")
            && e.notes.contains("Inside")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Time picker"
            && e.notes.contains("TimeScroll")
            && e.notes.contains("ScrollField")
            && e.notes.contains("vibrantColors")
            && e.notes.contains("200dp")
            && e.notes.contains("TimeInput")
            && e.notes.contains("ScrollDisplayModeToggle")
            && e.notes.contains("TimePickerLayoutType")
            && e.notes.contains("Horizontal")
            && e.notes.contains("ClockDisplayBottomMargin")
            && e.notes.contains("ClockFaceBottomMargin")
            && e.notes.contains("DisplaySeparatorWidth")
            && e.notes.contains("PeriodToggleMargin")
            && e.notes.contains("SupportLabelTop")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Search"
            && e.notes.contains("contained")
            && e.notes.contains("24→12")
            && e.notes.contains("no divider")
            && e.notes.contains("compact")
            && e.notes.contains("600dp")
            && e.notes.contains("fullscreen")
            && e.notes.contains("Quick results")
            && e.notes.contains("Results")
            && e.notes.contains("240")
            && e.notes.contains("scrim")
            && e.notes.contains("segmented")
            && e.notes.contains("2dp")
            && e.notes.contains("two-line")
            && e.notes.contains("40dp")
            && e.notes.contains("20dp")
            && e.notes.contains("clear-X")
            && e.notes.contains("no-results")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Date picker"
            && e.notes.contains("live start→end")
            && e.notes.contains("Select dates")
            && e.notes.contains("DateRangePicker")
            && e.notes.contains("drawRangeBackground")
            && e.notes.contains("VerticalMonthsList")
            && e.notes.contains("modal Confirm/Cancel")
            && e.notes.contains("modal month nav")
            && e.notes.contains("docked popup")
            && e.notes.contains("live day select")
            && e.notes.contains("DateRange")
            && e.notes.contains("DateInputValidator")
            && e.notes.contains("InputTextFieldPadding")
            && e.notes.contains("DatePickerModeTogglePadding")
            && e.notes.contains("DateRangePickerTitlePadding")
            && e.notes.contains("range-header chrome")
            && e.notes.contains("DateRangePickerHeadlinePadding")
            && e.notes.contains("DialogButtonsPadding")
            && e.notes.contains("MonthYearHeight")
            && e.notes.contains("HeaderContainerHeight")
            && e.notes.contains("HeaderHeightOffset")
            && e.notes.contains("ContainerWidth")
            && e.notes.contains("ContainerHeight")
            && e.notes.contains("Entered date")
            && e.notes.contains("Selected date")
            && e.notes.contains("Start date – End date")
            && e.notes.contains("Sep 15 – End date")
            && e.notes.contains("Start date – Sep 21")
            && e.notes.contains("year range")
            && e.notes.contains("SelectableDates")
            && e.notes.contains("Date not allowed")
            && e.notes.contains("Disabled")
            && e.notes.contains("YearPicker")
            && e.notes.contains("YearPicker trailing HorizontalDivider")
    }));
    assert!(INVENTORY
        .iter()
        .any(|e| { e.name == "Motion tokens" && e.notes.contains("TimeScroll") }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Button group"
            && e.notes.contains("Standard")
            && e.notes.contains("0.15")
            && e.notes.contains("OverflowIndicator")
            && e.notes.contains("More")
    }));
    assert!(INVENTORY
        .iter()
        .any(|e| { e.name == "Tooltip" && e.notes.contains("long-press") }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Icon button"
            && e.notes.contains("narrow")
            && e.notes.contains("wide")
            && e.notes.contains("toggle")
            && e.notes.contains("IconToggleButton")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Chip"
            && e.notes.contains("ChipShapes")
            && e.notes.contains("12/16/8")
            && e.notes.contains("check")
            && e.notes.contains("ElevatedFilterChip")
            && e.notes.contains("tonal")
            && e.notes.contains("rememberAnimatedShape")
            && e.notes.contains("with_animation")
            && e.notes.contains("avatar")
            && e.notes.contains("4dp")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Navigation rail"
            && e.notes.contains("WideNavigationRailItem")
            && e.notes.contains("Top")
            && e.notes.contains("Start")
            && e.notes.contains("iconPosition")
            && e.notes.contains("lerp")
            && e.notes.contains("in-flow")
            && e.notes.contains("Extended")
            && e.notes.contains("96")
            && e.notes.contains("narrow")
            && e.notes.contains("80")
            && e.notes.contains("hideOnCollapse")
            && e.notes.contains("always-extended")
            && e.notes.contains("Arrangement.Center")
            && e.notes.contains("Arrangement.Bottom")
            && e.notes.contains("MenuOpen")
            && e.notes.contains("Extended")
            && e.notes.contains("Create")
            && e.notes.contains("CornerLarge")
            && e.notes.contains("SurfaceContainer")
            && e.notes.contains("ContentPadding")
            && e.notes.contains("header-less")
            && e.notes.contains("secondary")
    }));
    assert!(INVENTORY.iter().any(|e| {
        e.name == "Menu"
            && e.notes.contains("submenu")
            && e.notes.contains("ActiveContainerShape")
            && e.notes.contains("typeahead")
            && e.notes.contains("overlay")
            && e.notes.contains("More")
            && e.notes.contains("unscrimmed")
            && e.notes.contains("overflow")
            && e.notes.contains("200ms")
            && e.notes.contains("GPUI")
            && e.notes.contains("autofocus")
            && e.notes.contains("in-page")
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
        "Side sheet",
        "Tooltip",
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
    assert_eq!(
        dialog::account_initials("leevilanuevanotes@google.com"),
        "L"
    );
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
    assert_eq!(menu.corners.top_left, 16.0);
    assert_eq!(menu.container, theme.color.surface_container_low);
    assert_eq!(menu.elevation_dp, 3.0);
    let selected = menu::resolve_item(&theme, true, InteractionState::Enabled);
    assert_eq!(selected.container, theme.color.tertiary_container);
    assert_eq!(selected.height_dp, 44.0);
    assert_eq!(selected.corners.top_left, 12.0);
}

#[test]
fn expressive_menu_tokens() {
    let theme = Theme::light();
    assert_eq!(menu::ITEM_HEIGHT_DP, 44.0);
    assert_eq!(menu::CONTAINER_CORNER_DP, 16.0);
    assert_eq!(menu::GROUP_GAP_DP, 2.0);
    assert_eq!(menu::GROUP_PAD_DP, 4.0);
    assert_eq!(menu::HORIZONTAL_GAP_DP, 2.0);
    assert_eq!(menu::HORIZONTAL_ICON_GAP_DP, 4.0);
    assert_eq!(menu::HORIZONTAL_ICON_SIZE_DP, 52.0);
    assert_eq!(menu::STYLE_ITEMS[menu::STYLE_SELECTED].label, "Bold");
    assert_eq!(menu::HORIZONTAL_LABELS[menu::HORIZONTAL_SELECTED], "Week");
    assert_eq!(menu::VERTICAL_GROUPS.len(), 3);
    assert_eq!(menu::EDIT_ITEMS[0].shortcut, "⌘X");
    assert_eq!(
        menu::trailing_text(&menu::MORE_ITEMS[0]),
        menu::SUBMENU_CHEVRON
    );

    let standard = menu::resolve_container(&theme, menu::MenuScheme::Standard);
    assert_eq!(standard.container, theme.color.surface_container_low);
    assert_eq!(standard.corners.top_left, 16.0);
    let vibrant = menu::resolve_container(&theme, menu::MenuScheme::Vibrant);
    assert_eq!(vibrant.container, theme.color.tertiary_container);

    let first = menu::resolve_group(&theme, menu::MenuScheme::Standard, 0, 3);
    assert_eq!(first.corners.top_left, 16.0);
    assert_eq!(first.corners.bottom_left, 8.0);
    let last = menu::resolve_group(&theme, menu::MenuScheme::Standard, 2, 3);
    assert_eq!(last.corners.top_left, 8.0);
    assert_eq!(last.corners.bottom_left, 16.0);

    let mid = menu::resolve_item_at(
        &theme,
        menu::MenuScheme::Standard,
        menu::MenuAxis::Vertical,
        1,
        3,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(mid.container, theme.color.surface_container_low);
    assert_eq!(mid.corners.top_left, 4.0);
    let bold = menu::resolve_item_at(
        &theme,
        menu::MenuScheme::Standard,
        menu::MenuAxis::Vertical,
        1,
        3,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(bold.container, theme.color.tertiary_container);
    assert_eq!(bold.label, theme.color.on_tertiary_container);
    assert_eq!(bold.corners.top_left, 12.0);

    let v_sel = menu::resolve_item_at(
        &theme,
        menu::MenuScheme::Vibrant,
        menu::MenuAxis::Vertical,
        1,
        3,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(v_sel.container, theme.color.tertiary);
    assert_eq!(v_sel.label, theme.color.on_tertiary);

    let week = menu::resolve_horizontal(
        &theme,
        menu::MenuScheme::Standard,
        1,
        4,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(week.container, theme.color.tertiary_container);
    assert_eq!(week.corners.top_left, 999.0);
    assert_eq!(week.height_dp, 44.0);
    let day = menu::resolve_horizontal(
        &theme,
        menu::MenuScheme::Standard,
        0,
        4,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(day.corners.top_left, 4.0);

    let icon_on = menu::resolve_horizontal_icon(&theme, menu::MenuScheme::Standard, 1, 3, true);
    assert_eq!(icon_on.height_dp, 52.0);
    assert_eq!(icon_on.corners.top_left, 999.0);
}

#[test]
fn expressive_menu_submenu_tokens() {
    let theme = Theme::light();
    assert_eq!(menu::ACTIVE_CONTAINER_CORNER_DP, 24.0);
    assert_eq!(menu::INACTIVE_CONTAINER_CORNER_DP, 8.0);
    assert_eq!(menu::SUBMENU_GAP_DP, 4.0);
    assert_eq!(menu::SUBMENU_ITEMS[menu::SUBMENU_SELECTED].label, "Share");
    assert_eq!(menu::SUBMENU_ITEMS[1].label, "Save");
    assert_eq!(menu::SUBMENU_ITEMS[2].label, "Sort");
    assert!(menu::CASCADE_OPEN);

    let rest = menu::group_corners(0, 3);
    assert_eq!(rest.top_left, 16.0);
    assert_eq!(rest.bottom_left, 8.0);
    let inactive = menu::group_corners_focus(0, 3, menu::MenuFocus::Inactive);
    assert_eq!(inactive.top_left, 8.0);
    assert_eq!(inactive.bottom_left, 8.0);
    let active = menu::resolve_submenu(&theme, menu::MenuScheme::Standard);
    assert_eq!(active.corners.top_left, 24.0);
    assert_eq!(active.container, theme.color.surface_container_low);
    assert_eq!(active.elevation_dp, 3.0);

    let labels = menu::submenu_labels();
    assert_eq!(
        menu::typeahead_index(&labels, 0, 's'),
        Some(1),
        "s from Share → Save"
    );
    assert_eq!(menu::typeahead_index(&labels, 1, 's'), Some(2));
    assert_eq!(menu::typeahead_index(&labels, 2, 's'), Some(0));
    assert_eq!(menu::typeahead_index(&labels, 0, 'x'), None);
    let parent = menu::parent_labels();
    assert_eq!(parent.last().copied(), Some("More"));
    assert_eq!(menu::typeahead_index(&parent, 0, 'm'), Some(6));
}

#[test]
fn expressive_menu_overlay_session() {
    assert_eq!(menu::MORE_INDEX, 6);
    assert!(!menu::OVERLAY_FLYOUT_OPEN);
    assert!(menu::is_submenu_trigger(menu::MORE_INDEX));
    assert!(!menu::is_submenu_trigger(0));
    assert_eq!(menu::parent_item_count(), 7);
    let (_, _, more) = menu::parent_item_at(menu::MORE_INDEX).expect("More");
    assert!(more.submenu);
    assert_eq!(more.label, "More");

    let mut overlay = menu::OverlayMenuSession::overlay();
    assert!(!overlay.submenu_open);
    assert_eq!(overlay.parent_focus(), menu::MenuFocus::Rest);
    assert_eq!(
        overlay.click_parent(menu::MORE_INDEX),
        menu::OverlayMenuAction::Stay,
        "More opens the End flyout instead of dismissing"
    );
    assert!(overlay.submenu_open);
    assert_eq!(overlay.parent_focus(), menu::MenuFocus::Inactive);
    assert_eq!(overlay.apply_key("s"), menu::OverlayMenuAction::Stay);
    assert_eq!(overlay.submenu_hi, 1, "s from Share → Save");
    assert_eq!(overlay.apply_key("s"), menu::OverlayMenuAction::Stay);
    assert_eq!(overlay.submenu_hi, 2);
    assert_eq!(overlay.click_submenu(2), menu::OverlayMenuAction::Commit);
    assert!(!overlay.submenu_open);
    assert_eq!(overlay.committed, menu::OverlayMenuCommit::Submenu(2));

    let mut overlay = menu::OverlayMenuSession::overlay();
    assert_eq!(
        overlay.click_parent(0),
        menu::OverlayMenuAction::Commit,
        "leaf items still dismiss the overlay"
    );
    let intent = overlay.hover_parent(menu::MORE_INDEX);
    assert!(
        !overlay.submenu_open,
        "hover-open waits HOVER_OPEN_DELAY_MS"
    );
    let menu::HoverOpenIntent::Delay { index, seq } = intent else {
        panic!("More hover should schedule a delay");
    };
    assert_eq!(index, menu::MORE_INDEX);
    assert_eq!(overlay.pending_hover, Some(menu::MORE_INDEX));
    assert!(overlay.confirm_hover_open(index, seq));
    assert!(overlay.submenu_open);

    overlay.hover_parent(1);
    assert!(!overlay.submenu_open);
    assert_eq!(overlay.pending_hover, None);

    let stale = overlay.hover_parent(menu::MORE_INDEX);
    overlay.hover_parent(1);
    if let menu::HoverOpenIntent::Delay { index, seq } = stale {
        assert!(
            !overlay.confirm_hover_open(index, seq),
            "leaving More cancels the pending timer"
        );
    }
    assert!(!overlay.submenu_open);

    overlay.hover_parent(menu::MORE_INDEX);
    overlay.hover_leave();
    assert!(!overlay.submenu_open);
    assert_eq!(overlay.pending_hover, None);

    overlay.hover_parent(menu::MORE_INDEX);
    assert_eq!(overlay.apply_key("left"), menu::OverlayMenuAction::Stay);
    assert!(!overlay.submenu_open);
    assert_eq!(
        overlay.apply_key("escape"),
        menu::OverlayMenuAction::Dismiss
    );
    overlay.hover_parent(menu::MORE_INDEX);
    assert_eq!(overlay.apply_key("right"), menu::OverlayMenuAction::Stay);
    assert!(overlay.submenu_open);
    assert_eq!(overlay.pending_hover, None);

    let cascade = menu::OverlayMenuSession::cascade();
    assert!(cascade.submenu_open);
    assert_eq!(cascade.parent_focus(), menu::MenuFocus::Inactive);

    let mut overflow = menu::OverlayMenuSession::standard_overflow();
    assert_eq!(overflow.kind, menu::GroupedPopupKind::StandardOverflow);
    assert_eq!(overflow.kind.groups().len(), 2);
    assert_eq!(overflow.kind.item_count(), 4);
    assert_eq!(overflow.kind.item_at(0).unwrap().2.label, "Left");
    assert_eq!(overflow.kind.item_at(2).unwrap().2.label, "Justify");
    assert_eq!(overflow.click_parent(3), menu::OverlayMenuAction::Stay);
    assert!(overflow.submenu_open);
    assert_eq!(overflow.apply_key("s"), menu::OverlayMenuAction::Stay);
    assert_eq!(overflow.submenu_hi, 1);

    let connected = menu::OverlayMenuSession::connected_overflow();
    assert_eq!(connected.kind.item_at(0).unwrap().2.label, "Cut");
    let split = menu::OverlayMenuSession::split();
    assert_eq!(split.kind.item_at(0).unwrap().2.label, "Add to cart");
    assert_eq!(split.kind.item_at(1).unwrap().2.label, "Save for later");
    assert_eq!(menu::HOVER_OPEN_DELAY_MS, 200);
    assert_eq!(menu::SPLIT_ITEMS.len(), 2);
    assert_eq!(menu::ALIGN_ITEMS.len(), 3);
}

#[test]
fn expressive_chip_tokens() {
    let theme = Theme::light();
    assert_eq!(chip::HEIGHT_DP, 32.0);
    assert_eq!(chip::UNSELECTED_CORNER_DP, 12.0);
    assert_eq!(chip::SELECTED_CORNER_DP, 16.0);
    assert_eq!(chip::PRESSED_CORNER_DP, 8.0);
    assert_eq!(chip::ICON_DP, 18.0);
    assert_eq!(chip::ICON_GAP_DP, 8.0);
    assert_eq!(chip::COMPACT_ICON_GAP_DP, 4.0);
    assert_eq!(chip::LEADING_PAD_START_DP, 8.0);
    assert_eq!(chip::TRAILING_PAD_END_DP, 8.0);
    assert_eq!(chip::FILTER_HERO[0].label, "Elevator");
    assert_eq!(chip::FILTER_HERO[1].label, "Washer");
    assert_eq!(chip::FILTER_HERO[1].selected, true);
    assert_eq!(chip::FILTER_HERO[2].label, "Pets");
    assert_eq!(chip::FILTER_HERO[2].state, InteractionState::Pressed);
    assert_eq!(chip::INPUT_HERO[0].label, "Portland");
    assert_eq!(chip::AVATAR_DP, 24.0);
    assert_eq!(chip::AVATAR_PAD_START_DP, 4.0);
    assert_eq!(chip::INPUT_AVATAR_LABEL, "Sofia");
    assert_eq!(chip::INPUT_AVATAR_KIND.label(), "sofia");
    assert!(chip::INPUT_AVATAR_HERO[0].has_avatar());
    assert_eq!(
        chip::demo_avatar(chip::INPUT_AVATAR_HERO[0]),
        Some(chip::INPUT_AVATAR_KIND)
    );
    assert_eq!(chip::demo_icon_gap_dp(chip::INPUT_AVATAR_HERO[0]), 4.0);
    assert_eq!(chip::icon_gap_dp(false), 8.0);
    let avatar_off = chip::resolve_demo(&theme, chip::INPUT_AVATAR_HERO[0]);
    assert_eq!(avatar_off.pad_start_dp, chip::AVATAR_PAD_START_DP);
    assert_eq!(avatar_off.pad_end_dp, chip::TRAILING_PAD_END_DP);
    assert_eq!(
        chip::animated_corner_dp(&theme, chip::ChipVariant::Filter, false, 0.0),
        theme.shapes.medium
    );
    assert_eq!(
        chip::animated_corner_dp(&theme, chip::ChipVariant::Filter, true, 0.0),
        chip::SELECTED_CORNER_DP
    );
    assert_eq!(
        chip::animated_corner_dp(&theme, chip::ChipVariant::Filter, false, 1.0),
        theme.shapes.small
    );
    let samples: [f32; 9] = core::array::from_fn(|i| {
        chip::animated_corner_dp(
            &theme,
            chip::ChipVariant::Filter,
            false,
            (i + 1) as f32 / 10.0,
        )
    });
    assert!(
        samples
            .iter()
            .any(|v| *v > theme.shapes.small && *v < theme.shapes.medium),
        "rememberAnimatedShape should pass through interior corners, got {samples:?}"
    );
    assert_eq!(chip::press_t(InteractionState::Pressed), 1.0);
    assert_eq!(chip::press_t(InteractionState::Enabled), 0.0);
    assert_eq!(chip::press_t_anim(true, 0.0), 0.0);
    assert_eq!(chip::press_t_anim(true, 1.0), 1.0);
    assert_eq!(chip::press_t_anim(false, 0.0), 1.0);
    assert_eq!(chip::press_t_anim(false, 1.0), 0.0);
    assert_eq!(chip::press_ms(&theme), theme.motion.spatial_fast_ms);
    assert!(menu::TYPEAHEAD_AUTOFOCUS);
    assert!(menu::typeahead_autofocus_in_page(true));
    assert!(!menu::typeahead_autofocus_in_page(false));
    assert_eq!(
        menu::typeahead_autofocus_kind(false, true, false, false),
        Some(menu::GroupedPopupKind::StandardOverflow)
    );
    assert_eq!(
        menu::typeahead_autofocus_kind(false, false, true, false),
        Some(menu::GroupedPopupKind::ConnectedOverflow)
    );
    assert_eq!(
        menu::typeahead_autofocus_kind(false, false, false, true),
        Some(menu::GroupedPopupKind::Split)
    );
    assert_eq!(
        menu::typeahead_autofocus_kind(false, false, false, false),
        None
    );
    assert!(chip::ChipVariant::Filter.morphs());
    assert!(chip::ChipVariant::Input.morphs());
    assert!(!chip::ChipVariant::Assist.morphs());
    assert!(!chip::ChipVariant::Suggestion.morphs());
    assert_eq!(
        chip::leading_icon(chip::ChipVariant::Filter, true),
        Some(chip::CHECK_GLYPH)
    );
    assert_eq!(
        chip::trailing_icon(chip::ChipVariant::Input),
        Some(chip::CLOSE_GLYPH)
    );

    let filter_off = chip::resolve(
        &theme,
        chip::ChipVariant::Filter,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(filter_off.corners.top_left, theme.shapes.medium);
    assert_eq!(filter_off.outline, Some((theme.color.outline_variant, 1.0)));
    assert_eq!(filter_off.content, theme.color.on_surface_variant);
    assert_eq!(filter_off.elevation_dp, 0.0);
    assert_eq!(filter_off.pad_start_dp, chip::PAD_H_DP);

    let filter_on = chip::resolve(
        &theme,
        chip::ChipVariant::Filter,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(filter_on.corners.top_left, chip::SELECTED_CORNER_DP);
    assert_eq!(filter_on.container, theme.color.secondary_container);
    assert_eq!(filter_on.content, theme.color.on_secondary_container);
    assert_eq!(filter_on.outline, None);
    assert_eq!(filter_on.pad_start_dp, chip::LEADING_PAD_START_DP);

    let filter_press = chip::resolve(
        &theme,
        chip::ChipVariant::Filter,
        false,
        InteractionState::Pressed,
    );
    assert_eq!(filter_press.corners.top_left, theme.shapes.small);

    let input_off = chip::resolve(
        &theme,
        chip::ChipVariant::Input,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(input_off.corners.top_left, theme.shapes.medium);
    assert_eq!(input_off.pad_end_dp, chip::TRAILING_PAD_END_DP);
    assert_eq!(
        input_off.secondary_content,
        Some(theme.color.on_surface_variant)
    );

    let input_on = chip::resolve(
        &theme,
        chip::ChipVariant::Input,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(input_on.corners.top_left, chip::SELECTED_CORNER_DP);
    assert_eq!(input_on.container, theme.color.secondary_container);

    let assist = chip::resolve(
        &theme,
        chip::ChipVariant::Assist,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(assist.corners.top_left, 16.0);
    let suggestion = chip::resolve(
        &theme,
        chip::ChipVariant::Suggestion,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(suggestion.corners.top_left, 16.0);

    assert_eq!(chip::ChipColor::TonalElevated.label(), "tonal-elevated");
    assert!(chip::ChipColor::Tonal.tonal());
    assert!(chip::ChipColor::TonalElevated.elevated());
    assert_eq!(chip::ELEVATED_DP, 1.0);
    assert_eq!(chip::ELEVATED_FILTER_HERO[0].label, "Elevator");
    assert_eq!(
        chip::ELEVATED_FILTER_HERO[0].color,
        chip::ChipColor::TonalElevated
    );
    assert_eq!(
        chip::demo_leading_icon(chip::ELEVATED_FILTER_HERO[0]),
        Some(chip::LEADING_GLYPH)
    );
    assert_eq!(
        chip::demo_leading_icon(chip::ELEVATED_FILTER_HERO[1]),
        Some(chip::CHECK_GLYPH)
    );
    let elev_off = chip::resolve_demo(&theme, chip::ELEVATED_FILTER_HERO[0]);
    assert_eq!(elev_off.container, theme.color.surface_container_low);
    assert_eq!(elev_off.content, theme.color.on_surface_variant);
    assert_eq!(elev_off.outline, None);
    assert_eq!(elev_off.elevation_dp, chip::ELEVATED_DP);
    assert_eq!(
        elev_off.secondary_content,
        Some(theme.color.on_surface_variant)
    );
    assert_eq!(elev_off.pad_start_dp, chip::LEADING_PAD_START_DP);
    let elev_on = chip::resolve_demo(&theme, chip::ELEVATED_FILTER_HERO[1]);
    assert_eq!(elev_on.container, theme.color.secondary_container);
    assert_eq!(elev_on.elevation_dp, chip::ELEVATED_DP);
    assert_eq!(
        elev_on.secondary_content,
        Some(theme.color.on_secondary_container)
    );
    let tonal_off = chip::resolve_demo(&theme, chip::TONAL_FILTER_HERO[0]);
    assert_eq!(tonal_off.outline, Some((theme.color.outline_variant, 1.0)));
    assert_eq!(tonal_off.elevation_dp, 0.0);
    assert_eq!(
        tonal_off.secondary_content,
        Some(theme.color.on_surface_variant)
    );
    let flat_lead = chip::leading_icon_color(&theme, chip::ChipColor::Flat, false);
    assert_eq!(flat_lead, theme.color.primary);
    assert!(!menu::OVERLAY_USES_SCRIM);
    assert_eq!(menu::OVERLAY_ANCHOR_LABEL, "Menu");
}

#[test]
fn expressive_wide_rail_icon_position() {
    let theme = Theme::light();
    assert_eq!(
        navigation_rail::icon_position_for(false),
        navigation_rail::IconPosition::Top
    );
    assert_eq!(
        navigation_rail::icon_position_for(true),
        navigation_rail::IconPosition::Start
    );
    assert_eq!(
        navigation_rail::icon_position_for_mode(navigation_rail::RailMode::Collapsed),
        navigation_rail::IconPosition::Top
    );
    assert_eq!(
        navigation_rail::icon_position_for_mode(navigation_rail::RailMode::Expanded),
        navigation_rail::IconPosition::Start
    );
    assert_eq!(navigation_rail::IconPosition::Top.label(), "top");
    assert_eq!(navigation_rail::IconPosition::Start.label(), "start");
    assert!(navigation_rail::IconPosition::Start.is_start());
    assert!(!navigation_rail::IconPosition::Top.is_start());

    assert_eq!(navigation_rail::WIDE_COLLAPSED_WIDTH_DP, 96.0);
    assert_eq!(navigation_rail::EXPANDED_WIDTH_MAX_DP, 360.0);
    assert_eq!(navigation_rail::START_INDICATOR_H_DP, 56.0);
    assert_eq!(navigation_rail::START_LEADING_DP, 16.0);
    assert_eq!(navigation_rail::START_TRAILING_DP, 16.0);
    assert_eq!(navigation_rail::START_ICON_LABEL_GAP_DP, 8.0);
    assert_eq!(navigation_rail::TOP_ICON_LABEL_GAP_DP, 4.0);
    assert_eq!(navigation_rail::ITEM_VERTICAL_SPACE_DP, 4.0);
    assert_eq!(navigation_rail::WIDE_TOP_SPACE_DP, 44.0);
    assert_eq!(navigation_rail::CONTENT_PAD_VERTICAL_DP, 44.0);
    assert_eq!(navigation_rail::CONTENT_PAD_HORIZONTAL_DP, 0.0);
    assert_eq!(navigation_rail::PAD_TOP_DP, 44.0);
    assert_eq!(navigation_rail::PAD_BOTTOM_DP, 44.0);
    let pad = navigation_rail::content_padding();
    assert!((pad.start_dp).abs() < 0.01);
    assert!((pad.end_dp).abs() < 0.01);
    assert!((pad.top_dp - 44.0).abs() < 0.01);
    assert!((pad.bottom_dp - 44.0).abs() < 0.01);
    assert_eq!(navigation_rail::content_padding_css(), "44px 0px");
    assert!((navigation_rail::start_indicator_width_dp(220.0) - 188.0).abs() < 0.01);
    assert_eq!(
        navigation_rail::resolve_wide_collapsed(&theme).width_dp,
        96.0
    );

    let top = navigation_rail::item_metrics(&theme, navigation_rail::IconPosition::Top);
    assert_eq!(top.indicator_w_dp, 56.0);
    assert_eq!(top.indicator_h_dp, 32.0);
    assert!(!top.indicator_full_width);
    assert_eq!(top.icon_label_gap_dp, 4.0);
    assert_eq!(top.label_style.name, "labelMedium");

    let start = navigation_rail::item_metrics(&theme, navigation_rail::IconPosition::Start);
    assert_eq!(start.indicator_h_dp, 56.0);
    assert!(start.indicator_full_width);
    assert_eq!(start.icon_label_gap_dp, 8.0);
    assert_eq!(start.pad_start_dp, 16.0);
    assert_eq!(start.pad_end_dp, 16.0);
    assert_eq!(start.label_style.name, "labelLarge");

    let rail = navigation_rail::resolve(&theme);
    assert_eq!(rail.active_label, theme.color.secondary);
    assert_eq!(rail.active_indicator, theme.color.secondary_container);
    assert_eq!(rail.inactive_label, theme.color.on_surface_variant);

    assert_eq!(navigation_rail::icon_position_t(false), 0.0);
    assert_eq!(navigation_rail::icon_position_t(true), 1.0);
    let top_m = navigation_rail::item_morph(&theme, 0.0, 80.0);
    assert_eq!(top_m.icon_position, navigation_rail::IconPosition::Top);
    assert!((top_m.icon_box_w_dp - 56.0).abs() < 0.01);
    assert!((top_m.icon_box_h_dp - 32.0).abs() < 0.01);
    assert!((top_m.icon_indicator_alpha - 1.0).abs() < 0.01);
    assert!(top_m.dest_indicator_alpha.abs() < 0.01);
    assert!(top_m.label_center);
    let start_m = navigation_rail::item_morph(&theme, 1.0, 220.0);
    assert_eq!(start_m.icon_position, navigation_rail::IconPosition::Start);
    assert!((start_m.icon_box_w_dp - 24.0).abs() < 0.01);
    assert!((start_m.icon_box_h_dp - 24.0).abs() < 0.01);
    assert!(start_m.icon_indicator_alpha.abs() < 0.01);
    assert!((start_m.dest_indicator_alpha - 1.0).abs() < 0.01);
    assert!(!start_m.label_center);
    assert!((start_m.dest_width_dp - 188.0).abs() < 0.01);
    let samples: [navigation_rail::RailItemMorph; 7] =
        core::array::from_fn(|i| navigation_rail::item_morph(&theme, (i + 1) as f32 / 10.0, 150.0));
    assert!(
        samples
            .iter()
            .any(|m| m.icon_box_w_dp > 24.0 && m.icon_box_w_dp < 56.0),
        "Top→Start lerp should pass through interior icon widths, got {:?}",
        samples.map(|m| m.icon_box_w_dp)
    );
    assert!(samples
        .iter()
        .any(|m| m.dest_indicator_alpha > 0.0 && m.dest_indicator_alpha < 1.0));
    assert!((navigation_rail::morph_width_eased(&theme, 0.0) - 96.0).abs() < 0.01);
    assert!((navigation_rail::morph_width_eased(&theme, 1.0) - 220.0).abs() < 0.01);
    assert!((navigation_rail::morph_width_narrow_dp(0.0) - 80.0).abs() < 0.01);
    assert_eq!(navigation_rail::RailCollapsedKind::Narrow.width_dp(), 80.0);
    assert_eq!(navigation_rail::RailCollapsedKind::Wide.width_dp(), 96.0);
    assert_eq!(navigation_rail::RailCollapsedKind::Narrow.label(), "narrow");
    assert!(navigation_rail::RailCollapsedKind::Narrow.is_narrow());
    assert_eq!(
        navigation_rail::resolve_mode_kind(
            &theme,
            navigation_rail::RailMode::Collapsed,
            navigation_rail::RailCollapsedKind::Narrow
        )
        .width_dp,
        80.0
    );
    assert!(
        (navigation_rail::morph_width_eased_kind(
            &theme,
            navigation_rail::RailCollapsedKind::Narrow,
            0.0
        ) - 80.0)
            .abs()
            < 0.01
    );
    assert_eq!(navigation_rail::RailMode::Collapsed.width_dp(), 96.0);
    assert_eq!(navigation_rail::RailMode::Collapsed.narrow_width_dp(), 80.0);
    assert_eq!(
        navigation_rail::resolve_mode(&theme, navigation_rail::RailMode::Collapsed).width_dp,
        96.0
    );
    assert_eq!(
        navigation_rail::RailExpandedLayout::Standard.label(),
        "standard"
    );
    assert_eq!(navigation_rail::RailExpandedLayout::Modal.label(), "modal");
    assert!(navigation_rail::RailExpandedLayout::Standard.in_flow());
    assert!(!navigation_rail::RailExpandedLayout::Modal.in_flow());
    assert!(!navigation_rail::is_modal_for(
        navigation_rail::RailMode::Expanded,
        navigation_rail::RailExpandedLayout::Standard
    ));
    assert!(navigation_rail::is_modal_for(
        navigation_rail::RailMode::Expanded,
        navigation_rail::RailExpandedLayout::Modal
    ));
    assert!(!navigation_rail::overlay_window_for(
        navigation_rail::RailMode::Expanded,
        navigation_rail::RailExpandedLayout::Standard
    ));
    assert!(!navigation_rail::focus_trapped_for(
        navigation_rail::RailMode::Expanded,
        navigation_rail::RailExpandedLayout::Standard
    ));
    assert!(
        navigation_rail::scrim_opacity_for(navigation_rail::RailExpandedLayout::Standard, 1.0)
            .abs()
            < 0.01
    );
    assert!(
        (navigation_rail::scrim_opacity_for(navigation_rail::RailExpandedLayout::Modal, 1.0)
            - navigation_rail::SCRIM_OPACITY)
            .abs()
            < 0.01
    );
    assert!(
        navigation_rail::elevation_dp_for(
            navigation_rail::RailExpandedLayout::Standard,
            1.0,
            &theme
        )
        .abs()
            < 0.01
    );
    assert_eq!(navigation_rail::IN_FLOW_BODY, "Inbox");
    assert_eq!(navigation_rail::HIDE_COLLAPSED_WIDTH_DP, 0.0);
    assert!(navigation_rail::HIDE_DEMO_HIDE_ON_COLLAPSE);
    assert!(navigation_rail::hide_on_collapse_for(
        navigation_rail::RailExpandedLayout::Modal,
        true
    ));
    assert!(!navigation_rail::hide_on_collapse_for(
        navigation_rail::RailExpandedLayout::Standard,
        true
    ));
    assert!(!navigation_rail::collapsed_visible(true));
    assert!(navigation_rail::collapsed_visible(false));
    assert_eq!(
        navigation_rail::icon_position_for_hide(false, true),
        navigation_rail::IconPosition::Start
    );
    assert_eq!(
        navigation_rail::icon_position_for_hide(false, false),
        navigation_rail::IconPosition::Top
    );
    assert!((navigation_rail::hide_slide_offset_dp(0.0) + 220.0).abs() < 0.01);
    assert!(navigation_rail::hide_slide_offset_dp(1.0).abs() < 0.01);
    assert!((navigation_rail::morph_width_hide_dp(0.0)).abs() < 0.01);
    assert!((navigation_rail::morph_width_hide_dp(1.0) - 220.0).abs() < 0.01);
    assert_eq!(navigation_rail::RailArrangement::Top.label(), "top");
    assert_eq!(navigation_rail::RailArrangement::Center.label(), "center");
    assert_eq!(navigation_rail::RailArrangement::Bottom.label(), "bottom");
    assert_eq!(
        navigation_rail::DEFAULT_ARRANGEMENT,
        navigation_rail::RailArrangement::Top
    );
    assert!(navigation_rail::HIDE_DEMO_ARRANGEMENT.is_center());
    assert_eq!(
        navigation_rail::HIDE_DEMO_ARRANGEMENT.justify_content(),
        "center"
    );
    assert!(navigation_rail::HEADER_DEMO_HAS_HEADER);
    assert!(navigation_rail::HEADER_DEMO_HAS_FAB);
    assert!(navigation_rail::WIDE_DEMO_HAS_EXTENDED_FAB);
    assert!(navigation_rail::MODAL_DEMO_HAS_EXTENDED_FAB);
    assert!(navigation_rail::NARROW_DEMO_HAS_EXTENDED_FAB);
    assert!(navigation_rail::HIDE_DEMO_HAS_EXTENDED_FAB);
    let fabh = navigation_rail::fab_morph_hide(&theme, 220.0);
    assert!((fabh.width_dp - 188.0).abs() < 0.01);
    assert!((fabh.label_alpha - 1.0).abs() < 0.01);
    assert_eq!(fabh.label, "Create");
    assert!(navigation_rail::WIDE_DEMO_LAYOUT.in_flow());
    assert_eq!(navigation_rail::FAB_LABEL, "Create");
    assert_eq!(navigation_rail::FAB_GLYPH, "+");
    assert_eq!(navigation_rail::FAB_CORNER_DP, 16.0);
    assert!((navigation_rail::fab_margin_collapsed_dp(96.0) - 20.0).abs() < 0.01);
    assert!((navigation_rail::fab_margin_collapsed_dp(80.0) - 12.0).abs() < 0.01);
    let fab0 = navigation_rail::fab_morph(&theme, 0.0, 96.0);
    assert!((fab0.width_dp - 56.0).abs() < 0.01);
    assert!(fab0.label_alpha < 0.01);
    assert!((fab0.margin_start_dp - 20.0).abs() < 0.01);
    let fab1 = navigation_rail::fab_morph(&theme, 1.0, 220.0);
    assert!((fab1.width_dp - 188.0).abs() < 0.01);
    assert!((fab1.label_alpha - 1.0).abs() < 0.01);
    assert!((fab1.margin_start_dp - 16.0).abs() < 0.01);
    let fabn0 = navigation_rail::fab_morph_kind(
        &theme,
        0.0,
        80.0,
        navigation_rail::RailCollapsedKind::Narrow,
    );
    assert!((fabn0.width_dp - 56.0).abs() < 0.01);
    assert!((fabn0.margin_start_dp - 12.0).abs() < 0.01);
    let fabn1 = navigation_rail::fab_morph_kind(
        &theme,
        1.0,
        220.0,
        navigation_rail::RailCollapsedKind::Narrow,
    );
    assert!((fabn1.width_dp - 188.0).abs() < 0.01);
    assert!((fabn1.margin_start_dp - 16.0).abs() < 0.01);
    assert_eq!(navigation_rail::SHAPE_DP, 0.0);
    assert_eq!(navigation_rail::MODAL_COLLAPSED_SHAPE_DP, 0.0);
    assert_eq!(navigation_rail::MODAL_EXPANDED_SHAPE_DP, 16.0);
    assert_eq!(navigation_rail::MODAL_EXPANDED_SHAPE_DP, theme.shapes.large);
    assert_eq!(navigation_rail::MODAL_EXPANDED_SHAPE_TOKEN, "CornerLarge");
    assert_eq!(navigation_rail::SHAPE_TOKEN, "CornerNone");
    assert_eq!(
        navigation_rail::collapsed_container_color(&theme),
        theme.color.surface
    );
    assert_eq!(
        navigation_rail::modal_container_color(&theme),
        theme.color.surface_container
    );
    assert_eq!(
        navigation_rail::shape_dp_for(navigation_rail::RailExpandedLayout::Standard, true),
        0.0
    );
    assert_eq!(
        navigation_rail::shape_dp_for(navigation_rail::RailExpandedLayout::Modal, true),
        16.0
    );
    assert_eq!(
        navigation_rail::shape_dp_for(navigation_rail::RailExpandedLayout::Modal, false),
        0.0
    );
    assert_eq!(navigation_rail::hide_shape_dp(), 16.0);
    let std0 = navigation_rail::container_morph(
        &theme,
        navigation_rail::RailExpandedLayout::Standard,
        1.0,
    );
    assert_eq!(std0.color, theme.color.surface);
    assert!((std0.corner_dp).abs() < 0.01);
    let modal0 =
        navigation_rail::container_morph(&theme, navigation_rail::RailExpandedLayout::Modal, 0.0);
    assert_eq!(modal0.color, theme.color.surface);
    assert!(modal0.corner_dp.abs() < 0.01);
    let modal1 =
        navigation_rail::container_morph(&theme, navigation_rail::RailExpandedLayout::Modal, 1.0);
    assert_eq!(modal1.color, theme.color.surface_container);
    assert!((modal1.corner_dp - 16.0).abs() < 0.01);
    assert_eq!(
        navigation_rail::resolve_layout(
            &theme,
            navigation_rail::RailMode::Expanded,
            navigation_rail::RailCollapsedKind::Wide,
            navigation_rail::RailExpandedLayout::Modal
        )
        .container,
        theme.color.surface_container
    );
    assert_eq!(
        navigation_rail::resolve_layout(
            &theme,
            navigation_rail::RailMode::Expanded,
            navigation_rail::RailCollapsedKind::Wide,
            navigation_rail::RailExpandedLayout::Standard
        )
        .container,
        theme.color.surface
    );
    assert!(navigation_rail::HEADER_DEMO_ARRANGEMENT.is_bottom());
    assert_eq!(
        navigation_rail::HEADER_DEMO_ARRANGEMENT.justify_content(),
        "flex-end"
    );
    assert!(navigation_rail::HEADER_DEMO_LAYOUT.in_flow());
    assert_eq!(
        navigation_rail::header_menu_glyph(false),
        navigation_rail::HEADER_MENU_GLYPH
    );
    assert_eq!(
        navigation_rail::header_menu_glyph(true),
        navigation_rail::HEADER_MENU_OPEN_GLYPH
    );
    assert_eq!(
        navigation_rail::header_menu_label(false),
        navigation_rail::HEADER_EXPAND_LABEL
    );
    assert_eq!(
        navigation_rail::header_menu_label(true),
        navigation_rail::HEADER_COLLAPSE_LABEL
    );
    assert_eq!(
        navigation_rail::header_state_description(false),
        navigation_rail::HEADER_STATE_COLLAPSED
    );
    assert_eq!(navigation_rail::HEADER_PAD_START_DP, 24.0);
    assert_eq!(navigation_rail::HEADER_BUTTON_DP, 40.0);
    assert_eq!(navigation_rail::HEADER_SPACE_DP, 40.0);
    assert_eq!(
        navigation_rail::header_space_dp(true, navigation_rail::RailArrangement::Top),
        40.0
    );
    assert_eq!(
        navigation_rail::header_space_dp(true, navigation_rail::RailArrangement::Bottom),
        0.0
    );
    assert_eq!(
        navigation_rail::header_space_dp(true, navigation_rail::RailArrangement::Center),
        0.0
    );
    assert!(navigation_rail::RailArrangement::Center.uses_full_height());
    assert!(!navigation_rail::RailArrangement::Bottom.uses_full_height());
    assert_eq!(navigation_rail::HIDE_MENU_LABEL, "Menu");
    assert_eq!(
        navigation_rail::WIDE_DEMO_MODE,
        navigation_rail::RailMode::Collapsed
    );
    let wide_top = navigation_rail::item_morph(&theme, 0.0, 96.0);
    assert_eq!(wide_top.icon_position, navigation_rail::IconPosition::Top);
    assert!((wide_top.dest_width_dp - 96.0).abs() < 0.01);
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
    assert!(date_picker::SELECTABLE_DATES_GRID);
    assert_eq!(
        date_picker::classify_day(2026, 9, 12, selected, today),
        date_picker::DayKind::Disabled
    );
    assert!(cells
        .iter()
        .any(|(d, k)| *d == 12 && *k == date_picker::DayKind::Disabled));
    assert!(!date_picker::day_accepts_tap(
        date_picker::DayKind::Disabled
    ));
    assert_eq!(
        date_picker::apply_docked_tap(2026, 9, 12),
        (
            date_picker::CivilDate {
                year: 2026,
                month: 9,
                day: 12
            },
            false
        )
    );
    assert_eq!(
        date_picker::apply_range_tap(
            date_picker::DateRangeSelection::demo(),
            date_picker::INPUT_ERROR_NOT_ALLOWED_DATE
        ),
        date_picker::DateRangeSelection::demo()
    );
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
        date_picker::DEMO_DISPLAY_MODE,
        date_picker::DatePickerDisplayMode::Input
    );
    assert_eq!(
        date_picker::LIVE_DISPLAY_MODE,
        date_picker::DatePickerDisplayMode::Picker
    );
    assert!(date_picker::SHOW_MODE_TOGGLE);
    assert_eq!(date_picker::TOGGLE_SIZE_DP, 48.0);
    assert_eq!(
        date_picker::apply_display_toggle(date_picker::DatePickerDisplayMode::Picker),
        date_picker::DatePickerDisplayMode::Input
    );
    assert_eq!(
        date_picker::supporting_for(date_picker::DatePickerDisplayMode::Picker),
        None
    );
    assert_eq!(date_picker::YEAR_RANGE_START, 1900);
    assert_eq!(date_picker::YEAR_RANGE_END, 2100);
    assert_eq!(date_picker::YEARS_IN_ROW, 3);
    assert_eq!(date_picker::YEAR_CONTAINER_W_DP, 72.0);
    assert_eq!(date_picker::YEAR_CONTAINER_H_DP, 36.0);
    assert_eq!(date_picker::YEAR_GAP_DP, 16.0);
    assert!(date_picker::YEAR_PICKER_DIVIDER);
    assert_eq!(date_picker::YEAR_PICKER_DIVIDER_H_DP, 1.0);
    assert_eq!(date_picker::year_picker_divider_height_css(), "1px");
    assert!(date_picker::year_picker_divider_visible(
        date_picker::DatePickerPane::Year
    ));
    assert!(!date_picker::year_picker_divider_visible(
        date_picker::DatePickerPane::Calendar
    ));
    assert_eq!(date_picker::year_window(2026)[4], 2026);
    assert_eq!(
        date_picker::classify_year(2026, 2026, 2026),
        date_picker::YearKind::Selected
    );
    assert_eq!(
        date_picker::classify_year(2025, 2026, 2025),
        date_picker::YearKind::Today
    );
    assert_eq!(
        date_picker::LIVE_PANE,
        date_picker::DatePickerPane::Calendar
    );
    assert_eq!(
        date_picker::apply_pane_toggle(date_picker::DatePickerPane::Calendar),
        date_picker::DatePickerPane::Year
    );
    assert_eq!(date_picker::INPUT_PLACEHOLDER, "MM/DD/YYYY");
    assert_eq!(
        date_picker::input_field_value(date_picker::RANGE_DEMO_START),
        "09/15/2026"
    );
    assert_eq!(
        date_picker::parse_input_field("09/15/2026"),
        Some(date_picker::RANGE_DEMO_START)
    );
    assert!(date_picker::is_input_valid("09/15/2026"));
    assert!(!date_picker::is_input_valid("13/40/2026"));
    assert_eq!(date_picker::RANGE_INPUT_HEADLINE, "Enter dates");
    assert_eq!(date_picker::RANGE_START_LABEL, "Start date");
    assert_eq!(date_picker::RANGE_END_LABEL, "End date");
    assert!(date_picker::is_range_input_valid(
        "09/15/2026",
        "09/21/2026"
    ));
    assert!(!date_picker::is_range_input_valid(
        "09/21/2026",
        "09/15/2026"
    ));
    assert!(!date_picker::is_range_input_valid(
        "09/15/1890",
        "09/21/2026"
    ));
    assert!(date_picker::RANGE_INPUT_ERRORS);
    assert_eq!(
        date_picker::INPUT_ERROR_FORMAT,
        "Date format not recognized"
    );
    assert_eq!(
        date_picker::INPUT_ERROR_YEAR_RANGE,
        "Date out of expected year range 1900 - 2100"
    );
    assert_eq!(
        date_picker::RANGE_INPUT_ERROR_ORDER,
        "End date can't be before start date"
    );
    assert_eq!(
        date_picker::range_input_error("13/40/2026", "09/21/2026"),
        date_picker::DateInputError::Format
    );
    assert_eq!(
        date_picker::range_input_error("09/15/1890", "09/21/2026"),
        date_picker::DateInputError::YearRange
    );
    assert_eq!(
        date_picker::range_input_error("09/15/2026", "09/21/2101"),
        date_picker::DateInputError::YearRange
    );
    assert_eq!(
        date_picker::range_input_error("09/21/1890", "09/15/1890"),
        date_picker::DateInputError::YearRange
    );
    assert_eq!(
        date_picker::range_input_error("09/21/2026", "09/15/2026"),
        date_picker::DateInputError::Order
    );
    assert_eq!(
        date_picker::range_input_error("09/15/2026", "09/21/2026"),
        date_picker::DateInputError::None
    );
    assert_eq!(
        date_picker::range_input_error("", ""),
        date_picker::DateInputError::None
    );
    assert!(date_picker::DATE_INPUT_ERRORS);
    assert_eq!(
        date_picker::date_input_error("13/40/2026"),
        date_picker::DateInputError::Format
    );
    assert_eq!(
        date_picker::date_input_error(date_picker::INPUT_ERROR_YEAR_SAMPLE),
        date_picker::DateInputError::YearRange
    );
    assert_eq!(
        date_picker::date_input_error("09/15/2101"),
        date_picker::DateInputError::YearRange
    );
    assert_eq!(
        date_picker::date_input_error("09/15/2026"),
        date_picker::DateInputError::None
    );
    assert!(date_picker::SELECTABLE_DATES);
    assert!(!date_picker::is_selectable_date(
        date_picker::INPUT_ERROR_NOT_ALLOWED_DATE
    ));
    assert!(date_picker::is_selectable_date(
        date_picker::RANGE_DEMO_START
    ));
    assert_eq!(
        date_picker::header_date_label(date_picker::INPUT_ERROR_NOT_ALLOWED_DATE),
        "Sat, Sep 12"
    );
    assert_eq!(
        date_picker::date_input_error(date_picker::INPUT_ERROR_NOT_ALLOWED_SAMPLE),
        date_picker::DateInputError::NotAllowed
    );
    assert_eq!(
        date_picker::range_input_error("09/12/2026", "09/21/2026"),
        date_picker::DateInputError::NotAllowed
    );
    assert_eq!(
        date_picker::range_input_error("09/15/2026", "09/13/2026"),
        date_picker::DateInputError::NotAllowed
    );
    assert!(date_picker::INPUT_EMPTY);
    assert!(date_picker::PICKER_EMPTY);
    assert_eq!(date_picker::INPUT_EMPTY_HEADLINE, "Entered date");
    assert_eq!(date_picker::PICKER_EMPTY_HEADLINE, "Selected date");
    let empty_cells = date_picker::month_grid_unselected(
        2026,
        9,
        date_picker::CivilDate {
            year: 2026,
            month: 9,
            day: 11,
        },
    );
    assert!(!empty_cells
        .iter()
        .any(|(_, k)| *k == date_picker::DayKind::Selected));
    assert!(empty_cells
        .iter()
        .any(|&(d, k)| d == 11 && k == date_picker::DayKind::Today));
    assert!(empty_cells
        .iter()
        .any(|&(d, k)| d == 15 && k == date_picker::DayKind::InMonth));
    assert_eq!(
        date_picker::date_headline(date_picker::DatePickerDisplayMode::Input, None),
        "Entered date"
    );
    assert_eq!(
        date_picker::date_headline(date_picker::DatePickerDisplayMode::Picker, None),
        "Selected date"
    );
    assert_eq!(
        date_picker::date_headline(
            date_picker::DatePickerDisplayMode::Input,
            Some(date_picker::RANGE_DEMO_START)
        ),
        date_picker::header_date_label(date_picker::RANGE_DEMO_START)
    );
    assert!(date_picker::RANGE_EMPTY);
    assert!(date_picker::RANGE_PICKER_EMPTY);
    assert_eq!(date_picker::RANGE_PICKER_TITLE, "Select dates");
    let range_empty_cells = date_picker::month_grid_range_selection(
        2026,
        9,
        date_picker::DateRangeSelection::empty(),
        date_picker::CivilDate {
            year: 2026,
            month: 9,
            day: 11,
        },
    );
    assert!(!range_empty_cells.iter().any(|(_, k)| matches!(
        *k,
        date_picker::DayKind::Selected | date_picker::DayKind::InRange
    )));
    assert_eq!(date_picker::RANGE_EMPTY_HEADLINE, "Start date – End date");
    assert_eq!(
        date_picker::header_range_selection(date_picker::DateRangeSelection::empty()),
        date_picker::RANGE_EMPTY_HEADLINE
    );
    assert!(date_picker::RANGE_START_ONLY);
    assert_eq!(
        date_picker::DateRangeSelection::start_only(),
        date_picker::DateRangeSelection {
            start: Some(date_picker::RANGE_DEMO_START),
            end: None,
        }
    );
    assert_eq!(
        date_picker::header_range_selection(date_picker::DateRangeSelection::start_only()),
        "Sep 15 – End date"
    );
    assert_eq!(
        date_picker::range_field_value(date_picker::DateRangeSelection::start_only(), false),
        "09/15/2026"
    );
    assert_eq!(
        date_picker::range_field_value(date_picker::DateRangeSelection::start_only(), true),
        ""
    );
    assert!(date_picker::RANGE_END_ONLY);
    assert_eq!(
        date_picker::DateRangeSelection::end_only(),
        date_picker::DateRangeSelection {
            start: None,
            end: Some(date_picker::RANGE_DEMO_END),
        }
    );
    assert_eq!(
        date_picker::header_range_selection(date_picker::DateRangeSelection::end_only()),
        date_picker::RANGE_END_ONLY_HEADLINE
    );
    assert_eq!(
        date_picker::range_field_value(date_picker::DateRangeSelection::end_only(), false),
        ""
    );
    assert_eq!(
        date_picker::range_field_value(date_picker::DateRangeSelection::end_only(), true),
        "09/21/2026"
    );
    assert_eq!(
        date_picker::range_month_of(date_picker::DateRangeSelection::end_only()),
        (2026, 9)
    );
    assert_eq!(
        date_picker::date_input_error(""),
        date_picker::DateInputError::None
    );
    assert!(date_picker::is_date_input_valid("09/15/2026"));
    assert!(!date_picker::is_date_input_valid("09/15/1890"));
    assert!(date_picker::range_input_ordered(
        date_picker::RANGE_DEMO_START,
        date_picker::RANGE_DEMO_END
    ));
    assert_eq!(
        date_picker::DatePickerDisplayMode::Picker.toggle(),
        date_picker::DatePickerDisplayMode::Input
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
    assert!(range
        .iter()
        .any(|(d, k)| *d == 18 && *k == date_picker::DayKind::InRange));
    assert!(range
        .iter()
        .any(|(d, k)| *d == 15 && *k == date_picker::DayKind::Selected));
    assert!(date_picker::RANGE_LIVE);
    let demo = date_picker::DateRangeSelection::demo();
    assert_eq!(date_picker::header_range_selection(demo), "Sep 15 – Sep 21");
    let mid = date_picker::apply_range_tap(
        demo,
        date_picker::CivilDate {
            year: 2026,
            month: 9,
            day: 10,
        },
    );
    assert_eq!(mid.end, None);
    assert_eq!(
        date_picker::header_range_selection(mid),
        "Sep 10 – End date"
    );
    let complete = date_picker::apply_range_tap(
        mid,
        date_picker::CivilDate {
            year: 2026,
            month: 9,
            day: 18,
        },
    );
    assert_eq!(
        complete,
        date_picker::DateRangeSelection {
            start: Some(date_picker::CivilDate {
                year: 2026,
                month: 9,
                day: 10,
            }),
            end: Some(date_picker::CivilDate {
                year: 2026,
                month: 9,
                day: 18,
            }),
        }
    );
    let restart = date_picker::apply_range_tap(complete, date_picker::RANGE_DEMO_START);
    assert_eq!(restart.start, Some(date_picker::RANGE_DEMO_START));
    assert_eq!(restart.end, None);
    let empty = date_picker::month_grid_range_selection(
        2026,
        9,
        date_picker::DateRangeSelection::empty(),
        today,
    );
    assert!(empty.iter().any(|(_, k)| *k == date_picker::DayKind::Today));
    assert!(!empty
        .iter()
        .any(|(_, k)| *k == date_picker::DayKind::InRange));
    assert!(date_picker::RANGE_MONTH_NAV);
    assert!(date_picker::RANGE_YEAR_PANE);
    assert!(date_picker::RANGE_SHOW_MODE_TOGGLE);
    assert!(date_picker::RANGE_ACTIONS);
    assert!(date_picker::RANGE_CONNECTOR);
    assert!(date_picker::RANGE_VERTICAL_MONTHS);
    assert_eq!(date_picker::RANGE_VISIBLE_MONTHS, 2);
    assert_eq!(date_picker::MONTH_SUBHEAD_PAD_START_DP, 24.0);
    assert_eq!(date_picker::MONTH_SUBHEAD_PAD_TOP_DP, 20.0);
    assert_eq!(date_picker::MONTH_SUBHEAD_PAD_BOTTOM_DP, 8.0);
    assert!(date_picker::HEADER_PADDINGS);
    assert!(date_picker::CONTAINER_SIZE);
    assert_eq!(date_picker::CONTAINER_W_DP, 360.0);
    assert_eq!(date_picker::CONTAINER_H_DP, 568.0);
    assert!(date_picker::HEADER_CONTAINER_HEIGHTS);
    assert_eq!(date_picker::HEADER_CONTAINER_H_DP, 120.0);
    assert!(date_picker::DATE_ENTRY_DIVIDER);
    assert_eq!(date_picker::DATE_ENTRY_DIVIDER_H_DP, 1.0);
    assert_eq!(date_picker::date_entry_divider_height_css(), "1px");
    assert!(date_picker::date_entry_divider_visible(true, true, true));
    assert!(date_picker::date_entry_divider_visible(true, false, false));
    assert!(date_picker::date_entry_divider_visible(false, true, false));
    assert!(date_picker::date_entry_divider_visible(false, false, true));
    assert!(!date_picker::date_entry_divider_visible(
        false, false, false
    ));
    assert_eq!(date_picker::RANGE_HEADER_CONTAINER_H_DP, 128.0);
    assert_eq!(date_picker::RANGE_HEADER_HEIGHT_OFFSET_DP, 60.0);
    assert_eq!(date_picker::RANGE_HEADER_MIN_H_DP, 68.0);
    assert_eq!(
        date_picker::RANGE_HEADER_MIN_H_DP,
        date_picker::RANGE_HEADER_CONTAINER_H_DP - date_picker::RANGE_HEADER_HEIGHT_OFFSET_DP
    );
    assert_eq!(date_picker::header_min_height_css(), "120px");
    assert_eq!(date_picker::range_header_min_height_css(), "68px");
    assert_eq!(date_picker::TITLE_PAD_START_DP, 24.0);
    assert_eq!(date_picker::TITLE_PAD_END_DP, 12.0);
    assert_eq!(date_picker::TITLE_PAD_TOP_DP, 16.0);
    assert_eq!(date_picker::HEADLINE_PAD_START_DP, 24.0);
    assert_eq!(date_picker::HEADLINE_PAD_END_DP, 12.0);
    assert_eq!(date_picker::HEADLINE_PAD_BOTTOM_DP, 12.0);
    assert_eq!(date_picker::HEADER_PAD_DP, date_picker::TITLE_PAD_START_DP);
    assert_eq!(date_picker::title_padding_css(), "16px 12px 0 24px");
    assert_eq!(date_picker::headline_padding_css(), "0 12px 12px 24px");
    assert!(date_picker::INPUT_FIELD_PADDINGS);
    assert_eq!(date_picker::INPUT_FIELD_PAD_START_DP, 24.0);
    assert_eq!(date_picker::INPUT_FIELD_PAD_END_DP, 24.0);
    assert_eq!(date_picker::INPUT_FIELD_PAD_TOP_DP, 10.0);
    assert_eq!(date_picker::INPUT_FIELD_PAD_BOTTOM_DP, 16.0);
    assert_eq!(
        date_picker::input_field_padding_css(),
        "10px 24px 16px 24px"
    );
    assert!(date_picker::TOGGLE_PADDINGS);
    assert_eq!(date_picker::TOGGLE_PAD_END_DP, 12.0);
    assert_eq!(date_picker::TOGGLE_PAD_BOTTOM_DP, 12.0);
    assert_eq!(date_picker::toggle_padding_css(), "0 12px 12px 0");
    assert!(date_picker::RANGE_HEADER_PADDINGS);
    assert_eq!(date_picker::RANGE_TITLE_PAD_START_DP, 64.0);
    assert_eq!(date_picker::RANGE_TITLE_PAD_END_DP, 12.0);
    assert_eq!(date_picker::RANGE_TITLE_PAD_TOP_DP, 0.0);
    assert_eq!(date_picker::RANGE_HEADLINE_PAD_START_DP, 64.0);
    assert_eq!(date_picker::RANGE_HEADLINE_PAD_END_DP, 12.0);
    assert_eq!(date_picker::RANGE_HEADLINE_PAD_BOTTOM_DP, 12.0);
    assert_eq!(date_picker::range_title_padding_css(), "0px 12px 0 64px");
    assert_eq!(
        date_picker::range_headline_padding_css(),
        "0 12px 12px 64px"
    );
    assert!(date_picker::RANGE_HEADER_CHROME);
    assert!(date_picker::RANGE_HEADER_CLOSE);
    assert_eq!(date_picker::RANGE_HEADER_CLOSE_TARGET_DP, 48.0);
    assert_eq!(date_picker::RANGE_HEADER_CLOSE_INSET_DP, 8.0);
    assert_eq!(
        date_picker::RANGE_HEADER_CLOSE_INSET_DP
            + date_picker::RANGE_HEADER_CLOSE_TARGET_DP
            + date_picker::RANGE_HEADER_CLOSE_INSET_DP,
        date_picker::RANGE_TITLE_PAD_START_DP
    );
    assert_eq!(date_picker::RANGE_HEADER_CLOSE_GLYPH, "✕");
    assert_eq!(date_picker::RANGE_HEADER_CLOSE_LABEL, "Close");
    assert_eq!(
        date_picker::apply_range_header_close(date_picker::DateRangeSelection::empty()),
        date_picker::DateRangeSelection::empty()
    );
    assert!(date_picker::DIALOG_BUTTONS_PADDINGS);
    assert_eq!(date_picker::DIALOG_BUTTONS_PAD_END_DP, 6.0);
    assert_eq!(date_picker::DIALOG_BUTTONS_PAD_BOTTOM_DP, 8.0);
    assert_eq!(date_picker::DIALOG_BUTTONS_MAIN_GAP_DP, 8.0);
    assert_eq!(date_picker::DIALOG_BUTTONS_CROSS_GAP_DP, 12.0);
    assert_eq!(date_picker::dialog_buttons_padding_css(), "0 6px 8px 0");
    assert_eq!(date_picker::month_subhead_label(2026, 9), "September 2026");
    assert_eq!(
        date_picker::range_visible_months(2026, 9),
        [(2026, 9), (2026, 10)]
    );
    assert_eq!(a.month_subhead_style.name, "titleSmall");
    assert_eq!(a.month_subhead, theme.color.on_surface_variant);
    assert_eq!(date_picker::RANGE_DIVIDER_H_DP, 1.0);
    assert!(date_picker::DATE_ACTIONS);
    assert_eq!(date_picker::DATE_DIVIDER_H_DP, 1.0);
    assert!(date_picker::DATE_MONTH_NAV);
    assert!(date_picker::MONTH_YEAR_NAV);
    assert_eq!(date_picker::MONTH_YEAR_H_DP, 56.0);
    assert_eq!(date_picker::MONTH_NAV_ICON_DP, 48.0);
    assert!(date_picker::DOCKED_YEAR_PANE);
    assert!(date_picker::DOCKED_LIVE_SELECT);
    assert!(date_picker::DOCKED_TRAILING);
    assert_eq!(
        date_picker::DOCKED_TRAILING_ICON,
        date_picker::INPUT_TOGGLE_CALENDAR
    );
    assert_eq!(date_picker::DOCKED_TRAILING_TARGET_DP, 48.0);
    assert!(!date_picker::apply_docked_toggle(true));
    assert!(date_picker::apply_docked_toggle(false));
    assert_eq!(date_picker::apply_docked_year(2026, 9, 2027), (2027, 9));
    assert_eq!(
        date_picker::apply_docked_tap(2026, 9, 16),
        (
            date_picker::CivilDate {
                year: 2026,
                month: 9,
                day: 16
            },
            true
        )
    );
    assert_eq!(
        date_picker::apply_docked_year(2026, 9, 1890),
        (date_picker::YEAR_RANGE_START, 9)
    );
    assert_eq!(date_picker::apply_date_month(2026, 9, 1), (2026, 10));
    assert_eq!(
        date_picker::apply_date_month(date_picker::YEAR_RANGE_START, 1, -1),
        (date_picker::YEAR_RANGE_START, 1)
    );
    assert_eq!(
        date_picker::apply_date_month(date_picker::YEAR_RANGE_END, 12, 1),
        (date_picker::YEAR_RANGE_END, 12)
    );
    assert_eq!(
        date_picker::apply_date_confirm(date_picker::RANGE_DEMO_START),
        date_picker::RANGE_DEMO_START
    );
    assert_eq!(
        date_picker::apply_date_dismiss(date_picker::RANGE_DEMO_START),
        date_picker::RANGE_DEMO_START
    );
    assert_eq!(
        date_picker::date_month_of(date_picker::RANGE_DEMO_START),
        (
            date_picker::RANGE_DEMO_START.year,
            date_picker::RANGE_DEMO_START.month
        )
    );
    assert_eq!(
        date_picker::range_fill(date_picker::DayKind::Selected, true, false),
        date_picker::RangeFill::StartHalf
    );
    assert_eq!(
        date_picker::range_fill(date_picker::DayKind::Selected, false, true),
        date_picker::RangeFill::EndHalf
    );
    assert_eq!(
        date_picker::range_fill(date_picker::DayKind::InRange, false, false),
        date_picker::RangeFill::Full
    );
    let demo_fills =
        date_picker::range_fills(2026, 9, range, date_picker::DateRangeSelection::demo());
    let fill_of = |day: u32| {
        range
            .iter()
            .zip(demo_fills.iter())
            .find_map(|((d, _), f)| (*d == day).then_some(*f))
    };
    assert_eq!(fill_of(15), Some(date_picker::RangeFill::StartHalf));
    assert_eq!(fill_of(18), Some(date_picker::RangeFill::Full));
    assert_eq!(fill_of(21), Some(date_picker::RangeFill::EndHalf));
    assert_eq!(
        date_picker::RangeFill::StartHalf.connector_left_dp(date_picker::DAY_DP),
        date_picker::DAY_DP / 2.0
    );
    assert_eq!(
        date_picker::apply_range_dismiss(date_picker::DateRangeSelection::demo()),
        date_picker::DateRangeSelection::demo()
    );
    assert_eq!(
        date_picker::range_month_of(date_picker::DateRangeSelection::empty()),
        (
            date_picker::RANGE_DEMO_START.year,
            date_picker::RANGE_DEMO_START.month
        )
    );
    assert_eq!(
        date_picker::range_title_for(date_picker::DatePickerDisplayMode::Picker),
        date_picker::RANGE_HERO_TITLE
    );
    assert_eq!(
        date_picker::range_title_for(date_picker::DatePickerDisplayMode::Input),
        date_picker::RANGE_INPUT_HEADLINE
    );
    assert_eq!(
        date_picker::range_field_value(date_picker::DateRangeSelection::demo(), true),
        "09/21/2026"
    );
    assert_eq!(date_picker::apply_range_month(2026, 9, 1), (2026, 10));
    assert_eq!(date_picker::apply_range_year(2026, 9, 2027), (2027, 9));
    assert_eq!(
        date_picker::apply_range_year(2026, 9, 1890),
        (date_picker::YEAR_RANGE_START, 9)
    );
    assert_eq!(date_picker::apply_range_month(2026, 1, -1), (2025, 12));
    assert_eq!(
        date_picker::apply_range_month(date_picker::YEAR_RANGE_START, 1, -1),
        (date_picker::YEAR_RANGE_START, 1)
    );
    assert_eq!(
        date_picker::apply_range_month(date_picker::YEAR_RANGE_END, 12, 1),
        (date_picker::YEAR_RANGE_END, 12)
    );
    let cross = date_picker::DateRangeSelection {
        start: Some(date_picker::CivilDate {
            year: 2026,
            month: 9,
            day: 28,
        }),
        end: Some(date_picker::CivilDate {
            year: 2026,
            month: 10,
            day: 5,
        }),
    };
    let oct = date_picker::month_grid_range_selection(2026, 10, cross, today);
    assert!(oct
        .iter()
        .any(|(d, k)| *d == 3 && *k == date_picker::DayKind::InRange));
    assert!(oct
        .iter()
        .any(|(d, k)| *d == 5 && *k == date_picker::DayKind::Selected));
    let sep = date_picker::month_grid_range_selection(2026, 9, cross, today);
    assert!(sep
        .iter()
        .any(|(d, k)| *d == 28 && *k == date_picker::DayKind::Selected));
    assert!(sep
        .iter()
        .any(|(d, k)| *d == 30 && *k == date_picker::DayKind::InRange));
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
    assert_eq!(
        typography::words("Reset settings?"),
        vec!["Reset", "settings?"]
    );
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
    assert_eq!(
        split_button::inner_pressed_dp(button::ButtonSize::Small),
        12.0
    );
    assert_eq!(
        split_button::trailing_icon_dp(button::ButtonSize::Small),
        22.0
    );
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
    assert_eq!(menu::SPLIT_ITEMS[0].label, split_button::DEMO_MENU[0]);
    assert_eq!(menu::SPLIT_ITEMS[1].label, split_button::DEMO_MENU[1]);
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
    assert_eq!(button_group::STANDARD_GAP_DP, 12.0);
    assert_eq!(button_group::EXPANDED_RATIO, 0.15);
    assert_eq!(button_group::STANDARD_SEGMENTS[1], "Center");
    let widths = button_group::standard_widths(3, Some(1), 88.0);
    assert!((widths[1] - 101.2).abs() < 0.01);
    assert!((widths[0] - 81.4).abs() < 0.01);
    assert!((widths[2] - 81.4).abs() < 0.01);
    assert!((widths.iter().sum::<f32>() - 264.0).abs() < 0.01);
    let std_sel = button_group::resolve_standard_scene(&theme, 1, 1);
    assert_eq!(std_sel.container, theme.color.primary);
    assert_eq!(std_sel.corners.top_left, 12.0);
    assert_eq!(std_sel.label_style.name, "labelLargeEmphasized");
    let std_idle = button_group::resolve_standard(&theme, 0, 3, false, false, Some(1));
    assert_eq!(std_idle.container, theme.color.secondary_container);
    assert_eq!(std_idle.corners.top_left, 20.0);
    assert_eq!(
        button_group::STANDARD_OVERFLOW_ITEMS,
        ["Left", "Right", "Justify"]
    );
    assert_eq!(
        button_group::STANDARD_OVERFLOW_GLYPH,
        button_group::OVERFLOW_GLYPH
    );
    let ov = button_group::resolve_standard_overflow(&theme, false);
    assert_eq!(ov.container, theme.color.primary);
    assert_eq!(ov.content, theme.color.on_primary);
    assert_eq!(ov.corners.top_left, 20.0);
    assert_eq!(ov.height_dp, 40.0);
    let ov_press = button_group::resolve_standard_overflow(&theme, true);
    assert_eq!(ov_press.corners.top_left, 8.0);
}

#[test]
fn icon_button_expressive_width_axis() {
    use icon_button::IconButtonWidth;
    let theme = Theme::light();
    let expected = [
        (
            button::ButtonSize::ExtraSmall,
            [
                (IconButtonWidth::Narrow, 28.0),
                (IconButtonWidth::Default, 32.0),
                (IconButtonWidth::Wide, 40.0),
            ],
        ),
        (
            button::ButtonSize::Small,
            [
                (IconButtonWidth::Narrow, 32.0),
                (IconButtonWidth::Default, 40.0),
                (IconButtonWidth::Wide, 52.0),
            ],
        ),
        (
            button::ButtonSize::Medium,
            [
                (IconButtonWidth::Narrow, 48.0),
                (IconButtonWidth::Default, 56.0),
                (IconButtonWidth::Wide, 72.0),
            ],
        ),
        (
            button::ButtonSize::Large,
            [
                (IconButtonWidth::Narrow, 64.0),
                (IconButtonWidth::Default, 96.0),
                (IconButtonWidth::Wide, 128.0),
            ],
        ),
        (
            button::ButtonSize::ExtraLarge,
            [
                (IconButtonWidth::Narrow, 104.0),
                (IconButtonWidth::Default, 136.0),
                (IconButtonWidth::Wide, 184.0),
            ],
        ),
    ];
    for (size, widths) in expected {
        for (width, want) in widths {
            assert_eq!(icon_button::container_width_dp(size, width), want);
            let a = icon_button::resolve_width(
                &theme,
                icon_button::IconButtonVariant::Filled,
                size,
                button::ButtonShape::Round,
                width,
                InteractionState::Enabled,
            );
            assert_eq!(a.width_dp, Some(want));
            assert_eq!(a.height_dp, size.height_dp());
            assert_eq!(a.pad_start_dp, icon_button::pad_h_dp(size, width));
            assert_eq!(a.pad_end_dp, a.pad_start_dp);
            assert_eq!(a.min_width_dp, Some(icon_button::TARGET_DP.max(want)));
        }
    }
    let narrow_s = icon_button::resolve_width(
        &theme,
        icon_button::IconButtonVariant::Filled,
        icon_button::WIDTH_HERO_SIZE,
        button::ButtonShape::Round,
        IconButtonWidth::Narrow,
        InteractionState::Enabled,
    );
    assert_eq!(narrow_s.width_dp, Some(32.0));
    assert_eq!(narrow_s.height_dp, 40.0);
    assert_eq!(narrow_s.pad_start_dp, 4.0);
    assert_eq!(narrow_s.min_width_dp, Some(48.0));
    let wide_m = icon_button::resolve_width(
        &theme,
        icon_button::IconButtonVariant::Filled,
        icon_button::WIDTH_HERO_SIZE_MEDIUM,
        button::ButtonShape::Round,
        IconButtonWidth::Wide,
        InteractionState::Enabled,
    );
    assert_eq!(wide_m.width_dp, Some(72.0));
    assert_eq!(wide_m.height_dp, 56.0);
    assert_eq!(wide_m.pad_start_dp, 24.0);
    assert_eq!(wide_m.corners.top_left, 28.0);
}

#[test]
fn icon_button_expressive_toggle_selection() {
    use icon_button::IconButtonSelection;
    let theme = Theme::light();
    let size = icon_button::TOGGLE_HERO_SIZE;
    let filled_off = icon_button::resolve_toggle(
        &theme,
        icon_button::IconButtonVariant::Filled,
        size,
        button::ButtonShape::Round,
        false,
        InteractionState::Enabled,
    );
    assert_eq!(filled_off.container, theme.color.surface_container);
    assert_eq!(filled_off.content, theme.color.on_surface_variant);
    assert_eq!(filled_off.corners.top_left, 20.0);
    assert_eq!(filled_off.outline, None);
    assert_eq!(IconButtonSelection::Unselected.glyph(), "☆");

    let filled_on = icon_button::resolve_toggle(
        &theme,
        icon_button::IconButtonVariant::Filled,
        size,
        button::ButtonShape::Round,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(filled_on.container, theme.color.primary);
    assert_eq!(filled_on.content, theme.color.on_primary);
    assert_eq!(filled_on.corners.top_left, 12.0);
    assert_eq!(IconButtonSelection::Selected.glyph(), "★");
    assert_eq!(
        icon_button::resting_shape(button::ButtonShape::Round, IconButtonSelection::Selected),
        button::ButtonShape::Square
    );

    let square_on = icon_button::resolve_toggle(
        &theme,
        icon_button::IconButtonVariant::Filled,
        size,
        button::ButtonShape::Square,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(square_on.corners.top_left, 20.0);

    let pressed = icon_button::resolve_toggle(
        &theme,
        icon_button::IconButtonVariant::Filled,
        size,
        button::ButtonShape::Round,
        true,
        InteractionState::Pressed,
    );
    assert_eq!(pressed.corners.top_left, 8.0);

    let tonal_on = icon_button::resolve_toggle(
        &theme,
        icon_button::IconButtonVariant::Tonal,
        size,
        button::ButtonShape::Round,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(tonal_on.container, theme.color.secondary);
    assert_eq!(tonal_on.content, theme.color.on_secondary);

    let outlined_on = icon_button::resolve_toggle(
        &theme,
        icon_button::IconButtonVariant::Outlined,
        size,
        button::ButtonShape::Round,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(outlined_on.container, theme.color.inverse_surface);
    assert_eq!(outlined_on.content, theme.color.inverse_on_surface);
    assert_eq!(outlined_on.outline, None);

    let standard_on = icon_button::resolve_toggle(
        &theme,
        icon_button::IconButtonVariant::Standard,
        size,
        button::ButtonShape::Round,
        true,
        InteractionState::Enabled,
    );
    assert_eq!(standard_on.content, theme.color.primary);
    assert_eq!(standard_on.container, theme.color.surface);

    let default_filled = icon_button::resolve(
        &theme,
        icon_button::IconButtonVariant::Filled,
        InteractionState::Enabled,
    );
    assert_eq!(default_filled.container, theme.color.primary);
    assert_eq!(default_filled.corners.top_left, 20.0);
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
    assert!(!ed.focused);
    assert_eq!(
        search::list_status("", true),
        search::SearchListStatus::Suggestions
    );
    assert_eq!(
        search::list_status(search::DEMO_QUERY, true),
        search::SearchListStatus::QuickResults
    );
    assert_eq!(
        search::list_status("App", false),
        search::SearchListStatus::Results
    );
    assert_eq!(search::DOCKED_MIN_H_DP, 240.0);
    assert!((search::docked_max_h_dp(720.0) - 480.0).abs() < 0.01);
    assert_eq!(search::docked_height_dp(100.0, 720.0), 240.0);
    assert_eq!(search::docked_height_dp(500.0, 720.0), 480.0);
    assert_eq!(search::docked_width_dp(800.0), 720.0);
    assert_eq!(search::SCRIM_OPACITY, 0.32);
    assert!(search::uses_docked_scrim(
        search::SearchExpandedLayout::Docked,
        true
    ));
    assert!(!search::uses_docked_scrim(
        search::SearchExpandedLayout::FullScreen,
        true
    ));
    assert!(search::dismiss_on_scrim());
    assert_eq!(search::ROW_GAP_DP, 2.0);
    assert_eq!(search::row_corners(0, 3, false).top_left, 16.0);
    assert_eq!(search::row_corners(1, 3, false).top_left, 4.0);
    assert_eq!(search::row_corners(0, 1, true).top_left, 16.0);
    assert!(search::shows_clear(search::DEMO_QUERY));
    assert!(!search::shows_clear(""));
    assert!(!search::shows_clear("   "));
    assert_eq!(search::trailing_action(""), search::TRAILING_MIC);
    assert_eq!(
        search::trailing_action(search::DEMO_QUERY),
        search::TRAILING_CLEAR
    );
    search::apply_clear(&mut ed);
    assert_eq!(ed.value(), "");
    assert!(ed.focused);
    assert_eq!(
        search::SearchListStatus::QuickResults.heading(),
        Some(search::QUICK_RESULTS_LABEL)
    );
    assert_eq!(
        search::SearchListStatus::Results.heading(),
        Some(search::RESULTS_LABEL)
    );
    assert_eq!(
        search::status_live_text(search::SearchListStatus::QuickResults, 1),
        "1 quick result"
    );
    assert_eq!(
        search::status_live_text(search::SearchListStatus::Results, 1),
        "1 result"
    );
    assert_eq!(
        search::status_chrome_h_dp(search::SearchListStatus::Suggestions),
        0.0
    );
    assert_eq!(
        search::status_chrome_h_dp(search::SearchListStatus::QuickResults),
        32.0
    );
    assert!(
        (search::expanded_list_h_dp(search::DEMO_QUERY, true)
            - (search::STATUS_H_DP + search::FILTER_ROW_H_DP + search::RESULT_H_DP))
            .abs()
            < 0.01
    );
    assert_eq!(search::RESULT_H_DP, 72.0);
    assert!(search::filter_suggestions(search::DEMO_EMPTY_QUERY).is_empty());
    assert!(search::shows_empty(search::DEMO_EMPTY_QUERY));
    assert!(!search::shows_empty(search::DEMO_QUERY));
    assert_eq!(search::EMPTY_H_DP, 56.0);
    assert_eq!(
        search::status_live_text(search::SearchListStatus::Results, 0),
        "0 results"
    );
    assert!(
        (search::expanded_list_h_dp(search::DEMO_EMPTY_QUERY, false)
            - (search::STATUS_H_DP + search::FILTER_ROW_H_DP + search::EMPTY_H_DP))
            .abs()
            < 0.01
    );
    assert!(search::SearchListStatus::QuickResults.shows_filters());
    assert!(!search::SearchListStatus::Suggestions.shows_filters());
    assert_eq!(search::FILTER_CHIP_H_DP, 32.0);
    assert_eq!(search::FILTER_ROW_H_DP, 48.0);
    assert!(search::SearchFilter::Apps.matches("App"));
    assert!(!search::SearchFilter::Settings.matches("App"));
    assert_eq!(search::item_category("App"), "apps");
    assert_eq!(search::item_category("Recent search"), "all");
    assert!(search::filter_suggestions_in(search::DEMO_QUERY, search::DEMO_FILTER).is_empty());
    assert!(search::shows_empty_in(
        search::DEMO_QUERY,
        search::DEMO_FILTER
    ));
    assert_eq!(search::supporting_for("App"), "Installed application");
    assert_eq!(search::ROW_LEADING_AVATAR_DP, 40.0);
    assert_eq!(search::ROW_LEADING_ICON_DP, 20.0);
    assert_eq!(
        search::row_leading_kind(search::SearchListStatus::Results, "App"),
        search::RowLeadingKind::Avatar
    );
    assert_eq!(
        search::row_leading_kind(search::SearchListStatus::QuickResults, "Shortcut"),
        search::RowLeadingKind::Icon
    );
    assert_eq!(
        search::row_leading_kind(search::SearchListStatus::Suggestions, "App"),
        search::RowLeadingKind::Icon
    );
    assert_eq!(
        search::row_leading_size_dp(
            search::SearchListStatus::Results,
            search::RowLeadingKind::Avatar
        ),
        40.0
    );
    assert_eq!(search::RESULT_OPEN, "↗");
    assert!(search::SearchListStatus::QuickResults.uses_two_line_rows());
    assert!(search::SearchListStatus::Results.shows_open_affordance());
    assert!(!search::SearchListStatus::Suggestions.uses_two_line_rows());
    assert_eq!(
        search::resolve_view(&theme).result_supporting_style.name,
        "bodyMedium"
    );
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
    assert_eq!(
        time_picker::DEMO_PERIOD.toggle(),
        time_picker::DayPeriod::Am
    );
    assert_eq!(time_picker::DEMO_DIAL, time_picker::DialFace::Hour);
    assert_eq!(
        time_picker::DEMO_LAYOUT,
        time_picker::TimePickerLayoutType::Vertical
    );
    assert_eq!(
        time_picker::DEMO_DESKTOP_LAYOUT,
        time_picker::TimePickerLayoutType::Horizontal
    );
    assert_eq!(
        time_picker::period_w_dp(time_picker::TimePickerLayoutType::Horizontal),
        216.0
    );
    assert_eq!(
        time_picker::period_h_dp(time_picker::TimePickerLayoutType::Horizontal),
        38.0
    );
    assert_eq!(time_picker::HORIZONTAL_GAP_DP, 24.0);
    assert!(time_picker::CLOCK_FACE_MARGINS);
    assert_eq!(time_picker::CLOCK_DISPLAY_BOTTOM_MARGIN_DP, 36.0);
    assert_eq!(time_picker::CLOCK_FACE_BOTTOM_MARGIN_DP, 24.0);
    assert!(time_picker::DISPLAY_SEPARATOR);
    assert_eq!(time_picker::DISPLAY_SEPARATOR_W_DP, 24.0);
    assert_eq!(time_picker::DISPLAY_SEPARATOR_H_DP, 80.0);
    assert_eq!(time_picker::INPUT_DISPLAY_SEPARATOR_H_DP, 72.0);
    assert!(time_picker::PERIOD_TOGGLE_MARGIN);
    assert_eq!(time_picker::PERIOD_TOGGLE_MARGIN_DP, 12.0);
    assert_eq!(
        time_picker::DEMO_STYLE,
        time_picker::TimePickerStyle::Scroll
    );
    assert_eq!(search::DEMO_STYLE, search::SearchStyle::Contained);
    assert!(search::DEMO_STYLE.recommended());
    assert!(!search::DEMO_STYLE.shows_divider());
    assert_eq!(search::contained_margin_dp(false), 24.0);
    assert_eq!(search::contained_margin_dp(true), 12.0);
    assert_eq!(search::contained_corner_dp(true), 28.0);
    assert_eq!(search::SUGGESTION_GROUPS.len(), 2);
    assert_eq!(search::SUGGESTION_GROUPS[0].title, "Recent");
    assert_eq!(search::SUGGESTION_GROUP_GAP_DP, 8.0);
    assert_eq!(search::filter_grouped_suggestions("").len(), 2);
    assert_eq!(search::filter_grouped_suggestions("set").len(), 1);
    assert_eq!(search::suggestion_group_chrome_h_dp(2), 72.0);
    assert_eq!(search::filter_suggestions(""), search::SUGGESTIONS.to_vec());
    let contained = search::contained_frame_at(1.0, search::contained_suggestion_count());
    assert!((contained.corner_dp - 28.0).abs() < 0.01);
    assert!((contained.margin_dp - 12.0).abs() < 0.01);
    assert!(contained.height_dp > search::HEIGHT_DP);
    assert_eq!(
        search::WindowWidthClass::from_width_dp(359.0),
        search::WindowWidthClass::Compact
    );
    assert_eq!(
        search::WindowWidthClass::from_width_dp(600.0),
        search::WindowWidthClass::Medium
    );
    assert_eq!(
        search::DEMO_WIDTH_CLASS.expanded_search(),
        search::SearchExpandedLayout::FullScreen
    );
    let compact = search::contained_frame_at_layout(
        search::SearchExpandedLayout::FullScreen,
        1.0,
        search::contained_suggestion_count(),
    );
    assert!((compact.corner_dp - 0.0).abs() < 0.01);
    assert!((compact.margin_dp - 0.0).abs() < 0.01);
    assert!(compact.height_dp >= search::ACTIVITY_MIN_H_DP);
    assert_eq!(
        search::contained_container(&theme),
        theme.color.surface_container_high
    );
    assert_eq!(
        time_picker::DEMO_DISPLAY_MODE,
        time_picker::TimePickerDisplayMode::Input
    );
    assert_eq!(
        time_picker::DEMO_DISPLAY_MODE.toggle(),
        time_picker::TimePickerDisplayMode::Scroll
    );
    assert_eq!(time_picker::DEMO_FORMAT, time_picker::TimeFormat::Hour24);
    assert!(time_picker::DEMO_FORMAT.is_24_hour());
    assert!(!time_picker::DEMO_FORMAT.shows_period());
    assert_eq!(time_picker::demo_hour(time_picker::DEMO_FORMAT), 18);
    assert_eq!(time_picker::to_hour24(6, time_picker::DayPeriod::Pm), 18);
    assert_eq!(time_picker::to_hour12(18), (6, time_picker::DayPeriod::Pm));
    assert_eq!(time_picker::to_hour24(12, time_picker::DayPeriod::Am), 0);
    assert_eq!(time_picker::to_hour12(0), (12, time_picker::DayPeriod::Am));
    assert_eq!(
        time_picker::format_hour_field_for(18, time_picker::TimeFormat::Hour24),
        "18"
    );
    assert_eq!(
        time_picker::format_hour_field_for(0, time_picker::TimeFormat::Hour24),
        "00"
    );
    assert_eq!(
        time_picker::header_label_for(
            18,
            30,
            time_picker::DayPeriod::Pm,
            time_picker::TimeFormat::Hour24
        ),
        "18:30"
    );
    assert_eq!(
        time_picker::TimePickerDisplayMode::Scroll.toggle_icon(),
        time_picker::KEYBOARD_ICON
    );
    assert_eq!(
        time_picker::TimePickerDisplayMode::Input.toggle_icon(),
        time_picker::SCHEDULE_ICON
    );
    let input_a = time_picker::resolve_input(&theme);
    assert_eq!(input_a.container, theme.color.primary_container);
    assert_eq!(input_a.field_w_dp, time_picker::INPUT_FIELD_W_DP);
    assert_eq!(input_a.field_h_dp, time_picker::INPUT_FIELD_H_DP);
    assert_eq!(
        input_a.field_corners.top_left,
        time_picker::INPUT_FIELD_CORNER_DP
    );
    assert_eq!(input_a.field_style.name, "displayLargeEmphasized");
    assert!(time_picker::SUPPORT_LABEL);
    assert_eq!(time_picker::SUPPORT_LABEL_TOP_DP, 7.0);
    assert_eq!(time_picker::support_label_top_css(), "7px");
    assert_eq!(
        time_picker::ScrollKind::Hour.support_label(),
        time_picker::INPUT_HOUR_LABEL
    );
    assert_eq!(
        time_picker::ScrollKind::Minute.support_label(),
        time_picker::INPUT_MINUTE_LABEL
    );
    assert_eq!(input_a.support_label, theme.color.on_surface_variant);
    assert_eq!(input_a.support_label_style.name, "bodySmall");
    let mut typed = time_picker::TimeInputState::demo();
    assert_eq!(
        typed.hour_value(),
        Some(time_picker::demo_hour(time_picker::DEMO_FORMAT))
    );
    assert_eq!(typed.minute_value(), Some(time_picker::DEMO_MINUTE));
    assert!(typed.is_input_valid());
    typed.apply_key("backspace");
    typed.apply_key("backspace");
    typed.apply_key("9");
    assert_eq!(typed.hour_value(), Some(9));
    assert_eq!(typed.focus, time_picker::ScrollKind::Minute);
    let mut midnight =
        time_picker::TimeInputState::from_clock(0, 0, time_picker::TimeFormat::Hour24);
    assert_eq!(midnight.hour.display(), "00");
    assert_eq!(midnight.hour_value(), Some(0));
    midnight.apply_key("backspace");
    midnight.apply_key("backspace");
    midnight.apply_key("2");
    midnight.apply_key("3");
    assert_eq!(midnight.hour_value(), Some(23));
    let mut scroll = time_picker::TimeScrollState::demo();
    let mut hour = time_picker::DEMO_HOUR;
    let mut minute = time_picker::DEMO_MINUTE;
    let next = time_picker::apply_display_toggle(
        time_picker::TimePickerDisplayMode::Scroll,
        &mut scroll,
        &mut typed,
        &mut hour,
        &mut minute,
    );
    assert_eq!(next, time_picker::TimePickerDisplayMode::Input);
    assert_eq!(time_picker::TimePickerStyle::Input.label(), "input");
    let scroll = time_picker::resolve_scroll(&theme);
    assert_eq!(scroll.container, theme.color.primary_container);
    assert_eq!(
        scroll.field_container,
        theme.color.surface_container_highest
    );
    assert_eq!(scroll.field_h_dp, time_picker::SCROLL_FIELD_H_DP);
    assert_eq!(
        scroll.field_corners.top_left,
        time_picker::SCROLL_FIELD_CORNER_DP
    );
    assert_eq!(scroll.selected_style.name, "displayLargeEmphasized");
    assert_eq!(scroll.unselected_style.name, "displayMedium");
    let mut wheel = time_picker::TimeScrollState::demo();
    assert_eq!(
        wheel.hour_value(),
        time_picker::demo_hour(time_picker::DEMO_FORMAT)
    );
    assert_eq!(wheel.minute_value(), time_picker::DEMO_MINUTE);
    assert!((time_picker::hour_index(6) - 5.0).abs() < 1e-5);
    assert_eq!(time_picker::hour_from_index(5), 6);
    assert!((time_picker::hour_index_for(18, time_picker::TimeFormat::Hour24) - 18.0).abs() < 1e-5);
    assert_eq!(
        time_picker::hour_from_index_for(18, time_picker::TimeFormat::Hour24),
        18
    );
    assert_eq!(time_picker::minute_from_index(30), 30);
    assert_eq!(time_picker::wrap_index(-1, 12), 11);
    assert!((time_picker::wrap_offset(-0.25, 12) - 11.75).abs() < 1e-5);
    wheel.hour.apply_delta_dp(-time_picker::SCROLL_ITEM_H_DP);
    assert_eq!(wheel.hour.selected_value(), 19);
    let mut wrap_h = time_picker::ScrollField::hour(12);
    wrap_h.apply_delta_dp(-time_picker::SCROLL_ITEM_H_DP);
    assert_eq!(wrap_h.selected_value(), 1);
    let mut wrap24 = time_picker::ScrollField::hour_with(23, time_picker::TimeFormat::Hour24);
    wrap24.apply_delta_dp(-time_picker::SCROLL_ITEM_H_DP);
    assert_eq!(wrap24.selected_value(), 0);
    let mut wrap24_zero = time_picker::ScrollField::hour_with(0, time_picker::TimeFormat::Hour24);
    wrap24_zero.apply_delta_dp(time_picker::SCROLL_ITEM_H_DP);
    assert_eq!(wrap24_zero.selected_value(), 23);
    let mut fmt = time_picker::TimeFormat::Hour12;
    let mut period = time_picker::DayPeriod::Pm;
    let mut scroll12 =
        time_picker::TimeScrollState::from_clock(6, 30, time_picker::TimeFormat::Hour12);
    let mut input12 =
        time_picker::TimeInputState::from_clock(6, 30, time_picker::TimeFormat::Hour12);
    let mut hour12 = 6u8;
    fmt = time_picker::apply_format_toggle(
        fmt,
        &mut period,
        &mut scroll12,
        &mut input12,
        &mut hour12,
        30,
    );
    assert_eq!(fmt, time_picker::TimeFormat::Hour24);
    assert_eq!(hour12, 18);
    assert!(!fmt.shows_period());
    assert_eq!(scroll12.hour_value(), 18);
    assert_eq!(input12.hour_value(), Some(18));
    time_picker::apply_wheel(&mut wheel.minute, time_picker::SCROLL_ITEM_H_DP);
    wheel.step_until_rest();
    assert!(wheel.resting());
    assert!((0..=59).contains(&wheel.minute_value()));
    let slots = time_picker::ScrollField::hour(6).slots();
    assert_eq!(
        slots.len(),
        (time_picker::SCROLL_SLOT_SPAN * 2 + 1) as usize
    );
    assert!(slots.iter().any(|s| s.selected && s.value == 6));
    assert_eq!(time_picker::TimePickerStyle::Scroll.label(), "scroll");
    assert_eq!(
        time_picker::hour_ring(18, time_picker::TimeFormat::Hour24),
        time_picker::DialRing::Inner
    );
    assert_eq!(
        time_picker::hour_ring(6, time_picker::TimeFormat::Hour24),
        time_picker::DialRing::Outer
    );
    assert_eq!(
        time_picker::hour_ring(0, time_picker::TimeFormat::Hour24),
        time_picker::DialRing::Outer
    );
    assert_eq!(
        time_picker::hour_ring(12, time_picker::TimeFormat::Hour24),
        time_picker::DialRing::Inner
    );
    assert!((time_picker::OUTER_CIRCLE_RADIUS_DP - 101.0).abs() < 0.01);
    assert!((time_picker::INNER_CIRCLE_RADIUS_DP - 69.0).abs() < 0.01);
    assert!(
        (time_picker::time_selector_w_dp(time_picker::TimeFormat::Hour24) - 114.0).abs() < 0.01
    );
    assert!((time_picker::time_selector_w_dp(time_picker::TimeFormat::Hour12) - 96.0).abs() < 0.01);
    assert_eq!(
        time_picker::dial_clock_hour(
            18,
            time_picker::TimeFormat::Hour24,
            time_picker::DayPeriod::Pm
        ),
        18
    );
    assert_eq!(
        time_picker::hour_from_dial(
            18,
            time_picker::DayPeriod::Pm,
            time_picker::TimeFormat::Hour24
        ),
        18
    );
    assert_eq!(
        time_picker::select_hour_for(6, 0, time_picker::TimeFormat::Hour24),
        0
    );
    assert_eq!(
        time_picker::hour_label(0, time_picker::TimeFormat::Hour24),
        "00"
    );
    assert_eq!(
        time_picker::hour_label(18, time_picker::TimeFormat::Hour24),
        "18"
    );
    let cells24 = time_picker::hour_cells(time_picker::TimeFormat::Hour24, 256.0, 48.0);
    assert_eq!(cells24.len(), 24);
    assert!(cells24
        .iter()
        .any(|c| c.hour == 18 && c.ring == time_picker::DialRing::Inner));
    assert!(cells24
        .iter()
        .any(|c| c.hour == 0 && c.ring == time_picker::DialRing::Outer));
    let (x0, y0) = time_picker::hour_offset_for(0, time_picker::TimeFormat::Hour24, 256.0, 48.0);
    let (_x12, y12) =
        time_picker::hour_offset_for(12, time_picker::TimeFormat::Hour24, 256.0, 48.0);
    assert!(x0 > 80.0 && x0 < 130.0, "00 sits top-center, x={x0}");
    assert!(y0 < 20.0, "00 sits on outer top, y={y0}");
    assert!(y12 > y0 + 20.0, "12 sits on inner top, y12={y12} y0={y0}");
    let (x18, y18) = time_picker::hour_offset_for(18, time_picker::TimeFormat::Hour24, 256.0, 48.0);
    let (x6, y6) = time_picker::hour_offset_for(6, time_picker::TimeFormat::Hour24, 256.0, 48.0);
    assert!(
        y18 > 140.0 && y18 < y6,
        "18 inner bottom vs 6 outer, y18={y18} y6={y6}"
    );
    assert!((x18 - x6).abs() < 8.0);
    let inner_r = time_picker::selector_radius_dp(
        time_picker::DialFace::Hour,
        18,
        time_picker::TimeFormat::Hour24,
        256.0,
    );
    assert!((inner_r - 69.0).abs() < 0.01);
    let (x, y) = time_picker::hour_offset(12, 256.0, 48.0);
    assert!(
        x > 80.0 && x < 130.0,
        "12 should sit near top center, x={x}"
    );
    assert!(y < 20.0, "12 should sit near top, y={y}");
    let (mx, my) = time_picker::minute_offset(30, 256.0, 48.0);
    assert!(mx > 80.0 && mx < 130.0, "30 sits bottom-center-ish x={mx}");
    assert!(my > 180.0, "30 sits near bottom, y={my}");
    assert!(
        (time_picker::hand_angle_deg(time_picker::DialFace::Minute, 6, 30) - 180.0).abs() < 0.01
    );
    let quad = time_picker::hand_quad(256.0, time_picker::DialFace::Minute, 6, 30, 48.0);
    assert_eq!(quad.len(), 4);
    assert!(
        time_picker::hand_svg_d(256.0, time_picker::DialFace::Minute, 6, 30, 48.0).starts_with('M')
    );
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
    assert!(navigation_rail::is_modal(
        navigation_rail::RailMode::Expanded
    ));
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
    let layer_box = search::morph_layer_box(search::MORPH_STAGE_W_DP, docked.height_dp, layer);
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
    assert!(navigation_rail::focus_trapped(
        navigation_rail::RailMode::Expanded
    ));
    assert!(navigation_rail::dismiss_on_scrim());
    assert_eq!(
        navigation_rail::modal_elevation_dp(&theme),
        theme.elevation.level2
    );
    let win = text_field::ime_caret_rect_in_window(8.0, 16.0, 3, 16.0);
    assert_eq!(win.2, text_field::IME_CARET_W_DP);
    let mut focused_ed =
        text_field::TextFieldEditor::new(text_field::TextFieldVariant::Outlined, "ab");
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
    assert!(
        (paint.left + paint.handle_w + paint.active + paint.handle_w + paint.right - 280.0).abs()
            < 1.0
    );
    let pts = progress::ptr_arc_polyline(40.0, 4.0, 90.0, 0.25);
    assert!(pts.len() > 4);
    let caps = progress::ptr_cap_centers(40.0, 4.0, 90.0, 0.25);
    assert_eq!(caps.len(), 2);
    assert_eq!(
        progress::clock_ms(&theme),
        theme.motion.effects_default_ms * 6
    );
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
    assert_eq!(
        carousel::apply_wheel(0, 96.0, 0.0),
        carousel::advance(0, carousel::inertial_steps(96.0, 0.0))
    );
    assert!((carousel::item_width_during_fling(0, 0, 0.0) - carousel::LARGE_W_DP).abs() < 0.01);
    assert!(carousel::item_width_during_fling(0, 0, 0.5) < carousel::LARGE_W_DP);
    assert!(carousel::item_width_during_fling(1, 0, 0.5) > carousel::SMALL_W_DP);
    let mut snap = carousel::FlingState::new(0);
    snap.leftover = carousel::FLING_UNIT * 0.6;
    snap.settle();
    assert_eq!(snap.selected, 1);
    assert!(snap.resting());
    assert_eq!(carousel::FLING_FRAME_DT, gpui_material::motion::FRAME_DT);
    assert_eq!(
        time_picker::SECOND_HAND_FRAME_MS,
        gpui_material::motion::FRAME_MS
    );
    assert!(progress::loading_svg_d(38.0, 0.0).starts_with('M'));
    assert!(progress::loading_svg_values(38.0, 4).contains(';'));
    let sausage = progress::round_capped_arc_polygon(48.0, 4.0, -90.0, 90.0);
    assert!(sausage.len() > 20);
    assert_eq!(
        progress::contained_loading_indicator(&theme).contained,
        true
    );
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
    assert!((navigation_rail::morph_width_dp(0.0) - 96.0).abs() < 0.01);
    assert!((navigation_rail::morph_width_narrow_dp(0.0) - 80.0).abs() < 0.01);
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
        !poly
            .iter()
            .any(|(x, y)| (*x - gap_mid).abs() < 6.0 && *y < 0.4),
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
    assert!(navigation_rail::overlay_window(
        navigation_rail::RailMode::Expanded
    ));
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
    assert!(PhotoKind::Mugs
        .css_background()
        .contains("url('data:image/jpeg"));
    for kind in PhotoKind::ALL {
        assert!(kind.is_licensed_camera(), "{}", kind.label());
        assert!(!kind.credit().license.is_empty());
        assert!(kind.jpeg_bytes().len() > 1000, "{}", kind.label());
        assert_eq!(&kind.jpeg_bytes()[0..2], &[0xFF, 0xD8]);
    }
    assert_eq!(PhotoKind::Bloom.credit().license, "CC0");
    assert_eq!(PhotoKind::Bloom.credit().source, "commons");
    assert_eq!(PhotoKind::Basket.credit().license, "CC BY-SA 4.0");
    assert_eq!(
        PhotoKind::PortraitCarmen.credit().license,
        "Unsplash License"
    );
    assert_eq!(PhotoKind::Lake.credit().license, "Public domain");
}
