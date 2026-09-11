//! Progress indicators. Specs: https://m3.material.io/components/progress-indicators/specs

use crate::argb::Argb;
use crate::theme::Theme;

pub const LINEAR_HEIGHT_DP: f32 = 4.0;
pub const CIRCULAR_SIZE_DP: f32 = 48.0;
pub const CIRCULAR_STROKE_DP: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearProgress {
    pub height_dp: f32,
    pub track: Argb,
    pub indicator: Argb,
    pub progress: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircularProgress {
    pub size_dp: f32,
    pub stroke_dp: f32,
    pub track: Argb,
    pub indicator: Argb,
    pub progress: f32,
}

pub fn linear(theme: &Theme, progress: f32) -> LinearProgress {
    LinearProgress {
        height_dp: LINEAR_HEIGHT_DP,
        track: theme.color.secondary_container,
        indicator: theme.color.primary,
        progress: progress.clamp(0.0, 1.0),
    }
}

pub fn circular(theme: &Theme, progress: f32) -> CircularProgress {
    CircularProgress {
        size_dp: CIRCULAR_SIZE_DP,
        stroke_dp: CIRCULAR_STROKE_DP,
        track: theme.color.secondary_container,
        indicator: theme.color.primary,
        progress: progress.clamp(0.0, 1.0),
    }
}
