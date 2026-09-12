//! Switch. Specs: https://m3.material.io/components/switch/specs

use crate::argb::Argb;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;

pub const TRACK_WIDTH_DP: f32 = 52.0;
pub const TRACK_HEIGHT_DP: f32 = 32.0;
pub const THUMB_UNSELECTED_DP: f32 = 16.0;
pub const THUMB_SELECTED_DP: f32 = 24.0;
pub const TARGET_DP: f32 = 48.0;
pub const TRACK_OUTLINE_DP: f32 = 2.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchAppearance {
    pub track_w: f32,
    pub track_h: f32,
    pub thumb_dp: f32,
    pub track: Argb,
    pub track_outline: Option<Argb>,
    pub thumb: Argb,
    pub state_layer: Argb,
    pub icon: Argb,
}

pub fn resolve(theme: &Theme, selected: bool, state: InteractionState) -> SwitchAppearance {
    let c = theme.color;
    let (track, outline, thumb, icon, thumb_dp) = if state.is_disabled() {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        let track = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTAINER_OPACITY)
            .composite_over(c.surface);
        if selected {
            (track, None, muted, c.surface, THUMB_SELECTED_DP)
        } else {
            (track, Some(muted), muted, muted, THUMB_UNSELECTED_DP)
        }
    } else if selected {
        (c.primary, None, c.on_primary, c.on_primary_container, THUMB_SELECTED_DP)
    } else {
        (
            c.surface_container_highest,
            Some(c.outline),
            c.outline,
            c.surface_container_highest,
            THUMB_UNSELECTED_DP,
        )
    };
    let layer = if selected { c.primary } else { c.on_surface };
    SwitchAppearance {
        track_w: TRACK_WIDTH_DP,
        track_h: TRACK_HEIGHT_DP,
        thumb_dp,
        track: apply_state_layer(track, layer, 0.0),
        track_outline: outline,
        thumb,
        state_layer: apply_state_layer(c.surface, layer, state.layer_opacity()),
        icon,
    }
}
