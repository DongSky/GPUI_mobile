//! Chips — M3 Expressive (Compose 1.5 / FilterChip + ElevatedFilterChip).
//! Specs: https://m3.material.io/components/chips/specs
//! Tokens: androidx `Shapes.defaultChipShapes` + `FilterChipTokens` /
//! `ChipsTokens` / `FilterChipDefaults` (`Chip.kt`).
//!
//! Baseline Assist / Suggestion stay 32dp full-round. Expressive FilterChip
//! and InputChip morph `ChipShapes`: CornerMedium rest, CornerFull selected,
//! CornerSmall pressed. Live press interpolates those CornerBasedShapes
//! (`rememberAnimatedShape` / spatial-fast). Expressive FilterChip defaults
//! to tonal leading-icon colors (`ChipsTokens.UnselectedLeadingIconColor` =
//! on-surface-variant). ElevatedFilterChip uses surface-container-low +
//! elevation 1 and no outline. InputChip may paint a 24dp avatar (takes
//! precedence over a leading icon) with compact 4dp arrangement.

use super::photo_stub::PhotoKind;
use crate::argb::Argb;
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::{
    DISABLED_CONTAINER_OPACITY, InteractionState, apply_state_layer, resolve_content,
};
use crate::theme::Theme;

/// `FilterChipTokens.ContainerHeight` / `InputChipTokens.ContainerHeight`.
pub const HEIGHT_DP: f32 = 32.0;
/// Label-only leading/trailing (`FilterChipDefaults.ContentPadding` rest 16).
pub const PAD_H_DP: f32 = 16.0;
/// Start inset when a leading icon is shown.
pub const LEADING_PAD_START_DP: f32 = 8.0;
/// End inset when a trailing icon is shown (`InputChip` close).
pub const TRAILING_PAD_END_DP: f32 = 8.0;
/// `FilterChipDefaults.IconSize` / `InputChipDefaults.IconSize`.
pub const ICON_DP: f32 = 18.0;
/// `FilterChipDefaults.HorizontalSpacing`.
pub const ICON_GAP_DP: f32 = 8.0;
/// `FilterChipDefaults.CompactHorizontalSpacing` / InputChip avatar arrangement.
pub const COMPACT_ICON_GAP_DP: f32 = 4.0;
/// `InputChipDefaults.AvatarSize` / `InputChipTokens.AvatarSize`.
pub const AVATAR_DP: f32 = 24.0;
/// Compose `contentPadding(hasAvatar=true)` start inset.
pub const AVATAR_PAD_START_DP: f32 = 4.0;
/// Licensed camera still for the InputChip avatar hero (Compose `avatar`).
pub const INPUT_AVATAR_KIND: PhotoKind = PhotoKind::PortraitSofia;
/// Official-style InputChip person label paired with [`INPUT_AVATAR_KIND`].
pub const INPUT_AVATAR_LABEL: &str = "Sofia";
/// Compose `Shapes.defaultChipShapes.shape` = `ShapeTokens.CornerMedium`.
pub const UNSELECTED_CORNER_DP: f32 = 12.0;
/// `selectedShape` = `ShapeTokens.CornerFull` (half of 32dp).
pub const SELECTED_CORNER_DP: f32 = 16.0;
/// `pressedShape` = `ShapeTokens.CornerSmall`.
pub const PRESSED_CORNER_DP: f32 = 8.0;
/// `FilterChipTokens.ElevatedContainerElevation`.
pub const ELEVATED_DP: f32 = 1.0;
/// Selected FilterChip leading check (official anatomy).
pub const CHECK_GLYPH: &str = "✓";
/// InputChip trailing dismiss.
pub const CLOSE_GLYPH: &str = "×";
/// Unselected FilterChip leading stand-in (catalog tonal/elevated hero).
pub const LEADING_GLYPH: &str = "◈";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChipVariant {
    Assist,
    Filter,
    Input,
    Suggestion,
}

impl ChipVariant {
    pub const ALL: [Self; 4] = [Self::Assist, Self::Filter, Self::Input, Self::Suggestion];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Assist => "assist",
            Self::Filter => "filter",
            Self::Input => "input",
            Self::Suggestion => "suggestion",
        }
    }

    /// Compose Expressive overloads: FilterChip / ElevatedFilterChip / InputChip.
    pub const fn morphs(self) -> bool {
        matches!(self, Self::Filter | Self::Input)
    }
}

