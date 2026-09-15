//! Menus — M3 Expressive (I/O 2026 / Compose 24.1.2).
//! Specs: https://m3.material.io/components/menus/specs
//! Tokens: androidx `SegmentedMenuTokens`, `StandardMenuTokens`, `VibrantMenuTokens`.
//!
//! Expressive vertical menus add standard (surface) + vibrant (tertiary)
//! schemes, grouped surfaces with a 2dp gap, and 44dp items. Nested submenus
//! fly out at `MenuAnchorPosition.End`; the focused surface uses
//! `ActiveContainerShape` 24dp and the parent morphs to
//! `InactiveContainerShape` 8dp. Horizontal segmented menus use a 2dp row;
//! the selected item goes full-round.

use crate::argb::Argb;
use crate::shape::Corners;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;
use crate::typography::TypeStyle;

/// Compose `SegmentedMenuTokens.Item`.
pub const ITEM_HEIGHT_DP: f32 = 44.0;
/// `ItemLeadingSpace` / `ItemTrailingSpace`.
pub const PAD_H_DP: f32 = 16.0;
/// `HorizontalItemLeadingSpace` / `HorizontalItemTrailingSpace`.
pub const HORIZONTAL_PAD_H_DP: f32 = 12.0;
/// Baseline leftover (legacy `MenuTokens.ContainerShape`).
pub const CORNER_DP: f32 = 4.0;
/// `ContainerShape` = corner-large.
pub const CONTAINER_CORNER_DP: f32 = 16.0;
/// `GroupShape` / `InactiveContainerShape` = corner-small.
pub const GROUP_CORNER_DP: f32 = 8.0;
/// `ItemFirstChildShape` / `ItemLastChildShape` / `ItemSelectedShape` = corner-medium.
pub const ITEM_OUTER_CORNER_DP: f32 = 12.0;
/// `ItemShape` / first-last inner = corner-extra-small.
pub const ITEM_INNER_CORNER_DP: f32 = 4.0;
/// `ActiveContainerShape` (hardcoded 24dp in Compose 24.1.2, not extra-large 28).
pub const ACTIVE_CONTAINER_CORNER_DP: f32 = 24.0;
/// `InactiveContainerShape` = corner-small. Parent morphs here while a submenu is open.
pub const INACTIVE_CONTAINER_CORNER_DP: f32 = 8.0;
/// Catalog gap between parent trailing edge and flyout (`MenuAnchorPosition.End`).
pub const SUBMENU_GAP_DP: f32 = 4.0;
/// `ItemLeadingIconSize` / `ItemTrailingIconSize`.
pub const ICON_DP: f32 = 20.0;
/// `ItemBetweenSpace`.
pub const ITEM_BETWEEN_SPACE_DP: f32 = 12.0;
/// `GroupPadding`.
pub const GROUP_PAD_DP: f32 = 4.0;
/// `SegmentedGap` / `MenuDefaults.GroupSpacing`.
pub const GROUP_GAP_DP: f32 = 2.0;
/// `HorizontalSegmentedGap`.
pub const HORIZONTAL_GAP_DP: f32 = 2.0;
/// `HorizontalIconOnlySegmentedGap`.
pub const HORIZONTAL_ICON_GAP_DP: f32 = 4.0;
/// Icon-only horizontal: 16dp pad + 20dp icon.
pub const HORIZONTAL_ICON_PAD_DP: f32 = 16.0;
pub const HORIZONTAL_ICON_SIZE_DP: f32 = 52.0;
/// Official width clamp.
pub const MIN_WIDTH_DP: f32 = 112.0;
pub const MAX_WIDTH_DP: f32 = 280.0;
pub const SUBMENU_CHEVRON: &str = "›";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuScheme {
    /// Surface-based (`StandardMenuTokens`).
    Standard,
    /// Tertiary-based (`VibrantMenuTokens`). Use sparingly.
    Vibrant,
}

