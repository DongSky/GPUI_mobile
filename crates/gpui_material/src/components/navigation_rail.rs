//! Navigation rail. Specs: https://m3.material.io/components/navigation-rail/specs
//!
//! Expressive WideNavigationRail: collapsed Top-icon destinations
//! (`NavigationRailCollapsedTokens.NarrowContainerWidth` 80 /
//! `ContainerWidth` 96, **default**) with a 56×32 indicator. Expanded
//! Start-icon destinations use a 56dp full-width pill
//! (`NavigationRailHorizontalItemTokens`). Expanded layout is either
//! **standard** (in-flow, 96↔220, no scrim, elevation 0, CornerNone /
//! Surface) or **modal** (overlay 96↔220 over a 32% scrim, elevation 2,
//! `modalExpandedShape` CornerLarge 16 / `ModalContainerColor`
//! SurfaceContainer). Compose `iconPosition` follows `railExpanded`
//! with a spatial-fast layout animation ([`item_morph`]). Modal
//! `hideOnCollapse` slides the rail offscreen instead of leaving a
//! collapsed 96/80 strip; items stay Start (`railExpanded = true`) and
//! the overlay keeps expanded shape + modal container. `Arrangement.Vertical`
//! is Top (default), Center (full container height), or Bottom
//! (remaining space below the header). Optional header (Menu / MenuOpen +
//! Compose `ExtendedFloatingActionButton(expanded = railExpanded)`)
//! stays at the top.

use super::{dialog, fab};
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
/// Expressive regular / extended FAB corner (`FabPrimaryContainerShape`).
pub const FAB_CORNER_DP: f32 = fab::CORNER_DP;
/// Compose sample header `ExtendedFloatingActionButton` icon.
pub const FAB_GLYPH: &str = "+";
/// Compose sample header `ExtendedFloatingActionButton` text.
pub const FAB_LABEL: &str = "Create";
/// Icon→label gap on the extended rail FAB.
pub const FAB_ICON_LABEL_GAP_DP: f32 = 8.0;
/// Expanded FAB leading/trailing inset (matches dest `FullWidthLeading`).
pub const FAB_PAD_EXPANDED_DP: f32 = START_LEADING_DP;
/// 32% scrim behind the expanded modal rail (same token as dialogs).
pub const SCRIM_OPACITY: f32 = dialog::SCRIM_OPACITY;
/// Compose `NavigationRailCollapsedTokens.ContainerShape` /
/// `WideNavigationRailDefaults.shape` (`CornerNone`).
pub const SHAPE_DP: f32 = 0.0;
/// Compose `WideNavigationRailDefaults.modalCollapsedShape` (= `shape`).
pub const MODAL_COLLAPSED_SHAPE_DP: f32 = SHAPE_DP;
/// Compose `NavigationRailExpandedTokens.ModalContainerShape` /
/// `WideNavigationRailDefaults.modalExpandedShape` (`CornerLarge`).
pub const MODAL_EXPANDED_SHAPE_DP: f32 = 16.0;
/// Token name for catalog / inventory (`ShapeKeyTokens.CornerLarge`).
pub const MODAL_EXPANDED_SHAPE_TOKEN: &str = "CornerLarge";
/// Token name for collapsed / standard (`ShapeKeyTokens.CornerNone`).
pub const SHAPE_TOKEN: &str = "CornerNone";

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
    /// WideNavigationRail width. Collapsed is Compose `ContainerWidth` 96
    /// (not the optional narrow 80).
    pub const fn width_dp(self) -> f32 {
        match self {
            Self::Collapsed => WIDE_COLLAPSED_WIDTH_DP,
            Self::Expanded => EXPANDED_WIDTH_DP,
        }
    }

    /// Baseline / `NarrowContainerWidth` 80 collapsed.
    pub const fn narrow_width_dp(self) -> f32 {
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
/// In-flow WideNavigationRail starts collapsed (Compose default).
pub const WIDE_DEMO_MODE: RailMode = RailMode::Collapsed;
/// Compose expanded layout: standard = in-flow, modal = overlay + scrim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailExpandedLayout {
    /// In-flow `WideNavigationRail`. No scrim, no overlay window, elevation 0.
    Standard,
    /// Modal overlay over a 32% scrim, elevation 2.
    Modal,
}

