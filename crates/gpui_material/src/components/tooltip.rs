//! Tooltips — plain and rich.
//! Specs: https://m3.material.io/components/tooltips/specs
//! Tokens: androidx Compose `PlainTooltipTokens` / `RichTooltipTokens` v0_210
//! plus `Tooltip.kt` layout (24dp min / 200dp plain max / 320dp rich max).

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

/// Spec / Compose plain tooltip minimum container height.
pub const PLAIN_MIN_H_DP: f32 = 24.0;
pub const PLAIN_MIN_W_DP: f32 = 40.0;
pub const PLAIN_MAX_W_DP: f32 = 200.0;
/// Spec table lists 8dp padding; Compose uses 8 horizontal / 4 vertical so
/// bodySmall (16 line) + 8 = 24dp container height.
pub const PLAIN_PAD_H_DP: f32 = 8.0;
pub const PLAIN_PAD_V_DP: f32 = 4.0;

pub const RICH_MAX_W_DP: f32 = 320.0;
pub const RICH_PAD_TOP_DP: f32 = 12.0;
pub const RICH_PAD_BOTTOM_DP: f32 = 8.0;
pub const RICH_PAD_H_DP: f32 = 16.0;

pub const ANCHOR_GAP_DP: f32 = 4.0;
pub const CARET_W_DP: f32 = 16.0;
pub const CARET_H_DP: f32 = 8.0;

/// Official overview: plain tooltip labels an icon-only control.
pub const PLAIN_TEXT: &str = "Add to library";
pub const PLAIN_ANCHOR: &str = "+";

/// Official rich configurations: subhead + supporting + two text buttons.
pub const RICH_SUBHEAD: &str = "Rich tooltip";
pub const RICH_SUPPORTING: &str =
    "Rich tooltips add a subhead, supporting text, and up to two actions.";
pub const RICH_ACTION_PRIMARY: &str = "Learn more";
pub const RICH_ACTION_SECONDARY: &str = "Dismiss";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipKind {
    Plain,
    Rich,
}

impl TooltipKind {
    pub const ALL: [Self; 2] = [Self::Plain, Self::Rich];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Rich => "rich",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipAppearance {
    pub kind: TooltipKind,
    pub min_height_dp: f32,
    pub min_width_dp: f32,
    pub max_width_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub supporting: Argb,
    pub subhead: Option<Argb>,
    pub action: Option<Argb>,
    pub elevation_dp: f32,
    pub pad_start_dp: f32,
    pub pad_end_dp: f32,
    pub pad_top_dp: f32,
    pub pad_bottom_dp: f32,
    pub supporting_style: TypeStyle,
    pub subhead_style: Option<TypeStyle>,
    pub action_style: Option<TypeStyle>,
}

pub fn resolve(theme: &Theme, kind: TooltipKind) -> TooltipAppearance {
    match kind {
        TooltipKind::Plain => resolve_plain(theme),
        TooltipKind::Rich => resolve_rich(theme),
    }
}

pub fn resolve_plain(theme: &Theme) -> TooltipAppearance {
    let c = theme.color;
    TooltipAppearance {
        kind: TooltipKind::Plain,
        min_height_dp: PLAIN_MIN_H_DP,
        min_width_dp: PLAIN_MIN_W_DP,
        max_width_dp: PLAIN_MAX_W_DP,
        corners: Corners::all(theme.shapes.extra_small),
        container: c.inverse_surface,
        supporting: c.inverse_on_surface,
        subhead: None,
        action: None,
        elevation_dp: theme.elevation.level0,
        pad_start_dp: PLAIN_PAD_H_DP,
        pad_end_dp: PLAIN_PAD_H_DP,
        pad_top_dp: PLAIN_PAD_V_DP,
        pad_bottom_dp: PLAIN_PAD_V_DP,
        supporting_style: theme.typography.body_small,
        subhead_style: None,
        action_style: None,
    }
}

pub fn resolve_rich(theme: &Theme) -> TooltipAppearance {
    let c = theme.color;
    TooltipAppearance {
        kind: TooltipKind::Rich,
        min_height_dp: RICH_PAD_TOP_DP + theme.typography.body_medium.line_height_sp + RICH_PAD_BOTTOM_DP,
        min_width_dp: PLAIN_MIN_W_DP,
        max_width_dp: RICH_MAX_W_DP,
        corners: Corners::all(theme.shapes.medium),
        container: c.surface_container,
        supporting: c.on_surface_variant,
        subhead: Some(c.on_surface_variant),
        action: Some(c.primary),
        elevation_dp: theme.elevation.level2,
        pad_start_dp: RICH_PAD_H_DP,
        pad_end_dp: RICH_PAD_H_DP,
        pad_top_dp: RICH_PAD_TOP_DP,
        pad_bottom_dp: RICH_PAD_BOTTOM_DP,
        supporting_style: theme.typography.body_medium,
        subhead_style: Some(theme.typography.title_small),
        action_style: Some(theme.typography.label_large),
    }
}

pub fn resolve_scene_plain(theme: &Theme) -> TooltipAppearance {
    resolve_plain(theme)
}

pub fn resolve_scene_rich(theme: &Theme) -> TooltipAppearance {
    resolve_rich(theme)
}

pub fn has_actions(kind: TooltipKind) -> bool {
    matches!(kind, TooltipKind::Rich)
}

/// Downward caret (16×8) attached under the tooltip, pointing at the anchor.
pub fn caret_down_points() -> [(f32, f32); 3] {
    [
        (0.0, 0.0),
        (CARET_W_DP, 0.0),
        (CARET_W_DP / 2.0, CARET_H_DP),
    ]
}

/// Upward caret for a rich tooltip sitting below its anchor.
pub fn caret_up_points() -> [(f32, f32); 3] {
    [
        (0.0, CARET_H_DP),
        (CARET_W_DP, CARET_H_DP),
        (CARET_W_DP / 2.0, 0.0),
    ]
}
