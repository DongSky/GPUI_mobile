//! Time picker (12-hour + 24-hour dial + Expressive TimeScroll + TimeInput).
//! Specs: https://m3.material.io/components/time-pickers/specs
//!
//! Dial: hour and minute faces plus an analog selector hand. GPUI/HTML
//! interpolate the hand angle when the face or value changes (spatial-fast).
//! 24-hour (`is24Hour`) paints Compose `ClockFace` dual rings: outer 00–11
//! (`OuterCircleToSizeRatio` 101/256) and inner 12–23 (`InnerCircle` 69/256).
//! `ClockFaceSizeModifier` picks 256 / 238 / 200 from available height
//! (`TimePickerMaxHeight` 384 / `TimePickerMidHeight` 330).
//!
//! Expressive (I/O 2026, recommended): Compose `TimeScroll` with two
//! `ScrollField`s (hours + minutes), `TimePickerDefaults.vibrantColors()`,
//! and `ScrollFieldDefaults.ScrollFieldHeight` 200 /
//! `ScrollFieldDefaults.shape` CornerLarge. `TimeInput` (96×72
//! fields, `TimePickerDefaults.shapes().timeFieldShape` CornerLarge) +
//! `ScrollDisplayModeToggle` switch Scroll ↔ Input + official
//! `TimePickerDialogDefaults.DisplayModeToggle` a11y strings +
//! TimeSelector `Select hour` / `Select minutes` and TimeInput
//! `for hour` / `for minutes`. 24-hour
//! (`is24Hour`) uses 00–23 and hides the AM/PM selector.

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

/// Compose `ClockDialContainerSize` — full ClockFace when height ≥ `TimePickerMaxHeight`.
pub const CLOCK_DP: f32 = 256.0;
/// Compose `ClockDialSelectorHandleContainerSize` (stays 48 at every ClockFace size).
pub const NUMBER_DP: f32 = 48.0;
/// Compose `TimePickerMaxHeight` — ClockFace uses `ClockDialContainerSize` at/above this.
pub const TIME_PICKER_MAX_HEIGHT_DP: f32 = 384.0;
/// Compose `TimePickerMidHeight` — ClockFace uses `ClockDialMidContainerSize` at/above this.
pub const TIME_PICKER_MID_HEIGHT_DP: f32 = 330.0;
/// Compose `ClockDialMidContainerSize`.
pub const CLOCK_DIAL_MID_CONTAINER_SIZE_DP: f32 = 238.0;
/// Compose `ClockDialMinContainerSize`.
pub const CLOCK_DIAL_MIN_CONTAINER_SIZE_DP: f32 = 200.0;
/// Catalog / hosts apply official `ClockFaceSizeModifier` sizes.
pub const CLOCK_DIAL_SIZES: bool = true;
/// Compose `OuterCircleToSizeRatio` × `ClockDialContainerSize` (101/256).
pub const OUTER_CIRCLE_RADIUS_DP: f32 = 101.0;
/// Compose `InnerCircleToSizeRatio` × `ClockDialContainerSize` (69/256).
pub const INNER_CIRCLE_RADIUS_DP: f32 = 69.0;
/// Time selector container width (12-hour).
pub const TIME_SELECTOR_W_DP: f32 = 96.0;
/// Time selector container width (24h vertical).
pub const TIME_SELECTOR_W_24H_DP: f32 = 114.0;
pub const TIME_SELECTOR_H_DP: f32 = 80.0;
/// Catalog / hosts apply official TimeSelector selected / idle colors.
pub const TIME_SELECTOR_COLORS: bool = true;
/// Compose `DisplaySeparatorWidth` — hour:minute colon slot.
pub const DISPLAY_SEPARATOR_W_DP: f32 = 24.0;
/// Dial `DisplaySeparator` height = `PeriodSelectorVerticalContainerHeight`.
pub const DISPLAY_SEPARATOR_H_DP: f32 = TIME_SELECTOR_H_DP;
/// Catalog / hosts apply official DisplaySeparator.
pub const DISPLAY_SEPARATOR: bool = true;
pub const CONTAINER_PAD_DP: f32 = 24.0;
pub const CORNER_DP: f32 = 28.0;
pub const PERIOD_W_DP: f32 = 52.0;
/// Half of `PeriodSelectorVerticalContainerHeight` (no unofficial 8dp gap).
pub const PERIOD_H_DP: f32 = 40.0;
/// Official PeriodSelector is a single outlined shell (no item gap).
pub const PERIOD_GAP_DP: f32 = 0.0;
/// Compose `PeriodSelectorVerticalContainerHeight`.
pub const PERIOD_CONTAINER_H_DP: f32 = 80.0;
/// Compose `PeriodSelectorOutlineWidth`.
pub const PERIOD_OUTLINE_W_DP: f32 = 1.0;
/// Catalog / hosts apply official PeriodSelector outline.
pub const PERIOD_OUTLINE: bool = true;
/// Compose `TimePickerPeriodToggle`.
pub const PERIOD_TOGGLE_LABEL: &str = "Select AM or PM";
/// Catalog / hosts apply official PeriodToggle contentDescription.
pub const PERIOD_TOGGLE_A11Y: bool = true;
/// Compose `TimePickerHourSelection`.
pub const HOUR_SELECTION: &str = "Select hour";
/// Compose `TimePickerMinuteSelection`.
pub const MINUTE_SELECTION: &str = "Select minutes";
/// Compose `TimeInputHourTextField`.
pub const INPUT_HOUR_FIELD: &str = "for hour";
/// Compose `TimeInputMinuteTextField`.
pub const INPUT_MINUTE_FIELD: &str = "for minutes";
/// Catalog / hosts apply official hour/minute selection a11y.
pub const HOUR_MINUTE_A11Y: bool = true;

/// CSS `border-width` for the PeriodSelector shell.
pub fn period_outline_w_css() -> String {
    format!("{:.0}px", PERIOD_OUTLINE_W_DP)
}
/// Specs: period selector in horizontal (landscape) layout.
pub const PERIOD_HORIZONTAL_W_DP: f32 = 216.0;
pub const PERIOD_HORIZONTAL_H_DP: f32 = 38.0;
/// Gap between the selector column and the 256dp ClockFace (horizontal).
pub const HORIZONTAL_GAP_DP: f32 = 24.0;
/// Compose `ClockDisplayBottomMargin` — vertical gap ClockDisplay → ClockFace.
pub const CLOCK_DISPLAY_BOTTOM_MARGIN_DP: f32 = 36.0;
/// Compose `ClockFaceBottomMargin` — space below ClockFace (vertical).
pub const CLOCK_FACE_BOTTOM_MARGIN_DP: f32 = 24.0;
/// Catalog / hosts apply official vertical ClockFace margins.
pub const CLOCK_FACE_MARGINS: bool = true;
/// Compose `PeriodToggleMargin` (start on vertical, top on horizontal).
pub const PERIOD_TOGGLE_MARGIN_DP: f32 = 12.0;
/// Catalog / hosts apply official period-toggle inset.
pub const PERIOD_TOGGLE_MARGIN: bool = true;

/// Compose `ClockFaceSizeModifier` container — Max 256 / Mid 238 / Min 200.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockDialSize {
    Max,
    Mid,
    Min,
}

impl ClockDialSize {
    pub const ALL: [Self; 3] = [Self::Max, Self::Mid, Self::Min];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Max => "max",
            Self::Mid => "mid",
            Self::Min => "min",
        }
    }

    pub const fn container_dp(self) -> f32 {
        match self {
            Self::Max => CLOCK_DP,
            Self::Mid => CLOCK_DIAL_MID_CONTAINER_SIZE_DP,
            Self::Min => CLOCK_DIAL_MIN_CONTAINER_SIZE_DP,
        }
    }
}

