//! Snackbar. Specs: https://m3.material.io/components/snackbar/specs

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const MIN_HEIGHT_DP: f32 = 48.0;
pub const PAD_H_DP: f32 = 16.0;
pub const CORNER_DP: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnackbarAppearance {
    pub min_height_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub supporting: Argb,
    pub action: Argb,
    pub supporting_style: TypeStyle,
    pub action_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> SnackbarAppearance {
    SnackbarAppearance {
        min_height_dp: MIN_HEIGHT_DP,
        corners: Corners::all(CORNER_DP),
        container: theme.color.inverse_surface,
        supporting: theme.color.inverse_on_surface,
        action: theme.color.inverse_primary,
        supporting_style: theme.typography.body_medium,
        action_style: theme.typography.label_large,
    }
}
