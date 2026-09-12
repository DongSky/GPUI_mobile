//! Interaction states and Material 3 state-layer opacities.
//! https://m3.material.io/foundations/interaction/states/state-layers

use crate::argb::Argb;

/// Official M3 state-layer opacities (published applying-states guidance).
pub const HOVER_OPACITY: f32 = 0.08;
pub const FOCUS_OPACITY: f32 = 0.10;
pub const PRESSED_OPACITY: f32 = 0.10;
pub const DRAGGED_OPACITY: f32 = 0.16;
pub const DISABLED_CONTENT_OPACITY: f32 = 0.38;
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractionState {
    Enabled,
    Disabled,
    Hovered,
    Focused,
    Pressed,
    Dragged,
    /// Component-level error (text fields). Mutually exclusive with disabled.
    Error,
    ErrorFocused,
}

impl InteractionState {
    pub const ALL_COMMON: [Self; 6] = [
        Self::Enabled,
        Self::Disabled,
        Self::Hovered,
        Self::Focused,
        Self::Pressed,
        Self::Dragged,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Hovered => "hovered",
            Self::Focused => "focused",
            Self::Pressed => "pressed",
            Self::Dragged => "dragged",
            Self::Error => "error",
            Self::ErrorFocused => "error + focused",
        }
    }

    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::ErrorFocused)
    }

    pub const fn layer_opacity(self) -> f32 {
        match self {
            Self::Enabled | Self::Disabled | Self::Error => 0.0,
            Self::Hovered => HOVER_OPACITY,
            Self::Focused | Self::ErrorFocused => FOCUS_OPACITY,
            Self::Pressed => PRESSED_OPACITY,
            Self::Dragged => DRAGGED_OPACITY,
        }
    }
}

/// Paint a container: optional disabled recipe, then a state layer of `content`.
pub fn resolve_container(
    base: Argb,
    content: Argb,
    state: InteractionState,
    surface: Argb,
    disabled_uses_on_surface: bool,
    on_surface: Argb,
) -> Argb {
    if state.is_disabled() {
        if disabled_uses_on_surface {
            return on_surface
                .with_alpha(DISABLED_CONTAINER_OPACITY)
                .composite_over(surface);
        }
        return base;
    }
    apply_state_layer(base, content, state.layer_opacity())
}

pub fn apply_state_layer(container: Argb, layer_color: Argb, opacity: f32) -> Argb {
    if opacity <= 0.0 {
        return container;
    }
    layer_color.with_alpha(opacity).composite_over(container)
}

pub fn resolve_content(enabled: Argb, on_surface: Argb, surface: Argb, disabled: bool) -> Argb {
    if disabled {
        on_surface
            .with_alpha(DISABLED_CONTENT_OPACITY)
            .composite_over(surface)
    } else {
        enabled
    }
}