/// Compose `ClockFaceSizeModifier` — size from the ClockFace max-height constraint.
pub fn clock_dial_size_for_max_height(max_height_dp: f32) -> ClockDialSize {
    if max_height_dp >= TIME_PICKER_MAX_HEIGHT_DP {
        ClockDialSize::Max
    } else if max_height_dp >= TIME_PICKER_MID_HEIGHT_DP {
        ClockDialSize::Mid
    } else {
        ClockDialSize::Min
    }
}

pub fn clock_dial_container_size_css(size: ClockDialSize) -> String {
    format!("{:.0}px", size.container_dp())
}

/// Compact host / catalog-column ClockFace sits below `TimePickerMidHeight`.
pub const DEMO_HOST_CLOCK_MAX_HEIGHT_DP: f32 = 320.0;
/// Compact hosts use `ClockDialMinContainerSize` 200 (replaces the old 192dp 0.75 scale).
pub const DEMO_HOST_CLOCK_SIZE: ClockDialSize = ClockDialSize::Min;

pub fn demo_host_clock_dp() -> f32 {
    DEMO_HOST_CLOCK_SIZE.container_dp()
}

/// Compose `TimePickerLayoutType` — vertical (portrait) vs horizontal (landscape).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePickerLayoutType {
    Vertical,
    Horizontal,
}

impl TimePickerLayoutType {
    pub const ALL: [Self; 2] = [Self::Vertical, Self::Horizontal];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }

    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Horizontal)
    }
}

/// Catalog / Android compact dial stays vertical. Desktop uses horizontal.
pub const DEMO_LAYOUT: TimePickerLayoutType = TimePickerLayoutType::Vertical;
pub const DEMO_DESKTOP_LAYOUT: TimePickerLayoutType = TimePickerLayoutType::Horizontal;

pub fn period_w_dp(layout: TimePickerLayoutType) -> f32 {
    match layout {
        TimePickerLayoutType::Vertical => PERIOD_W_DP,
        TimePickerLayoutType::Horizontal => PERIOD_HORIZONTAL_W_DP,
    }
}

pub fn period_h_dp(layout: TimePickerLayoutType) -> f32 {
    match layout {
        TimePickerLayoutType::Vertical => PERIOD_CONTAINER_H_DP,
        TimePickerLayoutType::Horizontal => PERIOD_HORIZONTAL_H_DP,
    }
}
/// Compose `TimePickerDialogTitle` (Picker) — `TimePickerDialogDefaults.Title`.
pub const TITLE: &str = "Select time";
/// Compose `TimeScrollDialogTitle`.
pub const SCROLL_TITLE: &str = "Select time";
/// Compose `TimeInputDialogTitle`.
pub const INPUT_TITLE: &str = "Enter time";
/// Compose `TimePickerDialogDefaults.Title` `padding(bottom = 20.dp)`.
pub const TITLE_PAD_BOTTOM_DP: f32 = 20.0;
/// Catalog / hosts apply official dialog title + 20dp bottom + labelMedium.
pub const DIALOG_TITLE: bool = true;
/// Compose `TimePickerCustomLayout` portrait title top.
pub const PORT_TITLE_TOP_DP: f32 = 24.0;
/// Compose `TimePickerCustomLayout` portrait actions bottom.
pub const PORT_ACTIONS_BOTTOM_DP: f32 = 24.0;
/// Compose `TimePickerCustomLayout` landscape title inset.
pub const LAND_TITLE_TOP_DP: f32 = 24.0;
/// Compose `TimePickerCustomLayout` landscape content top.
pub const LAND_CONTENT_TOP_DP: f32 = 16.0;
/// Compose `TimePickerCustomLayout` landscape content → actions gap.
pub const LAND_CONTENT_ACTIONS_DP: f32 = 4.0;
/// Compose `TimePickerCustomLayout` landscape actions bottom.
pub const LAND_ACTIONS_BOTTOM_DP: f32 = 8.0;
/// Compose `TimePickerDialog` action `Arrangement.spacedBy(8.dp)`.
pub const DIALOG_ACTIONS_GAP_DP: f32 = 8.0;
/// Catalog / hosts paint official TimePickerDialog Cancel / OK.
pub const DIALOG_ACTIONS: bool = true;
pub const DIALOG_OK: &str = "OK";
pub const DIALOG_CANCEL: &str = "Cancel";
/// Compose `TimePickerDialogDefaults.vibrantContainerColor` (`surfaceContainer`).
pub const VIBRANT_DIALOG: bool = true;
/// Compose `TimePickerDialogDefaults.vibrantShape` (`CornerExtraLarge`).
pub const VIBRANT_DIALOG_CORNER_DP: f32 = CORNER_DP;

/// Dialog shell for the Expressive TimeScroll / TimeInput hero.
pub fn vibrant_dialog_container(theme: &Theme) -> Argb {
    theme.color.surface_container
}

/// Title / on-container for `VibrantTimePickerDialog`.
pub fn vibrant_dialog_on_container(theme: &Theme) -> Argb {
    theme.color.on_surface
}

/// ScrollDisplayModeToggle / format toggle on the vibrant dialog.
pub fn vibrant_dialog_toggle(theme: &Theme) -> Argb {
    theme.color.on_surface_variant
}

/// CSS `padding-bottom` for `TimePickerDialogDefaults.Title`.
pub fn title_pad_bottom_css() -> String {
    format!("{:.0}px", TITLE_PAD_BOTTOM_DP)
}

/// CSS `padding` top for portrait `TimePickerCustomLayout` title.
pub fn port_title_top_css() -> String {
    format!("{:.0}px", PORT_TITLE_TOP_DP)
}

/// CSS `padding-bottom` for portrait dialog actions.
pub fn port_actions_bottom_css() -> String {
    format!("{:.0}px", PORT_ACTIONS_BOTTOM_DP)
}

/// CSS `margin` / `padding-top` for landscape content.
pub fn land_content_top_css() -> String {
    format!("{:.0}px", LAND_CONTENT_TOP_DP)
}

/// CSS `padding-bottom` for landscape dialog actions.
pub fn land_actions_bottom_css() -> String {
    format!("{:.0}px", LAND_ACTIONS_BOTTOM_DP)
}

/// CSS `gap` between Cancel and OK.
pub fn dialog_actions_gap_css() -> String {
    format!("{:.0}px", DIALOG_ACTIONS_GAP_DP)
}

/// Portrait vs landscape action-row bottom inset.
pub fn actions_bottom_dp(layout: TimePickerLayoutType) -> f32 {
    match layout {
        TimePickerLayoutType::Vertical => PORT_ACTIONS_BOTTOM_DP,
        TimePickerLayoutType::Horizontal => LAND_ACTIONS_BOTTOM_DP,
    }
}

pub const DEMO_HOUR: u8 = 6;
/// 6:30 PM in 24-hour (`is24Hour`) — catalog / host TimeInput hero.
pub const DEMO_HOUR_24: u8 = 18;
pub const DEMO_MINUTE: u8 = 30;
pub const MINUTE_STEP: u8 = 5;
pub const HAND_THICKNESS_DP: f32 = 2.0;
pub const HAND_HUB_DP: f32 = 8.0;
pub const HAND_LENGTH_RATIO: f32 = 0.38;
pub const HAND_DOTS: usize = 8;
pub const SECOND_HAND_THICKNESS_DP: f32 = 1.25;
pub const SECOND_HAND_LENGTH_SCALE: f32 = 0.92;
/// Catalog demo second (reproducible snapshot). Live hosts use `wall_second`.
pub const DEMO_SECOND: u8 = 12;
/// One revolution of the ticking second hand.
pub const SECOND_PERIOD_MS: u16 = 60_000;
/// GPUI animation period for the wall-clock second hand (shared vsync clock).
pub const SECOND_HAND_FRAME_MS: u16 = crate::motion::FRAME_MS;

