//! Navigation rail. Specs: https://m3.material.io/components/navigation-rail/specs
//!
//! Expressive WideNavigationRail: collapsed Top-icon destinations
//! (`NavigationRailCollapsedTokens.NarrowContainerWidth` 80 /
//! `ContainerWidth` 96) with a 56×32 indicator. Expanded Start-icon
//! destinations use a 56dp full-width pill (`NavigationRailHorizontalItemTokens`).
//! Modal expanded is a 220–360dp overlay over a 32% scrim.

use super::dialog;
use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

/// Compose `NavigationRailCollapsedTokens.NarrowContainerWidth`.
pub const WIDTH_DP: f32 = 80.0;
/// Compose `NavigationRailCollapsedTokens.ContainerWidth` (WideNavigationRail).
pub const WIDE_COLLAPSED_WIDTH_DP: f32 = 96.0;
/// Compose `NavigationRailExpandedTokens.ContainerWidthMinimum`.
pub const EXPANDED_WIDTH_DP: f32 = 220.0;
/// Compose `NavigationRailExpandedTokens.ContainerWidthMaximum`.
pub const EXPANDED_WIDTH_MAX_DP: f32 = 360.0;
/// Vertical / Top-icon indicator (`NavigationRailVerticalItemTokens`).
pub const INDICATOR_W_DP: f32 = 56.0;
pub const INDICATOR_H_DP: f32 = 32.0;
/// Horizontal / Start-icon indicator (`NavigationRailHorizontalItemTokens`).
pub const START_INDICATOR_H_DP: f32 = 56.0;
pub const START_LEADING_DP: f32 = 16.0;
pub const START_TRAILING_DP: f32 = 16.0;
pub const START_ICON_LABEL_GAP_DP: f32 = 8.0;
/// `NavigationRailVerticalItemTokens.IconLabelSpace`.
pub const TOP_ICON_LABEL_GAP_DP: f32 = 4.0;
/// `NavigationRailCollapsedTokens.ItemVerticalSpace`.
pub const ITEM_VERTICAL_SPACE_DP: f32 = 4.0;
/// `NavigationRailCollapsedTokens.TopSpace` / expanded `TopSpace`.
pub const WIDE_TOP_SPACE_DP: f32 = 44.0;
pub const ICON_DP: f32 = 24.0;
pub const DEST_GAP_DP: f32 = 12.0;
pub const PAD_TOP_DP: f32 = 16.0;
pub const FAB_SLOT_DP: f32 = 56.0;
/// 32% scrim behind the expanded modal rail (same token as dialogs).
pub const SCRIM_OPACITY: f32 = dialog::SCRIM_OPACITY;

pub const DESTINATIONS: [&str; 3] = ["Home", "Search", "Profile"];
pub const DESTINATION_ICONS: [&str; 3] = ["⌂", "⌕", "☺"];
/// `None` = no badge, `Some(0)` = small dot, `Some(n)` = large count.
pub const DESTINATION_BADGES: [Option<u32>; 3] = [None, Some(3), Some(0)];
pub const DEMO_SELECTED: usize = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailMode {
    Collapsed,
    Expanded,
}

impl RailMode {
    pub const fn width_dp(self) -> f32 {
        match self {
            Self::Collapsed => WIDTH_DP,
            Self::Expanded => EXPANDED_WIDTH_DP,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Collapsed => "collapsed",
            Self::Expanded => "expanded",
        }
    }
}

