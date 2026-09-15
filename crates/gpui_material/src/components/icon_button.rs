//! Icon buttons — M3 Expressive.
//! Specs: https://m3.material.io/components/icon-buttons/specs
//! Tokens: androidx Compose `*IconButtonTokens` + MDC `m3_comp_icon_button_*`
//! (XS 32 / S 40 / M 56 / L 96 / XL 136; Narrow / Default / Wide;
//! default action vs `IconToggleButton` selected colors + round↔square).

use crate::argb::Argb;
use crate::components::Appearance;
use crate::components::button::{self, ButtonShape, ButtonSize};
use crate::shape::Corners;
use crate::state::{
    DISABLED_CONTAINER_OPACITY, InteractionState, apply_state_layer, resolve_content,
};
use crate::theme::Theme;

pub const CONTAINER_DP: f32 = 40.0;
pub const ICON_DP: f32 = 24.0;
pub const TARGET_DP: f32 = 48.0;

/// Default size used by the official width-axis overview (S, 40dp).
pub const WIDTH_HERO_SIZE: ButtonSize = ButtonSize::Small;
/// Second official anatomy row (M, 56dp).
pub const WIDTH_HERO_SIZE_MEDIUM: ButtonSize = ButtonSize::Medium;
/// Official toggle overview uses the default S size.
pub const TOGGLE_HERO_SIZE: ButtonSize = ButtonSize::Small;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconButtonVariant {
    Standard,
    Filled,
    Tonal,
    Outlined,
}

impl IconButtonVariant {
    pub const ALL: [Self; 4] = [Self::Standard, Self::Filled, Self::Tonal, Self::Outlined];

    /// Spec anatomy order: filled, tonal, outlined, standard (A–D).
    pub const TOGGLE_OVERVIEW: [Self; 4] =
        [Self::Filled, Self::Tonal, Self::Outlined, Self::Standard];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Filled => "filled",
            Self::Tonal => "tonal",
            Self::Outlined => "outlined",
        }
    }
}

/// Expressive width axis (Compose `IconButtonWidthOption`).
/// `Default` is Uniform: leading + icon + trailing equals container height.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconButtonWidth {
    Narrow,
    Default,
    Wide,
}

impl IconButtonWidth {
    pub const ALL: [Self; 3] = [Self::Narrow, Self::Default, Self::Wide];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Narrow => "narrow",
            Self::Default => "default",
            Self::Wide => "wide",
        }
    }
}

/// Default action vs toggle (Compose `IconButton` / `IconToggleButton`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconButtonSelection {
    /// Non-toggle default action button.
    Default,
    /// Toggle, unselected (outlined glyph).
    Unselected,
    /// Toggle, selected (filled glyph + opposite resting shape).
    Selected,
}

impl IconButtonSelection {
    pub const TOGGLE: [Self; 2] = [Self::Unselected, Self::Selected];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Unselected => "unselected",
            Self::Selected => "selected",
        }
    }

    pub const fn is_selected(self) -> bool {
        matches!(self, Self::Selected)
    }

    /// Official: outlined icon unselected, filled icon selected.
    pub const fn glyph(self) -> &'static str {
        match self {
            Self::Unselected => "☆",
            Self::Default | Self::Selected => "★",
        }
    }
}

pub const fn container_dp(size: ButtonSize) -> f32 {
    size.height_dp()
}

pub const fn icon_dp(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::ExtraSmall => 20.0,
        ButtonSize::Small | ButtonSize::Medium => 24.0,
        ButtonSize::Large => 32.0,
        ButtonSize::ExtraLarge => 40.0,
    }
}

/// Horizontal inset from MDC `m3_comp_icon_button_*_{narrow,default,wide}_leading_space`.
pub const fn pad_h_dp(size: ButtonSize, width: IconButtonWidth) -> f32 {
    match (size, width) {
        (ButtonSize::ExtraSmall, IconButtonWidth::Narrow) => 4.0,
        (ButtonSize::ExtraSmall, IconButtonWidth::Default) => 6.0,
        (ButtonSize::ExtraSmall, IconButtonWidth::Wide) => 10.0,
        (ButtonSize::Small, IconButtonWidth::Narrow) => 4.0,
        (ButtonSize::Small, IconButtonWidth::Default) => 8.0,
        (ButtonSize::Small, IconButtonWidth::Wide) => 14.0,
        (ButtonSize::Medium, IconButtonWidth::Narrow) => 12.0,
        (ButtonSize::Medium, IconButtonWidth::Default) => 16.0,
        (ButtonSize::Medium, IconButtonWidth::Wide) => 24.0,
        (ButtonSize::Large, IconButtonWidth::Narrow) => 16.0,
        (ButtonSize::Large, IconButtonWidth::Default) => 32.0,
        (ButtonSize::Large, IconButtonWidth::Wide) => 48.0,
        (ButtonSize::ExtraLarge, IconButtonWidth::Narrow) => 32.0,
        (ButtonSize::ExtraLarge, IconButtonWidth::Default) => 48.0,
        (ButtonSize::ExtraLarge, IconButtonWidth::Wide) => 72.0,
    }
}