/// Seconds + in-second fraction from the host wall clock (UTC ≡ local seconds).
pub fn wall_second() -> (u8, f32) {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = (dur.as_secs() % 60) as u8;
    let tick = dur.subsec_millis() as f32 / 1000.0;
    (secs, tick)
}

/// Degrees from 12 o'clock for the live wall-clock second hand.
pub fn second_hand_angle_wall_clock() -> f32 {
    let (second, tick) = wall_second();
    second_hand_angle_deg(second, tick)
}

/// Spatial-fast duration for hour/minute hand motion.
pub fn hand_motion_ms(theme: &Theme) -> u16 {
    theme.motion.spatial_fast_ms
}

/// Shortest-path lerp between two clock angles (degrees clockwise from 12).
pub fn lerp_angle_deg(from: f32, to: f32, t: f32) -> f32 {
    let mut d = to - from;
    while d > 180.0 {
        d -= 360.0;
    }
    while d < -180.0 {
        d += 360.0;
    }
    from + d * t.clamp(0.0, 1.0)
}

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

/// Catalog / host hero starts on the hour face so Visual QA shows the 24h rings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialFace {
    Hour,
    Minute,
}

impl DialFace {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Minute => "minute",
        }
    }

    pub const fn toggle(self) -> Self {
        match self {
            Self::Hour => Self::Minute,
            Self::Minute => Self::Hour,
        }
    }
}

pub const DEMO_DIAL: DialFace = DialFace::Hour;

/// Compose `ClockFace` ring (`OuterCircle` 00–11 / `InnerCircle` 12–23).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialRing {
    Outer,
    Inner,
}

impl DialRing {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Outer => "outer",
            Self::Inner => "inner",
        }
    }
}

/// Compose `Hours` (12-hour outer ring, 12 at the top).
pub const HOURS_12: [u8; 12] = [12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
/// `Hours[i] % 12` — 24-hour outer ring (00 at the top).
pub const HOURS_24_OUTER: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
/// Compose `ExtraHours` — `Hours[i] % 12 + 12` (12 at the top).
pub const HOURS_24_INNER: [u8; 12] = [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];

pub fn inner_to_outer_scale() -> f32 {
    INNER_CIRCLE_RADIUS_DP / OUTER_CIRCLE_RADIUS_DP
}

pub fn circle_radius_dp(ring: DialRing, clock_dp: f32) -> f32 {
    let base = match ring {
        DialRing::Outer => OUTER_CIRCLE_RADIUS_DP,
        DialRing::Inner => INNER_CIRCLE_RADIUS_DP,
    };
    base * (clock_dp / CLOCK_DP)
}

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
    /// `TimePickerTokens.TimeSelectorContainerColor` (`PrimaryContainer`).
    pub time_selector_selected_container: Argb,
    /// `TimePickerTokens.TimeSelectorLabelTextColor` (`OnPrimaryContainer`).
    pub time_selector_selected: Argb,
    /// `TimePickerTokens.TimeSelectorUnselectedContainerColor`.
    pub time_selector_container: Argb,
    /// `TimePickerTokens.TimeSelectorUnselectedLabelColor` (`OnSurface`).
    pub time_selector_content: Argb,
    /// `TimePickerTokens.PeriodSelectorOutlineColor`.
    pub period_outline: Argb,
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
        title_style: theme.typography.label_medium,
        time_style: theme.typography.display_small.emphasized(),
        number_style: theme.typography.body_large,
        period_style: theme.typography.title_medium.emphasized(),
        time_selector_selected_container: c.primary_container,
        time_selector_selected: c.on_primary_container,
        time_selector_container: c.surface_container_highest,
        time_selector_content: c.on_surface,
        period_outline: c.outline,
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

fn polar_at(angle_deg: f32, radius: f32, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    let angle = angle_deg.to_radians();
    let cx = clock_dp / 2.0 + radius * angle.cos();
    let cy = clock_dp / 2.0 + radius * angle.sin();
    (cx - number_dp / 2.0, cy - number_dp / 2.0)
}

fn polar_offset(angle_deg: f32, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    polar_at(
        angle_deg,
        circle_radius_dp(DialRing::Outer, clock_dp),
        clock_dp,
        number_dp,
    )
}

/// Top-left of the hour cell inside a `clock_dp` square (12 at the top).
pub fn hour_offset(hour: u8, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    hour_offset_for(hour, TimeFormat::Hour12, clock_dp, number_dp)
}

/// 12-hour outer / 24-hour dual-ring hour cell (Compose `ClockFace`).
pub fn hour_offset_for(hour: u8, format: TimeFormat, clock_dp: f32, number_dp: f32) -> (f32, f32) {
    polar_at(
        (hour % 12) as f32 * 30.0 - 90.0,
        circle_radius_dp(hour_ring(hour, format), clock_dp),
        clock_dp,
        number_dp,
    )
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

pub fn select_hour_for(_current: u8, tapped: u8, format: TimeFormat) -> u8 {
    tapped.clamp(format.hour_min(), format.hour_max())
}

pub fn select_minute(_current: u8, tapped: u8) -> u8 {
    let m = tapped.min(59);
    (m / MINUTE_STEP) * MINUTE_STEP
}

/// Degrees from 12 o'clock, clockwise. CSS `rotate()` and GPUI dots share this.
pub fn hand_angle_deg(face: DialFace, hour: u8, minute: u8) -> f32 {
    match face {
        DialFace::Hour => hour_face_live_angle_deg(hour, minute, 0.0),
        DialFace::Minute => (minute.min(59) as f32) * 6.0,
    }
}

/// Continuous hour-face motion: `tick` in 0..=1 adds a fraction of a minute
/// so the analog hand eases while the hour dial is showing. 24-hour 00 and 12
/// share 12 o'clock; 18 shares 6 o'clock on the inner ring.
pub fn hour_face_live_angle_deg(hour: u8, minute: u8, tick: f32) -> f32 {
    let hours = (hour % 12) as f32;
    let minutes = minute as f32 + tick.clamp(0.0, 1.0);
    hours * 30.0 + minutes * 0.5
}

/// Degrees from 12 o'clock for a ticking second hand (`tick` is the
/// in-second fraction 0..=1). A 60 s repeating clock uses `tick = delta`.
pub fn second_hand_angle_deg(second: u8, tick: f32) -> f32 {
    (second.min(59) as f32 + tick.clamp(0.0, 1.0)) * 6.0
}

/// Center of the selector knob at an arbitrary clock angle (outer ring).
pub fn hand_end_at_angle(clock_dp: f32, angle_deg: f32, number_dp: f32) -> (f32, f32) {
    hand_end_at_radius(
        clock_dp,
        angle_deg,
        number_dp,
        circle_radius_dp(DialRing::Outer, clock_dp),
    )
}

/// Center of the selector knob at `radius` (outer 101 / inner 69 at 256dp).
pub fn hand_end_at_radius(
    clock_dp: f32,
    angle_deg: f32,
    number_dp: f32,
    radius: f32,
) -> (f32, f32) {
    let (x, y) = polar_at(angle_deg - 90.0, radius, clock_dp, number_dp);
    (x + number_dp / 2.0, y + number_dp / 2.0)
}

/// Center of the selected hour/minute cell (selector knob).
pub fn hand_end(clock_dp: f32, face: DialFace, hour: u8, minute: u8, number_dp: f32) -> (f32, f32) {
    hand_end_at_angle(clock_dp, hand_angle_deg(face, hour, minute), number_dp)
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

fn hand_quad_thick(
    clock_dp: f32,
    angle_deg: f32,
    number_dp: f32,
    thickness: f32,
    length_scale: f32,
) -> [(f32, f32); 4] {
    hand_quad_thick_radius(
        clock_dp,
        angle_deg,
        number_dp,
        thickness,
        length_scale,
        circle_radius_dp(DialRing::Outer, clock_dp),
    )
}

fn hand_quad_thick_radius(
    clock_dp: f32,
    angle_deg: f32,
    number_dp: f32,
    thickness: f32,
    length_scale: f32,
    radius: f32,
) -> [(f32, f32); 4] {
    let (ex0, ey0) = hand_end_at_radius(clock_dp, angle_deg, number_dp, radius);
    let cx = clock_dp / 2.0;
    let cy = clock_dp / 2.0;
    let ex = cx + (ex0 - cx) * length_scale;
    let ey = cy + (ey0 - cy) * length_scale;
    let dx = ex - cx;
    let dy = ey - cy;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let nx = -dy / len * (thickness / 2.0);
    let ny = dx / len * (thickness / 2.0);
    [
        (cx + nx, cy + ny),
        (ex + nx, ey + ny),
        (ex - nx, ey - ny),
        (cx - nx, cy - ny),
    ]
}

/// Filled quadrilateral for the analog selector hand at `angle_deg`.
pub fn hand_quad_at_angle(clock_dp: f32, angle_deg: f32, number_dp: f32) -> [(f32, f32); 4] {
    hand_quad_thick(clock_dp, angle_deg, number_dp, HAND_THICKNESS_DP, 1.0)
}

/// Selector hand at an explicit ring radius (inner 12–23 / outer 00–11).
pub fn hand_quad_at_radius(
    clock_dp: f32,
    angle_deg: f32,
    number_dp: f32,
    radius: f32,
) -> [(f32, f32); 4] {
    hand_quad_thick_radius(
        clock_dp,
        angle_deg,
        number_dp,
        HAND_THICKNESS_DP,
        1.0,
        radius,
    )
}

/// Thinner, slightly longer ticking second hand.
pub fn second_hand_quad(clock_dp: f32, angle_deg: f32, number_dp: f32) -> [(f32, f32); 4] {
    hand_quad_thick(
        clock_dp,
        angle_deg,
        number_dp,
        SECOND_HAND_THICKNESS_DP,
        SECOND_HAND_LENGTH_SCALE,
    )
}

pub fn second_hand_svg_d(clock_dp: f32, angle_deg: f32, number_dp: f32) -> String {
    let q = second_hand_quad(clock_dp, angle_deg, number_dp);
    format!(
        "M{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} Z",
        q[0].0, q[0].1, q[1].0, q[1].1, q[2].0, q[2].1, q[3].0, q[3].1
    )
}

/// Filled quadrilateral for the analog selector hand (hub → selected number).
pub fn hand_quad(
    clock_dp: f32,
    face: DialFace,
    hour: u8,
    minute: u8,
    number_dp: f32,
) -> [(f32, f32); 4] {
    hand_quad_at_angle(clock_dp, hand_angle_deg(face, hour, minute), number_dp)
}

pub fn hand_svg_d_at_angle(clock_dp: f32, angle_deg: f32, number_dp: f32) -> String {
    let q = hand_quad_at_angle(clock_dp, angle_deg, number_dp);
    format!(
        "M{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} Z",
        q[0].0, q[0].1, q[1].0, q[1].1, q[2].0, q[2].1, q[3].0, q[3].1
    )
}

pub fn hand_svg_d(clock_dp: f32, face: DialFace, hour: u8, minute: u8, number_dp: f32) -> String {
    hand_svg_d_at_angle(clock_dp, hand_angle_deg(face, hour, minute), number_dp)
}

/// Compose `TimePicker` display mode. Dial is baseline; Scroll/Input are Expressive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePickerStyle {
    Dial,
    Scroll,
    Input,
}

impl TimePickerStyle {
    pub const ALL: [Self; 3] = [Self::Dial, Self::Scroll, Self::Input];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Dial => "dial",
            Self::Scroll => "scroll",
            Self::Input => "input",
        }
    }

    /// Compose `TimePickerDialogDefaults.Title` string for this style.
    pub const fn title(self) -> &'static str {
        match self {
            Self::Dial => TITLE,
            Self::Scroll => SCROLL_TITLE,
            Self::Input => INPUT_TITLE,
        }
    }
}

