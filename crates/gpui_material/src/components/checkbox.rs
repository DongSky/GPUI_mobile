//! Checkbox. Specs: https://m3.material.io/components/checkbox/specs

use crate::argb::Argb;
use crate::state::{apply_state_layer, InteractionState};
use crate::theme::Theme;

pub const CONTAINER_DP: f32 = 18.0;
pub const CORNER_DP: f32 = 2.0;
pub const TARGET_DP: f32 = 48.0;
pub const STATE_LAYER_DP: f32 = 40.0;
pub const ICON_DP: f32 = 18.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckValue {
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckValue {
    pub const ALL: [Self; 3] = [Self::Unchecked, Self::Checked, Self::Indeterminate];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unchecked => "unchecked",
            Self::Checked => "checked",
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckboxAppearance {
    pub box_size_dp: f32,
    pub corner_dp: f32,
    pub target_dp: f32,
    pub state_layer_dp: f32,
    pub box_fill: Argb,
    pub box_outline: Option<Argb>,
    pub icon: Argb,
    pub state_layer: Argb,
    pub label: Argb,
}

pub fn resolve(theme: &Theme, value: CheckValue, state: InteractionState) -> CheckboxAppearance {
    let c = theme.color;
    let selected = !matches!(value, CheckValue::Unchecked);
    let (fill, outline, icon) = if state.is_disabled() {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        if selected {
            (muted, None, c.surface)
        } else {
            (c.surface, Some(muted), Argb::TRANSPARENT)
        }
    } else if selected {
        (c.primary, None, c.on_primary)
    } else {
        (c.surface, Some(c.on_surface_variant), Argb::TRANSPARENT)
    };
    let layer_color = if selected { c.primary } else { c.on_surface };
    let state_layer = apply_state_layer(c.surface, layer_color, state.layer_opacity());
    CheckboxAppearance {
        box_size_dp: CONTAINER_DP,
        corner_dp: CORNER_DP,
        target_dp: TARGET_DP,
        state_layer_dp: STATE_LAYER_DP,
        box_fill: fill,
        box_outline: outline,
        icon,
        state_layer,
        label: if state.is_disabled() {
            c.on_surface
                .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
                .composite_over(c.surface)
        } else {
            c.on_surface
        },
    }
}
