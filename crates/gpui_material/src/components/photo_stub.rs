//! Decoded JPEG catalog media for scene chrome.
//!
//! Official m3.material.io overviews use camera photos. Heroes encode a tiny
//! procedural JPEG per [`PhotoKind`], then **decode** it for HTML `data:` URIs
//! and GPUI mosaic samples — not CSS gradients and not a live camera roll.

use crate::argb::Argb;
use std::sync::OnceLock;

/// Named camera-style photo used by snackbar, tabs, sheets, carousel, and
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

    /// Dominant fill (seed + GPUI fallback when a host skips the mosaic).
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

    /// Encoded JPEG (SOI `FF D8`). Generated once per kind.
    pub fn jpeg_bytes(self) -> &'static [u8] {
        &jpegs()[self.index()]
    }

    /// `data:image/jpeg;base64,…` for catalog `<img>` / CSS `url()`.
    pub fn data_uri(self) -> String {
        format!("data:image/jpeg;base64,{}", base64_encode(self.jpeg_bytes()))
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

fn jpegs() -> &'static [Vec<u8>] {
    static CACHE: OnceLock<Vec<Vec<u8>>> = OnceLock::new();
    CACHE.get_or_init(|| PhotoKind::ALL.iter().map(|k| encode_jpeg(*k)).collect())
}

fn encode_jpeg(kind: PhotoKind) -> Vec<u8> {
    let (w, h) = if kind.is_portrait() {
        (96, 96)
    } else if kind == PhotoKind::Basket || kind == PhotoKind::Mugs || kind == PhotoKind::Dog {
        (160, 160)
    } else {
        (160, 120)
    };
    let rgb = rasterize(kind, w, h);
    let mut out = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 78);
    encoder
        .encode(&rgb, w, h, image::ExtendedColorType::Rgb8)
        .expect("jpeg encode");
    out
}

fn rasterize(kind: PhotoKind, w: u32, h: u32) -> Vec<u8> {
    let mut pix = Pix::new(w, h, 17 + kind.index() as u32 * 97);
    match kind {
        PhotoKind::Bloom => paint_bloom(&mut pix),
        PhotoKind::Egret => paint_egret(&mut pix),
        PhotoKind::Basket => paint_basket(&mut pix),
        PhotoKind::Mugs => paint_mugs(&mut pix),
        PhotoKind::Dog => paint_dog(&mut pix),
        PhotoKind::Party => paint_party(&mut pix),
        PhotoKind::Lake => paint_landscape(&mut pix, 0.55, 0x81, 0xD4, 0xFA, 0x02, 0x77, 0xBD),
        PhotoKind::Grove => paint_landscape(&mut pix, 0.42, 0xA5, 0xD6, 0xA7, 0x1B, 0x5E, 0x20),
        PhotoKind::Dune => paint_landscape(&mut pix, 0.58, 0xFF, 0xE0, 0xB2, 0xEF, 0x6C, 0x00),
        PhotoKind::Harbor => paint_landscape(&mut pix, 0.50, 0x80, 0xCB, 0xC4, 0x00, 0x69, 0x5C),
        PhotoKind::Peak => paint_landscape(&mut pix, 0.38, 0xB0, 0xBE, 0xC5, 0x37, 0x47, 0x4F),
        PhotoKind::Cove => paint_landscape(&mut pix, 0.52, 0xB2, 0xEB, 0xF2, 0x00, 0x83, 0x8F),
        PhotoKind::PortraitSofia => {
            paint_portrait(&mut pix, 0xF3, 0xD5, 0xC0, 0x5D, 0x40, 0x37, 0x8D, 0x6E, 0x63)
        }
        PhotoKind::PortraitCarmen => {
            paint_portrait(&mut pix, 0xE0, 0xBE, 0xA8, 0x3E, 0x27, 0x23, 0x6D, 0x4C, 0x41)
        }
        PhotoKind::PortraitAlejandro => {
            paint_portrait(&mut pix, 0xD7, 0xCC, 0xC8, 0x3E, 0x27, 0x23, 0x5D, 0x40, 0x37)
        }
        PhotoKind::PortraitOli => {
            paint_portrait(&mut pix, 0xFF, 0xE0, 0xB2, 0x4E, 0x34, 0x2E, 0xA1, 0x88, 0x7F)
        }
        PhotoKind::PortraitAna => {
            paint_portrait(&mut pix, 0xF8, 0xBB, 0xD0, 0x6D, 0x4C, 0x41, 0xBC, 0xAA, 0xA4)
        }
        PhotoKind::PortraitMarty => {
            paint_portrait(&mut pix, 0xFF, 0xCC, 0x80, 0x3E, 0x27, 0x23, 0x79, 0x55, 0x48)
        }
        PhotoKind::PortraitRenee => {
            paint_portrait(&mut pix, 0xFF, 0xE0, 0xB2, 0xBF, 0x36, 0x0C, 0xFF, 0xCC, 0x80)
        }
    }
    pix.grain(0.06);
    pix.rgb
}

