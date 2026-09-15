//! Expressive app bars (small + medium/large flexible + search).
//! Specs: https://m3.material.io/components/app-bars/specs
//!
//! Medium/large **flexible** bars replace the deprecated baseline medium/large
//! variants. They compress into the 64dp small bar on scroll. Search is the
//! fourth Expressive variant (56dp contained field inside a 64dp bar).

use crate::argb::Argb;
use crate::components::photo_stub::PhotoKind;
use crate::components::search;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 64.0;
pub const PAD_H_DP: f32 = 4.0;
pub const ICON_ROW_H_DP: f32 = 64.0;
pub const ICON_DP: f32 = 24.0;

/// Compose `TopAppBarDefaults` Expressive flexible heights.
pub const MEDIUM_EXPANDED_DP: f32 = 112.0;
pub const MEDIUM_SUBTITLE_EXPANDED_DP: f32 = 136.0;
pub const LARGE_EXPANDED_DP: f32 = 120.0;
pub const LARGE_SUBTITLE_EXPANDED_DP: f32 = 152.0;
pub const SEARCH_FIELD_H_DP: f32 = search::HEIGHT_DP;
pub const SEARCH_BAR_H_DP: f32 = 64.0;

/// Official-style album phone (decoded Bloom JPEG under a large flexible bar).
pub const SCENE_TITLE: &str = "Bloom";
pub const SCENE_SUBTITLE: &str = "April 12 – 16";
pub const SCENE_LEADING: &str = "←";
pub const SCENE_TRAILING: [&str; 2] = ["♡", "↗"];
pub const SCENE_PHOTO: PhotoKind = PhotoKind::Bloom;
pub const SCENE_VARIANT: AppBarVariant = AppBarVariant::LargeFlexible;
pub const STATUS_TIME: &str = "9:30";
pub const STATUS_H_DP: f32 = 24.0;
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 560.0;
pub const PHONE_CORNER_DP: f32 = 36.0;
pub const PHOTO_H_DP: f32 = 220.0;

pub const MEDIUM_SCENE_TITLE: &str = "Saved";
pub const MEDIUM_SCENE_SUBTITLE: &str = "12 albums";
pub const SEARCH_PLACEHOLDER: &str = search::PLACEHOLDER;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppBarVariant {
    Small,
    MediumFlexible,
    LargeFlexible,
    Search,
}

