//! Android 屏幕 → `PlatformDisplay` 的最小实现。PoC 阶段单显示器、单窗口。

use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};
use std::cell::Cell;

#[derive(Debug)]
pub(crate) struct AndroidDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    // 逻辑像素尺寸；旋转时由平台层更新。
    size: Cell<Size<Pixels>>,
}

// 仅主线程访问（与 WebDisplay 同理）。
unsafe impl Send for AndroidDisplay {}
unsafe impl Sync for AndroidDisplay {}

impl AndroidDisplay {
    pub fn new(size_physical: (i32, i32), scale: f32) -> Self {
        Self {
            id: DisplayId::new(1),
            uuid: uuid::Uuid::new_v4(),
            size: Cell::new(logical_size(size_physical, scale)),
        }
    }

    pub fn update_size(&self, size_physical: (i32, i32), scale: f32) {
        self.size.set(logical_size(size_physical, scale));
    }
}

pub(crate) fn logical_size(size_physical: (i32, i32), scale: f32) -> Size<Pixels> {
    Size {
        width: px(size_physical.0 as f32 / scale),
        height: px(size_physical.1 as f32 / scale),
    }
}

impl PlatformDisplay for AndroidDisplay {
    fn id(&self) -> DisplayId {
        self.id
    }

    fn uuid(&self) -> Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        Bounds { origin: Point::default(), size: self.size.get() }
    }

    fn default_bounds(&self) -> Bounds<Pixels> {
        // 移动端窗口铺满屏幕。
        self.bounds()
    }
}