struct Pix {
    w: u32,
    h: u32,
    rgb: Vec<u8>,
    seed: u32,
}

impl Pix {
    fn new(w: u32, h: u32, seed: u32) -> Self {
        Self {
            w,
            h,
            rgb: vec![0; (w * h * 3) as usize],
            seed,
        }
    }

    fn put(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return;
        }
        let i = ((y as u32 * self.w + x as u32) * 3) as usize;
        self.rgb[i] = r;
        self.rgb[i + 1] = g;
        self.rgb[i + 2] = b;
    }

    fn mix(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8, a: f32) {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return;
        }
        let a = a.clamp(0.0, 1.0);
        let i = ((y as u32 * self.w + x as u32) * 3) as usize;
        self.rgb[i] = mix_u8(self.rgb[i], r, a);
        self.rgb[i + 1] = mix_u8(self.rgb[i + 1], g, a);
        self.rgb[i + 2] = mix_u8(self.rgb[i + 2], b, a);
    }

    fn uv(&self, x: i32, y: i32) -> (f32, f32) {
        (x as f32 / self.w as f32, y as f32 / self.h as f32)
    }

    fn noise(&self, x: f32, y: f32, seed: u32) -> f32 {
        fbm(x, y, self.seed.wrapping_add(seed))
    }

    fn grain(&mut self, amount: f32) {
        for y in 0..self.h as i32 {
            for x in 0..self.w as i32 {
                let n = hash2(x, y, self.seed.wrapping_add(9)) * 2.0 - 1.0;
                let d = (n * amount * 40.0) as i32;
                let i = ((y as u32 * self.w + x as u32) * 3) as usize;
                for c in 0..3 {
                    self.rgb[i + c] = (self.rgb[i + c] as i32 + d).clamp(0, 255) as u8;
                }
            }
        }
    }
}

fn mix_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn hash2(x: i32, y: i32, seed: u32) -> f32 {
    let mut n = x.wrapping_mul(374_761_393)
        .wrapping_add(y.wrapping_mul(668_265_263))
        .wrapping_add(seed as i32);
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    ((n ^ (n >> 16)) as u32 as f32) / (u32::MAX as f32)
}

fn noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);
    lerp(
        lerp(hash2(x0, y0, seed), hash2(x0 + 1, y0, seed), sx),
        lerp(hash2(x0, y0 + 1, seed), hash2(x0 + 1, y0 + 1, seed), sx),
        sy,
    )
}

fn fbm(x: f32, y: f32, seed: u32) -> f32 {
    let mut a = 0.0;
    let mut amp = 0.5;
    let mut f = 1.0;
    for o in 0..5 {
        a += amp * noise(x * f, y * f, seed.wrapping_add(o * 19));
        amp *= 0.5;
        f *= 2.05;
    }
    a
}

fn paint_landscape(pix: &mut Pix, horizon: f32, sr: u8, sg: u8, sb: u8, gr: u8, gg: u8, gb: u8) {
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let n = pix.noise(u * 4.0, v * 4.0, 1);
            let ridge = horizon + (n - 0.5) * 0.12;
            if v < ridge {
                let t = (v / ridge).clamp(0.0, 1.0);
                let r = mix_u8(sr, 0xFF, 0.25 + t * 0.2);
                let g = mix_u8(sg, 0xFB, 0.15);
                let b = mix_u8(sb, 0xFF, 0.1);
                let cloud = pix.noise(u * 3.0, v * 6.0, 4);
                pix.put(
                    x,
                    y,
                    mix_u8(r, 255, cloud * 0.18),
                    mix_u8(g, 255, cloud * 0.18),
                    mix_u8(b, 255, cloud * 0.22),
                );
            } else {
                let t = ((v - ridge) / (1.0 - ridge)).clamp(0.0, 1.0);
                let grain = pix.noise(u * 8.0, v * 8.0, 7);
                pix.put(
                    x,
                    y,
                    mix_u8(gr, 20, t * 0.45 + grain * 0.1),
                    mix_u8(gg, 30, t * 0.4 + grain * 0.08),
                    mix_u8(gb, 24, t * 0.35),
                );
            }
        }
    }
}