impl AppBarVariant {
    pub const ALL: [Self; 4] = [
        Self::Small,
        Self::MediumFlexible,
        Self::LargeFlexible,
        Self::Search,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::MediumFlexible => "medium-flexible",
            Self::LargeFlexible => "large-flexible",
            Self::Search => "search",
        }
    }

    pub const fn expanded_height_dp(self, has_subtitle: bool) -> f32 {
        match (self, has_subtitle) {
            (Self::Small | Self::Search, _) => HEIGHT_DP,
            (Self::MediumFlexible, false) => MEDIUM_EXPANDED_DP,
            (Self::MediumFlexible, true) => MEDIUM_SUBTITLE_EXPANDED_DP,
            (Self::LargeFlexible, false) => LARGE_EXPANDED_DP,
            (Self::LargeFlexible, true) => LARGE_SUBTITLE_EXPANDED_DP,
        }
    }

    pub const fn collapsed_height_dp(self) -> f32 {
        HEIGHT_DP
    }

    pub const fn collapses(self) -> bool {
        matches!(self, Self::MediumFlexible | Self::LargeFlexible)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TitleAlign {
    Start,
    Center,
}

impl TitleAlign {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TopAppBarAppearance {
    pub variant: AppBarVariant,
    pub height_dp: f32,
    pub collapsed_height_dp: f32,
    pub expanded_height_dp: f32,
    pub collapse: f32,
    pub container: Argb,
    pub title: Argb,
    pub subtitle: Argb,
    pub icon: Argb,
    pub title_style: TypeStyle,
    pub subtitle_style: TypeStyle,
    pub elevation_dp: f32,
    pub title_align: TitleAlign,
    pub has_subtitle: bool,
    pub search_field_h_dp: f32,
}

pub fn resolve(theme: &Theme) -> TopAppBarAppearance {
    resolve_variant(theme, AppBarVariant::Small, None, 0.0, false)
}

pub fn resolve_variant(
    theme: &Theme,
    variant: AppBarVariant,
    subtitle: Option<&str>,
    collapse: f32,
    scrolled: bool,
) -> TopAppBarAppearance {
    let t = collapse.clamp(0.0, 1.0);
    let has_subtitle = subtitle.map(|s| !s.is_empty()).unwrap_or(false);
    let expanded = variant.expanded_height_dp(has_subtitle);
    let collapsed = variant.collapsed_height_dp();
    let height = if variant.collapses() {
        lerp(expanded, collapsed, t)
    } else {
        expanded
    };
    let filling = scrolled || t > 0.001;
    let (container, elevation_dp) = if filling {
        (theme.color.surface_container, theme.elevation.level2)
    } else {
        (theme.color.surface, theme.elevation.level0)
    };
    let (title_exp, title_col, sub_exp, sub_col) = typography_pair(theme, variant);
    TopAppBarAppearance {
        variant,
        height_dp: height,
        collapsed_height_dp: collapsed,
        expanded_height_dp: expanded,
        collapse: t,
        container,
        title: theme.color.on_surface,
        subtitle: theme.color.on_surface_variant,
        icon: theme.color.on_surface,
        title_style: lerp_style(title_exp, title_col, t),
        subtitle_style: lerp_style(sub_exp, sub_col, t),
        elevation_dp,
        title_align: TitleAlign::Start,
        has_subtitle,
        search_field_h_dp: SEARCH_FIELD_H_DP,
    }
}

pub fn resolve_scene(theme: &Theme, collapse: f32) -> TopAppBarAppearance {
    resolve_variant(
        theme,
        SCENE_VARIANT,
        Some(SCENE_SUBTITLE),
        collapse,
        collapse > 0.001,
    )
}

pub fn resolve_medium_scene(theme: &Theme, collapse: f32) -> TopAppBarAppearance {
    resolve_variant(
        theme,
        AppBarVariant::MediumFlexible,
        Some(MEDIUM_SCENE_SUBTITLE),
        collapse,
        collapse > 0.001,
    )
}

pub fn resolve_search(theme: &Theme) -> TopAppBarAppearance {
    resolve_variant(theme, AppBarVariant::Search, None, 0.0, false)
}

fn typography_pair(
    theme: &Theme,
    variant: AppBarVariant,
) -> (TypeStyle, TypeStyle, TypeStyle, TypeStyle) {
    let t = theme.typography;
    match variant {
        AppBarVariant::Small => (t.title_large, t.title_large, t.label_medium, t.label_medium),
        AppBarVariant::MediumFlexible => {
            (t.headline_medium, t.title_large, t.label_large, t.label_medium)
        }
        AppBarVariant::LargeFlexible => {
            (t.display_small, t.title_large, t.title_medium, t.label_medium)
        }
        AppBarVariant::Search => (t.body_large, t.body_large, t.body_large, t.body_large),
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    a + (b - a) * t
}

pub fn lerp_style(expanded: TypeStyle, collapsed: TypeStyle, t: f32) -> TypeStyle {
    let t = t.clamp(0.0, 1.0);
    TypeStyle {
        name: if t >= 1.0 {
            collapsed.name
        } else {
            expanded.name
        },
        size_sp: lerp(expanded.size_sp, collapsed.size_sp, t),
        line_height_sp: lerp(expanded.line_height_sp, collapsed.line_height_sp, t),
        tracking_sp: lerp(expanded.tracking_sp, collapsed.tracking_sp, t),
        weight: if t >= 0.5 {
            collapsed.weight
        } else {
            expanded.weight
        },
    }
}

/// Cycle expanded → mid → collapsed (catalog / host press).
pub fn next_collapse(current: f32) -> f32 {
    if current < 0.25 {
        0.5
    } else if current < 0.75 {
        1.0
    } else {
        0.0
    }
}

pub fn collapse_attr(collapse: f32) -> &'static str {
    if collapse >= 0.999 {
        "1"
    } else if collapse >= 0.25 {
        "0.5"
    } else {
        "0"
    }
}

pub fn title_below_icons(collapse: f32) -> bool {
    collapse < 0.999
}
