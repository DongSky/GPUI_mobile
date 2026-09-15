//! Docked search bar + expanded search view.
//! Specs: https://m3.material.io/components/search/specs
//!
//! Expressive (recommended): contained search — persistent filled container,
//! no divider. Compact width (`< 600dp`) expands to
//! `ExpandedFullScreenSearchBar` (0 margin / 0 corner). Medium+ (`≥ 600dp`)
//! uses `ExpandedDockedSearchBar` (Corner 28 stays, 24→12dp margin). Divided
//! (baseline) full-screen activity + divider remains available. Catalog /
//! hosts do not use `cx.transform` (height/margin/corner tokens only).

use crate::argb::Argb;
use crate::components::{list, Appearance};
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
/// Focused search shows a clear icon when the query is non-empty.
pub const TRAILING_CLEAR: &str = "✕";
pub const CLEAR_LABEL: &str = "Clear text";
pub const VIEW_BACK: &str = "←";
pub const VIEW_CORNER_DP: f32 = 28.0;
pub const VIEW_HEADER_DP: f32 = 72.0;
pub const SUGGESTION_H_DP: f32 = 56.0;
pub const SUGGESTIONS: [&str; 4] = ["App", "Shortcut", "Recent search", "Setting"];
/// Two-line queried results (Compose one-line is 56; list two-line is 72).
pub const RESULT_H_DP: f32 = 72.0;
/// bodyMedium supporting lines for queried Quick results / Results.
pub const SUGGESTION_SUPPORTING: [&str; 4] = [
    "Installed application",
    "Home screen shortcut",
    "Opened yesterday",
    "System settings",
];
/// Trailing open affordance on submitted Results (northeast / launch).
pub const RESULT_OPEN: &str = "↗";
/// Expressive search rows reuse list `segmentedShapes` (2dp gap, 4/16 corners).
pub const ROW_GAP_DP: f32 = list::SEGMENTED_GAP_DP;
pub const ROW_INNER_CORNER_DP: f32 = list::INNER_CORNER_DP;
pub const ROW_OUTER_CORNER_DP: f32 = list::OUTER_CORNER_DP;
/// Expressive search: gaps separate suggestion/result groups.
pub const SUGGESTION_GROUP_GAP_DP: f32 = 8.0;
pub const SUGGESTION_GROUP_TITLE_H_DP: f32 = 32.0;
pub const SUGGESTION_GROUP_RECENT: [&str; 3] = ["App", "Shortcut", "Recent search"];
pub const SUGGESTION_GROUP_MORE: [&str; 1] = ["Setting"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuggestionGroup {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const SUGGESTION_GROUPS: [SuggestionGroup; 2] = [
    SuggestionGroup {
        title: "Recent",
        items: &SUGGESTION_GROUP_RECENT,
    },
    SuggestionGroup {
        title: "Suggestions",
        items: &SUGGESTION_GROUP_MORE,
    },
];

/// Guidelines: focused search needs a status indicator (search icon or Results
/// label). Typing uses **Quick results**; a submitted query uses **Results**.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchListStatus {
    /// Empty query: historical suggestion groups, no status row.
    Suggestions,
    /// Non-empty query while the field is focused.
    QuickResults,
    /// Queried / submitted: input stays visible but is not focused.
    Results,
}

impl SearchListStatus {
    pub const ALL: [Self; 3] = [Self::Suggestions, Self::QuickResults, Self::Results];

    pub const fn attr(self) -> &'static str {
        match self {
            Self::Suggestions => "suggestions",
            Self::QuickResults => "quick-results",
            Self::Results => "results",
        }
    }

    pub const fn heading(self) -> Option<&'static str> {
        match self {
            Self::Suggestions => None,
            Self::QuickResults => Some(QUICK_RESULTS_LABEL),
            Self::Results => Some(RESULTS_LABEL),
        }
    }

    /// Suggestion groups (Recent / Suggestions) only before a query.
    pub const fn shows_suggestion_groups(self) -> bool {
        matches!(self, Self::Suggestions)
    }

    /// Queried lists use two-line items (headline + supporting).
    pub const fn uses_two_line_rows(self) -> bool {
        !self.shows_suggestion_groups()
    }

    /// Submitted Results show a trailing open affordance.
    pub const fn shows_open_affordance(self) -> bool {
        matches!(self, Self::Results)
    }
}

