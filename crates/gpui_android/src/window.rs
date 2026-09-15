//! Android `SurfaceHolder`/`ANativeWindow` 生命周期 → `PlatformWindow`。
//! wgpu surface 随 `surfaceCreated`/`surfaceDestroyed` 创建/销毁；渲染器本身
//! （device/queue/atlas/pipelines）在此期间保持存活，仅 surface 被
//! unconfigure/replace（见 gpui_wgpu::WgpuRenderer::unconfigure_surface /
//! replace_surface 文档）。

use crate::display::{AndroidDisplay, logical_size};
use crate::events::{TouchAction, TouchState};
use gpui::{
    Bounds, Capslock, DevicePixels, DispatchEventResult, GpuSpecs, Modifiers, Pixels,
    PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point,
    PromptButton, PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea,
};
use gpui_wgpu::{GpuContext, WgpuRenderer, WgpuSurfaceConfig, wgpu};
use ndk::native_window::NativeWindow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

/// 供 wgpu 建 surface 的 raw-window-handle 包装。
/// WgpuRenderer::new 要求 Clone + Debug + Send + Sync + 'static。
/// `NativeWindow` 本身已满足 Clone/Debug/Send/Sync（ndk 0.9 内部用
/// `unsafe impl Send/Sync` + 引用计数 `ANativeWindow_acquire`/`_release`），
/// 但它没有实现 `HasDisplayHandle`，因此仍需要这层包装来补上 Android 的
/// 空 display handle。
#[derive(Clone, Debug)]
pub(crate) struct AndroidRawWindow(NativeWindow);

impl raw_window_handle::HasWindowHandle for AndroidRawWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let ptr = std::ptr::NonNull::new(self.0.ptr().as_ptr().cast())
            .ok_or(raw_window_handle::HandleError::Unavailable)?;
        let handle = raw_window_handle::AndroidNdkWindowHandle::new(ptr);
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
    }
}

impl raw_window_handle::HasDisplayHandle for AndroidRawWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(raw_window_handle::DisplayHandle::android())
    }
}

#[derive(Default)]
pub(crate) struct WindowCallbacks {
    request_frame: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    input: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    hover_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
    hit_test_window_control: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
}

pub(crate) struct WindowState {
    renderer: WgpuRenderer,
    native_window: Option<NativeWindow>,
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    input_handler: Option<PlatformInputHandler>,
    is_active: bool,
    touch: TouchState,
}

pub(crate) struct AndroidWindowInner {
    pub(crate) state: RefCell<WindowState>,
    pub(crate) callbacks: RefCell<WindowCallbacks>,
    display: Rc<AndroidDisplay>,
    gpu_context: GpuContext,
    /// Last caret bounds from `update_ime_position` (NativeActivity has no IME).
    pub last_ime_bounds: Cell<Option<crate::ime::ImeBoundsDp>>,
    /// InputConnection-shaped session filled by `update_ime_position`.
    pub ime: std::cell::RefCell<crate::ime::ImeSession>,
    /// IMM JNI plan to flush on a live `JNIEnv` (`ImeJniQueue::dry_run_jni_env`).
    pub pending_jni: std::cell::RefCell<crate::ime::ImeJniQueue>,
}

impl AndroidWindowInner {
    pub fn new(
        native_window: NativeWindow,
        scale: f32,
        gpu_context: &GpuContext,
        display: Rc<AndroidDisplay>,
    ) -> anyhow::Result<Rc<Self>> {
        let physical = (native_window.width(), native_window.height());
        let raw = AndroidRawWindow(native_window.clone());
        let config = WgpuSurfaceConfig {
            size: Size {
                width: DevicePixels(physical.0),
                height: DevicePixels(physical.1),
            },
            transparent: false,
            // gpui_wgpu 文档建议移动端优先 Mailbox，避免生命周期切换时
            // get_current_texture 阻塞；不支持时 renderer 内部回退。
            preferred_present_mode: Some(wgpu::PresentMode::Mailbox),
        };
        let renderer = WgpuRenderer::new(gpu_context.clone(), &raw, config, None)?;
        let state = WindowState {
            renderer,
            native_window: Some(native_window),
            bounds: Bounds { origin: Point::default(), size: logical_size(physical, scale) },
            scale_factor: scale,
            input_handler: None,
            is_active: true,
            touch: TouchState::default(),
        };
        Ok(Rc::new(Self {
            state: RefCell::new(state),
            callbacks: RefCell::new(WindowCallbacks::default()),
            display,
            gpu_context: gpu_context.clone(),
            last_ime_bounds: Cell::new(None),
            ime: std::cell::RefCell::new(crate::ime::ImeSession::new()),
            pending_jni: std::cell::RefCell::new(crate::ime::ImeJniQueue::new()),
        }))
    }

