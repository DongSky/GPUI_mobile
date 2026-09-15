//! Material 3 Expressive button groups — standard and connected.
//!
//! Two variants ([m3.material.io/components/button-groups](https://m3.material.io/components/button-groups/specs)):
//!
//! - **Standard** — `ButtonGroupSmallTokens.BetweenSpace` 12dp; pressed/selected
//!   child grows by `ButtonGroupDefaults.ExpandedRatio` (0.15) and adjacent
//!   neighbors compress. Rest is round tonal; selected morphs round→square.
//!   Trailing `ButtonGroupDefaults.OverflowIndicator` is a filled icon button
//!   and is outside the neighbor-morph row.
//! - **Connected** — segmented buttons are deprecated; 2dp gap, 8dp inner
//!   corners, fully rounded outer. Selected morphs toward square (`checkedShape`);
//!   press uses the common-button pressed radius. Compose:
//!   `ButtonGroupDefaults.connectedLeading/Middle/TrailingButtonShapes`.

use crate::components::Appearance;
use crate::components::button::{self, ButtonSize, ButtonVariant};
use crate::components::icon_button::{self, IconButtonVariant};
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;

/// Gap between connected segments (MDC `connectedSpaceBetween`).
pub const CONNECTED_GAP_DP: f32 = 2.0;
/// Inner corners of a connected segment (unselected inner / selected morph).
pub const INNER_CORNER_DP: f32 = 8.0;

/// Compose `ButtonGroupSmallTokens.BetweenSpace` for standard groups.
pub const STANDARD_GAP_DP: f32 = 12.0;
/// `ButtonGroupDefaults.ExpandedRatio`: interacted child grows 15%.
pub const EXPANDED_RATIO: f32 = 0.15;
/// Hugged S tonal width for Start / Center / End.
pub const STANDARD_BASE_W_DP: f32 = 88.0;

pub const STANDARD_SEGMENTS: [&str; 3] = ["Start", "Center", "End"];
pub const STANDARD_SELECTED: usize = 1;
/// Compose `ButtonGroupDefaults.OverflowIndicator`: filled icon button at
/// the trailing edge. Hidden children (align extras) land in the menu.
pub const STANDARD_OVERFLOW_GLYPH: &str = "⋮";
/// Hidden children for the standard OverflowIndicator. The popup is a grouped
/// Expressive menu (`menu::STANDARD_OVERFLOW_GROUPS`: Left/Right/Justify + More ›).
pub const STANDARD_OVERFLOW_ITEMS: [&str; 3] = ["Left", "Right", "Justify"];
pub const STANDARD_OVERFLOW_OPEN: bool = true;

pub const DEMO_SEGMENTS: [&str; 3] = ["Day", "Week", "Month"];
pub const DEMO_SELECTED: usize = 1;

/// Official connected icon row (format / image / add) plus trailing overflow.
pub const ICON_SEGMENTS: [&str; 3] = ["✎", "🖼", "＋"];
pub const ICON_SELECTED: usize = 0;
pub const ICON_MIN_W_DP: f32 = 48.0;
pub const OVERFLOW_GLYPH: &str = "⋮";
/// Hidden children for the connected icon overflow. Popup is grouped
/// (`menu::CONNECTED_OVERFLOW_GROUPS`: Cut/Copy/Paste + More ›).
pub const OVERFLOW_ITEMS: [&str; 3] = ["Cut", "Copy", "Paste"];
pub const OVERFLOW_OPEN: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentRole {
    Leading,
    Middle,
    Trailing,
}

/// Widths for a standard group. The expanded child grows by [`EXPANDED_RATIO`];
/// adjacent neighbors share that extra width (sum stays `count * base`).
pub fn standard_widths(count: usize, expanded: Option<usize>, base: f32) -> Vec<f32> {
    let mut widths = vec![base; count];
    let Some(i) = expanded else {
        return widths;
    };
    if count == 0 || i >= count {
        return widths;
    }
    let extra = base * EXPANDED_RATIO;
    widths[i] = base + extra;
    let mut neighbors = Vec::new();
    if i > 0 {
        neighbors.push(i - 1);
    }
    if i + 1 < count {
        neighbors.push(i + 1);
    }
    if neighbors.is_empty() {
        return widths;
    }
    let share = extra / neighbors.len() as f32;
    for n in neighbors {
        widths[n] = (base - share).max(0.0);
    }
    widths
}

pub fn standard_width(index: usize, count: usize, expanded: Option<usize>) -> f32 {
    standard_widths(count, expanded, STANDARD_BASE_W_DP)
        .get(index)
        .copied()
        .unwrap_or(STANDARD_BASE_W_DP)
}

/// Standard group segment: tonal round at rest, filled square when selected,
/// pressed radius while pressed. Width comes from [`standard_width`].
pub fn resolve_standard(
    theme: &Theme,
    index: usize,
    count: usize,
    selected: bool,
    pressed: bool,
    expanded: Option<usize>,
) -> Appearance {
    let state = if pressed {
        InteractionState::Pressed
    } else {
        InteractionState::Enabled
    };
    let variant = if selected {
        ButtonVariant::Filled
    } else {
        ButtonVariant::Tonal
    };
    let shape = if selected {
        button::ButtonShape::Square
    } else {
        button::ButtonShape::Round
    };
    let mut appearance =
        button::resolve_expressive(theme, variant, ButtonSize::Small, shape, state);
    let w = standard_width(index, count, expanded);
    appearance.width_dp = Some(w);
    appearance.min_width_dp = Some(w);
    if selected {
        appearance.label_style = theme.typography.label_large.emphasized();
    }
    appearance
}

pub fn resolve_standard_scene(theme: &Theme, index: usize, selected: usize) -> Appearance {
    resolve_standard(
        theme,
        index,
        STANDARD_SEGMENTS.len(),
        index == selected,
        false,
        Some(selected),
    )
}

/// Trailing overflow affordance for a standard group.
/// Compose `ButtonGroupDefaults.OverflowIndicator` is a filled icon button
/// (`IconButtonDefaults.filledShape` / filled colors) and does **not**
/// participate in `ExpandedRatio` neighbor morph.
pub fn resolve_standard_overflow(theme: &Theme, pressed: bool) -> Appearance {
    let state = if pressed {
        InteractionState::Pressed
    } else {
        InteractionState::Enabled
    };
    icon_button::resolve_expressive(
        theme,
        IconButtonVariant::Filled,
        ButtonSize::Small,
        button::ButtonShape::Round,
        state,
    )
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
    resolve_segment_size(theme, ButtonSize::Small, index, count, selected, pressed)
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
    let mut appearance =
        button::resolve_expressive(theme, variant, size, button::ButtonShape::Round, state);
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
    ICON_SEGMENTS.get(index).copied().unwrap_or(OVERFLOW_GLYPH)
}

/// Catalog settings-like scene title (emphasized hero).
pub const SETTINGS_SCENE_TITLE: &str = "Sound & notifications";
pub const SETTINGS_VOLUME_TITLE: &str = "Volume";
pub const SETTINGS_QUIET_HOURS_TITLE: &str = "Quiet hours";
pub const SETTINGS_PAD_DP: f32 = 16.0;
pub const SETTINGS_GROUP_GAP_DP: f32 = 24.0;
pub const SETTINGS_ROW_GAP_DP: f32 = 8.0;
pub const SETTINGS_CORNER_DP: f32 = 16.0;
