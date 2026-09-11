//! 8-bit ARGB color with alpha compositing. Tokens store sRGB channel values
//! from androidx `PaletteTokens` (Material 3 token set v0_210).

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Argb(pub u32);

impl Argb {
    pub const TRANSPARENT: Self = Self(0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(255, r, g, b)
    }

    pub const fn rgba(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub const fn from_rgb_hex(hex: u32) -> Self {
        Self::rgb(((hex >> 16) & 0xff) as u8, ((hex >> 8) & 0xff) as u8, (hex & 0xff) as u8)
    }

    pub const fn a(self) -> u8 {
        (self.0 >> 24) as u8
    }
    pub const fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub const fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub const fn b(self) -> u8 {
        self.0 as u8
    }

    pub fn with_alpha(self, alpha: f32) -> Self {
        let a = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
        Self::rgba(a, self.r(), self.g(), self.b())
    }

    /// Source-over composite of `self` onto `backdrop`.
    pub fn composite_over(self, backdrop: Self) -> Self {
        let fa = self.a() as f32 / 255.0;
        if fa <= 0.0 {
            return backdrop;
        }
        if fa >= 1.0 {
            return Self::rgb(self.r(), self.g(), self.b());
        }
        let inv = 1.0 - fa;
        let mix = |fg: u8, bg: u8| (fg as f32 * fa + bg as f32 * inv).round() as u8;
        Self::rgb(
            mix(self.r(), backdrop.r()),
            mix(self.g(), backdrop.g()),
            mix(self.b(), backdrop.b()),
        )
    }

    pub fn css_hex(self) -> String {
        if self.a() == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r(), self.g(), self.b())
        } else {
            format!(
                "rgba({}, {}, {}, {:.3})",
                self.r(),
                self.g(),
                self.b(),
                self.a() as f32 / 255.0
            )
        }
    }

    /// 0x00RRGGBB for GPUI `rgb()` when opaque.
    pub const fn rgb_u32(self) -> u32 {
        self.0 & 0x00ff_ffff
    }
}
