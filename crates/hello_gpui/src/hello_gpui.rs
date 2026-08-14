//! PoC-1：彩色矩形 + 中英文文字 + 触摸计数器。

#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use gpui::{
    App, Application, Context, IntoElement, ParentElement, Render, Styled, Window, WindowOptions,
    div, px, rgb,
};
use gpui::prelude::*;
use gpui_android::AndroidPlatform;

struct HelloView {
    count: usize,
}

impl Render for HelloView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .gap_8()
            .bg(rgb(0x10_30_40))
            .font_family("Roboto")
            .text_color(rgb(0xff_ff_ff))
            .child(div().w(px(220.)).h(px(120.)).rounded_lg().bg(rgb(0xff_57_22)))
            .child(format!("你好，GPUI on Android！Count: {}", self.count))
            .child(
                div()
                    .id("counter-button")
                    .px_6()
                    .py_3()
                    .rounded_md()
                    .bg(rgb(0x21_96_f3))
                    .child("Tap me +1")
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.count += 1;
                        log::info!("count incremented to {}", this.count);
                        cx.notify();
                    })),
            )
    }
}

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("hello_gpui"),
    );
    std::panic::set_hook(Box::new(|info| {
        log::error!("PANIC: {info}");
    }));
    log::info!("android_main starting");

    let platform = AndroidPlatform::new(app);
    Application::with_platform(platform).run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| HelloView { count: 0 })
        })
        .expect("failed to open window");
    });
}
