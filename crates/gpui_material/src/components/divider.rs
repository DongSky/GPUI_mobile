//! Divider. Specs: https://m3.material.io/components/divider/specs

use crate::argb::Argb;
use crate::theme::Theme;

pub const THICKNESS_DP: f32 = 1.0;
pub const INSET_DP: f32 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DividerAppearance {
    pub thickness_dp: f32,
    pub color: Argb,
    pub inset_dp: f32,
}

pub fn resolve(theme: &Theme, inset: bool) -> DividerAppearance {
    DividerAppearance {
        thickness_dp: THICKNESS_DP,
        color: theme.color.outline_variant,
        inset_dp: if inset { INSET_DP } else { 0.0 },
    }
}
