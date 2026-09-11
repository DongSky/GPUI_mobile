//! Small top app bar. Specs: https://m3.material.io/components/top-app-bar/specs

use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 64.0;
pub const PAD_H_DP: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TopAppBarAppearance {
    pub height_dp: f32,
    pub container: Argb,
    pub title: Argb,
    pub icon: Argb,
    pub title_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> TopAppBarAppearance {
    TopAppBarAppearance {
        height_dp: HEIGHT_DP,
        container: theme.color.surface,
        title: theme.color.on_surface,
        icon: theme.color.on_surface,
        title_style: theme.typography.title_large,
    }
}