/// Compose `TimePickerDisplayMode` for `ScrollDisplayModeToggle` (Scroll ↔ Input).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePickerDisplayMode {
    Scroll,
    Input,
}

impl TimePickerDisplayMode {
    pub const ALL: [Self; 2] = [Self::Scroll, Self::Input];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Scroll => "scroll",
            Self::Input => "input",
        }
    }

    pub const fn toggle(self) -> Self {
        match self {
            Self::Scroll => Self::Input,
            Self::Input => Self::Scroll,
        }
    }

    pub const fn style(self) -> TimePickerStyle {
        match self {
            Self::Scroll => TimePickerStyle::Scroll,
            Self::Input => TimePickerStyle::Input,
        }
    }

    /// Icon for the *other* mode (`ScrollDisplayModeToggle`: Keyboard / SwipeVertical).
    pub const fn toggle_icon(self) -> &'static str {
        match self {
            Self::Scroll => KEYBOARD_ICON,
            Self::Input => SWIPE_VERTICAL_ICON,
        }
    }

    /// Compose `ScrollDisplayModeToggle` contentDescription / tooltip.
    pub const fn toggle_label(self) -> &'static str {
        match self {
            Self::Scroll => TOGGLE_KEYBOARD,
            Self::Input => TOGGLE_SCROLL,
        }
    }

    /// Compose `TimePickerDialogDefaults.Title` for Scroll ↔ Input.
    pub const fn title(self) -> &'static str {
        match self {
            Self::Scroll => SCROLL_TITLE,
            Self::Input => INPUT_TITLE,
        }
    }
}

/// Compose `TimePickerDialogDefaults.Title(displayMode)`.
pub fn title_for(mode: TimePickerDisplayMode) -> &'static str {
    mode.title()
}

/// Catalog / host hero starts on 24-hour TimeInput; toggle still paints TimeScroll.
pub const DEMO_DISPLAY_MODE: TimePickerDisplayMode = TimePickerDisplayMode::Input;

/// Compose `TimePickerState.is24Hour` (system settings; 24h hides AM/PM).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFormat {
    Hour12,
    Hour24,
}

impl TimeFormat {
    pub const ALL: [Self; 2] = [Self::Hour12, Self::Hour24];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Hour12 => "12",
            Self::Hour24 => "24",
        }
    }

    pub const fn is_24_hour(self) -> bool {
        matches!(self, Self::Hour24)
    }

    pub const fn shows_period(self) -> bool {
        matches!(self, Self::Hour12)
    }

    pub const fn hour_count(self) -> usize {
        match self {
            Self::Hour12 => HOUR_COUNT,
            Self::Hour24 => HOUR24_COUNT,
        }
    }

    pub const fn toggle_label(self) -> &'static str {
        match self {
            Self::Hour12 => "Switch to 24-hour",
            Self::Hour24 => "Switch to 12-hour",
        }
    }

    /// Label for the *other* format (12 when showing 24-hour).
    pub const fn toggle_text(self) -> &'static str {
        match self {
            Self::Hour12 => "24",
            Self::Hour24 => "12",
        }
    }

    pub const fn hour_min(self) -> u8 {
        match self {
            Self::Hour12 => 1,
            Self::Hour24 => 0,
        }
    }

    pub const fn hour_max(self) -> u8 {
        match self {
            Self::Hour12 => 12,
            Self::Hour24 => 23,
        }
    }

    pub const fn toggle(self) -> Self {
        match self {
            Self::Hour12 => Self::Hour24,
            Self::Hour24 => Self::Hour12,
        }
    }
}

/// Catalog / host Expressive hero uses 24-hour TimeInput + 24-hour dial (no AM/PM).
pub const DEMO_FORMAT: TimeFormat = TimeFormat::Hour24;

pub fn time_selector_w_dp(format: TimeFormat) -> f32 {
    if format.is_24_hour() {
        TIME_SELECTOR_W_24H_DP
    } else {
        TIME_SELECTOR_W_DP
    }
}

