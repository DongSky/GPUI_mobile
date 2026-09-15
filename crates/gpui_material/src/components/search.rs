//! Docked search bar + expanded search view.
//! Specs: https://m3.material.io/components/search/specs
//!
//! Catalog shows the 56dp docked bar and the expanded view/sheet (back +
//! input + suggestion list). Full-screen search activity is not a separate
//! platform window.

use crate::argb::Argb;
use crate::components::Appearance;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 56.0;
pub const PAD_H_DP: f32 = 16.0;
pub const ICON_DP: f32 = 24.0;
pub const GAP_DP: f32 = 16.0;
pub const AVATAR_DP: f32 = 30.0;
pub const PLACEHOLDER: &str = "Hinted search text";
pub const LEADING_ICON: &str = "⌕";
pub const TRAILING_MIC: &str = "🎤";
pub const VIEW_BACK: &str = "←";
pub const VIEW_CORNER_DP: f32 = 28.0;
pub const VIEW_HEADER_DP: f32 = 72.0;
pub const SUGGESTION_H_DP: f32 = 56.0;
pub const SUGGESTIONS: [&str; 4] = ["App", "Shortcut", "Recent search", "Setting"];
/// Catalog starts expanded so Visual QA can see the sheet + caret after the grow morph.
pub const VIEW_OPEN_BY_DEFAULT: bool = true;
/// Expanded view uses 0dp corners + surface (full-screen search activity).
pub const ACTIVITY_CORNER_DP: f32 = 0.0;
pub const ACTIVITY_MIN_H_DP: f32 = 320.0;

