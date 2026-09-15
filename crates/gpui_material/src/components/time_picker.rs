//! Time picker (12-hour dial). Specs: https://m3.material.io/components/time-pickers/specs
//!
//! Hour and minute faces plus a static analog selector hand. Motion of the
//! hand is not animated (no shared GPUI clock).

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
pub const MINUTE_STEP: u8 = 5;
pub const HAND_THICKNESS_DP: f32 = 2.0;
pub const HAND_HUB_DP: f32 = 8.0;
pub const HAND_LENGTH_RATIO: f32 = 0.38;
pub const HAND_DOTS: usize = 8;

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

/// Catalog hero starts on the minute face so Visual QA shows 00–55 + the hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialFace {
    Hour,
    Minute,
}

impl DialFace {
    pub const fn toggle(self) -> Self {
        match self {
            Self::Hour => Self::Minute,
            Self::Minute => Self::Hour,
        }
    }
}

pub const DEMO_DIAL: DialFace = DialFace::Minute;

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
    pub hand: Argb,
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
        hand: c.primary,
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

pub fn format_hour_field(hour: u8) -> String {
    format!("{:02}", hour.clamp(1, 12))
}

pub fn format_minute_field(minute: u8) -> String {
    format!("{:02}", minute.min(59))
}

pub fn header_label(hour: u8, minute: u8, period: DayPeriod) -> String {
    format!("{} {}", format_time(hour, minute), period.label())
}

fn polar_offset(angle_deg: f32, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    let angle = angle_deg.to_radians();
    let radius = (clock_dp / 2.0) - (number_dp / 2.0) - 4.0;
    let cx = clock_dp / 2.0 + radius * angle.cos();
    let cy = clock_dp / 2.0 + radius * angle.sin();
    (cx - number_dp / 2.0, cy - number_dp / 2.0)
}

/// Top-left of the hour cell inside a `clock_dp` square (12 at the top).
pub fn hour_offset(hour: u8, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    let idx = if hour == 0 { 12 } else { (hour - 1) % 12 + 1 };
    polar_offset((idx as f32) * 30.0 - 90.0, clock_dp, number_dp)
}

/// Top-left of a 5-minute label (0 at the top, 15 at the right).
pub fn minute_offset(minute: u8, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    let snapped = select_minute(0, minute);
    polar_offset((snapped as f32) * 6.0 - 90.0, clock_dp, number_dp)
}

pub fn minute_labels() -> impl Iterator<Item = u8> {
    (0..12).map(|i| i * MINUTE_STEP)
}

pub fn select_hour(_current: u8, tapped: u8) -> u8 {
    tapped.clamp(1, 12)
}

pub fn select_minute(_current: u8, tapped: u8) -> u8 {
    let m = tapped.min(59);
    (m / MINUTE_STEP) * MINUTE_STEP
}

/// Degrees from 12 o'clock, clockwise. CSS `rotate()` and GPUI dots share this.
pub fn hand_angle_deg(face: DialFace, hour: u8, minute: u8) -> f32 {
    match face {
        DialFace::Hour => {
            let h = if hour == 0 { 12 } else { hour };
            (h as f32) * 30.0 + (minute as f32) * 0.5
        }
        DialFace::Minute => (minute.min(59) as f32) * 6.0,
    }
}

/// Center of the selected hour/minute cell (selector knob).
pub fn hand_end(clock_dp: f32, face: DialFace, hour: u8, minute: u8, number_dp: f32) -> (f32, f32) {
    let (x, y) = match face {
        DialFace::Hour => hour_offset(hour, clock_dp, number_dp),
        DialFace::Minute => minute_offset(minute, clock_dp, number_dp),
    };
    (x + number_dp / 2.0, y + number_dp / 2.0)
}

/// Small dots from the hub toward the selected number (legacy GPUI fallback).
pub fn hand_dots(
    clock_dp: f32,
    face: DialFace,
    hour: u8,
    minute: u8,
    number_dp: f32,
) -> Vec<(f32, f32)> {
    let (ex, ey) = hand_end(clock_dp, face, hour, minute, number_dp);
    let cx = clock_dp / 2.0;
    let cy = clock_dp / 2.0;
    let n = HAND_DOTS.max(2);
    (1..n)
        .map(|i| {
            let t = i as f32 / n as f32;
            (
                cx + (ex - cx) * t - HAND_THICKNESS_DP,
                cy + (ey - cy) * t - HAND_THICKNESS_DP,
            )
        })
        .collect()
}

/// Filled quadrilateral for the analog selector hand (hub → selected number).
pub fn hand_quad(
    clock_dp: f32,
    face: DialFace,
    hour: u8,
    minute: u8,
    number_dp: f32,
) -> [(f32, f32); 4] {
    let (ex, ey) = hand_end(clock_dp, face, hour, minute, number_dp);
    let cx = clock_dp / 2.0;
    let cy = clock_dp / 2.0;
    let dx = ex - cx;
    let dy = ey - cy;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let nx = -dy / len * (HAND_THICKNESS_DP / 2.0);
    let ny = dx / len * (HAND_THICKNESS_DP / 2.0);
    [
        (cx + nx, cy + ny),
        (ex + nx, ey + ny),
        (ex - nx, ey - ny),
        (cx - nx, cy - ny),
    ]
}

pub fn hand_svg_d(
    clock_dp: f32,
    face: DialFace,
    hour: u8,
    minute: u8,
    number_dp: f32,
) -> String {
    let q = hand_quad(clock_dp, face, hour, minute, number_dp);
    format!(
        "M{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} Z",
        q[0].0, q[0].1, q[1].0, q[1].1, q[2].0, q[2].1, q[3].0, q[3].1
    )
}
