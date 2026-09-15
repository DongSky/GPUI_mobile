//! Slider — M3 Expressive (current m3.material.io / SliderTokens).
//! Specs: https://m3.material.io/components/sliders/specs
//!
//! Default size is XS: 16dp track, 4×44 handle, 6dp gap, 2dp inner corner,
//! 4dp stop indicators. Handle compresses to 2dp when pressed.

use crate::argb::Argb;
use crate::state::InteractionState;
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl SliderSize {
    pub const ALL: [Self; 5] = [
        Self::ExtraSmall,
        Self::Small,
        Self::Medium,
        Self::Large,
        Self::ExtraLarge,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ExtraSmall => "xs",
            Self::Small => "s",
            Self::Medium => "m",
            Self::Large => "l",
            Self::ExtraLarge => "xl",
        }
    }

    pub const fn track_h(self) -> f32 {
        match self {
            Self::ExtraSmall => 16.0,
            Self::Small => 24.0,
            Self::Medium => 40.0,
            Self::Large => 56.0,
            Self::ExtraLarge => 96.0,
        }
    }

    pub const fn track_corner(self) -> f32 {
        match self {
            Self::ExtraSmall | Self::Small => 8.0,
            Self::Medium => 12.0,
            Self::Large => 16.0,
            Self::ExtraLarge => 28.0,
        }
    }

    pub const fn handle_h(self) -> f32 {
        match self {
            Self::ExtraSmall | Self::Small => 44.0,
            Self::Medium => 52.0,
            Self::Large => 68.0,
            Self::ExtraLarge => 108.0,
        }
    }
}

pub const TRACK_HEIGHT_DP: f32 = 16.0;
pub const HANDLE_W_DP: f32 = 4.0;
pub const HANDLE_W_PRESSED_DP: f32 = 2.0;
pub const HANDLE_H_DP: f32 = 44.0;
pub const GAP_DP: f32 = 6.0;
pub const INNER_CORNER_DP: f32 = 2.0;
pub const STOP_DP: f32 = 4.0;
pub const TARGET_DP: f32 = 48.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderAppearance {
    pub track_h: f32,
    pub track_corner: f32,
    pub handle_w: f32,
    pub handle_h: f32,
    /// Painted handle height. Token `handle_h` stays the 44dp touch target;
    /// official overview photos are closer to track + 12dp (~28dp on XS).
    pub handle_h_visual: f32,
    pub gap_dp: f32,
    pub inner_corner: f32,
    pub stop_dp: f32,
    pub target_dp: f32,
    pub active: Argb,
    pub inactive: Argb,
    pub handle: Argb,
    pub stop_active: Argb,
    pub stop_inactive: Argb,
    pub value: f32,
    /// Discrete stop indicators along the track. Default 2 = ends only.
    /// Official overview Alarm row uses mid-track stops.
    pub stop_count: usize,
}

/// One volume-like row as on the official sliders overview.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverviewRow {
    pub icon: &'static str,
    pub label: &'static str,
    pub value: f32,
    pub stop_count: usize,
}

/// Official overview scene: Call / Alarm / Ring / Media.
/// Alarm is the mid-stop example.
pub const OVERVIEW_ROWS: [OverviewRow; 4] = [
    OverviewRow {
        icon: "☎",
        label: "Call volume",
        value: 0.35,
        stop_count: 2,
    },
    OverviewRow {
        icon: "⏰",
        label: "Alarm volume",
        value: 0.52,
        stop_count: 13,
    },
    OverviewRow {
        icon: "🔔",
        label: "Ring volume",
        value: 0.72,
        stop_count: 2,
    },
    OverviewRow {
        icon: "♪",
        label: "Media volume",
        value: 0.85,
        stop_count: 2,
    },
];

/// Evenly spaced stop positions in 0..=1.
pub fn stop_fractions(count: usize) -> Vec<f32> {
    let n = count.max(2);
    (0..n).map(|i| i as f32 / (n as f32 - 1.0)).collect()
}

/// How many stop dots belong on the active vs inactive segments.
pub fn segmented_stop_counts(value: f32, count: usize) -> (usize, usize) {
    let stops = stop_fractions(count);
    let active = stops.iter().filter(|s| **s <= value + 0.001).count();
    let inactive = stops.len().saturating_sub(active);
    (active.max(1), inactive.max(1))
}

pub fn resolve(theme: &Theme, value: f32, state: InteractionState) -> SliderAppearance {
    resolve_size(theme, SliderSize::ExtraSmall, value, state)
}