/// Compose `FilterChipDefaults` color factories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChipColor {
    /// Flat outlined (`filterChipColors` + `filterChipBorder`).
    Flat,
    /// Expressive FilterChip (`tonalFilterChipColors`): leading on-surface-variant.
    Tonal,
    /// ElevatedFilterChip (`elevatedFilterChipColors`): surface-container-low, elev 1.
    Elevated,
    /// Expressive ElevatedFilterChip (`tonalElevatedFilterChipColors`).
    TonalElevated,
}

impl ChipColor {
    pub const ALL: [Self; 4] = [Self::Flat, Self::Tonal, Self::Elevated, Self::TonalElevated];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Tonal => "tonal",
            Self::Elevated => "elevated",
            Self::TonalElevated => "tonal-elevated",
        }
    }

    pub const fn elevated(self) -> bool {
        matches!(self, Self::Elevated | Self::TonalElevated)
    }

    pub const fn tonal(self) -> bool {
        matches!(self, Self::Tonal | Self::TonalElevated)
    }
}

/// Official filter overview labels (m3 chips / amenities row).
pub const FILTER_HERO: [ChipDemo; 3] = [
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::Flat,
        label: "Elevator",
        selected: false,
        state: InteractionState::Enabled,
        leading: None,
        avatar: None,
    },
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::Flat,
        label: "Washer",
        selected: true,
        state: InteractionState::Enabled,
        leading: None,
        avatar: None,
    },
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::Flat,
        label: "Pets",
        selected: false,
        state: InteractionState::Pressed,
        leading: None,
        avatar: None,
    },
];

/// Expressive ElevatedFilterChip + tonal leading-icon defaults.
pub const ELEVATED_FILTER_HERO: [ChipDemo; 3] = [
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::TonalElevated,
        label: "Elevator",
        selected: false,
        state: InteractionState::Enabled,
        leading: Some(LEADING_GLYPH),
        avatar: None,
    },
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::TonalElevated,
        label: "Washer",
        selected: true,
        state: InteractionState::Enabled,
        leading: None,
        avatar: None,
    },
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::TonalElevated,
        label: "Pets",
        selected: false,
        state: InteractionState::Pressed,
        leading: Some(LEADING_GLYPH),
        avatar: None,
    },
];

/// Tonal (outlined) FilterChip pair so leading-icon color is visible.
pub const TONAL_FILTER_HERO: [ChipDemo; 2] = [
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::Tonal,
        label: "Wifi",
        selected: false,
        state: InteractionState::Enabled,
        leading: Some(LEADING_GLYPH),
        avatar: None,
    },
    ChipDemo {
        variant: ChipVariant::Filter,
        color: ChipColor::Tonal,
        label: "Wifi",
        selected: true,
        state: InteractionState::Enabled,
        leading: None,
        avatar: None,
    },
];

/// Official input overview (place + selected place).
pub const INPUT_HERO: [ChipDemo; 2] = [
    ChipDemo {
        variant: ChipVariant::Input,
        color: ChipColor::Flat,
        label: "Portland",
        selected: false,
        state: InteractionState::Enabled,
        leading: None,
        avatar: None,
    },
    ChipDemo {
        variant: ChipVariant::Input,
        color: ChipColor::Flat,
        label: "Portland",
        selected: true,
        state: InteractionState::Enabled,
        leading: None,
        avatar: None,
    },
];

