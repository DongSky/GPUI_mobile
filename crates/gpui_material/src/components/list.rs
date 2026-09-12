//! List items. Specs: https://m3.material.io/components/lists/specs

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListLines {
    One,
    Two,
    Three,
}

impl ListLines {
    pub const ALL: [Self; 3] = [Self::One, Self::Two, Self::Three];

    pub const fn height_dp(self) -> f32 {
        match self {
            Self::One => 56.0,
            Self::Two => 72.0,
            Self::Three => 88.0,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::One => "one-line",
            Self::Two => "two-line",
            Self::Three => "three-line",
        }
    }
}

pub const PAD_H_DP: f32 = 16.0;
pub const LEADING_DP: f32 = 24.0;
pub const LEADING_GAP_DP: f32 = 16.0;

pub fn resolve(theme: &Theme, lines: ListLines, state: InteractionState) -> Appearance {
    let c = theme.color;
    let base = if state == InteractionState::Enabled {
        c.surface
    } else {
        c.surface
    };
    let container = if state.is_disabled() {
        c.surface
    } else {
        apply_state_layer(base, c.on_surface, state.layer_opacity())
    };
    let content = if state.is_disabled() {
        c.on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface)
    } else {
        c.on_surface
    };
    let supporting = if state.is_disabled() {
        content
    } else {
        c.on_surface_variant
    };
    Appearance {
        width_dp: None,
        height_dp: lines.height_dp(),
        min_width_dp: None,
        corners: Corners::all(0.0),
        container,
        content,
        secondary_content: Some(supporting),
        outline: None,
        elevation_dp: 0.0,
        pad_start_dp: PAD_H_DP,
        pad_end_dp: PAD_H_DP,
        pad_top_dp: 8.0,
        pad_bottom_dp: 8.0,
        label_style: theme.typography.body_large,
        supporting_style: Some(theme.typography.body_medium),
    }
}
