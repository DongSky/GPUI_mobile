//! Filled and outlined text fields.
//! Specs: https://m3.material.io/components/text-fields/specs
//!
//! Visual tokens plus a host-testable editor (`TextFieldEditor`). Android
//! NativeActivity still stubs `update_ime_position`; the catalog and demo edit
//! through this editor (HTML `<input>` / on-screen keys).

use crate::argb::Argb;
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 56.0;
pub const PAD_H_DP: f32 = 16.0;
pub const INDICATOR_DP: f32 = 1.0;
pub const INDICATOR_FOCUSED_DP: f32 = 2.0;
pub const OUTLINE_DP: f32 = 1.0;
/// Compose `OutlinedTextFieldDefaults.FocusedBorderThickness` = 2.dp
/// (https://developer.android.com/reference/kotlin/androidx/compose/material3/OutlinedTextFieldDefaults).
pub const OUTLINE_FOCUSED_DP: f32 = 2.0;
pub const SUPPORTING_GAP_DP: f32 = 4.0;
pub const ICON_DP: f32 = 24.0;
pub const NOTCH_PAD_DP: f32 = 4.0;
/// Extra cutout beyond glyph-advance so wide letters (`W`, `@`) do not sliver.
pub const NOTCH_WIDTH_SAFETY_DP: f32 = 4.0;
/// Distance from the left outline to the start of the notched label.
/// Matches HTML `<legend>` `margin-left: 8px` so GPUI and fieldset line up.
pub const NOTCH_START_DP: f32 = 8.0;

/// Width of the top-outline *cutout* for `label` (bodySmall-ish glyph width).
/// Mapping paints left-stroke | gap+label | right-stroke so the border is
/// actually interrupted, not just a label drawn on top of a full stroke.
/// Wide letters (`m`, `w`, `@`) use a larger advance so the stroke does not
/// sliver under the floating legend.
pub fn roboto_advance_em(ch: char) -> f32 {
    match ch {
        'm' | 'M' | 'w' | 'W' | '@' => 0.96,
        'i' | 'l' | 'j' | 'I' | '.' | ',' | '\'' | '|' => 0.32,
        'f' | 't' | 'r' | 's' => 0.42,
        'A'..='Z' => 0.66,
        _ => 0.58,
    }
}

/// Measured (or Roboto-estimated) label width in dp at `size_sp`.
pub fn measured_label_width_dp(label: &str, size_sp: f32) -> f32 {
    label.chars().map(roboto_advance_em).sum::<f32>() * size_sp
}

/// Notch cutout from a host-measured floating-label width (GPUI text layout).
pub fn notch_width_from_measured_dp(measured_label_dp: f32) -> f32 {
    (measured_label_dp + NOTCH_PAD_DP * 2.0 + NOTCH_WIDTH_SAFETY_DP).max(28.0)
}

pub fn notch_width_dp(label: &str, label_size_sp: f32) -> f32 {
    notch_width_from_measured_dp(measured_label_width_dp(label, label_size_sp))
}

