//! 触摸事件 → GPUI 鼠标/滚轮事件的纯逻辑映射，可在 host 上单元测试。
//!
//! GPUI（及 gpui-component 的可滚动容器）只响应 `ScrollWheelEvent`
//! 驱动内容偏移——按住拖动只会产生 `MouseMove`，不会让 `List`/`div().overflow_y_scroll()`
//! 之类的容器滚动（这些容器的滚动监听只处理 `ScrollWheelEvent`，鼠标拖动语义留给
//! 滚动条滑块或可拖拽控件本身）。因此除了原有的 Mouse 事件映射外，
//! 按住状态下的 `Move` 还需额外合成一条 `ScrollWheelEvent`，
//! 用相邻两次触摸位置之差作为 `delta`（`ScrollDelta::Pixels`），
//! 并标记 `touch_phase = Moved`，这样单指滑动才能真正驱动列表滚动。

use gpui::{
    Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformInput,
    Point, ScrollDelta, ScrollWheelEvent, TouchPhase, px,
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
        let previous_position = self.last_position;
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
            TouchAction::Move => {
                let mut events = vec![PlatformInput::MouseMove(MouseMoveEvent {
                    position,
                    pressed_button: self.pressed.then_some(MouseButton::Left),
                    modifiers,
                })];
                if self.pressed {
                    let delta = Point::new(
                        position.x - previous_position.x,
                        position.y - previous_position.y,
                    );
                    if delta.x != px(0.) || delta.y != px(0.) {
                        events.push(PlatformInput::ScrollWheel(ScrollWheelEvent {
                            position,
                            delta: ScrollDelta::Pixels(delta),
                            modifiers,
                            touch_phase: TouchPhase::Moved,
                        }));
                    }
                }
                events
            }
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

    #[test]
    fn move_while_pressed_also_emits_scroll_wheel_delta() {
        let mut state = TouchState::default();
        state.map(TouchAction::Down, 540.0, 1800.0, 1.0);
        // 手指从 y=1800 滑到 y=1700（向上滑动），期望 delta.y 为负
        // （与 gpui-component scrollable_mask 的 `offset += delta` 约定一致，
        // 使内容随手指方向滚动、显示后续行）。
        let moved = state.map(TouchAction::Move, 540.0, 1700.0, 1.0);
        assert_eq!(moved.len(), 2, "expected MouseMove + ScrollWheel, got {moved:?}");
        match &moved[0] {
            PlatformInput::MouseMove(_) => {}
            other => panic!("expected MouseMove at index 0, got {other:?}"),
        }
        match &moved[1] {
            PlatformInput::ScrollWheel(e) => {
                assert_eq!(e.touch_phase, TouchPhase::Moved);
                match e.delta {
                    ScrollDelta::Pixels(delta) => {
                        assert_eq!(delta.x, px(0.0));
                        assert_eq!(delta.y, px(-100.0));
                    }
                    other => panic!("expected Pixels delta, got {other:?}"),
                }
            }
            other => panic!("expected ScrollWheel at index 1, got {other:?}"),
        }
    }

    #[test]
    fn move_without_press_does_not_emit_scroll_wheel() {
        let mut state = TouchState::default();
        // 未经过 Down 就收到 Move（理论上不应发生，但作为防御性测试保留）：
        // 不应产生滚轮事件，只应有一条 MouseMove。
        let moved = state.map(TouchAction::Move, 10.0, 10.0, 1.0);
        assert_eq!(moved.len(), 1);
        assert!(matches!(moved[0], PlatformInput::MouseMove(_)));
    }

    #[test]
    fn move_with_zero_delta_does_not_duplicate_scroll_wheel() {
        let mut state = TouchState::default();
        state.map(TouchAction::Down, 50.0, 50.0, 1.0);
        // 同一位置再次上报 Move（部分设备可能重复上报相同坐标）：不应产生零位移的滚轮事件。
        let moved = state.map(TouchAction::Move, 50.0, 50.0, 1.0);
        assert_eq!(moved.len(), 1);
        assert!(matches!(moved[0], PlatformInput::MouseMove(_)));
    }
}
