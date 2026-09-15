//! FAB menu — M3 Expressive (May 2025).
//! Specs: https://m3.material.io/components/fab-menu/specs
//! Tokens: androidx `FabMenuBaselineTokens` v0_14_0.
//!
//! Opens from any FAB size to show 2–6 related actions. One menu size:
//! 56dp full-round item pills + a 56dp close FAB. Replaces M2 speed-dial
//! stacked small FABs. Color sets: primary / secondary / tertiary.

use crate::argb::Argb;
use crate::components::fab::{self, FabSize, FabVariant};
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;
use crate::typography::TypeStyle;

/// Gap between the close FAB and the first item (`CloseButtonBetweenSpace`).
pub const CLOSE_GAP_DP: f32 = 8.0;
/// Gap between menu items (`ListItemBetweenSpace`).
pub const ITEM_GAP_DP: f32 = 4.0;
pub const ITEM_H_DP: f32 = 56.0;
pub const ITEM_MIN_W_DP: f32 = 56.0;
pub const ITEM_PAD_H_DP: f32 = 24.0;
pub const ITEM_ICON_DP: f32 = 24.0;
pub const ITEM_ICON_GAP_DP: f32 = 8.0;
pub const CLOSE_DP: f32 = 56.0;
pub const CLOSE_ICON_DP: f32 = 20.0;
pub const MARGIN_DP: f32 = 16.0;

pub const OPEN_GLYPH: &str = "+";
pub const CLOSE_GLYPH: &str = "✕";

/// Official overview sample (create: Document / Message / Folder). Spec allows 2–6 items.
pub const DEMO_ITEMS: [(&str, &str); 3] = [
    ("📄", "Document"),
    ("💬", "Message"),
    ("📁", "Folder"),
];
pub const DEMO_EXPANDED: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FabMenuColor {
    Primary,
    Secondary,
    Tertiary,
}

impl FabMenuColor {
    pub const ALL: [Self; 3] = [Self::Primary, Self::Secondary, Self::Tertiary];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
        }
    }

    pub const fn fab_variant(self) -> FabVariant {
        match self {
            Self::Primary => FabVariant::Primary,
            Self::Secondary => FabVariant::Secondary,
            Self::Tertiary => FabVariant::Tertiary,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FabMenuItemAppearance {
    pub height_dp: f32,
    pub min_width_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub content: Argb,
    pub elevation_dp: f32,
    pub pad_h_dp: f32,
    pub icon_dp: f32,
    pub icon_gap_dp: f32,
    pub label_style: TypeStyle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FabMenuCloseAppearance {
    pub size_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub content: Argb,
    pub elevation_dp: f32,
    pub icon_dp: f32,
    pub glyph: &'static str,
}

fn set_colors(theme: &Theme, color: FabMenuColor) -> (Argb, Argb, Argb, Argb) {
    let c = theme.color;
    match color {
        FabMenuColor::Primary => (
            c.primary_container,
            c.on_primary_container,
            c.primary,
            c.on_primary,
        ),
        FabMenuColor::Secondary => (
            c.secondary_container,
            c.on_secondary_container,
            c.secondary,
            c.on_secondary,
        ),
        FabMenuColor::Tertiary => (
            c.tertiary_container,
            c.on_tertiary_container,
            c.tertiary,
            c.on_tertiary,
        ),
    }
}

/// Closed trigger is the matching FAB (any size). Menu items stay 56dp.
pub fn resolve_trigger(
    theme: &Theme,
    color: FabMenuColor,
    size: FabSize,
    state: InteractionState,
) -> Appearance {
    fab::resolve_size(theme, color.fab_variant(), size, state)
}

/// Expanded close FAB: 56×56 full-round, solid set color, contrasting items.
pub fn resolve_close(theme: &Theme, color: FabMenuColor, expanded: bool) -> FabMenuCloseAppearance {
    let (container_set, on_container, solid, on_solid) = set_colors(theme, color);
    if expanded {
        FabMenuCloseAppearance {
            size_dp: CLOSE_DP,
            corners: Corners::all(CLOSE_DP / 2.0),
            container: solid,
            content: on_solid,
            elevation_dp: theme.elevation.level3,
            icon_dp: CLOSE_ICON_DP,
            glyph: CLOSE_GLYPH,
        }
    } else {
        let fab = resolve_trigger(theme, color, FabSize::Regular, InteractionState::Enabled);
        FabMenuCloseAppearance {
            size_dp: fab.height_dp,
            corners: fab.corners,
            container: container_set,
            content: on_container,
            elevation_dp: fab.elevation_dp,
            icon_dp: fab::ICON_DP,
            glyph: OPEN_GLYPH,
        }
    }
}

pub fn resolve_item(theme: &Theme, color: FabMenuColor) -> FabMenuItemAppearance {
    let (container, content, _, _) = set_colors(theme, color);
    FabMenuItemAppearance {
        height_dp: ITEM_H_DP,
        min_width_dp: ITEM_MIN_W_DP,
        corners: Corners::all(ITEM_H_DP / 2.0),
        container,
        content,
        elevation_dp: theme.elevation.level3,
        pad_h_dp: ITEM_PAD_H_DP,
        icon_dp: ITEM_ICON_DP,
        icon_gap_dp: ITEM_ICON_GAP_DP,
        label_style: theme.typography.title_medium,
    }
}
