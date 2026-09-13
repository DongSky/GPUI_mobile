//! Basic dialog. Specs: https://m3.material.io/components/dialogs/specs
//! Tokens: androidx `DialogTokens` v0_210.

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const CORNER_DP: f32 = 28.0;
pub const PAD_DP: f32 = 24.0;
pub const ACTION_GAP_DP: f32 = 8.0;
pub const MIN_WIDTH_DP: f32 = 280.0;
pub const ICON_DP: f32 = 24.0;
pub const SCRIM_OPACITY: f32 = 0.32;

/// Guidelines basic-dialog anatomy (icon + headline + supporting + text actions).
pub const RESET_HEADLINE: &str = "Reset settings?";
pub const RESET_SUPPORTING: &str =
    "This will reset your app preferences back to their default settings. You can't undo this action.";
/// Refresh-style glyph; official guidelines use a circular-arrows icon.
pub const RESET_ICON: &str = "↻";
pub const RESET_CANCEL: &str = "Cancel";
pub const RESET_ACCEPT: &str = "Accept";

/// Overview list-dialog hero (Phone ringtone).
pub const RINGTONE_HEADLINE: &str = "Phone ringtone";
pub const RINGTONE_OPTIONS: [&str; 4] = ["None", "Callisto", "Ganymede", "Luna"];
pub const RINGTONE_CANCEL: &str = "Cancel";
pub const RINGTONE_OK: &str = "OK";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DialogAppearance {
    pub corners: Corners,
    pub container: Argb,
    pub headline: Argb,
    pub supporting: Argb,
    pub action: Argb,
    pub icon: Argb,
    pub scrim: Argb,
    pub elevation_dp: f32,
    pub pad_dp: f32,
    pub min_width_dp: f32,
    pub headline_style: TypeStyle,
    pub supporting_style: TypeStyle,
    pub action_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> DialogAppearance {
    let c = theme.color;
    DialogAppearance {
        corners: Corners::all(CORNER_DP),
        container: c.surface_container_high,
        headline: c.on_surface,
        supporting: c.on_surface_variant,
        action: c.primary,
        icon: c.secondary,
        scrim: c.scrim.with_alpha(SCRIM_OPACITY).composite_over(c.surface),
        elevation_dp: theme.elevation.level3,
        pad_dp: PAD_DP,
        min_width_dp: MIN_WIDTH_DP,
        headline_style: theme.typography.headline_small,
        supporting_style: theme.typography.body_medium,
        action_style: theme.typography.label_large,
    }
}