pub fn resolve_size(
    theme: &Theme,
    size: SliderSize,
    value: f32,
    state: InteractionState,
) -> SliderAppearance {
    let c = theme.color;
    let value = value.clamp(0.0, 1.0);
    let pressed = matches!(state, InteractionState::Pressed | InteractionState::Focused);
    let handle_w = if pressed {
        HANDLE_W_PRESSED_DP
    } else {
        HANDLE_W_DP
    };
    let (active, inactive, handle, stop_active, stop_inactive) = if state.is_disabled() {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        let track = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTAINER_OPACITY)
            .composite_over(c.surface);
        (muted, track, muted, muted, track)
    } else {
        (
            // MDC-Android Slider: active = colorPrimary,
            // inactive = colorSurfaceContainerHighest
            // (https://github.com/material-components/material-components-android/blob/master/docs/components/Slider.md).
            c.primary,
            c.surface_container_highest,
            c.primary,
            c.on_primary,
            c.primary,
        )
    };
    SliderAppearance {
        track_h: size.track_h(),
        track_corner: size.track_corner(),
        handle_w,
        handle_h: size.handle_h(),
        handle_h_visual: size.track_h() + 12.0,
        gap_dp: GAP_DP,
        inner_corner: INNER_CORNER_DP,
        stop_dp: STOP_DP,
        target_dp: size.handle_h().max(TARGET_DP),
        active,
        inactive,
        handle,
        stop_active,
        stop_inactive,
        value,
        stop_count: 2,
    }
}

pub fn resolve_with_stops(
    theme: &Theme,
    value: f32,
    state: InteractionState,
    stop_count: usize,
) -> SliderAppearance {
    let mut a = resolve(theme, value, state);
    a.stop_count = stop_count.max(2);
    a
}

pub fn resolve_size_with_stops(
    theme: &Theme,
    size: SliderSize,
    value: f32,
    state: InteractionState,
    stop_count: usize,
) -> SliderAppearance {
    let mut a = resolve_size(theme, size, value, state);
    a.stop_count = stop_count.max(2);
    a
}

/// Dual-handle range slider (M3 Expressive / official dual-thumb pattern).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeSliderAppearance {
    pub track: SliderAppearance,
    pub start: f32,
    pub end: f32,
}

pub const RANGE_DEMO_START: f32 = 0.20;
pub const RANGE_DEMO_END: f32 = 0.75;
pub const RANGE_HERO_LABEL: &str = "Price range";
pub const RANGE_STEP: f32 = 0.05;
/// Thumbs cannot cross closer than this (5% of the track). Documented in the
/// range label and inventory; click/keyboard steps use the same 5% grid.
pub const RANGE_MIN_SPAN: f32 = 0.05;
/// When true, pointer-drag snaps to `RANGE_STEP` ticks (M3 discrete dual-thumb).
pub const RANGE_SNAP_WHILE_DRAG: bool = true;

/// Snap `v` onto the 5% tick grid used by click/keyboard.
pub fn snap_to_step(v: f32) -> f32 {
    ((v / RANGE_STEP).round() * RANGE_STEP).clamp(0.0, 1.0)
}

pub fn clamp_range(start: f32, end: f32) -> (f32, f32) {
    let start = start.clamp(0.0, 1.0 - RANGE_MIN_SPAN);
    let end = end.clamp(start + RANGE_MIN_SPAN, 1.0);
    (start, end)
}

pub fn nudge_start(start: f32, end: f32, delta: f32) -> (f32, f32) {
    clamp_range(start + delta, end)
}

pub fn nudge_end(start: f32, end: f32, delta: f32) -> (f32, f32) {
    clamp_range(start, end + delta)
}

/// Move the nearest thumb to `value` (0..=1). Used by track taps.
pub fn move_nearest(start: f32, end: f32, value: f32) -> (f32, f32) {
    let value = value.clamp(0.0, 1.0);
    if (value - start).abs() <= (value - end).abs() {
        clamp_range(value, end)
    } else {
        clamp_range(start, value)
    }
}

pub fn range_value_label(start: f32, end: f32) -> String {
    format!(
        "{} · {:.0}–{:.0}% · min span {:.0}%",
        RANGE_HERO_LABEL,
        start * 100.0,
        end * 100.0,
        RANGE_MIN_SPAN * 100.0
    )
}

/// Which handle is being dragged or keyboard-focused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeThumb {
    Start,
    End,
}

impl RangeThumb {
    pub const fn toggle(self) -> Self {
        match self {
            Self::Start => Self::End,
            Self::End => Self::Start,
        }
    }
}

/// Legacy 5% overlay used before local-X mapping. Kept for tests.
pub const RANGE_DRAG_CELLS: u32 = 21;
/// Catalog track width (desktop). Android uses 240dp.
pub const RANGE_TRACK_W_DP: f32 = 280.0;

pub fn nearest_thumb(start: f32, end: f32, fraction: f32) -> RangeThumb {
    if (fraction - start).abs() <= (fraction - end).abs() {
        RangeThumb::Start
    } else {
        RangeThumb::End
    }
}

/// Pointer-drag: move `thumb` to `fraction` (0..=1) without crossing.
pub fn drag_thumb(start: f32, end: f32, thumb: RangeThumb, fraction: f32) -> (f32, f32) {
    match thumb {
        RangeThumb::Start => clamp_range(fraction, end),
        RangeThumb::End => clamp_range(start, fraction),
    }
}

