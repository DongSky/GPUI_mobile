//! FAB (baseline small / regular / large / extended).
//! Specs: https://m3.material.io/components/floating-action-button/specs
//!
//! Not M3 Expressive (no XS–XL morph). Small 40 / regular 56 / large 96 / extended 56+label.

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{apply_state_layer, resolve_content, InteractionState};
use crate::theme::Theme;

pub const SIZE_DP: f32 = 56.0;
pub const SMALL_DP: f32 = 40.0;
pub const LARGE_DP: f32 = 96.0;
pub const ICON_DP: f32 = 24.0;
pub const SMALL_ICON_DP: f32 = 24.0;
pub const LARGE_ICON_DP: f32 = 36.0;
pub const CORNER_DP: f32 = 16.0;
pub const SMALL_CORNER_DP: f32 = 12.0;
pub const LARGE_CORNER_DP: f32 = 28.0;
pub const EXTENDED_MIN_WIDTH_DP: f32 = 80.0;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FabSize {
    Small,
    Regular,
    Large,
    Extended,
}

impl FabSize {
    pub const ALL: [Self; 4] = [Self::Small, Self::Regular, Self::Large, Self::Extended];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Regular => "regular",
            Self::Large => "large",
            Self::Extended => "extended",
        }
    }
}

pub fn resolve(theme: &Theme, variant: FabVariant, state: InteractionState) -> Appearance {
    resolve_size(theme, variant, FabSize::Regular, state)
}

pub fn resolve_size(
    theme: &Theme,
    variant: FabVariant,
    size: FabSize,
    state: InteractionState,
) -> Appearance {
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
    let (width, height, min_width, corner, icon_dp, pad_h) = match size {
        FabSize::Small => (
            Some(SMALL_DP),
            SMALL_DP,
            Some(SMALL_DP),
            SMALL_CORNER_DP,
            SMALL_ICON_DP,
            (SMALL_DP - SMALL_ICON_DP) / 2.0,
        ),
        FabSize::Regular => (
            Some(SIZE_DP),
            SIZE_DP,
            Some(SIZE_DP),
            CORNER_DP,
            ICON_DP,
            (SIZE_DP - ICON_DP) / 2.0,
        ),
        FabSize::Large => (
            Some(LARGE_DP),
            LARGE_DP,
            Some(LARGE_DP),
            LARGE_CORNER_DP,
            LARGE_ICON_DP,
            (LARGE_DP - LARGE_ICON_DP) / 2.0,
        ),
        FabSize::Extended => (
            None,
            SIZE_DP,
            Some(EXTENDED_MIN_WIDTH_DP),
            CORNER_DP,
            ICON_DP,
            16.0,
        ),
    };
    Appearance {
        width_dp: width,
        height_dp: height,
        min_width_dp: min_width,
        corners: Corners::all(corner),
        container,
        content,
        secondary_content: None,
        outline: None,
        elevation_dp: elevation,
        pad_start_dp: pad_h,
        pad_end_dp: pad_h,
        pad_top_dp: (height - icon_dp) / 2.0,
        pad_bottom_dp: (height - icon_dp) / 2.0,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}
