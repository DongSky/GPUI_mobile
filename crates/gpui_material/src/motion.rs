//! Material 3 motion tokens (duration + easing).
//! https://m3.material.io/styles/motion/easing-and-duration/tokens-specs
//!
//! Hosts without a GPUI animation clock can still evaluate the same curves
//! (`emphasized_at`, `lerp`) and reuse the CSS strings in catalogs.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    pub short1_ms: u16,
    pub short2_ms: u16,
    pub short3_ms: u16,
    pub short4_ms: u16,
    pub medium1_ms: u16,
    pub medium2_ms: u16,
    pub medium3_ms: u16,
    pub medium4_ms: u16,
    pub long1_ms: u16,
    pub long2_ms: u16,
    pub long3_ms: u16,
    pub long4_ms: u16,
    pub emphasized: &'static str,
    pub emphasized_decelerate: &'static str,
    pub emphasized_accelerate: &'static str,
    pub standard: &'static str,
    pub standard_decelerate: &'static str,
    pub standard_accelerate: &'static str,
    /// Expressive spatial spring (shape / size morph). m3.material.io motion physics.
    pub spatial_fast: &'static str,
    pub spatial_fast_ms: u16,
    pub effects_fast: &'static str,
    pub effects_fast_ms: u16,
    pub effects_default: &'static str,
    pub effects_default_ms: u16,
}

impl MotionTokens {
    pub const fn baseline() -> Self {
        Self {
            short1_ms: 50,
            short2_ms: 100,
            short3_ms: 150,
            short4_ms: 200,
            medium1_ms: 250,
            medium2_ms: 300,
            medium3_ms: 350,
            medium4_ms: 400,
            long1_ms: 450,
            long2_ms: 500,
            long3_ms: 550,
            long4_ms: 600,
            emphasized: "cubic-bezier(0.2, 0.0, 0.0, 1.0)",
            emphasized_decelerate: "cubic-bezier(0.05, 0.7, 0.1, 1.0)",
            emphasized_accelerate: "cubic-bezier(0.3, 0.0, 0.8, 0.15)",
            standard: "cubic-bezier(0.2, 0.0, 0.0, 1.0)",
            standard_decelerate: "cubic-bezier(0.0, 0.0, 0.0, 1.0)",
            standard_accelerate: "cubic-bezier(0.3, 0.0, 1.0, 1.0)",
            spatial_fast: "cubic-bezier(0.42, 1.67, 0.21, 0.90)",
            spatial_fast_ms: 350,
            effects_fast: "cubic-bezier(0.31, 0.94, 0.34, 1.00)",
            effects_fast_ms: 150,
            effects_default: "cubic-bezier(0.34, 0.80, 0.34, 1.00)",
            effects_default_ms: 200,
        }
    }

    /// M3 emphasized easing at `t` in `[0, 1]`.
    pub fn emphasized_at(self, t: f32) -> f32 {
        cubic_bezier(0.2, 0.0, 0.0, 1.0, t)
    }

    pub fn emphasized_decelerate_at(self, t: f32) -> f32 {
        cubic_bezier(0.05, 0.7, 0.1, 1.0, t)
    }

    /// Expressive spatial-fast spring (may overshoot). Used by search grow.
    pub fn spatial_fast_at(self, t: f32) -> f32 {
        cubic_bezier(0.42, 1.67, 0.21, 0.90, t).clamp(0.0, 1.2)
    }

    pub fn lerp(self, from: f32, to: f32, t: f32) -> f32 {
        from + (to - from) * self.emphasized_at(t)
    }

    /// CSS `transition` for Expressive state + shape morph.
    pub fn css_state_transition(self) -> String {
        format!(
            "background-color {ed}ms {ee}, color {ed}ms {ee}, border-color {ed}ms {ee}, box-shadow {ed}ms {ee}, opacity {ed}ms {ee}, border-radius {sd}ms {se}, transform {sd}ms {se}, width {sd}ms {se}, left {sd}ms {se}",
            ed = self.effects_default_ms,
            ee = self.effects_default,
            sd = self.spatial_fast_ms,
            se = self.spatial_fast,
        )
    }
}

/// Evaluate CSS `cubic-bezier(x1,y1,x2,y2)` at time `t` in `[0, 1]`.
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    let mut s = t;
    for _ in 0..10 {
        let x = bezier_coord(s, x1, x2);
        let dx = bezier_deriv(s, x1, x2);
        if dx.abs() < 1e-6 {
            break;
        }
        s = (s - (x - t) / dx).clamp(0.0, 1.0);
    }
    bezier_coord(s, y1, y2)
}

fn bezier_coord(t: f32, p1: f32, p2: f32) -> f32 {
    let u = 1.0 - t;
    3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t
}

fn bezier_deriv(t: f32, p1: f32, p2: f32) -> f32 {
    let u = 1.0 - t;
    3.0 * u * u * p1 + 6.0 * u * t * (p2 - p1) + 3.0 * t * t * (1.0 - p2)
}
