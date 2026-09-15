//! Time picker (12-hour dial). Specs: https://m3.material.io/components/time-pickers/specs
//!
//! Tokens + a catalog-scaled clock face. Motion of the selector hand is not
//! animated (no shared GPUI clock).

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const CLOCK_DP: f32 = 256.0;
pub const NUMBER_DP: f32 = 48.0;
pub const CONTAINER_PAD_DP: f32 = 24.0;
pub const CORNER_DP: f32 = 28.0;
pub const PERIOD_W_DP: f32 = 52.0;
pub const PERIOD_H_DP: f32 = 36.0;
pub const PERIOD_GAP_DP: f32 = 8.0;
pub const TITLE: &str = "Select time";
pub const DEMO_HOUR: u8 = 6;
pub const DEMO_MINUTE: u8 = 30;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayPeriod {
    Am,
    Pm,
}

impl DayPeriod {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Am => "AM",
            Self::Pm => "PM",
        }
    }

    pub const fn toggle(self) -> Self {
        match self {
            Self::Am => Self::Pm,
            Self::Pm => Self::Am,
        }
    }
}

pub const DEMO_PERIOD: DayPeriod = DayPeriod::Pm;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimePickerAppearance {
    pub corners: Corners,
    pub container: Argb,
    pub clock: Argb,
    pub number: Argb,
    pub number_selected_container: Argb,
    pub number_selected: Argb,
    pub period_selected_container: Argb,
    pub period_selected: Argb,
    pub period_idle_container: Argb,
    pub period_idle: Argb,
    pub header: Argb,
    pub elevation_dp: f32,
    pub clock_dp: f32,
    pub number_dp: f32,
    pub title_style: TypeStyle,
    pub time_style: TypeStyle,
    pub number_style: TypeStyle,
    pub period_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> TimePickerAppearance {
    let c = theme.color;
    TimePickerAppearance {
        corners: Corners::all(CORNER_DP),
        container: c.surface_container_high,
        clock: c.surface_container_highest,
        number: c.on_surface,
        number_selected_container: c.primary,
        number_selected: c.on_primary,
        period_selected_container: c.tertiary_container,
        period_selected: c.on_tertiary_container,
        period_idle_container: c.surface_container_highest,
        period_idle: c.on_surface,
        header: c.on_surface,
        elevation_dp: theme.elevation.level3,
        clock_dp: CLOCK_DP,
        number_dp: NUMBER_DP,
        title_style: theme.typography.label_large,
        time_style: theme.typography.display_small.emphasized(),
        number_style: theme.typography.body_large,
        period_style: theme.typography.title_medium.emphasized(),
    }
}

pub fn format_time(hour: u8, minute: u8) -> String {
    format!("{}:{:02}", hour.clamp(1, 12), minute.min(59))
}

pub fn header_label(hour: u8, minute: u8, period: DayPeriod) -> String {
    format!("{} {}", format_time(hour, minute), period.label())
}

/// Top-left of the hour cell inside a `clock_dp` square (12 at the top).
pub fn hour_offset(hour: u8, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    let idx = if hour == 0 { 12 } else { (hour - 1) % 12 + 1 };
    let angle_deg = (idx as f32) * 30.0 - 90.0;
    let angle = angle_deg.to_radians();
    let radius = (clock_dp / 2.0) - (number_dp / 2.0) - 4.0;
    let cx = clock_dp / 2.0 + radius * angle.cos();
    let cy = clock_dp / 2.0 + radius * angle.sin();
    (cx - number_dp / 2.0, cy - number_dp / 2.0)
}

pub fn select_hour(_current: u8, tapped: u8) -> u8 {
    tapped.clamp(1, 12)
}