impl RailExpandedLayout {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Modal => "modal",
        }
    }

    pub const fn in_flow(self) -> bool {
        matches!(self, Self::Standard)
    }

    pub const fn uses_scrim(self) -> bool {
        matches!(self, Self::Modal)
    }
}

pub const WIDE_DEMO_LAYOUT: RailExpandedLayout = RailExpandedLayout::Standard;
pub const MODAL_DEMO_LAYOUT: RailExpandedLayout = RailExpandedLayout::Modal;
/// Live optional narrow modal (`NarrowContainerWidth` 80).
pub const NARROW_DEMO_LAYOUT: RailExpandedLayout = RailExpandedLayout::Modal;
pub const NARROW_DEMO_MODE: RailMode = RailMode::Collapsed;
/// Placeholder content that shifts when the standard rail expands in-flow.
pub const IN_FLOW_BODY: &str = "Inbox";

/// Compose collapsed width: Wide `ContainerWidth` 96 vs optional `NarrowContainerWidth` 80.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailCollapsedKind {
    /// WideNavigationRail default collapsed width (96).
    Wide,
    /// Optional `NarrowContainerWidth` (80).
    Narrow,
}

impl RailCollapsedKind {
    pub const fn width_dp(self) -> f32 {
        match self {
            Self::Wide => WIDE_COLLAPSED_WIDTH_DP,
            Self::Narrow => WIDTH_DP,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Wide => "wide",
            Self::Narrow => "narrow",
        }
    }

    pub const fn is_narrow(self) -> bool {
        matches!(self, Self::Narrow)
    }

    pub const fn data_narrow(self) -> &'static str {
        if self.is_narrow() { "1" } else { "0" }
    }
}

pub const WIDE_DEMO_KIND: RailCollapsedKind = RailCollapsedKind::Wide;
pub const NARROW_DEMO_KIND: RailCollapsedKind = RailCollapsedKind::Narrow;

/// Compose `ModalWideNavigationRail.hideOnCollapse` — slide offscreen
/// when collapsed instead of staying as a collapsed wide rail.
pub const HIDE_ON_COLLAPSE_DEFAULT: bool = false;
/// Offscreen collapsed width when `hideOnCollapse` is true.
pub const HIDE_COLLAPSED_WIDTH_DP: f32 = 0.0;
/// Scaffold menu that expands a hidden modal rail.
pub const HIDE_MENU_GLYPH: &str = "☰";
pub const HIDE_MENU_LABEL: &str = "Menu";

/// Live dismissible modal (`hideOnCollapse = true`).
pub const HIDE_DEMO_LAYOUT: RailExpandedLayout = RailExpandedLayout::Modal;
pub const HIDE_DEMO_MODE: RailMode = RailMode::Collapsed;
pub const HIDE_DEMO_HIDE_ON_COLLAPSE: bool = true;
pub const HIDE_DEMO_ARRANGEMENT: RailArrangement = RailArrangement::Center;

/// Compose `Arrangement.Vertical` / `WideNavigationRailDefaults.arrangement`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailArrangement {
    /// Default. Header stays at the top; items pack below it.
    Top,
    /// Items are centered in the full container height; header stays top.
    Center,
    /// Items pack at the bottom of the remaining space below the header.
    Bottom,
}

impl RailArrangement {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Center => "center",
            Self::Bottom => "bottom",
        }
    }

    pub const fn justify_content(self) -> &'static str {
        match self {
            Self::Top => "flex-start",
            Self::Center => "center",
            Self::Bottom => "flex-end",
        }
    }

    pub const fn is_center(self) -> bool {
        matches!(self, Self::Center)
    }

    pub const fn is_bottom(self) -> bool {
        matches!(self, Self::Bottom)
    }

    /// Center uses the full container height; Top/Bottom use space below header.
    pub const fn uses_full_height(self) -> bool {
        matches!(self, Self::Center)
    }
}

/// Compose `WideNavigationRailDefaults.arrangement` = `Arrangement.Top`.
pub const DEFAULT_ARRANGEMENT: RailArrangement = RailArrangement::Top;

