//! Badge. Specs: https://m3.material.io/components/badges/specs
//! Tokens: androidx `BadgeTokens` v0_103.

use crate::argb::Argb;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const SMALL_DP: f32 = 6.0;
pub const LARGE_DP: f32 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeKind {
    Small,
    Large,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BadgeAppearance {
    pub size_dp: f32,
    pub container: Argb,
    pub label: Argb,
    pub label_style: TypeStyle,
}

pub fn resolve(theme: &Theme, kind: BadgeKind) -> BadgeAppearance {
    let c = theme.color;
    match kind {
        BadgeKind::Small => BadgeAppearance {
            size_dp: SMALL_DP,
            container: c.error,
            label: c.on_error,
            label_style: theme.typography.label_small,
        },
        BadgeKind::Large => BadgeAppearance {
            size_dp: LARGE_DP,
            container: c.error,
            label: c.on_error,
            label_style: theme.typography.label_small,
        },
    }
}

/// M3 large badges show 999+ above 999.
pub fn label_for_count(count: u32) -> String {
    if count > 999 {
        "999+".into()
    } else {
        count.to_string()
    }
}