    /// take/call/restore：回调执行期间不持有 RefCell 借用，允许重入（对照 WebWindowInner::with_callback）。
    fn with_callback<C, R>(
        &self,
        select: impl Fn(&mut WindowCallbacks) -> &mut Option<C>,
        invoke: impl FnOnce(&mut C) -> R,
    ) -> Option<R> {
        let mut callback = select(&mut self.callbacks.borrow_mut()).take()?;
        let result = invoke(&mut callback);
        *select(&mut self.callbacks.borrow_mut()) = Some(callback);
        Some(result)
    }

    pub fn surface_alive(&self) -> bool {
        self.state.borrow().native_window.is_some()
    }

    pub fn handle_surface_destroyed(&self) {
        let mut state = self.state.borrow_mut();
        state.renderer.unconfigure_surface();
        state.native_window = None;
    }

    pub fn handle_surface_recreated(&self, native_window: NativeWindow) -> anyhow::Result<()> {
        let scale = self.state.borrow().scale_factor;
        let physical = (native_window.width(), native_window.height());
        let raw = AndroidRawWindow(native_window.clone());
        let config = WgpuSurfaceConfig {
            size: Size {
                width: DevicePixels(physical.0),
                height: DevicePixels(physical.1),
            },
            transparent: false,
            preferred_present_mode: Some(wgpu::PresentMode::Mailbox),
        };
        let instance = self
            .gpu_context
            .borrow()
            .as_ref()
            .map(|context| context.instance.clone())
            .ok_or_else(|| anyhow::anyhow!("wgpu context missing during surface recreation"))?;
        {
            let mut state = self.state.borrow_mut();
            state.renderer.replace_surface(&raw, config, &instance)?;
            state.native_window = Some(native_window);
        }
        self.handle_resize(physical, scale);
        Ok(())
    }

    pub fn handle_resize(&self, physical: (i32, i32), scale: f32) {
        let new_size = logical_size(physical, scale);
        {
            let mut state = self.state.borrow_mut();
            state.bounds.size = new_size;
            state.scale_factor = scale;
            state.renderer.update_drawable_size(Size {
                width: DevicePixels(physical.0),
                height: DevicePixels(physical.1),
            });
        }
        self.display.update_size(physical, scale);
        self.with_callback(
            |callbacks| &mut callbacks.resize,
            |callback| callback(new_size, scale),
        );
    }

    pub fn handle_touch(&self, action: TouchAction, physical_x: f32, physical_y: f32) {
        let scale = self.state.borrow().scale_factor;
        let inputs = self
            .state
            .borrow_mut()
            .touch
            .map(action, physical_x, physical_y, scale);
        for input in inputs {
            self.with_callback(
                |callbacks| &mut callbacks.input,
                |callback| callback(input),
            );
        }
    }

    pub fn fire_request_frame(&self) {
        self.with_callback(
            |callbacks| &mut callbacks.request_frame,
            |callback| {
                callback(RequestFrameOptions {
                    require_presentation: false,
                    force_render: false,
                })
            },
        );
    }

    pub fn set_active(&self, active: bool) {
        self.state.borrow_mut().is_active = active;
        self.with_callback(
            |callbacks| &mut callbacks.active_status_change,
            |callback| callback(active),
        );
    }
}

pub(crate) struct AndroidWindow {
    pub(crate) inner: Rc<AndroidWindowInner>,
    display: Rc<dyn PlatformDisplay>,
}