/// Pointer-drag with optional tick-snap (`RANGE_SNAP_WHILE_DRAG`).
pub fn drag_thumb_snapped(
    start: f32,
    end: f32,
    thumb: RangeThumb,
    fraction: f32,
) -> (f32, f32) {
    let fraction = if RANGE_SNAP_WHILE_DRAG {
        snap_to_step(fraction)
    } else {
        fraction
    };
    drag_thumb(start, end, thumb, fraction)
}

pub fn nudge_thumb(start: f32, end: f32, thumb: RangeThumb, delta: f32) -> (f32, f32) {
    match thumb {
        RangeThumb::Start => nudge_start(start, end, delta),
        RangeThumb::End => nudge_end(start, end, delta),
    }
}

/// Keep min-span as one tick. Clicking the track snaps the nearest thumb
/// onto the 5% grid (M3 discrete dual-thumb), instead of a separate overlay
/// click-step that fought drag-snap.
pub fn click_step(start: f32, end: f32, fraction: f32) -> (f32, f32) {
    move_nearest(start, end, snap_to_step(fraction))
}

/// 5% tick fractions along a dual-thumb track (21 stops including ends).
pub fn range_tick_fractions() -> Vec<f32> {
    let n = ((1.0 / RANGE_STEP).round() as usize) + 1;
    stop_fractions(n)
}

pub fn range_tick_active(frac: f32, start: f32, end: f32) -> bool {
    frac + 1e-4 >= start && frac - 1e-4 <= end
}

pub fn drag_cell_fraction(index: u32) -> f32 {
    let n = RANGE_DRAG_CELLS.saturating_sub(1).max(1);
    (index as f32 / n as f32).clamp(0.0, 1.0)
}

/// Map a pointer's local X (hitbox left = 0) onto 0..=1.
pub fn fraction_from_local_x(x: f32, width: f32) -> f32 {
    if width <= 0.0 {
        0.0
    } else {
        (x / width).clamp(0.0, 1.0)
    }
}

/// Absolute paint boxes for a dual-thumb track (no flex min-width quantization).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangePaint {
    pub left: f32,
    pub start_handle: f32,
    pub active: f32,
    pub end_handle: f32,
    pub right: f32,
    pub handle_w: f32,
}

/// Discrete 5% tick count for the dual-thumb track (0%, 5%, …, 100%).
pub fn range_tick_count() -> usize {
    range_tick_fractions().len()
}

/// One painted stop on a dual-thumb track.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeTick {
    pub fraction: f32,
    pub x_dp: f32,
    pub active: bool,
}

/// Paint boxes for discrete snap ticks along a `width`-dp dual-thumb track.
pub fn range_ticks(start: f32, end: f32, width: f32, stop_dp: f32) -> Vec<RangeTick> {
    let width = width.max(1.0);
    let half = stop_dp / 2.0;
    range_tick_fractions()
        .into_iter()
        .map(|fraction| RangeTick {
            fraction,
            x_dp: (width * fraction - half).clamp(0.0, (width - stop_dp).max(0.0)),
            active: range_tick_active(fraction, start, end),
        })
        .collect()
}

/// Place thumbs on `start`/`end` of a `width`-dp track.
pub fn range_paint(start: f32, end: f32, width: f32, handle_w: f32) -> RangePaint {
    let hw = handle_w.max(HANDLE_W_DP);
    let width = width.max(hw * 2.0 + RANGE_MIN_SPAN);
    let start_x = (width * start.clamp(0.0, 1.0)).clamp(hw / 2.0, width - hw);
    let end_x = (width * end.clamp(0.0, 1.0)).clamp(start_x + hw, width - hw / 2.0);
    let left = (start_x - hw / 2.0).max(0.0);
    let end_left = end_x - hw / 2.0;
    RangePaint {
        left,
        start_handle: left,
        active: (end_left - (left + hw)).max(0.0),
        end_handle: end_left,
        right: (width - end_left - hw).max(0.0),
        handle_w: hw,
    }
}

/// Arrow / vim keys nudge the focused thumb. `left`/`right`/`h`/`l`.
pub fn apply_arrow(
    start: f32,
    end: f32,
    focus: RangeThumb,
    key: &str,
) -> Option<(f32, f32, RangeThumb)> {
    let delta = match key {
        "left" | "h" | "-" => -RANGE_STEP,
        "right" | "l" | "=" | "+" => RANGE_STEP,
        "up" | "k" => {
            let (s, e) = nudge_thumb(start, end, RangeThumb::Start, RANGE_STEP);
            return Some((s, e, RangeThumb::Start));
        }
        "down" | "j" => {
            let (s, e) = nudge_thumb(start, end, RangeThumb::End, -RANGE_STEP);
            return Some((s, e, RangeThumb::End));
        }
        _ => return None,
    };
    let (s, e) = nudge_thumb(start, end, focus, delta);
    Some((s, e, focus))
}

pub fn resolve_range(
    theme: &Theme,
    start: f32,
    end: f32,
    state: InteractionState,
) -> RangeSliderAppearance {
    let start = start.clamp(0.0, 1.0);
    let end = end.clamp(start, 1.0);
    RangeSliderAppearance {
        track: resolve(theme, (start + end) * 0.5, state),
        start,
        end,
    }
}
