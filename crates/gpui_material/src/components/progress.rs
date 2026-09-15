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

/// Determinate circular used as the circular-progress hero (round-capped filled arc).
pub fn loading_circular(theme: &Theme) -> CircularProgress {
    circular(theme, LOADING_PROGRESS)
}

/// Pull-to-refresh uses the contained Expressive loading indicator.
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
    polyline_svg_d(&ptr_arc_polyline(size, stroke, arc_deg, phase), false)
}

pub fn wave_svg_d(width: f32, height: f32, progress: f32, phase: f32) -> String {
    polyline_svg_d(&wave_polyline(width, height, progress, phase), false)
}

fn polyline_svg_d(pts: &[(f32, f32)], close: bool) -> String {
    let mut d = String::new();
    for (i, (x, y)) in pts.iter().enumerate() {
        if i == 0 {
            d.push_str(&format!("M{x:.2},{y:.2}"));
        } else {
            d.push_str(&format!(" L{x:.2},{y:.2}"));
        }
    }
    if close && !pts.is_empty() {
        d.push_str(" Z");
    }
    d
}

/// Filled sausage for a round-capped circular stroke (outer arc + caps + inner arc).
/// Used for determinate/indeterminate circular progress — gpui does not re-export
/// lyon `LineCap::Round`, so this polygon *is* the round-cap geometry.
pub const LINE_CAP: &str = "round";

/// Host-owned determinate wait (download bytes, job ticks, elapsed/duration).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaitProgress {
    pub completed: u64,
    pub total: u64,
}

impl WaitProgress {
    pub const fn bytes(completed: u64, total: u64) -> Self {
        Self { completed, total }
    }

    pub fn fraction(self) -> f32 {
        if self.total == 0 {
            1.0
        } else {
            (self.completed as f32 / self.total as f32).clamp(0.0, 1.0)
        }
    }

    pub fn from_elapsed_ms(elapsed_ms: u64, duration_ms: u64) -> Self {
        let total = duration_ms.max(1);
        Self {
            completed: elapsed_ms.min(total),
            total,
        }
    }

    pub fn from_fraction(progress: f32) -> Self {
        let p = progress.clamp(0.0, 1.0);
        Self {
            completed: (p * 1_000.0).round() as u64,
            total: 1_000,
        }
    }
}

/// Catalog snapshot (650 KB of 1 MB) matching `LOADING_PROGRESS`.
pub const DEMO_WAIT: WaitProgress = WaitProgress {
    completed: 650_000,
    total: 1_000_000,
};

pub fn determinate_wait_ms(theme: &Theme) -> u16 {
    clock_ms(theme).saturating_mul(2)
}

pub fn round_capped_arc_polygon(
    size: f32,
    stroke: f32,
    start_deg: f32,
    sweep_deg: f32,
) -> Vec<(f32, f32)> {
    let cx = size / 2.0;
    let cy = size / 2.0;
    let half = stroke.max(1.0) / 2.0;
    let r = (size / 2.0 - stroke).max(half);
    let r_out = r + half;
    let r_in = (r - half).max(0.4);
    let start = start_deg.to_radians();
    let sweep = sweep_deg.to_radians().clamp(0.08, std::f32::consts::TAU * 0.96);
    let n = ((sweep.abs() / 0.09).ceil() as usize).clamp(12, 48);
    let cap_n = 10usize;
    let mut pts = Vec::with_capacity((n + 1) * 2 + cap_n * 2);
    for i in 0..=n {
        let a = start + sweep * (i as f32 / n as f32);
        pts.push((cx + r_out * a.cos(), cy + r_out * a.sin()));
    }
    let end = start + sweep;
    let ecx = cx + r * end.cos();
    let ecy = cy + r * end.sin();
    for i in 1..=cap_n {
        let t = i as f32 / cap_n as f32;
        let ang = end + std::f32::consts::PI * t;
        pts.push((ecx + half * ang.cos(), ecy + half * ang.sin()));
    }
    for i in (0..=n).rev() {
        let a = start + sweep * (i as f32 / n as f32);
        pts.push((cx + r_in * a.cos(), cy + r_in * a.sin()));
    }
    let scx = cx + r * start.cos();
    let scy = cy + r * start.sin();
    for i in 1..=cap_n {
        let t = i as f32 / cap_n as f32;
        let ang = start + std::f32::consts::PI + std::f32::consts::PI * t;
        pts.push((scx + half * ang.cos(), scy + half * ang.sin()));
    }
    pts
}

pub fn round_capped_arc_svg_d(size: f32, stroke: f32, start_deg: f32, sweep_deg: f32) -> String {
    polyline_svg_d(
        &round_capped_arc_polygon(size, stroke, start_deg, sweep_deg),
        true,
    )
}

/// PTR / circular-indeterminate as a filled round-capped arc (`phase` 0..=1).
pub fn ptr_arc_fill(size: f32, stroke: f32, arc_deg: f32, phase: f32) -> Vec<(f32, f32)> {
    let start = phase * 360.0 - 90.0;
    round_capped_arc_polygon(size, stroke, start, arc_deg)
}

pub const LOADING_SIZE_DP: f32 = 38.0;
pub const LOADING_CONTAINED_DP: f32 = 48.0;
pub const LOADING_SHAPES: usize = 7;
pub const LOADING_SAMPLES: usize = 48;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadingIndicator {
    pub size_dp: f32,
    pub contained_dp: f32,
    pub contained: bool,
    pub container: Argb,
    pub indicator: Argb,
    pub duration_ms: u16,
}