/// Compose `InputChip(avatar = …)` — 24dp still + compact 4dp arrangement.
pub const INPUT_AVATAR_HERO: [ChipDemo; 2] = [
    ChipDemo {
        variant: ChipVariant::Input,
        color: ChipColor::Flat,
        label: INPUT_AVATAR_LABEL,
        selected: false,
        state: InteractionState::Enabled,
        leading: None,
        avatar: Some(INPUT_AVATAR_KIND),
    },
    ChipDemo {
        variant: ChipVariant::Input,
        color: ChipColor::Flat,
        label: INPUT_AVATAR_LABEL,
        selected: true,
        state: InteractionState::Enabled,
        leading: None,
        avatar: Some(INPUT_AVATAR_KIND),
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChipDemo {
    pub variant: ChipVariant,
    pub color: ChipColor,
    pub label: &'static str,
    pub selected: bool,
    pub state: InteractionState,
    pub leading: Option<&'static str>,
    /// InputChip `avatar` slot. Takes precedence over [`Self::leading`].
    pub avatar: Option<PhotoKind>,
}

impl ChipDemo {
    pub const fn has_avatar(self) -> bool {
        self.avatar.is_some() && matches!(self.variant, ChipVariant::Input)
    }
}

/// Discrete Compose `ChipShapes` corner (rest / selected / pressed).
pub fn corner_dp(
    theme: &Theme,
    variant: ChipVariant,
    selected: bool,
    state: InteractionState,
) -> f32 {
    animated_corner_dp(theme, variant, selected, press_t(state))
}

/// `1` while pressed, else `0` — catalog / hosts feed this into
/// [`animated_corner_dp`] (`rememberAnimatedShape`).
pub fn press_t(state: InteractionState) -> f32 {
    if matches!(state, InteractionState::Pressed) {
        1.0
    } else {
        0.0
    }
}

/// Linear press clock for a host `with_animation` delta (`1` = pressed).
pub fn press_t_anim(pressed: bool, delta: f32) -> f32 {
    let delta = delta.clamp(0.0, 1.0);
    if pressed { delta } else { 1.0 - delta }
}

/// Compose `rememberAnimatedShape` duration (spatial-fast).
pub fn press_ms(theme: &Theme) -> u16 {
    theme.motion.spatial_fast_ms
}

/// Rest or selected `ChipShapes` corner (pressed is applied by [`animated_corner_dp`]).
pub fn rest_corner_dp(theme: &Theme, variant: ChipVariant, selected: bool) -> f32 {
    if !variant.morphs() {
        HEIGHT_DP / 2.0
    } else if selected {
        SELECTED_CORNER_DP
    } else {
        theme.shapes.medium
    }
}

/// Compose `rememberAnimatedShape`: spatial-fast lerp from rest/selected toward
/// `pressedShape` (CornerSmall 8). `press_t` is linear time in `[0, 1]`.
pub fn animated_corner_dp(
    theme: &Theme,
    variant: ChipVariant,
    selected: bool,
    press_t: f32,
) -> f32 {
    let rest = rest_corner_dp(theme, variant, selected);
    if !variant.morphs() {
        return rest;
    }
    let pressed = theme.shapes.small;
    let t = theme.motion.spatial_fast_at(press_t.clamp(0.0, 1.0));
    let lo = rest.min(pressed);
    let hi = rest.max(pressed);
    (rest + (pressed - rest) * t).clamp(lo, hi)
}

/// Selected FilterChip shows a leading check; demos may supply a rest icon.
pub fn leading_icon(variant: ChipVariant, selected: bool) -> Option<&'static str> {
    leading_icon_with(variant, selected, None)
}

pub fn leading_icon_with(
    variant: ChipVariant,
    selected: bool,
    leading: Option<&'static str>,
) -> Option<&'static str> {
    if matches!(variant, ChipVariant::Filter) && selected {
        Some(CHECK_GLYPH)
    } else {
        leading
    }
}

pub fn demo_leading_icon(demo: ChipDemo) -> Option<&'static str> {
    if demo.has_avatar() {
        None
    } else {
        leading_icon_with(demo.variant, demo.selected, demo.leading)
    }
}

pub fn demo_avatar(demo: ChipDemo) -> Option<PhotoKind> {
    if demo.has_avatar() { demo.avatar } else { None }
}

/// `HorizontalSpacing` (8) or `CompactHorizontalSpacing` (4) when an avatar is shown.
pub fn icon_gap_dp(has_avatar: bool) -> f32 {
    if has_avatar {
        COMPACT_ICON_GAP_DP
    } else {
        ICON_GAP_DP
    }
}

pub fn demo_icon_gap_dp(demo: ChipDemo) -> f32 {
    icon_gap_dp(demo.has_avatar())
}

/// InputChip shows a trailing close (selected or not).
pub fn trailing_icon(variant: ChipVariant) -> Option<&'static str> {
    if matches!(variant, ChipVariant::Input) {
        Some(CLOSE_GLYPH)
    } else {
        None
    }
}

pub fn pad_start_dp(variant: ChipVariant, selected: bool) -> f32 {
    pad_start_with(variant, selected, None)
}

pub fn pad_start_with(variant: ChipVariant, selected: bool, leading: Option<&'static str>) -> f32 {
    pad_start_avatar(variant, selected, leading, None)
}

