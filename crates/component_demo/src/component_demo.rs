//! PoC-2：接入 gpui-component，展示 Button / Input（只读） / List（可滚动）。

#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use gpui::{App, Application, Context, IntoElement, ParentElement, Render, Styled, Window};
use gpui::prelude::*;
use gpui_android::AndroidPlatform;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputState};
use gpui_component::list::{List, ListDelegate, ListItem, ListState};
use gpui_component::{ActiveTheme, IndexPath, Root, v_flex};

struct RowListDelegate {
    rows: Vec<String>,
}

impl ListDelegate for RowListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.rows.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        self.rows
            .get(ix.row)
            .map(|row| ListItem::new(ix).child(row.clone()))
    }

    fn set_selected_index(
        &mut self,
        _ix: Option<IndexPath>,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) {
    }
}

struct ComponentDemoView {
    count: usize,
    input_state: gpui::Entity<InputState>,
    list_state: gpui::Entity<ListState<RowListDelegate>>,
}

impl ComponentDemoView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| {
            InputState::new(window, cx).default_value("gpui-component on Android")
        });
        let delegate = RowListDelegate {
            rows: (1..=20).map(|i| format!("Row {i}")).collect(),
        };
        let list_state = cx.new(|cx| ListState::new(delegate, window, cx).searchable(false));
        Self {
            count: 0,
            input_state,
            list_state,
        }
    }
}

impl Render for ComponentDemoView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .p_4()
            .gap_4()
            .child(
                Button::new("btn-count")
                    .primary()
                    .label(format!("Click me (count: {})", self.count))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.count += 1;
                        log::info!("count incremented to {}", this.count);
                        cx.notify();
                    })),
            )
            .child(Input::new(&self.input_state).readonly(true))
            .child(
                List::new(&self.list_state)
                    .flex_1()
                    .w_full(),
            )
    }
}

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("component_demo"),
    );
    std::panic::set_hook(Box::new(|info| {
        log::error!("PANIC: {info}");
    }));
    log::info!("android_main starting");

    let platform = AndroidPlatform::new(app);
    Application::with_platform(platform).run(|cx: &mut App| {
        gpui_component::init(cx);
        cx.open_window(gpui::WindowOptions::default(), |window, cx| {
            let view = cx.new(|cx| ComponentDemoView::new(window, cx));
            cx.new(|cx| Root::new(view, window, cx).bg(cx.theme().background))
        })
        .expect("failed to open window");
    });
}
