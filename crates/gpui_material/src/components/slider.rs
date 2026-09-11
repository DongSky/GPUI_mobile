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
            c.primary,
            c.secondary_container,
            c.primary,
            c.on_primary,
            c.on_secondary_container,
        )
    };
    SliderAppearance {
        track_h: size.track_h(),
        track_corner: size.track_corner(),
        handle_w,
        handle_h: size.handle_h(),
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
    }
}
