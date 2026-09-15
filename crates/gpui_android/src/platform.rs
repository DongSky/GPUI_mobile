//! GPUI 的 Android 主循环：以 android-activity(NativeActivity) 驱动事件泵，
//! 接线 dispatcher/display/window，加载系统字体，实现 `gpui::Platform`。

use crate::dispatcher::AndroidDispatcher;
use crate::display::AndroidDisplay;
use crate::events::TouchAction;
use crate::window::{AndroidWindow, AndroidWindowInner};
use android_activity::input::{InputEvent, MotionAction};
use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use anyhow::{Context as _, Result};
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DummyKeyboardMapper,
    ForegroundExecutor, Keymap, Menu, MenuItem, PathPromptOptions, Platform, PlatformDisplay,
    PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow,
    RunnableVariant, Task, ThermalState, WindowAppearance, WindowParams,
};
use gpui_wgpu::{CosmicTextSystem, GpuContext};
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

// 模拟器与真机通用的系统字体；缺失的仅记日志。
const SYSTEM_FONT_PATHS: &[&str] = &[
    "/system/fonts/Roboto-Regular.ttf",
    "/system/fonts/RobotoStatic-Regular.ttf",
    "/system/fonts/Roboto-Medium.ttf",
    "/system/fonts/NotoSansCJK-Regular.ttc",
    "/system/fonts/NotoSansSC-Regular.otf",
];

#[derive(Default)]
struct PlatformCallbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    keyboard_layout_change: Option<Box<dyn FnMut()>>,
    thermal_state_change: Option<Box<dyn FnMut()>>,
    system_wake: Option<Box<dyn FnMut()>>,
}

pub struct AndroidPlatform {
    app: AndroidApp,
    main_receiver: flume::Receiver<RunnableVariant>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    display: Rc<AndroidDisplay>,
    gpu_context: GpuContext,
    window: RefCell<Option<Rc<AndroidWindowInner>>>,
    active_window_handle: RefCell<Option<AnyWindowHandle>>,
    callbacks: RefCell<PlatformCallbacks>,
    quit_requested: Cell<bool>,
}

impl AndroidPlatform {
    /// 必须在 `android_main` 线程调用：`AndroidDispatcher::new` 会把调用线程记为
    /// GPUI 主线程（`is_main_thread()` 据此判断），后续 `Platform::run` 的事件泵
    /// 也只能在该线程上驱动（`android-activity` 对 `poll_events` 有同样的线程限制）。
    pub fn new(app: AndroidApp) -> Rc<Self> {
        let (dispatcher, main_receiver) = AndroidDispatcher::new(app.create_waker());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher.clone());
        let text_system = load_text_system();
        let scale = density_scale(&app);
        // 窗口尚未创建时以占位尺寸初始化，首个 InitWindow 后校正。
        let display = Rc::new(AndroidDisplay::new((1080, 2400), scale));
        Rc::new(Self {
            app,
            main_receiver,
            background_executor,
            foreground_executor,
            text_system,
            display,
            gpu_context: Rc::new(RefCell::new(None)),
            window: RefCell::new(None),
            active_window_handle: RefCell::new(None),
            callbacks: RefCell::new(PlatformCallbacks::default()),
            quit_requested: Cell::new(false),
        })
    }
}

fn density_scale(app: &AndroidApp) -> f32 {
    app.config().density().map(|dpi| dpi as f32 / 160.0).unwrap_or(1.0)
}

fn load_text_system() -> Arc<dyn PlatformTextSystem> {
    let text_system = Arc::new(CosmicTextSystem::new_without_system_fonts("Roboto"));
    let mut fonts: Vec<Cow<'static, [u8]>> = Vec::new();
    for path in SYSTEM_FONT_PATHS {
        match std::fs::read(path) {
            Ok(bytes) => fonts.push(Cow::Owned(bytes)),
            Err(error) => log::info!("skipping font {path}: {error}"),
        }
    }
    if let Err(error) = text_system.add_fonts(fonts) {
        log::error!("failed to load system fonts: {error:#}");
    }
    text_system
}

