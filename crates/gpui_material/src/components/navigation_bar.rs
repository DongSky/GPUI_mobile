//! Flexible / short navigation bar (Expressive).
//! Specs: https://m3.material.io/components/navigation-bar/specs
//! Tokens: androidx Compose `NavigationBarTokens` v0_11_0 +
//! `NavigationBarVerticalItemTokens` / `NavigationBarHorizontalItemTokens`.
//!
//! The baseline 80dp bar is **not recommended**. Use the 64dp flexible bar
//! with vertical items in compact windows and horizontal items in medium.

use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

/// Flexible / short container (`NavigationBarTokens.ContainerHeight`).
pub const HEIGHT_DP: f32 = 64.0;
/// Baseline tall bar — not recommended in Expressive.
pub const TALL_HEIGHT_DP: f32 = 80.0;

/// Vertical (compact) active indicator.
pub const VERTICAL_INDICATOR_W_DP: f32 = 56.0;
pub const VERTICAL_INDICATOR_H_DP: f32 = 32.0;
/// Space from indicator to label (`NavigationBarVerticalItemTokens.ContainerBetweenSpace`).
pub const VERTICAL_BETWEEN_DP: f32 = 6.0;

/// Horizontal (medium) active indicator.
pub const HORIZONTAL_INDICATOR_H_DP: f32 = 40.0;
pub const HORIZONTAL_INDICATOR_PAD_DP: f32 = 16.0;

pub const ICON_DP: f32 = 24.0;
/// Icon–label gap inside the indicator (`ItemActiveIndicatorIconLabelSpace`).
pub const ICON_LABEL_GAP_DP: f32 = 4.0;

/// Aliases used by compact / Gmail-peek hosts (vertical tokens).
pub const INDICATOR_W_DP: f32 = VERTICAL_INDICATOR_W_DP;
pub const INDICATOR_H_DP: f32 = VERTICAL_INDICATOR_H_DP;

pub const COMPACT_DESTS: [&str; 3] = ["Home", "Search", "Profile"];
pub const MEDIUM_DESTS: [&str; 4] = ["Home", "Search", "Saved", "Profile"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavBarLayout {
    /// Compact window: icon above label, equal-weight destinations.
    Vertical,
    /// Medium window: icon + label inside a 40dp pill, centered group.
    Horizontal,
}

impl NavBarLayout {
    pub const ALL: [Self; 2] = [Self::Vertical, Self::Horizontal];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }

    pub const fn compact(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavBarAppearance {
    pub layout: NavBarLayout,
    pub height_dp: f32,
    pub indicator_w_dp: f32,
    pub indicator_h_dp: f32,
    pub indicator_pad_h_dp: f32,
    pub icon_label_gap_dp: f32,
    pub container: Argb,
    pub active_indicator: Argb,
    pub active_icon: Argb,
    pub active_label: Argb,
    pub inactive_icon: Argb,
    pub inactive_label: Argb,
    pub elevation_dp: f32,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> NavBarAppearance {
    resolve_layout(theme, NavBarLayout::Vertical)
}

pub fn resolve_horizontal(theme: &Theme) -> NavBarAppearance {
    resolve_layout(theme, NavBarLayout::Horizontal)
}

pub fn resolve_layout(theme: &Theme, layout: NavBarLayout) -> NavBarAppearance {
    let c = theme.color;
    let (indicator_w_dp, indicator_h_dp, indicator_pad_h_dp, active_label) = match layout {
        NavBarLayout::Vertical => (
            VERTICAL_INDICATOR_W_DP,
            VERTICAL_INDICATOR_H_DP,
            0.0,
            c.secondary,
        ),
        NavBarLayout::Horizontal => (
            0.0,
            HORIZONTAL_INDICATOR_H_DP,
            HORIZONTAL_INDICATOR_PAD_DP,
            c.on_secondary_container,
        ),
    };
    NavBarAppearance {
        layout,
        height_dp: HEIGHT_DP,
        indicator_w_dp,
        indicator_h_dp,
        indicator_pad_h_dp,
        icon_label_gap_dp: match layout {
            NavBarLayout::Vertical => VERTICAL_BETWEEN_DP,
            NavBarLayout::Horizontal => ICON_LABEL_GAP_DP,
        },
        container: c.surface_container,
        active_indicator: c.secondary_container,
        active_icon: c.on_secondary_container,
        active_label,
        inactive_icon: c.on_surface_variant,
        inactive_label: c.on_surface_variant,
        elevation_dp: theme.elevation.level2,
        label_style: theme.typography.label_medium,
    }
}

pub fn destinations(layout: NavBarLayout) -> &'static [&'static str] {
    match layout {
        NavBarLayout::Vertical => &COMPACT_DESTS,
        NavBarLayout::Horizontal => &MEDIUM_DESTS,
    }
}

pub fn is_flexible_height(height_dp: f32) -> bool {
    (height_dp - HEIGHT_DP).abs() < 0.01
}