pub const QUICK_RESULTS_LABEL: &str = "Quick results";
pub const RESULTS_LABEL: &str = "Results";
/// Status row height (titleSmall, matches suggestion-group titles).
pub const STATUS_H_DP: f32 = 32.0;
/// Catalog queried sibling types this so Visual QA sees Quick results.
pub const DEMO_QUERY: &str = "app";

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
    pub result_h_dp: f32,
    pub title_style: TypeStyle,
    pub suggestion_style: TypeStyle,
    pub result_supporting_style: TypeStyle,
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
        result_h_dp: RESULT_H_DP,
        title_style: theme.typography.body_large,
        suggestion_style: theme.typography.body_large,
        result_supporting_style: theme.typography.body_medium,
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
    filter_grouped_suggestions(query)
        .into_iter()
        .flat_map(|(_, items)| items)
        .collect()
}

pub fn filter_grouped_suggestions(query: &str) -> Vec<(&'static str, Vec<&'static str>)> {
    let q = query.trim().to_ascii_lowercase();
    SUGGESTION_GROUPS
        .iter()
        .filter_map(|group| {
            let items: Vec<&'static str> = group
                .items
                .iter()
                .copied()
                .filter(|s| q.is_empty() || s.to_ascii_lowercase().contains(&q))
                .collect();
            if items.is_empty() {
                None
            } else {
                Some((group.title, items))
            }
        })
        .collect()
}

/// Title + inter-group gap chrome for the visible groups.
pub fn suggestion_group_chrome_h_dp(group_count: usize) -> f32 {
    if group_count == 0 {
        0.0
    } else {
        SUGGESTION_GROUP_TITLE_H_DP * group_count as f32
            + SUGGESTION_GROUP_GAP_DP * group_count.saturating_sub(1) as f32
    }
}

pub fn row_corners(index: usize, count: usize, selected: bool) -> Corners {
    list::segmented_corners(index, count, selected, false)
}

pub fn row_container(theme: &Theme, selected: bool) -> Argb {
    if selected {
        theme.color.secondary_container
    } else {
        theme.color.surface
    }
}

pub fn row_content(theme: &Theme, selected: bool) -> Argb {
    if selected {
        theme.color.on_secondary_container
    } else {
        theme.color.on_surface
    }
}

pub fn row_supporting(theme: &Theme, selected: bool) -> Argb {
    if selected {
        theme.color.on_secondary_container
    } else {
        theme.color.on_surface_variant
    }
}

pub fn row_height_dp(status: SearchListStatus) -> f32 {
    if status.uses_two_line_rows() {
        RESULT_H_DP
    } else {
        SUGGESTION_H_DP
    }
}

pub fn supporting_for(label: &str) -> &'static str {
    SUGGESTIONS
        .iter()
        .position(|s| s.eq_ignore_ascii_case(label))
        .map(|i| SUGGESTION_SUPPORTING[i])
        .unwrap_or("")
}

pub fn segmented_row_gaps_h_dp(item_count: usize) -> f32 {
    ROW_GAP_DP * item_count.saturating_sub(1) as f32
}

pub fn suggestion_list_h_dp(suggestion_count: usize, group_count: usize) -> f32 {
    let gaps = if suggestion_count == SUGGESTIONS.len() && group_count == SUGGESTION_GROUPS.len() {
        SUGGESTION_GROUPS
            .iter()
            .map(|g| segmented_row_gaps_h_dp(g.items.len()))
            .sum()
    } else {
        segmented_row_gaps_h_dp(suggestion_count.saturating_sub(group_count.saturating_sub(1)))
    };
    SUGGESTION_H_DP * suggestion_count as f32 + suggestion_group_chrome_h_dp(group_count) + gaps
}

pub fn grouped_suggestion_list_h_dp(query: &str) -> f32 {
    let groups = filter_grouped_suggestions(query);
    let n: usize = groups.iter().map(|(_, items)| items.len()).sum();
    let gaps: f32 = groups
        .iter()
        .map(|(_, items)| segmented_row_gaps_h_dp(items.len()))
        .sum();
    SUGGESTION_H_DP * n as f32 + suggestion_group_chrome_h_dp(groups.len()) + gaps
}

/// Empty query → suggestions; focused non-empty → Quick results; submitted → Results.
pub fn list_status(query: &str, input_focused: bool) -> SearchListStatus {
    if query.trim().is_empty() {
        SearchListStatus::Suggestions
    } else if input_focused {
        SearchListStatus::QuickResults
    } else {
        SearchListStatus::Results
    }
}

pub fn status_chrome_h_dp(status: SearchListStatus) -> f32 {
    if status.heading().is_some() {
        STATUS_H_DP
    } else {
        0.0
    }
}

