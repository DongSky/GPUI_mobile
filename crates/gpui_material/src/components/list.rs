//! List items. Specs: https://m3.material.io/components/lists/specs
//! Tokens: androidx Compose `ListTokens` (VERSION 29.0.0) +
//! `ListItemDefaults.segmentedShapes` / `segmentedColors` / `SegmentedGap`.
//!
//! Expressive (Dec 2025) recommends segmented lists: 2dp gap, extra-small
//! inner (4) / large outer (16) unselected, large (16) selected, selected
//! container `secondary-container`. Baseline 0-corner lists remain available.

use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListLines {
    One,
    Two,
    Three,
}

impl ListLines {
    pub const ALL: [Self; 3] = [Self::One, Self::Two, Self::Three];

    pub const fn height_dp(self) -> f32 {
        match self {
            Self::One => 56.0,
            Self::Two => 72.0,
            Self::Three => 88.0,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::One => "one-line",
            Self::Two => "two-line",
            Self::Three => "three-line",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListStyle {
    /// Baseline (not recommended for new designs).
    Baseline,
    /// Expressive segmented group.
    Segmented,
}

impl ListStyle {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Segmented => "segmented",
        }
    }
}

pub const PAD_H_DP: f32 = 16.0;
pub const LEADING_DP: f32 = 24.0;
pub const LEADING_GAP_DP: f32 = 16.0;

/// Compose `ListTokens.SegmentedGap`.
pub const SEGMENTED_GAP_DP: f32 = 2.0;
/// `ItemContainerExpressiveShape` = extra-small (inner / middle).
pub const INNER_CORNER_DP: f32 = 4.0;
/// `ContainerShape` / `ItemSelectedContainerExpressiveShape` = large.
pub const OUTER_CORNER_DP: f32 = 16.0;
pub const SELECTED_CORNER_DP: f32 = 16.0;
/// `ItemPressedContainerExpressiveShape` = large.
pub const PRESSED_CORNER_DP: f32 = 16.0;
/// `ItemLeadingIconExpressiveSize`.
pub const LEADING_ICON_DP: f32 = 20.0;
/// `ItemBetweenSpace` between leading icon and headline.
pub const ITEM_BETWEEN_SPACE_DP: f32 = 12.0;
/// `ItemLeadingAvatarSize`.
pub const LEADING_AVATAR_DP: f32 = 40.0;

/// Official-style settings hero (trailing switches, single-select).
pub const SCENE_COUNT: usize = 3;
pub const SCENE_SELECTED: usize = 0;
pub const SCENE_HEADLINES: [&str; SCENE_COUNT] = ["Wi-Fi", "Bluetooth", "Airplane mode"];
pub const SCENE_SUPPORTING: [&str; SCENE_COUNT] =
    ["Home network", "Not connected", "Radios off"];
pub const SCENE_ICONS: [&str; SCENE_COUNT] = ["⌁", "◉", "✈"];
pub const SCENE_TRAILING_ON: [bool; SCENE_COUNT] = [true, false, false];
pub const SCENE_KEYS: [&str; SCENE_COUNT] = ["wifi", "bluetooth", "airplane"];

/// Trailing drag-handle (`ListTokens.ItemTrailingIconSize`).
pub const DRAG_HANDLE_DP: f32 = 24.0;
pub const DRAG_HANDLE_GLYPH: &str = "⋮⋮";

/// Swipe-to-reveal rail (Compose `SwipeToDismissBox` around a list item).
pub const SWIPE_THRESHOLD_DP: f32 = 56.0;
pub const SWIPE_REVEAL_DP: f32 = 80.0;
pub const SWIPE_COUNT: usize = 3;
pub const SWIPE_DEMO_INDEX: usize = 0;
pub const SWIPE_DEMO_OFFSET_DP: f32 = 80.0;
pub const SWIPE_HEADLINES: [&str; SWIPE_COUNT] =
    ["Team sync notes", "Design review", "Lunch plans"];
pub const SWIPE_SUPPORTING: [&str; SWIPE_COUNT] =
    ["Alex · 10:24", "Jordan · Yesterday", "Sam · Mon"];
pub const SWIPE_KEYS: [&str; SWIPE_COUNT] = ["sync", "review", "lunch"];
pub const SWIPE_LEADING_LABEL: &str = "Archive";
pub const SWIPE_TRAILING_LABEL: &str = "Delete";

/// Segmented reorder hero (trailing drag handle, click/drag to restack).
pub const REORDER_COUNT: usize = 3;
pub const REORDER_HEADLINES: [&str; REORDER_COUNT] =
    ["Morning briefing", "Design critique", "Ship checklist"];
pub const REORDER_SUPPORTING: [&str; REORDER_COUNT] =
    ["Calendar", "Figma file", "Release notes"];
pub const REORDER_KEYS: [&str; REORDER_COUNT] = ["morning", "critique", "ship"];
pub const REORDER_DEMO: [usize; REORDER_COUNT] = [0, 1, 2];

pub fn resolve(theme: &Theme, lines: ListLines, state: InteractionState) -> Appearance {
    resolve_style(theme, ListStyle::Baseline, lines, 0, 1, false, state)
}

pub fn resolve_segmented(
    theme: &Theme,
    lines: ListLines,
    index: usize,
    count: usize,
    selected: bool,
    state: InteractionState,
) -> Appearance {
    resolve_style(
        theme,
        ListStyle::Segmented,
        lines,
        index,
        count,
        selected,
        state,
    )
}

pub fn resolve_scene(theme: &Theme, index: usize, selected: usize) -> Appearance {
    resolve_segmented(
        theme,
        ListLines::Two,
        index,
        SCENE_COUNT,
        index == selected,
        InteractionState::Enabled,
    )
}

pub fn resolve_style(
    theme: &Theme,
    style: ListStyle,
    lines: ListLines,
    index: usize,
    count: usize,
    selected: bool,
    state: InteractionState,
) -> Appearance {
    let c = theme.color;
    let pressed = state == InteractionState::Pressed;
    let corners = match style {
        ListStyle::Baseline => Corners::all(0.0),
        ListStyle::Segmented => segmented_corners(index, count, selected, pressed),
    };
    let (base, content, supporting) = if state.is_disabled() {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        (c.surface, muted, muted)
    } else if style == ListStyle::Segmented && selected {
        (
            c.secondary_container,
            c.on_secondary_container,
            c.on_secondary_container,
        )
    } else {
        (c.surface, c.on_surface, c.on_surface_variant)
    };
    let container = if state.is_disabled() {
        base
    } else {
        let layer = if selected {
            c.on_secondary_container
        } else {
            c.on_surface
        };
        apply_state_layer(base, layer, state.layer_opacity())
    };
    Appearance {
        width_dp: None,
        height_dp: lines.height_dp(),
        min_width_dp: None,
        corners,
        container,
        content,
        secondary_content: Some(supporting),
        outline: None,
        elevation_dp: 0.0,
        pad_start_dp: PAD_H_DP,
        pad_end_dp: PAD_H_DP,
        pad_top_dp: 8.0,
        pad_bottom_dp: 8.0,
        label_style: theme.typography.body_large,
        supporting_style: Some(theme.typography.body_medium),
    }
}

pub fn leading_action_container(theme: &Theme) -> crate::argb::Argb {
    theme.color.primary
}

pub fn leading_action_content(theme: &Theme) -> crate::argb::Argb {
    theme.color.on_primary
}

pub fn trailing_action_container(theme: &Theme) -> crate::argb::Argb {
    theme.color.error
}

pub fn trailing_action_content(theme: &Theme) -> crate::argb::Argb {
    theme.color.on_error
}

pub fn resolve_swipe_item(theme: &Theme, index: usize, count: usize) -> Appearance {
    resolve_style(
        theme,
        ListStyle::Baseline,
        ListLines::Two,
        index,
        count,
        false,
        InteractionState::Enabled,
    )
}

pub fn resolve_reorder_item(
    theme: &Theme,
    index: usize,
    count: usize,
    selected: bool,
) -> Appearance {
    resolve_segmented(
        theme,
        ListLines::Two,
        index,
        count,
        selected,
        InteractionState::Enabled,
    )
}

/// Horizontal swipe offset for one list row. Positive = start-to-end (Archive).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ListSwipeState {
    pub offset_x_dp: f32,
    pub dismissed: bool,
}

