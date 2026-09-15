//! Common buttons — M3 Expressive (May 2025).
//! Specs: https://m3.material.io/components/buttons/specs
//!
//! Sizes XS–XL, round/square, press shape-morph. Default size is Small (40dp).
//! Small horizontal padding is 16dp (Expressive recommendation).

use crate::argb::Argb;
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{
    DISABLED_CONTAINER_OPACITY, InteractionState, apply_state_layer, resolve_content,
};
use crate::theme::Theme;
use crate::typography::TypeStyle;

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

    /// Labels as shown on the current m3.material.io buttons overview.
    pub const fn overview_label(self) -> &'static str {
        match self {
            Self::Elevated => "Elevated",
            Self::Filled => "Filled",
            Self::Tonal => "Filled tonal",
            Self::Outlined => "Outlined",
            Self::Text => "Text",
        }
    }
}

/// Official overview order: Elevated, Filled, Filled tonal, Outlined, Text.
pub const OVERVIEW_ORDER: [ButtonVariant; 5] = [
    ButtonVariant::Elevated,
    ButtonVariant::Filled,
    ButtonVariant::Tonal,
    ButtonVariant::Outlined,
    ButtonVariant::Text,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl ButtonSize {
    pub const ALL: [Self; 5] = [
        Self::ExtraSmall,
        Self::Small,
        Self::Medium,
        Self::Large,
        Self::ExtraLarge,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ExtraSmall => "xs",
            Self::Small => "s",
            Self::Medium => "m",
            Self::Large => "l",
            Self::ExtraLarge => "xl",
        }
    }

    pub const fn height_dp(self) -> f32 {
        match self {
            Self::ExtraSmall => 32.0,
            Self::Small => 40.0,
            Self::Medium => 56.0,
            Self::Large => 96.0,
            Self::ExtraLarge => 136.0,
        }
    }

    pub const fn pad_h_dp(self) -> f32 {
        match self {
            Self::ExtraSmall => 12.0,
            Self::Small => 16.0,
            Self::Medium => 24.0,
            Self::Large => 48.0,
            Self::ExtraLarge => 64.0,
        }
    }

    pub const fn icon_dp(self) -> f32 {
        match self {
            Self::ExtraSmall | Self::Small => 20.0,
            Self::Medium => 24.0,
            Self::Large => 32.0,
            Self::ExtraLarge => 40.0,
        }
    }

    pub const fn outline_dp(self) -> f32 {
        match self {
            Self::ExtraSmall | Self::Small | Self::Medium => 1.0,
            Self::Large => 2.0,
            Self::ExtraLarge => 3.0,
        }
    }

    pub const fn square_rest_dp(self) -> f32 {
        match self {
            Self::ExtraSmall | Self::Small => 12.0,
            Self::Medium => 16.0,
            Self::Large | Self::ExtraLarge => 28.0,
        }
    }

    pub const fn pressed_corner_dp(self) -> f32 {
        match self {
            Self::ExtraSmall | Self::Small => 8.0,
            Self::Medium => 12.0,
            Self::Large | Self::ExtraLarge => 16.0,
        }
    }

    pub fn label_style(self, theme: &Theme) -> TypeStyle {
        match self {
            Self::ExtraSmall | Self::Small => theme.typography.label_large,
            Self::Medium => theme.typography.title_medium,
            Self::Large => theme.typography.headline_small,
            Self::ExtraLarge => theme.typography.headline_medium,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonShape {
    Round,
    Square,
}

impl ButtonShape {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Round => "round",
            Self::Square => "square",
        }
    }

    /// Toggle selected resting shape (Compose `IconToggleButton` round↔square).
    pub const fn opposite(self) -> Self {
        match self {
            Self::Round => Self::Square,
            Self::Square => Self::Round,
        }
    }
}

pub const HEIGHT_DP: f32 = 40.0;
pub const MIN_WIDTH_DP: f32 = 64.0;
pub const PAD_DP: f32 = 16.0;
pub const ICON_SIZE_DP: f32 = 20.0;
pub const ICON_GAP_DP: f32 = 8.0;
pub const OUTLINE_WIDTH_DP: f32 = 1.0;

pub fn corner_dp(size: ButtonSize, shape: ButtonShape, state: InteractionState) -> f32 {
    if matches!(state, InteractionState::Pressed) {
        return size.pressed_corner_dp();
    }
    match shape {
        ButtonShape::Round => size.height_dp() / 2.0,
        ButtonShape::Square => size.square_rest_dp(),
    }
}

pub fn resolve(theme: &Theme, variant: ButtonVariant, state: InteractionState) -> Appearance {
    resolve_expressive(theme, variant, ButtonSize::Small, ButtonShape::Round, state)
}

pub fn resolve_expressive(
    theme: &Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
    state: InteractionState,
) -> Appearance {
    let c = theme.color;
    let surface = c.surface;
    let outline_w = size.outline_dp();
    let (base_container, label, outline, elevation) = match variant {
        ButtonVariant::Filled => (c.primary, c.on_primary, None, 0.0),
        ButtonVariant::Tonal => (c.secondary_container, c.on_secondary_container, None, 0.0),
        ButtonVariant::Elevated => (c.surface_container_low, c.primary, None, 1.0),
        ButtonVariant::Outlined => (
            surface,
            c.on_surface_variant,
            Some((c.outline_variant, outline_w)),
            0.0,
        ),
        // Text buttons have no filled container (official dialog Cancel/Accept).
        // Using `surface` painted as an opaque pill on desktop/Android.
        ButtonVariant::Text => (Argb::TRANSPARENT, c.primary, None, 0.0),
    };

    let elevation = match (variant, state) {
        (_, InteractionState::Disabled) => 0.0,
        (ButtonVariant::Elevated, InteractionState::Hovered | InteractionState::Dragged) => 3.0,
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
            ButtonVariant::Outlined => surface,
            ButtonVariant::Text => Argb::TRANSPARENT,
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

    let height = size.height_dp();
    let pad = size.pad_h_dp();
    Appearance {
        width_dp: None,
        height_dp: height,
        min_width_dp: Some(MIN_WIDTH_DP.max(height.min(64.0))),
        corners: Corners::all(corner_dp(size, shape, state)),
        container,
        content,
        secondary_content: None,
        outline,
        elevation_dp: elevation,
        pad_start_dp: pad,
        pad_end_dp: pad,
        pad_top_dp: 0.0,
        pad_bottom_dp: 0.0,
        label_style: size.label_style(theme),
        supporting_style: None,
    }
}
