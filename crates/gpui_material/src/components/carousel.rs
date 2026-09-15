//! Carousel. Specs: https://m3.material.io/components/carousel/specs
//!
//! Catalog: hero / multi-browse / uncontained / centered-hero / full-screen.
//! Media tiles use role-color fills (photo stubs) plus a parallax offset
//! while flinging. Centered + full-screen sit in a phone-frame mask.
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
/// Photo-stub captions for media tiles (official uses landscape photos).
pub const MEDIA_CAPTIONS: [&str; 4] = ["Lake", "Grove", "Dune", "Harbor"];
pub const DEMO_INDEX: usize = 0;
pub const HERO_TITLE: &str = "Hero carousel";
pub const MULTI_LARGE_W_DP: f32 = 186.0;
pub const MULTI_SMALL_W_DP: f32 = 56.0;
pub const UNCONTAINED_W_DP: f32 = 220.0;
pub const UNCONTAINED_SMALL_W_DP: f32 = 140.0;
pub const CENTERED_LARGE_W_DP: f32 = 200.0;
pub const CENTERED_SMALL_W_DP: f32 = 72.0;
pub const FULLSCREEN_W_DP: f32 = 336.0;
pub const FULLSCREEN_H_DP: f32 = 420.0;
pub const PARALLAX_MAX_DP: f32 = 12.0;
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 220.0;
pub const PHONE_FULLSCREEN_H_DP: f32 = 480.0;
pub const PHONE_CORNER_DP: f32 = 36.0;
pub const PHONE_BEZEL_DP: f32 = 12.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAxis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselLayout {
    Hero,
    MultiBrowse,
    Uncontained,
    CenteredHero,
    FullScreen,
}

impl CarouselLayout {
    pub const ALL: [Self; 5] = [
        Self::Hero,
        Self::MultiBrowse,
        Self::Uncontained,
        Self::CenteredHero,
        Self::FullScreen,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Hero => "hero",
            Self::MultiBrowse => "multi-browse",
            Self::Uncontained => "uncontained",
            Self::CenteredHero => "centered-hero",
            Self::FullScreen => "full-screen",
        }
    }

    pub const fn large_w(self) -> f32 {
        match self {
            Self::Hero => LARGE_W_DP,
            Self::MultiBrowse => MULTI_LARGE_W_DP,
            Self::Uncontained => UNCONTAINED_W_DP,
            Self::CenteredHero => CENTERED_LARGE_W_DP,
            Self::FullScreen => FULLSCREEN_W_DP,
        }
    }

    pub const fn small_w(self) -> f32 {
        match self {
            Self::Hero => SMALL_W_DP,
            Self::MultiBrowse => MULTI_SMALL_W_DP,
            Self::Uncontained => UNCONTAINED_SMALL_W_DP,
            Self::CenteredHero => CENTERED_SMALL_W_DP,
            Self::FullScreen => FULLSCREEN_W_DP,
        }
    }

    pub const fn axis(self) -> CarouselAxis {
        match self {
            Self::FullScreen => CarouselAxis::Vertical,
            _ => CarouselAxis::Horizontal,
        }
    }

    pub const fn center_aligned(self) -> bool {
        matches!(self, Self::CenteredHero)
    }

    pub const fn uses_phone_frame(self) -> bool {
        matches!(self, Self::CenteredHero | Self::FullScreen)
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Hero => Self::MultiBrowse,
            Self::MultiBrowse => Self::Uncontained,
            Self::Uncontained => Self::CenteredHero,
            Self::CenteredHero => Self::FullScreen,
            Self::FullScreen => Self::Hero,
        }
    }
}

pub fn item_height_for(layout: CarouselLayout) -> f32 {
    match layout {
        CarouselLayout::FullScreen => FULLSCREEN_H_DP,
        _ => HEIGHT_DP,
    }
}

pub fn phone_frame_h(layout: CarouselLayout) -> f32 {
    match layout {
        CarouselLayout::FullScreen => PHONE_FULLSCREEN_H_DP,
        _ => PHONE_H_DP,
    }
}
/// Wheel/fling distance that maps to one item. Larger deltas skip further.
pub const FLING_UNIT: f32 = 24.0;
/// Exponential decay per second for a one-shot velocity model (`v * e^{-k t}`).
/// `k = 2` so a ~96dp flick rests ~2 items away (beyond ±1).
pub const FLING_DECAY: f32 = 2.0;
/// When velocity dies, leftover ≥ this fraction of `FLING_UNIT` snaps one more item.
pub const FLING_SNAP_FRACTION: f32 = 0.5;

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
    item_width_for(CarouselLayout::Hero, index, selected)
}

pub fn item_width_for(layout: CarouselLayout, index: usize, selected: usize) -> f32 {
    if index == selected {
        layout.large_w()
    } else {
        layout.small_w()
    }
}

/// Hero/neighbor widths while a fling leftover is mid-item (`offset_t` = leftover / unit).
pub fn item_width_during_fling(index: usize, selected: usize, offset_t: f32) -> f32 {
    item_width_during_fling_for(CarouselLayout::Hero, index, selected, offset_t)
}

pub fn item_width_during_fling_for(
    layout: CarouselLayout,
    index: usize,
    selected: usize,
    offset_t: f32,
) -> f32 {
    let t = offset_t.clamp(-1.0, 1.0);
    if t.abs() < 0.001 {
        return item_width_for(layout, index, selected);
    }
    let next = if t >= 0.0 {
        advance(selected, 1)
    } else {
        advance(selected, -1)
    };
    let at = t.abs();
    let large = layout.large_w();
    let small = layout.small_w();
    if index == selected {
        large + (small - large) * at
    } else if index == next {
        small + (large - small) * at
    } else {
        small
    }
}

