//! Licensed camera stills for catalog scene chrome.
//!
//! Official [m3.material.io](https://m3.material.io) overviews use real
//! photographs. Heroes load a small bundled JPEG per [`PhotoKind`] (Commons /
//! Unsplash licenses in `assets/photos/README.md`), then **decode** it for HTML
//! `data:` URIs and GPUI mosaic samples.

use crate::argb::Argb;

/// Named camera photo used by snackbar, tabs, sheets, carousel, and
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

/// License + source page for a bundled still.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhotoCredit {
    pub license: &'static str,
    pub source: &'static str,
    pub page: &'static str,
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
    pub const ALL: [Self; 19] = [
        Self::Bloom,
        Self::Egret,
        Self::Basket,
        Self::Mugs,
        Self::Dog,
        Self::Party,
        Self::Lake,
        Self::Grove,
        Self::Dune,
        Self::Harbor,
        Self::Peak,
        Self::Cove,
        Self::PortraitSofia,
        Self::PortraitCarmen,
        Self::PortraitAlejandro,
        Self::PortraitOli,
        Self::PortraitAna,
        Self::PortraitMarty,
        Self::PortraitRenee,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::Bloom => 0,
            Self::Egret => 1,
            Self::Basket => 2,
            Self::Mugs => 3,
            Self::Dog => 4,
            Self::Party => 5,
            Self::Lake => 6,
            Self::Grove => 7,
            Self::Dune => 8,
            Self::Harbor => 9,
            Self::Peak => 10,
            Self::Cove => 11,
            Self::PortraitSofia => 12,
            Self::PortraitCarmen => 13,
            Self::PortraitAlejandro => 14,
            Self::PortraitOli => 15,
            Self::PortraitAna => 16,
            Self::PortraitMarty => 17,
            Self::PortraitRenee => 18,
        }
    }

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

    /// Every catalog still is a bundled licensed camera JPEG (not procedural).
    pub const fn is_licensed_camera(self) -> bool {
        true
    }

    pub const fn credit(self) -> PhotoCredit {
        match self {
            Self::Bloom => PhotoCredit {
                license: "CC0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Pink_Peony_Flower.jpg",
            },
            Self::Egret => PhotoCredit {
                license: "CC BY-SA 3.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Great_Egret_Inching_Closer.JPG",
            },
            Self::Basket => PhotoCredit {
                license: "CC BY-SA 4.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Wicker_basket.jpg",
            },
            Self::Mugs => PhotoCredit {
                license: "CC BY 2.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:%22All_I_want_is_a_proper_cup_of_coffee_...%22_(15637245653).jpg",
            },
            Self::Dog => PhotoCredit {
                license: "CC BY-SA 3.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Yellow_Labrador_Retriever_2.jpg",
            },
            Self::Party => PhotoCredit {
                license: "CC BY-SA 2.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:English_birthday_party_-_Flickr_-_gagilas.jpg",
            },
            Self::Lake => PhotoCredit {
                license: "Public domain",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Moraine_Lake_17092005.jpg",
            },
            Self::Grove => PhotoCredit {
                license: "CC BY-SA 2.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Redwood_National_Park,_fog_in_the_forest.jpg",
            },
            Self::Dune => PhotoCredit {
                license: "CC BY-SA 3.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Namib-Naukluft_Sand_Dunes_(2011).jpg",
            },
            Self::Harbor => PhotoCredit {
                license: "CC BY-SA 3.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Fishing_boats._Ano_Symi_harbor,_Greece.jpg",
            },
            Self::Peak => PhotoCredit {
                license: "CC BY-SA 3.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Matterhorn_from_Domh%C3%BCtte_-_2.jpg",
            },
            Self::Cove => PhotoCredit {
                license: "CC BY-SA 3.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:McWay_Falls_Big_Sur_May_2011_003.jpg",
            },
            Self::PortraitSofia => PhotoCredit {
                license: "CC BY-SA 3.0",
                source: "commons",
                page: "https://commons.wikimedia.org/wiki/File:Laughing_Woman_(Imagicity_1162).jpg",
            },
            Self::PortraitCarmen => PhotoCredit {
                license: "Unsplash License",
                source: "unsplash",
                page: "https://images.unsplash.com/photo-1494790108377-be9c29b29330",
            },
            Self::PortraitAlejandro => PhotoCredit {
                license: "Unsplash License",
                source: "unsplash",
                page: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d",
            },
            Self::PortraitOli => PhotoCredit {
                license: "Unsplash License",
                source: "unsplash",
                page: "https://images.unsplash.com/photo-1500648767791-00dcc994a43e",
            },
            Self::PortraitAna => PhotoCredit {
                license: "Unsplash License",
                source: "unsplash",
                page: "https://images.unsplash.com/photo-1534528741775-53994a69daeb",
            },
            Self::PortraitMarty => PhotoCredit {
                license: "Unsplash License",
                source: "unsplash",
                page: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e",
            },
            Self::PortraitRenee => PhotoCredit {
                license: "Unsplash License",
                source: "unsplash",
                page: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80",
            },
        }
    }

    /// Dominant fill sampled from the bundled still (seed + GPUI fallback).
    pub const fn fill(self) -> Argb {
        match self {
            Self::Bloom => Argb::rgb(0xA3, 0x46, 0x5C),
            Self::Egret => Argb::rgb(0x8E, 0xBC, 0xEA),
            Self::Basket => Argb::rgb(0x48, 0x48, 0x50),
            Self::Mugs => Argb::rgb(0x76, 0x4F, 0x2C),
            Self::Dog => Argb::rgb(0x84, 0x83, 0x6B),
            Self::Party => Argb::rgb(0x63, 0x5A, 0x47),
            Self::Lake => Argb::rgb(0x4D, 0x65, 0x82),
            Self::Grove => Argb::rgb(0x69, 0x71, 0x4F),
            Self::Dune => Argb::rgb(0x97, 0x58, 0x45),
            Self::Harbor => Argb::rgb(0x4B, 0x4C, 0x4C),
            Self::Peak => Argb::rgb(0x48, 0x6A, 0xB0),
            Self::Cove => Argb::rgb(0x36, 0x65, 0x6C),
            Self::PortraitSofia => Argb::rgb(0x64, 0x6E, 0x69),
            Self::PortraitCarmen => Argb::rgb(0x7E, 0x48, 0x4E),
            Self::PortraitAlejandro => Argb::rgb(0x94, 0x8A, 0x85),
            Self::PortraitOli => Argb::rgb(0x6D, 0x6F, 0x77),
            Self::PortraitAna => Argb::rgb(0x70, 0x73, 0xA8),
            Self::PortraitMarty => Argb::rgb(0x9C, 0x9A, 0x97),
            Self::PortraitRenee => Argb::rgb(0x86, 0x87, 0x8B),
        }
    }

    pub const fn accent(self) -> Argb {
        match self {
            Self::Bloom => Argb::rgb(0xF4, 0x41, 0xA9),
            Self::Egret => Argb::rgb(0x01, 0x57, 0x9B),
            Self::Basket => Argb::rgb(0x92, 0x87, 0x85),
            Self::Mugs => Argb::rgb(0xBC, 0x70, 0x24),
            Self::Dog => Argb::rgb(0x3E, 0x27, 0x23),
            Self::Party => Argb::rgb(0xFF, 0xF5, 0x9D),
            Self::Lake => Argb::rgb(0x01, 0x57, 0x9B),
            Self::Grove => Argb::rgb(0x2E, 0x7D, 0x32),
            Self::Dune => Argb::rgb(0x77, 0x3D, 0x3C),
            Self::Harbor => Argb::rgb(0x00, 0x69, 0x5C),
            Self::Peak => Argb::rgb(0x36, 0x60, 0xB6),
            Self::Cove => Argb::rgb(0x00, 0x83, 0x8F),
            Self::PortraitSofia => Argb::rgb(0x5D, 0x40, 0x37),
            Self::PortraitCarmen => Argb::rgb(0x3E, 0x27, 0x23),
            Self::PortraitAlejandro => Argb::rgb(0x3E, 0x27, 0x23),
            Self::PortraitOli => Argb::rgb(0x4E, 0x34, 0x2E),
            Self::PortraitAna => Argb::rgb(0x6D, 0x4C, 0x41),
            Self::PortraitMarty => Argb::rgb(0x3E, 0x27, 0x23),
            Self::PortraitRenee => Argb::rgb(0x5D, 0x43, 0x46),
        }
    }

    pub const fn on_fill(self) -> Argb {
        let f = self.fill();
        let y = (f.r() as u32 * 299 + f.g() as u32 * 587 + f.b() as u32 * 114) / 1000;
        if y > 160 {
            Argb::rgb(0x3E, 0x27, 0x23)
        } else {
            Argb::WHITE
        }
    }

    /// Bundled JPEG bytes (SOI `FF D8`).
    pub fn jpeg_bytes(self) -> &'static [u8] {
        jpeg_static(self)
    }

    /// `data:image/jpeg;base64,…` for catalog `<img>` / CSS `url()`.
    pub fn data_uri(self) -> String {
        format!(
            "data:image/jpeg;base64,{}",
            base64_encode(self.jpeg_bytes())
        )
    }

    /// CSS background using the decoded JPEG URI (cover, centered).
    pub fn css_background(self) -> String {
        format!("url('{}') center / cover no-repeat", self.data_uri())
    }

    /// Decode the bundled JPEG to packed RGB8.
    pub fn decode_rgb(self) -> (u32, u32, Vec<u8>) {
        let img = image::load_from_memory(self.jpeg_bytes())
            .expect("catalog JPEG must decode")
            .to_rgb8();
        (img.width(), img.height(), img.into_raw())
    }

    /// Sample the decoded JPEG into a GPUI mosaic (no image element API).
    pub fn mosaic(self, cols: u32, rows: u32) -> Vec<Argb> {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let (w, h, rgb) = self.decode_rgb();
        let mut out = Vec::with_capacity((cols * rows) as usize);
        for y in 0..rows {
            let sy = (((y as f32 + 0.5) / rows as f32) * h as f32) as u32;
            let sy = sy.min(h.saturating_sub(1));
            for x in 0..cols {
                let sx = (((x as f32 + 0.5) / cols as f32) * w as f32) as u32;
                let sx = sx.min(w.saturating_sub(1));
                let i = ((sy * w + sx) * 3) as usize;
                out.push(Argb::rgb(rgb[i], rgb[i + 1], rgb[i + 2]));
            }
        }
        out
    }
}

