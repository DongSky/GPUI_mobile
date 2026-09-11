//! Common buttons — filled / tonal / elevated / outlined / text.
//! Specs: https://m3.material.io/components/buttons/specs

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{
    apply_state_layer, resolve_content, InteractionState, DISABLED_CONTAINER_OPACITY,
};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Filled,
    Tonal,
    Elevated,
    Outlined,
    Text,
}

impl ButtonVariant {
    pub const ALL: [Self; 5] = [
        Self::Filled,
        Self::Tonal,
        Self::Elevated,
        Self::Outlined,
        Self::Text,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Tonal => "tonal",
            Self::Elevated => "elevated",
            Self::Outlined => "outlined",
            Self::Text => "text",
        }
    }
}

pub const HEIGHT_DP: f32 = 40.0;
pub const MIN_WIDTH_DP: f32 = 64.0;
pub const PAD_DP: f32 = 24.0;
pub const PAD_WITH_ICON_START_DP: f32 = 16.0;
pub const ICON_SIZE_DP: f32 = 18.0;
pub const ICON_GAP_DP: f32 = 8.0;
pub const OUTLINE_WIDTH_DP: f32 = 1.0;

pub fn resolve(theme: &Theme, variant: ButtonVariant, state: InteractionState) -> Appearance {
    let c = theme.color;
    let surface = c.surface;
    let (base_container, label, outline, elevation) = match variant {
        ButtonVariant::Filled => (c.primary, c.on_primary, None, 0.0),
        ButtonVariant::Tonal => (c.secondary_container, c.on_secondary_container, None, 0.0),
        ButtonVariant::Elevated => (c.surface_container_low, c.primary, None, 1.0),
        ButtonVariant::Outlined => (surface, c.primary, Some((c.outline, OUTLINE_WIDTH_DP)), 0.0),
        ButtonVariant::Text => (surface, c.primary, None, 0.0),
    };

    let elevation = match (variant, state) {
        (_, InteractionState::Disabled) => 0.0,
        (ButtonVariant::Elevated, InteractionState::Hovered | InteractionState::Dragged) => 2.0,
        (ButtonVariant::Filled, InteractionState::Hovered) => 1.0,
        _ => elevation,
    };

    let (container, content, outline) = if state.is_disabled() {
        let content = resolve_content(label, c.on_surface, surface, true);
        let container = match variant {
            ButtonVariant::Filled | ButtonVariant::Tonal | ButtonVariant::Elevated => c
                .on_surface
                .with_alpha(DISABLED_CONTAINER_OPACITY)
                .composite_over(surface),
            ButtonVariant::Outlined | ButtonVariant::Text => surface,
        };
        let outline = outline.map(|(_, w)| {
            (
                c.on_surface
                    .with_alpha(DISABLED_CONTAINER_OPACITY)
                    .composite_over(surface),
                w,
            )
        });
        (container, content, outline)
    } else {
        let painted = apply_state_layer(base_container, label, state.layer_opacity());
        (painted, label, outline)
    };

    Appearance {
        width_dp: None,
        height_dp: HEIGHT_DP,
        min_width_dp: Some(MIN_WIDTH_DP),
        corners: Corners::all(HEIGHT_DP / 2.0),
        container,
        content,
        secondary_content: None,
        outline,
        elevation_dp: elevation,
        pad_start_dp: PAD_DP,
        pad_end_dp: PAD_DP,
        pad_top_dp: 0.0,
        pad_bottom_dp: 0.0,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}