impl ListSwipeState {
    pub fn revealed() -> Self {
        Self {
            offset_x_dp: SWIPE_DEMO_OFFSET_DP,
            dismissed: false,
        }
    }

    pub fn settled() -> Self {
        Self {
            offset_x_dp: 0.0,
            dismissed: false,
        }
    }

    pub fn swipe(&mut self, dx_dp: f32) {
        if self.dismissed {
            return;
        }
        self.offset_x_dp = (self.offset_x_dp + dx_dp).clamp(-SWIPE_REVEAL_DP, SWIPE_REVEAL_DP);
    }

    pub fn settle(&mut self) {
        if self.dismissed {
            return;
        }
        if self.offset_x_dp.abs() >= SWIPE_THRESHOLD_DP {
            self.offset_x_dp = self.offset_x_dp.signum() * SWIPE_REVEAL_DP;
        } else {
            self.offset_x_dp = 0.0;
        }
    }

    pub fn leading_revealed(&self) -> bool {
        self.offset_x_dp >= SWIPE_THRESHOLD_DP
    }

    pub fn trailing_revealed(&self) -> bool {
        self.offset_x_dp <= -SWIPE_THRESHOLD_DP
    }
}

/// Move `from` to `to` in a reorder permutation (drag-handle restack).
pub fn move_item(order: &mut [usize], from: usize, to: usize) {
    if from >= order.len() || to >= order.len() || from == to {
        return;
    }
    let item = order[from];
    if from < to {
        order.copy_within(from + 1..=to, from);
    } else {
        order.copy_within(to..from, to + 1);
    }
    order[to] = item;
}

/// Unselected: 16dp outer / 4dp inner. Selected or pressed: 16dp all.
/// Single-item lists use the large outer shape on every corner.
pub fn segmented_corners(index: usize, count: usize, selected: bool, pressed: bool) -> Corners {
    if selected || pressed {
        return Corners::all(SELECTED_CORNER_DP);
    }
    let inner = INNER_CORNER_DP;
    let outer = OUTER_CORNER_DP;
    if count <= 1 {
        return Corners::all(outer);
    }
    if index == 0 {
        Corners {
            top_left: outer,
            top_right: outer,
            bottom_right: inner,
            bottom_left: inner,
        }
    } else if index + 1 >= count {
        Corners {
            top_left: inner,
            top_right: inner,
            bottom_right: outer,
            bottom_left: outer,
        }
    } else {
        Corners::all(inner)
    }
}
