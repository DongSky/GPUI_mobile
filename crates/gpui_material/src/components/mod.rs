pub mod badge;
pub mod bottom_sheet;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod date_picker;
pub mod dialog;
pub mod divider;
pub mod fab;
pub mod icon_button;
pub mod list;
pub mod menu;
pub mod navigation_bar;
pub mod progress;
pub mod radio;
pub mod slider;
pub mod snackbar;
pub mod switch;
pub mod tabs;
pub mod text_field;
pub mod top_app_bar;

use crate::argb::Argb;
use crate::shape::Corners;
use crate::typography::TypeStyle;

/// Flattened paint recipe used by both the HTML catalog and the GPUI demo.
#[derive(Clone, Debug, PartialEq)]
pub struct Appearance {
    pub width_dp: Option<f32>,
    pub height_dp: f32,
    pub min_width_dp: Option<f32>,
    pub corners: Corners,
    pub container: Argb,
    pub content: Argb,
    pub secondary_content: Option<Argb>,
    pub outline: Option<(Argb, f32)>,
    pub elevation_dp: f32,
    pub pad_start_dp: f32,
    pub pad_end_dp: f32,
    pub pad_top_dp: f32,
    pub pad_bottom_dp: f32,
    pub label_style: TypeStyle,
    pub supporting_style: Option<TypeStyle>,
}

impl Appearance {
    pub fn outline_css(&self) -> String {
        match self.outline {
            Some((color, width)) => format!("{width:.0}px solid {}", color.css_hex()),
            None => "none".into(),
        }
    }
}