pub fn status_live_text(status: SearchListStatus, result_count: usize) -> String {
    match status {
        SearchListStatus::Suggestions => String::new(),
        SearchListStatus::QuickResults => {
            if result_count == 1 {
                "1 quick result".to_string()
            } else {
                format!("{result_count} quick results")
            }
        }
        SearchListStatus::Results => {
            if result_count == 1 {
                "1 result".to_string()
            } else {
                format!("{result_count} results")
            }
        }
    }
}

/// List height under the 56dp header: groups when idle, status + two-line rows when queried.
pub fn expanded_list_h_dp(query: &str, input_focused: bool) -> f32 {
    let status = list_status(query, input_focused);
    let n = filter_suggestions(query).len();
    if status.shows_suggestion_groups() {
        grouped_suggestion_list_h_dp(query)
    } else {
        let rows = n.max(1);
        status_chrome_h_dp(status)
            + row_height_dp(status) * rows as f32
            + segmented_row_gaps_h_dp(rows)
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

/// Guidelines: focused search can show an optional clear icon to remove input.
pub fn shows_clear(query: &str) -> bool {
    !query.trim().is_empty()
}

pub fn trailing_action(query: &str) -> &'static str {
    if shows_clear(query) {
        TRAILING_CLEAR
    } else {
        TRAILING_MIC
    }
}

/// Empty the field and keep it focused (Quick results after clear).
pub fn apply_clear(ed: &mut crate::components::text_field::TextFieldEditor) {
    ed.set_value("");
    ed.set_focus(true);
}

/// Docked bar (`0`) vs full-screen search activity (`1`) morph parameter.
pub fn morph_t(open: bool) -> f32 {
    if open {
        1.0
    } else {
        0.0
    }
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

/// PathBuilder / CSS scale for a morph frame (hosts interpolate this with
/// `with_animation` instead of painting only the settled end state).
pub fn morph_path_scale(frame: MorphFrame) -> f32 {
    morph_layer_transform(frame).scale
}

/// Shared-element scale at linear time (eased). Used by GPUI canvas clocks.
pub fn morph_path_scale_eased(linear: f32) -> f32 {
    morph_path_scale(morph_frame_eased(linear))
}

/// HTML `data-search-anim-scale` for the live CSS/PathBuilder scale.
pub fn morph_path_scale_attr(frame: MorphFrame) -> String {
    format!("{:.2}", morph_path_scale(frame))
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

/// Compose / m3.material.io search style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchStyle {
    /// Recommended Expressive: persistent filled container, rounded, no divider.
    Contained,
    /// Baseline: divider + full-screen activity flatten (not recommended).
    Divided,
}

impl SearchStyle {
    pub const ALL: [Self; 2] = [Self::Contained, Self::Divided];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Contained => "contained",
            Self::Divided => "divided",
        }
    }

    pub const fn recommended(self) -> bool {
        matches!(self, Self::Contained)
    }

    pub const fn shows_divider(self) -> bool {
        matches!(self, Self::Divided)
    }
}

/// Catalog / host hero uses contained (recommended). Divided stays available.
pub const DEMO_STYLE: SearchStyle = SearchStyle::Contained;
/// Contained container keeps the 56dp-bar full-round (28) when docked/focused.
pub const CONTAINED_CORNER_DP: f32 = HEIGHT_DP / 2.0;
/// Unfocused horizontal margin (`SearchBar` rest, medium docked).
pub const CONTAINED_MARGIN_UNFOCUSED_DP: f32 = 24.0;
/// Focused horizontal margin (Expressive docked: 24 → 12).
pub const CONTAINED_MARGIN_FOCUSED_DP: f32 = 12.0;
/// Compact collapsed SearchBar inset (full-screen expand goes to 0).
pub const CONTAINED_MARGIN_COMPACT_UNFOCUSED_DP: f32 = 16.0;
/// Contained header stays the 56dp search bar (not the 72dp activity header).
pub const CONTAINED_HEADER_DP: f32 = HEIGHT_DP;
/// Compose `WindowWidthSizeClass.Compact` exclusive upper bound.
pub const COMPACT_MAX_WIDTH_DP: f32 = 600.0;
/// Phone / compact catalog column (full-screen default).
pub const DEMO_COMPACT_WIDTH_DP: f32 = 360.0;
/// Tablet / medium catalog column (docked).
pub const DEMO_MEDIUM_WIDTH_DP: f32 = 720.0;
/// Catalog / desktop tablet stage used to compute the docked ⅔ height cap.
pub const DEMO_SCREEN_H_DP: f32 = 720.0;
/// Specs: docked container min height.
pub const DOCKED_MIN_H_DP: f32 = 240.0;
/// Specs: docked container max = ⅔ of screen height.
pub const DOCKED_MAX_SCREEN_FRAC: f32 = 2.0 / 3.0;
/// Specs: docked container min width.
pub const DOCKED_MIN_W_DP: f32 = 360.0;
/// Specs: docked container max width.
pub const DOCKED_MAX_W_DP: f32 = 720.0;
/// Docked search covers main content with the same 32% scrim as dialogs.
pub const SCRIM_OPACITY: f32 = crate::components::dialog::SCRIM_OPACITY;
/// Catalog hero uses compact (phone) so expanded search is full-screen.
pub const DEMO_WIDTH_CLASS: WindowWidthClass = WindowWidthClass::Compact;

