//! Navigation bar. Specs: https://m3.material.io/components/navigation-bar/specs

use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 80.0;
pub const INDICATOR_W_DP: f32 = 64.0;
pub const INDICATOR_H_DP: f32 = 32.0;
pub const ICON_DP: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavBarAppearance {
    pub height_dp: f32,
    pub container: Argb,
    pub active_indicator: Argb,
    pub active_icon: Argb,
    pub active_label: Argb,
    pub inactive_icon: Argb,
    pub inactive_label: Argb,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> NavBarAppearance {
    let c = theme.color;
    NavBarAppearance {
        height_dp: HEIGHT_DP,
        container: c.surface_container,
        active_indicator: c.secondary_container,
        active_icon: c.on_secondary_container,
        active_label: c.on_surface,
        inactive_icon: c.on_surface_variant,
        inactive_label: c.on_surface_variant,
        label_style: theme.typography.label_medium,
    }
}
