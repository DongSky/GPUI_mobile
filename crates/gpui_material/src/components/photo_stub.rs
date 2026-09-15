//! Photographic CSS/GPUI stubs for catalog scene chrome.
//!
//! Official m3.material.io overviews use camera photos. This crate stays
//! host-testable (no image decoder), so heroes paint layered gradients that
//! read as landscapes, portraits, product, and chat media instead of flat
//! role-color tiles.

use crate::argb::Argb;

/// Named camera-style stub used by snackbar, tabs, sheets, carousel, and
/// the FAB-menu / split-button / toolbar product scenes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhotoKind {
    Bloom,
    Egret,
    Basket,
    Mugs,
    Dog,
    Party,
    Lake,
    Grove,
    Dune,
    Harbor,
    Peak,
    Cove,
    PortraitSofia,
    PortraitCarmen,
    PortraitAlejandro,
    PortraitOli,
    PortraitAna,
    PortraitMarty,
    PortraitRenee,
}

impl PhotoKind {
    pub const CAROUSEL: [Self; 4] = [Self::Lake, Self::Grove, Self::Dune, Self::Harbor];
    pub const SHARE_ALBUM: [Self; 6] = [
        Self::Party,
        Self::Bloom,
        Self::Lake,
        Self::Grove,
        Self::Peak,
        Self::Cove,
    ];
    pub const TABS: [Self; 2] = [Self::Bloom, Self::Egret];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Bloom => "bloom",
            Self::Egret => "egret",
            Self::Basket => "basket",
            Self::Mugs => "mugs",
            Self::Dog => "dog",
            Self::Party => "party",
            Self::Lake => "lake",
            Self::Grove => "grove",
            Self::Dune => "dune",
            Self::Harbor => "harbor",
            Self::Peak => "peak",
            Self::Cove => "cove",
            Self::PortraitSofia => "sofia",
            Self::PortraitCarmen => "carmen",
            Self::PortraitAlejandro => "alejandro",
            Self::PortraitOli => "oli",
            Self::PortraitAna => "ana",
            Self::PortraitMarty => "marty",
            Self::PortraitRenee => "renee",
        }
    }

    pub const fn is_portrait(self) -> bool {
        matches!(
            self,
            Self::PortraitSofia
                | Self::PortraitCarmen
                | Self::PortraitAlejandro
                | Self::PortraitOli
                | Self::PortraitAna
                | Self::PortraitMarty
                | Self::PortraitRenee
        )
    }

    /// Dominant fill for GPUI hosts (no CSS gradient).
    pub const fn fill(self) -> Argb {
        match self {
            Self::Bloom => Argb::rgb(0xF8, 0xBB, 0xD0),
            Self::Egret => Argb::rgb(0x54, 0x6E, 0x7A),
            Self::Basket => Argb::rgb(0xC4, 0xA4, 0x84),
            Self::Mugs => Argb::rgb(0xE8, 0xEE, 0xF4),
            Self::Dog => Argb::rgb(0x5D, 0x40, 0x37),
            Self::Party => Argb::rgb(0x81, 0xC7, 0x84),
            Self::Lake => Argb::rgb(0x4F, 0xC3, 0xF7),
            Self::Grove => Argb::rgb(0x66, 0xBB, 0x6A),
            Self::Dune => Argb::rgb(0xFF, 0xCC, 0x80),
            Self::Harbor => Argb::rgb(0x4D, 0xB6, 0xAC),
            Self::Peak => Argb::rgb(0x90, 0xA4, 0xAE),
            Self::Cove => Argb::rgb(0x80, 0xDE, 0xEA),
            Self::PortraitSofia => Argb::rgb(0xD7, 0xA8, 0x8C),
            Self::PortraitCarmen => Argb::rgb(0xC6, 0x90, 0x76),
            Self::PortraitAlejandro => Argb::rgb(0x8D, 0x6E, 0x63),
            Self::PortraitOli => Argb::rgb(0xBC, 0xAA, 0xA4),
            Self::PortraitAna => Argb::rgb(0xE0, 0xBE, 0xA8),
            Self::PortraitMarty => Argb::rgb(0xA1, 0x88, 0x7F),
            Self::PortraitRenee => Argb::rgb(0xFF, 0xCC, 0x80),
        }
    }

    pub const fn accent(self) -> Argb {
        match self {
            Self::Bloom => Argb::rgb(0x81, 0xC7, 0x84),
            Self::Egret => Argb::rgb(0xEC, 0xEF, 0xF1),
            Self::Basket => Argb::rgb(0x6F, 0x4E, 0x37),
            Self::Mugs => Argb::rgb(0xC6, 0x28, 0x28),
            Self::Dog => Argb::rgb(0x21, 0x21, 0x21),
            Self::Party => Argb::rgb(0xFF, 0xF5, 0x9D),
            Self::Lake => Argb::rgb(0x01, 0x57, 0x9B),
            Self::Grove => Argb::rgb(0x2E, 0x7D, 0x32),
            Self::Dune => Argb::rgb(0xEF, 0x6C, 0x00),
            Self::Harbor => Argb::rgb(0x00, 0x69, 0x5C),
            Self::Peak => Argb::rgb(0x37, 0x47, 0x4F),
            Self::Cove => Argb::rgb(0x00, 0x83, 0x8F),
            Self::PortraitSofia => Argb::rgb(0x5D, 0x40, 0x37),
            Self::PortraitCarmen => Argb::rgb(0x3E, 0x27, 0x23),
            Self::PortraitAlejandro => Argb::rgb(0x3E, 0x27, 0x23),
            Self::PortraitOli => Argb::rgb(0x4E, 0x34, 0x2E),
            Self::PortraitAna => Argb::rgb(0x6D, 0x4C, 0x41),
            Self::PortraitMarty => Argb::rgb(0x3E, 0x27, 0x23),
            Self::PortraitRenee => Argb::rgb(0xBF, 0x36, 0x0C),
        }
    }

    pub const fn on_fill(self) -> Argb {
        match self {
            Self::Bloom | Self::Mugs | Self::Dune | Self::Party | Self::PortraitRenee => {
                Argb::rgb(0x3E, 0x27, 0x23)
            }
            _ => Argb::WHITE,
        }
    }

    /// Layered CSS background. Portraits are circular-friendly radial fills.
    pub const fn css_background(self) -> &'static str {
        match self {
            Self::Bloom => {
                "linear-gradient(180deg,#E1BEE7 0%,#F8BBD0 38%,#C8E6C9 72%,#43A047 100%)"
            }
            Self::Egret => {
                "linear-gradient(180deg,#90CAF9 0%,#BBDEFB 42%,#78909C 70%,#37474F 100%),radial-gradient(circle at 68% 58%,#ECEFF1 0 8%,transparent 9%)"
            }
            Self::Basket => {
                "radial-gradient(circle at 50% 42%,#D7B899 0 16%,#A67C52 16% 28%,#C4A484 28% 46%,#8B5E3C 46% 68%,#6F4E37 68% 100%)"
            }
            Self::Mugs => {
                "linear-gradient(180deg,#EEF3F8 0%,#F7F1E8 55%,#E0D6C8 100%)"
            }
            Self::Dog => {
                "radial-gradient(ellipse at 50% 38%,#6D4C41 0 28%,#A1887F 28% 52%,#3E2723 52% 100%)"
            }
            Self::Party => {
                "linear-gradient(180deg,#81C784 0%,#AED581 36%,#FFF59D 68%,#FFCC80 100%)"
            }
            Self::Lake => {
                "linear-gradient(180deg,#81D4FA 0%,#4FC3F7 45%,#0277BD 100%)"
            }
            Self::Grove => {
                "linear-gradient(180deg,#A5D6A7 0%,#66BB6A 40%,#2E7D32 100%)"
            }
            Self::Dune => {
                "linear-gradient(180deg,#FFE0B2 0%,#FFCC80 42%,#EF6C00 100%)"
            }
            Self::Harbor => {
                "linear-gradient(180deg,#80CBC4 0%,#4DB6AC 48%,#00695C 100%)"
            }
            Self::Peak => {
                "linear-gradient(180deg,#B0BEC5 0%,#90A4AE 40%,#37474F 100%)"
            }
            Self::Cove => {
                "linear-gradient(180deg,#B2EBF2 0%,#80DEEA 50%,#00838F 100%)"
            }
            Self::PortraitSofia => {
                "radial-gradient(circle at 50% 28%,#F3D5C0 0 22%,#5D4037 22% 40%,#8D6E63 40% 100%)"
            }
            Self::PortraitCarmen => {
                "radial-gradient(circle at 48% 30%,#E0BEA8 0 20%,#3E2723 20% 42%,#6D4C41 42% 100%)"
            }
            Self::PortraitAlejandro => {
                "radial-gradient(circle at 50% 26%,#D7CCC8 0 18%,#3E2723 18% 38%,#5D4037 38% 100%)"
            }
            Self::PortraitOli => {
                "radial-gradient(circle at 52% 30%,#FFE0B2 0 22%,#4E342E 22% 44%,#A1887F 44% 100%)"
            }
            Self::PortraitAna => {
                "radial-gradient(circle at 50% 28%,#F8BBD0 0 20%,#6D4C41 20% 40%,#BCAAA4 40% 100%)"
            }
            Self::PortraitMarty => {
                "radial-gradient(circle at 46% 32%,#FFCC80 0 18%,#3E2723 18% 40%,#795548 40% 100%)"
            }
            Self::PortraitRenee => {
                "radial-gradient(circle at 50% 30%,#FFE0B2 0 24%,#BF360C 24% 48%,#FFCC80 48% 100%)"
            }
        }
    }
}

pub fn carousel_kind(index: usize) -> PhotoKind {
    PhotoKind::CAROUSEL[index % PhotoKind::CAROUSEL.len()]
}

pub fn share_kind(index: usize) -> PhotoKind {
    PhotoKind::SHARE_ALBUM[index % PhotoKind::SHARE_ALBUM.len()]
}

pub fn tabs_kind(index: usize) -> PhotoKind {
    PhotoKind::TABS[index % PhotoKind::TABS.len()]
}
