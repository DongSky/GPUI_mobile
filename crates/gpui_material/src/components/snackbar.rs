//! Snackbar. Specs: https://m3.material.io/components/snackbar/specs
//!
//! Visual tokens plus a host-testable timeout / swipe-to-dismiss runtime.

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const MIN_HEIGHT_DP: f32 = 48.0;
pub const PAD_H_DP: f32 = 16.0;
pub const CORNER_DP: f32 = 4.0;
/// Material snackbar short duration.
pub const TIMEOUT_SHORT_MS: u32 = 4000;
/// Material snackbar long duration.
pub const TIMEOUT_LONG_MS: u32 = 10000;
/// Horizontal swipe distance that dismisses the bar.
pub const SWIPE_DISMISS_DP: f32 = 72.0;
pub const DEMO_MESSAGE: &str = "Can't send right now. Try again later.";
pub const DEMO_ACTION: &str = "Retry";
/// Official overview hero: in-app mail list + “Email archived” + Undo + close.
pub const SCENE_TITLE: &str = "Inbox";
pub const SCENE_MESSAGE: &str = "Email archived";
pub const SCENE_ACTION: &str = "Undo";
pub const CLOSE_GLYPH: &str = "✕";
pub const HAS_CLOSE: bool = true;

/// Official Inbox row: avatar initials + sender + subject + timestamp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MailRow {
    pub initials: &'static str,
    pub from: &'static str,
    pub subject: &'static str,
    pub time: &'static str,
}

pub const MAIL_ROWS: [MailRow; 3] = [
    MailRow {
        initials: "AR",
        from: "Alex Rivera",
        subject: "Design sync notes",
        time: "9:41",
    },
    MailRow {
        initials: "JL",
        from: "Jordan Lee",
        subject: "Q3 planning",
        time: "8:12",
    },
    MailRow {
        initials: "SC",
        from: "Sam Chen",
        subject: "Invoice #1842",
        time: "Yesterday",
    },
];
pub const INBOX_NAV: [(&str, &str); 3] = [("✉", "Inbox"), ("★", "Starred"), ("👤", "Profile")];
pub const STATUS_H_DP: f32 = 24.0;
pub const STATUS_TIME: &str = "9:41";
pub const AVATAR_DP: f32 = 40.0;
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 520.0;
pub const PHONE_CORNER_DP: f32 = 36.0;
pub const PHONE_BEZEL_DP: f32 = 12.0;
pub const CLOSE_DP: f32 = 24.0;

pub fn mail_avatar_fill(theme: &Theme, index: usize) -> Argb {
    let c = theme.color;
    match index % 3 {
        0 => c.primary_container,
        1 => c.secondary_container,
        _ => c.tertiary_container,
    }
}

pub fn mail_avatar_on(theme: &Theme, index: usize) -> Argb {
    let c = theme.color;
    match index % 3 {
        0 => c.on_primary_container,
        1 => c.on_secondary_container,
        _ => c.on_tertiary_container,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnackbarAppearance {
    pub min_height_dp: f32,
    pub corners: Corners,
    pub container: Argb,
    pub supporting: Argb,
    pub action: Argb,
    pub close: Argb,
    pub supporting_style: TypeStyle,
    pub action_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> SnackbarAppearance {
    SnackbarAppearance {
        min_height_dp: MIN_HEIGHT_DP,
        corners: Corners::all(CORNER_DP),
        container: theme.color.inverse_surface,
        supporting: theme.color.inverse_on_surface,
        action: theme.color.inverse_primary,
        close: theme.color.inverse_on_surface,
        supporting_style: theme.typography.body_medium,
        action_style: theme.typography.label_large,
    }
}

/// Live snackbar: remaining timeout + swipe offset. Hosts tick at `FRAME_MS`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnackbarState {
    pub visible: bool,
    pub offset_x_dp: f32,
    pub remaining_ms: f32,
}

impl SnackbarState {
    pub fn short() -> Self {
        Self {
            visible: true,
            offset_x_dp: 0.0,
            remaining_ms: TIMEOUT_SHORT_MS as f32,
        }
    }

    pub fn long() -> Self {
        Self {
            visible: true,
            offset_x_dp: 0.0,
            remaining_ms: TIMEOUT_LONG_MS as f32,
        }
    }

    /// Advance the timeout. Returns true when this tick hid the bar.
    pub fn tick(&mut self, dt_ms: f32) -> bool {
        if !self.visible {
            return false;
        }
        self.remaining_ms -= dt_ms.max(0.0);
        if self.remaining_ms <= 0.0 || self.offset_x_dp.abs() >= SWIPE_DISMISS_DP {
            self.visible = false;
            self.remaining_ms = 0.0;
            return true;
        }
        false
    }

    pub fn swipe(&mut self, dx_dp: f32) {
        if !self.visible {
            return;
        }
        self.offset_x_dp += dx_dp;
        if self.offset_x_dp.abs() >= SWIPE_DISMISS_DP {
            self.visible = false;
            self.remaining_ms = 0.0;
        }
    }

    /// Close affordance (official Action + close pair).
    pub fn close(&mut self) {
        self.visible = false;
        self.remaining_ms = 0.0;
        self.offset_x_dp = 0.0;
    }

    pub fn dismissed(&self) -> bool {
        !self.visible
    }

    pub fn opacity(&self) -> f32 {
        if !self.visible {
            return 0.0;
        }
        1.0 - (self.offset_x_dp.abs() / SWIPE_DISMISS_DP).clamp(0.0, 1.0)
    }
}
