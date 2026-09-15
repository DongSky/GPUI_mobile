//! Navigation rail. Specs: https://m3.material.io/components/navigation-rail/specs
//!
//! Collapsed 80dp destinations with a 56×32 active indicator, expanded 220dp
//! modal with a FAB slot and destination badges.

use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const WIDTH_DP: f32 = 80.0;
pub const EXPANDED_WIDTH_DP: f32 = 220.0;
pub const INDICATOR_W_DP: f32 = 56.0;
pub const INDICATOR_H_DP: f32 = 32.0;
pub const ICON_DP: f32 = 24.0;
pub const DEST_GAP_DP: f32 = 12.0;
pub const PAD_TOP_DP: f32 = 16.0;
pub const FAB_SLOT_DP: f32 = 56.0;

pub const DESTINATIONS: [&str; 3] = ["Home", "Search", "Profile"];
pub const DESTINATION_ICONS: [&str; 3] = ["⌂", "⌕", "☺"];
/// `None` = no badge, `Some(0)` = small dot, `Some(n)` = large count.
pub const DESTINATION_BADGES: [Option<u32>; 3] = [None, Some(3), Some(0)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailMode {
    Collapsed,
    Expanded,
}

impl RailMode {
    pub const fn width_dp(self) -> f32 {
        match self {
            Self::Collapsed => WIDTH_DP,
            Self::Expanded => EXPANDED_WIDTH_DP,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Collapsed => "collapsed",
            Self::Expanded => "expanded",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavRailAppearance {
    pub width_dp: f32,
    pub expanded_width_dp: f32,
    pub container: Argb,
    pub active_indicator: Argb,
    pub active_icon: Argb,
    pub active_label: Argb,
    pub inactive_icon: Argb,
    pub inactive_label: Argb,
    pub fab: Argb,
    pub fab_icon: Argb,
    pub badge: Argb,
    pub badge_label: Argb,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> NavRailAppearance {
    let c = theme.color;
    NavRailAppearance {
        width_dp: WIDTH_DP,
        expanded_width_dp: EXPANDED_WIDTH_DP,
        container: c.surface,
        active_indicator: c.secondary_container,
        active_icon: c.on_secondary_container,
        active_label: c.on_surface,
        inactive_icon: c.on_surface_variant,
        inactive_label: c.on_surface_variant,
        fab: c.primary_container,
        fab_icon: c.on_primary_container,
        badge: c.error,
        badge_label: c.on_error,
        label_style: theme.typography.label_medium,
    }
}

pub fn resolve_mode(theme: &Theme, mode: RailMode) -> NavRailAppearance {
    let mut a = resolve(theme);
    a.width_dp = mode.width_dp();
    a
}
