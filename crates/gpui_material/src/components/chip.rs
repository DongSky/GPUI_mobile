//! Chips. Specs: https://m3.material.io/components/chips/specs

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{apply_state_layer, resolve_content, InteractionState, DISABLED_CONTAINER_OPACITY};
use crate::theme::Theme;

pub const HEIGHT_DP: f32 = 32.0;
pub const PAD_H_DP: f32 = 16.0;
pub const ICON_DP: f32 = 18.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChipVariant {
    Assist,
    Filter,
    Input,
    Suggestion,
}

impl ChipVariant {
    pub const ALL: [Self; 4] = [Self::Assist, Self::Filter, Self::Input, Self::Suggestion];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Assist => "assist",
            Self::Filter => "filter",
            Self::Input => "input",
            Self::Suggestion => "suggestion",
        }
    }
}

pub fn resolve(
    theme: &Theme,
    variant: ChipVariant,
    selected: bool,
    state: InteractionState,
) -> Appearance {
    let c = theme.color;
    let (base, label, outline) = match (variant, selected) {
        (ChipVariant::Filter | ChipVariant::Input, true) => {
            (c.secondary_container, c.on_secondary_container, None)
        }
        (ChipVariant::Suggestion, false) => (c.surface_container_low, c.on_surface_variant, None),
        (ChipVariant::Suggestion, true) => (c.secondary_container, c.on_secondary_container, None),
        _ => (c.surface, c.on_surface, Some((c.outline, 1.0))),
    };
    let (container, content, outline) = if state.is_disabled() {
        (
            if selected {
                c.on_surface
                    .with_alpha(DISABLED_CONTAINER_OPACITY)
                    .composite_over(c.surface)
            } else {
                c.surface
            },
            resolve_content(label, c.on_surface, c.surface, true),
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
            apply_state_layer(base, label, state.layer_opacity()),
            label,
            outline,
        )
    };
    Appearance {
        width_dp: None,
        height_dp: HEIGHT_DP,
        min_width_dp: None,
        corners: Corners::all(HEIGHT_DP / 2.0),
        container,
        content,
        secondary_content: None,
        outline,
        elevation_dp: 0.0,
        pad_start_dp: PAD_H_DP,
        pad_end_dp: PAD_H_DP,
        pad_top_dp: 0.0,
        pad_bottom_dp: 0.0,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}