/// Role-color photo stub (official uses landscape tiles).
pub fn media_fill(theme: &Theme, index: usize) -> Argb {
    let c = theme.color;
    match index % 4 {
        0 => c.primary_container,
        1 => c.secondary_container,
        2 => c.tertiary_container,
        _ => c.error_container,
    }
}

pub fn media_on(theme: &Theme, index: usize) -> Argb {
    let c = theme.color;
    match index % 4 {
        0 => c.on_primary_container,
        1 => c.on_secondary_container,
        2 => c.on_tertiary_container,
        _ => c.on_error_container,
    }
}

/// Horizontal content shift while a fling leftover is mid-item.
pub fn parallax_offset_dp(offset_t: f32) -> f32 {
    offset_t.clamp(-1.0, 1.0) * PARALLAX_MAX_DP
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

/// Integrate `v₀ e^{-kt}` to rest (`distance = v₀ / k`) and round to items.
pub fn inertial_steps(dx: f32, dy: f32) -> i32 {
    let dominant = if dx.abs() >= dy.abs() { dx } else { dy };
    if dominant.abs() < 0.5 {
        return 0;
    }
    let v0 = dominant / FLING_UNIT;
    let distance = v0 / FLING_DECAY;
    let mag = distance.abs().round() as i32;
    let mag = mag.clamp(1, ITEMS.len() as i32 - 1);
    if distance >= 0.0 { mag } else { -mag }
}

/// Advance leftover fling velocity by `dt_s`. Returns `(velocity, residual, index_delta)`.
pub fn integrate_fling(velocity: f32, residual: f32, dt_s: f32) -> (f32, f32, i32) {
    let dt = dt_s.max(0.0);
    let next_v = decay_velocity(velocity, dt);
    let displacement = (velocity + next_v) * 0.5 * dt;
    let acc = residual + displacement;
    let steps = acc.trunc() as i32;
    (next_v, acc - steps as f32, steps)
}

/// Decay leftover fling velocity (`dt_s` in seconds).
pub fn decay_velocity(v: f32, dt_s: f32) -> f32 {
    v * (-FLING_DECAY * dt_s.max(0.0)).exp()
}

/// Per-frame inertial fling (velocity → item steps). Live hosts step this
/// from a vsync / rAF clock; one-shot wheel still uses `apply_wheel`.
pub const FLING_FRAME_DT: f32 = crate::motion::FRAME_DT;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlingState {
    pub selected: usize,
    pub velocity: f32,
    pub leftover: f32,
}

impl FlingState {
    pub fn new(selected: usize) -> Self {
        Self {
            selected: clamp_index(selected),
            velocity: 0.0,
            leftover: 0.0,
        }
    }

    pub fn impulse(&mut self, dx: f32, dy: f32) {
        let dominant = if dx.abs() >= dy.abs() { dx } else { dy };
        self.velocity += dominant;
    }

    /// Integrate one frame. Returns the new selected index.
    pub fn step(&mut self, dt_s: f32) -> usize {
        let dt = dt_s.max(0.0);
        self.leftover += self.velocity * dt;
        self.velocity = decay_velocity(self.velocity, dt);
        if self.velocity.abs() < 0.5 {
            self.velocity = 0.0;
        }
        while self.leftover.abs() >= FLING_UNIT {
            let dir = if self.leftover > 0.0 { 1 } else { -1 };
            self.selected = advance(self.selected, dir);
            self.leftover -= dir as f32 * FLING_UNIT;
        }
        if self.velocity.abs() < 0.5 {
            self.settle();
        }
        self.selected
    }

    /// Snap leftover to the nearest item once the flick has died.
    pub fn settle(&mut self) {
        if self.leftover.abs() >= FLING_UNIT * FLING_SNAP_FRACTION {
            let dir = if self.leftover > 0.0 { 1 } else { -1 };
            self.selected = advance(self.selected, dir);
        }
        self.leftover = 0.0;
        self.velocity = 0.0;
    }

    /// Signed progress toward the next item (`leftover / FLING_UNIT`).
    pub fn snap_offset_t(&self) -> f32 {
        self.leftover / FLING_UNIT
    }

    pub fn resting(&self) -> bool {
        self.velocity.abs() < 0.5 && self.leftover.abs() < FLING_UNIT * FLING_SNAP_FRACTION
    }

    /// True while a live host should keep requesting frames.
    pub fn needs_frame(&self) -> bool {
        !self.resting()
    }

    /// Seed velocity so integrating to rest advances about `steps` items.
    pub fn impulse_items(&mut self, steps: i32) {
        if steps == 0 {
            return;
        }
        self.velocity += steps as f32 * FLING_UNIT * FLING_DECAY;
    }

    /// Integrate until rest or `max_frames` at `dt_s`.
    pub fn step_until_rest(&mut self, dt_s: f32, max_frames: usize) -> usize {
        for _ in 0..max_frames {
            if self.resting() {
                break;
            }
            self.step(dt_s);
        }
        self.selected
    }

    /// Live-clock step using a wall `dt_s` (clamped). Returns the selected index.
    pub fn step_live(&mut self, dt_s: f32) -> usize {
        self.step(dt_s.clamp(0.0, 0.05).max(0.0))
    }
}

/// Wheel / trackpad: inertial item count, then `FlingState` steps at 60 Hz.
pub fn apply_wheel(selected: usize, dx: f32, dy: f32) -> usize {
    let steps = inertial_steps(dx, dy);
    if steps == 0 {
        return clamp_index(selected);
    }
    let mut fling = FlingState::new(selected);
    fling.impulse_items(steps);
    fling.step_until_rest(FLING_FRAME_DT, 180)
}