/// Compose material3-adaptive window width size class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowWidthClass {
    /// width < 600dp — phones in portrait.
    Compact,
    /// width ≥ 600dp — tablets / desktop (medium and up share docked search).
    Medium,
}

impl WindowWidthClass {
    pub const ALL: [Self; 2] = [Self::Compact, Self::Medium];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Medium => "medium",
        }
    }

    pub fn from_width_dp(width_dp: f32) -> Self {
        if width_dp < COMPACT_MAX_WIDTH_DP {
            Self::Compact
        } else {
            Self::Medium
        }
    }

    /// `ExpandedFullScreenSearchBar` on compact; `ExpandedDockedSearchBar` on medium+.
    pub const fn expanded_search(self) -> SearchExpandedLayout {
        match self {
            Self::Compact => SearchExpandedLayout::FullScreen,
            Self::Medium => SearchExpandedLayout::Docked,
        }
    }
}

/// Compose expanded search host (`ExpandedFullScreenSearchBar` vs docked popup).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchExpandedLayout {
    FullScreen,
    Docked,
}

impl SearchExpandedLayout {
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullScreen => "fullscreen",
            Self::Docked => "docked",
        }
    }

    /// Docked opens a list over a scrim; full-screen replaces the page.
    pub const fn uses_scrim(self) -> bool {
        matches!(self, Self::Docked)
    }
}

pub fn docked_max_h_dp(screen_h_dp: f32) -> f32 {
    screen_h_dp * DOCKED_MAX_SCREEN_FRAC
}

/// Specs clamp: min 240dp, max ⅔ of the window.
pub fn docked_height_dp(content_h_dp: f32, screen_h_dp: f32) -> f32 {
    content_h_dp.clamp(DOCKED_MIN_H_DP, docked_max_h_dp(screen_h_dp))
}

pub fn docked_width_dp(available_w_dp: f32) -> f32 {
    available_w_dp.clamp(DOCKED_MIN_W_DP, DOCKED_MAX_W_DP)
}

/// List viewport under the pinned 56dp bar (results scroll beneath the bar).
pub fn docked_list_max_h_dp(screen_h_dp: f32) -> f32 {
    (docked_max_h_dp(screen_h_dp) - CONTAINED_HEADER_DP).max(0.0)
}

pub fn uses_docked_scrim(layout: SearchExpandedLayout, focused: bool) -> bool {
    layout.uses_scrim() && focused
}

pub fn docked_scrim(theme: &Theme) -> crate::argb::Argb {
    theme.color.scrim.with_alpha(SCRIM_OPACITY)
}

pub fn dismiss_on_scrim() -> bool {
    true
}

pub fn contained_margin_dp(focused: bool) -> f32 {
    contained_margin_dp_layout(SearchExpandedLayout::Docked, focused)
}

pub fn contained_margin_dp_layout(layout: SearchExpandedLayout, focused: bool) -> f32 {
    match layout {
        SearchExpandedLayout::Docked => {
            if focused {
                CONTAINED_MARGIN_FOCUSED_DP
            } else {
                CONTAINED_MARGIN_UNFOCUSED_DP
            }
        }
        SearchExpandedLayout::FullScreen => {
            if focused {
                0.0
            } else {
                CONTAINED_MARGIN_COMPACT_UNFOCUSED_DP
            }
        }
    }
}

pub fn contained_height_dp(focused: bool, suggestion_count: usize) -> f32 {
    contained_height_dp_layout(SearchExpandedLayout::Docked, focused, suggestion_count)
}

