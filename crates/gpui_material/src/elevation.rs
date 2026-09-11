//! Material 3 elevation levels (dp) and a simple umbra shadow for catalog/HTML.
//! https://m3.material.io/styles/elevation

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElevationLevels {
    pub level0: f32,
    pub level1: f32,
    pub level2: f32,
    pub level3: f32,
    pub level4: f32,
    pub level5: f32,
}

impl ElevationLevels {
    pub const fn baseline() -> Self {
        Self {
            level0: 0.0,
            level1: 1.0,
            level2: 3.0,
            level3: 6.0,
            level4: 8.0,
            level5: 12.0,
        }
    }

    pub fn css_shadow(dp: f32) -> String {
        if dp <= 0.0 {
            return "none".into();
        }
        // Approximate M3 key+ambient umbra for catalog QA.
        let y = (dp * 0.5).max(1.0);
        let blur = dp * 1.5;
        format!("0 {y:.1}px {blur:.1}px rgba(0, 0, 0, 0.20)")
    }
}
