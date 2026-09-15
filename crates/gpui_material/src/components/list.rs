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
pub const SCENE_SUPPORTING: [&str; SCENE_COUNT] = ["Home network", "Not connected", "Radios off"];
pub const SCENE_ICONS: [&str; SCENE_COUNT] = ["⌁", "◉", "✈"];
pub const SCENE_TRAILING_ON: [bool; SCENE_COUNT] = [true, false, false];
pub const SCENE_KEYS: [&str; SCENE_COUNT] = ["wifi", "bluetooth", "airplane"];

/// Trailing drag-handle (`ListTokens.ItemTrailingIconSize`).
pub const DRAG_HANDLE_DP: f32 = 24.0;
pub const DRAG_HANDLE_GLYPH: &str = "⋮⋮";

/// Swipe-to-reveal rail (Compose `SwipeToDismissBox` / MDC `SwipeableListItem`).
/// Drag + LazyColumn-style inertial fling (`v e^{-kt}`) settles to Closed / Open
/// (±80) / `STATE_SWIPE_PRIMARY_ACTION` (±row) with overshoot past the anchors.
pub const SWIPE_THRESHOLD_DP: f32 = 56.0;
pub const SWIPE_REVEAL_DP: f32 = 80.0;
/// Catalog / desktop swipe row width (`ListItemLayout` full-swipe distance).
pub const SWIPE_ROW_WIDTH_DP: f32 = 360.0;
/// MDC `STATE_SWIPE_PRIMARY_ACTION` — sheet fully off the row.
pub const SWIPE_PRIMARY_ACTION_DP: f32 = SWIPE_ROW_WIDTH_DP;
/// Compose `SwipeToDismissBox` default positional threshold (50% of row).
pub const SWIPE_PRIMARY_THRESHOLD_DP: f32 = SWIPE_PRIMARY_ACTION_DP * 0.5;
/// MDC `SwipeableListItem.getSwipeMaxOvershoot` analog (dp past Open / Primary).
pub const SWIPE_OVERSHOOT_DP: f32 = 16.0;
/// Same exponential decay as `carousel::FLING_DECAY` (LazyColumn analog).
pub const SWIPE_FLING_DECAY: f32 = 2.0;
/// Rest when leftover velocity drops below this (dp/s).
pub const SWIPE_FLING_REST_DP: f32 = 24.0;
/// Flick this fast (dp/s) to commit the next anchor in that direction.
pub const SWIPE_FLING_VELOCITY_DP: f32 = 800.0;
/// Shared vsync step (`motion::FRAME_DT`).
pub const SWIPE_FRAME_DT: f32 = crate::motion::FRAME_DT;
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
pub const REORDER_SUPPORTING: [&str; REORDER_COUNT] = ["Calendar", "Figma file", "Release notes"];
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

/// MDC `SwipeableListItem` swipe states (values match the Android constants).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipePhase {
    /// `STATE_DRAGGING` = 1
    Dragging,
    /// `STATE_SETTLING` = 2
    Settling,
    /// `STATE_CLOSED` = 3
    Closed,
    /// `STATE_OPEN` = 4 (revealed to intrinsic rail width)
    Open,
    /// `STATE_SWIPE_PRIMARY_ACTION` = 5 (full-width dismiss)
    SwipePrimaryAction,
}

impl SwipePhase {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Dragging => "dragging",
            Self::Settling => "settling",
            Self::Closed => "closed",
            Self::Open => "open",
            Self::SwipePrimaryAction => "primary",
        }
    }
}

/// Horizontal swipe offset for one list row. Positive = start-to-end (Archive).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ListSwipeState {
    pub offset_x_dp: f32,
    pub velocity_dp_s: f32,
    pub dismissed: bool,
    pub phase: SwipePhase,
}

impl ListSwipeState {
    pub fn revealed() -> Self {
        Self {
            offset_x_dp: SWIPE_DEMO_OFFSET_DP,
            velocity_dp_s: 0.0,
            dismissed: false,
            phase: SwipePhase::Open,
        }
    }

    pub fn settled() -> Self {
        Self {
            offset_x_dp: 0.0,
            velocity_dp_s: 0.0,
            dismissed: false,
            phase: SwipePhase::Closed,
        }
    }

    fn clamp_offset(offset: f32) -> f32 {
        let max = SWIPE_PRIMARY_ACTION_DP + SWIPE_OVERSHOOT_DP;
        offset.clamp(-max, max)
    }

    pub fn swipe(&mut self, dx_dp: f32) {
        if self.dismissed {
            return;
        }
        self.phase = SwipePhase::Dragging;
        self.offset_x_dp = Self::clamp_offset(self.offset_x_dp + dx_dp);
    }