fn paint_bloom(pix: &mut Pix) {
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let n = pix.noise(u * 5.0, v * 5.0, 2);
            let r = mix_u8(0xE1, 0xF8, v * 0.4 + n * 0.3);
            let g = mix_u8(0xBE, 0xC8, v * 0.5);
            let b = mix_u8(0xE7, 0xD0, (1.0 - v) * 0.3);
            pix.put(x, y, r, mix_u8(g, 0x43, v * 0.55), mix_u8(b, 0xA0, v * 0.2));
        }
    }
    for i in 0..18 {
        let cx = 0.15 + hash2(i, 3, pix.seed) * 0.7;
        let cy = 0.12 + hash2(i, 5, pix.seed) * 0.55;
        let rad = 0.04 + hash2(i, 8, pix.seed) * 0.09;
        blob(
            pix,
            cx,
            cy,
            rad,
            mix_u8(0xF8, 0xCE, hash2(i, 1, pix.seed)),
            mix_u8(0xBB, 0x93, hash2(i, 2, pix.seed)),
            mix_u8(0xD0, 0xD8, hash2(i, 4, pix.seed)),
            0.55,
        );
    }
}

fn paint_egret(pix: &mut Pix) {
    paint_landscape(pix, 0.62, 0x90, 0xCA, 0xF9, 0x37, 0x47, 0x4F);
    blob(pix, 0.68, 0.58, 0.10, 0xEC, 0xEF, 0xF1, 0.95);
    blob(pix, 0.72, 0.48, 0.05, 0xFA, 0xFA, 0xFA, 0.95);
    blob(pix, 0.64, 0.50, 0.035, 0xFF, 0xFF, 0xFF, 0.9);
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let dx = u - 0.68;
            let dy = v - 0.42;
            if dx * dx * 6.0 + (dy + 0.08).abs() * 0.04 < 0.002 && v < 0.62 {
                pix.mix(x, y, 0xFA, 0xFA, 0xFA, 0.85);
            }
        }
    }
}

fn paint_basket(pix: &mut Pix) {
    let cx = pix.w as f32 * 0.5;
    let cy = pix.h as f32 * 0.46;
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let dx = x as f32 - cx;
            let dy = (y as f32 - cy) * 0.92;
            let r = (dx * dx + dy * dy).sqrt() / pix.w as f32;
            let ang = dy.atan2(dx);
            let weave = ((r * 28.0).sin() * 0.5 + (ang * 14.0).sin() * 0.5) * 0.5 + 0.5;
            let n = pix.noise(x as f32 * 0.08, y as f32 * 0.08, 3);
            let t = (r * 1.6).clamp(0.0, 1.0);
            let wood = mix_u8(0xD7, 0x6F, t * 0.7 + weave * 0.2 + n * 0.1);
            let g = mix_u8(0xB8, 0x4E, t * 0.65 + weave * 0.15);
            let b = mix_u8(0x99, 0x37, t * 0.6);
            pix.put(x, y, wood, g, b);
        }
    }
}

fn paint_mugs(pix: &mut Pix) {
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let n = pix.noise(u * 3.0, v * 3.0, 2);
            pix.put(
                x,
                y,
                mix_u8(0xEE, 0xE0, v * 0.4 + n * 0.1),
                mix_u8(0xF3, 0xD6, v * 0.35),
                mix_u8(0xF8, 0xC8, v * 0.3),
            );
        }
    }
    let colors: [(u8, u8, u8); 5] = [
        (0x1A, 0x23, 0x7E),
        (0x21, 0x21, 0x21),
        (0xFA, 0xFA, 0xFA),
        (0x1B, 0x5E, 0x20),
        (0xC6, 0x28, 0x28),
    ];
    for (i, (cr, cg, cb)) in colors.iter().enumerate() {
        let cx = 0.14 + i as f32 * 0.18;
        mug(pix, cx, 0.52, *cr, *cg, *cb);
    }
}

fn mug(pix: &mut Pix, cx: f32, cy: f32, r: u8, g: u8, b: u8) {
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let dx = (u - cx) * pix.w as f32 / pix.h as f32;
            let dy = v - cy;
            if dx.abs() < 0.07 && dy > -0.22 && dy < 0.18 {
                let shade = 0.75 + (0.07 - dx.abs()) / 0.07 * 0.25;
                pix.put(
                    x,
                    y,
                    (r as f32 * shade) as u8,
                    (g as f32 * shade) as u8,
                    (b as f32 * shade) as u8,
                );
            }
            if (dx - 0.075).abs() < 0.025 && dy.abs() < 0.08 && dy < 0.05 {
                pix.mix(x, y, r, g, b, 0.9);
            }
        }
    }
}

