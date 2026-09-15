//! Carousel. Specs: https://m3.material.io/components/carousel/specs
//!
//! Catalog stub: hero large item + smaller neighbors (multi-browse peek).
//! Click / wheel snap the selected index; fling uses velocity/decay so a
//! large delta can skip more than one item.

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const LARGE_W_DP: f32 = 256.0;
pub const SMALL_W_DP: f32 = 120.0;
pub const HEIGHT_DP: f32 = 168.0;
pub const GAP_DP: f32 = 8.0;
pub const CORNER_DP: f32 = 28.0;
pub const ITEMS: [&str; 4] = ["One", "Two", "Three", "Four"];
pub const DEMO_INDEX: usize = 0;
pub const HERO_TITLE: &str = "Hero carousel";
/// Wheel/fling distance that maps to one item. Larger deltas skip further.
pub const FLING_UNIT: f32 = 24.0;
/// Exponential decay per second for a one-shot velocity model (`v * e^{-k t}`).
pub const FLING_DECAY: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarouselAppearance {
    pub large_w_dp: f32,
    pub small_w_dp: f32,
    pub height_dp: f32,
    pub gap_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub neighbor: Argb,
    pub label: Argb,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> CarouselAppearance {
    let c = theme.color;
    CarouselAppearance {
        large_w_dp: LARGE_W_DP,
        small_w_dp: SMALL_W_DP,
        height_dp: HEIGHT_DP,
        gap_dp: GAP_DP,
        corners: Corners::all(CORNER_DP),
        container: c.primary_container,
        neighbor: c.secondary_container,
        label: c.on_primary_container,
        label_style: theme.typography.title_medium.emphasized(),
    }
}

pub fn item_width_dp(index: usize, selected: usize) -> f32 {
    if index == selected {
        LARGE_W_DP
    } else {
        SMALL_W_DP
    }
}

pub fn wrap_index(index: isize) -> usize {
    let n = ITEMS.len() as isize;
    (((index % n) + n) % n) as usize
}

pub fn clamp_index(index: usize) -> usize {
    index.min(ITEMS.len().saturating_sub(1))
}

/// Snap to `target` (used by tile taps).
pub fn snap_to(target: usize) -> usize {
    clamp_index(target)
}

/// One-item fling / wheel step.
pub fn advance(selected: usize, delta: i32) -> usize {
    wrap_index(selected as isize + delta as isize)
}

/// Convert a scroll delta into a signed item step. Magnitude can exceed ±1.
pub fn fling_steps(dx: f32, dy: f32) -> i32 {
    let dominant = if dx.abs() >= dy.abs() { dx } else { dy };
    if dominant.abs() < 0.5 {
        return 0;
    }
    let mag = (dominant.abs() / FLING_UNIT).round() as i32;
    let mag = mag.clamp(1, ITEMS.len() as i32 - 1);
    if dominant > 0.0 { mag } else { -mag }
}

/// Convert a scroll delta into an item step (0 if below the stub threshold).
pub fn fling_step(dx: f32, dy: f32) -> i32 {
    fling_steps(dx, dy)
}

/// Decay leftover fling velocity (`dt_s` in seconds).
pub fn decay_velocity(v: f32, dt_s: f32) -> f32 {
    v * (-FLING_DECAY * dt_s.max(0.0)).exp()
}