    /// Add leftover velocity (wheel / pointer release). Same units as carousel fling.
    pub fn impulse(&mut self, dx_dp: f32) {
        if self.dismissed {
            return;
        }
        self.velocity_dp_s += dx_dp;
        self.phase = SwipePhase::Settling;
    }

    pub fn settle(&mut self) {
        if self.dismissed {
            return;
        }
        self.velocity_dp_s = 0.0;
        let x = self.offset_x_dp;
        if x.abs() >= SWIPE_PRIMARY_THRESHOLD_DP {
            self.commit_primary(x.signum());
        } else if x.abs() >= SWIPE_THRESHOLD_DP {
            self.offset_x_dp = x.signum() * SWIPE_REVEAL_DP;
            self.phase = SwipePhase::Open;
        } else {
            self.offset_x_dp = 0.0;
            self.phase = SwipePhase::Closed;
        }
    }

    fn commit_primary(&mut self, sign: f32) {
        self.offset_x_dp = sign * SWIPE_PRIMARY_ACTION_DP;
        self.velocity_dp_s = 0.0;
        self.dismissed = true;
        self.phase = SwipePhase::SwipePrimaryAction;
    }

    /// Integrate one LazyColumn-style frame. Returns the new offset.
    pub fn step(&mut self, dt_s: f32) -> f32 {
        if self.dismissed {
            return self.offset_x_dp;
        }
        let dt = dt_s.max(0.0);
        if self.velocity_dp_s.abs() < SWIPE_FLING_REST_DP {
            self.velocity_dp_s = 0.0;
            self.settle();
            return self.offset_x_dp;
        }
        self.phase = SwipePhase::Settling;
        self.offset_x_dp = Self::clamp_offset(self.offset_x_dp + self.velocity_dp_s * dt);
        self.velocity_dp_s *= (-SWIPE_FLING_DECAY * dt).exp();
        if self.velocity_dp_s.abs() >= SWIPE_FLING_VELOCITY_DP
            && self.offset_x_dp.abs() >= SWIPE_PRIMARY_THRESHOLD_DP
        {
            self.commit_primary(self.offset_x_dp.signum());
            return self.offset_x_dp;
        }
        if self.offset_x_dp.abs() >= SWIPE_PRIMARY_ACTION_DP {
            self.commit_primary(self.offset_x_dp.signum());
        }
        self.offset_x_dp
    }

    pub fn step_live(&mut self, dt_s: f32) -> f32 {
        self.step(dt_s.clamp(0.0, 0.05).max(0.0))
    }

    pub fn step_until_rest(&mut self, dt_s: f32, max_frames: usize) -> f32 {
        for _ in 0..max_frames {
            if self.resting() {
                break;
            }
            self.step(dt_s);
        }
        if !self.resting() {
            self.settle();
        }
        self.offset_x_dp
    }

    pub fn resting(&self) -> bool {
        self.dismissed
            || (self.velocity_dp_s.abs() < SWIPE_FLING_REST_DP
                && matches!(
                    self.phase,
                    SwipePhase::Closed | SwipePhase::Open | SwipePhase::SwipePrimaryAction
                ))
    }

    pub fn needs_frame(&self) -> bool {
        !self.resting()
    }

    pub fn leading_revealed(&self) -> bool {
        self.offset_x_dp >= SWIPE_THRESHOLD_DP
    }

    pub fn trailing_revealed(&self) -> bool {
        self.offset_x_dp <= -SWIPE_THRESHOLD_DP
    }

    pub fn primary_action(&self) -> bool {
        self.phase == SwipePhase::SwipePrimaryAction || self.dismissed
    }
}

/// Wheel / trackpad: add leftover velocity (host ticks `step_live`).
pub fn apply_wheel(state: &mut ListSwipeState, dx: f32) {
    state.impulse(dx);
}

/// Growing `ListItemRevealLayout` rail: 80dp at Open, stretches toward the row
/// width once the sheet passes the intrinsic reveal (primary-action visual).
pub fn leading_rail_width_dp(offset_x_dp: f32) -> f32 {
    if offset_x_dp > SWIPE_REVEAL_DP {
        offset_x_dp.clamp(SWIPE_REVEAL_DP, SWIPE_PRIMARY_ACTION_DP)
    } else {
        SWIPE_REVEAL_DP
    }
}

pub fn trailing_rail_width_dp(offset_x_dp: f32) -> f32 {
    leading_rail_width_dp(-offset_x_dp)
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