impl AndroidPlatform {
    fn handle_main_event(
        &self,
        event: MainEvent,
        launched: &mut bool,
        on_finish_launching: &mut Option<Box<dyn FnOnce()>>,
    ) {
        match event {
            MainEvent::InitWindow { .. } => {
                unsafe {
                    crate::ime::attach_native_activity(
                        self.app.vm_as_ptr(),
                        self.app.activity_as_ptr(),
                    );
                }
                if !*launched {
                    *launched = true;
                    // 首个 surface 就绪后才启动 app：open_window 需要 native_window。
                    if let Some(callback) = on_finish_launching.take() {
                        callback();
                    }
                } else if let Some(window) = self.window.borrow().clone() {
                    match self.app.native_window() {
                        Some(native_window) => {
                            if let Err(error) = window.handle_surface_recreated(native_window) {
                                log::error!("failed to recreate surface: {error:#}");
                            }
                        }
                        None => log::error!("InitWindow fired but native_window() is None"),
                    }
                }
            }
            MainEvent::TerminateWindow { .. } => {
                if let Some(window) = self.window.borrow().clone() {
                    window.handle_surface_destroyed();
                }
            }
            MainEvent::WindowResized { .. }
            | MainEvent::ContentRectChanged { .. }
            | MainEvent::ConfigChanged { .. } => {
                let scale = density_scale(&self.app);
                if let (Some(window), Some(native_window)) =
                    (self.window.borrow().clone(), self.app.native_window())
                {
                    window.handle_resize(
                        (native_window.width(), native_window.height()),
                        scale,
                    );
                }
            }
            MainEvent::GainedFocus | MainEvent::Resume { .. } => {
                if let Some(window) = self.window.borrow().clone() {
                    window.set_active(true);
                }
            }
            MainEvent::LostFocus | MainEvent::Pause => {
                if let Some(window) = self.window.borrow().clone() {
                    window.set_active(false);
                }
            }
            MainEvent::Destroy => {
                if let Some(mut callback) = self.callbacks.borrow_mut().quit.take() {
                    callback();
                }
                self.quit_requested.set(true);
            }
            MainEvent::InputAvailable
            | MainEvent::RedrawNeeded { .. }
            | MainEvent::Start
            | MainEvent::Stop
            | MainEvent::SaveState { .. }
            | MainEvent::LowMemory
            | MainEvent::InsetsChanged { .. } => {}
            // MainEvent 是 non_exhaustive。
            _ => {}
        }
    }

    fn pump_input(&self) {
        let Some(window) = self.window.borrow().clone() else {
            return;
        };
        match self.app.input_events_iter() {
            Ok(mut iter) => loop {
                let processed = iter.next(|event| {
                    if let InputEvent::MotionEvent(motion) = event {
                        let pointer = motion.pointer_at_index(motion.pointer_index().min(
                            motion.pointer_count().saturating_sub(1),
                        ));
                        let (x, y) = (pointer.x(), pointer.y());
                        let action = match motion.action() {
                            MotionAction::Down | MotionAction::PointerDown => {
                                Some(TouchAction::Down)
                            }
                            MotionAction::Move => Some(TouchAction::Move),
                            MotionAction::Up | MotionAction::PointerUp => Some(TouchAction::Up),
                            MotionAction::Cancel => Some(TouchAction::Cancel),
                            _ => None,
                        };
                        if let Some(action) = action {
                            window.handle_touch(action, x, y);
                            return InputStatus::Handled;
                        }
                    }
                    InputStatus::Unhandled
                });
                if !processed {
                    break;
                }
            },
            Err(error) => log::warn!("input_events_iter failed: {error:?}"),
        }
    }
}

