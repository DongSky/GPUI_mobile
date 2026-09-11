//! Radio button. Specs: https://m3.material.io/components/radio-button/specs

use crate::argb::Argb;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;

pub const OUTER_DP: f32 = 20.0;
pub const INNER_DP: f32 = 10.0;
pub const TARGET_DP: f32 = 48.0;
pub const STATE_LAYER_DP: f32 = 40.0;
pub const STROKE_DP: f32 = 2.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadioAppearance {
    pub outer_dp: f32,
    pub inner_dp: f32,
    pub stroke_dp: f32,
    pub target_dp: f32,
    pub ring: Argb,
    pub inner: Option<Argb>,
    pub state_layer: Argb,
    pub label: Argb,
}

pub fn resolve(theme: &Theme, selected: bool, state: InteractionState) -> RadioAppearance {
    let c = theme.color;
    let (ring, inner) = if state.is_disabled() {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        (muted, selected.then_some(muted))
    } else if selected {
        (c.primary, Some(c.primary))
    } else {
        (c.on_surface_variant, None)
    };
    let layer_color = if selected { c.primary } else { c.on_surface };
    RadioAppearance {
        outer_dp: OUTER_DP,
        inner_dp: INNER_DP,
        stroke_dp: STROKE_DP,
        target_dp: TARGET_DP,
        ring,
        inner,
        state_layer: apply_state_layer(c.surface, layer_color, state.layer_opacity()),
        label: if state.is_disabled() {
            c.on_surface
                .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
                .composite_over(c.surface)
        } else {
            c.on_surface
        },
    }
}