pub const DEMO_MODE: RailMode = RailMode::Expanded;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavRailAppearance {
    pub width_dp: f32,
    pub expanded_width_dp: f32,
    pub container: Argb,
    pub active_indicator: Argb,
    pub active_icon: Argb,
    pub active_label: Argb,
    pub inactive_icon: Argb,
    pub inactive_label: Argb,
    pub fab: Argb,
    pub fab_icon: Argb,
    pub badge: Argb,
    pub badge_label: Argb,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> NavRailAppearance {
    let c = theme.color;
    NavRailAppearance {
        width_dp: WIDTH_DP,
        expanded_width_dp: EXPANDED_WIDTH_DP,
        container: c.surface,
        active_indicator: c.secondary_container,
        active_icon: c.on_secondary_container,
        // `NavigationRailColorTokens.ItemActiveLabelText` = Secondary
        // (Expressive May 2025: vertical active label is secondary).
        active_label: c.secondary,
        inactive_icon: c.on_surface_variant,
        inactive_label: c.on_surface_variant,
        fab: c.primary_container,
        fab_icon: c.on_primary_container,
        badge: c.error,
        badge_label: c.on_error,
        label_style: theme.typography.label_medium,
    }
}

pub fn resolve_mode(theme: &Theme, mode: RailMode) -> NavRailAppearance {
    let mut a = resolve(theme);
    a.width_dp = mode.width_dp();
    a
}

pub fn clamp_destination(index: usize) -> usize {
    index.min(DESTINATIONS.len().saturating_sub(1))
}

pub fn select_destination(_current: usize, tapped: usize) -> usize {
    clamp_destination(tapped)
}

pub fn toggle_mode(mode: RailMode) -> RailMode {
    match mode {
        RailMode::Collapsed => RailMode::Expanded,
        RailMode::Expanded => RailMode::Collapsed,
    }
}

/// Scrim fill for the expanded modal rail (composited over surface).
pub fn scrim(theme: &Theme) -> Argb {
    theme
        .color
        .scrim
        .with_alpha(SCRIM_OPACITY)
        .composite_over(theme.color.surface)
}

pub fn is_modal(mode: RailMode) -> bool {
    mode == RailMode::Expanded
}

/// Expanded rail traps destination focus; scrim tap dismisses (catalog overlay).
pub fn focus_trapped(mode: RailMode) -> bool {
    is_modal(mode)
}

pub fn dismiss_on_scrim() -> bool {
    true
}

/// Modal rail elevation (level 2). Desktop/HTML host the expanded rail in an
/// overlay window layer (`data-rail-window`); NativeActivity stays single-window.
pub fn modal_elevation_dp(theme: &Theme) -> f32 {
    theme.elevation.level2
}

/// Whether the expanded rail is painted as its own overlay window (not in-flow).
pub fn overlay_window(mode: RailMode) -> bool {
    is_modal(mode)
}

pub fn overlay_window_attr(mode: RailMode) -> &'static str {
    if overlay_window(mode) { "1" } else { "0" }
}

/// Chrome for the expanded rail. Desktop/HTML use a popup-role overlay
/// (`gpui::WindowKind::PopUp`); NativeActivity stays a single OS window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailChrome {
    Overlay,
    Popup,
}

/// `gpui::WindowKind::PopUp` name for desktop hosts.
pub const POPUP_WINDOW_KIND: &str = "popup";
/// gpui enum variant spelling (`WindowKind::PopUp`).
pub const GPUI_WINDOW_KIND: &str = "PopUp";
/// Desktop catalog keeps the in-window overlay: Linux `WindowParams.kind` is
/// unused, so `cx.open_window(WindowKind::PopUp)` would be a second Normal
/// window. NativeActivity cannot open a second OS window.
pub const OS_POPUP_OPENED: bool = false;

pub fn rail_chrome(mode: RailMode) -> RailChrome {
    if is_modal(mode) {
        RailChrome::Popup
    } else {
        RailChrome::Overlay
    }
}

pub fn rail_chrome_attr(mode: RailMode) -> &'static str {
    match rail_chrome(mode) {
        RailChrome::Popup => "popup",
        RailChrome::Overlay => "overlay",
    }
}

/// Desktop `WindowKind::PopUp` spec. NativeActivity cannot open a second OS window.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OsPopupSpec {
    pub kind: &'static str,
    pub gpui_kind: &'static str,
    pub width_dp: f32,
    pub height_dp: f32,
    pub focus: bool,
    pub movable: bool,
    pub title: &'static str,
    pub supported_on_android: bool,
}

pub const OS_POPUP_HEIGHT_DP: f32 = 880.0;
pub const OS_POPUP_TITLE: &str = "Navigation rail";

pub fn os_popup_spec(mode: RailMode) -> OsPopupSpec {
    OsPopupSpec {
        kind: POPUP_WINDOW_KIND,
        gpui_kind: GPUI_WINDOW_KIND,
        width_dp: if is_modal(mode) {
            EXPANDED_WIDTH_DP
        } else {
            WIDTH_DP
        },
        height_dp: OS_POPUP_HEIGHT_DP,
        focus: true,
        movable: false,
        title: OS_POPUP_TITLE,
        supported_on_android: false,
    }
}

