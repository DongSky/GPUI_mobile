//! NativeActivity IME groundwork.
//!
//! There is still no JNI `InputConnection` on this NativeActivity host. This
//! module is the host-testable protocol a later JNI layer should call:
//! caret bounds from `gpui_material::text_field::ime_caret_rect_dp` are stored
//! by `PlatformWindow::update_ime_position`, and `ImeSession` mirrors the
//! Android `InputConnection` methods (`commitText`, `deleteSurroundingText`,
//! `setComposingText`, `setSelection`, `getTextBeforeCursor`).

use std::cell::Cell;

/// Logical caret rectangle: `x, y, width, height` in device-independent pixels.
pub type ImeBoundsDp = [f32; 4];

/// Store the last IME caret rectangle (test hook + Android window field).
pub fn record_ime_position(slot: &Cell<Option<ImeBoundsDp>>, x: f32, y: f32, w: f32, h: f32) {
    slot.set(Some([x, y, w, h]));
}

pub fn last_ime_position(slot: &Cell<Option<ImeBoundsDp>>) -> Option<ImeBoundsDp> {
    slot.get()
}

/// `PlatformWindow::update_ime_position` feeds this from a GPUI caret `Bounds`.
pub fn record_caret_rect(slot: &Cell<Option<ImeBoundsDp>>, rect: (f32, f32, f32, f32)) {
    record_ime_position(slot, rect.0, rect.1, rect.2, rect.3);
}

/// Window-space caret rect from a field origin plus a local caret rect.
pub fn bounds_from_field_caret(
    field_x: f32,
    field_y: f32,
    caret: (f32, f32, f32, f32),
) -> ImeBoundsDp {
    [field_x + caret.0, field_y + caret.1, caret.2, caret.3]
}

/// Requests a later JNI `InputMethodManager` can consume.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImeRequest {
    ShowSoftInput,
    HideSoftInput,
    UpdateCursorAnchor,
}

/// Host-testable InputConnection session (no JNI).
#[derive(Clone, Debug, PartialEq)]
pub struct ImeSession {
    text: String,
    sel_start: usize,
    sel_end: usize,
    composing: Option<(usize, usize)>,
    pub bounds: Option<ImeBoundsDp>,
    pub shown: bool,
    pub last_request: Option<ImeRequest>,
}

impl Default for ImeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ImeSession {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            sel_start: 0,
            sel_end: 0,
            composing: None,
            bounds: None,
            shown: false,
            last_request: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn selection(&self) -> (usize, usize) {
        let a = self.sel_start.min(self.sel_end);
        let b = self.sel_start.max(self.sel_end);
        (a, b)
    }

    pub fn composing(&self) -> Option<(usize, usize)> {
        self.composing
    }

    pub fn show_soft_input(&mut self) {
        self.shown = true;
        self.last_request = Some(ImeRequest::ShowSoftInput);
    }

    pub fn hide_soft_input(&mut self) {
        self.shown = false;
        self.last_request = Some(ImeRequest::HideSoftInput);
    }

    /// `InputConnection.updateCursorAnchorInfo` / `update_ime_position`.
    pub fn update_cursor_anchor(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.bounds = Some([x, y, w, h]);
        self.shown = true;
        self.last_request = Some(ImeRequest::UpdateCursorAnchor);
    }

    pub fn set_selection(&mut self, start: usize, end: usize) {
        let n = self.text.chars().count();
        self.sel_start = start.min(n);
        self.sel_end = end.min(n);
    }

    pub fn get_text_before_cursor(&self, n: usize) -> String {
        let (start, _) = self.selection();
        let chars: Vec<char> = self.text.chars().collect();
        let from = start.saturating_sub(n);
        chars[from..start].iter().collect()
    }

    pub fn get_text_after_cursor(&self, n: usize) -> String {
        let (_, end) = self.selection();
        let chars: Vec<char> = self.text.chars().collect();
        let to = (end + n).min(chars.len());
        chars[end..to].iter().collect()
    }

    pub fn get_selected_text(&self) -> String {
        let (start, end) = self.selection();
        self.text.chars().skip(start).take(end - start).collect()
    }

    /// `InputConnection.commitText`.
    pub fn commit_text(&mut self, text: &str, _new_cursor_position: i32) {
        self.replace_selection(text);
        self.composing = None;
    }

    /// `InputConnection.setComposingText` (underlined composing region).
    pub fn set_composing_text(&mut self, text: &str, _new_cursor_position: i32) {
        if let Some((a, b)) = self.composing {
            self.sel_start = a;
            self.sel_end = b;
        }
        self.replace_selection(text);
        let end = self.sel_start;
        let start = end.saturating_sub(text.chars().count());
        self.composing = if text.is_empty() {
            None
        } else {
            Some((start, end))
        };
    }

