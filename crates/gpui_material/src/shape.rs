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

    /// LTR start-edge corners (modal side sheet `corner-large.start`).
    pub const fn start(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: radius,
        }
    }

    pub fn css(self) -> String {
        format!(
            "{:.0}px {:.0}px {:.0}px {:.0}px",
            self.top_left, self.top_right, self.bottom_right, self.bottom_left
        )
    }
}

/// Material `RoundedPolygon` / `CornerRounding.smoothing` (0 = circular, 1 = fully smooth).
/// androidx.graphics.shapes uses this to consume extra edge and flatten the bulge.
pub const CORNER_SMOOTHING: f32 = 0.6;
/// κ for a circular quarter-cubic (≈ 4/3 tan(π/8)).
pub const CIRCULAR_KAPPA: f32 = 0.55228475;

/// Extra edge consumed beyond `radius` when smoothing > 0 (`√2 − 1` scale).
pub fn smooth_cut(radius: f32, smoothing: f32) -> f32 {
    let s = smoothing.clamp(0.0, 1.0);
    radius * (1.0 + 0.29289322 * s)
}

/// Cubic κ: circular at 0, flatter superellipse as smoothing rises.
pub fn smooth_kappa(smoothing: f32) -> f32 {
    let s = smoothing.clamp(0.0, 1.0);
    CIRCULAR_KAPPA + (0.85 - CIRCULAR_KAPPA) * s
}

/// Clockwise quarter-cubic from `from` → `to` around a corner at `corner`.
/// Matches androidx `RoundedPolygon` + `CornerRounding(radius, smoothing)`.
/// Returns `[c1, c2, to]` control points (start is `from`).
pub fn rounded_polygon_quarter(
    from: (f32, f32),
    corner: (f32, f32),
    to: (f32, f32),
    kappa: f32,
) -> [(f32, f32); 3] {
    let c1 = (
        from.0 + (corner.0 - from.0) * kappa,
        from.1 + (corner.1 - from.1) * kappa,
    );
    let c2 = (
        to.0 + (corner.0 - to.0) * kappa,
        to.1 + (corner.1 - to.1) * kappa,
    );
    [c1, c2, to]
}

/// Sample a cubic Bézier `p0..p3` (excludes `p0`, includes `p3`).
pub fn sample_cubic(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    n: usize,
) -> Vec<(f32, f32)> {
    let n = n.max(1);
    (1..=n)
        .map(|i| {
            let t = i as f32 / n as f32;
            let u = 1.0 - t;
            let uu = u * u;
            let tt = t * t;
            let a = uu * u;
            let b = 3.0 * uu * t;
            let c = 3.0 * u * tt;
            let d = tt * t;
            (
                a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0,
                a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1,
            )
        })
        .collect()
}

/// Flatten a RoundedPolygon quarter (excludes `from`, includes `to`).
pub fn flatten_rounded_quarter(
    from: (f32, f32),
    corner: (f32, f32),
    to: (f32, f32),
    kappa: f32,
    n: usize,
) -> Vec<(f32, f32)> {
    let [c1, c2, end] = rounded_polygon_quarter(from, corner, to, kappa);
    sample_cubic(from, c1, c2, end, n)
}
