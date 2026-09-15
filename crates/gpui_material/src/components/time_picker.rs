//! Time picker (12-hour dial + Expressive TimeScroll + TimeInput).
//! Specs: https://m3.material.io/components/time-pickers/specs
//!
//! Dial: hour and minute faces plus an analog selector hand. GPUI/HTML
//! interpolate the hand angle when the face or value changes (spatial-fast).
//!
//! Expressive (I/O 2026, recommended): Compose `TimeScroll` with two
//! `ScrollField`s (hours + minutes), `TimePickerDefaults.vibrantColors()`,
//! and `ScrollFieldDefaults.ScrollFieldHeight` 200. `TimeInput` (96×72
//! fields) + `ScrollDisplayModeToggle` switch Scroll ↔ Input. Dial remains.

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
        DialFace::Hour => hour_face_live_angle_deg(hour, minute, 0.0),
        DialFace::Minute => (minute.min(59) as f32) * 6.0,
    }
}

/// Continuous hour-face motion: `tick` in 0..=1 adds a fraction of a minute
/// so the analog hand eases while the hour dial is showing.
pub fn hour_face_live_angle_deg(hour: u8, minute: u8, tick: f32) -> f32 {
    let h = if hour == 0 { 12 } else { hour };
    let minutes = minute as f32 + tick.clamp(0.0, 1.0);
    (h as f32) * 30.0 + minutes * 0.5
}

/// Degrees from 12 o'clock for a ticking second hand (`tick` is the
/// in-second fraction 0..=1). A 60 s repeating clock uses `tick = delta`.
pub fn second_hand_angle_deg(second: u8, tick: f32) -> f32 {
    (second.min(59) as f32 + tick.clamp(0.0, 1.0)) * 6.0
}

/// Center of the selector knob at an arbitrary clock angle.
pub fn hand_end_at_angle(clock_dp: f32, angle_deg: f32, number_dp: f32) -> (f32, f32) {
    let (x, y) = polar_offset(angle_deg - 90.0, clock_dp, number_dp);
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
    let (ex0, ey0) = hand_end_at_angle(clock_dp, angle_deg, number_dp);
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

    /// Icon for the *other* mode (keyboard when scrolling, schedule when typing).
    pub const fn toggle_icon(self) -> &'static str {
        match self {
            Self::Scroll => KEYBOARD_ICON,
            Self::Input => SCHEDULE_ICON,
        }
    }

    pub const fn toggle_label(self) -> &'static str {
        match self {
            Self::Scroll => "Switch to input mode",
            Self::Input => "Switch to scroll mode",
        }
    }
}

/// Catalog / host hero starts on TimeScroll; toggle paints TimeInput.
pub const DEMO_DISPLAY_MODE: TimePickerDisplayMode = TimePickerDisplayMode::Scroll;
/// Compose `TimePickerDialogDefaults.ScrollDisplayModeToggle` 48dp target.
pub const TOGGLE_SIZE_DP: f32 = 48.0;
pub const TOGGLE_ICON_DP: f32 = 24.0;
/// Keyboard — switch Scroll → Input.
pub const KEYBOARD_ICON: &str = "⌨";
/// Schedule / clock — switch Input → Scroll.
pub const SCHEDULE_ICON: &str = "◷";

/// Time-input field tokens (`TimeInputTokens.TimeFieldContainer*`).
pub const INPUT_FIELD_W_DP: f32 = 96.0;
pub const INPUT_FIELD_H_DP: f32 = 72.0;
/// `TimePickerDefaults.shapes().timeFieldShape` / extra-large.
pub const INPUT_FIELD_CORNER_DP: f32 = 28.0;
pub const INPUT_PERIOD_W_DP: f32 = 52.0;
pub const INPUT_PERIOD_H_DP: f32 = 72.0;
pub const INPUT_GAP_DP: f32 = 24.0;
pub const INPUT_COLON_GAP_DP: f32 = 8.0;

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
/// `TimePickerDefaults.shapes().timeFieldShape` / extra-large.
pub const SCROLL_FIELD_CORNER_DP: f32 = 28.0;
/// Colon `offset(y = (-4).dp)` in the Compose time-selection sample.
pub const SCROLL_COLON_OFFSET_Y_DP: f32 = -4.0;
pub const HOUR_COUNT: usize = 12;
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
    /// Item-space offset; 0 centers index 0. Always wrapped into `0..count`.
    pub offset: f32,
    /// Items per second (positive = later values move toward center).
    pub velocity: f32,
}