pub const fn container_width_dp(size: ButtonSize, width: IconButtonWidth) -> f32 {
    pad_h_dp(size, width) * 2.0 + icon_dp(size)
}

/// Selected toggle inverts the resting shape (round↔square). Press still uses
/// the shared pressed corner.
pub const fn resting_shape(shape: ButtonShape, selection: IconButtonSelection) -> ButtonShape {
    match selection {
        IconButtonSelection::Selected => shape.opposite(),
        IconButtonSelection::Default | IconButtonSelection::Unselected => shape,
    }
}

pub fn resolve(theme: &Theme, variant: IconButtonVariant, state: InteractionState) -> Appearance {
    resolve_expressive(theme, variant, ButtonSize::Small, ButtonShape::Round, state)
}

pub fn resolve_expressive(
    theme: &Theme,
    variant: IconButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
    state: InteractionState,
) -> Appearance {
    resolve_width(theme, variant, size, shape, IconButtonWidth::Default, state)
}

pub fn resolve_width(
    theme: &Theme,
    variant: IconButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
    width: IconButtonWidth,
    state: InteractionState,
) -> Appearance {
    resolve_selection(
        theme,
        variant,
        size,
        shape,
        width,
        IconButtonSelection::Default,
        state,
    )
}

/// Compose `IconToggleButton` at default (uniform) width.
pub fn resolve_toggle(
    theme: &Theme,
    variant: IconButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
    selected: bool,
    state: InteractionState,
) -> Appearance {
    resolve_selection(
        theme,
        variant,
        size,
        shape,
        IconButtonWidth::Default,
        if selected {
            IconButtonSelection::Selected
        } else {
            IconButtonSelection::Unselected
        },
        state,
    )
}

pub fn resolve_selection(
    theme: &Theme,
    variant: IconButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
    width: IconButtonWidth,
    selection: IconButtonSelection,
    state: InteractionState,
) -> Appearance {
    let c = theme.color;
    let outline_w = size.outline_dp();
    let (base, icon, outline) = colors(c, variant, selection, outline_w);
    let filled_disabled = matches!(
        (variant, selection),
        (IconButtonVariant::Filled | IconButtonVariant::Tonal, _)
            | (IconButtonVariant::Outlined, IconButtonSelection::Selected)
    );
    let (container, content, outline) = if state.is_disabled() {
        let container = if filled_disabled {
            c.on_surface
                .with_alpha(DISABLED_CONTAINER_OPACITY)
                .composite_over(c.surface)
        } else {
            c.surface
        };
        (
            container,
            resolve_content(icon, c.on_surface, c.surface, true),
            outline.map(|(_, w)| {
                (
                    c.on_surface
                        .with_alpha(DISABLED_CONTAINER_OPACITY)
                        .composite_over(c.surface),
                    w,
                )
            }),
        )
    } else {
        (
            apply_state_layer(base, icon, state.layer_opacity()),
            icon,
            outline,
        )
    };
    let height = container_dp(size);
    let icon_size = icon_dp(size);
    let pad_h = pad_h_dp(size, width);
    let container_w = container_width_dp(size, width);
    let pad_v = (height - icon_size) / 2.0;
    let paint_shape = resting_shape(shape, selection);
    Appearance {
        width_dp: Some(container_w),
        height_dp: height,
        min_width_dp: Some(TARGET_DP.max(container_w)),
        corners: Corners::all(button::corner_dp(size, paint_shape, state)),
        container,
        content,
        secondary_content: None,
        outline,
        elevation_dp: 0.0,
        pad_start_dp: pad_h,
        pad_end_dp: pad_h,
        pad_top_dp: pad_v,
        pad_bottom_dp: pad_v,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}

/// Color roles from the current m3.material.io icon-button spec table:
/// Default | Toggle unselected | Toggle selected.
fn colors(
    c: crate::color::ColorScheme,
    variant: IconButtonVariant,
    selection: IconButtonSelection,
    outline_w: f32,
) -> (Argb, Argb, Option<(Argb, f32)>) {
    match (variant, selection) {
        (IconButtonVariant::Standard, IconButtonSelection::Selected) => {
            (c.surface, c.primary, None)
        }
        (IconButtonVariant::Standard, _) => (c.surface, c.on_surface_variant, None),
        (IconButtonVariant::Filled, IconButtonSelection::Unselected) => {
            (c.surface_container, c.on_surface_variant, None)
        }
        (IconButtonVariant::Filled, _) => (c.primary, c.on_primary, None),
        (IconButtonVariant::Tonal, IconButtonSelection::Selected) => {
            (c.secondary, c.on_secondary, None)
        }
        (IconButtonVariant::Tonal, _) => (c.secondary_container, c.on_secondary_container, None),
        (IconButtonVariant::Outlined, IconButtonSelection::Selected) => {
            (c.inverse_surface, c.inverse_on_surface, None)
        }
        (IconButtonVariant::Outlined, _) => (
            c.surface,
            c.on_surface_variant,
            Some((c.outline_variant, outline_w)),
        ),
    }
}