pub fn pad_start_avatar(
    variant: ChipVariant,
    selected: bool,
    leading: Option<&'static str>,
    avatar: Option<PhotoKind>,
) -> f32 {
    if avatar.is_some() && matches!(variant, ChipVariant::Input) {
        AVATAR_PAD_START_DP
    } else if leading_icon_with(variant, selected, leading).is_some() {
        LEADING_PAD_START_DP
    } else {
        PAD_H_DP
    }
}

pub fn pad_end_dp(variant: ChipVariant) -> f32 {
    if trailing_icon(variant).is_some() {
        TRAILING_PAD_END_DP
    } else {
        PAD_H_DP
    }
}

/// `FilterChipTokens.UnselectedLeadingIconColor` = primary;
/// tonal / `ChipsTokens.UnselectedLeadingIconColor` = on-surface-variant.
/// Trailing (InputChip close) stays on-surface-variant when unselected.
pub fn leading_icon_color(theme: &Theme, color: ChipColor, selected: bool) -> Argb {
    icon_color(theme, color, selected, true)
}

pub fn icon_color(theme: &Theme, color: ChipColor, selected: bool, leading: bool) -> Argb {
    let c = theme.color;
    if selected {
        c.on_secondary_container
    } else if leading && !color.tonal() {
        c.primary
    } else {
        c.on_surface_variant
    }
}

pub fn resolve_demo(theme: &Theme, demo: ChipDemo) -> Appearance {
    resolve_style(
        theme,
        demo.variant,
        demo.color,
        demo.selected,
        demo.state,
        demo.leading,
        demo.avatar,
    )
}

pub fn resolve(
    theme: &Theme,
    variant: ChipVariant,
    selected: bool,
    state: InteractionState,
) -> Appearance {
    resolve_style(theme, variant, ChipColor::Flat, selected, state, None, None)
}

pub fn resolve_style(
    theme: &Theme,
    variant: ChipVariant,
    color: ChipColor,
    selected: bool,
    state: InteractionState,
    leading: Option<&'static str>,
    avatar: Option<PhotoKind>,
) -> Appearance {
    let c = theme.color;
    let elevated = color.elevated() && variant.morphs();
    let (base, label, outline, elevation_dp) = match (variant, selected, elevated) {
        (ChipVariant::Filter | ChipVariant::Input, true, _) => (
            c.secondary_container,
            c.on_secondary_container,
            None,
            if elevated { ELEVATED_DP } else { 0.0 },
        ),
        (ChipVariant::Suggestion, false, _) => {
            (c.surface_container_low, c.on_surface_variant, None, 0.0)
        }
        (ChipVariant::Suggestion, true, _) => {
            (c.secondary_container, c.on_secondary_container, None, 0.0)
        }
        (ChipVariant::Filter | ChipVariant::Input, false, true) => (
            c.surface_container_low,
            c.on_surface_variant,
            None,
            ELEVATED_DP,
        ),
        (ChipVariant::Filter | ChipVariant::Input, false, false) => (
            c.surface,
            c.on_surface_variant,
            Some((c.outline_variant, 1.0)),
            0.0,
        ),
        _ => (c.surface, c.on_surface, Some((c.outline, 1.0)), 0.0),
    };
    let (container, content, outline) = if state.is_disabled() {
        (
            if selected || elevated {
                c.on_surface
                    .with_alpha(DISABLED_CONTAINER_OPACITY)
                    .composite_over(c.surface)
            } else {
                c.surface
            },
            resolve_content(label, c.on_surface, c.surface, true),
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
            apply_state_layer(base, label, state.layer_opacity()),
            label,
            outline,
        )
    };
    let r = corner_dp(theme, variant, selected, state);
    Appearance {
        width_dp: None,
        height_dp: HEIGHT_DP,
        min_width_dp: None,
        corners: Corners::all(r),
        container,
        content,
        secondary_content: Some(icon_color(
            theme,
            color,
            selected,
            !matches!(variant, ChipVariant::Input),
        )),
        outline,
        elevation_dp,
        pad_start_dp: pad_start_avatar(variant, selected, leading, avatar),
        pad_end_dp: pad_end_dp(variant),
        pad_top_dp: 0.0,
        pad_bottom_dp: 0.0,
        label_style: theme.typography.label_large,
        supporting_style: None,
    }
}
