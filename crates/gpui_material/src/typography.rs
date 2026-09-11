//! Material 3 type scale (baseline, Roboto).
//!
//! Values match androidx `TypeScaleTokens` (v0_103):
//! https://m3.material.io/styles/typography/type-scale-tokens

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeStyle {
    pub name: &'static str,
    pub size_sp: f32,
    pub line_height_sp: f32,
    pub tracking_sp: f32,
    pub weight: u16,
}

impl TypeStyle {
    pub const fn css_weight(self) -> u16 {
        self.weight
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeScale {
    pub display_large: TypeStyle,
    pub display_medium: TypeStyle,
    pub display_small: TypeStyle,
    pub headline_large: TypeStyle,
    pub headline_medium: TypeStyle,
    pub headline_small: TypeStyle,
    pub title_large: TypeStyle,
    pub title_medium: TypeStyle,
    pub title_small: TypeStyle,
    pub body_large: TypeStyle,
    pub body_medium: TypeStyle,
    pub body_small: TypeStyle,
    pub label_large: TypeStyle,
    pub label_medium: TypeStyle,
    pub label_small: TypeStyle,
}

impl TypeScale {
    pub const fn baseline() -> Self {
        const fn style(
            name: &'static str,
            size_sp: f32,
            line_height_sp: f32,
            tracking_sp: f32,
            weight: u16,
        ) -> TypeStyle {
            TypeStyle {
                name,
                size_sp,
                line_height_sp,
                tracking_sp,
                weight,
            }
        }
        Self {
            display_large: style("displayLarge", 57.0, 64.0, -0.2, 400),
            display_medium: style("displayMedium", 45.0, 52.0, 0.0, 400),
            display_small: style("displaySmall", 36.0, 44.0, 0.0, 400),
            headline_large: style("headlineLarge", 32.0, 40.0, 0.0, 400),
            headline_medium: style("headlineMedium", 28.0, 36.0, 0.0, 400),
            headline_small: style("headlineSmall", 24.0, 32.0, 0.0, 400),
            title_large: style("titleLarge", 22.0, 28.0, 0.0, 400),
            title_medium: style("titleMedium", 16.0, 24.0, 0.2, 500),
            title_small: style("titleSmall", 14.0, 20.0, 0.1, 500),
            body_large: style("bodyLarge", 16.0, 24.0, 0.5, 400),
            body_medium: style("bodyMedium", 14.0, 20.0, 0.2, 400),
            body_small: style("bodySmall", 12.0, 16.0, 0.4, 400),
            label_large: style("labelLarge", 14.0, 20.0, 0.1, 500),
            label_medium: style("labelMedium", 12.0, 16.0, 0.5, 500),
            label_small: style("labelSmall", 11.0, 16.0, 0.5, 500),
        }
    }

    pub fn all(self) -> [TypeStyle; 15] {
        [
            self.display_large,
            self.display_medium,
            self.display_small,
            self.headline_large,
            self.headline_medium,
            self.headline_small,
            self.title_large,
            self.title_medium,
            self.title_small,
            self.body_large,
            self.body_medium,
            self.body_small,
            self.label_large,
            self.label_medium,
            self.label_small,
        ]
    }
}

pub const FONT_FAMILY: &str = "Roboto";