/// Compose `NavigationRailBaselineItemTokens.HeaderSpaceMinimum`
/// (`WNRHeaderPadding`). Applied under the header when arrangement is Top.
pub const HEADER_SPACE_DP: f32 = 40.0;
/// Official sample `IconButton` start padding on the header.
pub const HEADER_PAD_START_DP: f32 = 24.0;
/// Default S icon button (`IconButtonTokens` container).
pub const HEADER_BUTTON_DP: f32 = 40.0;
/// Compose sample `Icons.Filled.Menu` / collapsed header.
pub const HEADER_MENU_GLYPH: &str = "☰";
/// Compose sample `Icons.AutoMirrored.Filled.MenuOpen` / expanded header.
pub const HEADER_MENU_OPEN_GLYPH: &str = "☰←";
/// Compose sample tooltip / `headerDescription` when collapsed.
pub const HEADER_EXPAND_LABEL: &str = "Expand rail";
/// Compose sample tooltip / `headerDescription` when expanded.
pub const HEADER_COLLAPSE_LABEL: &str = "Collapse rail";
/// Compose sample `stateDescription` when collapsed.
pub const HEADER_STATE_COLLAPSED: &str = "Collapsed";
/// Compose sample `stateDescription` when expanded.
pub const HEADER_STATE_EXPANDED: &str = "Expanded";
/// Tooltip anchor for the header menu (`TooltipAnchorPosition.Above`).
pub const HEADER_TOOLTIP_ABOVE: &str = "above";

/// Live standard WideNavigationRail with header + `Arrangement.Bottom`.
pub const HEADER_DEMO_LAYOUT: RailExpandedLayout = RailExpandedLayout::Standard;
pub const HEADER_DEMO_MODE: RailMode = RailMode::Collapsed;
pub const HEADER_DEMO_ARRANGEMENT: RailArrangement = RailArrangement::Bottom;
pub const HEADER_DEMO_HAS_HEADER: bool = true;
/// Official “menu and FAB” arrangement: Menu + Extended FAB in the header.
pub const HEADER_DEMO_HAS_FAB: bool = true;
/// Gap between the menu row and the header FAB.
pub const HEADER_FAB_GAP_DP: f32 = DEST_GAP_DP;

/// Compose sample Menu ↔ MenuOpen glyph.
pub fn header_menu_glyph(expanded: bool) -> &'static str {
    if expanded {
        HEADER_MENU_OPEN_GLYPH
    } else {
        HEADER_MENU_GLYPH
    }
}

/// Compose sample "Expand rail" / "Collapse rail".
pub fn header_menu_label(expanded: bool) -> &'static str {
    if expanded {
        HEADER_COLLAPSE_LABEL
    } else {
        HEADER_EXPAND_LABEL
    }
}

/// Compose sample `stateDescription` Expanded / Collapsed.
pub fn header_state_description(expanded: bool) -> &'static str {
    if expanded {
        HEADER_STATE_EXPANDED
    } else {
        HEADER_STATE_COLLAPSED
    }
}

/// Collapsed FAB side inset: center the 56dp slot in `collapsed_width`.
pub fn fab_margin_collapsed_dp(collapsed_width_dp: f32) -> f32 {
    ((collapsed_width_dp - FAB_SLOT_DP) / 2.0).max(0.0)
}

/// Side inset for the rail FAB. Collapsed is centered; expanded is 16dp.
pub fn fab_margin_dp(e: f32, collapsed_width_dp: f32) -> f32 {
    lerp(
        fab_margin_collapsed_dp(collapsed_width_dp),
        FAB_PAD_EXPANDED_DP,
        e.clamp(0.0, 1.0),
    )
}

/// FAB width: rail minus leading + trailing insets (56 collapsed, ~188 expanded).
pub fn fab_extended_width_dp(rail_width_dp: f32, margin_dp: f32) -> f32 {
    (rail_width_dp - margin_dp * 2.0).max(FAB_SLOT_DP)
}

/// Compose `ExtendedFloatingActionButton(expanded = railExpanded)` metrics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailFabMorph {
    pub t: f32,
    pub width_dp: f32,
    pub height_dp: f32,
    pub radius_dp: f32,
    pub margin_start_dp: f32,
    pub pad_h_dp: f32,
    pub gap_dp: f32,
    pub label_alpha: f32,
    pub glyph: &'static str,
    pub label: &'static str,
}