/// M3 Expressive morphing loading indicator (short waits).
/// Uncontained: primary morph on the surface (m3.material.io loading-indicator).
pub fn loading_indicator(theme: &Theme) -> LoadingIndicator {
    LoadingIndicator {
        size_dp: LOADING_SIZE_DP,
        contained_dp: LOADING_CONTAINED_DP,
        contained: false,
        container: theme.color.surface_container_highest,
        indicator: theme.color.primary,
        duration_ms: clock_ms(theme),
    }
}

/// Contained loading indicator — preferred for pull-to-refresh.
/// Container = primary-container, indicator = on-primary-container.
pub fn contained_loading_indicator(theme: &Theme) -> LoadingIndicator {
    LoadingIndicator {
        size_dp: LOADING_SIZE_DP,
        contained_dp: LOADING_CONTAINED_DP,
        contained: true,
        container: theme.color.primary_container,
        indicator: theme.color.on_primary_container,
        duration_ms: clock_ms(theme),
    }
}

fn loading_radius(kind: usize, t: f32) -> f32 {
    let tau = std::f32::consts::TAU;
    match kind % LOADING_SHAPES {
        0 => star_radius(8, 0.52, t),
        1 => ngon_radius(9, t),
        2 => ngon_radius(5, t),
        3 => ellipse_radius(1.0, 0.58, t * tau),
        4 => star_radius(12, 0.68, t),
        5 => ngon_radius(4, t),
        _ => ellipse_radius(1.0, 0.74, t * tau),
    }
}

fn star_radius(points: usize, inner: f32, t: f32) -> f32 {
    let u = (t * points as f32).fract();
    if u < 0.5 {
        1.0 + (inner - 1.0) * (u * 2.0)
    } else {
        inner + (1.0 - inner) * ((u - 0.5) * 2.0)
    }
}

fn ngon_radius(sides: usize, t: f32) -> f32 {
    let interior = std::f32::consts::TAU / sides as f32;
    let a = t * std::f32::consts::TAU;
    let local = (a + interior / 2.0).rem_euclid(interior) - interior / 2.0;
    ((interior / 2.0).cos() / local.cos().max(0.18)).clamp(0.45, 1.35)
}

fn ellipse_radius(rx: f32, ry: f32, angle: f32) -> f32 {
    let c = angle.cos();
    let s = angle.sin();
    (rx * ry) / (ry * c).hypot(rx * s).max(1e-4)
}

fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Phase along the 7-shape morph for a determinate wait (`progress` 0..=1).
/// Stops on the last shape at 100% (does not wrap back to the first).
pub fn loading_phase_for_progress(progress: f32) -> f32 {
    progress.clamp(0.0, 1.0) * ((LOADING_SHAPES - 1) as f32 / LOADING_SHAPES as f32)
}

fn loading_polygon_ex(size: f32, phase: f32, rotate: bool) -> Vec<(f32, f32)> {
    let phase = phase.clamp(0.0, 0.999_999);
    let cx = size / 2.0;
    let cy = size / 2.0;
    let max_r = (size / 2.0 - 0.5).max(1.0);
    let x = phase * LOADING_SHAPES as f32;
    let i0 = x.floor() as usize;
    let f = smoothstep(x.fract());
    let i1 = (i0 + 1) % LOADING_SHAPES;
    let rot = if rotate {
        phase * std::f32::consts::TAU
    } else {
        0.0
    };
    (0..LOADING_SAMPLES)
        .map(|i| {
            let t = i as f32 / LOADING_SAMPLES as f32;
            let r = (loading_radius(i0, t) * (1.0 - f) + loading_radius(i1, t) * f) * max_r;
            let a = t * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2 + rot;
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect()
}

/// Morphing Expressive polygon at `phase` 0..=1 (rotates + cycles 7 shapes).
pub fn loading_polygon(size: f32, phase: f32) -> Vec<(f32, f32)> {
    loading_polygon_ex(size, phase.fract().abs(), true)
}

/// Determinate Expressive morph: shape follows `progress`, no spin.
pub fn loading_polygon_for_progress(size: f32, progress: f32) -> Vec<(f32, f32)> {
    loading_polygon_ex(size, loading_phase_for_progress(progress), false)
}

/// Determinate morph driven by a wait/download source.
pub fn loading_polygon_for_wait(size: f32, wait: WaitProgress) -> Vec<(f32, f32)> {
    loading_polygon_for_progress(size, wait.fraction())
}

pub fn loading_svg_d(size: f32, phase: f32) -> String {
    polyline_svg_d(&loading_polygon(size, phase), true)
}

pub fn loading_svg_d_for_progress(size: f32, progress: f32) -> String {
    polyline_svg_d(&loading_polygon_for_progress(size, progress), true)
}

pub fn loading_svg_d_for_wait(size: f32, wait: WaitProgress) -> String {
    loading_svg_d_for_progress(size, wait.fraction())
}

/// Semicolon-separated path `d` values for SVG `<animate>`.
pub fn loading_svg_values(size: f32, frames: usize) -> String {
    let n = frames.max(2);
    (0..n)
        .map(|i| loading_svg_d(size, i as f32 / n as f32))
        .collect::<Vec<_>>()
        .join(";")
}

/// Determinate wait frames (no wrap): progress 0..=1 across `frames`.
pub fn loading_svg_values_for_wait(size: f32, frames: usize) -> String {
    let n = frames.max(2);
    (0..n)
        .map(|i| {
            let p = i as f32 / (n - 1) as f32;
            loading_svg_d_for_progress(size, p)
        })
        .collect::<Vec<_>>()
        .join(";")
}
