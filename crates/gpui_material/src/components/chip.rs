//! Chips — M3 Expressive (Compose 1.5 / FilterChip + InputChip morph).
//! Specs: https://m3.material.io/components/chips/specs
//! Tokens: androidx `Shapes.defaultChipShapes` + `FilterChipTokens` /
//! `InputChipTokens` (`Chip.kt`).
//!
//! Baseline Assist / Suggestion stay 32dp full-round. Expressive FilterChip
//! and InputChip morph `ChipShapes`: CornerMedium rest, CornerFull selected,
//! CornerSmall pressed.

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{
    apply_state_layer, resolve_content, InteractionState, DISABLED_CONTAINER_OPACITY,
};
use crate::theme::Theme;

/// `FilterChipTokens.ContainerHeight` / `InputChipTokens.ContainerHeight`.
pub const HEIGHT_DP: f32 = 32.0;
/// Label-only leading/trailing (`FilterChipDefaults.ContentPadding`).
pub const PAD_H_DP: f32 = 16.0;
/// Start inset when a leading icon is shown.
pub const LEADING_PAD_START_DP: f32 = 8.0;
/// End inset when a trailing icon is shown (`InputChip` close).
pub const TRAILING_PAD_END_DP: f32 = 8.0;
/// `FilterChipDefaults.IconSize` / `InputChipDefaults.IconSize`.
pub const ICON_DP: f32 = 18.0;
/// `FilterChipDefaults.HorizontalSpacing`.
pub const ICON_GAP_DP: f32 = 8.0;
/// `FilterChipDefaults.CompactHorizontalSpacing`.
pub const COMPACT_ICON_GAP_DP: f32 = 4.0;
/// Compose `Shapes.defaultChipShapes.shape` = `ShapeTokens.CornerMedium`.
pub const UNSELECTED_CORNER_DP: f32 = 12.0;
/// `selectedShape` = `ShapeTokens.CornerFull` (half of 32dp).
pub const SELECTED_CORNER_DP: f32 = 16.0;
/// `pressedShape` = `ShapeTokens.CornerSmall`.
pub const PRESSED_CORNER_DP: f32 = 8.0;
/// Selected FilterChip leading check (official anatomy).
pub const CHECK_GLYPH: &str = "✓";
/// InputChip trailing dismiss.
pub const CLOSE_GLYPH: &str = "×";

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

    /// Compose Expressive overloads: FilterChip / ElevatedFilterChip / InputChip.
    pub const fn morphs(self) -> bool {
        matches!(self, Self::Filter | Self::Input)
    }
}

/// Official filter overview labels (m3 chips / amenities row).
pub const FILTER_HERO: [ChipDemo; 3] = [
    ChipDemo {
        variant: ChipVariant::Filter,
        label: "Elevator",
        selected: false,
        state: InteractionState::Enabled,
    },
    ChipDemo {
        variant: ChipVariant::Filter,
        label: "Washer",
        selected: true,
        state: InteractionState::Enabled,
    },
    ChipDemo {
        variant: ChipVariant::Filter,
        label: "Pets",
        selected: false,
        state: InteractionState::Pressed,
    },
];

/// Official input overview (place + selected place).
pub const INPUT_HERO: [ChipDemo; 2] = [
    ChipDemo {
        variant: ChipVariant::Input,
        label: "Portland",
        selected: false,
        state: InteractionState::Enabled,
    },
    ChipDemo {
        variant: ChipVariant::Input,
        label: "Portland",
        selected: true,
        state: InteractionState::Enabled,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChipDemo {
    pub variant: ChipVariant,
    pub label: &'static str,
    pub selected: bool,
    pub state: InteractionState,
}

/// Compose `ChipShapes` corner for this variant / selection / interaction.
pub fn corner_dp(
    theme: &Theme,
    variant: ChipVariant,
    selected: bool,
    state: InteractionState,
) -> f32 {
    if !variant.morphs() {
        return HEIGHT_DP / 2.0;
    }
    if matches!(state, InteractionState::Pressed) {
        theme.shapes.small
    } else if selected {
        HEIGHT_DP / 2.0
    } else {
        theme.shapes.medium
    }
}

/// Selected FilterChip shows a leading check.
pub fn leading_icon(variant: ChipVariant, selected: bool) -> Option<&'static str> {
    if matches!(variant, ChipVariant::Filter) && selected {
        Some(CHECK_GLYPH)
    } else {
        None
    }
}

/// InputChip shows a trailing close (selected or not).
pub fn trailing_icon(variant: ChipVariant) -> Option<&'static str> {
    if matches!(variant, ChipVariant::Input) {
        Some(CLOSE_GLYPH)
    } else {
        None
    }
}

pub fn pad_start_dp(variant: ChipVariant, selected: bool) -> f32 {
    if leading_icon(variant, selected).is_some() {
        LEADING_PAD_START_DP
    } else {
        PAD_H_DP
    }
}

pub fn pad_end_dp(variant: ChipVariant) -> f32 {
    if trailing_icon(variant).is_some() {
        TRAILING_PAD_END_DP
    } else {
        PAD_H_DP
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
    let r = corner_dp(theme, variant, selected, state);
    Appearance {
        width_dp: None,
        height_dp: HEIGHT_DP,
        min_width_dp: None,
        corners: Corners::all(r),
        container,
        content,
        secondary_content: Some(if selected {
            c.on_secondary_container
        } else {
            c.on_surface_variant
        }),
        outline,
        elevation_dp: 0.0,
        pad_start_dp: pad_start_dp(variant, selected),
        pad_end_dp: pad_end_dp(variant),
        pad_top_dp: 0.0,
        pad_bottom_dp: 0.0,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}
