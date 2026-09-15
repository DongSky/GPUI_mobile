//! Split button — M3 Expressive (May 2025).
//! Specs: https://m3.material.io/components/split-button/specs
//! Tokens: androidx `SplitButtonSmallTokens` 18_0_18 (+ size variants).
//!
//! Leading primary action + trailing menu button, 2dp gap. Outer corners
//! are full-round; inner corners morph on press. Opening the menu rounds
//! the trailing button to full and keeps a pressed state layer — container
//! colors do not change on selection.

use crate::components::Appearance;
use crate::components::button::{self, ButtonSize, ButtonVariant};
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;

pub const GAP_DP: f32 = 2.0;
pub const TRAILING_MIN_W_DP: f32 = 48.0;
pub const CARET_CLOSED: &str = "▾";
pub const CARET_OPEN: &str = "▴";

pub const DEMO_LABEL: &str = "$7.49";
pub const DEMO_LEADING_ICON: &str = "+";
/// Trailing related actions. The popup is grouped (`menu::SPLIT_MENU_GROUPS`:
/// these two + More › Share/Save/Sort).
pub const DEMO_MENU: [&str; 2] = ["Add to cart", "Save for later"];
pub const DEMO_OPEN: bool = false;
/// Official overview: enamel-mugs product card.
pub const SCENE_TITLE: &str = "Enamel mugs";
pub const SCENE_SUBTITLE: &str = "Comes in navy, black, white, forest, cherry.";
pub const SCENE_PHOTO: crate::components::photo_stub::PhotoKind =
    crate::components::photo_stub::PhotoKind::Mugs;
pub const SCENE_MUG_COLORS: [(u8, u8, u8); 5] = [
    (0x1A, 0x23, 0x7E),
    (0x21, 0x21, 0x21),
    (0xFA, 0xFA, 0xFA),
    (0x1B, 0x5E, 0x20),
    (0xC6, 0x28, 0x28),
];
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 420.0;
pub const PHONE_CORNER_DP: f32 = 28.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitButtonVariant {
    Filled,
    Tonal,
    Elevated,
    Outlined,
}

impl SplitButtonVariant {
    pub const ALL: [Self; 4] = [Self::Filled, Self::Tonal, Self::Elevated, Self::Outlined];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Tonal => "tonal",
            Self::Elevated => "elevated",
            Self::Outlined => "outlined",
        }
    }

    pub const fn button_variant(self) -> ButtonVariant {
        match self {
            Self::Filled => ButtonVariant::Filled,
            Self::Tonal => ButtonVariant::Tonal,
            Self::Elevated => ButtonVariant::Elevated,
            Self::Outlined => ButtonVariant::Outlined,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitRole {
    Leading,
    Trailing,
}

/// Inner rest corners: extra-small (4) for XS–M, small (8) for L, medium (12) for XL.
pub const fn inner_rest_dp(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::ExtraSmall | ButtonSize::Small | ButtonSize::Medium => 4.0,
        ButtonSize::Large => 8.0,
        ButtonSize::ExtraLarge => 12.0,
    }
}

/// Inner pressed / hovered corners (`InnerPressedCorner`).
pub const fn inner_pressed_dp(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::ExtraSmall => 8.0,
        ButtonSize::Small | ButtonSize::Medium => 12.0,
        ButtonSize::Large | ButtonSize::ExtraLarge => 20.0,
    }
}

pub const fn trailing_icon_dp(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::ExtraSmall | ButtonSize::Small => 22.0,
        ButtonSize::Medium => 26.0,
        ButtonSize::Large => 38.0,
        ButtonSize::ExtraLarge => 50.0,
    }
}

pub const fn trailing_pad_h_dp(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::ExtraSmall => 10.0,
        ButtonSize::Small => 13.0,
        ButtonSize::Medium => 15.0,
        ButtonSize::Large => 26.0,
        ButtonSize::ExtraLarge => 32.0,
    }
}

pub fn corners(role: SplitRole, size: ButtonSize, pressed: bool, trailing_open: bool) -> Corners {
    let outer = size.height_dp() / 2.0;
    if trailing_open && matches!(role, SplitRole::Trailing) {
        return Corners::all(outer);
    }
    let inner = if pressed {
        inner_pressed_dp(size)
    } else {
        inner_rest_dp(size)
    };
    match role {
        SplitRole::Leading => Corners {
            top_left: outer,
            top_right: inner,
            bottom_right: inner,
            bottom_left: outer,
        },
        SplitRole::Trailing => Corners {
            top_left: inner,
            top_right: outer,
            bottom_right: outer,
            bottom_left: inner,
        },
    }
}

fn paint(
    theme: &Theme,
    variant: SplitButtonVariant,
    size: ButtonSize,
    role: SplitRole,
    pressed: bool,
    trailing_open: bool,
) -> Appearance {
    let state = if pressed || (trailing_open && matches!(role, SplitRole::Trailing)) {
        InteractionState::Pressed
    } else {
        InteractionState::Enabled
    };
    let mut appearance = button::resolve_expressive(
        theme,
        variant.button_variant(),
        size,
        button::ButtonShape::Round,
        state,
    );
    appearance.corners = corners(role, size, pressed, trailing_open);
    match role {
        SplitRole::Leading => {
            appearance.pad_end_dp = size.pad_h_dp() - 4.0;
        }
        SplitRole::Trailing => {
            appearance.min_width_dp = Some(TRAILING_MIN_W_DP.max(size.height_dp().min(48.0)));
            appearance.width_dp = Some(TRAILING_MIN_W_DP.max(size.height_dp() * 0.85));
            appearance.pad_start_dp = trailing_pad_h_dp(size);
            appearance.pad_end_dp = trailing_pad_h_dp(size);
        }
    }
    appearance
}

pub fn resolve_leading(
    theme: &Theme,
    variant: SplitButtonVariant,
    size: ButtonSize,
    pressed: bool,
) -> Appearance {
    paint(theme, variant, size, SplitRole::Leading, pressed, false)
}

pub fn resolve_trailing(
    theme: &Theme,
    variant: SplitButtonVariant,
    size: ButtonSize,
    open: bool,
) -> Appearance {
    paint(theme, variant, size, SplitRole::Trailing, false, open)
}

pub fn caret(open: bool) -> &'static str {
    if open { CARET_OPEN } else { CARET_CLOSED }
}