pub fn hour_ring(hour: u8, format: TimeFormat) -> DialRing {
    if format.is_24_hour() && hour >= 12 {
        DialRing::Inner
    } else {
        DialRing::Outer
    }
}

pub fn selector_radius_dp(face: DialFace, hour: u8, format: TimeFormat, clock_dp: f32) -> f32 {
    match face {
        DialFace::Hour => circle_radius_dp(hour_ring(hour, format), clock_dp),
        DialFace::Minute => circle_radius_dp(DialRing::Outer, clock_dp),
    }
}

pub fn hour_label(hour: u8, format: TimeFormat) -> String {
    if format.is_24_hour() {
        format!("{:02}", hour.min(23))
    } else {
        hour.clamp(1, 12).to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialHour {
    pub hour: u8,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub ring: DialRing,
}

pub fn hour_cells(format: TimeFormat, clock_dp: f32, number_dp: f32) -> Vec<DialHour> {
    match format {
        TimeFormat::Hour12 => HOURS_12
            .iter()
            .copied()
            .map(|hour| DialHour {
                hour,
                label: hour_label(hour, format),
                x: hour_offset_for(hour, format, clock_dp, number_dp).0,
                y: hour_offset_for(hour, format, clock_dp, number_dp).1,
                ring: DialRing::Outer,
            })
            .collect(),
        TimeFormat::Hour24 => HOURS_24_OUTER
            .iter()
            .copied()
            .chain(HOURS_24_INNER.iter().copied())
            .map(|hour| {
                let (x, y) = hour_offset_for(hour, format, clock_dp, number_dp);
                DialHour {
                    hour,
                    label: hour_label(hour, format),
                    x,
                    y,
                    ring: hour_ring(hour, format),
                }
            })
            .collect(),
    }
}

pub fn demo_hour(format: TimeFormat) -> u8 {
    match format {
        TimeFormat::Hour12 => DEMO_HOUR,
        TimeFormat::Hour24 => DEMO_HOUR_24,
    }
}

pub fn to_hour24(hour12: u8, period: DayPeriod) -> u8 {
    let h = hour12.clamp(1, 12);
    match period {
        DayPeriod::Am => {
            if h == 12 {
                0
            } else {
                h
            }
        }
        DayPeriod::Pm => {
            if h == 12 {
                12
            } else {
                h + 12
            }
        }
    }
}

pub fn to_hour12(hour24: u8) -> (u8, DayPeriod) {
    let h = hour24.min(23);
    if h == 0 {
        (12, DayPeriod::Am)
    } else if h < 12 {
        (h, DayPeriod::Am)
    } else if h == 12 {
        (12, DayPeriod::Pm)
    } else {
        (h - 12, DayPeriod::Pm)
    }
}

/// Dial hour in the active format (1–12 or 00–23). 24-hour keeps the 0–23 value
/// so the inner ring (12–23) can be selected without remapping through AM/PM.
pub fn dial_clock_hour(hour: u8, format: TimeFormat, _period: DayPeriod) -> u8 {
    hour.clamp(format.hour_min(), format.hour_max())
}

/// Map a dial tap into the active `TimeFormat` hour.
pub fn hour_from_dial(tapped: u8, _period: DayPeriod, format: TimeFormat) -> u8 {
    if format.is_24_hour() {
        tapped.min(23)
    } else {
        tapped.clamp(1, 12)
    }
}

pub fn format_hour_field_for(hour: u8, format: TimeFormat) -> String {
    format!("{:02}", hour.clamp(format.hour_min(), format.hour_max()))
}

pub fn header_label_for(hour: u8, minute: u8, period: DayPeriod, format: TimeFormat) -> String {
    if format.is_24_hour() {
        format!(
            "{}:{}",
            format_hour_field_for(hour, format),
            format_minute_field(minute)
        )
    } else {
        header_label(hour, minute, period)
    }
}

/// Compose `TimePickerDialogDefaults.ScrollDisplayModeToggle` 48dp target.
pub const TOGGLE_SIZE_DP: f32 = 48.0;
pub const TOGGLE_ICON_DP: f32 = 24.0;
/// Keyboard — Scroll/Picker → Input (`Icons.Filled.Keyboard`).
pub const KEYBOARD_ICON: &str = "⌨";
/// Schedule / clock — `DisplayModeToggle` Input → Picker (`Icons.Filled.Schedule`).
pub const SCHEDULE_ICON: &str = "◷";
/// SwipeVertical — `ScrollDisplayModeToggle` Input → Scroll (`Icons.Filled.SwipeVertical`).
pub const SWIPE_VERTICAL_ICON: &str = "⇅";
/// Catalog / hosts apply official ScrollDisplayModeToggle SwipeVertical.
pub const SWIPE_VERTICAL: bool = true;

/// Compose `TimePickerDialogDefaults.DisplayModeToggle` icon (Keyboard / Schedule).
pub const fn display_mode_toggle_icon(is_picker: bool) -> &'static str {
    if is_picker {
        KEYBOARD_ICON
    } else {
        SCHEDULE_ICON
    }
}
/// Compose `m3c_time_picker_toggle_keyboard` (`DisplayModeToggle` / `ScrollDisplayModeToggle`).
pub const TOGGLE_KEYBOARD: &str = "Switch to text input mode";
/// Compose `m3c_time_picker_toggle_scroll` (`ScrollDisplayModeToggle` while Input).
pub const TOGGLE_SCROLL: &str = "Switch to scroll mode";
/// Compose `m3c_time_picker_toggle_touch` (`DisplayModeToggle` while Input → Picker).
pub const TOGGLE_TOUCH: &str = "Switch to clock mode";
/// Catalog / hosts apply official DisplayModeToggle a11y + tooltip strings.
pub const DISPLAY_MODE_TOGGLE: bool = true;

/// Compose `TimePickerDialogDefaults.DisplayModeToggle` a11y string.
/// `picker` is the current clock (`TimePickerDisplayMode.Picker`) state.
pub const fn display_mode_toggle_label(picker: bool) -> &'static str {
    if picker {
        TOGGLE_KEYBOARD
    } else {
        TOGGLE_TOUCH
    }
}

/// Compose `TimePickerDefaults.shapes().timeFieldShape` / `ScrollFieldDefaults.shape`
/// (`ShapeKeyTokens.CornerLarge` / `ShapeDefaults.Large`).
pub const TIME_FIELD_SHAPE_CORNER_DP: f32 = 16.0;
/// Catalog / hosts apply official TimePickerShapes + ScrollField shape.
pub const TIME_PICKER_SHAPES: bool = true;

/// Time-input field tokens (`TimeInputTokens.TimeFieldContainer*` size).
pub const INPUT_FIELD_W_DP: f32 = 96.0;
pub const INPUT_FIELD_H_DP: f32 = 72.0;
/// TimeInput field corners follow `TimePickerDefaults.shapes().timeFieldShape`.
pub const INPUT_FIELD_CORNER_DP: f32 = TIME_FIELD_SHAPE_CORNER_DP;
pub const INPUT_PERIOD_W_DP: f32 = 52.0;
pub const INPUT_PERIOD_H_DP: f32 = 72.0;
pub const INPUT_GAP_DP: f32 = 24.0;
pub const INPUT_COLON_GAP_DP: f32 = 8.0;
/// TimeInput `DisplaySeparator` height = `PeriodSelectorContainerHeight`.
pub const INPUT_DISPLAY_SEPARATOR_H_DP: f32 = INPUT_FIELD_H_DP;
/// Compose TimeInput `OutlinedTextFieldDefaults` focused border thickness.
pub const TIME_FIELD_FOCUS_OUTLINE_W_DP: f32 = 2.0;
/// Compose TimeInput unfocused border thickness (vibrant color is transparent).
pub const TIME_FIELD_UNFOCUSED_OUTLINE_W_DP: f32 = 1.0;
/// Catalog / hosts apply official vibrant TimeInput field outline + colors.
pub const TIME_FIELD_OUTLINE: bool = true;

