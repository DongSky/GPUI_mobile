//! Material 3 shape scale (dp corner radii).
//! https://m3.material.io/styles/shape/shape-scale-tokens

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shapes {
    pub none: f32,
    pub extra_small: f32,
    pub small: f32,
    pub medium: f32,
    pub large: f32,
    pub large_increased: f32,
    pub extra_large: f32,
    pub extra_large_increased: f32,
    pub extra_extra_large: f32,
    pub full: f32,
}

impl Shapes {
    /// Current M3 Expressive shape scale (m3.material.io / shape-scale-tokens).
    pub const fn baseline() -> Self {
        Self {
            none: 0.0,
            extra_small: 4.0,
            small: 8.0,
            medium: 12.0,
            large: 16.0,
            large_increased: 20.0,
            extra_large: 28.0,
            extra_large_increased: 32.0,
            extra_extra_large: 48.0,
            full: 999.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corners {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl Corners {
    pub const fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    pub const fn extra_small_top(extra_small: f32) -> Self {
        Self {
            top_left: extra_small,
            top_right: extra_small,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }
    }

    pub fn css(self) -> String {
        format!(
            "{:.0}px {:.0}px {:.0}px {:.0}px",
            self.top_left, self.top_right, self.bottom_right, self.bottom_left
        )
    }
}
