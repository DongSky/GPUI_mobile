//! Side sheets (standard / modal / detached).
//! Specs: https://m3.material.io/components/side-sheets/specs
//!
//! Navigation drawers are **not recommended** in Expressive (use the expanded
//! navigation rail). Side sheets remain the supplementary pane.

use crate::argb::Argb;
use crate::components::dialog;
use crate::components::photo_stub::PhotoKind;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const WIDTH_DP: f32 = 256.0;
pub const MAX_WIDTH_DP: f32 = 400.0;
pub const PAD_H_DP: f32 = 24.0;
pub const PAD_H_ICON_DP: f32 = 16.0;
pub const PAD_TOP_GAP_DP: f32 = 12.0;
pub const ACTIONS_H_DP: f32 = 72.0;
pub const ACTIONS_PAD_TOP_DP: f32 = 16.0;
pub const ACTIONS_PAD_BOTTOM_DP: f32 = 24.0;
pub const DETACHED_MARGIN_DP: f32 = 16.0;
pub const SCRIM_OPACITY: f32 = dialog::SCRIM_OPACITY;

pub const HEADLINE: &str = "Filters";
pub const CLOSE_GLYPH: &str = "✕";
pub const APPLY_LABEL: &str = "Apply";
pub const FILTERS: [(&str, &str); 3] = [
    ("Date", "Any time"),
    ("People", "Anyone"),
    ("Places", "Nearby"),
];
pub const SCENE_PHOTOS: [PhotoKind; 4] = [
    PhotoKind::Bloom,
    PhotoKind::Egret,
    PhotoKind::Lake,
    PhotoKind::Grove,
];
pub const STATUS_TIME: &str = "9:30";
pub const STATUS_H_DP: f32 = 24.0;
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 560.0;
pub const PHONE_CORNER_DP: f32 = 36.0;
pub const PHOTO_TILE_H_DP: f32 = 140.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideSheetVariant {
    Standard,
    Modal,
    Detached,
}

impl SideSheetVariant {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Modal, Self::Detached];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Modal => "modal",
            Self::Detached => "detached",
        }
    }

    pub const fn modal(self) -> bool {
        matches!(self, Self::Modal)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SideSheetAppearance {
    pub variant: SideSheetVariant,
    pub width_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub headline: Argb,
    pub content: Argb,
    pub supporting: Argb,
    pub close: Argb,
    pub action: Argb,
    pub divider: Argb,
    pub scrim: Argb,
    pub elevation_dp: f32,
    pub margin_dp: f32,
    pub headline_style: TypeStyle,
    pub supporting_style: TypeStyle,
}

pub fn resolve(theme: &Theme, variant: SideSheetVariant) -> SideSheetAppearance {
    let c = theme.color;
    let (container, elevation_dp, corners, margin_dp) = match variant {
        SideSheetVariant::Standard => (
            c.surface,
            theme.elevation.level0,
            Corners::all(theme.shapes.none),
            0.0,
        ),
        SideSheetVariant::Modal => (
            c.surface_container_low,
            theme.elevation.level1,
            Corners::start(theme.shapes.large),
            0.0,
        ),
        SideSheetVariant::Detached => (
            c.surface_container_low,
            theme.elevation.level1,
            Corners::all(theme.shapes.large),
            DETACHED_MARGIN_DP,
        ),
    };
    SideSheetAppearance {
        variant,
        width_dp: WIDTH_DP,
        corners,
        container,
        headline: c.on_surface_variant,
        content: c.on_surface,
        supporting: c.on_surface_variant,
        close: c.on_surface_variant,
        action: c.primary,
        divider: c.outline,
        scrim: c.scrim.with_alpha(SCRIM_OPACITY).composite_over(c.surface),
        elevation_dp,
        margin_dp,
        headline_style: theme.typography.title_large,
        supporting_style: theme.typography.body_medium,
    }
}

pub fn resolve_scene(theme: &Theme) -> SideSheetAppearance {
    resolve(theme, SideSheetVariant::Modal)
}

pub fn is_modal(variant: SideSheetVariant) -> bool {
    variant.modal()
}

pub fn dismiss_on_scrim(variant: SideSheetVariant) -> bool {
    variant.modal()
}
