//! Slider (baseline Material You: 4dp track, 20dp thumb).
//! Specs: https://m3.material.io/components/sliders/specs
//!
//! Uses the pre-Expressive baseline (not the 16dp / 4×44 handle from SliderTokens v2).

use crate::argb::Argb;
use crate::state::InteractionState;
use crate::theme::Theme;

pub const TRACK_HEIGHT_DP: f32 = 4.0;
pub const THUMB_DP: f32 = 20.0;
pub const TARGET_DP: f32 = 48.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderAppearance {
    pub track_h: f32,
    pub thumb_dp: f32,
    pub target_dp: f32,
    pub active: Argb,
    pub inactive: Argb,
    pub thumb: Argb,
    pub value: f32,
}

pub fn resolve(theme: &Theme, value: f32, state: InteractionState) -> SliderAppearance {
    let c = theme.color;
    let value = value.clamp(0.0, 1.0);
    let (active, inactive, thumb) = if state.is_disabled() {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        let track = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTAINER_OPACITY)
            .composite_over(c.surface);
        (muted, track, muted)
    } else {
        (c.primary, c.secondary_container, c.primary)
    };
    SliderAppearance {
        track_h: TRACK_HEIGHT_DP,
        thumb_dp: THUMB_DP,
        target_dp: TARGET_DP,
        active,
        inactive,
        thumb,
        value,
    }
}
