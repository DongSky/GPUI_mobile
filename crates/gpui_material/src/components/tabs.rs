//! Primary / secondary tabs. Specs: https://m3.material.io/components/tabs/specs
//! Tokens: androidx `PrimaryNavigationTabTokens` v0_162.

use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 48.0;
pub const HEIGHT_WITH_ICON_DP: f32 = 64.0;
pub const INDICATOR_H_PRIMARY_DP: f32 = 3.0;
pub const INDICATOR_H_SECONDARY_DP: f32 = 2.0;
pub const INDICATOR_CORNER_DP: f32 = 3.0;
/// Official primary-with-icon row (64dp).
pub const DEMO_ICONS: [&str; 3] = ["●", "○", "◐"];
pub const DEMO_ICON_LABELS: [&str; 3] = ["News", "Video", "Photos"];
/// Official overview: phone “My saved media” (Video / Photos / Audio) over tiles.
pub const SCENE_TITLE: &str = "My saved media";
pub const SCENE_ICONS: [&str; 3] = ["▶", "🖼", "♪"];
pub const SCENE_LABELS: [&str; 3] = ["Video", "Photos", "Audio"];
pub const SCENE_TILES: [&str; 6] = ["Dawn", "Peak", "Cove", "Trail", "Bloom", "Mist"];
pub const SCENE_TILE_H_DP: f32 = 96.0;
pub const SCENE_TILE_CORNER_DP: f32 = 16.0;
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 520.0;
pub const PHONE_CORNER_DP: f32 = 36.0;
pub const PHONE_BEZEL_DP: f32 = 12.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsVariant {
    Primary,
    Secondary,
}

impl TabsVariant {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsAppearance {
    pub height_dp: f32,
    pub container: Argb,
    pub active_label: Argb,
    pub inactive_label: Argb,
    pub indicator: Argb,
    pub indicator_h: f32,
    pub indicator_full_width: bool,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme, variant: TabsVariant) -> TabsAppearance {
    let c = theme.color;
    match variant {
        TabsVariant::Primary => TabsAppearance {
            height_dp: HEIGHT_DP,
            container: c.surface,
            active_label: c.primary,
            inactive_label: c.on_surface_variant,
            indicator: c.primary,
            indicator_h: INDICATOR_H_PRIMARY_DP,
            indicator_full_width: false,
            label_style: theme.typography.title_small,
        },
        TabsVariant::Secondary => TabsAppearance {
            height_dp: HEIGHT_DP,
            container: c.surface,
            active_label: c.on_surface,
            inactive_label: c.on_surface_variant,
            indicator: c.primary,
            indicator_h: INDICATOR_H_SECONDARY_DP,
            indicator_full_width: true,
            label_style: theme.typography.title_small,
        },
    }
}

/// Primary tabs with leading icons (official overview often shows this pair).
pub fn resolve_with_icons(theme: &Theme, variant: TabsVariant) -> TabsAppearance {
    let mut appearance = resolve(theme, variant);
    appearance.height_dp = HEIGHT_WITH_ICON_DP;
    appearance
}

/// Role-color photo stub for the saved-media scene.
pub fn scene_tile_fill(theme: &Theme, index: usize) -> crate::argb::Argb {
    let c = theme.color;
    match index % 3 {
        0 => c.primary_container,
        1 => c.secondary_container,
        _ => c.tertiary_container,
    }
}

pub fn scene_tile_on(theme: &Theme, index: usize) -> crate::argb::Argb {
    let c = theme.color;
    match index % 3 {
        0 => c.on_primary_container,
        1 => c.on_secondary_container,
        _ => c.on_tertiary_container,
    }
}
