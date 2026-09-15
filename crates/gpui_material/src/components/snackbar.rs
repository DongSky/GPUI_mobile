//! Snackbar. Specs: https://m3.material.io/components/snackbar/specs
//!
//! Visual tokens plus a host-testable timeout / swipe-to-dismiss runtime.

use crate::argb::Argb;
use crate::components::photo_stub::PhotoKind;
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
/// Official overview hero: Gmail-style list + “Email archived” + Action + close.
/// The phone chrome has no Inbox title — mail peeks from the top edge.
pub const SCENE_TITLE: &str = "Inbox";
pub const SCENE_SHOW_TITLE: bool = false;
pub const SCENE_MESSAGE: &str = "Email archived";
pub const SCENE_ACTION: &str = "Action";
pub const CLOSE_GLYPH: &str = "✕";
pub const HAS_CLOSE: bool = true;
/// Clipped height of the peeking mail row (official shows a sliver of the previous thread).
pub const PEEK_H_DP: f32 = 28.0;

/// Official mail row: photo avatar + sender + preview + relative time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MailRow {
    pub from: &'static str,
    pub subject: &'static str,
    pub time: &'static str,
    pub photo: PhotoKind,
    pub peek: bool,
}

impl MailRow {
    pub const fn initials(self) -> &'static str {
        match self.photo {
            PhotoKind::PortraitSofia => "SS",
            PhotoKind::PortraitCarmen => "CV",
            _ => "•",
        }
    }
}

pub const MAIL_ROWS: [MailRow; 3] = [
    MailRow {
        from: "Shows lined up",
        subject: "I just saw there are a couple of good shows lined",
        time: "Tue",
        photo: PhotoKind::PortraitAna,
        peek: true,
    },
    MailRow {
        from: "Sofia Sacchi",
        subject: "Bonjour de Paris",
        time: "1 hr ago",
        photo: PhotoKind::PortraitSofia,
        peek: false,
    },
    MailRow {
        from: "Carmen Villanueva",
        subject: "Graduación de nietos",
        time: "yesterday",
        photo: PhotoKind::PortraitCarmen,
        peek: false,
    },
];

/// Official Gmail destinations (not Inbox/Starred/Profile).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InboxDest {
    pub glyph: &'static str,
    pub label: &'static str,
    pub svg: &'static str,
}

pub const INBOX_NAV: [InboxDest; 4] = [
    InboxDest {
        glyph: "✉",
        label: "Mail",
        svg: r#"<svg class="nav-ico" viewBox="0 0 24 24" width="20" height="20" fill="currentColor" aria-hidden="true"><path d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4-8 5-8-5V6l8 5 8-5v2z"/></svg>"#,
    },
    InboxDest {
        glyph: "💬",
        label: "Chat",
        svg: r#"<svg class="nav-ico" viewBox="0 0 24 24" width="20" height="20" fill="currentColor" aria-hidden="true"><path d="M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z"/></svg>"#,
    },
    InboxDest {
        glyph: "▦",
        label: "Rooms",
        svg: r#"<svg class="nav-ico" viewBox="0 0 24 24" width="20" height="20" fill="currentColor" aria-hidden="true"><path d="M10 4H5c-.55 0-1 .45-1 1v5h6V4zm10 0h-5v6h6V5c0-.55-.45-1-1-1zM4 13v5c0 .55.45 1 1 1h5v-6H4zm10 6h5c.55 0 1-.45 1-1v-5h-6v6z"/></svg>"#,
    },
    InboxDest {
        glyph: "▶",
        label: "Meet",
        svg: r#"<svg class="nav-ico" viewBox="0 0 24 24" width="20" height="20" fill="currentColor" aria-hidden="true"><path d="M17 10.5V7c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-3.5l4 4v-11l-4 4z"/></svg>"#,
    },
];
pub const MEET_BADGE_INDEX: usize = 3;
pub const STATUS_H_DP: f32 = 24.0;
pub const STATUS_TIME: &str = "9:41";
pub const AVATAR_DP: f32 = 40.0;
pub const PHONE_W_DP: f32 = 360.0;
pub const PHONE_H_DP: f32 = 560.0;
pub const PHONE_CORNER_DP: f32 = 36.0;
pub const PHONE_BEZEL_DP: f32 = 12.0;
pub const CLOSE_DP: f32 = 24.0;

pub fn mail_avatar_fill(_theme: &Theme, index: usize) -> Argb {
    MAIL_ROWS[index % MAIL_ROWS.len()].photo.fill()
}

pub fn mail_avatar_on(_theme: &Theme, index: usize) -> Argb {
    MAIL_ROWS[index % MAIL_ROWS.len()].photo.on_fill()
}

pub fn mail_avatar_kind(index: usize) -> PhotoKind {
    MAIL_ROWS[index % MAIL_ROWS.len()].photo
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