impl MenuScheme {
    pub const ALL: [Self; 2] = [Self::Standard, Self::Vibrant];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Vibrant => "vibrant",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAxis {
    Vertical,
    Horizontal,
}

impl MenuAxis {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Compose `MenuGroupShapes`: rest container vs focused submenu vs unfocused parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuFocus {
    /// `ContainerShape` / grouped first-last (16 / 8).
    Rest,
    /// Focused surface in a submenu chain (`ActiveContainerShape` 24).
    Active,
    /// Unfocused parent while a submenu is open (`InactiveContainerShape` 8).
    Inactive,
}

impl MenuFocus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Rest => "rest",
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuAppearance {
    pub container: Argb,
    pub corners: Corners,
    pub elevation_dp: f32,
    pub min_width_dp: f32,
    pub max_width_dp: f32,
    pub pad_dp: f32,
    pub gap_dp: f32,
    pub scheme: MenuScheme,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuItemAppearance {
    pub height_dp: f32,
    pub container: Argb,
    pub label: Argb,
    pub icon: Argb,
    pub shortcut: Argb,
    pub label_style: TypeStyle,
    pub shortcut_style: TypeStyle,
    pub corners: Corners,
    pub pad_h_dp: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuDemoItem {
    pub icon: &'static str,
    pub label: &'static str,
    pub shortcut: &'static str,
    pub submenu: bool,
}

/// Official-style type group (Italic / Bold / Underline).
pub const STYLE_ITEMS: [MenuDemoItem; 3] = [
    MenuDemoItem {
        icon: "I",
        label: "Italic",
        shortcut: "",
        submenu: false,
    },
    MenuDemoItem {
        icon: "B",
        label: "Bold",
        shortcut: "",
        submenu: false,
    },
    MenuDemoItem {
        icon: "U",
        label: "Underline",
        shortcut: "",
        submenu: false,
    },
];
pub const STYLE_SELECTED: usize = 1;

/// Edit group with trailing shortcuts.
pub const EDIT_ITEMS: [MenuDemoItem; 3] = [
    MenuDemoItem {
        icon: "✄",
        label: "Cut",
        shortcut: "⌘X",
        submenu: false,
    },
    MenuDemoItem {
        icon: "❐",
        label: "Copy",
        shortcut: "⌘C",
        submenu: false,
    },
    MenuDemoItem {
        icon: "📋",
        label: "Paste",
        shortcut: "⌘V",
        submenu: false,
    },
];

/// Trailing submenu row.
pub const MORE_ITEMS: [MenuDemoItem; 1] = [MenuDemoItem {
    icon: "⋯",
    label: "More",
    shortcut: "",
    submenu: true,
}];

pub const VERTICAL_GROUPS: [&[MenuDemoItem]; 3] = [&STYLE_ITEMS, &EDIT_ITEMS, &MORE_ITEMS];

/// Nested flyout from More › (first letters cycle on typeahead `s`).
pub const SUBMENU_ITEMS: [MenuDemoItem; 3] = [
    MenuDemoItem {
        icon: "↗",
        label: "Share",
        shortcut: "",
        submenu: false,
    },
    MenuDemoItem {
        icon: "⬇",
        label: "Save",
        shortcut: "",
        submenu: false,
    },
    MenuDemoItem {
        icon: "⇅",
        label: "Sort",
        shortcut: "",
        submenu: false,
    },
];
pub const SUBMENU_SELECTED: usize = 0;
/// Catalog hero paints the cascade open (screenshot + dump-dom).
pub const CASCADE_OPEN: bool = true;

pub const HORIZONTAL_LABELS: [&str; 4] = ["Day", "Week", "Month", "Year"];
pub const HORIZONTAL_SELECTED: usize = 1;

pub const HORIZONTAL_ICONS: [&str; 3] = ["B", "I", "U"];
pub const HORIZONTAL_ICON_SELECTED: usize = 1;

fn scheme_colors(theme: &Theme, scheme: MenuScheme, selected: bool) -> (Argb, Argb, Argb, Argb) {
    let c = theme.color;
    match (scheme, selected) {
        (MenuScheme::Standard, true) => (
            c.tertiary_container,
            c.on_tertiary_container,
            c.on_tertiary_container,
            c.on_tertiary_container,
        ),
        (MenuScheme::Standard, false) => (
            c.surface_container_low,
            c.on_surface,
            c.on_surface_variant,
            c.on_surface_variant,
        ),
        (MenuScheme::Vibrant, true) => (c.tertiary, c.on_tertiary, c.on_tertiary, c.on_tertiary),
        (MenuScheme::Vibrant, false) => (
            c.tertiary_container,
            c.on_tertiary_container,
            c.on_tertiary_container,
            c.on_tertiary_container,
        ),
    }
}

/// Group / standalone container (`StandardMenuTokens.ContainerColor` /
/// `VibrantMenuTokens.ContainerColor` = surface-container-low / tertiary-container).
pub fn resolve_container(theme: &Theme, scheme: MenuScheme) -> MenuAppearance {
    let (container, _, _, _) = scheme_colors(theme, scheme, false);
    MenuAppearance {
        container,
        corners: Corners::all(CONTAINER_CORNER_DP),
        elevation_dp: theme.elevation.level2,
        min_width_dp: MIN_WIDTH_DP,
        max_width_dp: MAX_WIDTH_DP,
        pad_dp: GROUP_PAD_DP,
        gap_dp: GROUP_GAP_DP,
        scheme,
    }
}

/// Default vertical standard shell (overflow / split / overlay).
pub fn resolve_menu(theme: &Theme) -> MenuAppearance {
    resolve_container(theme, MenuScheme::Standard)
}

pub fn resolve_group(
    theme: &Theme,
    scheme: MenuScheme,
    index: usize,
    count: usize,
) -> MenuAppearance {
    resolve_group_focus(theme, scheme, index, count, MenuFocus::Rest)
}

pub fn resolve_group_focus(
    theme: &Theme,
    scheme: MenuScheme,
    index: usize,
    count: usize,
    focus: MenuFocus,
) -> MenuAppearance {
    let mut a = resolve_container(theme, scheme);
    a.corners = group_corners_focus(index, count, focus);
    a
}

/// Standalone focused submenu (`ActiveContainerShape` 24).
pub fn resolve_submenu(theme: &Theme, scheme: MenuScheme) -> MenuAppearance {
    resolve_group_focus(theme, scheme, 0, 1, MenuFocus::Active)
}

/// Default standard item (overflow / split). Standalone selected uses medium corners.
pub fn resolve_item(theme: &Theme, selected: bool, state: InteractionState) -> MenuItemAppearance {
    resolve_item_at(
        theme,
        MenuScheme::Standard,
        MenuAxis::Vertical,
        0,
        1,
        selected,
        state,
    )
}

pub fn resolve_item_at(
    theme: &Theme,
    scheme: MenuScheme,
    axis: MenuAxis,
    index: usize,
    count: usize,
    selected: bool,
    state: InteractionState,
) -> MenuItemAppearance {
    let c = theme.color;
    let (base, label, icon, shortcut) = scheme_colors(theme, scheme, selected);
    let layer = label;
    let (container, label, icon, shortcut) = if state.is_disabled() {
        let muted = match scheme {
            MenuScheme::Standard => c
                .on_surface
                .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
                .composite_over(base),
            MenuScheme::Vibrant => c
                .on_tertiary_container
                .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
                .composite_over(base),
        };
        (base, muted, muted, muted)
    } else {
        (
            apply_state_layer(base, layer, state.layer_opacity()),
            label,
            icon,
            shortcut,
        )
    };
    let pressed = state == InteractionState::Pressed || state == InteractionState::Hovered;
    MenuItemAppearance {
        height_dp: if axis == MenuAxis::Horizontal {
            HORIZONTAL_ICON_SIZE_DP - HORIZONTAL_ICON_PAD_DP
        } else {
            ITEM_HEIGHT_DP
        },
        container,
        label,
        icon,
        shortcut,
        label_style: theme.typography.body_large,
        shortcut_style: theme.typography.label_small,
        corners: item_corners(axis, index, count, selected, pressed),
        pad_h_dp: if axis == MenuAxis::Horizontal {
            HORIZONTAL_PAD_H_DP
        } else {
            PAD_H_DP
        },
    }
}

pub fn resolve_horizontal(
    theme: &Theme,
    scheme: MenuScheme,
    index: usize,
    count: usize,
    selected: bool,
    state: InteractionState,
) -> MenuItemAppearance {
    let mut a = resolve_item_at(
        theme,
        scheme,
        MenuAxis::Horizontal,
        index,
        count,
        selected,
        state,
    );
    a.height_dp = ITEM_HEIGHT_DP;
    a.pad_h_dp = HORIZONTAL_PAD_H_DP;
    a.corners = item_corners(MenuAxis::Horizontal, index, count, selected, false);
    a
}

pub fn resolve_horizontal_icon(
    theme: &Theme,
    scheme: MenuScheme,
    index: usize,
    count: usize,
    selected: bool,
) -> MenuItemAppearance {
    let mut a = resolve_horizontal(
        theme,
        scheme,
        index,
        count,
        selected,
        InteractionState::Enabled,
    );
    a.height_dp = HORIZONTAL_ICON_SIZE_DP;
    a.pad_h_dp = HORIZONTAL_ICON_PAD_DP;
    a
}

/// First/last groups: large outer / small inner. Standalone: large.
pub fn group_corners(index: usize, count: usize) -> Corners {
    group_corners_focus(index, count, MenuFocus::Rest)
}

pub fn group_corners_focus(index: usize, count: usize, focus: MenuFocus) -> Corners {
    match focus {
        MenuFocus::Active => return Corners::all(ACTIVE_CONTAINER_CORNER_DP),
        MenuFocus::Inactive => return Corners::all(INACTIVE_CONTAINER_CORNER_DP),
        MenuFocus::Rest => {}
    }
    if count <= 1 {
        return Corners::all(CONTAINER_CORNER_DP);
    }
    if index == 0 {
        Corners {
            top_left: CONTAINER_CORNER_DP,
            top_right: CONTAINER_CORNER_DP,
            bottom_right: GROUP_CORNER_DP,
            bottom_left: GROUP_CORNER_DP,
        }
    } else if index + 1 >= count {
        Corners {
            top_left: GROUP_CORNER_DP,
            top_right: GROUP_CORNER_DP,
            bottom_right: CONTAINER_CORNER_DP,
            bottom_left: CONTAINER_CORNER_DP,
        }
    } else {
        Corners::all(GROUP_CORNER_DP)
    }
}

/// Vertical: first/last medium outer + extra-small inner; selected medium.
/// Horizontal: selected `CornerFull`; rest extra-small (pressed/hover medium).
pub fn item_corners(
    axis: MenuAxis,
    index: usize,
    count: usize,
    selected: bool,
    pressed: bool,
) -> Corners {
    if axis == MenuAxis::Horizontal {
        if selected {
            return Corners::all(999.0);
        }
        return Corners::all(if pressed {
            ITEM_OUTER_CORNER_DP
        } else {
            ITEM_INNER_CORNER_DP
        });
    }
    if selected || pressed {
        return Corners::all(ITEM_OUTER_CORNER_DP);
    }
    if count <= 1 {
        return Corners::all(ITEM_OUTER_CORNER_DP);
    }
    if index == 0 {
        Corners {
            top_left: ITEM_OUTER_CORNER_DP,
            top_right: ITEM_OUTER_CORNER_DP,
            bottom_right: ITEM_INNER_CORNER_DP,
            bottom_left: ITEM_INNER_CORNER_DP,
        }
    } else if index + 1 >= count {
        Corners {
            top_left: ITEM_INNER_CORNER_DP,
            top_right: ITEM_INNER_CORNER_DP,
            bottom_right: ITEM_OUTER_CORNER_DP,
            bottom_left: ITEM_OUTER_CORNER_DP,
        }
    } else {
        Corners::all(ITEM_INNER_CORNER_DP)
    }
}

pub fn trailing_text(item: &MenuDemoItem) -> &'static str {
    if item.submenu {
        SUBMENU_CHEVRON
    } else {
        item.shortcut
    }
}

pub fn parent_labels() -> Vec<&'static str> {
    VERTICAL_GROUPS
        .iter()
        .flat_map(|group| group.iter().map(|item| item.label))
        .collect()
}

pub fn submenu_labels() -> Vec<&'static str> {
    SUBMENU_ITEMS.iter().map(|item| item.label).collect()
}

/// Flat index of the More › trigger in `VERTICAL_GROUPS`.
pub const MORE_INDEX: usize = STYLE_ITEMS.len() + EDIT_ITEMS.len();

/// Overlay menus start as the grouped parent only; More opens the flyout.
pub const OVERLAY_FLYOUT_OPEN: bool = false;

pub fn parent_item_count() -> usize {
    VERTICAL_GROUPS.iter().map(|group| group.len()).sum()
}

pub fn parent_item_at(index: usize) -> Option<(usize, usize, MenuDemoItem)> {
    let mut n = 0;
    for (gi, group) in VERTICAL_GROUPS.iter().enumerate() {
        for (i, item) in group.iter().enumerate() {
            if n == index {
                return Some((gi, i, *item));
            }
            n += 1;
        }
    }
    None
}

pub fn is_submenu_trigger(index: usize) -> bool {
    parent_item_at(index)
        .map(|(_, _, item)| item.submenu)
        .unwrap_or(false)
}

/// WAI-ARIA menu typeahead: next label whose first character matches `ch`
/// (case-insensitive), wrapping from `from + 1`.
pub fn typeahead_index(labels: &[&str], from: usize, ch: char) -> Option<usize> {
    let needle = ch.to_lowercase().next()?;
    if needle.is_control() {
        return None;
    }
    let n = labels.len();
    if n == 0 {
        return None;
    }
    let start = (from + 1) % n;
    (0..n).find_map(|step| {
        let i = (start + step) % n;
        let first = labels[i].chars().next()?.to_lowercase().next()?;
        (first == needle).then_some(i)
    })
}

/// Last committed leaf in an overlay / live cascade.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayMenuCommit {
    Parent(usize),
    Submenu(usize),
}

