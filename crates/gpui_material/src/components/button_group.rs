//! Material 3 Expressive connected button group.
//!
//! Segmented buttons are deprecated; connected groups replace them:
//! 2dp gap, 8dp inner corners, fully rounded outer corners. Selected segments
//! morph toward square (`checkedShape`); press uses the common-button pressed
//! radius. Specs: https://m3.material.io/components/button-groups/specs
//! Compose: `ButtonGroupDefaults.connectedLeading/Middle/TrailingButtonShapes`.

use crate::components::button::{self, ButtonSize, ButtonVariant};
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;

/// Gap between connected segments (MDC `connectedSpaceBetween`).
pub const CONNECTED_GAP_DP: f32 = 2.0;
/// Inner corners of a connected segment (unselected inner / selected morph).
pub const INNER_CORNER_DP: f32 = 8.0;

pub const DEMO_SEGMENTS: [&str; 3] = ["Day", "Week", "Month"];
pub const DEMO_SELECTED: usize = 1;

/// Official connected icon row (format / image / add) plus trailing overflow.
pub const ICON_SEGMENTS: [&str; 3] = ["✎", "🖼", "＋"];
pub const ICON_SELECTED: usize = 0;
pub const ICON_MIN_W_DP: f32 = 48.0;
pub const OVERFLOW_GLYPH: &str = "⋮";
pub const OVERFLOW_ITEMS: [&str; 3] = ["Cut", "Copy", "Paste"];
pub const OVERFLOW_OPEN: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentRole {
    Leading,
    Middle,
    Trailing,
}

pub fn segment_role(index: usize, count: usize) -> SegmentRole {
    if index == 0 {
        SegmentRole::Leading
    } else if count <= 1 || index + 1 >= count {
        SegmentRole::Trailing
    } else {
        SegmentRole::Middle
    }
}

pub fn segment_corners(
    role: SegmentRole,
    size: ButtonSize,
    selected: bool,
    pressed: bool,
) -> Corners {
    if pressed {
        return Corners::all(size.pressed_corner_dp());
    }
    let outer = size.height_dp() / 2.0;
    let inner = INNER_CORNER_DP;
    // Selected morphs toward square: outer full-round drops to the inner 8dp.
    let (left, right) = match role {
        SegmentRole::Leading => {
            if selected {
                (inner, inner)
            } else {
                (outer, inner)
            }
        }
        SegmentRole::Middle => (inner, inner),
        SegmentRole::Trailing => {
            if selected {
                (inner, inner)
            } else {
                (inner, outer)
            }
        }
    };
    Corners {
        top_left: left,
        top_right: right,
        bottom_right: right,
        bottom_left: left,
    }
}

pub fn resolve_segment(
    theme: &Theme,
    index: usize,
    count: usize,
    selected: bool,
    pressed: bool,
) -> Appearance {
    resolve_segment_size(
        theme,
        ButtonSize::Small,
        index,
        count,
        selected,
        pressed,
    )
}

pub fn resolve_segment_size(
    theme: &Theme,
    size: ButtonSize,
    index: usize,
    count: usize,
    selected: bool,
    pressed: bool,
) -> Appearance {
    let state = if pressed {
        InteractionState::Pressed
    } else {
        InteractionState::Enabled
    };
    let variant = if selected {
        ButtonVariant::Filled
    } else {
        ButtonVariant::Outlined
    };
    let mut appearance = button::resolve_expressive(
        theme,
        variant,
        size,
        button::ButtonShape::Round,
        state,
    );
    appearance.corners = segment_corners(segment_role(index, count), size, selected, pressed);
    if selected {
        appearance.label_style = theme.typography.label_large.emphasized();
    }
    appearance
}

/// Connected icon segment (square-ish min width; same corners / selected morph).
pub fn resolve_icon_segment(
    theme: &Theme,
    index: usize,
    count: usize,
    selected: bool,
    pressed: bool,
) -> Appearance {
    let mut appearance = resolve_segment(theme, index, count, selected, pressed);
    appearance.min_width_dp = Some(ICON_MIN_W_DP);
    appearance.pad_start_dp = 12.0;
    appearance.pad_end_dp = 12.0;
    appearance
}

/// Icon row plus trailing overflow affordance.
pub fn icon_group_count() -> usize {
    ICON_SEGMENTS.len() + 1
}

pub fn overflow_index() -> usize {
    ICON_SEGMENTS.len()
}

pub fn icon_glyph(index: usize) -> &'static str {
    ICON_SEGMENTS
        .get(index)
        .copied()
        .unwrap_or(OVERFLOW_GLYPH)
}

/// Catalog settings-like scene title (emphasized hero).
pub const SETTINGS_SCENE_TITLE: &str = "Sound & notifications";
pub const SETTINGS_VOLUME_TITLE: &str = "Volume";
pub const SETTINGS_QUIET_HOURS_TITLE: &str = "Quiet hours";
pub const SETTINGS_PAD_DP: f32 = 16.0;
pub const SETTINGS_GROUP_GAP_DP: f32 = 24.0;
pub const SETTINGS_ROW_GAP_DP: f32 = 8.0;
pub const SETTINGS_CORNER_DP: f32 = 16.0;
