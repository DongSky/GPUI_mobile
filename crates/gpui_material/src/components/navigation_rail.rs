//! Navigation rail. Specs: https://m3.material.io/components/navigation-rail/specs
//!
//! Catalog stub: 80dp vertical destinations with a 56×32 active indicator.
//! Collapsed/expanded modal and FAB alignment are not wired.

use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const WIDTH_DP: f32 = 80.0;
pub const INDICATOR_W_DP: f32 = 56.0;
pub const INDICATOR_H_DP: f32 = 32.0;
pub const ICON_DP: f32 = 24.0;
pub const DEST_GAP_DP: f32 = 12.0;
pub const PAD_TOP_DP: f32 = 16.0;

pub const DESTINATIONS: [&str; 3] = ["Home", "Search", "Profile"];
pub const DESTINATION_ICONS: [&str; 3] = ["⌂", "⌕", "☺"];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavRailAppearance {
    pub width_dp: f32,
    pub container: Argb,
    pub active_indicator: Argb,
    pub active_icon: Argb,
    pub active_label: Argb,
    pub inactive_icon: Argb,
    pub inactive_label: Argb,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> NavRailAppearance {
    let c = theme.color;
    NavRailAppearance {
        width_dp: WIDTH_DP,
        container: c.surface,
        active_indicator: c.secondary_container,
        active_icon: c.on_secondary_container,
        active_label: c.on_surface,
        inactive_icon: c.on_surface_variant,
        inactive_label: c.on_surface_variant,
        label_style: theme.typography.label_medium,
    }
}
