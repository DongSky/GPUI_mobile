//! 触摸事件 → GPUI 鼠标事件的纯逻辑映射，可在 host 上单元测试。

use gpui::{
    Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformInput,
    Point, px,
};

/// 平台无关的触摸动作，与 android-activity 解耦以便 host 单测。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TouchAction {
    Down,
    Move,
    Up,
    Cancel,
}

/// 单指触摸 → 鼠标左键的映射状态机。多指手势留二期。
#[derive(Debug, Default)]
pub struct TouchState {
    pressed: bool,
    last_position: Point<Pixels>,
}

impl TouchState {
    pub fn last_position(&self) -> Point<Pixels> {
        self.last_position
    }

    pub fn map(
        &mut self,
        action: TouchAction,
        physical_x: f32,
        physical_y: f32,
        scale: f32,
    ) -> Vec<PlatformInput> {
        let position = Point::new(px(physical_x / scale), px(physical_y / scale));
        self.last_position = position;
        let modifiers = Modifiers::default();
        match action {
            TouchAction::Down => {
                self.pressed = true;
                vec![PlatformInput::MouseDown(MouseDownEvent {
                    button: MouseButton::Left,
                    position,
                    modifiers,
                    click_count: 1,
                    first_mouse: false,
                })]
            }
            TouchAction::Move => vec![PlatformInput::MouseMove(MouseMoveEvent {
                position,
                pressed_button: self.pressed.then_some(MouseButton::Left),
                modifiers,
            })],
            TouchAction::Up | TouchAction::Cancel => {
                if !self.pressed {
                    return Vec::new();
                }
                self.pressed = false;
                vec![PlatformInput::MouseUp(MouseUpEvent {
                    button: MouseButton::Left,
                    position,
                    modifiers,
                    click_count: 1,
                })]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{PlatformInput, MouseButton, px};

    #[test]
    fn down_then_up_produces_press_release_pair() {
        let mut state = TouchState::default();
        let down = state.map(TouchAction::Down, 100.0, 200.0, 1.0);
        assert_eq!(down.len(), 1);
        match &down[0] {
            PlatformInput::MouseDown(e) => {
                assert_eq!(e.button, MouseButton::Left);
                assert_eq!(e.position.x, px(100.0));
                assert_eq!(e.position.y, px(200.0));
                assert_eq!(e.click_count, 1);
            }
            other => panic!("expected MouseDown, got {other:?}"),
        }
        let up = state.map(TouchAction::Up, 100.0, 200.0, 1.0);
        match &up[0] {
            PlatformInput::MouseUp(e) => assert_eq!(e.button, MouseButton::Left),
            other => panic!("expected MouseUp, got {other:?}"),
        }
    }

    #[test]
    fn move_while_pressed_reports_left_button() {
        let mut state = TouchState::default();
        state.map(TouchAction::Down, 0.0, 0.0, 1.0);
        let moved = state.map(TouchAction::Move, 10.0, 10.0, 1.0);
        match &moved[0] {
            PlatformInput::MouseMove(e) => assert_eq!(e.pressed_button, Some(MouseButton::Left)),
            other => panic!("expected MouseMove, got {other:?}"),
        }
    }

    #[test]
    fn positions_scale_from_physical_to_logical() {
        let mut state = TouchState::default();
        let down = state.map(TouchAction::Down, 210.0, 420.0, 2.625);
        match &down[0] {
            PlatformInput::MouseDown(e) => {
                assert_eq!(e.position.x, px(80.0));
                assert_eq!(e.position.y, px(160.0));
            }
            other => panic!("expected MouseDown, got {other:?}"),
        }
    }

    #[test]
    fn cancel_while_pressed_synthesizes_mouse_up() {
        let mut state = TouchState::default();
        state.map(TouchAction::Down, 5.0, 5.0, 1.0);
        let cancelled = state.map(TouchAction::Cancel, 5.0, 5.0, 1.0);
        assert!(matches!(cancelled[0], PlatformInput::MouseUp(_)));
    }

    #[test]
    fn cancel_without_press_is_noop() {
        let mut state = TouchState::default();
        assert!(state.map(TouchAction::Cancel, 0.0, 0.0, 1.0).is_empty());
    }
}
