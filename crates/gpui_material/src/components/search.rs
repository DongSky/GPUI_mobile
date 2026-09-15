//! Docked search bar + expanded search view.
//! Specs: https://m3.material.io/components/search/specs
//!
//! Catalog shows the 56dp docked bar and the expanded view/sheet (back +
//! input + suggestion list). Full-screen search activity is not a separate
//! platform window.

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
pub const VIEW_BACK: &str = "←";
pub const VIEW_CORNER_DP: f32 = 28.0;
pub const VIEW_HEADER_DP: f32 = 72.0;
pub const SUGGESTION_H_DP: f32 = 56.0;
pub const SUGGESTIONS: [&str; 4] = ["App", "Shortcut", "Recent search", "Setting"];
/// Catalog starts expanded so Visual QA can see the sheet without a tap.
pub const VIEW_OPEN_BY_DEFAULT: bool = true;

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

#[derive(Clone, Debug, PartialEq)]
pub struct SearchViewAppearance {
    pub container: Argb,
    pub header: Argb,
    pub input: Argb,
    pub placeholder: Argb,
    pub divider: Argb,
    pub suggestion: Argb,
    pub suggestion_icon: Argb,
    pub elevation_dp: f32,
    pub corners: Corners,
    pub header_h_dp: f32,
    pub suggestion_h_dp: f32,
    pub title_style: TypeStyle,
    pub suggestion_style: TypeStyle,
}

pub fn resolve_view(theme: &Theme) -> SearchViewAppearance {
    let c = theme.color;
    SearchViewAppearance {
        container: c.surface_container_high,
        header: c.on_surface,
        input: c.on_surface,
        placeholder: c.on_surface_variant,
        divider: c.outline_variant,
        suggestion: c.on_surface,
        suggestion_icon: c.on_surface_variant,
        elevation_dp: theme.elevation.level2,
        corners: Corners::all(VIEW_CORNER_DP),
        header_h_dp: VIEW_HEADER_DP,
        suggestion_h_dp: SUGGESTION_H_DP,
        title_style: theme.typography.body_large,
        suggestion_style: theme.typography.body_large,
    }
}
