use crate::color::ColorScheme;
use crate::elevation::ElevationLevels;
use crate::motion::MotionTokens;
use crate::shape::Shapes;
use crate::typography::TypeScale;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub dark: bool,
    pub color: ColorScheme,
    pub typography: TypeScale,
    pub shapes: Shapes,
    pub elevation: ElevationLevels,
    pub motion: MotionTokens,
}

impl Theme {
    pub const fn light() -> Self {
        Self {
            dark: false,
            color: ColorScheme::light(),
            typography: TypeScale::baseline(),
            shapes: Shapes::baseline(),
            elevation: ElevationLevels::baseline(),
            motion: MotionTokens::baseline(),
        }
    }

    pub const fn dark() -> Self {
        Self {
            dark: true,
            color: ColorScheme::dark(),
            typography: TypeScale::baseline(),
            shapes: Shapes::baseline(),
            elevation: ElevationLevels::baseline(),
            motion: MotionTokens::baseline(),
        }
    }
}
