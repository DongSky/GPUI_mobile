//! Filled and outlined text fields.
//! Specs: https://m3.material.io/components/text-fields/specs
//!
//! Android IME / caret editing is out of scope for the platform layer; this
//! module resolves visual tokens and states (enabled/disabled/hover/focus/error).

use crate::argb::Argb;
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 56.0;
pub const PAD_H_DP: f32 = 16.0;
pub const INDICATOR_DP: f32 = 1.0;
pub const INDICATOR_FOCUSED_DP: f32 = 2.0;
pub const OUTLINE_DP: f32 = 1.0;
pub const OUTLINE_FOCUSED_DP: f32 = 2.0;
pub const SUPPORTING_GAP_DP: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldVariant {
    Filled,
    Outlined,
}

impl TextFieldVariant {
    pub const ALL: [Self; 2] = [Self::Filled, Self::Outlined];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Outlined => "outlined",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextFieldAppearance {
    pub field: Appearance,
    pub label: Argb,
    pub input: Argb,
    pub supporting: Argb,
    pub label_style: TypeStyle,
    pub input_style: TypeStyle,
    pub supporting_style: TypeStyle,
    pub populated: bool,
}

pub fn resolve(
    theme: &Theme,
    variant: TextFieldVariant,
    state: InteractionState,
    populated: bool,
) -> TextFieldAppearance {
    let c = theme.color;
    let focused = matches!(
        state,
        InteractionState::Focused | InteractionState::ErrorFocused
    );
    let error = state.is_error();
    let disabled = state.is_disabled();

    let (label, input, supporting, indicator) = if disabled {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        (muted, muted, muted, muted)
    } else if error && focused {
        (c.error, c.on_surface, c.error, c.error)
    } else if error && state == InteractionState::Hovered {
        (c.on_error_container, c.on_surface, c.error, c.on_error_container)
    } else if error {
        (c.error, c.on_surface, c.error, c.error)
    } else if focused {
        (c.primary, c.on_surface, c.on_surface_variant, c.primary)
    } else if state == InteractionState::Hovered {
        (c.on_surface_variant, c.on_surface, c.on_surface_variant, c.on_surface)
    } else {
        (
            c.on_surface_variant,
            c.on_surface,
            c.on_surface_variant,
            c.on_surface_variant,
        )
    };

    let (container, corners, outline) = match variant {
        TextFieldVariant::Filled => {
            let base = if disabled {
                c.on_surface
                    .with_alpha(0.04)
                    .composite_over(c.surface)
            } else {
                c.surface_container_highest
            };
            let width = if focused {
                INDICATOR_FOCUSED_DP
            } else {
                INDICATOR_DP
            };
            (
                base,
                Corners::extra_small_top(theme.shapes.extra_small),
                Some((indicator, width)),
            )
        }
        TextFieldVariant::Outlined => {
            let width = if focused {
                OUTLINE_FOCUSED_DP
            } else {
                OUTLINE_DP
            };
            (
                c.surface,
                Corners::all(theme.shapes.extra_small),
                Some((indicator, width)),
            )
        }
    };

    let label_style = if populated || focused {
        theme.typography.body_small
    } else {
        theme.typography.body_large
    };

    TextFieldAppearance {
        field: Appearance {
            width_dp: None,
            height_dp: HEIGHT_DP,
            min_width_dp: Some(210.0),
            corners,
            container,
            content: input,
            secondary_content: Some(label),
            outline,
            elevation_dp: 0.0,
            pad_start_dp: PAD_H_DP,
            pad_end_dp: PAD_H_DP,
            pad_top_dp: 8.0,
            pad_bottom_dp: 8.0,
            label_style,
            supporting_style: Some(theme.typography.body_small),
        },
        label,
        input,
        supporting,
        label_style,
        input_style: theme.typography.body_large,
        supporting_style: theme.typography.body_small,
        populated,
    }
}
