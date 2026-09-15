//! NativeActivity IME stub.
//!
//! There is no `InputConnection` on this NativeActivity host. Caret bounds
//! computed by `gpui_material::text_field::ime_caret_rect_dp` are recorded here
//! so a later JNI / InputConnection layer can consume them via
//! `PlatformWindow::update_ime_position`.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_caret_rect_for_update_ime_position() {
        let slot = Cell::new(None);
        assert_eq!(last_ime_position(&slot), None);
        record_ime_position(&slot, 16.0, 16.0, 2.0, 24.0);
        assert_eq!(last_ime_position(&slot), Some([16.0, 16.0, 2.0, 24.0]));
        record_caret_rect(&slot, (20.0, 8.0, 2.0, 24.0));
        assert_eq!(last_ime_position(&slot), Some([20.0, 8.0, 2.0, 24.0]));
    }
}
