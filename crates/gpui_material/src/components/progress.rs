//! Progress indicators. Specs: https://m3.material.io/components/progress-indicators/specs
//!
//! Determinate linear/circular plus indeterminate / pull-to-refresh tokens.
//! HTML catalog animates with motion springs; GPUI uses `Animation` as a
//! repeating clock (wavy, sliding head, spinning PTR arc).

use crate::argb::Argb;
use crate::theme::Theme;

pub const LINEAR_HEIGHT_DP: f32 = 4.0;
pub const CIRCULAR_SIZE_DP: f32 = 48.0;
pub const CIRCULAR_STROKE_DP: f32 = 4.0;
/// Sliding head width for the indeterminate linear indicator (fraction of track).
pub const INDETERMINATE_SPAN: f32 = 0.35;
pub const PTR_SIZE_DP: f32 = 40.0;
pub const PTR_STROKE_DP: f32 = 4.0;
pub const PTR_LABEL: &str = "Pull to refresh";

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndeterminateLinear {
    pub height_dp: f32,
    pub track: Argb,
    pub indicator: Argb,
    pub head_span: f32,
    pub duration_ms: u16,
    pub easing: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndeterminateCircular {
    pub size_dp: f32,
    pub stroke_dp: f32,
    pub track: Argb,
    pub indicator: Argb,
    pub arc_deg: f32,
    pub duration_ms: u16,
    pub easing: &'static str,
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

/// Shared repeating-clock duration for wavy / indeterminate / PTR / loading.
pub fn clock_ms(theme: &Theme) -> u16 {
    theme.motion.effects_default_ms.saturating_mul(6)
}

pub fn linear_indeterminate(theme: &Theme) -> IndeterminateLinear {
    IndeterminateLinear {
        height_dp: LINEAR_HEIGHT_DP,
        track: theme.color.secondary_container,
        indicator: theme.color.primary,
        head_span: INDETERMINATE_SPAN,
        duration_ms: clock_ms(theme),
        easing: theme.motion.effects_default,
    }
}

pub fn circular_indeterminate(theme: &Theme) -> IndeterminateCircular {
    IndeterminateCircular {
        size_dp: CIRCULAR_SIZE_DP,
        stroke_dp: CIRCULAR_STROKE_DP,
        track: theme.color.secondary_container,
        indicator: theme.color.primary,
        arc_deg: 90.0,
        duration_ms: clock_ms(theme),
        easing: theme.motion.effects_default,
    }
}

pub const LOADING_LABEL: &str = "Loading";
pub const LOADING_PROGRESS: f32 = 0.65;

/// Determinate circular used as the loading-indicator hero (round-capped arc).
pub fn loading_circular(theme: &Theme) -> CircularProgress {
    circular(theme, LOADING_PROGRESS)
}

/// Pull-to-refresh style circular at the top of a scroll surface.
pub fn pull_to_refresh(theme: &Theme) -> IndeterminateCircular {
    let mut a = circular_indeterminate(theme);
    a.size_dp = PTR_SIZE_DP;
    a.stroke_dp = PTR_STROKE_DP;
    a
}

pub const WAVE_AMPLITUDE_DP: f32 = 3.0;
pub const WAVE_WAVELENGTH_DP: f32 = 20.0;
pub const WAVE_POINTS: usize = 48;
pub const WAVE_STROKE_DP: f32 = 4.0;
pub const WAVE_HEIGHT_DP: f32 = 16.0;
pub const WAVE_WIDTH_DP: f32 = 240.0;
pub const WAVE_DEMO_PROGRESS: f32 = 0.6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WavyProgress {
    pub width_dp: f32,
    pub height_dp: f32,
    pub stroke_dp: f32,
    pub track: Argb,
    pub indicator: Argb,
    pub progress: f32,
    pub duration_ms: u16,
}

pub fn wavy(theme: &Theme, progress: f32) -> WavyProgress {
    WavyProgress {
        width_dp: WAVE_WIDTH_DP,
        height_dp: WAVE_HEIGHT_DP,
        stroke_dp: WAVE_STROKE_DP,
        track: theme.color.secondary_container,
        indicator: theme.color.primary,
        progress: progress.clamp(0.0, 1.0),
        duration_ms: clock_ms(theme),
    }
}

/// Sine-wave polyline for the determinate wavy indicator (`phase` in 0..=1).
pub fn wave_polyline(width: f32, height: f32, progress: f32, phase: f32) -> Vec<(f32, f32)> {
    let usable = (width * progress.clamp(0.0, 1.0)).max(1.0);
    let n = WAVE_POINTS.max(2);
    (0..=n)
        .map(|i| {
            let t = i as f32 / n as f32;
            let x = t * usable;
            let y = height / 2.0
                + WAVE_AMPLITUDE_DP
                    * (std::f32::consts::TAU * (x / WAVE_WAVELENGTH_DP + phase)).sin();
            (x, y)
        })
        .collect()
}

/// Spinning PTR / circular-indeterminate arc (`phase` 0..=1 rotates from 12 o'clock).
pub fn ptr_arc_polyline(size: f32, stroke: f32, arc_deg: f32, phase: f32) -> Vec<(f32, f32)> {
    let cx = size / 2.0;
    let cy = size / 2.0;
    let r = (size / 2.0 - stroke).max(1.0);
    let start = phase * 360.0 - 90.0;
    let n = 20usize;
    (0..=n)
        .map(|i| {
            let a = (start + arc_deg * (i as f32 / n as f32)).to_radians();
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect()
}

/// Endpoints of the PTR / circular arc, used as round-cap disc centers.
pub fn ptr_cap_centers(size: f32, stroke: f32, arc_deg: f32, phase: f32) -> [(f32, f32); 2] {
    let pts = ptr_arc_polyline(size, stroke, arc_deg, phase);
    let first = pts.first().copied().unwrap_or((size / 2.0, stroke));
    let last = pts.last().copied().unwrap_or(first);
    [first, last]
}

/// Determinate circular arc from 12 o'clock (`progress` 0..=1, `phase` unused).
pub fn determinate_arc_polyline(size: f32, stroke: f32, progress: f32, phase: f32) -> Vec<(f32, f32)> {
    ptr_arc_polyline(size, stroke, 360.0 * progress.clamp(0.05, 1.0), phase)
}

pub fn ptr_arc_svg_d(size: f32, stroke: f32, arc_deg: f32, phase: f32) -> String {
    let pts = ptr_arc_polyline(size, stroke, arc_deg, phase);
    let mut d = String::new();
    for (i, (x, y)) in pts.iter().enumerate() {
        if i == 0 {
            d.push_str(&format!("M{x:.2},{y:.2}"));
        } else {
            d.push_str(&format!(" L{x:.2},{y:.2}"));
        }
    }
    d
}

pub fn wave_svg_d(width: f32, height: f32, progress: f32, phase: f32) -> String {
    let pts = wave_polyline(width, height, progress, phase);
    let mut d = String::new();
    for (i, (x, y)) in pts.iter().enumerate() {
        if i == 0 {
            d.push_str(&format!("M{x:.2},{y:.2}"));
        } else {
            d.push_str(&format!(" L{x:.2},{y:.2}"));
        }
    }
    d
}