/// Notch frame using a host-measured floating-label width (GPUI `layout_line`).
/// `measured_label_dp <= 1` falls back to the Roboto-advance estimate.
pub fn notch_frame_from_layout(
    label: &str,
    appearance: &TextFieldAppearance,
    measured_label_dp: f32,
) -> NotchFrame {
    let width_dp = if measured_label_dp > 1.0 {
        notch_width_from_measured_dp(measured_label_dp)
    } else {
        notch_width_dp(label, appearance.label_style.size_sp)
    };
    let stroke = appearance
        .field
        .outline
        .map(|(_, w)| w)
        .unwrap_or(OUTLINE_DP)
        .max(1.0);
    NotchFrame {
        start_dp: NOTCH_START_DP,
        width_dp,
        stroke_dp: stroke,
        radius_dp: appearance.field.corners.top_left.max(stroke),
        label_h_dp: appearance.label_style.line_height_sp,
        field_h_dp: appearance.field.height_dp,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldVariant {
    Filled,
    Outlined,
}

impl TextFieldVariant {
    pub const ALL: [Self; 2] = [Self::Filled, Self::Outlined];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Outlined => "outlined",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextFieldAppearance {
    pub field: Appearance,
    pub label: Argb,
    pub input: Argb,
    pub supporting: Argb,
    pub label_style: TypeStyle,
    pub input_style: TypeStyle,
    pub supporting_style: TypeStyle,
    pub populated: bool,
    /// Label is floated (populated or focused).
    pub floating: bool,
    /// Outlined + floating: cut a 4dp-padded notch in the top outline.
    pub notched: bool,
    pub leading_icon: Argb,
    pub trailing_icon: Argb,
    pub caret: Argb,
    /// Parent/page fill used to erase the outline under a floating label.
    pub cutout_fill: Argb,
}

pub fn resolve(
    theme: &Theme,
    variant: TextFieldVariant,
    state: InteractionState,
    populated: bool,
) -> TextFieldAppearance {
    let c = theme.color;
    let focused = matches!(
        state,
        InteractionState::Focused | InteractionState::ErrorFocused
    );
    let error = state.is_error();
    let disabled = state.is_disabled();

    let (label, input, supporting, indicator) = if disabled {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        (muted, muted, muted, muted)
    } else if error && focused {
        (c.error, c.on_surface, c.error, c.error)
    } else if error && state == InteractionState::Hovered {
        (
            c.on_error_container,
            c.on_surface,
            c.error,
            c.on_error_container,
        )
    } else if error {
        (c.error, c.on_surface, c.error, c.error)
    } else if focused {
        (c.primary, c.on_surface, c.on_surface_variant, c.primary)
    } else if state == InteractionState::Hovered {
        (
            c.on_surface_variant,
            c.on_surface,
            c.on_surface_variant,
            c.on_surface,
        )
    } else {
        (
            c.on_surface_variant,
            c.on_surface,
            c.on_surface_variant,
            c.on_surface_variant,
        )
    };

    let (container, corners, outline) = match variant {
        TextFieldVariant::Filled => {
            let base = if disabled {
                c.on_surface.with_alpha(0.04).composite_over(c.surface)
            } else {
                c.surface_container_highest
            };
            let width = if focused {
                INDICATOR_FOCUSED_DP
            } else {
                INDICATOR_DP
            };
            (
                base,
                Corners::extra_small_top(theme.shapes.extra_small),
                Some((indicator, width)),
            )
        }
        TextFieldVariant::Outlined => {
            let width = if focused {
                OUTLINE_FOCUSED_DP
            } else {
                OUTLINE_DP
            };
            (
                c.surface,
                Corners::all(theme.shapes.extra_small),
                Some((indicator, width)),
            )
        }
    };

    let label_style = if populated || focused {
        theme.typography.body_small
    } else {
        theme.typography.body_large
    };

    TextFieldAppearance {
        field: Appearance {
            width_dp: None,
            height_dp: HEIGHT_DP,
            min_width_dp: Some(210.0),
            corners,
            container,
            content: input,
            secondary_content: Some(label),
            outline,
            elevation_dp: 0.0,
            pad_start_dp: PAD_H_DP,
            pad_end_dp: PAD_H_DP,
            pad_top_dp: 8.0,
            pad_bottom_dp: 8.0,
            label_style,
            supporting_style: Some(theme.typography.body_small),
        },
        label,
        input,
        supporting,
        label_style,
        input_style: theme.typography.body_large,
        supporting_style: theme.typography.body_small,
        populated,
        floating: populated || focused,
        notched: matches!(variant, TextFieldVariant::Outlined) && (populated || focused),
        leading_icon: if disabled {
            muted_icon(theme)
        } else {
            c.on_surface_variant
        },
        trailing_icon: if disabled {
            muted_icon(theme)
        } else if error {
            c.error
        } else {
            c.on_surface_variant
        },
        caret: if error { c.error } else { c.primary },
        cutout_fill: c.background,
    }
}

/// Shared notch geometry for desktop GPUI / Android / HTML paint tricks.
/// Mapping paints left-stroke | gap+label | right-stroke so the border is
/// actually interrupted (Compose OutlinedTextField / HTML fieldset), not a
/// label overlay on a continuous stroke.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NotchCutout {
    pub start_dp: f32,
    pub width_dp: f32,
    pub stroke_dp: f32,
    pub label_h_dp: f32,
}

pub fn notch_cutout(label: &str, appearance: &TextFieldAppearance) -> NotchCutout {
    let stroke = appearance
        .field
        .outline
        .map(|(_, w)| w)
        .unwrap_or(OUTLINE_DP)
        .max(1.0);
    NotchCutout {
        start_dp: NOTCH_START_DP,
        width_dp: notch_width_dp(label, appearance.label_style.size_sp),
        stroke_dp: stroke,
        label_h_dp: appearance.label_style.line_height_sp,
    }
}

/// Frame geometry for painting a 4dp rounded outline that actually meets the
/// 1–2dp stroke (corner *tiles*, not a stroke-height bar with fake rounding).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NotchFrame {
    pub start_dp: f32,
    pub width_dp: f32,
    pub stroke_dp: f32,
    pub radius_dp: f32,
    pub label_h_dp: f32,
    pub field_h_dp: f32,
}

impl NotchFrame {
    pub fn top_lead_dp(self) -> f32 {
        (self.start_dp - self.radius_dp).max(0.0)
    }

    pub fn middle_h_dp(self) -> f32 {
        (self.field_h_dp - self.radius_dp * 2.0).max(24.0)
    }

    pub fn inner_radius_dp(self) -> f32 {
        (self.radius_dp - self.stroke_dp).max(0.0)
    }

    /// Height of the top-outline cutout. Fieldset only interrupts the stroke,
    /// not a full label-tall hole.
    pub fn notch_gap_h_dp(self) -> f32 {
        self.stroke_dp.max(1.0)
    }

    /// Label origin relative to the field's top-left (legend sitting on the
    /// top stroke, like HTML fieldset).
    pub fn label_origin_dp(self) -> (f32, f32) {
        let x = self.radius_dp + self.top_lead_dp();
        let y = self.stroke_dp / 2.0 - self.label_h_dp / 2.0;
        (x, y)
    }

    /// Inner fill of a filled-with-hole outline: inset by the stroke.
    pub fn hole_rect(self, width_dp: f32) -> (f32, f32, f32, f32, f32) {
        let s = self.stroke_dp.max(1.0);
        let w = width_dp.max(self.radius_dp * 2.0 + self.width_dp + self.start_dp);
        (
            s,
            s,
            (w - s * 2.0).max(1.0),
            (self.field_h_dp - s * 2.0).max(1.0),
            self.inner_radius_dp(),
        )
    }

    /// Top-edge gap that breaks the ring for the floating legend.
    pub fn notch_gap_rect(self) -> (f32, f32, f32, f32) {
        (
            self.start_dp,
            0.0,
            self.width_dp,
            self.stroke_dp.max(1.0),
        )
    }

    /// Centerline verbs for a notched rounded-rect stroke (clockwise, open at
    /// the top-edge cutout). Shared by HTML SVG and GPUI `PathBuilder`.
    pub fn outline_verbs(self, width_dp: f32) -> Vec<OutlineVerb> {
        let s = self.stroke_dp.max(1.0);
        let r = self.radius_dp.max(s);
        let w = width_dp.max(r * 2.0 + self.width_dp + self.start_dp);
        let h = self.field_h_dp.max(r * 2.0);
        let half = s / 2.0;
        let notch_l = self.start_dp.max(half);
        let notch_r = (self.start_dp + self.width_dp).min(w - r);
        vec![
            OutlineVerb::Move(notch_r, half),
            OutlineVerb::Line(w - r, half),
            OutlineVerb::arc_cw(w - half, r, r - half),
            OutlineVerb::Line(w - half, h - r),
            OutlineVerb::arc_cw(w - r, h - half, r - half),
            OutlineVerb::Line(r, h - half),
            OutlineVerb::arc_cw(half, h - r, r - half),
            OutlineVerb::Line(half, r),
            OutlineVerb::arc_cw(r, half, r - half),
            OutlineVerb::Line(notch_l, half),
        ]
    }

    pub fn outline_svg_d(self, width_dp: f32) -> String {
        outline_verbs_svg_d(&self.outline_verbs(width_dp))
    }

    /// Single C-shaped even-odd path: outer clockwise, inner counterclockwise,
    /// joined at the legend gap. Corners are androidx `RoundedPolygon` cubics
    /// (`CIRCULAR_KAPPA`) so GPUI flatten and HTML SVG share one contour.
    pub fn evenodd_verbs(self, width_dp: f32) -> Vec<OutlineVerb> {
        let s = self.stroke_dp.max(1.0);
        let r = self.radius_dp.max(s);
        let w = width_dp.max(r * 2.0 + self.width_dp + self.start_dp);
        let h = self.field_h_dp.max(r * 2.0);
        let notch_l = self.start_dp.max(0.0);
        let notch_r = (self.start_dp + self.width_dp).min(w);
        let ir = (r - s).max(0.0);
        let ix = s;
        let iy = s;
        let iw = (w - s * 2.0).max(1.0);
        let ih = (h - s * 2.0).max(1.0);
        let notch_l_i = notch_l.clamp(ix, ix + iw);
        let notch_r_i = notch_r.clamp(ix, ix + iw);
        let k = crate::shape::CIRCULAR_KAPPA;

        let mut v = Vec::with_capacity(24);
        v.push(OutlineVerb::Move(notch_r, 0.0));
        v.push(OutlineVerb::Line(w - r, 0.0));
        v.push(OutlineVerb::cubic_quarter((w - r, 0.0), (w, 0.0), (w, r), k));
        v.push(OutlineVerb::Line(w, h - r));
        v.push(OutlineVerb::cubic_quarter((w, h - r), (w, h), (w - r, h), k));
        v.push(OutlineVerb::Line(r, h));
        v.push(OutlineVerb::cubic_quarter((r, h), (0.0, h), (0.0, h - r), k));
        v.push(OutlineVerb::Line(0.0, r));
        v.push(OutlineVerb::cubic_quarter((0.0, r), (0.0, 0.0), (r, 0.0), k));
        v.push(OutlineVerb::Line(notch_l, 0.0));
        v.push(OutlineVerb::Line(notch_l_i, iy));
        if ir < 0.5 {
            v.push(OutlineVerb::Line(ix, iy));
            v.push(OutlineVerb::Line(ix, iy + ih));
            v.push(OutlineVerb::Line(ix + iw, iy + ih));
            v.push(OutlineVerb::Line(ix + iw, iy));
            v.push(OutlineVerb::Line(notch_r_i, iy));
        } else {
            v.push(OutlineVerb::Line(ix + ir, iy));
            v.push(OutlineVerb::cubic_quarter(
                (ix + ir, iy),
                (ix, iy),
                (ix, iy + ir),
                k,
            ));
            v.push(OutlineVerb::Line(ix, iy + ih - ir));
            v.push(OutlineVerb::cubic_quarter(
                (ix, iy + ih - ir),
                (ix, iy + ih),
                (ix + ir, iy + ih),
                k,
            ));
            v.push(OutlineVerb::Line(ix + iw - ir, iy + ih));
            v.push(OutlineVerb::cubic_quarter(
                (ix + iw - ir, iy + ih),
                (ix + iw, iy + ih),
                (ix + iw, iy + ih - ir),
                k,
            ));
            v.push(OutlineVerb::Line(ix + iw, iy + ir));
            v.push(OutlineVerb::cubic_quarter(
                (ix + iw, iy + ir),
                (ix + iw, iy),
                (ix + iw - ir, iy),
                k,
            ));
            v.push(OutlineVerb::Line(notch_r_i, iy));
        }
        v.push(OutlineVerb::Line(notch_r, 0.0));
        v.push(OutlineVerb::Close);
        v
    }

    /// Number of `Move` subpaths. The C-shaped notch is a single contour.
    pub fn evenodd_subpath_count(self, width_dp: f32) -> usize {
        self.evenodd_verbs(width_dp)
            .iter()
            .filter(|v| matches!(v, OutlineVerb::Move(_, _)))
            .count()
    }

    pub fn evenodd_svg_d(self, width_dp: f32) -> String {
        outline_verbs_svg_d(&self.evenodd_verbs(width_dp))
    }

    /// Flattened C-path for GPUI even-odd fill. Corners are RoundedPolygon
    /// cubics (`CIRCULAR_KAPPA`), tessellated from `evenodd_verbs`.
    pub fn evenodd_polygon(self, width_dp: f32) -> Vec<(f32, f32)> {
        flatten_outline_verbs(&self.evenodd_verbs(width_dp))
    }
}

/// Tessellate verbs to a polyline (arcs/cubics become line samples).
pub fn flatten_outline_verbs(verbs: &[OutlineVerb]) -> Vec<(f32, f32)> {
    let mut pts = Vec::with_capacity(verbs.len() * 6);
    let mut cx = 0.0_f32;
    let mut cy = 0.0_f32;
    for v in verbs {
        match *v {
            OutlineVerb::Move(x, y) | OutlineVerb::Line(x, y) => {
                pts.push((x, y));
                cx = x;
                cy = y;
            }
            OutlineVerb::Arc {
                to_x,
                to_y,
                radius,
                clockwise,
            } => {
                for (x, y) in tessellate_arc(cx, cy, to_x, to_y, radius.max(0.5), clockwise) {
                    pts.push((x, y));
                }
                cx = to_x;
                cy = to_y;
            }
            OutlineVerb::Cubic {
                c1_x,
                c1_y,
                c2_x,
                c2_y,
                to_x,
                to_y,
            } => {
                for (x, y) in crate::shape::sample_cubic(
                    (cx, cy),
                    (c1_x, c1_y),
                    (c2_x, c2_y),
                    (to_x, to_y),
                    8,
                ) {
                    pts.push((x, y));
                }
                cx = to_x;
                cy = to_y;
            }
            OutlineVerb::Close => {
                if let Some(&(x, y)) = pts.first() {
                    pts.push((x, y));
                    cx = x;
                    cy = y;
                }
            }
        }
    }
    pts
}

fn tessellate_arc(
    from_x: f32,
    from_y: f32,
    to_x: f32,
    to_y: f32,
    radius: f32,
    clockwise: bool,
) -> Vec<(f32, f32)> {
    // Quarter-circle corners used by the notched outline: infer center from
    // axis-aligned from→to with equal radius.
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let (cx, cy, a0, sweep) = if dx.abs() > 0.25 && dy.abs() > 0.25 {
        // Both (from_x, to_y) and (to_x, from_y) are radius-far from the
        // endpoints. Keep the center whose directed sweep is ~90°, not 270°.
        let candidates = [(from_x, to_y), (to_x, from_y)];
        let mut best = (from_x, to_y, 0.0_f32, -std::f32::consts::FRAC_PI_2);
        let mut best_err = f32::MAX;
        for (cx, cy) in candidates {
            let a0 = (from_y - cy).atan2(from_x - cx);
            let a1 = (to_y - cy).atan2(to_x - cx);
            let mut sweep = a1 - a0;
            if clockwise {
                if sweep > 0.0 {
                    sweep -= std::f32::consts::TAU;
                }
            } else if sweep < 0.0 {
                sweep += std::f32::consts::TAU;
            }
            let err = (sweep.abs() - std::f32::consts::FRAC_PI_2).abs();
            if err < best_err {
                best_err = err;
                best = (cx, cy, a0, sweep);
            }
        }
        best
    } else {
        return vec![(to_x, to_y)];
    };
    let n = ((sweep.abs() / 0.18).ceil() as usize).clamp(4, 12);
    (1..=n)
        .map(|i| {
            let a = a0 + sweep * (i as f32 / n as f32);
            (cx + radius * a.cos(), cy + radius * a.sin())
        })
        .collect()
}

#[allow(dead_code)]
fn rounded_rect_verbs(x: f32, y: f32, w: f32, h: f32, r: f32, clockwise: bool) -> Vec<OutlineVerb> {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    if r < 0.5 {
        return vec![
            OutlineVerb::Move(x, y),
            OutlineVerb::Line(x + w, y),
            OutlineVerb::Line(x + w, y + h),
            OutlineVerb::Line(x, y + h),
            OutlineVerb::Close,
        ];
    }
    if clockwise {
        vec![
            OutlineVerb::Move(x + r, y),
            OutlineVerb::Line(x + w - r, y),
            OutlineVerb::arc_cw(x + w, y + r, r),
            OutlineVerb::Line(x + w, y + h - r),
            OutlineVerb::arc_cw(x + w - r, y + h, r),
            OutlineVerb::Line(x + r, y + h),
            OutlineVerb::arc_cw(x, y + h - r, r),
            OutlineVerb::Line(x, y + r),
            OutlineVerb::arc_cw(x + r, y, r),
            OutlineVerb::Close,
        ]
    } else {
        vec![
            OutlineVerb::Move(x + r, y),
            OutlineVerb::arc_ccw(x, y + r, r),
            OutlineVerb::Line(x, y + h - r),
            OutlineVerb::arc_ccw(x + r, y + h, r),
            OutlineVerb::Line(x + w - r, y + h),
            OutlineVerb::arc_ccw(x + w, y + h - r, r),
            OutlineVerb::Line(x + w, y + r),
            OutlineVerb::arc_ccw(x + w - r, y, r),
            OutlineVerb::Close,
        ]
    }
}

fn outline_verbs_svg_d(verbs: &[OutlineVerb]) -> String {
    let mut d = String::new();
    for v in verbs {
        match v {
            OutlineVerb::Move(x, y) => d.push_str(&format!("M{x:.2},{y:.2}")),
            OutlineVerb::Line(x, y) => d.push_str(&format!(" L{x:.2},{y:.2}")),
            OutlineVerb::Arc {
                to_x,
                to_y,
                radius,
                clockwise,
            } => d.push_str(&format!(
                " A{radius:.2},{radius:.2} 0 0 {sweep} {to_x:.2},{to_y:.2}",
                sweep = if *clockwise { 1 } else { 0 },
            )),
            OutlineVerb::Cubic {
                c1_x,
                c1_y,
                c2_x,
                c2_y,
                to_x,
                to_y,
            } => d.push_str(&format!(
                " C{c1_x:.2},{c1_y:.2} {c2_x:.2},{c2_y:.2} {to_x:.2},{to_y:.2}"
            )),
            OutlineVerb::Close => d.push_str(" Z"),
        }
    }
    d
}

/// One step of a notched outline path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutlineVerb {
    Move(f32, f32),
    Line(f32, f32),
    Arc {
        to_x: f32,
        to_y: f32,
        radius: f32,
        clockwise: bool,
    },
    Cubic {
        c1_x: f32,
        c1_y: f32,
        c2_x: f32,
        c2_y: f32,
        to_x: f32,
        to_y: f32,
    },
    Close,
}