/// Host action after a click or key on the overlay session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayMenuAction {
    /// Keep the overlay / cascade mounted (More opened, arrows, typeahead).
    Stay,
    /// A leaf was activated. Overlay hosts dismiss; in-page cascade stays.
    Commit,
    /// Escape from the parent (or equivalent). Overlay hosts dismiss.
    Dismiss,
}

/// Live GPUI overlay + cascade: grouped parent, End flyout, typeahead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayMenuSession {
    pub submenu_open: bool,
    pub parent_hi: usize,
    pub submenu_hi: usize,
    pub committed: OverlayMenuCommit,
}

impl OverlayMenuSession {
    /// Overlay default: grouped surfaces only (clicking More must not dismiss).
    pub fn overlay() -> Self {
        Self {
            submenu_open: OVERLAY_FLYOUT_OPEN,
            parent_hi: STYLE_SELECTED,
            submenu_hi: SUBMENU_SELECTED,
            committed: OverlayMenuCommit::Parent(STYLE_SELECTED),
        }
    }

    /// Catalog cascade hero default-open for anatomy screenshots.
    pub fn cascade() -> Self {
        Self {
            submenu_open: CASCADE_OPEN,
            parent_hi: MORE_INDEX,
            submenu_hi: SUBMENU_SELECTED,
            committed: OverlayMenuCommit::Submenu(SUBMENU_SELECTED),
        }
    }

