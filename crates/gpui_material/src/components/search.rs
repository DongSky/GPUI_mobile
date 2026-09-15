//! Docked search bar. Specs: https://m3.material.io/components/search/specs
//!
//! Catalog hero is the 56dp docked bar (full round, surface-container-high),
//! not the expanded search view.

use crate::argb::Argb;
use crate::components::Appearance;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 56.0;
pub const PAD_H_DP: f32 = 16.0;
pub const ICON_DP: f32 = 24.0;
pub const GAP_DP: f32 = 16.0;
pub const AVATAR_DP: f32 = 30.0;
pub const PLACEHOLDER: &str = "Hinted search text";
pub const LEADING_ICON: &str = "⌕";
pub const TRAILING_MIC: &str = "🎤";

#[derive(Clone, Debug, PartialEq)]
pub struct SearchAppearance {
    pub bar: Appearance,
    pub placeholder: Argb,
    pub leading_icon: Argb,
    pub trailing_icon: Argb,
    pub avatar: Argb,
    pub avatar_label: Argb,
    pub placeholder_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> SearchAppearance {
    let c = theme.color;
    SearchAppearance {
        bar: Appearance {
            width_dp: None,
            height_dp: HEIGHT_DP,
            min_width_dp: Some(360.0),
            corners: Corners::all(HEIGHT_DP / 2.0),
            container: c.surface_container_high,
            content: c.on_surface,
            secondary_content: Some(c.on_surface_variant),
            outline: None,
            elevation_dp: 0.0,
            pad_start_dp: PAD_H_DP,
            pad_end_dp: PAD_H_DP,
            pad_top_dp: 0.0,
            pad_bottom_dp: 0.0,
            label_style: theme.typography.body_large,
            supporting_style: Some(theme.typography.body_large),
        },
        placeholder: c.on_surface_variant,
        leading_icon: c.on_surface,
        trailing_icon: c.on_surface_variant,
        avatar: c.primary_container,
        avatar_label: c.on_primary_container,
        placeholder_style: theme.typography.body_large,
    }
}
