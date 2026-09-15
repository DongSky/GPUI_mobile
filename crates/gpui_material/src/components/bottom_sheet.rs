//! Bottom sheet. Specs: https://m3.material.io/components/bottom-sheets/specs
//! Tokens: androidx `SheetBottomTokens` v0_210.

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;

pub const CORNER_TOP_DP: f32 = 28.0;
pub const HANDLE_W_DP: f32 = 32.0;
pub const HANDLE_H_DP: f32 = 4.0;
pub const HANDLE_PAD_TOP_DP: f32 = 16.0;
pub const SCRIM_OPACITY: f32 = 0.32;
/// Official overview: share sheet over a photo grid.
pub const SHARE_TITLE: &str = "Share";
pub const SHARE_ACTIONS: [(&str, &str); 4] = [
    ("🔗", "Copy link"),
    ("👤", "Add person"),
    ("★", "Add to favorites"),
    ("↗", "Share to…"),
];
pub const PHOTO_GRID: [&str; 6] = ["Lake", "Grove", "Dune", "Harbor", "Peak", "Cove"];
pub const PHOTO_TILE_H_DP: f32 = 88.0;
pub const PHOTO_TILE_CORNER_DP: f32 = 12.0;
/// Official share sheet people row (avatars + Add).
pub const PEOPLE: [(&str, &str); 4] = [("AR", "Alex"), ("JL", "Jordan"), ("SC", "Sam"), ("+", "Add")];
pub const PEOPLE_DP: f32 = 56.0;
pub const STATUS_H_DP: f32 = 24.0;
pub const STATUS_TIME: &str = "9:41";
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 620.0;
pub const PHONE_CORNER_DP: f32 = 36.0;
pub const PHONE_BEZEL_DP: f32 = 12.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BottomSheetAppearance {
    pub corners: Corners,
    pub container: Argb,
    pub handle: Argb,
    pub content: Argb,
    pub scrim: Argb,
    pub elevation_dp: f32,
    pub handle_w: f32,
    pub handle_h: f32,
}

pub fn resolve(theme: &Theme, modal: bool) -> BottomSheetAppearance {
    let c = theme.color;
    BottomSheetAppearance {
        corners: Corners {
            top_left: CORNER_TOP_DP,
            top_right: CORNER_TOP_DP,
            bottom_right: 0.0,
            bottom_left: 0.0,
        },
        container: c.surface_container_low,
        handle: c.on_surface_variant,
        content: c.on_surface,
        scrim: if modal {
            c.scrim.with_alpha(SCRIM_OPACITY).composite_over(c.surface)
        } else {
            c.surface
        },
        elevation_dp: theme.elevation.level1,
        handle_w: HANDLE_W_DP,
        handle_h: HANDLE_H_DP,
    }
}

/// Role-color photo stub behind the share sheet.
pub fn photo_fill(theme: &Theme, index: usize) -> Argb {
    let c = theme.color;
    match index % 3 {
        0 => c.primary_container,
        1 => c.secondary_container,
        _ => c.tertiary_container,
    }
}

pub fn photo_on(theme: &Theme, index: usize) -> Argb {
    let c = theme.color;
    match index % 3 {
        0 => c.on_primary_container,
        1 => c.on_secondary_container,
        _ => c.on_tertiary_container,
    }
}

pub fn people_fill(theme: &Theme, index: usize) -> Argb {
    photo_fill(theme, index)
}

pub fn people_on(theme: &Theme, index: usize) -> Argb {
    photo_on(theme, index)
}