/// CSS `border-width` for a TimeInput field.
pub fn time_field_outline_w_css(focused: bool) -> String {
    format!(
        "{:.0}px",
        if focused {
            TIME_FIELD_FOCUS_OUTLINE_W_DP
        } else {
            TIME_FIELD_UNFOCUSED_OUTLINE_W_DP
        }
    )
}

/// Compose `SupportLabelTop` on TimeInput Hour / Minute supporting text.
pub const SUPPORT_LABEL_TOP_DP: f32 = 7.0;
/// Catalog / hosts paint official TimeInput supporting labels.
pub const SUPPORT_LABEL: bool = true;
/// Compose `Strings.TimePickerHour`.
pub const INPUT_HOUR_LABEL: &str = "Hour";
/// Compose `Strings.TimePickerMinute`.
pub const INPUT_MINUTE_LABEL: &str = "Minute";

/// CSS `margin-top` for TimeInput supporting text.
pub fn support_label_top_css() -> String {
    format!("{:.0}px", SUPPORT_LABEL_TOP_DP)
}

/// Catalog / host hero uses Compose `TimeScroll` (recommended).
pub const DEMO_STYLE: TimePickerStyle = TimePickerStyle::Scroll;

/// Compose `ScrollFieldDefaults.ScrollFieldHeight` (three-item window).
pub const SCROLL_FIELD_H_DP: f32 = 200.0;
/// Time-selection sample width (`Modifier.size(width = 100.dp, …)`).
pub const SCROLL_FIELD_W_DP: f32 = 100.0;
pub const SCROLL_VISIBLE: u8 = 3;
pub const SCROLL_ITEM_H_DP: f32 = SCROLL_FIELD_H_DP / SCROLL_VISIBLE as f32;
/// Official time-selection sample: 8dp between fields, 12dp row pad.
pub const SCROLL_GAP_DP: f32 = 8.0;
pub const SCROLL_PAD_DP: f32 = 12.0;
/// TimeScroll field corners follow `ScrollFieldDefaults.shape` (`CornerLarge`).
pub const SCROLL_FIELD_CORNER_DP: f32 = TIME_FIELD_SHAPE_CORNER_DP;

/// CSS `border-radius` for official time-field / ScrollField shape.
pub fn time_field_shape_corner_css() -> String {
    format!("{:.0}px", TIME_FIELD_SHAPE_CORNER_DP)
}
/// Colon `offset(y = (-4).dp)` in the Compose time-selection sample.
pub const SCROLL_COLON_OFFSET_Y_DP: f32 = -4.0;
pub const HOUR_COUNT: usize = 12;
pub const HOUR24_COUNT: usize = 24;
pub const MINUTE_COUNT: usize = 60;
/// Same LazyColumn-style decay as list swipe / carousel.
pub const SCROLL_FLING_DECAY: f32 = 2.0;
pub const SCROLL_FLING_REST: f32 = 0.35;
pub const SCROLL_SNAP_STIFFNESS: f32 = 14.0;
pub const SCROLL_FRAME_DT: f32 = crate::motion::FRAME_DT;
/// Slots painted above/below the selected item (plus the center).
pub const SCROLL_SLOT_SPAN: i32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollKind {
    Hour,
    Minute,
}

impl ScrollKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Minute => "minute",
        }
    }

    /// Official TimeInput supporting text (`Hour` / `Minute`).
    pub const fn support_label(self) -> &'static str {
        match self {
            Self::Hour => INPUT_HOUR_LABEL,
            Self::Minute => INPUT_MINUTE_LABEL,
        }
    }

    /// Official TimeSelector contentDescription (`Select hour` / `Select minutes`).
    pub const fn selection_label(self) -> &'static str {
        match self {
            Self::Hour => HOUR_SELECTION,
            Self::Minute => MINUTE_SELECTION,
        }
    }

    /// Official TimeInput text-field a11y (`for hour` / `for minutes`).
    pub const fn input_field_label(self) -> &'static str {
        match self {
            Self::Hour => INPUT_HOUR_FIELD,
            Self::Minute => INPUT_MINUTE_FIELD,
        }
    }

    pub const fn count(self) -> usize {
        match self {
            Self::Hour => HOUR_COUNT,
            Self::Minute => MINUTE_COUNT,
        }
    }
}

/// Compose `rememberScrollFieldState` (wrapping wheel).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollField {
    pub kind: ScrollKind,
    /// Hour wheel uses this; minutes ignore it.
    pub format: TimeFormat,
    /// Item-space offset; 0 centers index 0. Always wrapped into `0..count`.
    pub offset: f32,
    /// Items per second (positive = later values move toward center).
    pub velocity: f32,
}

impl ScrollField {
    pub fn hour(hour: u8) -> Self {
        Self::hour_with(hour, TimeFormat::Hour12)
    }

    pub fn hour_with(hour: u8, format: TimeFormat) -> Self {
        Self {
            kind: ScrollKind::Hour,
            format,
            offset: hour_index_for(hour, format),
            velocity: 0.0,
        }
    }

    pub fn minute(minute: u8) -> Self {
        Self {
            kind: ScrollKind::Minute,
            format: TimeFormat::Hour12,
            offset: minute_index(minute),
            velocity: 0.0,
        }
    }

    pub fn count(self) -> usize {
        match self.kind {
            ScrollKind::Hour => self.format.hour_count(),
            ScrollKind::Minute => MINUTE_COUNT,
        }
    }

    pub fn selected_index(self) -> usize {
        wrap_index(self.offset.round() as i32, self.count())
    }

    pub fn selected_value(self) -> u8 {
        match self.kind {
            ScrollKind::Hour => hour_from_index_for(self.selected_index(), self.format),
            ScrollKind::Minute => minute_from_index(self.selected_index()),
        }
    }

    pub fn apply_delta_dp(&mut self, dy_dp: f32) {
        let count = self.count();
        self.offset = wrap_offset(self.offset - dy_dp / SCROLL_ITEM_H_DP, count);
    }

    pub fn impulse(&mut self, items_per_sec: f32) {
        self.velocity += items_per_sec;
    }

    pub fn snap_to_index(&mut self, index: usize) {
        let count = self.count();
        let target = wrap_index(index as i32, count) as f32;
        self.offset = shortest_target(self.offset, target, count);
        self.velocity = 0.0;
    }

    pub fn step(&mut self, dt_s: f32) -> f32 {
        let dt = dt_s.clamp(0.0, 0.05);
        let count = self.count();
        if self.velocity.abs() >= SCROLL_FLING_REST {
            self.offset = wrap_offset(self.offset + self.velocity * dt, count);
            self.velocity *= (-SCROLL_FLING_DECAY * dt).exp();
            if self.velocity.abs() < SCROLL_FLING_REST {
                self.velocity = 0.0;
            }
        } else {
            self.velocity = 0.0;
            let target = self.offset.round();
            let delta = shortest_delta(self.offset, target, count);
            if delta.abs() < 0.002 {
                self.offset = wrap_offset(target, count);
            } else {
                self.offset = wrap_offset(
                    self.offset + delta * (1.0 - (-SCROLL_SNAP_STIFFNESS * dt).exp()),
                    count,
                );
            }
        }
        self.offset
    }

    pub fn step_live(&mut self, dt_s: f32) -> f32 {
        self.step(dt_s)
    }

    pub fn resting(self) -> bool {
        self.velocity.abs() < SCROLL_FLING_REST
            && shortest_delta(self.offset, self.offset.round(), self.count()).abs() < 0.002
    }

    pub fn needs_frame(self) -> bool {
        !self.resting()
    }