impl OutlineVerb {
    pub fn arc_cw(to_x: f32, to_y: f32, radius: f32) -> Self {
        Self::Arc {
            to_x,
            to_y,
            radius,
            clockwise: true,
        }
    }

    pub fn arc_ccw(to_x: f32, to_y: f32, radius: f32) -> Self {
        Self::Arc {
            to_x,
            to_y,
            radius,
            clockwise: false,
        }
    }

    pub fn cubic_quarter(
        from: (f32, f32),
        corner: (f32, f32),
        to: (f32, f32),
        kappa: f32,
    ) -> Self {
        let [c1, c2, end] = crate::shape::rounded_polygon_quarter(from, corner, to, kappa);
        Self::Cubic {
            c1_x: c1.0,
            c1_y: c1.1,
            c2_x: c2.0,
            c2_y: c2.1,
            to_x: end.0,
            to_y: end.1,
        }
    }
}

pub fn notch_frame(label: &str, appearance: &TextFieldAppearance) -> NotchFrame {
    notch_frame_from_layout(label, appearance, 0.0)
}

fn muted_icon(theme: &Theme) -> Argb {
    theme
        .color
        .on_surface
        .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
        .composite_over(theme.color.surface)
}

/// Host-testable caret editor. Used by the HTML catalog and the Android demo
/// so fields are actually editable without a system `InputConnection`.
#[derive(Clone, Debug, PartialEq)]
pub struct TextFieldEditor {
    pub variant: TextFieldVariant,
    value: String,
    caret: usize,
    pub focused: bool,
    pub error: bool,
    pub disabled: bool,
}