pub const MOSAIC_WIDE_COLS: u32 = 16;
pub const MOSAIC_WIDE_ROWS: u32 = 12;
pub const MOSAIC_AVATAR: u32 = 8;

pub fn carousel_kind(index: usize) -> PhotoKind {
    PhotoKind::CAROUSEL[index % PhotoKind::CAROUSEL.len()]
}

pub fn share_kind(index: usize) -> PhotoKind {
    PhotoKind::SHARE_ALBUM[index % PhotoKind::SHARE_ALBUM.len()]
}

pub fn tabs_kind(index: usize) -> PhotoKind {
    PhotoKind::TABS[index % PhotoKind::TABS.len()]
}

fn jpeg_static(kind: PhotoKind) -> &'static [u8] {
    match kind {
        PhotoKind::Bloom => include_bytes!("../../assets/photos/bloom.jpg"),
        PhotoKind::Egret => include_bytes!("../../assets/photos/egret.jpg"),
        PhotoKind::Basket => include_bytes!("../../assets/photos/basket.jpg"),
        PhotoKind::Mugs => include_bytes!("../../assets/photos/mugs.jpg"),
        PhotoKind::Dog => include_bytes!("../../assets/photos/dog.jpg"),
        PhotoKind::Party => include_bytes!("../../assets/photos/party.jpg"),
        PhotoKind::Lake => include_bytes!("../../assets/photos/lake.jpg"),
        PhotoKind::Grove => include_bytes!("../../assets/photos/grove.jpg"),
        PhotoKind::Dune => include_bytes!("../../assets/photos/dune.jpg"),
        PhotoKind::Harbor => include_bytes!("../../assets/photos/harbor.jpg"),
        PhotoKind::Peak => include_bytes!("../../assets/photos/peak.jpg"),
        PhotoKind::Cove => include_bytes!("../../assets/photos/cove.jpg"),
        PhotoKind::PortraitSofia => include_bytes!("../../assets/photos/sofia.jpg"),
        PhotoKind::PortraitCarmen => include_bytes!("../../assets/photos/carmen.jpg"),
        PhotoKind::PortraitAlejandro => include_bytes!("../../assets/photos/alejandro.jpg"),
        PhotoKind::PortraitOli => include_bytes!("../../assets/photos/oli.jpg"),
        PhotoKind::PortraitAna => include_bytes!("../../assets/photos/ana.jpg"),
        PhotoKind::PortraitMarty => include_bytes!("../../assets/photos/marty.jpg"),
        PhotoKind::PortraitRenee => include_bytes!("../../assets/photos/renee.jpg"),
    }
}

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < data.len() {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}