/// Interpolate Regular 56 → Extended (rail − 32) with spatial-fast easing.
/// `rail_width_dp` is the current (already width-lerped) rail width.
pub fn fab_morph(theme: &Theme, t: f32, rail_width_dp: f32) -> RailFabMorph {
    fab_morph_kind(theme, t, rail_width_dp, RailCollapsedKind::Wide)
}

/// Regular → Extended FAB for Wide 96 or optional narrow 80 collapsed.
pub fn fab_morph_kind(
    theme: &Theme,
    t: f32,
    rail_width_dp: f32,
    collapsed: RailCollapsedKind,
) -> RailFabMorph {
    let e = icon_position_eased(theme, t.clamp(0.0, 1.0));
    let margin = fab_margin_dp(e, collapsed.width_dp());
    let width = fab_extended_width_dp(rail_width_dp, margin);
    RailFabMorph {
        t: e,
        width_dp: width,
        height_dp: FAB_SLOT_DP,
        radius_dp: FAB_CORNER_DP,
        margin_start_dp: margin,
        pad_h_dp: FAB_PAD_EXPANDED_DP,
        gap_dp: lerp(0.0, FAB_ICON_LABEL_GAP_DP, e),
        label_alpha: e,
        glyph: FAB_GLYPH,
        label: FAB_LABEL,
    }
}

pub fn fab_morph_for_mode(
    theme: &Theme,
    mode: RailMode,
    rail_width_dp: f32,
    collapsed: RailCollapsedKind,
) -> RailFabMorph {
    fab_morph_kind(
        theme,
        icon_position_t(matches!(mode, RailMode::Expanded)),
        rail_width_dp,
        collapsed,
    )
}

/// Header gap under the slot. Compose applies `WNRHeaderPadding` when the
/// header is present and arrangement is Top.
pub fn header_space_dp(has_header: bool, arrangement: RailArrangement) -> f32 {
    if has_header && matches!(arrangement, RailArrangement::Top) {
        HEADER_SPACE_DP
    } else {
        0.0
    }
}

pub fn hide_on_collapse_for(layout: RailExpandedLayout, hide: bool) -> bool {
    hide && layout == RailExpandedLayout::Modal
}

/// Collapsed rail stays on screen unless `hideOnCollapse`.
pub fn collapsed_visible(hide_on_collapse: bool) -> bool {
    !hide_on_collapse
}

/// Compose sample sets `railExpanded = true` for hide-on-collapse items.
pub fn icon_position_for_hide(expanded: bool, hide_on_collapse: bool) -> IconPosition {
    if hide_on_collapse {
        IconPosition::Start
    } else {
        icon_position_for(expanded)
    }
}

pub fn icon_position_for_hide_mode(mode: RailMode, hide_on_collapse: bool) -> IconPosition {
    icon_position_for_hide(matches!(mode, RailMode::Expanded), hide_on_collapse)
}

/// Width 0↔220 when the modal rail hides on collapse.
pub fn morph_width_hide_dp(t: f32) -> f32 {
    morph_width_between(HIDE_COLLAPSED_WIDTH_DP, EXPANDED_WIDTH_DP, t)
}

/// `translateX`: −width when hidden, 0 when shown.
pub fn hide_slide_offset_for(width_dp: f32, t: f32) -> f32 {
    -width_dp * (1.0 - t.clamp(0.0, 1.0))
}

/// `translateX`: −220 when hidden, 0 when shown.
pub fn hide_slide_offset_dp(t: f32) -> f32 {
    hide_slide_offset_for(EXPANDED_WIDTH_DP, t)
}

pub fn hide_slide_offset_eased(theme: &Theme, t: f32) -> f32 {
    hide_slide_offset_dp(icon_position_eased(theme, t))
}

pub fn morph_width_eased_hide(theme: &Theme, t: f32) -> f32 {
    morph_width_between(
        HIDE_COLLAPSED_WIDTH_DP,
        EXPANDED_WIDTH_DP,
        icon_position_eased(theme, t),
    )
}

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
    resolve_mode_kind(theme, mode, RailCollapsedKind::Wide)
}