impl TextFieldEditor {
    pub fn new(variant: TextFieldVariant, initial: impl Into<String>) -> Self {
        let value = initial.into();
        let caret = value.chars().count();
        Self {
            variant,
            value,
            caret,
            focused: false,
            error: false,
            disabled: false,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn caret(&self) -> usize {
        self.caret
    }

    pub fn populated(&self) -> bool {
        !self.value.is_empty()
    }

    pub fn interaction_state(&self) -> InteractionState {
        if self.disabled {
            InteractionState::Disabled
        } else if self.error && self.focused {
            InteractionState::ErrorFocused
        } else if self.error {
            InteractionState::Error
        } else if self.focused {
            InteractionState::Focused
        } else {
            InteractionState::Enabled
        }
    }

    pub fn set_focus(&mut self, focused: bool) {
        if !self.disabled {
            self.focused = focused;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.disabled {
            return;
        }
        if ch == '\u{8}' || ch == '\u{7f}' {
            self.backspace();
            return;
        }
        if ch.is_control() {
            return;
        }
        let mut chars: Vec<char> = self.value.chars().collect();
        let i = self.caret.min(chars.len());
        chars.insert(i, ch);
        self.caret = i + 1;
        self.value = chars.into_iter().collect();
    }

    pub fn insert_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    pub fn backspace(&mut self) {
        if self.disabled || self.caret == 0 {
            return;
        }
        let mut chars: Vec<char> = self.value.chars().collect();
        chars.remove(self.caret - 1);
        self.caret -= 1;
        self.value = chars.into_iter().collect();
    }

    pub fn move_caret(&mut self, delta: i32) {
        let len = self.value.chars().count() as i32;
        self.caret = (self.caret as i32 + delta).clamp(0, len) as usize;
    }

    pub fn appearance(&self, theme: &Theme) -> TextFieldAppearance {
        resolve(
            theme,
            self.variant,
            self.interaction_state(),
            self.populated(),
        )
    }

    /// Visible value with a `|` caret when focused (Android demo / tests).
    pub fn display_with_caret(&self) -> String {
        if !self.focused {
            return self.value.clone();
        }
        let chars: Vec<char> = self.value.chars().collect();
        let i = self.caret.min(chars.len());
        let mut out = String::new();
        for (idx, ch) in chars.iter().enumerate() {
            if idx == i {
                out.push('|');
            }
            out.push(*ch);
        }
        if i == chars.len() {
            out.push('|');
        }
        out
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.caret = self.value.chars().count();
    }
}

pub const IME_CARET_W_DP: f32 = 2.0;
pub const IME_CARET_H_DP: f32 = 24.0;

/// Logical caret origin inside a 56dp field, for NativeActivity IME stubs.
pub fn ime_cursor_origin_dp(caret_chars: usize, size_sp: f32) -> (f32, f32) {
    (
        PAD_H_DP + caret_chars as f32 * size_sp * 0.52,
        HEIGHT_DP / 2.0,
    )
}

/// Caret rectangle (`x, y, w, h`) consumed by `gpui_android::ime` / `update_ime_position`.
pub fn ime_caret_rect_dp(caret_chars: usize, size_sp: f32) -> (f32, f32, f32, f32) {
    let (x, y) = ime_cursor_origin_dp(caret_chars, size_sp);
    (
        x,
        y - IME_CARET_H_DP / 2.0,
        IME_CARET_W_DP,
        IME_CARET_H_DP,
    )
}

/// Window-space caret rect for `update_ime_position` (`field` origin + local caret).
pub fn ime_caret_rect_in_window(
    field_x: f32,
    field_y: f32,
    caret_chars: usize,
    size_sp: f32,
) -> (f32, f32, f32, f32) {
    let (x, y, w, h) = ime_caret_rect_dp(caret_chars, size_sp);
    (field_x + x, field_y + y, w, h)
}

/// Lightweight email check used by the outlined error demo.
pub fn looks_like_email(s: &str) -> bool {
    let Some(at) = s.find('@') else {
        return false;
    };
    at > 0 && s[at + 1..].contains('.')
}