fn paint_dog(pix: &mut Pix) {
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let n = pix.noise(u * 6.0, v * 6.0, 4);
            pix.put(
                x,
                y,
                mix_u8(0x3E, 0x6D, n),
                mix_u8(0x27, 0x4C, n),
                mix_u8(0x23, 0x41, n),
            );
        }
    }
    blob(pix, 0.50, 0.46, 0.28, 0x6D, 0x4C, 0x41, 0.95);
    blob(pix, 0.32, 0.28, 0.12, 0x4E, 0x34, 0x2E, 0.95);
    blob(pix, 0.70, 0.30, 0.11, 0x4E, 0x34, 0x2E, 0.95);
    blob(pix, 0.42, 0.44, 0.05, 0x21, 0x21, 0x21, 0.9);
    blob(pix, 0.58, 0.44, 0.05, 0x21, 0x21, 0x21, 0.9);
    blob(pix, 0.43, 0.43, 0.015, 0xFF, 0xF8, 0xE1, 0.9);
    blob(pix, 0.59, 0.43, 0.015, 0xFF, 0xF8, 0xE1, 0.9);
    blob(pix, 0.50, 0.58, 0.06, 0x21, 0x21, 0x21, 0.7);
}

fn paint_party(pix: &mut Pix) {
    paint_landscape(pix, 0.72, 0x81, 0xC7, 0x84, 0x2E, 0x7D, 0x32);
    for i in 0..5 {
        let cx = 0.12 + i as f32 * 0.18;
        blob(
            pix,
            cx,
            0.62,
            0.10,
            mix_u8(0x6D, 0xFF, hash2(i, 1, pix.seed)),
            mix_u8(0x4C, 0xCC, hash2(i, 2, pix.seed)),
            mix_u8(0x41, 0x80, hash2(i, 3, pix.seed)),
            0.92,
        );
        blob(pix, cx, 0.48, 0.055, 0xF3, 0xD5, 0xC0, 0.95);
    }
    for x in 0..pix.w as i32 {
        let u = x as f32 / pix.w as f32;
        let y = (0.18 + (u * 12.0).sin() * 0.03) * pix.h as f32;
        pix.put(x, y as i32, 0xFF, 0xF5, 0x9D);
        if x % 18 < 9 {
            blob(
                pix,
                u,
                0.22,
                0.03,
                0xE9,
                0x1E,
                0x63,
                0.8,
            );
        }
    }
}

fn paint_portrait(
    pix: &mut Pix,
    skin_r: u8,
    skin_g: u8,
    skin_b: u8,
    hair_r: u8,
    hair_g: u8,
    hair_b: u8,
    shirt_r: u8,
    shirt_g: u8,
    shirt_b: u8,
) {
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let n = pix.noise(u * 4.0, v * 4.0, 8);
            pix.put(
                x,
                y,
                mix_u8(shirt_r, 0x90, n * 0.25 + v * 0.2),
                mix_u8(shirt_g, 0x80, n * 0.2),
                mix_u8(shirt_b, 0x70, n * 0.15),
            );
        }
    }
    blob(pix, 0.50, 0.78, 0.42, shirt_r, shirt_g, shirt_b, 1.0);
    blob(pix, 0.50, 0.22, 0.28, hair_r, hair_g, hair_b, 1.0);
    blob(pix, 0.50, 0.40, 0.22, skin_r, skin_g, skin_b, 1.0);
    blob(pix, 0.38, 0.38, 0.035, 0x3E, 0x27, 0x23, 0.9);
    blob(pix, 0.62, 0.38, 0.035, 0x3E, 0x27, 0x23, 0.9);
    blob(pix, 0.50, 0.50, 0.04, mix_u8(skin_r, 180, 0.3), mix_u8(skin_g, 80, 0.2), mix_u8(skin_b, 80, 0.2), 0.55);
}

fn blob(pix: &mut Pix, cx: f32, cy: f32, rad: f32, r: u8, g: u8, b: u8, a: f32) {
    for y in 0..pix.h as i32 {
        for x in 0..pix.w as i32 {
            let (u, v) = pix.uv(x, y);
            let dx = (u - cx) * pix.w as f32 / pix.h as f32;
            let dy = v - cy;
            let d = (dx * dx + dy * dy).sqrt();
            if d < rad {
                let edge = (1.0 - d / rad).clamp(0.0, 1.0);
                pix.mix(x, y, r, g, b, a * edge.powf(0.55));
            }
        }
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
