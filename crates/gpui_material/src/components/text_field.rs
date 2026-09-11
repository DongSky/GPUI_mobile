//! Filled and outlined text fields.
//! Specs: https://m3.material.io/components/text-fields/specs
//!
//! Visual tokens plus a host-testable editor (`TextFieldEditor`). Android
//! NativeActivity still stubs `update_ime_position`; the catalog and demo edit
//! through this editor (HTML `<input>` / on-screen keys).

use crate::argb::Argb;
use crate::components::Appearance;
use crate::shape::Corners;
use crate::state::InteractionState;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const HEIGHT_DP: f32 = 56.0;
pub const PAD_H_DP: f32 = 16.0;
pub const INDICATOR_DP: f32 = 1.0;
pub const INDICATOR_FOCUSED_DP: f32 = 2.0;
pub const OUTLINE_DP: f32 = 1.0;
pub const OUTLINE_FOCUSED_DP: f32 = 2.0;
pub const SUPPORTING_GAP_DP: f32 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldVariant {
    Filled,
    Outlined,
}

impl TextFieldVariant {
    pub const ALL: [Self; 2] = [Self::Filled, Self::Outlined];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Outlined => "outlined",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextFieldAppearance {
    pub field: Appearance,
    pub label: Argb,
    pub input: Argb,
    pub supporting: Argb,
    pub label_style: TypeStyle,
    pub input_style: TypeStyle,
    pub supporting_style: TypeStyle,
    pub populated: bool,
}

pub fn resolve(
    theme: &Theme,
    variant: TextFieldVariant,
    state: InteractionState,
    populated: bool,
) -> TextFieldAppearance {
    let c = theme.color;
    let focused = matches!(
        state,
        InteractionState::Focused | InteractionState::ErrorFocused
    );
    let error = state.is_error();
    let disabled = state.is_disabled();

    let (label, input, supporting, indicator) = if disabled {
        let muted = c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface);
        (muted, muted, muted, muted)
    } else if error && focused {
        (c.error, c.on_surface, c.error, c.error)
    } else if error && state == InteractionState::Hovered {
        (
            c.on_error_container,
            c.on_surface,
            c.error,
            c.on_error_container,
        )
    } else if error {
        (c.error, c.on_surface, c.error, c.error)
    } else if focused {
        (c.primary, c.on_surface, c.on_surface_variant, c.primary)
    } else if state == InteractionState::Hovered {
        (
            c.on_surface_variant,
            c.on_surface,
            c.on_surface_variant,
            c.on_surface,
        )
    } else {
        (
            c.on_surface_variant,
            c.on_surface,
            c.on_surface_variant,
            c.on_surface_variant,
        )
    };

    let (container, corners, outline) = match variant {
        TextFieldVariant::Filled => {
            let base = if disabled {
                c.on_surface.with_alpha(0.04).composite_over(c.surface)
            } else {
                c.surface_container_highest
            };
            let width = if focused {
                INDICATOR_FOCUSED_DP
            } else {
                INDICATOR_DP
            };
            (
                base,
                Corners::extra_small_top(theme.shapes.extra_small),
                Some((indicator, width)),
            )
        }
        TextFieldVariant::Outlined => {
            let width = if focused {
                OUTLINE_FOCUSED_DP
            } else {
                OUTLINE_DP
            };
            (
                c.surface,
                Corners::all(theme.shapes.extra_small),
                Some((indicator, width)),
            )
        }
    };

    let label_style = if populated || focused {
        theme.typography.body_small
    } else {
        theme.typography.body_large
    };

    TextFieldAppearance {
        field: Appearance {
            width_dp: None,
            height_dp: HEIGHT_DP,
            min_width_dp: Some(210.0),
            corners,
            container,
            content: input,
            secondary_content: Some(label),
            outline,
            elevation_dp: 0.0,
            pad_start_dp: PAD_H_DP,
            pad_end_dp: PAD_H_DP,
            pad_top_dp: 8.0,
            pad_bottom_dp: 8.0,
            label_style,
            supporting_style: Some(theme.typography.body_small),
        },
        label,
        input,
        supporting,
        label_style,
        input_style: theme.typography.body_large,
        supporting_style: theme.typography.body_small,
        populated,
    }
}

/// Host-testable caret editor. Used by the HTML catalog and the Android demo
/// so fields are actually editable without a system `InputConnection`.
#[derive(Clone, Debug, PartialEq)]
pub struct TextFieldEditor {
    pub variant: TextFieldVariant,
    value: String,
    caret: usize,
    pub focused: bool,
    pub error: bool,
    pub disabled: bool,
}

impl TextFieldEditor {
    pub fn new(variant: TextFieldVariant, initial: impl Into<String>) -> Self {
        let value = initial.into();
        let caret = value.chars().count();
        Self {
            variant,
            value,
            caret,
            focused: false,
            error: false,
            disabled: false,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn caret(&self) -> usize {
        self.caret
    }

    pub fn populated(&self) -> bool {
        !self.value.is_empty()
    }

    pub fn interaction_state(&self) -> InteractionState {
        if self.disabled {
            InteractionState::Disabled
        } else if self.error && self.focused {
            InteractionState::ErrorFocused
        } else if self.error {
            InteractionState::Error
        } else if self.focused {
            InteractionState::Focused
        } else {
            InteractionState::Enabled
        }
    }

    pub fn set_focus(&mut self, focused: bool) {
        if !self.disabled {
            self.focused = focused;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.disabled {
            return;
        }
        if ch == '\u{8}' || ch == '\u{7f}' {
            self.backspace();
            return;
        }
        if ch.is_control() {
            return;
        }
        let mut chars: Vec<char> = self.value.chars().collect();
        let i = self.caret.min(chars.len());
        chars.insert(i, ch);
        self.caret = i + 1;
        self.value = chars.into_iter().collect();
    }

    pub fn insert_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    pub fn backspace(&mut self) {
        if self.disabled || self.caret == 0 {
            return;
        }
        let mut chars: Vec<char> = self.value.chars().collect();
        chars.remove(self.caret - 1);
        self.caret -= 1;
        self.value = chars.into_iter().collect();
    }

    pub fn move_caret(&mut self, delta: i32) {
        let len = self.value.chars().count() as i32;
        self.caret = (self.caret as i32 + delta).clamp(0, len) as usize;
    }

    pub fn appearance(&self, theme: &Theme) -> TextFieldAppearance {
        resolve(
            theme,
            self.variant,
            self.interaction_state(),
            self.populated(),
        )
    }

    /// Visible value with a `|` caret when focused (Android demo / tests).
    pub fn display_with_caret(&self) -> String {
        if !self.focused {
            return self.value.clone();
        }
        let chars: Vec<char> = self.value.chars().collect();
        let i = self.caret.min(chars.len());
        let mut out = String::new();
        for (idx, ch) in chars.iter().enumerate() {
            if idx == i {
                out.push('|');
            }
            out.push(*ch);
        }
        if i == chars.len() {
            out.push('|');
        }
        out
    }
}

/// Lightweight email check used by the outlined error demo.
pub fn looks_like_email(s: &str) -> bool {
    let Some(at) = s.find('@') else {
        return false;
    };
    at > 0 && s[at + 1..].contains('.')
}