    pub fn parent_focus(self) -> MenuFocus {
        if self.submenu_open {
            MenuFocus::Inactive
        } else {
            MenuFocus::Rest
        }
    }

    pub fn parent_selected(self, index: usize) -> bool {
        matches!(self.committed, OverlayMenuCommit::Parent(i) if i == index)
    }

    pub fn submenu_selected(self, index: usize) -> bool {
        self.submenu_hi == index
    }

    pub fn parent_state(self, index: usize, item: &MenuDemoItem) -> InteractionState {
        if item.submenu && self.submenu_open {
            InteractionState::Hovered
        } else if self.parent_hi == index {
            InteractionState::Hovered
        } else {
            InteractionState::Enabled
        }
    }

    pub fn hover_parent(&mut self, index: usize) {
        self.parent_hi = index;
        self.submenu_open = is_submenu_trigger(index);
        if self.submenu_open {
            self.submenu_hi = SUBMENU_SELECTED;
        }
    }

    pub fn hover_leave(&mut self) {
        self.submenu_open = false;
    }

    pub fn click_parent(&mut self, index: usize) -> OverlayMenuAction {
        self.parent_hi = index;
        if is_submenu_trigger(index) {
            self.submenu_open = true;
            self.submenu_hi = SUBMENU_SELECTED;
            OverlayMenuAction::Stay
        } else {
            self.submenu_open = false;
            self.committed = OverlayMenuCommit::Parent(index);
            OverlayMenuAction::Commit
        }
    }