pub fn os_popup_attr() -> &'static str {
    if OS_POPUP_OPENED { "1" } else { "0" }
}

/// `WindowOptions { kind: WindowKind::PopUp, width }` tokens for desktop hosts.
/// Do not call `open_window` with this on Linux or NativeActivity.
pub fn os_popup_window_options(mode: RailMode) -> (&'static str, f32, bool) {
    let spec = os_popup_spec(mode);
    (GPUI_WINDOW_KIND, spec.width_dp, spec.supported_on_android)
}

pub fn is_active(selected: usize, index: usize) -> bool {
    selected == index
}

/// Compose `NavigationItemIconPosition` for `WideNavigationRailItem`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconPosition {
    /// Icon above the label. Use with collapsed wide rails.
    Top,
    /// Icon at the start of the label. Use with expanded wide rails.
    Start,
}

impl IconPosition {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Start => "start",
        }
    }

    pub const fn is_start(self) -> bool {
        matches!(self, Self::Start)
    }
}

/// Compose `WideNavigationRailItemDefaults.iconPositionFor`.
pub const fn icon_position_for(rail_expanded: bool) -> IconPosition {
    if rail_expanded {
        IconPosition::Start
    } else {
        IconPosition::Top
    }
}

pub const fn icon_position_for_mode(mode: RailMode) -> IconPosition {
    icon_position_for(matches!(mode, RailMode::Expanded))
}

/// `NavigationRailVerticalItemTokens` / `NavigationRailHorizontalItemTokens`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailItemMetrics {
    pub icon_position: IconPosition,
    pub indicator_w_dp: f32,
    pub indicator_h_dp: f32,
    pub indicator_full_width: bool,
    pub icon_label_gap_dp: f32,
    pub pad_start_dp: f32,
    pub pad_end_dp: f32,
    pub label_style: TypeStyle,
}

pub fn item_metrics(theme: &Theme, position: IconPosition) -> RailItemMetrics {
    match position {
        IconPosition::Top => RailItemMetrics {
            icon_position: position,
            indicator_w_dp: INDICATOR_W_DP,
            indicator_h_dp: INDICATOR_H_DP,
            indicator_full_width: false,
            icon_label_gap_dp: TOP_ICON_LABEL_GAP_DP,
            pad_start_dp: 0.0,
            pad_end_dp: 0.0,
            label_style: theme.typography.label_medium,
        },
        IconPosition::Start => RailItemMetrics {
            icon_position: position,
            indicator_w_dp: 0.0,
            indicator_h_dp: START_INDICATOR_H_DP,
            indicator_full_width: true,
            icon_label_gap_dp: START_ICON_LABEL_GAP_DP,
            pad_start_dp: START_LEADING_DP,
            pad_end_dp: START_TRAILING_DP,
            label_style: theme.typography.label_large,
        },
    }
}

pub fn item_metrics_for_mode(theme: &Theme, mode: RailMode) -> RailItemMetrics {
    item_metrics(theme, icon_position_for_mode(mode))
}

/// Start-icon pill width: rail minus FullWidthLeading/Trailing 16+16.
pub fn start_indicator_width_dp(rail_width_dp: f32) -> f32 {
    (rail_width_dp - START_LEADING_DP - START_TRAILING_DP).max(0.0)
}

pub fn resolve_wide_collapsed(theme: &Theme) -> NavRailAppearance {
    let mut a = resolve(theme);
    a.width_dp = WIDE_COLLAPSED_WIDTH_DP;
    a
}

/// 0 = collapsed, 1 = expanded modal.
pub fn morph_t(expanded: bool) -> f32 {
    if expanded { 1.0 } else { 0.0 }
}

pub fn morph_width_dp(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    WIDTH_DP + (EXPANDED_WIDTH_DP - WIDTH_DP) * t
}

pub fn scrim_opacity_at(t: f32) -> f32 {
    SCRIM_OPACITY * t.clamp(0.0, 1.0)
}

pub fn morph_ms(theme: &Theme) -> u16 {
    theme.motion.spatial_fast_ms
}

pub fn elevation_dp_at(t: f32, theme: &Theme) -> f32 {
    theme.elevation.level2 * t.clamp(0.0, 1.0)
}