/// Wide 96 or optional narrow 80 collapsed width.
pub fn resolve_mode_kind(
    theme: &Theme,
    mode: RailMode,
    collapsed: RailCollapsedKind,
) -> NavRailAppearance {
    let mut a = resolve(theme);
    a.width_dp = match mode {
        RailMode::Collapsed => collapsed.width_dp(),
        RailMode::Expanded => EXPANDED_WIDTH_DP,
    };
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

/// Compose `NavigationRailCollapsedTokens.ContainerColor` (Surface).
pub fn collapsed_container_color(theme: &Theme) -> Argb {
    theme.color.surface
}

/// Compose `NavigationRailExpandedTokens.ModalContainerColor` (SurfaceContainer).
pub fn modal_container_color(theme: &Theme) -> Argb {
    theme.color.surface_container
}

/// Standard in-flow stays Surface; modal expanded uses SurfaceContainer.
pub fn container_color_for(theme: &Theme, layout: RailExpandedLayout, expanded: bool) -> Argb {
    if layout.uses_scrim() && expanded {
        modal_container_color(theme)
    } else {
        collapsed_container_color(theme)
    }
}

/// Modal overlay always paints expanded shape + modal container color.
pub fn hide_container_color(theme: &Theme) -> Argb {
    modal_container_color(theme)
}

/// `WideNavigationRailDefaults.shape` / collapsed modal shape.
pub fn shape_dp() -> f32 {
    SHAPE_DP
}

/// Expanded modal container corners. Standard stays 0.
pub fn shape_dp_for(layout: RailExpandedLayout, expanded: bool) -> f32 {
    if layout.uses_scrim() && expanded {
        MODAL_EXPANDED_SHAPE_DP
    } else {
        SHAPE_DP
    }
}

/// `hideOnCollapse` overlay uses `modalExpandedShape` even while sliding.
pub fn hide_shape_dp() -> f32 {
    MODAL_EXPANDED_SHAPE_DP
}

/// Compose container color + CornerLarge lerp for a modal expand clock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailContainerMorph {
    pub t: f32,
    pub color: Argb,
    pub corner_dp: f32,
}

/// Interpolate collapsed Surface/CornerNone → modal SurfaceContainer/CornerLarge.
/// Standard layout stays Surface / 0. `t` is linear; easing is spatial-fast.
pub fn container_morph(theme: &Theme, layout: RailExpandedLayout, t: f32) -> RailContainerMorph {
    let e = icon_position_eased(theme, t.clamp(0.0, 1.0));
    if layout.uses_scrim() {
        RailContainerMorph {
            t: e,
            color: collapsed_container_color(theme).lerp(modal_container_color(theme), e),
            corner_dp: lerp(MODAL_COLLAPSED_SHAPE_DP, MODAL_EXPANDED_SHAPE_DP, e),
        }
    } else {
        RailContainerMorph {
            t: e,
            color: collapsed_container_color(theme),
            corner_dp: SHAPE_DP,
        }
    }
}

pub fn container_morph_for_mode(
    theme: &Theme,
    layout: RailExpandedLayout,
    mode: RailMode,
) -> RailContainerMorph {
    container_morph(
        theme,
        layout,
        icon_position_t(matches!(mode, RailMode::Expanded)),
    )
}

/// Wide 96/80 + layout-aware container (modal expanded = SurfaceContainer).
pub fn resolve_layout(
    theme: &Theme,
    mode: RailMode,
    collapsed: RailCollapsedKind,
    layout: RailExpandedLayout,
) -> NavRailAppearance {
    let mut a = resolve_mode_kind(theme, mode, collapsed);
    a.container = container_color_for(theme, layout, matches!(mode, RailMode::Expanded));
    a
}

/// Scrim fill for the expanded modal rail (composited over surface).
pub fn scrim(theme: &Theme) -> Argb {
    theme
        .color
        .scrim
        .with_alpha(SCRIM_OPACITY)
        .composite_over(theme.color.surface)
}

