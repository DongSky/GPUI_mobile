//! FAB (regular). Specs: https://m3.material.io/components/floating-action-button/specs

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{apply_state_layer, resolve_content, InteractionState};
use crate::theme::Theme;

pub const SIZE_DP: f32 = 56.0;
pub const ICON_DP: f32 = 24.0;
pub const CORNER_DP: f32 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FabVariant {
    Primary,
    Surface,
    Secondary,
    Tertiary,
}

impl FabVariant {
    pub const ALL: [Self; 4] = [
        Self::Primary,
        Self::Surface,
        Self::Secondary,
        Self::Tertiary,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Surface => "surface",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
        }
    }
}

pub fn resolve(theme: &Theme, variant: FabVariant, state: InteractionState) -> Appearance {
    let c = theme.color;
    let (base, icon) = match variant {
        FabVariant::Primary => (c.primary_container, c.on_primary_container),
        FabVariant::Surface => (c.surface_container_high, c.primary),
        FabVariant::Secondary => (c.secondary_container, c.on_secondary_container),
        FabVariant::Tertiary => (c.tertiary_container, c.on_tertiary_container),
    };
    let elevation = match state {
        InteractionState::Disabled => 0.0,
        InteractionState::Hovered | InteractionState::Dragged => theme.elevation.level4,
        InteractionState::Pressed => theme.elevation.level3,
        _ => theme.elevation.level3,
    };
    let (container, content) = if state.is_disabled() {
        (
            c.on_surface
                .with_alpha(crate::state::DISABLED_CONTAINER_OPACITY)
                .composite_over(c.surface),
            resolve_content(icon, c.on_surface, c.surface, true),
        )
    } else {
        (apply_state_layer(base, icon, state.layer_opacity()), icon)
    };
    Appearance {
        width_dp: Some(SIZE_DP),
        height_dp: SIZE_DP,
        min_width_dp: Some(SIZE_DP),
        corners: Corners::all(CORNER_DP),
        container,
        content,
        secondary_content: None,
        outline: None,
        elevation_dp: elevation,
        pad_start_dp: (SIZE_DP - ICON_DP) / 2.0,
        pad_end_dp: (SIZE_DP - ICON_DP) / 2.0,
        pad_top_dp: (SIZE_DP - ICON_DP) / 2.0,
        pad_bottom_dp: (SIZE_DP - ICON_DP) / 2.0,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}
