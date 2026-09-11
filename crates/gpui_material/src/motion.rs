//! Material 3 motion tokens (duration + easing).
//! https://m3.material.io/styles/motion/easing-and-duration/tokens-specs

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
        }
    }
}