/// Modal-overlay helper: Expanded ⇒ modal chrome. Standard in-flow uses
/// [`is_modal_for`] with [`RailExpandedLayout::Standard`].
pub fn is_modal(mode: RailMode) -> bool {
    is_modal_for(mode, RailExpandedLayout::Modal)
}

pub fn is_modal_for(mode: RailMode, layout: RailExpandedLayout) -> bool {
    mode == RailMode::Expanded && layout == RailExpandedLayout::Modal
}

/// Expanded rail traps destination focus; scrim tap dismisses (catalog overlay).
pub fn focus_trapped(mode: RailMode) -> bool {
    focus_trapped_for(mode, RailExpandedLayout::Modal)
}

pub fn focus_trapped_for(mode: RailMode, layout: RailExpandedLayout) -> bool {
    is_modal_for(mode, layout)
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
    overlay_window_for(mode, RailExpandedLayout::Modal)
}

pub fn overlay_window_for(mode: RailMode, layout: RailExpandedLayout) -> bool {
    is_modal_for(mode, layout)
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
    rail_chrome_for(mode, RailExpandedLayout::Modal)
}

pub fn rail_chrome_for(mode: RailMode, layout: RailExpandedLayout) -> RailChrome {
    if is_modal_for(mode, layout) {
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
            WIDE_COLLAPSED_WIDTH_DP
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

/// 0 = collapsed, 1 = expanded.
pub fn morph_t(expanded: bool) -> f32 {
    if expanded { 1.0 } else { 0.0 }
}

pub fn morph_width_between(collapsed_dp: f32, expanded_dp: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    collapsed_dp + (expanded_dp - collapsed_dp) * t
}

/// WideNavigationRail width: Compose `ContainerWidth` 96 ↔ 220.
pub fn morph_width_dp(t: f32) -> f32 {
    morph_width_between(WIDE_COLLAPSED_WIDTH_DP, EXPANDED_WIDTH_DP, t)
}

/// Optional narrow collapsed 80 ↔ 220.
pub fn morph_width_narrow_dp(t: f32) -> f32 {
    morph_width_between(WIDTH_DP, EXPANDED_WIDTH_DP, t)
}

/// [`morph_width_dp`] after spatial-fast (catalog / GPUI width clock).
pub fn morph_width_eased(theme: &Theme, t: f32) -> f32 {
    morph_width_eased_kind(theme, RailCollapsedKind::Wide, t)
}

/// Spatial-fast width for Wide 96 or optional narrow 80 collapsed.
pub fn morph_width_eased_kind(theme: &Theme, collapsed: RailCollapsedKind, t: f32) -> f32 {
    morph_width_between(
        collapsed.width_dp(),
        EXPANDED_WIDTH_DP,
        icon_position_eased(theme, t),
    )
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linear 0 = Top / collapsed, 1 = Start / expanded.
pub fn icon_position_t(expanded: bool) -> f32 {
    if expanded { 1.0 } else { 0.0 }
}

/// Spatial-fast Top→Start layout progress. Clamped to `[0, 1]`.
pub fn icon_position_eased(theme: &Theme, t: f32) -> f32 {
    theme
        .motion
        .spatial_fast_at(t.clamp(0.0, 1.0))
        .clamp(0.0, 1.0)
}

/// Compose `iconPosition` layout animation (Top 56×32 → Start 56dp pill).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailItemMorph {
    pub t: f32,
    pub icon_position: IconPosition,
    pub dest_ml_dp: f32,
    pub dest_width_dp: f32,
    pub dest_height_dp: f32,
    pub dest_pad_start_dp: f32,
    pub dest_pad_end_dp: f32,
    pub dest_radius_dp: f32,
    pub dest_indicator_alpha: f32,
    pub icon_box_w_dp: f32,
    pub icon_box_h_dp: f32,
    pub icon_left_dp: f32,
    pub icon_top_dp: f32,
    pub icon_indicator_alpha: f32,
    pub icon_label_gap_dp: f32,
    pub label_size_sp: f32,
    pub label_line_sp: f32,
    pub label_left_dp: f32,
    pub label_top_dp: f32,
    pub label_width_dp: f32,
    pub label_center: bool,
    pub badge_right_dp: f32,
}

/// Interpolate Top→Start metrics. `t` is linear time in `[0, 1]`; easing is
/// spatial-fast (same clock as [`morph_width_eased`]).
pub fn item_morph(theme: &Theme, t: f32, rail_width_dp: f32) -> RailItemMorph {
    let t = t.clamp(0.0, 1.0);
    let e = icon_position_eased(theme, t);
    let top = item_metrics(theme, IconPosition::Top);
    let start = item_metrics(theme, IconPosition::Start);
    let dest_ml = lerp(0.0, START_LEADING_DP, e);
    let dest_pad_start = lerp(0.0, START_LEADING_DP, e);
    let dest_pad_end = lerp(0.0, START_TRAILING_DP, e);
    let dest_width = (rail_width_dp - dest_ml * 2.0).max(0.0);
    let top_h = INDICATOR_H_DP + TOP_ICON_LABEL_GAP_DP + top.label_style.line_height_sp;
    let dest_height = lerp(top_h, START_INDICATOR_H_DP, e);
    let icon_box_w = lerp(INDICATOR_W_DP, ICON_DP, e);
    let icon_box_h = lerp(INDICATOR_H_DP, ICON_DP, e);
    let icon_left = lerp(
        (dest_width - INDICATOR_W_DP).max(0.0) / 2.0,
        dest_pad_start,
        e,
    );
    let icon_top = lerp(0.0, (dest_height - ICON_DP).max(0.0) / 2.0, e);
    let label_size = lerp(top.label_style.size_sp, start.label_style.size_sp, e);
    let label_line = lerp(
        top.label_style.line_height_sp,
        start.label_style.line_height_sp,
        e,
    );
    let label_left = lerp(0.0, dest_pad_start + ICON_DP + START_ICON_LABEL_GAP_DP, e);
    let label_top = lerp(
        INDICATOR_H_DP + TOP_ICON_LABEL_GAP_DP,
        (dest_height - label_line).max(0.0) / 2.0,
        e,
    );
    let label_width = (dest_width - label_left - dest_pad_end).max(0.0);
    RailItemMorph {
        t: e,
        icon_position: if e >= 0.5 {
            IconPosition::Start
        } else {
            IconPosition::Top
        },
        dest_ml_dp: dest_ml,
        dest_width_dp: dest_width,
        dest_height_dp: dest_height,
        dest_pad_start_dp: dest_pad_start,
        dest_pad_end_dp: dest_pad_end,
        dest_radius_dp: dest_height / 2.0,
        dest_indicator_alpha: e,
        icon_box_w_dp: icon_box_w,
        icon_box_h_dp: icon_box_h,
        icon_left_dp: icon_left,
        icon_top_dp: icon_top,
        icon_indicator_alpha: 1.0 - e,
        icon_label_gap_dp: lerp(top.icon_label_gap_dp, start.icon_label_gap_dp, e),
        label_size_sp: label_size,
        label_line_sp: label_line,
        label_left_dp: label_left,
        label_top_dp: label_top,
        label_width_dp: label_width,
        label_center: e < 0.5,
        badge_right_dp: lerp(18.0, 8.0, e),
    }
}

pub fn item_morph_for_mode(theme: &Theme, mode: RailMode, rail_width_dp: f32) -> RailItemMorph {
    item_morph(
        theme,
        icon_position_t(matches!(mode, RailMode::Expanded)),
        rail_width_dp,
    )
}

pub fn scrim_opacity_at(t: f32) -> f32 {
    SCRIM_OPACITY * t.clamp(0.0, 1.0)
}

pub fn scrim_opacity_for(layout: RailExpandedLayout, t: f32) -> f32 {
    if layout.uses_scrim() {
        scrim_opacity_at(t)
    } else {
        0.0
    }
}

pub fn morph_ms(theme: &Theme) -> u16 {
    theme.motion.spatial_fast_ms
}

pub fn elevation_dp_at(t: f32, theme: &Theme) -> f32 {
    theme.elevation.level2 * t.clamp(0.0, 1.0)
}

pub fn elevation_dp_for(layout: RailExpandedLayout, t: f32, theme: &Theme) -> f32 {
    if layout.uses_scrim() {
        elevation_dp_at(t, theme)
    } else {
        0.0
    }
}