#[derive(Clone, Debug, PartialEq)]
pub struct SearchAppearance {
    pub bar: Appearance,
    pub placeholder: Argb,
    pub leading_icon: Argb,
    pub trailing_icon: Argb,
    pub avatar: Argb,
    pub avatar_label: Argb,
    pub placeholder_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> SearchAppearance {
    let c = theme.color;
    SearchAppearance {
        bar: Appearance {
            width_dp: None,
            height_dp: HEIGHT_DP,
            min_width_dp: Some(360.0),
            corners: Corners::all(HEIGHT_DP / 2.0),
            container: c.surface_container_high,
            content: c.on_surface,
            secondary_content: Some(c.on_surface_variant),
            outline: None,
            elevation_dp: 0.0,
            pad_start_dp: PAD_H_DP,
            pad_end_dp: PAD_H_DP,
            pad_top_dp: 0.0,
            pad_bottom_dp: 0.0,
            label_style: theme.typography.body_large,
            supporting_style: Some(theme.typography.body_large),
        },
        placeholder: c.on_surface_variant,
        leading_icon: c.on_surface,
        trailing_icon: c.on_surface_variant,
        avatar: c.primary_container,
        avatar_label: c.on_primary_container,
        placeholder_style: theme.typography.body_large,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchViewAppearance {
    pub container: Argb,
    pub header: Argb,
    pub input: Argb,
    pub placeholder: Argb,
    pub divider: Argb,
    pub suggestion: Argb,
    pub suggestion_icon: Argb,
    pub elevation_dp: f32,
    pub corners: Corners,
    pub header_h_dp: f32,
    pub suggestion_h_dp: f32,
    pub title_style: TypeStyle,
    pub suggestion_style: TypeStyle,
}

pub fn resolve_view(theme: &Theme) -> SearchViewAppearance {
    let c = theme.color;
    SearchViewAppearance {
        container: c.surface_container_high,
        header: c.on_surface,
        input: c.on_surface,
        placeholder: c.on_surface_variant,
        divider: c.outline_variant,
        suggestion: c.on_surface,
        suggestion_icon: c.on_surface_variant,
        elevation_dp: theme.elevation.level2,
        corners: Corners::all(VIEW_CORNER_DP),
        header_h_dp: VIEW_HEADER_DP,
        suggestion_h_dp: SUGGESTION_H_DP,
        title_style: theme.typography.body_large,
        suggestion_style: theme.typography.body_large,
    }
}

pub fn resolve_activity(theme: &Theme) -> SearchViewAppearance {
    let mut a = resolve_view(theme);
    a.container = theme.color.surface;
    a.corners = crate::shape::Corners::all(ACTIVITY_CORNER_DP);
    a.elevation_dp = 0.0;
    a
}

pub fn filter_suggestions(query: &str) -> Vec<&'static str> {
    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        SUGGESTIONS.to_vec()
    } else {
        SUGGESTIONS
            .iter()
            .copied()
            .filter(|s| s.to_ascii_lowercase().contains(&q))
            .collect()
    }
}

pub fn apply_search_key(query: &str, key: &str) -> String {
    let mut chars: Vec<char> = query.chars().collect();
    match key {
        "backspace" | "delete" => {
            chars.pop();
        }
        "space" => chars.push(' '),
        k if k.len() == 1 => {
            let ch = k.chars().next().unwrap();
            if !ch.is_control() {
                chars.push(ch);
            }
        }
        _ => {}
    }
    chars.into_iter().collect()
}

pub fn query_display(query: &str) -> &str {
    if query.is_empty() {
        PLACEHOLDER
    } else {
        query
    }
}

pub const EMPTY_SUGGESTIONS: &str = "No matching apps";

/// Docked bar (`0`) vs full-screen search activity (`1`) morph parameter.
pub fn morph_t(open: bool) -> f32 {
    if open { 1.0 } else { 0.0 }
}

/// Interpolated container height for the docked→activity growing-bar.
pub fn morph_height_dp(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    HEIGHT_DP + (ACTIVITY_MIN_H_DP - HEIGHT_DP) * t
}

/// Interpolated corner radius (28dp docked → 0dp activity).
pub fn morph_corner_dp_at(t: f32) -> f32 {
    (HEIGHT_DP / 2.0) * (1.0 - t.clamp(0.0, 1.0))
}

/// Docked bar (28dp) vs activity (0dp) corner for the morph.
pub fn morph_corner_dp(open: bool) -> f32 {
    morph_corner_dp_at(morph_t(open))
}

/// Spatial-fast duration for the growing-bar layout animation.
pub fn morph_ms(theme: &Theme) -> u16 {
    theme.motion.spatial_fast_ms
}

/// Suggestion-list opacity during the docked→activity grow.
pub fn morph_list_opacity(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

/// Docked avatar / leading search icon fades out as the activity header takes over.
pub fn morph_avatar_opacity(t: f32) -> f32 {
    (1.0 - t.clamp(0.0, 1.0)).max(0.0)
}

/// Activity back chevron fades in as the docked leading chrome fades out.
pub fn morph_back_opacity(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

/// Compose SearchBar-style shared-element frame (one container, not a swap).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MorphFrame {
    pub t: f32,
    pub height_dp: f32,
    pub corner_dp: f32,
    pub header_h_dp: f32,
    pub suggestion_opacity: f32,
    /// Horizontal inset that collapses as the bar grows into the activity.
    pub inset_h_dp: f32,
    /// Subtle shared-element scale (docked 0.94 → activity 1.0).
    pub scale: f32,
    /// Docked leading icon + avatar (1 at t=0).
    pub leading_docked_opacity: f32,
    /// Activity back chevron (1 at t=1).
    pub leading_activity_opacity: f32,
}

/// How far the docked bar sits inset from the activity edges (shared-element).
pub const SHARED_INSET_DOCKED_DP: f32 = 16.0;
pub const SHARED_SCALE_DOCKED: f32 = 0.94;

/// Spatial-fast easing for the growing-bar (may slightly overshoot, then clamp).
pub fn morph_eased_t(linear: f32) -> f32 {
    crate::motion::cubic_bezier(0.42, 1.67, 0.21, 0.90, linear.clamp(0.0, 1.0)).clamp(0.0, 1.0)
}

/// Shared-element container at linear or already-eased `t` (0 docked … 1 activity).
pub fn morph_frame_at(t: f32) -> MorphFrame {
    let t = t.clamp(0.0, 1.0);
    MorphFrame {
        t,
        height_dp: morph_height_dp(t),
        corner_dp: morph_corner_dp_at(t),
        header_h_dp: HEIGHT_DP + (VIEW_HEADER_DP - HEIGHT_DP) * t,
        suggestion_opacity: t,
        inset_h_dp: SHARED_INSET_DOCKED_DP * (1.0 - t),
        scale: SHARED_SCALE_DOCKED + (1.0 - SHARED_SCALE_DOCKED) * t,
        leading_docked_opacity: morph_avatar_opacity(t),
        leading_activity_opacity: morph_back_opacity(t),
    }
}

pub fn morph_frame_eased(linear: f32) -> MorphFrame {
    morph_frame_at(morph_eased_t(linear))
}

/// Typical catalog column used to approximate CSS `transform: scale` in GPUI
/// (gpui `div` has no layer transform; HTML uses a real CSS scale).
pub const MORPH_STAGE_W_DP: f32 = 640.0;

/// Horizontal margin that combines the shared-element inset with a centered
/// scale shrink (`docked` 0.94 → `activity` 1.0).
pub fn morph_scaled_margin_dp(frame: MorphFrame, stage_w_dp: f32) -> f32 {
    let inner = (stage_w_dp - frame.inset_h_dp * 2.0).max(0.0);
    frame.inset_h_dp + inner * (1.0 - frame.scale) * 0.5
}

pub fn morph_scale_attr(frame: MorphFrame) -> String {
    format!("{:.2}", frame.scale)
}

/// CSS / GPUI layer transform for the shared-element search container.
/// GPUI `div` still has no element transform; hosts paint the container fill
/// with `PathBuilder::scale` about [`TRANSFORM_ORIGIN`] and apply
/// [`morph_layer_box`] for layout. HTML uses CSS `transform`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MorphLayerTransform {
    pub scale: f32,
    pub origin_x_frac: f32,
    pub origin_y_frac: f32,
}

pub const TRANSFORM_ORIGIN: &str = "top center";

pub fn morph_layer_transform(frame: MorphFrame) -> MorphLayerTransform {
    MorphLayerTransform {
        scale: frame.scale,
        origin_x_frac: 0.5,
        origin_y_frac: 0.0,
    }
}

pub fn morph_layer_css(frame: MorphFrame) -> String {
    format!("scale({:.2})", frame.scale)
}

/// Axis-aligned box after applying `transform` around its origin (top-center).
/// GPUI `div` has no element scale; hosts apply this to height + x inset so
/// layout matches CSS `transform-origin: top center` beyond `inset_h_dp`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MorphLayerBox {
    pub x_dp: f32,
    pub y_dp: f32,
    pub width_dp: f32,
    pub height_dp: f32,
}