    pub fn finish_composing_text(&mut self) {
        self.composing = None;
    }

    /// `InputConnection.deleteSurroundingText`.
    pub fn delete_surrounding_text(&mut self, before_length: usize, after_length: usize) {
        let (start, end) = self.selection();
        let mut chars: Vec<char> = self.text.chars().collect();
        let del_end = (end + after_length).min(chars.len());
        if del_end > end {
            chars.drain(end..del_end);
        }
        let del_start = start.saturating_sub(before_length);
        if start > del_start {
            chars.drain(del_start..start);
        }
        self.text = chars.into_iter().collect();
        let caret = del_start;
        self.sel_start = caret;
        self.sel_end = caret;
        self.composing = None;
    }

    fn replace_selection(&mut self, insert: &str) {
        let (start, end) = self.selection();
        let mut chars: Vec<char> = self.text.chars().collect();
        chars.drain(start..end);
        for (i, ch) in insert.chars().enumerate() {
            chars.insert(start + i, ch);
        }
        self.text = chars.into_iter().collect();
        let caret = start + insert.chars().count();
        self.sel_start = caret;
        self.sel_end = caret;
    }
}

/// Apply a GPUI `update_ime_position` rect to both the legacy slot and the session.
pub fn apply_update_ime_position(
    slot: &Cell<Option<ImeBoundsDp>>,
    session: &std::cell::RefCell<ImeSession>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    record_ime_position(slot, x, y, w, h);
    session.borrow_mut().update_cursor_anchor(x, y, w, h);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn records_caret_rect_for_update_ime_position() {
        let slot = Cell::new(None);
        assert_eq!(last_ime_position(&slot), None);
        record_ime_position(&slot, 16.0, 16.0, 2.0, 24.0);
        assert_eq!(last_ime_position(&slot), Some([16.0, 16.0, 2.0, 24.0]));
        record_caret_rect(&slot, (20.0, 8.0, 2.0, 24.0));
        assert_eq!(last_ime_position(&slot), Some([20.0, 8.0, 2.0, 24.0]));
    }

    #[test]
    fn input_connection_commit_and_delete() {
        let mut ime = ImeSession::new();
        ime.show_soft_input();
        assert!(ime.shown);
        ime.commit_text("hel", 1);
        ime.commit_text("lo", 1);
        assert_eq!(ime.text(), "hello");
        assert_eq!(ime.get_text_before_cursor(3), "llo");
        ime.set_selection(0, 2);
        assert_eq!(ime.get_selected_text(), "he");
        ime.commit_text("He", 1);
        assert_eq!(ime.text(), "Hello");
        ime.set_selection(5, 5);
        ime.delete_surrounding_text(1, 0);
        assert_eq!(ime.text(), "Hell");
        ime.hide_soft_input();
        assert!(!ime.shown);
        assert_eq!(ime.last_request, Some(ImeRequest::HideSoftInput));
    }

    #[test]
    fn composing_region_and_cursor_anchor() {
        let mut ime = ImeSession::new();
        ime.set_composing_text("ni", 1);
        assert_eq!(ime.text(), "ni");
        assert_eq!(ime.composing(), Some((0, 2)));
        ime.set_composing_text("你好", 1);
        assert_eq!(ime.text(), "你好");
        ime.finish_composing_text();
        assert_eq!(ime.composing(), None);
        ime.update_cursor_anchor(24.0, 12.0, 2.0, 24.0);
        assert_eq!(ime.bounds, Some([24.0, 12.0, 2.0, 24.0]));
        assert_eq!(ime.last_request, Some(ImeRequest::UpdateCursorAnchor));
        let mapped = bounds_from_field_caret(8.0, 16.0, (16.0, 16.0, 2.0, 24.0));
        assert_eq!(mapped, [24.0, 32.0, 2.0, 24.0]);
    }

    #[test]
    fn apply_update_ime_position_fills_slot_and_session() {
        let slot = Cell::new(None);
        let session = RefCell::new(ImeSession::new());
        apply_update_ime_position(&slot, &session, 10.0, 20.0, 2.0, 24.0);
        assert_eq!(last_ime_position(&slot), Some([10.0, 20.0, 2.0, 24.0]));
        assert_eq!(session.borrow().bounds, Some([10.0, 20.0, 2.0, 24.0]));
        assert!(session.borrow().shown);
    }
}