impl AndroidWindow {
    pub fn new(inner: Rc<AndroidWindowInner>, display: Rc<AndroidDisplay>) -> Self {
        Self { inner, display: display as Rc<dyn PlatformDisplay> }
    }
}

impl raw_window_handle::HasWindowHandle for AndroidWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let state = self.inner.state.borrow();
        let native_window = state
            .native_window
            .as_ref()
            .ok_or(raw_window_handle::HandleError::Unavailable)?;
        let ptr = std::ptr::NonNull::new(native_window.ptr().as_ptr().cast())
            .ok_or(raw_window_handle::HandleError::Unavailable)?;
        let handle = raw_window_handle::AndroidNdkWindowHandle::new(ptr);
        // handle 必须即取即用，不得跨 handle_surface_destroyed 持有，否则底层 ANativeWindow 指针悬垂
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
    }
}

impl raw_window_handle::HasDisplayHandle for AndroidWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(raw_window_handle::DisplayHandle::android())
    }
}

impl PlatformWindow for AndroidWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.inner.state.borrow().bounds
    }
    fn is_maximized(&self) -> bool {
        true
    }
    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }
    fn content_size(&self) -> Size<Pixels> {
        self.inner.state.borrow().bounds.size
    }
    fn resize(&mut self, _size: Size<Pixels>) {}
    fn scale_factor(&self) -> f32 {
        self.inner.state.borrow().scale_factor
    }
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }
    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }
    fn mouse_position(&self) -> Point<Pixels> {
        self.inner.state.borrow().touch.last_position()
    }
    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }
    fn capslock(&self) -> Capslock {
        Capslock::default()
    }
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.inner.state.borrow_mut().input_handler = Some(input_handler);
    }
    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.inner.state.borrow_mut().input_handler.take()
    }
    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        None
    }
    fn activate(&self) {}
    fn is_active(&self) -> bool {
        self.inner.state.borrow().is_active
    }
    fn is_hovered(&self) -> bool {
        false
    }
    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }
    fn set_title(&mut self, _title: &str) {}
    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}
    fn minimize(&self) {}
    fn zoom(&self) {}
    fn toggle_fullscreen(&self) {}
    fn is_fullscreen(&self) -> bool {
        true
    }
    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.inner.callbacks.borrow_mut().request_frame = Some(callback);
    }
    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.inner.callbacks.borrow_mut().input = Some(callback);
    }
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().active_status_change = Some(callback);
    }
    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().hover_status_change = Some(callback);
    }
    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.inner.callbacks.borrow_mut().resize = Some(callback);
    }
    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().moved = Some(callback);
    }
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.inner.callbacks.borrow_mut().should_close = Some(callback);
    }
    fn on_hit_test_window_control(
        &self,
        callback: Box<dyn FnMut() -> Option<WindowControlArea>>,
    ) {
        self.inner.callbacks.borrow_mut().hit_test_window_control = Some(callback);
    }
    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.inner.callbacks.borrow_mut().close = Some(callback);
    }
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().appearance_changed = Some(callback);
    }
    fn draw(&self, scene: &Scene) {
        // 后台期间 surface 已销毁，跳过绘制。
        if !self.inner.surface_alive() {
            return;
        }
        self.inner.state.borrow_mut().renderer.draw(scene);
    }
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.inner.state.borrow().renderer.sprite_atlas().clone()
    }
    fn is_subpixel_rendering_supported(&self) -> bool {
        self.inner.state.borrow().renderer.supports_dual_source_blending()
    }
    fn gpu_specs(&self) -> Option<GpuSpecs> {
        Some(self.inner.state.borrow().renderer.gpu_specs())
    }
    fn update_ime_position(&self, bounds: Bounds<Pixels>) {
        // Record caret + ImeSession + pending IMM JNI queue. NativeActivity
        // still has no live `JNIEnv` / View-backed InputConnection.
        crate::ime::apply_update_ime_position_queued(
            &self.inner.last_ime_bounds,
            &self.inner.ime,
            &self.inner.pending_jni,
            f32::from(bounds.origin.x),
            f32::from(bounds.origin.y),
            f32::from(bounds.size.width),
            f32::from(bounds.size.height),
        );
    }
}
