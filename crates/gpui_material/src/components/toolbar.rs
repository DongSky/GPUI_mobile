//! Toolbars — M3 Expressive (May 2025).
//! Specs: https://m3.material.io/components/toolbars/specs
//! Tokens: androidx `FloatingToolbarTokens` 12_0_0.
//!
//! Floating: 64dp full-round container above body content.
//! Docked: spans the window width. Pair a FAB for the highest-priority action.

use crate::argb::Argb;
use crate::components::fab::{self, FabSize, FabVariant};
use crate::components::icon_button::{self, IconButtonVariant};
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;

pub const HEIGHT_DP: f32 = 64.0;
pub const PAD_H_DP: f32 = 8.0;
pub const ITEM_GAP_DP: f32 = 4.0;
pub const EXTERNAL_PAD_DP: f32 = 16.0;
pub const FAB_GAP_DP: f32 = 8.0;

pub const DEMO_ICONS: [&str; 4] = ["🔍", "🗑", "⬇", "↗"];
pub const DEMO_FAB: &str = "+";
pub const DEMO_EXPANDED: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolbarKind {
    Floating,
    Docked,
}

impl ToolbarKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Floating => "floating",
            Self::Docked => "docked",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolbarColor {
    Standard,
    Vibrant,
}

impl ToolbarColor {
    pub const ALL: [Self; 2] = [Self::Standard, Self::Vibrant];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Vibrant => "vibrant",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolbarAxis {
    Horizontal,
    Vertical,
}

impl ToolbarAxis {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolbarAppearance {
    pub height_dp: f32,
    pub pad_h_dp: f32,
    pub item_gap_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub icon: Argb,
    pub elevation_dp: f32,
    pub kind: ToolbarKind,
    pub color: ToolbarColor,
    pub axis: ToolbarAxis,
}

pub fn resolve(
    theme: &Theme,
    kind: ToolbarKind,
    color: ToolbarColor,
    axis: ToolbarAxis,
) -> ToolbarAppearance {
    let c = theme.color;
    let (container, icon) = match color {
        ToolbarColor::Standard => (c.surface_container, c.on_surface_variant),
        ToolbarColor::Vibrant => (c.primary_container, c.on_primary_container),
    };
    let corners = match kind {
        ToolbarKind::Floating => Corners::all(HEIGHT_DP / 2.0),
        ToolbarKind::Docked => Corners::all(0.0),
    };
    let elevation = match kind {
        ToolbarKind::Floating => theme.elevation.level3,
        ToolbarKind::Docked => 0.0,
    };
    ToolbarAppearance {
        height_dp: HEIGHT_DP,
        pad_h_dp: PAD_H_DP,
        item_gap_dp: ITEM_GAP_DP,
        corners,
        container,
        icon,
        elevation_dp: elevation,
        kind,
        color,
        axis,
    }
}

/// Icon affordance inside the toolbar (48dp target, 24dp glyph).
pub fn resolve_icon(theme: &Theme, color: ToolbarColor) -> Appearance {
    let mut appearance = icon_button::resolve(
        theme,
        IconButtonVariant::Standard,
        InteractionState::Enabled,
    );
    appearance.content = match color {
        ToolbarColor::Standard => theme.color.on_surface_variant,
        ToolbarColor::Vibrant => theme.color.on_primary_container,
    };
    appearance.container = match color {
        ToolbarColor::Standard => theme.color.surface_container,
        ToolbarColor::Vibrant => theme.color.primary_container,
    };
    appearance
}

/// Paired FAB for the highest-priority action (official floating + FAB layout).
pub fn resolve_fab(theme: &Theme, color: ToolbarColor) -> Appearance {
    let variant = match color {
        ToolbarColor::Standard => FabVariant::Primary,
        ToolbarColor::Vibrant => FabVariant::Primary,
    };
    fab::resolve_size(theme, variant, FabSize::Regular, InteractionState::Enabled)
}
