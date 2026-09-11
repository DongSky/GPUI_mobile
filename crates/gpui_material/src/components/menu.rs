//! Menus. Specs: https://m3.material.io/components/menus/specs
//! Tokens: androidx `MenuTokens` v0_210.

use crate::argb::Argb;
use crate::shape::Corners;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const ITEM_HEIGHT_DP: f32 = 48.0;
pub const PAD_H_DP: f32 = 12.0;
pub const CORNER_DP: f32 = 4.0;
pub const ICON_DP: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuAppearance {
    pub container: Argb,
    pub corners: Corners,
    pub elevation_dp: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuItemAppearance {
    pub height_dp: f32,
    pub container: Argb,
    pub label: Argb,
    pub icon: Argb,
    pub label_style: TypeStyle,
}

pub fn resolve_menu(theme: &Theme) -> MenuAppearance {
    MenuAppearance {
        container: theme.color.surface_container,
        corners: Corners::all(CORNER_DP),
        elevation_dp: theme.elevation.level2,
    }
}

pub fn resolve_item(theme: &Theme, selected: bool, state: InteractionState) -> MenuItemAppearance {
    let c = theme.color;
    let (base, label, icon) = if selected {
        (
            c.secondary_container,
            c.on_secondary_container,
            c.on_secondary_container,
        )
    } else {
        (c.surface_container, c.on_surface, c.on_surface_variant)
    };
    let container = if state.is_disabled() {
        c.surface_container
    } else {
        apply_state_layer(base, label, state.layer_opacity())
    };
    let label = if state.is_disabled() {
        c.on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface_container)
    } else {
        label
    };
    MenuItemAppearance {
        height_dp: ITEM_HEIGHT_DP,
        container,
        label,
        icon,
        label_style: theme.typography.body_large,
    }
}
