//! Golden tests against androidx Material 3 token values (v0_210 palette / type scale).

use gpui_material::components::{
    badge, bottom_sheet, button, button_group, card, checkbox, chip, date_picker, dialog, divider,
    fab, icon_button, list, menu, navigation_bar, progress, radio, slider, snackbar, switch, tabs,
    text_field, top_app_bar,
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
    assert_eq!(cut.start_dp, 12.0);
    assert_eq!(cut.stroke_dp, 2.0);
    assert!(cut.width_dp >= 28.0);
    assert_eq!(focused.cutout_fill, theme.color.background);
    assert!(text_field::notch_width_dp("Email", 12.0) >= 28.0);

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
    assert!(html.contains("data-dialog=\"basic\""));
    assert!(html.contains("data-dialog=\"list\""));
    assert!(html.contains("Phone ringtone"));
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

    let sheet = bottom_sheet::resolve(&theme, true);
    assert_eq!(sheet.corners.top_left, 28.0);
    assert_eq!(sheet.corners.bottom_left, 0.0);
    assert_eq!(sheet.container, theme.color.surface_container_low);
    assert_eq!((sheet.handle_w, sheet.handle_h), (32.0, 4.0));
    assert_eq!(sheet.elevation_dp, 1.0);

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
    assert_eq!(typography::FONT_FAMILY_DESKTOP, "Liberation Sans");
    assert_eq!(typography::words("Call volume"), vec!["Call", "volume"]);
    assert_eq!(typography::WORD_GAP_DP, 6.0);
    assert_eq!(typography::words("Reset settings?"), vec!["Reset", "settings?"]);
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
}