    pub fn click_submenu(&mut self, index: usize) -> OverlayMenuAction {
        self.submenu_hi = index;
        self.committed = OverlayMenuCommit::Submenu(index);
        self.submenu_open = false;
        OverlayMenuAction::Commit
    }

    fn move_hi(&mut self, delta: isize) {
        if self.submenu_open {
            let n = SUBMENU_ITEMS.len() as isize;
            let cur = self.submenu_hi as isize;
            self.submenu_hi = ((cur + delta).rem_euclid(n)) as usize;
        } else {
            let n = parent_item_count() as isize;
            if n == 0 {
                return;
            }
            let cur = self.parent_hi as isize;
            self.parent_hi = ((cur + delta).rem_euclid(n)) as usize;
        }
    }

    fn typeahead(&mut self, ch: char) {
        if self.submenu_open {
            let labels = submenu_labels();
            if let Some(i) = typeahead_index(&labels, self.submenu_hi, ch) {
                self.submenu_hi = i;
            }
        } else {
            let labels = parent_labels();
            if let Some(i) = typeahead_index(&labels, self.parent_hi, ch) {
                self.parent_hi = i;
            }
        }
    }

    /// WAI-ARIA menu keys (`right`/`left`/`up`/`down`/`escape`/`enter` + typeahead).
    pub fn apply_key(&mut self, key: &str) -> OverlayMenuAction {
        match key {
            "right" | "arrowright" => {
                if is_submenu_trigger(self.parent_hi) {
                    self.submenu_open = true;
                    self.submenu_hi = SUBMENU_SELECTED;
                }
                OverlayMenuAction::Stay
            }
            "left" | "arrowleft" if self.submenu_open => {
                self.submenu_open = false;
                self.parent_hi = MORE_INDEX;
                OverlayMenuAction::Stay
            }
            "escape" if self.submenu_open => {
                self.submenu_open = false;
                self.parent_hi = MORE_INDEX;
                OverlayMenuAction::Stay
            }
            "escape" => OverlayMenuAction::Dismiss,
            "down" | "j" | "arrowdown" => {
                self.move_hi(1);
                OverlayMenuAction::Stay
            }
            "up" | "k" | "arrowup" => {
                self.move_hi(-1);
                OverlayMenuAction::Stay
            }
            "enter" | "space" => {
                if self.submenu_open {
                    self.click_submenu(self.submenu_hi)
                } else {
                    self.click_parent(self.parent_hi)
                }
            }
            k if k.chars().count() == 1 => {
                if let Some(ch) = k.chars().next() {
                    self.typeahead(ch);
                }
                OverlayMenuAction::Stay
            }
            _ => OverlayMenuAction::Stay,
        }
    }
}
