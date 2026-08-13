//! GPUI 的 Android 平台层：以 android-activity(NativeActivity) 驱动主循环，
//! 渲染与文字复用上游 gpui_wgpu。桌面窗口专属能力（多窗口、剪贴板、菜单、
//! 文件对话框等）在 PoC 阶段以合理默认值 stub。

pub mod events;
pub mod timers;

#[cfg(target_os = "android")]
mod dispatcher;
#[cfg(target_os = "android")]
mod display;
#[cfg(target_os = "android")]
mod platform;
#[cfg(target_os = "android")]
mod window;

#[cfg(target_os = "android")]
pub use platform::AndroidPlatform;
