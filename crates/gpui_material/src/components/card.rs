//! Cards. Specs: https://m3.material.io/components/cards/specs

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;

pub const CORNER_DP: f32 = 12.0;
pub const PAD_DP: f32 = 16.0;
pub const GAP_DP: f32 = 8.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardVariant {
    Elevated,
    Filled,
    Outlined,
}

impl CardVariant {
    pub const ALL: [Self; 3] = [Self::Elevated, Self::Filled, Self::Outlined];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Elevated => "elevated",
            Self::Filled => "filled",
            Self::Outlined => "outlined",
        }
    }
}

pub fn resolve(theme: &Theme, variant: CardVariant, state: InteractionState) -> Appearance {
    let c = theme.color;
    let (base, outline, elevation) = match variant {
        CardVariant::Elevated => (c.surface_container_low, None, theme.elevation.level1),
        CardVariant::Filled => (c.surface_container_highest, None, 0.0),
        CardVariant::Outlined => (c.surface, Some((c.outline_variant, 1.0)), 0.0),
    };
    let elevation = match (variant, state) {
        (_, InteractionState::Disabled) => 0.0,
        (CardVariant::Elevated, InteractionState::Hovered) => theme.elevation.level2,
        (CardVariant::Elevated, InteractionState::Pressed) => theme.elevation.level1,
        _ => elevation,
    };
    let container = if state.is_disabled() {
        c.on_surface
            .with_alpha(crate::state::DISABLED_CONTAINER_OPACITY)
            .composite_over(c.surface)
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
    Appearance {
        width_dp: None,
        height_dp: 88.0,
        min_width_dp: None,
        corners: Corners::all(CORNER_DP),
        container,
        content,
        secondary_content: Some(c.on_surface_variant),
        outline,
        elevation_dp: elevation,
        pad_start_dp: PAD_DP,
        pad_end_dp: PAD_DP,
        pad_top_dp: PAD_DP,
        pad_bottom_dp: PAD_DP,
        label_style: theme.typography.title_medium,
        supporting_style: Some(theme.typography.body_medium),
    }
}
