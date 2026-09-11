//! Standard icon button. Specs: https://m3.material.io/components/icon-buttons/specs

use crate::components::button::{self, ButtonShape, ButtonSize};
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{
    apply_state_layer, resolve_content, InteractionState, DISABLED_CONTAINER_OPACITY,
};
use crate::theme::Theme;

pub const CONTAINER_DP: f32 = 40.0;
pub const ICON_DP: f32 = 24.0;
pub const TARGET_DP: f32 = 48.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconButtonVariant {
    Standard,
    Filled,
    Tonal,
    Outlined,
}

impl IconButtonVariant {
    pub const ALL: [Self; 4] = [Self::Standard, Self::Filled, Self::Tonal, Self::Outlined];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Filled => "filled",
            Self::Tonal => "tonal",
            Self::Outlined => "outlined",
        }
    }
}

pub fn resolve(theme: &Theme, variant: IconButtonVariant, state: InteractionState) -> Appearance {
    let c = theme.color;
    let (base, icon, outline) = match variant {
        IconButtonVariant::Standard => (c.surface, c.on_surface_variant, None),
        IconButtonVariant::Filled => (c.primary, c.on_primary, None),
        IconButtonVariant::Tonal => (c.secondary_container, c.on_secondary_container, None),
        IconButtonVariant::Outlined => (
            c.surface,
            c.on_surface_variant,
            Some((c.outline_variant, 1.0)),
        ),
    };
    let (container, content, outline) = if state.is_disabled() {
        let container = match variant {
            IconButtonVariant::Filled | IconButtonVariant::Tonal => c
                .on_surface
                .with_alpha(DISABLED_CONTAINER_OPACITY)
                .composite_over(c.surface),
            _ => c.surface,
        };
        (
            container,
            resolve_content(icon, c.on_surface, c.surface, true),
            outline.map(|(_, w)| {
                (
                    c.on_surface
                        .with_alpha(DISABLED_CONTAINER_OPACITY)
                        .composite_over(c.surface),
                    w,
                )
            }),
        )
    } else {
        (
            apply_state_layer(base, icon, state.layer_opacity()),
            icon,
            outline,
        )
    };
    Appearance {
        width_dp: Some(CONTAINER_DP),
        height_dp: CONTAINER_DP,
        min_width_dp: Some(TARGET_DP),
        corners: Corners::all(button::corner_dp(
            ButtonSize::Small,
            ButtonShape::Round,
            state,
        )),
        container,
        content,
        secondary_content: None,
        outline,
        elevation_dp: 0.0,
        pad_start_dp: (CONTAINER_DP - ICON_DP) / 2.0,
        pad_end_dp: (CONTAINER_DP - ICON_DP) / 2.0,
        pad_top_dp: (CONTAINER_DP - ICON_DP) / 2.0,
        pad_bottom_dp: (CONTAINER_DP - ICON_DP) / 2.0,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}