impl Platform for AndroidPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }
    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }
    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        let mut on_finish_launching = Some(on_finish_launching);
        let mut launched = false;
        loop {
            if self.quit_requested.get() {
                break;
            }
            let has_surface = self
                .window
                .borrow()
                .as_ref()
                .is_some_and(|window| window.surface_alive());
            // 有 surface 时以约 60fps 节奏驱动帧；无 surface 时放慢轮询。
            // PoC 恒定重绘，AChoreographer vsync 留二期。
            let timeout = if has_surface {
                Duration::from_millis(16)
            } else {
                Duration::from_millis(100)
            };
            self.app.poll_events(Some(timeout), |event| match event {
                PollEvent::Wake | PollEvent::Timeout => {}
                PollEvent::Main(main_event) => {
                    self.handle_main_event(main_event, &mut launched, &mut on_finish_launching)
                }
                _ => {}
            });
            while let Ok(runnable) = self.main_receiver.try_recv() {
                runnable.run();
            }
            self.pump_input();
            if let Some(window) = self.window.borrow().clone() {
                if window.surface_alive() {
                    window.fire_request_frame();
                }
            }
        }
    }

    fn quit(&self) {
        self.quit_requested.set(true);
        // 自唤醒结束 poll 等待。
        self.app.create_waker().wake();
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {}
    fn activate(&self, _ignoring_other_apps: bool) {}
    fn hide(&self) {}
    fn hide_other_apps(&self) {}
    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.display.clone()]
    }
    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }
    fn active_window(&self) -> Option<AnyWindowHandle> {
        *self.active_window_handle.borrow()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        _params: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        unsafe {
            crate::ime::attach_native_activity(
                self.app.vm_as_ptr(),
                self.app.activity_as_ptr(),
            );
        }
        anyhow::ensure!(
            self.window.borrow().is_none(),
            "gpui_android supports a single window"
        );
        let native_window = self
            .app
            .native_window()
            .context("native window unavailable; open windows from the run() callback")?;
        let scale = density_scale(&self.app);
        self.display
            .update_size((native_window.width(), native_window.height()), scale);
        let inner =
            AndroidWindowInner::new(native_window, scale, &self.gpu_context, self.display.clone())?;
        *self.window.borrow_mut() = Some(inner.clone());
        *self.active_window_handle.borrow_mut() = Some(handle);
        Ok(Box::new(AndroidWindow::new(inner, self.display.clone())))
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn open_url(&self, url: &str) {
        log::warn!("open_url({url}) is not supported yet on Android");
    }
    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.callbacks.borrow_mut().open_urls = Some(callback);
    }
    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!("not supported on Android")))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (sender, receiver) = oneshot::channel();
        sender
            .send(Err(anyhow::anyhow!("file prompts are not supported on Android")))
            .ok();
        receiver
    }
    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (sender, receiver) = oneshot::channel();
        sender
            .send(Err(anyhow::anyhow!("file prompts are not supported on Android")))
            .ok();
        receiver
    }
    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }
    fn reveal_path(&self, _path: &Path) {}
    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().quit = Some(callback);
    }
    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().reopen = Some(callback);
    }
    fn on_system_wake(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().system_wake = Some(callback);
    }

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}
    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}
    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.callbacks.borrow_mut().app_menu_action = Some(callback);
    }
    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().will_open_app_menu = Some(callback);
    }
    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.callbacks.borrow_mut().validate_app_menu_command = Some(callback);
    }

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }
    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().thermal_state_change = Some(callback);
    }

    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow::anyhow!("app_path is not available on Android"))
    }
    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!("not available on Android"))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {}
    fn hide_cursor_until_mouse_moves(&self) {}
    fn is_cursor_visible(&self) -> bool {
        false
    }
    fn should_auto_hide_scrollbars(&self) -> bool {
        true
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        None
    }
    fn write_to_clipboard(&self, _item: ClipboardItem) {
        log::warn!("clipboard is not supported yet on Android");
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!("credential storage is not available on Android")))
    }
    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }
    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!("credential storage is not available on Android")))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(AndroidKeyboardLayout)
    }
    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }
    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().keyboard_layout_change = Some(callback);
    }
}

struct AndroidKeyboardLayout;

impl PlatformKeyboardLayout for AndroidKeyboardLayout {
    fn id(&self) -> &str {
        "android"
    }
    fn name(&self) -> &str {
        "Android"
    }
}