pub fn contained_height_dp_layout(
    layout: SearchExpandedLayout,
    focused: bool,
    suggestion_count: usize,
) -> f32 {
    if !focused {
        return HEIGHT_DP;
    }
    let list =
        CONTAINED_HEADER_DP + suggestion_list_h_dp(suggestion_count, SUGGESTION_GROUPS.len());
    match layout {
        SearchExpandedLayout::Docked => docked_height_dp(list, DEMO_SCREEN_H_DP),
        SearchExpandedLayout::FullScreen => list.max(ACTIVITY_MIN_H_DP),
    }
}

pub fn contained_corner_dp(_focused: bool) -> f32 {
    CONTAINED_CORNER_DP
}

pub fn contained_corner_dp_layout(layout: SearchExpandedLayout, focused: bool) -> f32 {
    match layout {
        SearchExpandedLayout::Docked => CONTAINED_CORNER_DP,
        SearchExpandedLayout::FullScreen => {
            if focused {
                ACTIVITY_CORNER_DP
            } else {
                CONTAINED_CORNER_DP
            }
        }
    }
}

/// Shared-element stand-in for contained expand (corners stay 28; no flatten).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContainedFrame {
    pub t: f32,
    pub height_dp: f32,
    pub corner_dp: f32,
    pub header_h_dp: f32,
    pub margin_dp: f32,
    pub suggestion_opacity: f32,
    pub leading_docked_opacity: f32,
    pub leading_activity_opacity: f32,
}

pub fn contained_suggestion_count() -> usize {
    SUGGESTIONS.len()
}

pub fn contained_frame_at(t: f32, suggestion_count: usize) -> ContainedFrame {
    contained_frame_at_layout(SearchExpandedLayout::Docked, t, suggestion_count)
}

pub fn contained_frame_at_layout(
    layout: SearchExpandedLayout,
    t: f32,
    suggestion_count: usize,
) -> ContainedFrame {
    let t = t.clamp(0.0, 1.0);
    let collapsed_h = HEIGHT_DP;
    let expanded_h = contained_height_dp_layout(layout, true, suggestion_count);
    let collapsed_m = contained_margin_dp_layout(layout, false);
    let expanded_m = contained_margin_dp_layout(layout, true);
    let collapsed_c = contained_corner_dp_layout(layout, false);
    let expanded_c = contained_corner_dp_layout(layout, true);
    ContainedFrame {
        t,
        height_dp: collapsed_h + (expanded_h - collapsed_h) * t,
        corner_dp: collapsed_c + (expanded_c - collapsed_c) * t,
        header_h_dp: CONTAINED_HEADER_DP,
        margin_dp: collapsed_m + (expanded_m - collapsed_m) * t,
        suggestion_opacity: t,
        leading_docked_opacity: morph_avatar_opacity(t),
        leading_activity_opacity: morph_back_opacity(t),
    }
}

pub fn contained_frame_eased(linear: f32, suggestion_count: usize) -> ContainedFrame {
    contained_frame_at(morph_eased_t(linear), suggestion_count)
}

pub fn contained_frame_eased_layout(
    layout: SearchExpandedLayout,
    linear: f32,
    suggestion_count: usize,
) -> ContainedFrame {
    contained_frame_at_layout(layout, morph_eased_t(linear), suggestion_count)
}

pub fn contained_frame_for_width(width_dp: f32, t: f32, suggestion_count: usize) -> ContainedFrame {
    contained_frame_at_layout(
        WindowWidthClass::from_width_dp(width_dp).expanded_search(),
        t,
        suggestion_count,
    )
}

pub fn contained_frame_eased_for_width(
    width_dp: f32,
    linear: f32,
    suggestion_count: usize,
) -> ContainedFrame {
    contained_frame_for_width(width_dp, morph_eased_t(linear), suggestion_count)
}

/// Persistent filled container (contained never lerps to activity `surface`).
pub fn contained_container(theme: &Theme) -> crate::argb::Argb {
    theme.color.surface_container_high
}

pub fn apply_key_to_editor(ed: &mut crate::components::text_field::TextFieldEditor, key: &str) {
    match key {
        "backspace" | "delete" => {
            ed.set_focus(true);
            ed.backspace();
        }
        "left" => ed.move_caret(-1),
        "right" => ed.move_caret(1),
        "space" => {
            ed.set_focus(true);
            ed.insert_char(' ');
        }
        "enter" => {
            if let Some(first) = filter_suggestions(ed.value()).first().copied() {
                ed.set_value(first);
            }
            // Queried: input remains visible but not focused (Results status).
            ed.set_focus(false);
        }
        k if k.len() == 1 => {
            if let Some(ch) = k.chars().next() {
                ed.set_focus(true);
                ed.insert_char(ch);
            }
        }
        _ => {}
    }
}
