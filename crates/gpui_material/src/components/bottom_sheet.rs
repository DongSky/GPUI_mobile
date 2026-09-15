//! Bottom sheet. Specs: https://m3.material.io/components/bottom-sheets/specs
//! Tokens: androidx `SheetBottomTokens` v0_210.

use crate::argb::Argb;
use crate::components::photo_stub::{self, PhotoKind};
use crate::shape::Corners;
use crate::theme::Theme;

pub const CORNER_TOP_DP: f32 = 28.0;
pub const HANDLE_W_DP: f32 = 32.0;
pub const HANDLE_H_DP: f32 = 4.0;
pub const HANDLE_PAD_TOP_DP: f32 = 16.0;
pub const SCRIM_OPACITY: f32 = 0.32;
/// Official overview: share sheet over a photo album, horizontal actions + Send.
pub const SHARE_TITLE: &str = "Share";
pub const SEND_TITLE: &str = "Send";
pub const ALBUM_ACTIONS: [&str; 3] = ["Add photos", "Save all", "Share"];
pub const SHARE_ACTIONS: [(&str, &str); 5] = [
    ("↗", "Share"),
    ("+", "Add to"),
    ("🗑", "Trash"),
    ("🖨", "Order prints"),
    ("📁", "Move to archive"),
];
pub const PHOTO_GRID: [&str; 6] = ["Party", "Bloom", "Lake", "Grove", "Peak", "Cove"];
pub const PHOTO_TILE_H_DP: f32 = 160.0;
pub const PHOTO_TILE_CORNER_DP: f32 = 0.0;
/// Official share sheet people row (named camera portraits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Person {
    pub first: &'static str,
    pub last: &'static str,
    pub photo: PhotoKind,
}

pub const PEOPLE: [Person; 5] = [
    Person {
        first: "Alejandro",
        last: "Ortega",
        photo: PhotoKind::PortraitAlejandro,
    },
    Person {
        first: "Oli",
        last: "Ortega",
        photo: PhotoKind::PortraitOli,
    },
    Person {
        first: "Carmen",
        last: "Villanueva",
        photo: PhotoKind::PortraitCarmen,
    },
    Person {
        first: "Ana",
        last: "Russo",
        photo: PhotoKind::PortraitAna,
    },
    Person {
        first: "Marty",
        last: "Reyes",
        photo: PhotoKind::PortraitMarty,
    },
];
pub const PEOPLE_DP: f32 = 64.0;
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

/// Photographic album stub behind the share sheet.
pub fn photo_kind(index: usize) -> PhotoKind {
    photo_stub::share_kind(index)
}

pub fn photo_fill(_theme: &Theme, index: usize) -> Argb {
    photo_kind(index).fill()
}

pub fn photo_on(_theme: &Theme, index: usize) -> Argb {
    photo_kind(index).on_fill()
}

pub fn people_fill(_theme: &Theme, index: usize) -> Argb {
    PEOPLE[index % PEOPLE.len()].photo.fill()
}

pub fn people_on(_theme: &Theme, index: usize) -> Argb {
    PEOPLE[index % PEOPLE.len()].photo.on_fill()
}