    pub fn slots(self) -> Vec<ScrollSlot> {
        let count = self.count();
        let center = (SCROLL_FIELD_H_DP - SCROLL_ITEM_H_DP) / 2.0;
        let mut out = Vec::new();
        let base = self.offset.floor() as i32;
        for rel in -SCROLL_SLOT_SPAN..=SCROLL_SLOT_SPAN {
            let logical = base + rel;
            let index = wrap_index(logical, count);
            let y = (logical as f32 - self.offset) * SCROLL_ITEM_H_DP + center;
            let dist = (logical as f32 - self.offset).abs();
            let selected = dist < 0.5;
            let opacity = (1.0 - dist * 0.42).clamp(0.28, 1.0);
            let value = match self.kind {
                ScrollKind::Hour => hour_from_index_for(index, self.format),
                ScrollKind::Minute => minute_from_index(index),
            };
            out.push(ScrollSlot {
                index,
                value,
                label: match self.kind {
                    ScrollKind::Hour => format_hour_field_for(value, self.format),
                    ScrollKind::Minute => format_minute_field(value),
                },
                y_dp: y,
                selected,
                opacity,
            });
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollSlot {
    pub index: usize,
    pub value: u8,
    pub label: String,
    pub y_dp: f32,
    pub selected: bool,
    pub opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeScrollState {
    pub hour: ScrollField,
    pub minute: ScrollField,
    pub format: TimeFormat,
}

impl TimeScrollState {
    pub fn demo() -> Self {
        Self::from_clock(demo_hour(DEMO_FORMAT), DEMO_MINUTE, DEMO_FORMAT)
    }

    pub fn from_clock(hour: u8, minute: u8, format: TimeFormat) -> Self {
        Self {
            hour: ScrollField::hour_with(hour, format),
            minute: ScrollField::minute(minute),
            format,
        }
    }

    pub fn apply_format(&mut self, format: TimeFormat, hour: u8) {
        self.format = format;
        self.hour = ScrollField::hour_with(hour, format);
    }

    pub fn hour_value(self) -> u8 {
        self.hour.selected_value()
    }

    pub fn minute_value(self) -> u8 {
        self.minute.selected_value()
    }

    pub fn field_mut(&mut self, kind: ScrollKind) -> &mut ScrollField {
        match kind {
            ScrollKind::Hour => &mut self.hour,
            ScrollKind::Minute => &mut self.minute,
        }
    }

    pub fn step_live(&mut self, dt_s: f32) -> bool {
        self.hour.step_live(dt_s);
        self.minute.step_live(dt_s);
        self.needs_frame()
    }

    pub fn resting(self) -> bool {
        self.hour.resting() && self.minute.resting()
    }

    pub fn needs_frame(self) -> bool {
        self.hour.needs_frame() || self.minute.needs_frame()
    }

    pub fn step_until_rest(&mut self) {
        for _ in 0..180 {
            if self.resting() {
                break;
            }
            self.step_live(SCROLL_FRAME_DT);
        }
        if !self.resting() {
            self.hour.snap_to_index(self.hour.selected_index());
            self.minute.snap_to_index(self.minute.selected_index());
        }
    }
}

/// Wheel / trackpad: add leftover velocity (host ticks `step_live`).
pub fn apply_wheel(field: &mut ScrollField, dy_dp: f32) {
    // Scroll down (positive) advances the wheel toward later values.
    field.impulse(dy_dp / SCROLL_ITEM_H_DP * 8.0);
    field.apply_delta_dp(-dy_dp * 0.15);
}

pub fn hour_index(hour: u8) -> f32 {
    hour_index_for(hour, TimeFormat::Hour12)
}

pub fn hour_index_for(hour: u8, format: TimeFormat) -> f32 {
    match format {
        TimeFormat::Hour12 => (hour.clamp(1, 12) - 1) as f32,
        TimeFormat::Hour24 => hour.min(23) as f32,
    }
}

pub fn minute_index(minute: u8) -> f32 {
    minute.min(59) as f32
}

pub fn hour_from_index(index: usize) -> u8 {
    hour_from_index_for(index, TimeFormat::Hour12)
}

pub fn hour_from_index_for(index: usize, format: TimeFormat) -> u8 {
    match format {
        TimeFormat::Hour12 => ((index % HOUR_COUNT) + 1) as u8,
        TimeFormat::Hour24 => (index % HOUR24_COUNT) as u8,
    }
}

pub fn minute_from_index(index: usize) -> u8 {
    (index % MINUTE_COUNT) as u8
}

pub fn wrap_index(index: i32, count: usize) -> usize {
    let c = count as i32;
    (((index % c) + c) % c) as usize
}

pub fn wrap_offset(offset: f32, count: usize) -> f32 {
    let c = count as f32;
    let mut o = offset % c;
    if o < 0.0 {
        o += c;
    }
    o
}

fn shortest_delta(from: f32, to: f32, count: usize) -> f32 {
    let c = count as f32;
    let mut d = to - from;
    if d > c / 2.0 {
        d -= c;
    } else if d < -c / 2.0 {
        d += c;
    }
    d
}

fn shortest_target(from: f32, to: f32, count: usize) -> f32 {
    wrap_offset(from + shortest_delta(from, to, count), count)
}

/// Compose `TimePickerDefaults.vibrantColors()` + `TimeScroll` field tokens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeScrollAppearance {
    pub corners: Corners,
    pub container: Argb,
    pub header: Argb,
    pub field_container: Argb,
    pub field_corners: Corners,
    pub selected: Argb,
    pub unselected: Argb,
    pub colon: Argb,
    pub period_selected_container: Argb,
    pub period_selected: Argb,
    pub period_idle_container: Argb,
    pub period_idle: Argb,
    /// `TimePickerTokens.PeriodSelectorOutlineColor`.
    pub period_outline: Argb,
    pub elevation_dp: f32,
    pub field_w_dp: f32,
    pub field_h_dp: f32,
    pub item_h_dp: f32,
    pub title_style: TypeStyle,
    pub selected_style: TypeStyle,
    pub unselected_style: TypeStyle,
    pub colon_style: TypeStyle,
    pub period_style: TypeStyle,
}

/// Vibrant TimeScroll (recommended Expressive hero).
pub fn resolve_scroll(theme: &Theme) -> TimeScrollAppearance {
    let c = theme.color;
    TimeScrollAppearance {
        corners: Corners::all(CORNER_DP),
        container: c.primary_container,
        header: c.on_primary_container,
        field_container: c.surface_container_highest,
        field_corners: Corners::all(SCROLL_FIELD_CORNER_DP),
        selected: c.on_surface,
        unselected: c.on_surface_variant,
        colon: c.on_primary_container,
        period_selected_container: c.on_primary_container,
        period_selected: c.primary_container,
        period_idle_container: c.primary,
        period_idle: c.on_primary,
        period_outline: c.outline,
        elevation_dp: theme.elevation.level3,
        field_w_dp: SCROLL_FIELD_W_DP,
        field_h_dp: SCROLL_FIELD_H_DP,
        item_h_dp: SCROLL_ITEM_H_DP,
        title_style: theme.typography.label_medium,
        selected_style: theme.typography.display_large.emphasized(),
        unselected_style: theme.typography.display_medium,
        colon_style: theme.typography.display_large,
        period_style: theme.typography.title_medium.emphasized(),
    }
}

/// Compose `TimeInput` + `TimeInputDefaults.vibrantColors()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeInputAppearance {
    pub corners: Corners,
    pub container: Argb,
    pub header: Argb,
    pub field_container: Argb,
    pub field_focused: Argb,
    pub field_corners: Corners,
    pub field_content: Argb,
    pub field_focused_content: Argb,
    pub colon: Argb,
    pub period_selected_container: Argb,
    pub period_selected: Argb,
    pub period_idle_container: Argb,
    pub period_idle: Argb,
    pub toggle: Argb,
    pub elevation_dp: f32,
    pub field_w_dp: f32,
    pub field_h_dp: f32,
    pub period_w_dp: f32,
    pub period_h_dp: f32,
    pub title_style: TypeStyle,
    pub field_style: TypeStyle,
    pub colon_style: TypeStyle,
    pub period_style: TypeStyle,
    /// `TimeInputTokens.TimeFieldSupportingTextColor`.
    pub support_label: Argb,
    /// `TimeInputTokens.TimeFieldSupportingTextFont`.
    pub support_label_style: TypeStyle,
    /// Vibrant unfocused outline (`Transparent`).
    pub field_outline: Argb,
    /// Vibrant focused outline (`Primary`, 2dp).
    pub field_focused_outline: Argb,
    /// `TimePickerTokens.PeriodSelectorOutlineColor`.
    pub period_outline: Argb,
}

pub fn resolve_input(theme: &Theme) -> TimeInputAppearance {
    let c = theme.color;
    TimeInputAppearance {
        corners: Corners::all(CORNER_DP),
        container: c.primary_container,
        header: c.on_primary_container,
        field_container: c.surface_container_lowest,
        field_focused: c.surface_container_lowest,
        field_corners: Corners::all(INPUT_FIELD_CORNER_DP),
        field_content: c.on_surface,
        field_focused_content: c.primary,
        colon: c.on_primary_container,
        period_selected_container: c.on_primary_container,
        period_selected: c.primary_container,
        period_idle_container: c.primary,
        period_idle: c.on_primary,
        toggle: c.on_primary_container,
        elevation_dp: theme.elevation.level3,
        field_w_dp: INPUT_FIELD_W_DP,
        field_h_dp: INPUT_FIELD_H_DP,
        period_w_dp: INPUT_PERIOD_W_DP,
        period_h_dp: INPUT_PERIOD_H_DP,
        title_style: theme.typography.label_medium,
        field_style: theme.typography.display_large.emphasized(),
        colon_style: theme.typography.display_large,
        period_style: theme.typography.title_medium.emphasized(),
        support_label: c.on_surface_variant,
        support_label_style: theme.typography.body_small,
        field_outline: Argb::TRANSPARENT,
        field_focused_outline: c.primary,
        period_outline: c.outline,
    }
}

pub fn resolve_toggle(theme: &Theme) -> (f32, Argb) {
    (TOGGLE_SIZE_DP, theme.color.on_primary_container)
}

/// Two-digit TimeInput field (hour 1–12 or 00–23 / minute 00–59).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeInputField {
    pub kind: ScrollKind,
    pub digits: String,
    pub format: TimeFormat,
}

impl TimeInputField {
    pub fn hour(hour: u8) -> Self {
        Self::hour_with(hour, TimeFormat::Hour12)
    }