impl ScrollField {
    pub fn hour(hour: u8) -> Self {
        Self {
            kind: ScrollKind::Hour,
            offset: hour_index(hour),
            velocity: 0.0,
        }
    }

    pub fn minute(minute: u8) -> Self {
        Self {
            kind: ScrollKind::Minute,
            offset: minute_index(minute),
            velocity: 0.0,
        }
    }

    pub fn count(self) -> usize {
        self.kind.count()
    }

    pub fn selected_index(self) -> usize {
        wrap_index(self.offset.round() as i32, self.count())
    }

    pub fn selected_value(self) -> u8 {
        match self.kind {
            ScrollKind::Hour => hour_from_index(self.selected_index()),
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
                ScrollKind::Hour => hour_from_index(index),
                ScrollKind::Minute => minute_from_index(index),
            };
            out.push(ScrollSlot {
                index,
                value,
                label: match self.kind {
                    ScrollKind::Hour => format_hour_field(value),
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
}

impl TimeScrollState {
    pub fn demo() -> Self {
        Self {
            hour: ScrollField::hour(DEMO_HOUR),
            minute: ScrollField::minute(DEMO_MINUTE),
        }
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
    (hour.clamp(1, 12) - 1) as f32
}

pub fn minute_index(minute: u8) -> f32 {
    minute.min(59) as f32
}

pub fn hour_from_index(index: usize) -> u8 {
    ((index % HOUR_COUNT) + 1) as u8
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
        elevation_dp: theme.elevation.level3,
        field_w_dp: SCROLL_FIELD_W_DP,
        field_h_dp: SCROLL_FIELD_H_DP,
        item_h_dp: SCROLL_ITEM_H_DP,
        title_style: theme.typography.label_large,
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
}

pub fn resolve_input(theme: &Theme) -> TimeInputAppearance {
    let c = theme.color;
    TimeInputAppearance {
        corners: Corners::all(CORNER_DP),
        container: c.primary_container,
        header: c.on_primary_container,
        field_container: c.surface_container_highest,
        field_focused: c.on_primary_container,
        field_corners: Corners::all(INPUT_FIELD_CORNER_DP),
        field_content: c.on_surface,
        field_focused_content: c.primary_container,
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
        title_style: theme.typography.label_large,
        field_style: theme.typography.display_large.emphasized(),
        colon_style: theme.typography.display_large,
        period_style: theme.typography.title_medium.emphasized(),
    }
}

pub fn resolve_toggle(theme: &Theme) -> (f32, Argb) {
    (TOGGLE_SIZE_DP, theme.color.on_primary_container)
}

/// Two-digit TimeInput field (hour 1–12 / minute 00–59).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeInputField {
    pub kind: ScrollKind,
    pub digits: String,
}

impl TimeInputField {
    pub fn hour(hour: u8) -> Self {
        Self {
            kind: ScrollKind::Hour,
            digits: format_hour_field(hour),
        }
    }

    pub fn minute(minute: u8) -> Self {
        Self {
            kind: ScrollKind::Minute,
            digits: format_minute_field(minute),
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
            ScrollKind::Hour => (1..=12).contains(&n).then_some(n),
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
            ScrollKind::Hour => n <= 12,
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
                self.digits.len() == 2 || (self.digits.len() == 1 && self.digits.as_str() >= "2")
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
}

impl TimeInputState {
    pub fn demo() -> Self {
        Self::from_clock(DEMO_HOUR, DEMO_MINUTE)
    }

    pub fn from_clock(hour: u8, minute: u8) -> Self {
        Self {
            hour: TimeInputField::hour(hour),
            minute: TimeInputField::minute(minute),
            focus: ScrollKind::Hour,
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
            self.hour_value().unwrap_or(hour.clamp(1, 12)),
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
    let next = mode.toggle();
    match next {
        TimePickerDisplayMode::Input => {
            *hour = scroll.hour_value();
            *minute = scroll.minute_value();
            *input = TimeInputState::from_clock(*hour, *minute);
        }
        TimePickerDisplayMode::Scroll => {
            let (h, m) = input.commit_or(*hour, *minute);
            *hour = h;
            *minute = m;
            scroll.hour.snap_to_index(hour_index(h) as usize);
            scroll.minute.snap_to_index(minute_index(m) as usize);
        }
    }
    next
}