pub fn morph_layer_box(
    stage_w_dp: f32,
    unscaled_h_dp: f32,
    transform: MorphLayerTransform,
) -> MorphLayerBox {
    let w = stage_w_dp * transform.scale;
    let h = unscaled_h_dp * transform.scale;
    MorphLayerBox {
        x_dp: (stage_w_dp - w) * transform.origin_x_frac,
        y_dp: (unscaled_h_dp - h) * transform.origin_y_frac,
        width_dp: w,
        height_dp: h,
    }
}

/// Shared-element height after top-center scale (GPUI stand-in for CSS scale).
pub fn morph_layer_height_dp(frame: MorphFrame) -> f32 {
    morph_layer_box(
        MORPH_STAGE_W_DP,
        frame.height_dp,
        morph_layer_transform(frame),
    )
    .height_dp
}

/// Map a local point through the layer transform (origin at top-center of `w×h`).
pub fn morph_layer_map_point(
    x: f32,
    y: f32,
    stage_w_dp: f32,
    unscaled_h_dp: f32,
    transform: MorphLayerTransform,
) -> (f32, f32) {
    let ox = stage_w_dp * transform.origin_x_frac;
    let oy = unscaled_h_dp * transform.origin_y_frac;
    (
        ox + (x - ox) * transform.scale,
        oy + (y - oy) * transform.scale,
    )
}

/// CSS `top center` origin in the same space as a GPUI `bounds`.
pub fn path_scale_origin_dp(x: f32, y: f32, width: f32) -> (f32, f32) {
    let layer = morph_layer_transform(morph_frame_at(0.0));
    (x + width * layer.origin_x_frac, y)
}

/// Translations wrapping `PathBuilder::scale` so it matches
/// `transform-origin: top center`. Apply `pre`, then `scale`, then `post`.
pub fn top_center_scale_translates(x: f32, y: f32, width: f32) -> [(f32, f32); 2] {
    let (ox, oy) = path_scale_origin_dp(x, y, width);
    [(-ox, -oy), (ox, oy)]
}

/// Container fill lerp: docked `surface-container-high` → activity `surface`.
pub fn morph_container(theme: &Theme, t: f32) -> crate::argb::Argb {
    theme
        .color
        .surface_container_high
        .lerp(theme.color.surface, t.clamp(0.0, 1.0))
}

pub fn pick_suggestion(query: &str, index: usize) -> Option<&'static str> {
    filter_suggestions(query).get(index).copied()
}

pub fn apply_key_to_editor(ed: &mut crate::components::text_field::TextFieldEditor, key: &str) {
    match key {
        "backspace" | "delete" => ed.backspace(),
        "left" => ed.move_caret(-1),
        "right" => ed.move_caret(1),
        "space" => ed.insert_char(' '),
        "enter" => {
            if let Some(first) = filter_suggestions(ed.value()).first().copied() {
                ed.set_value(first);
            }
        }
        k if k.len() == 1 => {
            if let Some(ch) = k.chars().next() {
                ed.insert_char(ch);
            }
        }
        _ => {}
    }
}