    pub fn hour_with(hour: u8, format: TimeFormat) -> Self {
        Self {
            kind: ScrollKind::Hour,
            digits: format_hour_field_for(hour, format),
            format,
        }
    }

    pub fn minute(minute: u8) -> Self {
        Self {
            kind: ScrollKind::Minute,
            digits: format_minute_field(minute),
            format: TimeFormat::Hour12,
        }
    }

    pub fn display(&self) -> String {
        match self.digits.len() {
            0 => "--".into(),
            1 => format!("0{}", self.digits),
            _ => self.digits.chars().take(2).collect(),
        }
    }

    pub fn value(&self) -> Option<u8> {
        let n: u8 = self.digits.parse().ok()?;
        match self.kind {
            ScrollKind::Hour => (self.format.hour_min()..=self.format.hour_max())
                .contains(&n)
                .then_some(n),
            ScrollKind::Minute => (n <= 59).then_some(n),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.value().is_some()
    }

    pub fn backspace(&mut self) {
        self.digits.pop();
    }

    /// Returns true when the caret should advance to the next field.
    pub fn apply_digit(&mut self, digit: char) -> bool {
        if !digit.is_ascii_digit() {
            return false;
        }
        if self.digits.len() >= 2 {
            self.digits.clear();
        }
        let next = format!("{}{digit}", self.digits);
        let n: u8 = next.parse().unwrap_or(99);
        let accept = match self.kind {
            ScrollKind::Hour => n <= self.format.hour_max(),
            ScrollKind::Minute => n <= 59,
        };
        if accept {
            self.digits = next;
        } else if self.digits.is_empty() {
            return false;
        } else {
            self.digits.clear();
            return self.apply_digit(digit);
        }
        match self.kind {
            ScrollKind::Hour => {
                let first_done = if self.format.is_24_hour() { "3" } else { "2" };
                self.digits.len() == 2
                    || (self.digits.len() == 1 && self.digits.as_str() >= first_done)
            }
            ScrollKind::Minute => self.digits.len() == 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeInputState {
    pub hour: TimeInputField,
    pub minute: TimeInputField,
    pub focus: ScrollKind,
    pub format: TimeFormat,
}

impl TimeInputState {
    pub fn demo() -> Self {
        Self::from_clock(demo_hour(DEMO_FORMAT), DEMO_MINUTE, DEMO_FORMAT)
    }

    pub fn from_clock(hour: u8, minute: u8, format: TimeFormat) -> Self {
        Self {
            hour: TimeInputField::hour_with(hour, format),
            minute: TimeInputField::minute(minute),
            focus: ScrollKind::Hour,
            format,
        }
    }

    pub fn hour_value(&self) -> Option<u8> {
        self.hour.value()
    }

    pub fn minute_value(&self) -> Option<u8> {
        self.minute.value()
    }

    pub fn is_input_valid(&self) -> bool {
        self.hour.is_valid() && self.minute.is_valid()
    }

    pub fn focused_mut(&mut self) -> &mut TimeInputField {
        match self.focus {
            ScrollKind::Hour => &mut self.hour,
            ScrollKind::Minute => &mut self.minute,
        }
    }

    pub fn apply_key(&mut self, key: &str) {
        match key {
            "backspace" | "delete" => self.focused_mut().backspace(),
            "left" => self.focus = ScrollKind::Hour,
            "right" | "tab" => self.focus = ScrollKind::Minute,
            k if k.len() == 1 => {
                if let Some(ch) = k.chars().next() {
                    if self.focused_mut().apply_digit(ch) && self.focus == ScrollKind::Hour {
                        self.focus = ScrollKind::Minute;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn commit_or(&self, hour: u8, minute: u8) -> (u8, u8) {
        (
            self.hour_value()
                .unwrap_or(hour.clamp(self.format.hour_min(), self.format.hour_max())),
            self.minute_value().unwrap_or(minute.min(59)),
        )
    }
}

pub fn apply_display_toggle(
    mode: TimePickerDisplayMode,
    scroll: &mut TimeScrollState,
    input: &mut TimeInputState,
    hour: &mut u8,
    minute: &mut u8,
) -> TimePickerDisplayMode {
    let format = scroll.format;
    let next = mode.toggle();
    match next {
        TimePickerDisplayMode::Input => {
            *hour = scroll.hour_value();
            *minute = scroll.minute_value();
            *input = TimeInputState::from_clock(*hour, *minute, format);
        }
        TimePickerDisplayMode::Scroll => {
            let (h, m) = input.commit_or(*hour, *minute);
            *hour = h;
            *minute = m;
            scroll
                .hour
                .snap_to_index(hour_index_for(h, format) as usize);
            scroll.minute.snap_to_index(minute_index(m) as usize);
        }
    }
    next
}

pub fn apply_format_toggle(
    format: TimeFormat,
    period: &mut DayPeriod,
    scroll: &mut TimeScrollState,
    input: &mut TimeInputState,
    hour: &mut u8,
    minute: u8,
) -> TimeFormat {
    let next = format.toggle();
    match next {
        TimeFormat::Hour24 => {
            *hour = to_hour24(*hour, *period);
        }
        TimeFormat::Hour12 => {
            let (h, p) = to_hour12(*hour);
            *hour = h;
            *period = p;
        }
    }
    scroll.apply_format(next, *hour);
    *input = TimeInputState::from_clock(*hour, minute, next);
    next
}
