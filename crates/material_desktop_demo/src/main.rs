//! Desktop Material 3 / Expressive catalog.
//!
//! Paints `gpui_material::resolve()` appearances with the same GPUI `div`
//! mapping as Android `component_demo`. Tokens live only in `gpui_material`.
//!
//! Desktop type: Roboto when installed (`typography::desktop_font_family()`),
//! else Liberation Sans. Mesa llvmpipe / cosmic-text often collapses space
//! glyphs, so multi-word copy is mapped as separate word elements with
//! `WORD_GAP_DP`. Hardware GL is unavailable when `/dev/dri` is missing
//! (`scripts/desktop.sh` sets `WGPU_BACKEND=gl`).
//!
//! Boot pattern matches upstream Zed GPUI examples (`gpui_platform::application`).

#![cfg(not(target_os = "android"))]

use gpui::prelude::*;
use gpui::{
    black, canvas, div, point, px, size, Animation, AnimationExt, App, Bounds, Context,
    FillOptions, FillRule, FocusHandle, FontWeight, IntoElement, KeyDownEvent, MouseButton,
    MouseDownEvent, MouseMoveEvent, ParentElement, PathBuilder, PathStyle, Render, ScrollDelta,
    ScrollWheelEvent, SharedString, Stateful, StrokeOptions, Styled, TextRun, TitlebarOptions,
    Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui_material::components::date_picker::{self, CivilDate, DayKind};
use gpui_material::components::text_field::TextFieldEditor;
use gpui_material::components::time_picker::{self, DayPeriod, DialFace};
use gpui_material::components::{
    badge, bottom_sheet, button, button_group, carousel, checkbox, chip, dialog, fab_menu,
    icon_button, list, menu, navigation_bar, navigation_rail, photo_stub, progress, radio, search,
    side_sheet, slider, snackbar, split_button, switch, tabs, text_field, toolbar, tooltip,
    top_app_bar, Appearance,
};
use gpui_material::theme::Theme;
use gpui_material::typography;
use gpui_material::{Argb, InteractionState};
use gpui_platform::application;
use lyon::tessellation::{LineCap, LineJoin};
use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

fn paint(c: Argb) -> gpui::Rgba {
    gpui::Rgba {
        r: c.r() as f32 / 255.0,
        g: c.g() as f32 / 255.0,
        b: c.b() as f32 / 255.0,
        a: c.a() as f32 / 255.0,
    }
}

fn photo_mosaic(kind: photo_stub::PhotoKind, cols: u32, rows: u32) -> impl IntoElement {
    let cells = std::rc::Rc::new(kind.mosaic(cols, rows));
    div().size_full().flex().flex_col().children((0..rows).map({
        let cells = cells.clone();
        move |y| {
            let start = (y * cols) as usize;
            let row = cells[start..start + cols as usize].to_vec();
            div().flex().flex_1().w_full().children(
                row.into_iter()
                    .map(|c| div().flex_1().h_full().bg(paint(c))),
            )
        }
    }))
}

fn photo_fill(kind: photo_stub::PhotoKind) -> impl IntoElement {
    photo_mosaic(
        kind,
        photo_stub::MOSAIC_WIDE_COLS,
        photo_stub::MOSAIC_WIDE_ROWS,
    )
}

fn photo_avatar(kind: photo_stub::PhotoKind, size: f32) -> impl IntoElement {
    div()
        .w(px(size))
        .h(px(size))
        .rounded(px(size / 2.0))
        .overflow_hidden()
        .child(photo_mosaic(
            kind,
            photo_stub::MOSAIC_AVATAR,
            photo_stub::MOSAIC_AVATAR,
        ))
}

fn type_size(style: gpui_material::typography::TypeStyle) -> gpui::Pixels {
    px(style.size_sp)
}

fn type_weight(style: gpui_material::typography::TypeStyle) -> FontWeight {
    match style.weight {
        w if w >= 700 => FontWeight::BOLD,
        w if w >= 500 => FontWeight::MEDIUM,
        _ => FontWeight::NORMAL,
    }
}

fn feed_outline_verbs(
    builder: &mut PathBuilder,
    origin: gpui::Point<gpui::Pixels>,
    verbs: impl IntoIterator<Item = text_field::OutlineVerb>,
) {
    for v in verbs {
        match v {
            text_field::OutlineVerb::Move(x, y) => {
                builder.move_to(point(origin.x + px(x), origin.y + px(y)));
            }
            text_field::OutlineVerb::Line(x, y) => {
                builder.line_to(point(origin.x + px(x), origin.y + px(y)));
            }
            text_field::OutlineVerb::Arc {
                to_x,
                to_y,
                radius,
                clockwise,
            } => {
                builder.arc_to(
                    point(px(radius), px(radius)),
                    px(0.),
                    false,
                    clockwise,
                    point(origin.x + px(to_x), origin.y + px(to_y)),
                );
            }
            text_field::OutlineVerb::Cubic {
                c1_x,
                c1_y,
                c2_x,
                c2_y,
                to_x,
                to_y,
            } => {
                builder.cubic_bezier_to(
                    point(origin.x + px(to_x), origin.y + px(to_y)),
                    point(origin.x + px(c1_x), origin.y + px(c1_y)),
                    point(origin.x + px(c2_x), origin.y + px(c2_y)),
                );
            }
            text_field::OutlineVerb::Close => builder.close(),
        }
    }
}

fn stroke_round(width: f32) -> PathBuilder {
    PathBuilder::stroke(px(width)).with_style(PathStyle::Stroke(
        StrokeOptions::default()
            .with_line_width(width)
            .with_line_cap(gpui_line_cap(progress::STROKE_CAP))
            .with_line_join(LineJoin::Round),
    ))
}

fn gpui_line_cap(cap: progress::StrokeCap) -> LineCap {
    match cap {
        progress::StrokeCap::Round => LineCap::Round,
    }
}

fn measure_label_width_dp(window: &mut Window, label: &str, size_sp: f32) -> f32 {
    if label.is_empty() {
        return 0.0;
    }
    let run = TextRun {
        len: label.len(),
        font: gpui::font(typography::desktop_font_family()),
        color: black(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let layout = window
        .text_system()
        .layout_line(label, px(size_sp), &[run], None);
    f32::from(layout.width)
}

#[allow(dead_code)]
fn paint_search_scaled_fill(
    window: &mut Window,
    bounds: gpui::Bounds<gpui::Pixels>,
    scale: f32,
    radius: f32,
    color: gpui::Rgba,
) {
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let [pre, post] = search::top_center_scale_translates(
        f32::from(bounds.origin.x),
        f32::from(bounds.origin.y),
        w,
    );
    let r = radius.max(0.0).min(w / 2.0).min(h / 2.0);
    let k = gpui_material::shape::CIRCULAR_KAPPA;
    let verbs = if r < 0.5 {
        vec![
            text_field::OutlineVerb::Move(0.0, 0.0),
            text_field::OutlineVerb::Line(w, 0.0),
            text_field::OutlineVerb::Line(w, h),
            text_field::OutlineVerb::Line(0.0, h),
            text_field::OutlineVerb::Close,
        ]
    } else {
        vec![
            text_field::OutlineVerb::Move(r, 0.0),
            text_field::OutlineVerb::Line(w - r, 0.0),
            text_field::OutlineVerb::cubic_quarter((w - r, 0.0), (w, 0.0), (w, r), k),
            text_field::OutlineVerb::Line(w, h - r),
            text_field::OutlineVerb::cubic_quarter((w, h - r), (w, h), (w - r, h), k),
            text_field::OutlineVerb::Line(r, h),
            text_field::OutlineVerb::cubic_quarter((r, h), (0.0, h), (0.0, h - r), k),
            text_field::OutlineVerb::Line(0.0, r),
            text_field::OutlineVerb::cubic_quarter((0.0, r), (0.0, 0.0), (r, 0.0), k),
            text_field::OutlineVerb::Close,
        ]
    };
    let mut builder = PathBuilder::fill();
    builder.translate(point(px(pre.0), px(pre.1)));
    builder.scale(scale);
    builder.translate(point(px(post.0), px(post.1)));
    feed_outline_verbs(&mut builder, bounds.origin, verbs);
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

fn nav_rail_os_popup_options(spec: navigation_rail::OsPopupSpec) -> WindowOptions {
    WindowOptions {
        kind: WindowKind::PopUp,
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: point(px(0.), px(0.)),
            size: size(px(spec.width_dp), px(spec.height_dp)),
        })),
        titlebar: Some(TitlebarOptions {
            title: Some(spec.title.into()),
            ..Default::default()
        }),
        focus: spec.focus,
        show: true,
        is_movable: spec.movable,
        app_id: Some("dev.gpui.material_desktop_demo".into()),
        ..Default::default()
    }
}

fn paint_round_polyline(
    window: &mut Window,
    origin: gpui::Point<gpui::Pixels>,
    pts: &[(f32, f32)],
    stroke: f32,
    color: gpui::Rgba,
) {
    if pts.len() < 2 {
        return;
    }
    let mut builder = stroke_round(stroke);
    for (i, (x, y)) in pts.iter().enumerate() {
        let p = point(origin.x + px(*x), origin.y + px(*y));
        if i == 0 {
            builder.move_to(p);
        } else {
            builder.line_to(p);
        }
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

fn paint_filled_polygon(
    window: &mut Window,
    origin: gpui::Point<gpui::Pixels>,
    pts: &[(f32, f32)],
    color: gpui::Rgba,
) {
    if pts.len() < 3 {
        return;
    }
    let mut builder = PathBuilder::fill();
    for (i, (x, y)) in pts.iter().enumerate() {
        let p = point(origin.x + px(*x), origin.y + px(*y));
        if i == 0 {
            builder.move_to(p);
        } else {
            builder.line_to(p);
        }
    }
    builder.close();
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Overlay {
    None,
    Dialog,
    ListDialog,
    FullscreenDialog,
    Menu,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LiveMenuHost {
    Overlay,
    Cascade,
    StandardOverflow,
    ConnectedOverflow,
    Split,
}

impl LiveMenuHost {
    fn prefix(self) -> &'static str {
        match self {
            Self::Overlay => "ov",
            Self::Cascade => "cas",
            Self::StandardOverflow => "stdov",
            Self::ConnectedOverflow => "iconov",
            Self::Split => "split",
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::Overlay => "menu-overlay-cascade",
            Self::Cascade => "menu-cascade",
            Self::StandardOverflow => "std-overflow-cascade",
            Self::ConnectedOverflow => "icon-overflow-cascade",
            Self::Split => "split-menu-cascade",
        }
    }
}

struct CatalogView {
    dark: bool,
    taps: usize,
    checked: bool,
    switched: bool,
    overlay: Overlay,
    overlay_menu: menu::OverlayMenuSession,
    cascade_menu: menu::OverlayMenuSession,
    standard_overflow_menu: menu::OverlayMenuSession,
    connected_overflow_menu: menu::OverlayMenuSession,
    split_menu: menu::OverlayMenuSession,
    slider: f32,
    ringtone: usize,
    tab: usize,
    picker_year: i32,
    picker_month: u32,
    selected: CivilDate,
    today: CivilDate,
    filled: TextFieldEditor,
    outlined: TextFieldEditor,
    group_selected: usize,
    standard_selected: usize,
    list_selected: usize,
    list_swipe: list::ListSwipeState,
    list_swipe_at: Option<Instant>,
    list_order: [usize; 3],
    tooltip_plain_open: bool,
    tooltip_rich_open: bool,
    icon_selected: usize,
    overflow_open: bool,
    standard_overflow_open: bool,
    range_start: f32,
    range_end: f32,
    range_drag: Option<slider::RangeThumb>,
    range_focus: slider::RangeThumb,
    range_moved: bool,
    range_hit: Rc<Cell<(f32, f32)>>,
    docked_open: bool,
    date_display: date_picker::DatePickerDisplayMode,
    date_pane: date_picker::DatePickerPane,
    date_range: date_picker::DateRangeSelection,
    date_range_committed: date_picker::DateRangeSelection,
    range_year: i32,
    range_month: u32,
    range_pane: date_picker::DatePickerPane,
    range_display: date_picker::DatePickerDisplayMode,
    search_open: bool,
    search: TextFieldEditor,
    search_filter: search::SearchFilter,
    rail_selected: usize,
    rail_mode: navigation_rail::RailMode,
    wide_rail_mode: navigation_rail::RailMode,
    narrow_rail_mode: navigation_rail::RailMode,
    hide_rail_mode: navigation_rail::RailMode,
    header_rail_mode: navigation_rail::RailMode,
    header_tooltip_open: bool,
    carousel_index: usize,
    carousel_layout: carousel::CarouselLayout,
    carousel_fling: carousel::FlingState,
    carousel_fling_at: Option<Instant>,
    typeahead_focus: FocusHandle,
    typeahead_host: Option<LiveMenuHost>,
    chip_pressed: Option<u32>,
    chip_press_seq: u32,
    chip_press_anim: Option<u32>,
    snack_state: snackbar::SnackbarState,
    snack_at: Instant,
    fab_menu_open: bool,
    split_open: bool,
    app_bar_collapse: f32,
    last_catalog_ime: Option<[f32; 4]>,
    time_hour: u8,
    time_minute: u8,
    time_period: DayPeriod,
    time_dial: DialFace,
    time_hand_from: f32,
    time_hand_gen: u32,
    time_scroll: time_picker::TimeScrollState,
    time_scroll_at: Option<Instant>,
    time_display: time_picker::TimePickerDisplayMode,
    time_input: time_picker::TimeInputState,
    time_format: time_picker::TimeFormat,
}

impl CatalogView {
    fn theme(&self) -> Theme {
        if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }

    fn bump_time_hand(&mut self) {
        self.time_hand_from =
            time_picker::hand_angle_deg(self.time_dial, self.time_dial_hour(), self.time_minute);
        self.time_hand_gen = self.time_hand_gen.wrapping_add(1);
    }

    fn time_dial_hour(&self) -> u8 {
        time_picker::dial_clock_hour(self.time_hour, self.time_format, self.time_period)
    }

    fn sync_period_from_hour(&mut self) {
        if self.time_format.is_24_hour() {
            self.time_period = time_picker::to_hour12(self.time_hour).1;
        }
    }

    fn live_menu(&self, host: LiveMenuHost) -> menu::OverlayMenuSession {
        match host {
            LiveMenuHost::Overlay => self.overlay_menu,
            LiveMenuHost::Cascade => self.cascade_menu,
            LiveMenuHost::StandardOverflow => self.standard_overflow_menu,
            LiveMenuHost::ConnectedOverflow => self.connected_overflow_menu,
            LiveMenuHost::Split => self.split_menu,
        }
    }

    fn live_menu_mut(&mut self, host: LiveMenuHost) -> &mut menu::OverlayMenuSession {
        match host {
            LiveMenuHost::Overlay => &mut self.overlay_menu,
            LiveMenuHost::Cascade => &mut self.cascade_menu,
            LiveMenuHost::StandardOverflow => &mut self.standard_overflow_menu,
            LiveMenuHost::ConnectedOverflow => &mut self.connected_overflow_menu,
            LiveMenuHost::Split => &mut self.split_menu,
        }
    }

    fn focus_typeahead(&mut self, host: LiveMenuHost, window: &mut Window, cx: &mut Context<Self>) {
        if !menu::TYPEAHEAD_AUTOFOCUS {
            return;
        }
        self.typeahead_host = Some(host);
        self.typeahead_focus.focus(window, cx);
    }

    fn ensure_in_page_typeahead(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if menu::typeahead_autofocus_in_page(true) && self.typeahead_host.is_none() {
            self.focus_typeahead(LiveMenuHost::Cascade, window, cx);
        }
    }

    fn hover_live_parent(&mut self, host: LiveMenuHost, index: usize, cx: &mut Context<Self>) {
        let was = self.live_menu(host);
        let intent = self.live_menu_mut(host).hover_parent(index);
        if self.live_menu(host) != was {
            cx.notify();
        }
        if let menu::HoverOpenIntent::Delay { index, seq } = intent {
            cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(menu::HOVER_OPEN_DELAY_MS))
                    .await;
                this.update(cx, |this, cx| {
                    if this.live_menu_mut(host).confirm_hover_open(index, seq) {
                        cx.notify();
                    }
                })
                .ok();
            })
            .detach();
        }
    }

    fn tick_list_swipe(&mut self, cx: &mut Context<Self>) {
        if self.list_swipe.resting() {
            self.list_swipe_at = None;
            return;
        }
        let now = Instant::now();
        let dt = self
            .list_swipe_at
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(list::SWIPE_FRAME_DT);
        self.list_swipe_at = Some(now);
        self.list_swipe.step_live(dt);
        if self.list_swipe.needs_frame() {
            cx.notify();
        }
    }

    fn tick_carousel_fling(&mut self, cx: &mut Context<Self>) {
        if self.carousel_fling.resting() {
            self.carousel_fling.selected = self.carousel_index;
            self.carousel_fling_at = None;
            return;
        }
        let now = Instant::now();
        let dt = self
            .carousel_fling_at
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(carousel::FLING_FRAME_DT);
        self.carousel_fling_at = Some(now);
        self.carousel_index = self.carousel_fling.step_live(dt);
        if self.carousel_fling.needs_frame() {
            cx.notify();
        }
    }

    fn tick_time_scroll(&mut self, cx: &mut Context<Self>) {
        if self.time_scroll.resting() {
            self.time_hour = self.time_scroll.hour_value();
            self.time_minute = self.time_scroll.minute_value();
            self.sync_period_from_hour();
            self.time_scroll_at = None;
            return;
        }
        let now = Instant::now();
        let dt = self
            .time_scroll_at
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(time_picker::SCROLL_FRAME_DT);
        self.time_scroll_at = Some(now);
        self.time_scroll.step_live(dt);
        self.time_hour = self.time_scroll.hour_value();
        self.time_minute = self.time_scroll.minute_value();
        self.sync_period_from_hour();
        if self.time_scroll.needs_frame() {
            cx.notify();
        }
    }

    fn toggle_time_format(&mut self) {
        self.time_format = time_picker::apply_format_toggle(
            self.time_format,
            &mut self.time_period,
            &mut self.time_scroll,
            &mut self.time_input,
            &mut self.time_hour,
            self.time_minute,
        );
        self.bump_time_hand();
    }

    fn toggle_time_display(&mut self) {
        self.time_display = time_picker::apply_display_toggle(
            self.time_display,
            &mut self.time_scroll,
            &mut self.time_input,
            &mut self.time_hour,
            &mut self.time_minute,
        );
    }

    fn toggle_date_display(&mut self) {
        self.date_display = date_picker::apply_display_toggle(self.date_display);
        if self.date_display == date_picker::DatePickerDisplayMode::Input {
            self.date_pane = date_picker::DatePickerPane::Calendar;
        }
    }

    fn toggle_date_pane(&mut self) {
        if self.date_display == date_picker::DatePickerDisplayMode::Picker {
            self.date_pane = date_picker::apply_pane_toggle(self.date_pane);
        }
    }

    fn select_picker_year(&mut self, year: i32) {
        self.picker_year = date_picker::clamp_year(year);
        self.date_pane = date_picker::DatePickerPane::Calendar;
    }

    fn tap_date_range(&mut self, day: CivilDate) {
        self.date_range = date_picker::apply_range_tap(self.date_range, day);
    }

    fn shift_range_month(&mut self, delta: i32) {
        if self.range_display != date_picker::DatePickerDisplayMode::Picker
            || self.range_pane != date_picker::DatePickerPane::Calendar
        {
            return;
        }
        let (y, m) = date_picker::apply_range_month(self.range_year, self.range_month, delta);
        self.range_year = y;
        self.range_month = m;
    }

    fn toggle_range_pane(&mut self) {
        if self.range_display == date_picker::DatePickerDisplayMode::Picker {
            self.range_pane = date_picker::apply_pane_toggle(self.range_pane);
        }
    }

    fn toggle_range_display(&mut self) {
        self.range_display = date_picker::apply_display_toggle(self.range_display);
        self.range_pane = date_picker::DatePickerPane::Calendar;
    }

    fn confirm_date_range(&mut self) {
        self.date_range_committed = date_picker::apply_range_confirm(self.date_range);
    }

    fn dismiss_date_range(&mut self) {
        self.date_range = date_picker::apply_range_dismiss(self.date_range_committed);
        let (y, m) = date_picker::range_month_of(self.date_range);
        self.range_year = y;
        self.range_month = m;
        self.range_pane = date_picker::DatePickerPane::Calendar;
    }

    fn select_range_year(&mut self, year: i32) {
        let (y, m) = date_picker::apply_range_year(self.range_year, self.range_month, year);
        self.range_year = y;
        self.range_month = m;
        self.range_pane = date_picker::DatePickerPane::Calendar;
    }

    fn tick_snack(&mut self, cx: &mut Context<Self>) {
        if !self.snack_state.visible {
            return;
        }
        let elapsed = self.snack_at.elapsed().as_secs_f32() * 1000.0;
        let left = (snackbar::TIMEOUT_SHORT_MS as f32 - elapsed).max(0.0);
        self.snack_state.remaining_ms = left;
        if left <= 0.0 {
            self.snack_state.visible = false;
            cx.notify();
        }
    }
}

impl Render for CatalogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.tick_carousel_fling(cx);
        self.tick_list_swipe(cx);
        self.tick_time_scroll(cx);
        self.tick_snack(cx);
        self.ensure_in_page_typeahead(window, cx);
        let theme = self.theme();
        let c = theme.color;
        let bar = top_app_bar::resolve(&theme);

        let chrome = div()
            .h(px(bar.height_dp))
            .w_full()
            .px(px(16.))
            .flex()
            .items_center()
            .justify_between()
            .bg(paint(bar.container))
            .child(
                div()
                    .text_size(px(bar.title_style.size_sp))
                    .text_color(paint(bar.title))
                    .whitespace_nowrap()
                    .font_weight(type_weight(bar.title_style.emphasized()))
                    .child(spaced_line(
                        "Material 3 desktop",
                        bar.title_style.emphasized().size_sp,
                        paint(bar.title),
                    )),
            )
            .child(
                div()
                    .id("theme-toggle")
                    .px(px(12.))
                    .py(px(8.))
                    .rounded(px(20.))
                    .bg(paint(c.secondary_container))
                    .text_color(paint(c.on_secondary_container))
                    .text_size(type_size(theme.typography.label_large))
                    .child(if self.dark { "Dark" } else { "Light" })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.dark = !this.dark;
                        cx.notify();
                    })),
            );

        let show_menu = matches!(self.overlay, Overlay::Menu);
        let body = match self.overlay {
            Overlay::None | Overlay::Menu => catalog_body(self, &theme, cx).into_any_element(),
            Overlay::Dialog => dialog_overlay(&theme, cx).into_any_element(),
            Overlay::ListDialog => list_dialog_overlay(self, &theme, cx).into_any_element(),
            Overlay::FullscreenDialog => fullscreen_dialog_overlay(&theme, cx).into_any_element(),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(paint(c.background))
            .font_family(typography::desktop_font_family())
            .text_color(paint(c.on_background))
            .child(chrome)
            .child(
                div()
                    .id("catalog-host")
                    .relative()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .w_full()
                    .child(body)
                    .when(show_menu, |el| el.child(menu_overlay(self, &theme, cx))),
            )
    }
}

fn catalog_body(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let c = theme.color;
    let filled_a = this.filled.appearance(theme);
    let outlined_a = this.outlined.appearance(theme);
    let empty_filled = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        false,
    );
    let empty_outlined = text_field::resolve_expressive(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Enabled,
        false,
    );
    let check = checkbox::resolve(
        theme,
        if this.checked {
            checkbox::CheckValue::Checked
        } else {
            checkbox::CheckValue::Unchecked
        },
        InteractionState::Enabled,
    );
    let sw = switch::resolve(theme, this.switched, InteractionState::Enabled);
    let tabs_p = tabs::resolve(theme, tabs::TabsVariant::Primary);
    let pick = date_picker::resolve(theme);
    let cells = date_picker::month_grid_classified(
        this.picker_year,
        this.picker_month,
        this.selected,
        this.today,
    );

    div()
        .id("catalog-scroll")
        .flex_1()
        .w_full()
        .overflow_y_scroll()
        .p(px(20.))
        .gap(px(16.))
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(px(theme.typography.body_medium.size_sp))
                .text_color(paint(c.on_surface_variant))
                .whitespace_nowrap()
                .child(spaced_line(
                    "Desktop catalog · same gpui_material::resolve() as Android + HTML.",
                    theme.typography.body_medium.size_sp,
                    paint(c.on_surface_variant),
                )),
        )
        .child(settings_scene(this, theme, cx))
        .child(section_title(theme, "Buttons"))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .children(button::OVERVIEW_ORDER.iter().map(|variant| {
                    m_button(
                        variant.label(),
                        theme,
                        *variant,
                        InteractionState::Enabled,
                        variant.overview_label(),
                        cx.listener(|this, _, _, cx| {
                            this.taps += 1;
                            cx.notify();
                        }),
                    )
                    .into_any_element()
                })),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .items_center()
                .children(button::ButtonSize::ALL.iter().map(|size| {
                    let a = button::resolve_expressive(
                        theme,
                        button::ButtonVariant::Filled,
                        *size,
                        button::ButtonShape::Round,
                        InteractionState::Enabled,
                    );
                    div()
                        .h(px(a.height_dp))
                        .px(px(a.pad_start_dp))
                        .rounded(px(a.corners.top_left))
                        .bg(paint(a.container))
                        .text_color(paint(a.content))
                        .text_size(type_size(a.label_style))
                        .font_weight(FontWeight::MEDIUM)
                        .flex()
                        .items_center()
                        .child("Label")
                })),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(paint(c.on_surface_variant))
                .child(format!(
                    "Filled tap count {} · sizes XS–XL from resolve_expressive()",
                    this.taps
                )),
        )
        .child(connected_button_group(theme, this.group_selected, cx))
        .child(standard_button_group(
            this,
            theme,
            this.standard_selected,
            this.standard_overflow_open,
            cx,
        ))
        .child(connected_icon_group(
            this,
            theme,
            this.icon_selected,
            this.overflow_open,
            cx,
        ))
        .child(section_title(theme, "Split button"))
        .child(desktop_split_button(this, theme, cx))
        .child(section_title(theme, "Icon buttons"))
        .child(div().flex().gap(px(8.)).items_center().children(
            icon_button::IconButtonVariant::ALL.iter().map(|variant| {
                let a = icon_button::resolve(theme, *variant, InteractionState::Enabled);
                paint_icon_button(&a)
            }),
        ))
        .child(desktop_icon_button_widths(theme))
        .child(desktop_icon_button_toggles(theme))
        .child(section_title(theme, "FAB menu"))
        .child(desktop_fab_menu(this, theme, cx))
        .child(section_title(theme, "Toolbars"))
        .child(desktop_toolbar(theme))
        .child(section_title(theme, "Text fields"))
        .child(field_block(
            "hero-empty-filled",
            &empty_filled,
            "Label",
            "",
            "",
            |_, _, _| {},
        ))
        .child(field_block(
            "hero-empty-outlined",
            &empty_outlined,
            "Label",
            "",
            "",
            |_, _, _| {},
        ))
        .child(field_block(
            "field-outlined",
            &outlined_a,
            "Email",
            this.outlined.display_with_caret(),
            if this.outlined.error {
                "Enter a valid email"
            } else {
                "Supporting text"
            },
            cx.listener(|this, _, _, cx| {
                this.filled.set_focus(false);
                this.outlined.set_focus(true);
                this.outlined.insert_char('a');
                let v = this.outlined.value().to_string();
                this.outlined.error = !v.is_empty() && !text_field::looks_like_email(&v);
                this.last_catalog_ime = text_field::catalog_ime_from_focused(&this.outlined, 16.0)
                    .map(|r| [r.0, r.1, r.2, r.3]);
                cx.notify();
            }),
        ))
        .child(field_block(
            "field-filled",
            &filled_a,
            "Label",
            this.filled.display_with_caret(),
            "Supporting text",
            cx.listener(|this, _, _, cx| {
                this.outlined.set_focus(false);
                this.filled.set_focus(true);
                this.filled.insert_char('x');
                this.last_catalog_ime = text_field::catalog_ime_from_focused(&this.filled, 16.0)
                    .map(|r| [r.0, r.1, r.2, r.3]);
                cx.notify();
            }),
        ))
        .child(section_title(theme, "Slider"))
        .child(volume_slider_scene(theme, this.slider, cx))
        .child(range_slider_hero(this, theme, cx))
        .child(section_title(theme, "Progress"))
        .child(progress_heroes(theme))
        .child(section_title(theme, "Carousel"))
        .child(carousel_hero(this, theme, cx))
        .child(
            div()
                .text_size(px(12.))
                .text_color(paint(c.on_surface_variant))
                .child(spaced_line(
                    format!(
                        "Media {:.0}% · XS track 16 / handle visual 28 (token 44) · Alarm 13 stops",
                        this.slider * 100.0
                    ),
                    12.0,
                    paint(c.on_surface_variant),
                )),
        )
        .child(section_title(theme, "Selection · tabs"))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(16.))
                .child(
                    div()
                        .id("checkbox")
                        .w(px(check.target_dp))
                        .h(px(check.target_dp))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(check.box_size_dp))
                                .h(px(check.box_size_dp))
                                .rounded(px(check.corner_dp))
                                .bg(paint(check.box_fill))
                                .when(check.box_outline.is_some(), |el| {
                                    el.border_1()
                                        .border_color(paint(check.box_outline.unwrap()))
                                })
                                .text_color(paint(check.icon))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(if this.checked { "✓" } else { "" }),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.checked = !this.checked;
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .id("switch")
                        .w(px(sw.track_w))
                        .h(px(sw.track_h))
                        .rounded(px(sw.track_h / 2.0))
                        .bg(paint(sw.track))
                        .when(sw.track_outline.is_some(), |el| {
                            el.border_2().border_color(paint(sw.track_outline.unwrap()))
                        })
                        .flex()
                        .items_center()
                        .px(px(4.))
                        .when(this.switched, |el| el.justify_end())
                        .child(
                            div()
                                .w(px(sw.thumb_dp))
                                .h(px(sw.thumb_dp))
                                .rounded(px(sw.thumb_dp / 2.0))
                                .bg(paint(sw.thumb)),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.switched = !this.switched;
                            cx.notify();
                        })),
                ),
        )
        .child(tab_row(&tabs_p, this.tab, cx))
        .child(section_title(theme, "Lists"))
        .child(desktop_lists(this, theme, cx))
        .child(desktop_list_swipe(this, theme, cx))
        .child(desktop_list_reorder(this, theme, cx))
        .child(desktop_media_scene(this, theme, cx))
        .child(section_title(theme, "Chips"))
        .child(desktop_chips(this, theme, cx))
        .child(section_title(theme, "Menu"))
        .child(desktop_menus(this, theme, cx))
        .child(section_title(theme, "Snackbar"))
        .child(desktop_mail_snack(this, theme, cx))
        .child(section_title(theme, "Navigation bar"))
        .child(desktop_nav_bars(theme))
        .child(section_title(theme, "Tooltip"))
        .child(desktop_tooltips(this, theme, cx))
        .child(section_title(theme, "Top app bar"))
        .child(desktop_app_bar_scene(this, theme, cx))
        .child(section_title(theme, "Bottom sheet"))
        .child(desktop_share_sheet(theme))
        .child(section_title(theme, "Side sheet"))
        .child(desktop_side_sheet(theme))
        .child(section_title(theme, "Navigation rail"))
        .child(wide_rail_icon_hero(this, theme, cx))
        .child(wide_rail_in_flow_hero(this, theme, cx))
        .child(nav_rail_hero(this, theme, cx))
        .child(narrow_rail_hero(this, theme, cx))
        .child(hide_rail_hero(this, theme, cx))
        .child(header_rail_hero(this, theme, cx))
        .child(section_title(theme, "Dialog"))
        .child(
            div()
                .flex()
                .gap(px(8.))
                .child(m_button(
                    "open-dialog",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Reset settings",
                    cx.listener(|this, _, _, cx| {
                        this.overlay = Overlay::Dialog;
                        cx.notify();
                    }),
                ))
                .child(m_button(
                    "open-list-dialog",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Phone ringtone",
                    cx.listener(|this, _, _, cx| {
                        this.overlay = Overlay::ListDialog;
                        cx.notify();
                    }),
                ))
                .child(m_button(
                    "open-fullscreen-dialog",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Event",
                    cx.listener(|this, _, _, cx| {
                        this.overlay = Overlay::FullscreenDialog;
                        cx.notify();
                    }),
                ))
                .child(m_button(
                    "open-menu",
                    theme,
                    button::ButtonVariant::Tonal,
                    InteractionState::Enabled,
                    "Menu",
                    cx.listener(|this, _, window, cx| {
                        this.overlay = Overlay::Menu;
                        this.overlay_menu = menu::OverlayMenuSession::overlay();
                        this.focus_typeahead(LiveMenuHost::Overlay, window, cx);
                        cx.notify();
                    }),
                )),
        )
        .child(section_title(theme, "Search"))
        .child(search_bar_hero(this, theme, cx))
        .child(section_title(theme, "Time picker"))
        .child(time_scroll_hero(this, theme, cx))
        .child(time_picker_hero(this, theme, cx))
        .child(section_title(theme, "Date picker"))
        .child(date_range_hero(this, theme, &pick, cx))
        .child(docked_date_picker(this, theme, &pick, &cells, cx))
        .child(date_picker_card(this, theme, &pick, &cells, cx))
        .child(date_input_card(this, theme, &pick))
        .child(date_range_input_card(theme, &pick))
}

fn paint_icon_button(a: &Appearance) -> impl IntoElement + use<> {
    paint_icon_button_glyph(a, "★")
}

fn paint_icon_button_glyph(a: &Appearance, glyph: &'static str) -> impl IntoElement + use<> {
    let w = a.width_dp.unwrap_or(a.height_dp);
    div()
        .w(px(w))
        .h(px(a.height_dp))
        .rounded(px(a.corners.top_left))
        .bg(paint(a.container))
        .text_color(paint(a.content))
        .flex()
        .items_center()
        .justify_center()
        .when(a.outline.is_some(), |el| {
            el.border_1().border_color(paint(a.outline.unwrap().0))
        })
        .child(glyph)
}

fn desktop_icon_button_widths(theme: &Theme) -> impl IntoElement {
    div().flex().flex_col().gap(px(8.)).children(
        [
            icon_button::WIDTH_HERO_SIZE,
            icon_button::WIDTH_HERO_SIZE_MEDIUM,
        ]
        .iter()
        .map(|size| {
            div().flex().gap(px(8.)).items_center().children(
                icon_button::IconButtonWidth::ALL.iter().map(|width| {
                    let a = icon_button::resolve_width(
                        theme,
                        icon_button::IconButtonVariant::Filled,
                        *size,
                        button::ButtonShape::Round,
                        *width,
                        InteractionState::Enabled,
                    );
                    paint_icon_button(&a)
                }),
            )
        }),
    )
}

fn desktop_icon_button_toggles(theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            div().flex().gap(px(8.)).items_center().children(
                icon_button::IconButtonVariant::TOGGLE_OVERVIEW
                    .iter()
                    .flat_map(|variant| {
                        icon_button::IconButtonSelection::TOGGLE
                            .iter()
                            .map(|selection| {
                                let a = icon_button::resolve_selection(
                                    theme,
                                    *variant,
                                    icon_button::TOGGLE_HERO_SIZE,
                                    button::ButtonShape::Round,
                                    icon_button::IconButtonWidth::Default,
                                    *selection,
                                    InteractionState::Enabled,
                                );
                                paint_icon_button_glyph(&a, selection.glyph())
                            })
                    }),
            ),
        )
        .child(
            div().flex().gap(px(8.)).items_center().children(
                icon_button::IconButtonSelection::TOGGLE
                    .iter()
                    .map(|selection| {
                        let a = icon_button::resolve_selection(
                            theme,
                            icon_button::IconButtonVariant::Filled,
                            icon_button::TOGGLE_HERO_SIZE,
                            button::ButtonShape::Square,
                            icon_button::IconButtonWidth::Default,
                            *selection,
                            InteractionState::Enabled,
                        );
                        paint_icon_button_glyph(&a, selection.glyph())
                    }),
            ),
        )
}

fn paint_chip(theme: &Theme, demo: chip::ChipDemo, id: u32, pressed: bool) -> Stateful<gpui::Div> {
    let state = if pressed {
        InteractionState::Pressed
    } else {
        demo.state
    };
    let a = chip::resolve_style(
        theme,
        demo.variant,
        demo.color,
        demo.selected,
        state,
        demo.leading,
        demo.avatar,
    );
    let r = chip::animated_corner_dp(theme, demo.variant, demo.selected, chip::press_t(state));
    let lead = chip::demo_leading_icon(demo);
    let avatar = chip::demo_avatar(demo);
    let trail = chip::trailing_icon(demo.variant);
    let ico = a.secondary_content.unwrap_or(a.content);
    div()
        .id(SharedString::from(format!("chip-{id}")))
        .h(px(a.height_dp))
        .pl(px(a.pad_start_dp))
        .pr(px(a.pad_end_dp))
        .rounded(px(r))
        .bg(paint(a.container))
        .text_color(paint(a.content))
        .flex()
        .items_center()
        .gap(px(chip::demo_icon_gap_dp(demo)))
        .when(a.elevation_dp > 0.0, |el| el.shadow_sm())
        .when(a.outline.is_some(), |el| {
            el.border_1().border_color(paint(a.outline.unwrap().0))
        })
        .children(avatar.map(|kind| photo_avatar(kind, chip::AVATAR_DP)))
        .children(lead.map(|g| {
            div()
                .w(px(chip::ICON_DP))
                .h(px(chip::ICON_DP))
                .flex()
                .items_center()
                .justify_center()
                .text_color(paint(ico))
                .child(g)
        }))
        .child(demo.label)
        .children(trail.map(|g| {
            div()
                .w(px(chip::ICON_DP))
                .h(px(chip::ICON_DP))
                .flex()
                .items_center()
                .justify_center()
                .text_color(paint(ico))
                .child(g)
        }))
}

fn live_chip(
    theme: &Theme,
    demo: chip::ChipDemo,
    id: u32,
    pressed: bool,
    animate: bool,
    press_seq: u32,
    cx: &mut Context<CatalogView>,
) -> gpui::AnyElement {
    let el = bind_chip_press(paint_chip(theme, demo, id, pressed), id, cx);
    if animate && demo.variant.morphs() {
        let theme = *theme;
        let variant = demo.variant;
        let selected = demo.selected;
        el.with_animation(
            SharedString::from(format!(
                "chip-press-{}-{}-{}",
                id,
                press_seq,
                if pressed { "in" } else { "out" }
            )),
            Animation::new(Duration::from_millis(chip::press_ms(&theme) as u64)),
            move |this, delta| {
                let t = chip::press_t_anim(pressed, delta);
                this.rounded(px(chip::animated_corner_dp(&theme, variant, selected, t)))
            },
        )
        .into_any_element()
    } else {
        el.into_any_element()
    }
}

fn bind_chip_press(
    el: Stateful<gpui::Div>,
    id: u32,
    cx: &mut Context<CatalogView>,
) -> Stateful<gpui::Div> {
    el.on_mouse_down(
        MouseButton::Left,
        cx.listener(move |this, _, _, cx| {
            this.chip_press_seq = this.chip_press_seq.wrapping_add(1);
            this.chip_press_anim = Some(id);
            this.chip_pressed = Some(id);
            cx.notify();
        }),
    )
    .on_mouse_up(
        MouseButton::Left,
        cx.listener(move |this, _, _, cx| {
            this.chip_press_seq = this.chip_press_seq.wrapping_add(1);
            this.chip_press_anim = Some(id);
            this.chip_pressed = None;
            cx.notify();
        }),
    )
}

fn desktop_chips(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let pressed = this.chip_pressed;
    let press_seq = this.chip_press_seq;
    let anim = this.chip_press_anim;
    let filter: Vec<_> = chip::FILTER_HERO
        .iter()
        .enumerate()
        .map(|(i, demo)| {
            let id = 10 + i as u32;
            live_chip(
                theme,
                *demo,
                id,
                pressed == Some(id),
                anim == Some(id),
                press_seq,
                cx,
            )
        })
        .collect();
    let tonal: Vec<_> = chip::TONAL_FILTER_HERO
        .iter()
        .enumerate()
        .map(|(i, demo)| {
            let id = 20 + i as u32;
            live_chip(
                theme,
                *demo,
                id,
                pressed == Some(id),
                anim == Some(id),
                press_seq,
                cx,
            )
        })
        .collect();
    let elevated: Vec<_> = chip::ELEVATED_FILTER_HERO
        .iter()
        .enumerate()
        .map(|(i, demo)| {
            let id = 30 + i as u32;
            live_chip(
                theme,
                *demo,
                id,
                pressed == Some(id),
                anim == Some(id),
                press_seq,
                cx,
            )
        })
        .collect();
    let input: Vec<_> = chip::INPUT_HERO
        .iter()
        .enumerate()
        .map(|(i, demo)| {
            let id = 40 + i as u32;
            live_chip(
                theme,
                *demo,
                id,
                pressed == Some(id),
                anim == Some(id),
                press_seq,
                cx,
            )
        })
        .collect();
    let avatars: Vec<_> = chip::INPUT_AVATAR_HERO
        .iter()
        .enumerate()
        .map(|(i, demo)| {
            let id = 50 + i as u32;
            live_chip(
                theme,
                *demo,
                id,
                pressed == Some(id),
                anim == Some(id),
                press_seq,
                cx,
            )
        })
        .collect();
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .items_center()
                .children(filter),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .items_center()
                .children(tonal),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .items_center()
                .children(elevated),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .items_center()
                .children(input),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .items_center()
                .children(avatars),
        )
}

fn paint_menu_item(item: menu::MenuDemoItem, a: menu::MenuItemAppearance) -> impl IntoElement {
    let trail = menu::trailing_text(&item);
    div()
        .h(px(a.height_dp))
        .px(px(a.pad_h_dp))
        .rounded_tl(px(a.corners.top_left))
        .rounded_tr(px(a.corners.top_right))
        .rounded_br(px(a.corners.bottom_right))
        .rounded_bl(px(a.corners.bottom_left))
        .bg(paint(a.container))
        .text_color(paint(a.label))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(menu::ITEM_BETWEEN_SPACE_DP))
        .child(
            div()
                .w(px(menu::ICON_DP))
                .text_color(paint(a.icon))
                .child(item.icon),
        )
        .child(div().flex_1().child(item.label))
        .when(!trail.is_empty(), |el| {
            el.child(div().text_color(paint(a.shortcut)).child(trail))
        })
}

fn desktop_vertical_menu(theme: &Theme, scheme: menu::MenuScheme) -> impl IntoElement {
    desktop_vertical_menu_focus(theme, scheme, menu::MenuFocus::Rest, false)
}

fn desktop_vertical_menu_focus(
    theme: &Theme,
    scheme: menu::MenuScheme,
    focus: menu::MenuFocus,
    submenu_open: bool,
) -> impl IntoElement {
    let groups = menu::VERTICAL_GROUPS;
    let group_count = groups.len();
    div()
        .flex()
        .flex_col()
        .gap(px(menu::GROUP_GAP_DP))
        .w(px(220.))
        .children(groups.iter().enumerate().map(move |(gi, group)| {
            let shell = menu::resolve_group_focus(theme, scheme, gi, group_count, focus);
            let items: Vec<(menu::MenuDemoItem, menu::MenuItemAppearance)> = group
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let selected = gi == 0 && i == menu::STYLE_SELECTED;
                    let state = if item.submenu && submenu_open {
                        InteractionState::Hovered
                    } else {
                        InteractionState::Enabled
                    };
                    (
                        *item,
                        menu::resolve_item_at(
                            theme,
                            scheme,
                            menu::MenuAxis::Vertical,
                            i,
                            group.len(),
                            selected,
                            state,
                        ),
                    )
                })
                .collect();
            div()
                .p(px(shell.pad_dp))
                .rounded_tl(px(shell.corners.top_left))
                .rounded_tr(px(shell.corners.top_right))
                .rounded_br(px(shell.corners.bottom_right))
                .rounded_bl(px(shell.corners.bottom_left))
                .bg(paint(shell.container))
                .flex()
                .flex_col()
                .children(items.into_iter().map(|(item, a)| paint_menu_item(item, a)))
        }))
}

fn desktop_live_menu_item(
    id: SharedString,
    item: menu::MenuDemoItem,
    a: menu::MenuItemAppearance,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    on_move: impl Fn(&MouseMoveEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let trail = menu::trailing_text(&item);
    div()
        .id(id)
        .h(px(a.height_dp))
        .px(px(a.pad_h_dp))
        .rounded_tl(px(a.corners.top_left))
        .rounded_tr(px(a.corners.top_right))
        .rounded_br(px(a.corners.bottom_right))
        .rounded_bl(px(a.corners.bottom_left))
        .bg(paint(a.container))
        .text_color(paint(a.label))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(menu::ITEM_BETWEEN_SPACE_DP))
        .child(
            div()
                .w(px(menu::ICON_DP))
                .text_color(paint(a.icon))
                .child(item.icon),
        )
        .child(div().flex_1().child(item.label))
        .when(!trail.is_empty(), |el| {
            el.child(div().text_color(paint(a.shortcut)).child(trail))
        })
        .on_click(on_click)
        .on_mouse_move(on_move)
}

fn apply_live_menu_action(
    this: &mut CatalogView,
    host: LiveMenuHost,
    action: menu::OverlayMenuAction,
    cx: &mut Context<CatalogView>,
) {
    if matches!(
        action,
        menu::OverlayMenuAction::Commit | menu::OverlayMenuAction::Dismiss
    ) {
        match host {
            LiveMenuHost::Overlay => this.overlay = Overlay::None,
            LiveMenuHost::Cascade => {}
            LiveMenuHost::StandardOverflow => {
                this.standard_overflow_open = false;
                this.standard_overflow_menu = menu::OverlayMenuSession::standard_overflow();
            }
            LiveMenuHost::ConnectedOverflow => {
                this.overflow_open = false;
                this.connected_overflow_menu = menu::OverlayMenuSession::connected_overflow();
            }
            LiveMenuHost::Split => {
                this.split_open = false;
                this.split_menu = menu::OverlayMenuSession::split();
            }
        }
        if this.typeahead_host == Some(host) {
            this.typeahead_host = None;
        }
    }
    cx.notify();
}

fn desktop_live_parent(
    theme: &Theme,
    session: menu::OverlayMenuSession,
    cx: &mut Context<CatalogView>,
    host: LiveMenuHost,
) -> impl IntoElement {
    let groups = session.kind.groups();
    let group_count = groups.len();
    let scheme = menu::MenuScheme::Standard;
    let focus = session.parent_focus();
    let prefix = host.prefix();
    div()
        .flex()
        .flex_col()
        .gap(px(menu::GROUP_GAP_DP))
        .w(px(220.))
        .children(groups.iter().enumerate().map(move |(gi, group)| {
            let shell = menu::resolve_group_focus(theme, scheme, gi, group_count, focus);
            let start: usize = groups.iter().take(gi).map(|g| g.len()).sum();
            let rows: Vec<(usize, menu::MenuDemoItem, menu::MenuItemAppearance)> = group
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let index = start + i;
                    (
                        index,
                        *item,
                        menu::resolve_item_at(
                            theme,
                            scheme,
                            menu::MenuAxis::Vertical,
                            i,
                            group.len(),
                            session.parent_selected(index),
                            session.parent_state(index, item),
                        ),
                    )
                })
                .collect();
            div()
                .p(px(shell.pad_dp))
                .rounded_tl(px(shell.corners.top_left))
                .rounded_tr(px(shell.corners.top_right))
                .rounded_br(px(shell.corners.bottom_right))
                .rounded_bl(px(shell.corners.bottom_left))
                .bg(paint(shell.container))
                .flex()
                .flex_col()
                .children(rows.into_iter().map(|(index, item, a)| {
                    desktop_live_menu_item(
                        SharedString::from(format!("{prefix}-menu-{index}")),
                        item,
                        a,
                        cx.listener(move |this, _, _, cx| {
                            let action = this.live_menu_mut(host).click_parent(index);
                            apply_live_menu_action(this, host, action, cx);
                        }),
                        cx.listener(move |this, _, _, cx| {
                            this.hover_live_parent(host, index, cx);
                        }),
                    )
                }))
        }))
}

fn desktop_live_flyout(
    theme: &Theme,
    session: menu::OverlayMenuSession,
    cx: &mut Context<CatalogView>,
    host: LiveMenuHost,
) -> impl IntoElement {
    let scheme = menu::MenuScheme::Standard;
    let shell = menu::resolve_submenu(theme, scheme);
    let count = menu::SUBMENU_ITEMS.len();
    let prefix = host.prefix();
    let rows: Vec<(usize, menu::MenuDemoItem, menu::MenuItemAppearance)> = menu::SUBMENU_ITEMS
        .iter()
        .enumerate()
        .map(|(i, item)| {
            (
                i,
                *item,
                menu::resolve_item_at(
                    theme,
                    scheme,
                    menu::MenuAxis::Vertical,
                    i,
                    count,
                    session.submenu_selected(i),
                    if session.submenu_hi == i {
                        InteractionState::Hovered
                    } else {
                        InteractionState::Enabled
                    },
                ),
            )
        })
        .collect();
    div()
        .p(px(shell.pad_dp))
        .rounded(px(shell.corners.top_left))
        .bg(paint(shell.container))
        .flex()
        .flex_col()
        .children(rows.into_iter().map(|(i, item, a)| {
            desktop_live_menu_item(
                SharedString::from(format!("{prefix}-sub-{i}")),
                item,
                a,
                cx.listener(move |this, _, _, cx| {
                    let action = this.live_menu_mut(host).click_submenu(i);
                    apply_live_menu_action(this, host, action, cx);
                }),
                cx.listener(move |this, _, _, cx| {
                    this.live_menu_mut(host).submenu_hi = i;
                    cx.notify();
                }),
            )
        }))
}

fn desktop_live_cascade(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
    host: LiveMenuHost,
) -> impl IntoElement {
    let session = this.live_menu(host);
    div()
        .id(host.id())
        .tab_index(0)
        .when(
            menu::TYPEAHEAD_AUTOFOCUS && this.typeahead_host == Some(host),
            |el| el.track_focus(&this.typeahead_focus),
        )
        .flex()
        .flex_row()
        .items_end()
        .gap(px(menu::SUBMENU_GAP_DP))
        .on_key_down(cx.listener(move |this, ev: &KeyDownEvent, _, cx| {
            let action = this.live_menu_mut(host).apply_key(&ev.keystroke.key);
            apply_live_menu_action(this, host, action, cx);
        }))
        .when(host != LiveMenuHost::Overlay, |el| {
            el.on_hover(cx.listener(move |this, hovered: &bool, window, cx| {
                if *hovered {
                    if host == LiveMenuHost::Cascade {
                        this.focus_typeahead(LiveMenuHost::Cascade, window, cx);
                    }
                } else {
                    this.live_menu_mut(host).hover_leave();
                    cx.notify();
                }
            }))
        })
        .child(desktop_live_parent(theme, session, cx, host))
        .when(session.submenu_open, |el| {
            el.child(desktop_live_flyout(theme, session, cx, host))
        })
}

fn menu_overlay(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let anchor = button::resolve(
        theme,
        button::ButtonVariant::Tonal,
        InteractionState::Enabled,
    );
    div()
        .id("menu-popup-host")
        .absolute()
        .top(px(0.))
        .left(px(0.))
        .size_full()
        .on_click(cx.listener(|this, _, _, cx| {
            this.overlay = Overlay::None;
            cx.notify();
        }))
        .child(
            div()
                .id("menu-anchored")
                .absolute()
                .top(px(8.))
                .left(px(20.))
                .flex()
                .flex_col()
                .gap(px(menu::OVERLAY_ANCHOR_GAP_DP))
                .child(
                    div()
                        .id("menu-anchor")
                        .h(px(anchor.height_dp))
                        .px(px(16.))
                        .rounded(px(anchor.corners.top_left))
                        .bg(paint(anchor.container))
                        .text_color(paint(anchor.content))
                        .flex()
                        .items_center()
                        .child(menu::OVERLAY_ANCHOR_LABEL),
                )
                .child(desktop_live_cascade(this, theme, cx, LiveMenuHost::Overlay)),
        )
}

fn desktop_horizontal_menu(theme: &Theme) -> impl IntoElement {
    let shell = menu::resolve_container(theme, menu::MenuScheme::Standard);
    let count = menu::HORIZONTAL_LABELS.len();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(menu::HORIZONTAL_GAP_DP))
        .p(px(shell.pad_dp))
        .rounded(px(shell.corners.top_left))
        .bg(paint(shell.container))
        .children(
            menu::HORIZONTAL_LABELS
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let selected = i == menu::HORIZONTAL_SELECTED;
                    let a = menu::resolve_horizontal(
                        theme,
                        menu::MenuScheme::Standard,
                        i,
                        count,
                        selected,
                        InteractionState::Enabled,
                    );
                    div()
                        .h(px(a.height_dp))
                        .px(px(a.pad_h_dp))
                        .rounded(px(a.corners.top_left))
                        .bg(paint(a.container))
                        .text_color(paint(a.label))
                        .flex()
                        .items_center()
                        .child(*label)
                }),
        )
}

fn desktop_horizontal_icons(theme: &Theme) -> impl IntoElement {
    let shell = menu::resolve_container(theme, menu::MenuScheme::Standard);
    let count = menu::HORIZONTAL_ICONS.len();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(menu::HORIZONTAL_ICON_GAP_DP))
        .p(px(shell.pad_dp))
        .rounded(px(shell.corners.top_left))
        .bg(paint(shell.container))
        .children(menu::HORIZONTAL_ICONS.iter().enumerate().map(|(i, glyph)| {
            let selected = i == menu::HORIZONTAL_ICON_SELECTED;
            let a = menu::resolve_horizontal_icon(
                theme,
                menu::MenuScheme::Standard,
                i,
                count,
                selected,
            );
            div()
                .w(px(a.height_dp))
                .h(px(a.height_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(a.container))
                .text_color(paint(a.label))
                .flex()
                .items_center()
                .justify_center()
                .child(*glyph)
        }))
}

fn desktop_menus(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(12.))
                .child(desktop_vertical_menu(theme, menu::MenuScheme::Standard))
                .child(desktop_vertical_menu(theme, menu::MenuScheme::Vibrant)),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(12.))
                .items_center()
                .child(desktop_horizontal_menu(theme))
                .child(desktop_horizontal_icons(theme)),
        )
        .child(desktop_live_cascade(this, theme, cx, LiveMenuHost::Cascade))
}
fn standard_button_group(
    this: &CatalogView,
    theme: &Theme,
    selected: usize,
    overflow_open: bool,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let ov = button_group::resolve_standard_overflow(theme, false);
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(8.))
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(button_group::STANDARD_GAP_DP))
                .children(
                    button_group::STANDARD_SEGMENTS
                        .iter()
                        .enumerate()
                        .map(|(i, label)| {
                            let a = button_group::resolve_standard_scene(theme, i, selected);
                            let w = a.width_dp.unwrap_or(button_group::STANDARD_BASE_W_DP);
                            div()
                                .id(SharedString::from(format!("std-group-{i}")))
                                .h(px(a.height_dp))
                                .w(px(w))
                                .px(px(a.pad_start_dp))
                                .rounded(px(a.corners.top_left))
                                .bg(paint(a.container))
                                .text_color(paint(a.content))
                                .text_size(type_size(a.label_style))
                                .font_weight(type_weight(a.label_style))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(*label)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.standard_selected = i;
                                    cx.notify();
                                }))
                        }),
                )
                .child(
                    div()
                        .id("std-group-overflow")
                        .h(px(ov.height_dp))
                        .w(px(ov.width_dp.unwrap_or(40.)))
                        .rounded(px(ov.corners.top_left))
                        .bg(paint(ov.container))
                        .text_color(paint(ov.content))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(button_group::STANDARD_OVERFLOW_GLYPH)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.standard_overflow_open = !this.standard_overflow_open;
                            if this.standard_overflow_open {
                                this.standard_overflow_menu =
                                    menu::OverlayMenuSession::standard_overflow();
                                this.focus_typeahead(LiveMenuHost::StandardOverflow, window, cx);
                            } else if this.typeahead_host == Some(LiveMenuHost::StandardOverflow) {
                                this.typeahead_host = None;
                            }
                            cx.notify();
                        })),
                ),
        )
        .when(overflow_open, |el| {
            el.child(desktop_live_cascade(
                this,
                theme,
                cx,
                LiveMenuHost::StandardOverflow,
            ))
        })
}

fn connected_button_group(
    theme: &Theme,
    selected: usize,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let count = button_group::DEMO_SEGMENTS.len();
    div()
        .flex()
        .flex_row()
        .gap(px(button_group::CONNECTED_GAP_DP))
        .children(
            button_group::DEMO_SEGMENTS
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let a = button_group::resolve_segment(theme, i, count, i == selected, false);
                    div()
                        .id(SharedString::from(format!("group-{i}")))
                        .h(px(a.height_dp))
                        .px(px(a.pad_start_dp))
                        .rounded_tl(px(a.corners.top_left))
                        .rounded_tr(px(a.corners.top_right))
                        .rounded_br(px(a.corners.bottom_right))
                        .rounded_bl(px(a.corners.bottom_left))
                        .bg(paint(a.container))
                        .text_color(paint(a.content))
                        .text_size(type_size(a.label_style))
                        .font_weight(type_weight(a.label_style))
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(a.outline.is_some(), |el| {
                            let (color, _) = a.outline.unwrap();
                            el.border_1().border_color(paint(color))
                        })
                        .child(*label)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.group_selected = i;
                            cx.notify();
                        }))
                }),
        )
}

fn connected_icon_group(
    this: &CatalogView,
    theme: &Theme,
    selected: usize,
    overflow_open: bool,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let count = button_group::icon_group_count();
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(8.))
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(button_group::CONNECTED_GAP_DP))
                .children((0..count).map(|i| {
                    let overflow = i == button_group::overflow_index();
                    let sel = !overflow && i == selected;
                    let a = button_group::resolve_icon_segment(theme, i, count, sel, false);
                    let glyph = button_group::icon_glyph(i);
                    div()
                        .id(SharedString::from(format!("icon-group-{i}")))
                        .h(px(a.height_dp))
                        .w(px(a.min_width_dp.unwrap_or(button_group::ICON_MIN_W_DP)))
                        .rounded_tl(px(a.corners.top_left))
                        .rounded_tr(px(a.corners.top_right))
                        .rounded_br(px(a.corners.bottom_right))
                        .rounded_bl(px(a.corners.bottom_left))
                        .bg(paint(a.container))
                        .text_color(paint(a.content))
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(a.outline.is_some(), |el| {
                            let (color, _) = a.outline.unwrap();
                            el.border_1().border_color(paint(color))
                        })
                        .child(glyph)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            if overflow {
                                this.overflow_open = !this.overflow_open;
                                if this.overflow_open {
                                    this.connected_overflow_menu =
                                        menu::OverlayMenuSession::connected_overflow();
                                    this.focus_typeahead(
                                        LiveMenuHost::ConnectedOverflow,
                                        window,
                                        cx,
                                    );
                                } else if this.typeahead_host
                                    == Some(LiveMenuHost::ConnectedOverflow)
                                {
                                    this.typeahead_host = None;
                                }
                            } else {
                                this.icon_selected = i;
                            }
                            cx.notify();
                        }))
                })),
        )
        .when(overflow_open, |el| {
            el.child(desktop_live_cascade(
                this,
                theme,
                cx,
                LiveMenuHost::ConnectedOverflow,
            ))
        })
}

fn desktop_split_button(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let lead = split_button::resolve_leading(
        theme,
        split_button::SplitButtonVariant::Filled,
        button::ButtonSize::Small,
        false,
    );
    let trail = split_button::resolve_trailing(
        theme,
        split_button::SplitButtonVariant::Filled,
        button::ButtonSize::Small,
        this.split_open,
    );
    let split = div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(8.))
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(split_button::GAP_DP))
                .child(
                    div()
                        .id("split-lead")
                        .h(px(lead.height_dp))
                        .px(px(lead.pad_start_dp))
                        .rounded_tl(px(lead.corners.top_left))
                        .rounded_tr(px(lead.corners.top_right))
                        .rounded_br(px(lead.corners.bottom_right))
                        .rounded_bl(px(lead.corners.bottom_left))
                        .bg(paint(lead.container))
                        .text_color(paint(lead.content))
                        .flex()
                        .items_center()
                        .child(format!(
                            "{} {}",
                            split_button::DEMO_LEADING_ICON,
                            split_button::DEMO_LABEL
                        )),
                )
                .child(
                    div()
                        .id("split-trail")
                        .h(px(trail.height_dp))
                        .w(px(trail
                            .min_width_dp
                            .unwrap_or(split_button::TRAILING_MIN_W_DP)))
                        .rounded_tl(px(trail.corners.top_left))
                        .rounded_tr(px(trail.corners.top_right))
                        .rounded_br(px(trail.corners.bottom_right))
                        .rounded_bl(px(trail.corners.bottom_left))
                        .bg(paint(trail.container))
                        .text_color(paint(trail.content))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(split_button::caret(this.split_open))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.split_open = !this.split_open;
                            if this.split_open {
                                this.split_menu = menu::OverlayMenuSession::split();
                                this.focus_typeahead(LiveMenuHost::Split, window, cx);
                            } else if this.typeahead_host == Some(LiveMenuHost::Split) {
                                this.typeahead_host = None;
                            }
                            cx.notify();
                        })),
                ),
        )
        .when(this.split_open, |el| {
            el.child(desktop_live_cascade(this, theme, cx, LiveMenuHost::Split))
        });
    div()
        .id("split-scene")
        .w(px(split_button::PHONE_W_DP))
        .rounded(px(split_button::PHONE_CORNER_DP))
        .overflow_hidden()
        .bg(paint(theme.color.surface))
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(200.))
                .w_full()
                .overflow_hidden()
                .child(photo_fill(split_button::SCENE_PHOTO)),
        )
        .child(
            div()
                .px(px(20.))
                .pb(px(16.))
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(spaced_line(
                    split_button::SCENE_TITLE,
                    22.0,
                    paint(theme.color.on_surface),
                ))
                .child(spaced_line(
                    split_button::SCENE_SUBTITLE,
                    14.0,
                    paint(theme.color.on_surface_variant),
                ))
                .child(split),
        )
}

fn desktop_fab_menu(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let color = fab_menu::FabMenuColor::Primary;
    let item = fab_menu::resolve_item(theme, color);
    let close = fab_menu::resolve_close(theme, color, this.fab_menu_open);
    let menu = div()
        .flex()
        .flex_col()
        .items_end()
        .gap(px(fab_menu::ITEM_GAP_DP))
        .when(this.fab_menu_open, |el| {
            el.children(fab_menu::DEMO_ITEMS.iter().map(|(icon, label)| {
                div()
                    .h(px(item.height_dp))
                    .px(px(item.pad_h_dp))
                    .rounded(px(item.corners.top_left))
                    .bg(paint(item.container))
                    .text_color(paint(item.content))
                    .flex()
                    .items_center()
                    .gap(px(item.icon_gap_dp))
                    .child(*icon)
                    .child(*label)
            }))
        })
        .child(
            div()
                .id("fab-menu-close")
                .w(px(close.size_dp))
                .h(px(close.size_dp))
                .rounded(px(close.corners.top_left))
                .bg(paint(close.container))
                .text_color(paint(close.content))
                .flex()
                .items_center()
                .justify_center()
                .child(close.glyph)
                .on_click(cx.listener(|this, _, _, cx| {
                    this.fab_menu_open = !this.fab_menu_open;
                    cx.notify();
                })),
        );
    div()
        .id("fab-scene")
        .w(px(fab_menu::PHONE_W_DP))
        .h(px(fab_menu::PHONE_H_DP))
        .rounded(px(fab_menu::PHONE_CORNER_DP))
        .overflow_hidden()
        .relative()
        .child(
            div()
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .child(photo_fill(fab_menu::SCENE_PHOTO)),
        )
        .child(
            div()
                .absolute()
                .bottom(px(fab_menu::SCENE_INSET_DP))
                .right(px(fab_menu::SCENE_INSET_DP))
                .child(menu),
        )
}

fn desktop_toolbar(theme: &Theme) -> impl IntoElement {
    let a = toolbar::resolve(
        theme,
        toolbar::ToolbarKind::Floating,
        toolbar::ToolbarColor::Vibrant,
        toolbar::ToolbarAxis::Horizontal,
    );
    let icon = toolbar::resolve_icon(theme, toolbar::ToolbarColor::Vibrant);
    let fab = toolbar::resolve_fab(theme, toolbar::ToolbarColor::Vibrant);
    let bar = div()
        .flex()
        .items_center()
        .gap(px(toolbar::FAB_GAP_DP))
        .child(
            div()
                .h(px(a.height_dp))
                .px(px(a.pad_h_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(a.container))
                .flex()
                .items_center()
                .gap(px(a.item_gap_dp))
                .children(toolbar::DEMO_ICONS.iter().map(|glyph| {
                    div()
                        .w(px(icon.height_dp))
                        .h(px(icon.height_dp))
                        .rounded(px(icon.corners.top_left))
                        .bg(paint(icon.container))
                        .text_color(paint(icon.content))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(*glyph)
                })),
        )
        .child(
            div()
                .w(px(fab.width_dp.unwrap_or(fab.height_dp)))
                .h(px(fab.height_dp))
                .rounded(px(fab.corners.top_left))
                .bg(paint(fab.container))
                .text_color(paint(fab.content))
                .flex()
                .items_center()
                .justify_center()
                .child(toolbar::DEMO_FAB),
        );
    div()
        .id("toolbar-scene")
        .w(px(toolbar::PHONE_W_DP))
        .rounded(px(toolbar::PHONE_CORNER_DP))
        .overflow_hidden()
        .bg(paint(theme.color.surface))
        .flex()
        .flex_col()
        .child(
            div()
                .px(px(16.))
                .py(px(12.))
                .flex()
                .items_center()
                .gap(px(12.))
                .child(photo_avatar(toolbar::SCENE_AVATAR, 40.0))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .child(spaced_line(
                            toolbar::SCENE_FROM,
                            16.0,
                            paint(theme.color.on_surface),
                        ))
                        .child(spaced_line(
                            toolbar::SCENE_TIME,
                            12.0,
                            paint(theme.color.on_surface_variant),
                        )),
                )
                .child(toolbar::SCENE_STAR),
        )
        .children(toolbar::SCENE_BUBBLES.iter().map(|b| {
            div()
                .mx(px(16.))
                .my(px(6.))
                .px(px(12.))
                .py(px(10.))
                .rounded(px(16.))
                .bg(paint(theme.color.surface_container_low))
                .child(spaced_line(*b, 14.0, paint(theme.color.on_surface)))
        }))
        .child(
            div()
                .h(px(140.))
                .mx(px(16.))
                .rounded(px(16.))
                .overflow_hidden()
                .child(photo_fill(toolbar::SCENE_PHOTO)),
        )
        .child(div().p(px(16.)).child(bar))
}

fn range_slider_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let range = slider::resolve_range(
        theme,
        this.range_start,
        this.range_end,
        InteractionState::Enabled,
    );
    let t = range.track;
    let total = slider::RANGE_TRACK_W_DP;
    let paint_r = slider::range_paint(range.start, range.end, total, t.handle_w.max(4.0));
    let y_track = ((t.target_dp - t.track_h) / 2.0).max(0.0);
    let y_handle = ((t.target_dp - t.handle_h_visual) / 2.0).max(0.0);
    let hit = this.range_hit.clone();
    let hit_move = this.range_hit.clone();
    let hit_click = this.range_hit.clone();
    div()
        .id("range-slider")
        .tab_index(0)
        .w_full()
        .flex()
        .flex_col()
        .gap(px(4.))
        .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
            if let Some((s, e, thumb)) = slider::apply_arrow(
                this.range_start,
                this.range_end,
                this.range_focus,
                &ev.keystroke.key,
            ) {
                this.range_start = s;
                this.range_end = e;
                this.range_focus = thumb;
                cx.notify();
            }
        }))
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _, _, cx| {
                this.range_drag = None;
                cx.notify();
            }),
        )
        .child(spaced_line(
            slider::range_value_label(range.start, range.end),
            12.0,
            paint(theme.color.on_surface),
        ))
        .child(
            div()
                .id("range-track")
                .relative()
                .w(px(total))
                .h(px(t.target_dp))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                        let (origin, w) = hit.get();
                        let frac =
                            slider::fraction_from_local_x(f32::from(ev.position.x) - origin, w);
                        this.range_focus =
                            slider::nearest_thumb(this.range_start, this.range_end, frac);
                        this.range_drag = Some(this.range_focus);
                        this.range_moved = false;
                        cx.notify();
                    }),
                )
                .on_mouse_move(cx.listener(move |this, ev: &MouseMoveEvent, _, cx| {
                    if ev.dragging() {
                        if let Some(thumb) = this.range_drag {
                            let (origin, w) = hit_move.get();
                            let frac =
                                slider::fraction_from_local_x(f32::from(ev.position.x) - origin, w);
                            let (s, e) = slider::drag_thumb_snapped(
                                this.range_start,
                                this.range_end,
                                thumb,
                                frac,
                            );
                            this.range_start = s;
                            this.range_end = e;
                            this.range_moved = true;
                            cx.notify();
                        }
                    }
                }))
                .on_click(cx.listener(move |this, ev: &gpui::ClickEvent, _, cx| {
                    if this.range_moved {
                        this.range_drag = None;
                        this.range_moved = false;
                        cx.notify();
                        return;
                    }
                    let (origin, w) = hit_click.get();
                    let frac =
                        slider::fraction_from_local_x(f32::from(ev.position().x) - origin, w);
                    let (s, e) = slider::click_step(this.range_start, this.range_end, frac);
                    this.range_start = s;
                    this.range_end = e;
                    this.range_drag = None;
                    cx.notify();
                }))
                .child(
                    div()
                        .absolute()
                        .left(px(0.))
                        .top(px(y_track))
                        .h(px(t.track_h))
                        .w(px(paint_r.left))
                        .rounded(px(t.track_corner))
                        .bg(paint(t.inactive)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.start_handle))
                        .top(px(y_handle))
                        .w(px(paint_r.handle_w.max(12.0)))
                        .h(px(t.handle_h_visual))
                        .rounded(px(2.))
                        .bg(paint(t.handle)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.start_handle + paint_r.handle_w.max(12.0)))
                        .top(px(y_track))
                        .h(px(t.track_h))
                        .w(px(paint_r.active))
                        .rounded(px(t.inner_corner))
                        .bg(paint(t.active)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.end_handle))
                        .top(px(y_handle))
                        .w(px(paint_r.handle_w.max(12.0)))
                        .h(px(t.handle_h_visual))
                        .rounded(px(2.))
                        .bg(paint(t.handle)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(paint_r.end_handle + paint_r.handle_w.max(12.0)))
                        .top(px(y_track))
                        .h(px(t.track_h))
                        .w(px(paint_r.right))
                        .rounded(px(t.track_corner))
                        .bg(paint(t.inactive)),
                )
                .children(slider::range_tick_fractions().into_iter().map(|frac| {
                    let x = (frac * total - t.stop_dp / 2.0).max(0.0);
                    let active = slider::range_tick_active(frac, range.start, range.end);
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(y_track + (t.track_h - t.stop_dp) / 2.0))
                        .w(px(t.stop_dp))
                        .h(px(t.stop_dp))
                        .rounded(px(t.stop_dp / 2.0))
                        .bg(paint(if active {
                            t.stop_active
                        } else {
                            t.stop_inactive
                        }))
                }))
                .child({
                    let hit = this.range_hit.clone();
                    canvas(
                        move |bounds, _, _| {
                            hit.set((
                                f32::from(bounds.origin.x),
                                f32::from(bounds.size.width).max(1.0),
                            ));
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top(px(0.))
                    .left(px(0.))
                    .w(px(total))
                    .h(px(t.target_dp))
                }),
        )
}

fn settings_scene(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let title = theme.typography.title_large.emphasized();
    let section = theme.typography.title_medium.emphasized();
    let outlined = this.outlined.appearance(theme);
    div()
        .w_full()
        .p(px(button_group::SETTINGS_PAD_DP))
        .rounded(px(button_group::SETTINGS_CORNER_DP))
        .bg(paint(theme.color.surface_container))
        .flex()
        .flex_col()
        .gap(px(button_group::SETTINGS_GROUP_GAP_DP))
        .child(div().font_weight(type_weight(title)).child(spaced_line(
            button_group::SETTINGS_SCENE_TITLE,
            title.size_sp,
            paint(theme.color.on_surface),
        )))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(button_group::SETTINGS_ROW_GAP_DP))
                .child(div().font_weight(type_weight(section)).child(spaced_line(
                    button_group::SETTINGS_VOLUME_TITLE,
                    section.size_sp,
                    paint(theme.color.on_surface),
                )))
                .child(volume_slider_scene(theme, this.slider, cx)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(button_group::SETTINGS_ROW_GAP_DP))
                .child(div().font_weight(type_weight(section)).child(spaced_line(
                    button_group::SETTINGS_QUIET_HOURS_TITLE,
                    section.size_sp,
                    paint(theme.color.on_surface),
                )))
                .child(field_block(
                    "settings-email",
                    &outlined,
                    "Email",
                    this.outlined.display_with_caret(),
                    if this.outlined.error {
                        "Enter a valid email"
                    } else {
                        "Supporting text"
                    },
                    cx.listener(|this, _, _, cx| {
                        this.filled.set_focus(false);
                        this.outlined.set_focus(true);
                        cx.notify();
                    }),
                ))
                .child(connected_button_group(theme, this.group_selected, cx))
                .child(connected_icon_group(
                    this,
                    theme,
                    this.icon_selected,
                    false,
                    cx,
                )),
        )
        .child(m_button(
            "settings-reset",
            theme,
            button::ButtonVariant::Text,
            InteractionState::Enabled,
            "Reset settings",
            cx.listener(|this, _, _, cx| {
                this.overlay = Overlay::Dialog;
                cx.notify();
            }),
        ))
}

fn docked_date_picker(
    this: &CatalogView,
    theme: &Theme,
    pick: &date_picker::DatePickerAppearance,
    cells: &[(u32, DayKind); 42],
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        if this.docked_open {
            InteractionState::Focused
        } else {
            InteractionState::Enabled
        },
        true,
    );
    let cal_w = pick.day_dp * 7.0;
    let value = date_picker::docked_field_value(this.selected);
    let popup = this.docked_open.then(|| {
        div()
            .w_full()
            .p(px(12.))
            .rounded_tl(px(8.))
            .rounded_tr(px(pick.corners.top_right))
            .rounded_br(px(pick.corners.bottom_right))
            .rounded_bl(px(pick.corners.bottom_left))
            .bg(paint(pick.container))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(4.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(div().id("docked-month-prev").p(px(8.)).child("<").on_click(
                        cx.listener(|this, _, _, cx| {
                            let (y, m) =
                                date_picker::add_months(this.picker_year, this.picker_month, -1);
                            this.picker_year = y;
                            this.picker_month = m;
                            cx.notify();
                        }),
                    ))
                    .child(spaced_line(
                        date_picker::month_nav_label(this.picker_year, this.picker_month),
                        pick.year_style.size_sp,
                        paint(pick.header_year),
                    ))
                    .child(div().id("docked-month-next").p(px(8.)).child(">").on_click(
                        cx.listener(|this, _, _, cx| {
                            let (y, m) =
                                date_picker::add_months(this.picker_year, this.picker_month, 1);
                            this.picker_year = y;
                            this.picker_month = m;
                            cx.notify();
                        }),
                    )),
            )
            .child(weekday_row(pick, cal_w))
            .child(div().w(px(cal_w)).flex().flex_wrap().children(
                cells.iter().copied().enumerate().map(|(i, (day, kind))| {
                    let (bg, fg, radius) = day_colors(pick, kind);
                    let in_month = kind != DayKind::OutOfMonth;
                    let year = this.picker_year;
                    let month = this.picker_month;
                    div()
                        .id(SharedString::from(format!("docked-day-{i}")))
                        .w(px(pick.day_dp))
                        .h(px(pick.day_dp))
                        .rounded(px(radius))
                        .bg(bg)
                        .text_color(fg)
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(kind == DayKind::Today, |el| {
                            el.border_1().border_color(paint(pick.day_today_outline))
                        })
                        .child(day.to_string())
                        .when(in_month, |el| {
                            el.on_click(cx.listener(move |this, _, _, cx| {
                                this.selected = CivilDate { year, month, day };
                                if date_picker::DOCKED_DISMISS_ON_SELECT {
                                    this.docked_open = false;
                                }
                                cx.notify();
                            }))
                        })
                }),
            ))
            .into_any_element()
    });
    div()
        .id("docked-date")
        .flex()
        .flex_col()
        .gap(px(4.))
        .w(px(cal_w + 32.0))
        .on_mouse_down_out(cx.listener(|this, _, _, cx| {
            if date_picker::DOCKED_DISMISS_ON_OUTSIDE && this.docked_open {
                this.docked_open = false;
                cx.notify();
            }
        }))
        .child(field_block(
            "docked-date-field",
            &field,
            date_picker::DOCKED_FIELD_LABEL,
            value,
            "",
            cx.listener(|this, _, _, cx| {
                this.docked_open = !this.docked_open;
                cx.notify();
            }),
        ))
        .children(popup)
}

fn slider_stop(slide: &slider::SliderAppearance, active: bool) -> impl IntoElement {
    div()
        .w(px(slide.stop_dp))
        .h(px(slide.stop_dp))
        .rounded(px(slide.stop_dp / 2.0))
        .bg(paint(if active {
            slide.stop_active
        } else {
            slide.stop_inactive
        }))
}

fn expressive_slider(slide: &slider::SliderAppearance) -> impl IntoElement {
    let fractions = slider::stop_fractions(slide.stop_count);
    let rail = div()
        .w_full()
        .h(px(slide.target_dp))
        .flex()
        .items_center()
        .child(
            div()
                .h(px(slide.track_h))
                .w(px(180. * slide.value.max(0.12)))
                .rounded(px(slide.track_corner))
                .bg(paint(slide.active)),
        )
        .child(
            div()
                .mx(px(slide.gap_dp))
                .w(px(slide.handle_w))
                .h(px(slide.handle_h_visual))
                .rounded(px(2.))
                .bg(paint(slide.handle)),
        )
        .child(
            div()
                .h(px(slide.track_h))
                .flex_1()
                .rounded(px(slide.track_corner))
                .bg(paint(slide.inactive)),
        );
    if slide.stop_count <= 2 {
        return rail.into_any_element();
    }
    // Unified rail overlay: even mid-stops across the full track (official Alarm).
    div()
        .w_full()
        .flex()
        .flex_col()
        .child(rail)
        .child(
            div()
                .w_full()
                .h(px(slide.target_dp))
                .mt(px(-slide.target_dp))
                .px(px(8.))
                .flex()
                .items_center()
                .justify_between()
                .children(
                    fractions
                        .into_iter()
                        .map(|frac| slider_stop(slide, frac <= slide.value + 0.001)),
                ),
        )
        .into_any_element()
}

fn volume_slider_scene(
    theme: &Theme,
    media_value: f32,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    div().w_full().flex().flex_col().gap(px(12.)).children(
        slider::OVERVIEW_ROWS.iter().copied().map(|row| {
            let value = if row.label.starts_with("Media") {
                media_value
            } else {
                row.value
            };
            let slide =
                slider::resolve_with_stops(theme, value, InteractionState::Enabled, row.stop_count);
            let interactive = row.label.starts_with("Media");
            div()
                .id(SharedString::from(row.label))
                .w_full()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(
                    div()
                        .w(px(24.))
                        .h(px(24.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(paint(theme.color.on_surface_variant))
                        .child(row.icon),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .gap(px(4.))
                        .child(spaced_line(row.label, 12.0, paint(theme.color.on_surface)))
                        .child(expressive_slider(&slide)),
                )
                .when(interactive, |el| {
                    el.on_click(cx.listener(|this, _, _, cx| {
                        this.slider = ((this.slider + 0.1) * 10.0).round() / 10.0;
                        if this.slider > 1.0 {
                            this.slider = 0.0;
                        }
                        cx.notify();
                    }))
                })
        }),
    )
}

fn dialog_account_row(theme: &Theme, email: &'static str) -> impl IntoElement {
    let c = theme.color;
    div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.))
        .py(px(4.))
        .child(
            div()
                .w(px(40.))
                .h(px(40.))
                .rounded(px(20.))
                .bg(paint(c.secondary_container))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_size(px(14.))
                        .text_color(paint(c.on_secondary_container))
                        .child(dialog::account_initials(email)),
                ),
        )
        .child(
            div()
                .text_size(px(14.))
                .text_color(paint(c.on_surface))
                .child(email.to_string()),
        )
}

fn dismiss_overlay(
    this: &mut CatalogView,
    _: &gpui::ClickEvent,
    _: &mut Window,
    cx: &mut Context<CatalogView>,
) {
    this.overlay = Overlay::None;
    cx.notify();
}

fn dialog_overlay(theme: &Theme, cx: &mut Context<CatalogView>) -> impl IntoElement {
    let a = dialog::resolve(theme);
    div()
        .id("dialog-scrim")
        .flex_1()
        .w_full()
        .bg(paint(a.scrim))
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.))
        .on_click(cx.listener(dismiss_overlay))
        .child(
            div()
                .id("dialog-card")
                .w(px(a.min_width_dp + 40.0))
                .min_w(px(a.min_width_dp))
                .p(px(a.pad_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(a.container))
                .gap(px(16.))
                .flex()
                .flex_col()
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_center()
                        .text_size(px(dialog::ICON_DP))
                        .text_color(paint(a.icon))
                        .child(dialog::RESET_ICON),
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_center()
                        .font_weight(type_weight(a.headline_style))
                        .child(spaced_line(
                            dialog::RESET_HEADLINE,
                            a.headline_style.size_sp,
                            paint(a.headline),
                        )),
                )
                .child(div().w_full().child(spaced_line(
                    dialog::RESET_SUPPORTING,
                    a.supporting_style.size_sp,
                    paint(a.supporting),
                )))
                .children(
                    dialog::RESET_ACCOUNTS
                        .iter()
                        .map(|email| dialog_account_row(theme, email)),
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_end()
                        .gap(px(dialog::ACTION_GAP_DP))
                        .child(m_button(
                            "dialog-cancel",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RESET_CANCEL,
                            cx.listener(dismiss_overlay),
                        ))
                        .child(m_button(
                            "dialog-ok",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RESET_ACCEPT,
                            cx.listener(dismiss_overlay),
                        )),
                ),
        )
}

fn list_dialog_overlay(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = dialog::resolve(theme);
    div()
        .id("dialog-list-scrim")
        .flex_1()
        .w_full()
        .bg(paint(a.scrim))
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.))
        .on_click(cx.listener(dismiss_overlay))
        .child(
            div()
                .id("dialog-list-card")
                .w(px(a.min_width_dp + 40.0))
                .min_w(px(a.min_width_dp))
                .p(px(a.pad_dp))
                .rounded(px(a.corners.top_left))
                .bg(paint(a.container))
                .gap(px(8.))
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(type_size(a.headline_style))
                        .font_weight(type_weight(a.headline_style))
                        .text_color(paint(a.headline))
                        .child(dialog::RINGTONE_HEADLINE),
                )
                .children(
                    dialog::RINGTONE_OPTIONS
                        .iter()
                        .enumerate()
                        .map(|(i, label)| {
                            let selected = this.ringtone == i;
                            let r = radio::resolve(theme, selected, InteractionState::Enabled);
                            div()
                                .id(SharedString::from(format!("ringtone-{i}")))
                                .w_full()
                                .h(px(r.target_dp))
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(theme.typography.body_large.size_sp))
                                        .text_color(paint(r.label))
                                        .child(*label),
                                )
                                .child(
                                    div()
                                        .w(px(r.outer_dp))
                                        .h(px(r.outer_dp))
                                        .rounded(px(r.outer_dp / 2.0))
                                        .border_2()
                                        .border_color(paint(r.ring))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .when(r.inner.is_some(), |el| {
                                            el.child(
                                                div()
                                                    .w(px(r.inner_dp))
                                                    .h(px(r.inner_dp))
                                                    .rounded(px(r.inner_dp / 2.0))
                                                    .bg(paint(r.inner.unwrap())),
                                            )
                                        }),
                                )
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.ringtone = i;
                                    cx.notify();
                                }))
                        }),
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .justify_end()
                        .gap(px(dialog::ACTION_GAP_DP))
                        .child(m_button(
                            "list-cancel",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RINGTONE_CANCEL,
                            cx.listener(dismiss_overlay),
                        ))
                        .child(m_button(
                            "list-ok",
                            theme,
                            button::ButtonVariant::Text,
                            InteractionState::Enabled,
                            dialog::RINGTONE_OK,
                            cx.listener(dismiss_overlay),
                        )),
                ),
        )
}

fn fullscreen_dialog_overlay(theme: &Theme, cx: &mut Context<CatalogView>) -> impl IntoElement {
    let a = dialog::resolve_fullscreen(theme);
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Filled,
        InteractionState::Enabled,
        false,
    );
    div()
        .id("dialog-fullscreen")
        .flex_1()
        .w_full()
        .bg(paint(a.container))
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(a.header_h_dp))
                .w_full()
                .px(px(16.))
                .flex()
                .items_center()
                .gap(px(12.))
                .child(
                    div()
                        .id("fs-close")
                        .text_size(px(dialog::ICON_DP))
                        .text_color(paint(a.icon))
                        .child(dialog::FULLSCREEN_CLOSE)
                        .on_click(cx.listener(dismiss_overlay)),
                )
                .child(
                    div()
                        .flex_1()
                        .text_size(px(a.headline_style.size_sp))
                        .font_weight(type_weight(a.headline_style))
                        .text_color(paint(a.headline))
                        .child(dialog::FULLSCREEN_HEADLINE),
                )
                .child(m_button(
                    "fs-save",
                    theme,
                    button::ButtonVariant::Text,
                    InteractionState::Enabled,
                    dialog::FULLSCREEN_SAVE,
                    cx.listener(dismiss_overlay),
                )),
        )
        .when(dialog::FULLSCREEN_HAS_DIVIDER, |el| {
            el.child(div().h(px(1.)).w_full().bg(paint(a.divider)))
        })
        .child(div().flex().flex_col().gap(px(12.)).p(px(24.)).children(
            dialog::FULLSCREEN_FIELDS.iter().map(|label| {
                div()
                    .h(px(56.))
                    .px(px(16.))
                    .rounded(px(4.))
                    .bg(paint(field.field.container))
                    .text_color(paint(a.supporting))
                    .flex()
                    .items_center()
                    .child(*label)
            }),
        ))
}

fn day_colors(
    pick: &date_picker::DatePickerAppearance,
    kind: DayKind,
) -> (gpui::Rgba, gpui::Rgba, f32) {
    match kind {
        DayKind::Selected => (
            paint(pick.day_selected_container),
            paint(pick.day_selected),
            pick.day_dp / 2.0,
        ),
        DayKind::InRange => (paint(pick.day_range_container), paint(pick.day_range), 0.0),
        DayKind::Today => (paint(pick.container), paint(pick.day), pick.day_dp / 2.0),
        DayKind::InMonth => (paint(pick.container), paint(pick.day), pick.day_dp / 2.0),
        DayKind::OutOfMonth => (
            paint(pick.container),
            paint(pick.day_out),
            pick.day_dp / 2.0,
        ),
    }
}

fn weekday_row(pick: &date_picker::DatePickerAppearance, cal_w: f32) -> impl IntoElement {
    div()
        .w(px(cal_w))
        .flex()
        .flex_wrap()
        .children(date_picker::WEEKDAYS.iter().map(|d| {
            div()
                .w(px(pick.day_dp))
                .h(px(32.))
                .flex()
                .items_center()
                .justify_center()
                .text_color(paint(pick.weekday))
                .child(*d)
        }))
}

fn date_range_hero(
    this: &CatalogView,
    theme: &Theme,
    pick: &date_picker::DatePickerAppearance,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let today = this.today;
    let year = this.range_year;
    let month = this.range_month;
    let cells = date_picker::month_grid_range_selection(year, month, this.date_range, today);
    let cal_w = pick.day_dp * 7.0;
    let picker = this.range_display == date_picker::DatePickerDisplayMode::Picker;
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    let outline = field
        .field
        .outline
        .map(|(c, _)| c)
        .unwrap_or(theme.color.outline);
    div()
        .w(px(cal_w + 32.0))
        .p(px(16.))
        .rounded(px(pick.corners.top_left))
        .bg(paint(pick.container))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.))
                        .child(spaced_line(
                            date_picker::range_title_for(this.range_display),
                            pick.year_style.size_sp,
                            paint(pick.header_year),
                        ))
                        .child(
                            div()
                                .font_weight(type_weight(pick.date_style))
                                .child(spaced_line(
                                    date_picker::header_range_selection(this.date_range),
                                    pick.date_style.size_sp.min(28.0),
                                    paint(pick.header_date),
                                )),
                        ),
                )
                .when(date_picker::RANGE_SHOW_MODE_TOGGLE, |el| {
                    el.child(
                        div()
                            .id("range-display-toggle")
                            .w(px(date_picker::TOGGLE_SIZE_DP))
                            .h(px(date_picker::TOGGLE_SIZE_DP))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(this.range_display.toggle_icon())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_range_display();
                                cx.notify();
                            })),
                    )
                }),
        )
        .when(picker, |el| {
            el.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .when(
                        this.range_pane == date_picker::DatePickerPane::Calendar,
                        |nav| {
                            nav.child(div().id("range-month-prev").p(px(8.)).child("<").on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.shift_range_month(-1);
                                    cx.notify();
                                }),
                            ))
                        },
                    )
                    .child(
                        div()
                            .id("range-year-toggle")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_range_pane();
                                cx.notify();
                            }))
                            .child(spaced_line(
                                date_picker::month_nav_label(year, month),
                                pick.year_style.size_sp,
                                paint(pick.header_year),
                            )),
                    )
                    .when(
                        this.range_pane == date_picker::DatePickerPane::Calendar,
                        |nav| {
                            nav.child(div().id("range-month-next").p(px(8.)).child(">").on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.shift_range_month(1);
                                    cx.notify();
                                }),
                            ))
                        },
                    ),
            )
        })
        .when(
            picker && this.range_pane == date_picker::DatePickerPane::Calendar,
            |el| {
                el.child(weekday_row(pick, cal_w)).child(
                    div().w(px(cal_w)).flex().flex_wrap().children(
                        cells.iter().copied().enumerate().map(|(i, (day, kind))| {
                            let (bg, fg, radius) = day_colors(pick, kind);
                            let in_month = kind != DayKind::OutOfMonth;
                            div()
                                .id(SharedString::from(format!("range-day-{i}")))
                                .w(px(pick.day_dp))
                                .h(px(pick.day_dp))
                                .rounded(px(radius))
                                .bg(bg)
                                .text_color(fg)
                                .flex()
                                .items_center()
                                .justify_center()
                                .when(kind == DayKind::Today, |el| {
                                    el.border_1().border_color(paint(pick.day_today_outline))
                                })
                                .child(day.to_string())
                                .when(in_month, |el| {
                                    el.on_click(cx.listener(move |this, _, _, cx| {
                                        this.tap_date_range(CivilDate { year, month, day });
                                        cx.notify();
                                    }))
                                })
                        }),
                    ),
                )
            },
        )
        .when(
            picker && this.range_pane == date_picker::DatePickerPane::Year,
            |el| {
                el.child(
                    div()
                        .w(px(cal_w))
                        .flex()
                        .flex_wrap()
                        .justify_between()
                        .children(date_picker::year_window(year).into_iter().map(|y| {
                            let kind = date_picker::classify_year(y, year, this.today.year);
                            let (bg, fg) = match kind {
                                date_picker::YearKind::Selected => {
                                    (paint(pick.day_selected_container), paint(pick.day_selected))
                                }
                                date_picker::YearKind::Today | date_picker::YearKind::Default => {
                                    (paint(pick.container), paint(pick.header_year))
                                }
                            };
                            div()
                                .id(SharedString::from(format!("range-year-{y}")))
                                .w(px(date_picker::YEAR_CONTAINER_W_DP))
                                .h(px(date_picker::YEAR_CONTAINER_H_DP))
                                .mt(px(date_picker::YEAR_GAP_DP / 2.0))
                                .rounded(px(date_picker::YEAR_CONTAINER_H_DP / 2.0))
                                .bg(bg)
                                .text_color(fg)
                                .flex()
                                .items_center()
                                .justify_center()
                                .when(kind == date_picker::YearKind::Today, |el| {
                                    el.border_1().border_color(paint(pick.day_today_outline))
                                })
                                .child(y.to_string())
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.select_range_year(y);
                                    cx.notify();
                                }))
                        })),
                )
            },
        )
        .when(!picker, |el| {
            el.child(
                div()
                    .w_full()
                    .px(px(field.field.pad_start_dp))
                    .py(px(8.))
                    .rounded(px(field.field.corners.top_left))
                    .border_1()
                    .border_color(paint(outline))
                    .child(spaced_line(
                        date_picker::RANGE_START_LABEL,
                        field.label_style.size_sp,
                        paint(field.label),
                    ))
                    .child(spaced_line(
                        date_picker::range_field_value(this.date_range, false),
                        field.input_style.size_sp,
                        paint(field.input),
                    )),
            )
            .child(
                div()
                    .w_full()
                    .px(px(field.field.pad_start_dp))
                    .py(px(8.))
                    .rounded(px(field.field.corners.top_left))
                    .border_1()
                    .border_color(paint(outline))
                    .child(spaced_line(
                        date_picker::RANGE_END_LABEL,
                        field.label_style.size_sp,
                        paint(field.label),
                    ))
                    .child(spaced_line(
                        date_picker::range_field_value(this.date_range, true),
                        field.input_style.size_sp,
                        paint(field.input),
                    )),
            )
        })
        .when(date_picker::RANGE_ACTIONS, |el| {
            el.child(
                div()
                    .w_full()
                    .h(px(date_picker::RANGE_DIVIDER_H_DP))
                    .bg(paint(theme.color.outline_variant)),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .justify_end()
                    .gap(px(dialog::ACTION_GAP_DP))
                    .child(
                        div()
                            .id("range-cancel")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.dismiss_date_range();
                                cx.notify();
                            }))
                            .child(spaced_line(
                                date_picker::INPUT_CANCEL,
                                14.0,
                                paint(theme.color.primary),
                            )),
                    )
                    .child(
                        div()
                            .id("range-ok")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.confirm_date_range();
                                cx.notify();
                            }))
                            .child(spaced_line(
                                date_picker::INPUT_OK,
                                14.0,
                                paint(theme.color.primary),
                            )),
                    ),
            )
        })
}

fn date_input_card(
    this: &CatalogView,
    theme: &Theme,
    pick: &date_picker::DatePickerAppearance,
) -> impl IntoElement {
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    let value = date_picker::input_field_value(this.selected);
    div()
        .w(px(pick.day_dp * 7.0 + 32.0))
        .p(px(16.))
        .rounded(px(pick.corners.top_left))
        .bg(paint(pick.container))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.))
                        .child(spaced_line(
                            date_picker::INPUT_HEADLINE,
                            pick.year_style.size_sp,
                            paint(pick.header_year),
                        ))
                        .child(
                            div()
                                .font_weight(type_weight(pick.date_style))
                                .child(spaced_line(
                                    date_picker::header_date_label(this.selected),
                                    pick.date_style.size_sp,
                                    paint(pick.header_date),
                                )),
                        )
                        .child(spaced_line(
                            date_picker::INPUT_SUPPORTING,
                            pick.year_style.size_sp,
                            paint(pick.header_year),
                        )),
                )
                .child(
                    div()
                        .w(px(48.))
                        .h(px(48.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(date_picker::DEMO_DISPLAY_MODE.toggle_icon()),
                ),
        )
        .child(
            div()
                .w_full()
                .px(px(field.field.pad_start_dp))
                .py(px(8.))
                .rounded(px(field.field.corners.top_left))
                .border_1()
                .border_color(paint(
                    field
                        .field
                        .outline
                        .map(|(c, _)| c)
                        .unwrap_or(theme.color.outline),
                ))
                .child(spaced_line(
                    date_picker::INPUT_FIELD_LABEL,
                    field.label_style.size_sp,
                    paint(field.label),
                ))
                .child(spaced_line(
                    value,
                    field.input_style.size_sp,
                    paint(field.input),
                )),
        )
        .child(
            div()
                .w_full()
                .flex()
                .justify_end()
                .gap(px(dialog::ACTION_GAP_DP))
                .child(spaced_line(
                    date_picker::INPUT_CANCEL,
                    14.0,
                    paint(theme.color.primary),
                ))
                .child(spaced_line(
                    date_picker::INPUT_OK,
                    14.0,
                    paint(theme.color.primary),
                )),
        )
}

fn date_range_input_card(
    theme: &Theme,
    pick: &date_picker::DatePickerAppearance,
) -> impl IntoElement {
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    let start = date_picker::input_field_value(date_picker::RANGE_DEMO_START);
    let end = date_picker::input_field_value(date_picker::RANGE_DEMO_END);
    let outline = field
        .field
        .outline
        .map(|(c, _)| c)
        .unwrap_or(theme.color.outline);
    div()
        .w(px(pick.day_dp * 7.0 + 32.0))
        .p(px(16.))
        .rounded(px(pick.corners.top_left))
        .bg(paint(pick.container))
        .flex()
        .flex_col()
        .gap(px(date_picker::RANGE_INPUT_GAP_DP))
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.))
                        .child(spaced_line(
                            date_picker::RANGE_INPUT_HEADLINE,
                            pick.year_style.size_sp,
                            paint(pick.header_year),
                        ))
                        .child(
                            div()
                                .font_weight(type_weight(pick.date_style))
                                .child(spaced_line(
                                    date_picker::header_range_label(
                                        date_picker::RANGE_DEMO_START,
                                        date_picker::RANGE_DEMO_END,
                                    ),
                                    pick.date_style.size_sp.min(28.0),
                                    paint(pick.header_date),
                                )),
                        ),
                )
                .child(
                    div()
                        .w(px(date_picker::TOGGLE_SIZE_DP))
                        .h(px(date_picker::TOGGLE_SIZE_DP))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(date_picker::DEMO_DISPLAY_MODE.toggle_icon()),
                ),
        )
        .child(div().w_full().h(px(1.)).bg(paint(pick.header_year)))
        .child(
            div()
                .w_full()
                .px(px(field.field.pad_start_dp))
                .py(px(8.))
                .rounded(px(field.field.corners.top_left))
                .border_1()
                .border_color(paint(outline))
                .child(spaced_line(
                    date_picker::RANGE_START_LABEL,
                    field.label_style.size_sp,
                    paint(field.label),
                ))
                .child(spaced_line(
                    start,
                    field.input_style.size_sp,
                    paint(field.input),
                )),
        )
        .child(
            div()
                .w_full()
                .px(px(field.field.pad_start_dp))
                .py(px(8.))
                .rounded(px(field.field.corners.top_left))
                .border_1()
                .border_color(paint(outline))
                .child(spaced_line(
                    date_picker::RANGE_END_LABEL,
                    field.label_style.size_sp,
                    paint(field.label),
                ))
                .child(spaced_line(
                    end,
                    field.input_style.size_sp,
                    paint(field.input),
                )),
        )
        .child(
            div()
                .w_full()
                .flex()
                .justify_end()
                .gap(px(dialog::ACTION_GAP_DP))
                .child(spaced_line(
                    date_picker::INPUT_CANCEL,
                    14.0,
                    paint(theme.color.primary),
                ))
                .child(spaced_line(
                    date_picker::INPUT_OK,
                    14.0,
                    paint(theme.color.primary),
                )),
        )
}

fn date_picker_card(
    this: &CatalogView,
    theme: &Theme,
    pick: &date_picker::DatePickerAppearance,
    cells: &[(u32, DayKind); 42],
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let cal_w = pick.day_dp * 7.0;
    let input_mode = this.date_display == date_picker::DatePickerDisplayMode::Input;
    let year_pane = !input_mode && this.date_pane == date_picker::DatePickerPane::Year;
    let calendar_pane = !input_mode && this.date_pane == date_picker::DatePickerPane::Calendar;
    let field = text_field::resolve(
        theme,
        text_field::TextFieldVariant::Outlined,
        InteractionState::Focused,
        true,
    );
    let value = date_picker::input_field_value(this.selected);
    div()
        .w(px(cal_w + 32.0))
        .p(px(16.))
        .rounded(px(pick.corners.top_left))
        .bg(paint(pick.container))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.))
                        .child(spaced_line(
                            date_picker::INPUT_HEADLINE,
                            pick.year_style.size_sp,
                            paint(pick.header_year),
                        ))
                        .child(
                            div()
                                .font_weight(type_weight(pick.date_style))
                                .child(spaced_line(
                                    date_picker::header_date_label(this.selected),
                                    pick.date_style.size_sp,
                                    paint(pick.header_date),
                                )),
                        )
                        .children(date_picker::supporting_for(this.date_display).map(
                            |supporting| {
                                spaced_line(
                                    supporting,
                                    pick.year_style.size_sp,
                                    paint(pick.header_year),
                                )
                            },
                        )),
                )
                .when(date_picker::SHOW_MODE_TOGGLE, |el| {
                    el.child(
                        div()
                            .id("date-display-toggle")
                            .w(px(date_picker::TOGGLE_SIZE_DP))
                            .h(px(date_picker::TOGGLE_SIZE_DP))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(this.date_display.toggle_icon())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_date_display();
                                cx.notify();
                            })),
                    )
                }),
        )
        .when(!input_mode, |el| {
            el.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .when(calendar_pane, |nav| {
                        nav.child(div().id("month-prev").p(px(8.)).child("<").on_click(
                            cx.listener(|this, _, _, cx| {
                                let (y, m) = date_picker::add_months(
                                    this.picker_year,
                                    this.picker_month,
                                    -1,
                                );
                                this.picker_year = y;
                                this.picker_month = m;
                                cx.notify();
                            }),
                        ))
                    })
                    .child(
                        div()
                            .id("year-control")
                            .child(spaced_line(
                                date_picker::month_nav_label(this.picker_year, this.picker_month),
                                pick.year_style.size_sp,
                                paint(pick.header_year),
                            ))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_date_pane();
                                cx.notify();
                            })),
                    )
                    .when(calendar_pane, |nav| {
                        nav.child(div().id("month-next").p(px(8.)).child(">").on_click(
                            cx.listener(|this, _, _, cx| {
                                let (y, m) =
                                    date_picker::add_months(this.picker_year, this.picker_month, 1);
                                this.picker_year = y;
                                this.picker_month = m;
                                cx.notify();
                            }),
                        ))
                    }),
            )
        })
        .when(calendar_pane, |el| {
            el.child(weekday_row(pick, cal_w)).child(
                div().w(px(cal_w)).flex().flex_wrap().children(
                    cells.iter().copied().enumerate().map(|(i, (day, kind))| {
                        let (bg, fg, radius) = day_colors(pick, kind);
                        let in_month = kind != DayKind::OutOfMonth;
                        let year = this.picker_year;
                        let month = this.picker_month;
                        div()
                            .id(SharedString::from(format!("day-{i}")))
                            .w(px(pick.day_dp))
                            .h(px(pick.day_dp))
                            .rounded(px(radius))
                            .bg(bg)
                            .text_color(fg)
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(kind == DayKind::Today, |el| {
                                el.border_1().border_color(paint(pick.day_today_outline))
                            })
                            .child(day.to_string())
                            .when(in_month, |el| {
                                el.on_click(cx.listener(move |this, _, _, cx| {
                                    this.selected = CivilDate { year, month, day };
                                    cx.notify();
                                }))
                            })
                    }),
                ),
            )
        })
        .when(year_pane, |el| {
            el.child(
                div()
                    .w(px(cal_w))
                    .flex()
                    .flex_wrap()
                    .justify_between()
                    .children(
                        date_picker::year_window(this.picker_year)
                            .into_iter()
                            .map(|year| {
                                let kind = date_picker::classify_year(
                                    year,
                                    this.picker_year,
                                    this.today.year,
                                );
                                let (bg, fg) = match kind {
                                    date_picker::YearKind::Selected => (
                                        paint(pick.day_selected_container),
                                        paint(pick.day_selected),
                                    ),
                                    date_picker::YearKind::Today
                                    | date_picker::YearKind::Default => {
                                        (paint(pick.container), paint(pick.header_year))
                                    }
                                };
                                div()
                                    .id(SharedString::from(format!("year-{year}")))
                                    .w(px(date_picker::YEAR_CONTAINER_W_DP))
                                    .h(px(date_picker::YEAR_CONTAINER_H_DP))
                                    .mt(px(date_picker::YEAR_GAP_DP / 2.0))
                                    .rounded(px(date_picker::YEAR_CONTAINER_H_DP / 2.0))
                                    .bg(bg)
                                    .text_color(fg)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .when(kind == date_picker::YearKind::Today, |el| {
                                        el.border_1().border_color(paint(pick.day_today_outline))
                                    })
                                    .child(year.to_string())
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.select_picker_year(year);
                                        cx.notify();
                                    }))
                            }),
                    ),
            )
        })
        .when(input_mode, |el| {
            el.child(
                div()
                    .w_full()
                    .px(px(field.field.pad_start_dp))
                    .py(px(8.))
                    .rounded(px(field.field.corners.top_left))
                    .border_1()
                    .border_color(paint(
                        field
                            .field
                            .outline
                            .map(|(c, _)| c)
                            .unwrap_or(theme.color.outline),
                    ))
                    .child(spaced_line(
                        date_picker::INPUT_FIELD_LABEL,
                        field.label_style.size_sp,
                        paint(field.label),
                    ))
                    .child(spaced_line(
                        value,
                        field.input_style.size_sp,
                        paint(field.input),
                    )),
            )
        })
        .child(
            div()
                .w_full()
                .flex()
                .justify_end()
                .gap(px(dialog::ACTION_GAP_DP))
                .child(m_button(
                    "date-cancel",
                    theme,
                    button::ButtonVariant::Text,
                    InteractionState::Enabled,
                    "Cancel",
                    |_, _, _| {},
                ))
                .child(m_button(
                    "date-ok",
                    theme,
                    button::ButtonVariant::Text,
                    InteractionState::Enabled,
                    "OK",
                    |_, _, _| {},
                )),
        )
}

fn tab_row(
    a: &tabs::TabsAppearance,
    selected: usize,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let labels = ["One", "Two", "Three"];
    div()
        .w_full()
        .h(px(a.height_dp))
        .bg(paint(a.container))
        .flex()
        .children(labels.into_iter().enumerate().map(|(i, label)| {
            let active = selected == i;
            div()
                .id(SharedString::from(format!("tab-{i}")))
                .flex_1()
                .h_full()
                .flex()
                .flex_col()
                .items_center()
                .justify_end()
                .pb(px(8.))
                .text_color(paint(if active {
                    a.active_label
                } else {
                    a.inactive_label
                }))
                .child(label)
                .when(active, |el| {
                    el.child(
                        div()
                            .mt(px(4.))
                            .h(px(a.indicator_h))
                            .w(px(if a.indicator_full_width { 96. } else { 48. }))
                            .rounded_tl(px(3.))
                            .rounded_tr(px(3.))
                            .bg(paint(a.indicator)),
                    )
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.tab = i;
                    cx.notify();
                }))
        }))
}

fn desktop_mail_snack(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let snack = snackbar::resolve(theme);
    div()
        .id("mail-scene")
        .w(px(snackbar::PHONE_W_DP))
        .rounded(px(snackbar::PHONE_CORNER_DP))
        .p(px(12.))
        .border_color(paint(theme.color.on_surface))
        .overflow_hidden()
        .bg(paint(theme.color.surface))
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(snackbar::STATUS_H_DP))
                .px(px(20.))
                .flex()
                .justify_between()
                .items_center()
                .text_size(px(12.))
                .text_color(paint(theme.color.on_surface))
                .child(snackbar::STATUS_TIME)
                .child("5G · 100%"),
        )
        .children(snackbar::MAIL_ROWS.iter().map(|row| {
            div()
                .h(px(if row.peek { snackbar::PEEK_H_DP } else { 72.0 }))
                .px(px(16.))
                .overflow_hidden()
                .flex()
                .items_center()
                .gap(px(12.))
                .child(photo_avatar(row.photo, snackbar::AVATAR_DP))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .child(spaced_line(row.from, 16.0, paint(theme.color.on_surface)))
                        .child(spaced_line(
                            row.subject,
                            14.0,
                            paint(theme.color.on_surface_variant),
                        )),
                )
                .child(spaced_line(
                    row.time,
                    12.0,
                    paint(theme.color.on_surface_variant),
                ))
        }))
        .when(this.snack_state.visible, |el| {
            el.child(
                div()
                    .id("snackbar")
                    .h(px(snack.min_height_dp))
                    .px(px(16.))
                    .ml(px(this.snack_state.offset_x_dp))
                    .rounded(px(snack.corners.top_left))
                    .bg(paint(snack.container))
                    .text_color(paint(snack.supporting))
                    .flex()
                    .items_center()
                    .justify_between()
                    .opacity(this.snack_state.opacity())
                    .child(spaced_line(
                        snackbar::SCENE_MESSAGE,
                        snack.supporting_style.size_sp,
                        paint(snack.supporting),
                    ))
                    .child(
                        div()
                            .text_color(paint(snack.action))
                            .child(snackbar::SCENE_ACTION),
                    )
                    .child(
                        div()
                            .id("snack-close")
                            .w(px(snackbar::CLOSE_DP))
                            .h(px(snackbar::CLOSE_DP))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(paint(snack.close))
                            .child(snackbar::CLOSE_GLYPH)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.snack_state.close();
                                cx.notify();
                            })),
                    )
                    .on_scroll_wheel(cx.listener(|this, ev: &ScrollWheelEvent, _, cx| {
                        let dx = match ev.delta {
                            ScrollDelta::Pixels(p) => f32::from(p.x),
                            ScrollDelta::Lines(p) => p.x * 16.0,
                        };
                        this.snack_state.swipe(dx);
                        cx.notify();
                    })),
            )
        })
        .child({
            let nav = navigation_bar::resolve(theme);
            div()
                .h(px(nav.height_dp))
                .w_full()
                .bg(paint(nav.container))
                .flex()
                .children(snackbar::INBOX_NAV.iter().enumerate().map(|(i, dest)| {
                    let active = i == 0;
                    div()
                        .flex_1()
                        .h_full()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(nav.icon_label_gap_dp))
                        .text_color(paint(if active {
                            nav.active_label
                        } else {
                            nav.inactive_label
                        }))
                        .child(
                            div()
                                .w(px(nav.indicator_w_dp))
                                .h(px(nav.indicator_h_dp))
                                .rounded(px(16.))
                                .bg(paint(if active {
                                    nav.active_indicator
                                } else {
                                    nav.container
                                }))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(dest.glyph),
                        )
                        .child(dest.label)
                }))
        })
}

fn desktop_nav_bars(theme: &Theme) -> impl IntoElement {
    let compact = navigation_bar::resolve(theme);
    let medium = navigation_bar::resolve_horizontal(theme);
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(desktop_nav_row(
            &compact,
            navigation_bar::COMPACT_DESTS.as_slice(),
        ))
        .child(desktop_nav_row(
            &medium,
            navigation_bar::MEDIUM_DESTS.as_slice(),
        ))
}

fn desktop_nav_row(
    nav: &navigation_bar::NavBarAppearance,
    dests: &[&'static str],
) -> impl IntoElement {
    let horizontal = nav.layout == navigation_bar::NavBarLayout::Horizontal;
    div()
        .h(px(nav.height_dp))
        .w_full()
        .bg(paint(nav.container))
        .flex()
        .items_center()
        .when(horizontal, |el| el.justify_center().gap(px(8.)))
        .children(dests.iter().enumerate().map(|(i, label)| {
            let active = i == 0;
            let item = div()
                .when(!horizontal, |el| el.flex_1())
                .h_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(paint(if active {
                    nav.active_label
                } else {
                    nav.inactive_label
                }));
            if horizontal {
                item.child(
                    div()
                        .h(px(nav.indicator_h_dp))
                        .px(px(nav.indicator_pad_h_dp))
                        .rounded(px(nav.indicator_h_dp / 2.0))
                        .gap(px(nav.icon_label_gap_dp))
                        .flex()
                        .items_center()
                        .bg(paint(if active {
                            nav.active_indicator
                        } else {
                            nav.container
                        }))
                        .text_color(paint(if active {
                            nav.active_icon
                        } else {
                            nav.inactive_icon
                        }))
                        .child(if active { "●" } else { "○" })
                        .child(*label),
                )
            } else {
                item.flex_col()
                    .gap(px(nav.icon_label_gap_dp))
                    .child(
                        div()
                            .w(px(nav.indicator_w_dp))
                            .h(px(nav.indicator_h_dp))
                            .rounded(px(16.))
                            .bg(paint(if active {
                                nav.active_indicator
                            } else {
                                nav.container
                            }))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(paint(if active {
                                nav.active_icon
                            } else {
                                nav.inactive_icon
                            }))
                            .child(if active { "●" } else { "○" }),
                    )
                    .child(*label)
            }
        }))
}

fn tooltip_caret(color: gpui::Rgba, down: bool) -> impl IntoElement {
    let pts = if down {
        tooltip::caret_down_points()
    } else {
        tooltip::caret_up_points()
    };
    canvas(
        move |_, _, _| {},
        move |bounds, _, window, _| {
            paint_filled_polygon(window, bounds.origin, &pts, color);
        },
    )
    .w(px(tooltip::CARET_W_DP))
    .h(px(tooltip::CARET_H_DP))
}

fn desktop_list_swipe(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let off = this.list_swipe.offset_x_dp;
    let lead_w = list::leading_rail_width_dp(off);
    let trail_w = list::trailing_rail_width_dp(off);
    div()
        .w(px(list::SWIPE_ROW_WIDTH_DP))
        .overflow_hidden()
        .relative()
        .child(
            div()
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .flex()
                .child(
                    div()
                        .w(px(lead_w))
                        .h_full()
                        .bg(paint(list::leading_action_container(theme)))
                        .text_color(paint(list::leading_action_content(theme)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(list::SWIPE_LEADING_LABEL),
                )
                .child(div().flex_1())
                .child(
                    div()
                        .w(px(trail_w))
                        .h_full()
                        .bg(paint(list::trailing_action_container(theme)))
                        .text_color(paint(list::trailing_action_content(theme)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(list::SWIPE_TRAILING_LABEL),
                ),
        )
        .child(
            div()
                .ml(px(off))
                .flex()
                .flex_col()
                .bg(paint(theme.color.surface))
                .children((0..list::SWIPE_COUNT).map(|i| {
                    let a = list::resolve_swipe_item(theme, i, list::SWIPE_COUNT);
                    div()
                        .id(SharedString::from(format!("swipe-{i}")))
                        .h(px(a.height_dp))
                        .px(px(a.pad_start_dp))
                        .bg(paint(a.container))
                        .flex()
                        .flex_col()
                        .justify_center()
                        .child(
                            div()
                                .text_size(px(a.label_style.size_sp))
                                .text_color(paint(a.content))
                                .child(list::SWIPE_HEADLINES[i]),
                        )
                        .child(
                            div()
                                .text_size(px(a.supporting_style.unwrap().size_sp))
                                .text_color(paint(a.secondary_content.unwrap()))
                                .child(list::SWIPE_SUPPORTING[i]),
                        )
                }))
                .on_scroll_wheel(cx.listener(|this, ev: &ScrollWheelEvent, _, cx| {
                    let dx = match ev.delta {
                        ScrollDelta::Pixels(p) => f32::from(p.x),
                        ScrollDelta::Lines(p) => p.x * 16.0,
                    };
                    list::apply_wheel(&mut this.list_swipe, dx);
                    this.list_swipe_at = None;
                    cx.notify();
                })),
        )
}

fn desktop_list_reorder(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let order = this.list_order;
    div()
        .flex()
        .flex_col()
        .gap(px(list::SEGMENTED_GAP_DP))
        .w(px(360.))
        .children((0..list::REORDER_COUNT).map(move |pos| {
            let id = order[pos];
            let a = list::resolve_reorder_item(theme, pos, list::REORDER_COUNT, pos == 0);
            div()
                .id(SharedString::from(format!("reorder-{id}")))
                .h(px(a.height_dp))
                .px(px(a.pad_start_dp))
                .rounded_tl(px(a.corners.top_left))
                .rounded_tr(px(a.corners.top_right))
                .rounded_br(px(a.corners.bottom_right))
                .rounded_bl(px(a.corners.bottom_left))
                .bg(paint(a.container))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(list::ITEM_BETWEEN_SPACE_DP))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .child(
                            div()
                                .text_size(px(a.label_style.size_sp))
                                .text_color(paint(a.content))
                                .child(list::REORDER_HEADLINES[id]),
                        )
                        .child(
                            div()
                                .text_size(px(a.supporting_style.unwrap().size_sp))
                                .text_color(paint(a.secondary_content.unwrap()))
                                .child(list::REORDER_SUPPORTING[id]),
                        ),
                )
                .child(
                    div()
                        .id(SharedString::from(format!("handle-{id}")))
                        .w(px(list::DRAG_HANDLE_DP))
                        .h(px(list::DRAG_HANDLE_DP))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(paint(a.content))
                        .child(list::DRAG_HANDLE_GLYPH)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if pos > 0 {
                                list::move_item(&mut this.list_order, pos, pos - 1);
                                cx.notify();
                            }
                        })),
                )
        }))
}

fn desktop_lists(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let selected = this.list_selected;
    div()
        .flex()
        .flex_col()
        .gap(px(list::SEGMENTED_GAP_DP))
        .w(px(360.))
        .children((0..list::SCENE_COUNT).map(move |i| {
            let a = list::resolve_scene(theme, i, selected);
            let sw = switch::resolve(theme, list::SCENE_TRAILING_ON[i], InteractionState::Enabled);
            div()
                .id(SharedString::from(format!("list-{i}")))
                .h(px(a.height_dp))
                .px(px(a.pad_start_dp))
                .rounded_tl(px(a.corners.top_left))
                .rounded_tr(px(a.corners.top_right))
                .rounded_br(px(a.corners.bottom_right))
                .rounded_bl(px(a.corners.bottom_left))
                .bg(paint(a.container))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(list::ITEM_BETWEEN_SPACE_DP))
                .child(
                    div()
                        .w(px(list::LEADING_ICON_DP))
                        .h(px(list::LEADING_ICON_DP))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(paint(a.content))
                        .child(list::SCENE_ICONS[i]),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .child(
                            div()
                                .text_size(px(a.label_style.size_sp))
                                .text_color(paint(a.content))
                                .child(list::SCENE_HEADLINES[i]),
                        )
                        .child(
                            div()
                                .text_size(px(a.supporting_style.unwrap().size_sp))
                                .text_color(paint(a.secondary_content.unwrap()))
                                .child(list::SCENE_SUPPORTING[i]),
                        ),
                )
                .child(mini_switch(&sw, list::SCENE_TRAILING_ON[i]))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.list_selected = i;
                    cx.notify();
                }))
        }))
}

fn mini_switch(sw: &switch::SwitchAppearance, on: bool) -> impl IntoElement {
    div()
        .w(px(sw.track_w))
        .h(px(sw.track_h))
        .rounded(px(sw.track_h / 2.0))
        .bg(paint(sw.track))
        .when(sw.track_outline.is_some(), |el| {
            el.border_2().border_color(paint(sw.track_outline.unwrap()))
        })
        .flex()
        .items_center()
        .px(px(4.))
        .when(on, |el| el.justify_end())
        .child(
            div()
                .w(px(sw.thumb_dp))
                .h(px(sw.thumb_dp))
                .rounded(px(sw.thumb_dp / 2.0))
                .bg(paint(sw.thumb)),
        )
}

fn desktop_tooltips(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let plain = tooltip::resolve_plain(theme);
    let rich = tooltip::resolve_rich(theme);
    let anchor = icon_button::resolve(
        theme,
        icon_button::IconButtonVariant::Tonal,
        InteractionState::Enabled,
    );
    div()
        .flex()
        .flex_wrap()
        .gap(px(32.))
        .items_end()
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(tooltip::ANCHOR_GAP_DP))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .when(this.tooltip_plain_open, |el| {
                            el.child(
                                div()
                                    .h(px(plain.min_height_dp))
                                    .px(px(plain.pad_start_dp))
                                    .py(px(plain.pad_top_dp))
                                    .rounded(px(plain.corners.top_left))
                                    .bg(paint(plain.container))
                                    .text_color(paint(plain.supporting))
                                    .text_size(px(plain.supporting_style.size_sp))
                                    .flex()
                                    .items_center()
                                    .child(tooltip::PLAIN_TEXT),
                            )
                            .child(tooltip_caret(paint(plain.container), true))
                        }),
                )
                .child(
                    div()
                        .id("tooltip-plain-anchor")
                        .w(px(anchor.height_dp))
                        .h(px(anchor.height_dp))
                        .rounded(px(anchor.corners.top_left))
                        .bg(paint(anchor.container))
                        .text_color(paint(anchor.content))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(tooltip::PLAIN_ANCHOR)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.tooltip_plain_open = !this.tooltip_plain_open;
                            cx.notify();
                        })),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .id("tooltip-rich-anchor")
                        .w(px(anchor.height_dp))
                        .h(px(anchor.height_dp))
                        .rounded(px(anchor.corners.top_left))
                        .bg(paint(anchor.container))
                        .text_color(paint(anchor.content))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child("?")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.tooltip_rich_open = !this.tooltip_rich_open;
                            cx.notify();
                        })),
                )
                .when(this.tooltip_rich_open, |el| {
                    el.child(tooltip_caret(paint(rich.container), false)).child(
                        div()
                            .w(px(280.))
                            .px(px(rich.pad_start_dp))
                            .pt(px(rich.pad_top_dp))
                            .pb(px(rich.pad_bottom_dp))
                            .rounded(px(rich.corners.top_left))
                            .bg(paint(rich.container))
                            .shadow_sm()
                            .flex()
                            .flex_col()
                            .gap(px(4.))
                            .child(
                                div()
                                    .text_size(px(rich.subhead_style.unwrap().size_sp))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(paint(rich.subhead.unwrap()))
                                    .child(tooltip::RICH_SUBHEAD),
                            )
                            .child(
                                div()
                                    .text_size(px(rich.supporting_style.size_sp))
                                    .text_color(paint(rich.supporting))
                                    .child(tooltip::RICH_SUPPORTING),
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_end()
                                    .gap(px(16.))
                                    .pt(px(8.))
                                    .text_color(paint(rich.action.unwrap()))
                                    .child(tooltip::RICH_ACTION_PRIMARY)
                                    .child(tooltip::RICH_ACTION_SECONDARY),
                            ),
                    )
                }),
        )
}

fn desktop_media_scene(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = tabs::resolve_with_icons(theme, tabs::TabsVariant::Primary);
    div()
        .id("media-scene")
        .w(px(tabs::PHONE_W_DP))
        .rounded(px(tabs::PHONE_CORNER_DP))
        .p(px(12.))
        .border_color(paint(theme.color.on_surface))
        .overflow_hidden()
        .bg(paint(theme.color.surface))
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(tabs::STATUS_H_DP))
                .px(px(20.))
                .flex()
                .justify_between()
                .items_center()
                .text_size(px(12.))
                .text_color(paint(theme.color.on_surface))
                .child(tabs::STATUS_TIME)
                .child("5G · 100%"),
        )
        .child(
            div()
                .h(px(56.))
                .px(px(16.))
                .flex()
                .items_center()
                .gap(px(12.))
                .child(spaced_line(
                    tabs::SCENE_LEADING,
                    18.0,
                    paint(theme.color.on_surface),
                ))
                .child(div().flex_1().child(spaced_line(
                    tabs::SCENE_TITLE,
                    theme.typography.title_large.size_sp,
                    paint(theme.color.on_surface),
                )))
                .children(
                    tabs::SCENE_TRAILING
                        .iter()
                        .map(|g| spaced_line(*g, 16.0, paint(theme.color.on_surface))),
                ),
        )
        .child(
            div()
                .w_full()
                .h(px(a.height_dp))
                .bg(paint(a.container))
                .flex()
                .children(
                    tabs::SCENE_ICONS
                        .iter()
                        .zip(tabs::SCENE_LABELS.iter())
                        .enumerate()
                        .map(|(i, (icon, label))| {
                            let active = this.tab == i;
                            div()
                                .id(SharedString::from(format!("media-tab-{i}")))
                                .flex_1()
                                .h_full()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_end()
                                .pb(px(8.))
                                .text_color(paint(if active {
                                    a.active_label
                                } else {
                                    a.inactive_label
                                }))
                                .child(*icon)
                                .child(*label)
                                .when(active, |el| {
                                    el.child(
                                        div()
                                            .mt(px(4.))
                                            .h(px(a.indicator_h))
                                            .w(px(48.))
                                            .rounded_tl(px(3.))
                                            .rounded_tr(px(3.))
                                            .bg(paint(a.indicator)),
                                    )
                                })
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.tab = i;
                                    cx.notify();
                                }))
                        }),
                ),
        )
        .child(div().p(px(12.)).flex().flex_wrap().gap(px(8.)).children(
            tabs::SCENE_TILES.iter().enumerate().map(|(i, _caption)| {
                div()
                    .w(px(150.))
                    .h(px(tabs::SCENE_TILE_H_DP))
                    .rounded(px(tabs::SCENE_TILE_CORNER_DP))
                    .overflow_hidden()
                    .child(photo_fill(tabs::scene_tile_kind(i)))
            }),
        ))
}

fn desktop_app_bar_scene(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = top_app_bar::resolve_scene(theme, this.app_bar_collapse);
    let search_a = top_app_bar::resolve_search(theme);
    let search = search::resolve(theme);
    div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(
            div()
                .id("appbar-scene")
                .w(px(top_app_bar::PHONE_W_DP))
                .rounded(px(top_app_bar::PHONE_CORNER_DP))
                .p(px(12.))
                .overflow_hidden()
                .bg(paint(theme.color.surface))
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(top_app_bar::STATUS_H_DP))
                        .px(px(20.))
                        .flex()
                        .justify_between()
                        .items_center()
                        .text_size(px(12.))
                        .text_color(paint(theme.color.on_surface))
                        .child(top_app_bar::STATUS_TIME)
                        .child("5G · 100%"),
                )
                .child(
                    div()
                        .id("appbar-large")
                        .w_full()
                        .h(px(a.height_dp))
                        .bg(paint(a.container))
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .h(px(64.))
                                .px(px(8.))
                                .flex()
                                .items_center()
                                .gap(px(8.))
                                .child(spaced_line(top_app_bar::SCENE_LEADING, 18.0, paint(a.icon)))
                                .when(this.app_bar_collapse >= 0.999, |el| {
                                    el.child(div().flex_1().child(spaced_line(
                                        top_app_bar::SCENE_TITLE,
                                        a.title_style.size_sp,
                                        paint(a.title),
                                    )))
                                })
                                .when(this.app_bar_collapse < 0.999, |el| el.child(div().flex_1()))
                                .children(
                                    top_app_bar::SCENE_TRAILING
                                        .iter()
                                        .map(|g| spaced_line(*g, 16.0, paint(a.icon))),
                                ),
                        )
                        .when(this.app_bar_collapse < 0.999, |el| {
                            el.child(
                                div()
                                    .px(px(16.))
                                    .pb(px(12.))
                                    .flex()
                                    .flex_col()
                                    .child(spaced_line(
                                        top_app_bar::SCENE_TITLE,
                                        a.title_style.size_sp,
                                        paint(a.title),
                                    ))
                                    .child(spaced_line(
                                        top_app_bar::SCENE_SUBTITLE,
                                        a.subtitle_style.size_sp,
                                        paint(a.subtitle),
                                    )),
                            )
                        })
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app_bar_collapse =
                                top_app_bar::next_collapse(this.app_bar_collapse);
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .w_full()
                        .h(px(top_app_bar::PHOTO_H_DP))
                        .overflow_hidden()
                        .child(photo_fill(top_app_bar::SCENE_PHOTO)),
                ),
        )
        .child(
            div()
                .w(px(top_app_bar::PHONE_W_DP))
                .h(px(search_a.height_dp))
                .bg(paint(search_a.container))
                .flex()
                .items_center()
                .px(px(8.))
                .child(
                    div()
                        .flex_1()
                        .h(px(search_a.search_field_h_dp))
                        .rounded(px(search_a.search_field_h_dp / 2.0))
                        .px(px(16.))
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .bg(paint(search.bar.container))
                        .child(spaced_line(
                            search::LEADING_ICON,
                            16.0,
                            paint(search.leading_icon),
                        ))
                        .child(div().flex_1().child(spaced_line(
                            top_app_bar::SEARCH_PLACEHOLDER,
                            search.placeholder_style.size_sp,
                            paint(search.placeholder),
                        ))),
                ),
        )
}

fn desktop_side_sheet(theme: &Theme) -> impl IntoElement {
    let a = side_sheet::resolve_scene(theme);
    div()
        .id("side-scene")
        .w(px(side_sheet::PHONE_W_DP))
        .h(px(side_sheet::PHONE_H_DP))
        .rounded(px(side_sheet::PHONE_CORNER_DP))
        .overflow_hidden()
        .relative()
        .bg(paint(theme.color.surface))
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(side_sheet::STATUS_H_DP))
                .px(px(20.))
                .flex()
                .justify_between()
                .items_center()
                .text_size(px(12.))
                .text_color(paint(theme.color.on_surface))
                .child(side_sheet::STATUS_TIME)
                .child("5G · 100%"),
        )
        .child(div().p(px(8.)).flex().flex_wrap().gap(px(8.)).children(
            side_sheet::SCENE_PHOTOS.iter().map(|kind| {
                div()
                    .w(px(160.))
                    .h(px(side_sheet::PHOTO_TILE_H_DP))
                    .overflow_hidden()
                    .child(photo_fill(*kind))
            }),
        ))
        .child(
            div()
                .id("side-scrim")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .bg(paint(a.scrim)),
        )
        .child(
            div()
                .id("side-sheet")
                .absolute()
                .top(px(0.))
                .right(px(0.))
                .h_full()
                .w(px(a.width_dp))
                .bg(paint(a.container))
                .rounded_tl(px(a.corners.top_left))
                .rounded_bl(px(a.corners.bottom_left))
                .flex()
                .flex_col()
                .child(
                    div()
                        .px(px(side_sheet::PAD_H_DP))
                        .pt(px(16.))
                        .pb(px(12.))
                        .flex()
                        .justify_between()
                        .items_center()
                        .child(spaced_line(
                            side_sheet::HEADLINE,
                            a.headline_style.size_sp,
                            paint(a.headline),
                        ))
                        .child(spaced_line(side_sheet::CLOSE_GLYPH, 16.0, paint(a.close))),
                )
                .children(side_sheet::FILTERS.iter().map(|(label, value)| {
                    div()
                        .h(px(56.))
                        .px(px(side_sheet::PAD_H_DP))
                        .flex()
                        .justify_between()
                        .items_center()
                        .child(spaced_line(*label, 16.0, paint(a.content)))
                        .child(spaced_line(*value, 14.0, paint(a.supporting)))
                }))
                .child(
                    div()
                        .h(px(side_sheet::ACTIONS_H_DP))
                        .px(px(side_sheet::PAD_H_DP))
                        .flex()
                        .items_center()
                        .child(spaced_line(side_sheet::APPLY_LABEL, 14.0, paint(a.action))),
                ),
        )
}

fn desktop_share_sheet(theme: &Theme) -> impl IntoElement {
    let a = bottom_sheet::resolve(theme, true);
    div()
        .id("share-scene")
        .w(px(bottom_sheet::PHONE_W_DP))
        .rounded(px(bottom_sheet::PHONE_CORNER_DP))
        .p(px(12.))
        .border_color(paint(theme.color.on_surface))
        .overflow_hidden()
        .relative()
        .bg(paint(theme.color.surface))
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(bottom_sheet::STATUS_H_DP))
                .px(px(20.))
                .flex()
                .justify_between()
                .items_center()
                .text_size(px(12.))
                .text_color(paint(theme.color.on_surface))
                .child(bottom_sheet::STATUS_TIME)
                .child("5G · 100%"),
        )
        .child(
            div().p(px(8.)).flex().flex_wrap().gap(px(4.)).children(
                bottom_sheet::PHOTO_GRID
                    .iter()
                    .enumerate()
                    .map(|(i, _caption)| {
                        div()
                            .w(px(108.))
                            .h(px(bottom_sheet::PHOTO_TILE_H_DP))
                            .rounded(px(bottom_sheet::PHOTO_TILE_CORNER_DP))
                            .overflow_hidden()
                            .child(photo_fill(bottom_sheet::photo_kind(i)))
                    }),
            ),
        )
        .child(
            div()
                .w_full()
                .bg(paint(a.container))
                .rounded_tl(px(a.corners.top_left))
                .rounded_tr(px(a.corners.top_right))
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .mt(px(16.))
                        .mb(px(8.))
                        .w(px(a.handle_w))
                        .h(px(a.handle_h))
                        .rounded(px(2.))
                        .bg(paint(a.handle)),
                )
                .child(
                    div()
                        .w_full()
                        .px(px(8.))
                        .pb(px(8.))
                        .flex()
                        .justify_between()
                        .children(
                            bottom_sheet::SHARE_ACTIONS
                                .into_iter()
                                .map(|(icon, label)| {
                                    div()
                                        .w(px(56.))
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap(px(4.))
                                        .text_color(paint(a.content))
                                        .child(icon)
                                        .child(spaced_line(label, 10.0, paint(a.content)))
                                }),
                        ),
                )
                .child(spaced_line(
                    bottom_sheet::SEND_TITLE,
                    theme.typography.title_medium.size_sp,
                    paint(a.content),
                ))
                .child(
                    div()
                        .w_full()
                        .px(px(8.))
                        .pb(px(8.))
                        .flex()
                        .gap(px(8.))
                        .children(bottom_sheet::PEOPLE.iter().map(|person| {
                            div()
                                .w(px(bottom_sheet::PEOPLE_DP))
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap(px(4.))
                                .child(photo_avatar(person.photo, 40.0))
                                .child(spaced_line(person.first, 11.0, paint(a.content)))
                        })),
                ),
        )
}

fn m_button(
    id: &'static str,
    theme: &Theme,
    variant: button::ButtonVariant,
    state: InteractionState,
    label: impl Into<SharedString>,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let a = button::resolve(theme, variant, state);
    let disabled = state.is_disabled();
    div()
        .id(id)
        .h(px(a.height_dp))
        .px(px(a.pad_start_dp))
        .flex()
        .items_center()
        .justify_center()
        .when(a.container.a() > 0, |el| {
            el.rounded(px(a.corners.top_left)).bg(paint(a.container))
        })
        .text_color(paint(a.content))
        .text_size(type_size(a.label_style))
        .font_weight(FontWeight::MEDIUM)
        .when(a.outline.is_some(), |el| {
            let (color, _) = a.outline.unwrap();
            el.border_1().border_color(paint(color))
        })
        .child(spaced_line(
            label.into().to_string(),
            a.label_style.size_sp,
            paint(a.content),
        ))
        .when(!disabled, |el| el.on_click(on_click))
}

fn search_bar_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let open = this.search_open;
    let a = search::resolve(theme);
    let view = search::resolve_view(theme);
    let grouped = search::filter_grouped_suggestions_in(this.search.value(), this.search_filter);
    let suggestion_count = search::contained_suggestion_count();
    let query_label = if this.search.focused {
        this.search.display_with_caret()
    } else {
        search::query_display(this.search.value()).to_string()
    };
    let morph_ms = search::morph_ms(theme) as u64;
    let search_layout = search::WindowWidthClass::Medium.expanded_search();
    let contained_bg = search::contained_container(theme);
    let query_color = paint(if open && !this.search.value().is_empty() {
        view.input
    } else if open {
        view.placeholder
    } else {
        a.placeholder
    });
    let lead_color = paint(a.leading_icon);
    let back_color = paint(view.header);
    let avatar_bg = paint(a.avatar);
    let avatar_fg = paint(a.avatar_label);
    let mic_color = paint(if open { view.header } else { a.trailing_icon });
    let header = div()
        .id(if open {
            "search-activity"
        } else {
            "search-bar"
        })
        .w_full()
        .px(px(16.))
        .flex()
        .items_center()
        .gap(px(search::GAP_DP))
        .when(!open, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                this.search_open = true;
                this.search.set_focus(true);
                cx.notify();
            }))
        })
        .with_animation(
            if open {
                "search-header-in"
            } else {
                "search-header-out"
            },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let linear = if open { delta } else { 1.0 - delta };
                let frame =
                    search::contained_frame_eased_layout(search_layout, linear, suggestion_count);
                this.h(px(frame.header_h_dp))
            },
        )
        .child(
            div()
                .relative()
                .w(px(search::ICON_DP))
                .h(px(search::ICON_DP))
                .child(
                    div()
                        .id("search-lead-docked")
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(search::ICON_DP))
                        .h(px(search::ICON_DP))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(lead_color)
                        .child(search::LEADING_ICON)
                        .with_animation(
                            if open {
                                "search-lead-out"
                            } else {
                                "search-lead-in"
                            },
                            Animation::new(Duration::from_millis(morph_ms)),
                            move |this, delta| {
                                let linear = if open { delta } else { 1.0 - delta };
                                this.opacity(search::morph_avatar_opacity(search::morph_eased_t(
                                    linear,
                                )))
                            },
                        ),
                )
                .child(
                    div()
                        .id("search-back")
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(search::ICON_DP))
                        .h(px(search::ICON_DP))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(back_color)
                        .child(search::VIEW_BACK)
                        .when(open, |el| {
                            el.on_click(cx.listener(|this, _, _, cx| {
                                this.search_open = false;
                                this.search.set_value("");
                                this.search.set_focus(false);
                                cx.notify();
                            }))
                        })
                        .with_animation(
                            if open {
                                "search-back-in"
                            } else {
                                "search-back-out"
                            },
                            Animation::new(Duration::from_millis(morph_ms)),
                            move |this, delta| {
                                let linear = if open { delta } else { 1.0 - delta };
                                this.opacity(search::morph_back_opacity(search::morph_eased_t(
                                    linear,
                                )))
                            },
                        ),
                ),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(a.placeholder_style.size_sp))
                .text_color(query_color)
                .child(query_label),
        )
        .child({
            let show_clear = search::shows_clear(this.search.value());
            div()
                .id("search-trailing")
                .w(px(search::ICON_DP))
                .h(px(search::ICON_DP))
                .flex()
                .items_center()
                .justify_center()
                .text_color(mic_color)
                .child(search::trailing_action(this.search.value()))
                .when(show_clear, |el| {
                    el.on_click(cx.listener(|this, _, _, cx| {
                        search::apply_clear(&mut this.search);
                        cx.notify();
                    }))
                })
        })
        .child(
            div()
                .w(px(search::AVATAR_DP))
                .h(px(search::AVATAR_DP))
                .rounded(px(search::AVATAR_DP / 2.0))
                .bg(avatar_bg)
                .text_color(avatar_fg)
                .flex()
                .items_center()
                .justify_center()
                .child("A")
                .with_animation(
                    if open {
                        "search-avatar-out"
                    } else {
                        "search-avatar-in"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let linear = if open { delta } else { 1.0 - delta };
                        this.opacity(search::morph_avatar_opacity(search::morph_eased_t(linear)))
                    },
                ),
        )
        .into_any_element();
    let status = search::list_status(this.search.value(), this.search.focused);
    let show_groups = status.shows_suggestion_groups();
    let mut flat = 0usize;
    let mut list_children: Vec<gpui::AnyElement> = Vec::new();
    if let Some(heading) = status.heading() {
        list_children.push(
            div()
                .id("search-status")
                .h(px(search::STATUS_H_DP))
                .px(px(16.))
                .flex()
                .items_center()
                .text_size(px(12.))
                .text_color(paint(view.suggestion_icon))
                .child(heading)
                .into_any_element(),
        );
    }
    if status.shows_filters() {
        list_children.push(
            div()
                .id("search-filters")
                .h(px(search::FILTER_ROW_H_DP))
                .px(px(16.))
                .flex()
                .items_center()
                .gap(px(search::FILTER_GAP_DP))
                .children(search::SearchFilter::ALL.iter().copied().map(|f| {
                    let selected = this.search_filter == f;
                    let a = chip::resolve(
                        theme,
                        chip::ChipVariant::Filter,
                        selected,
                        InteractionState::Enabled,
                    );
                    div()
                        .id(SharedString::from(format!("search-filter-{}", f.attr())))
                        .h(px(a.height_dp))
                        .px(px(a.pad_start_dp))
                        .flex()
                        .items_center()
                        .gap(px(8.))
                        .bg(paint(a.container))
                        .text_color(paint(a.content))
                        .rounded(px(a.corners.top_left))
                        .when(selected, |el| el.child(chip::CHECK_GLYPH))
                        .child(f.label())
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.search_filter = f;
                            cx.notify();
                        }))
                }))
                .into_any_element(),
        );
    }
    if grouped.is_empty() {
        list_children.push(
            div()
                .id("search-empty")
                .h(px(search::EMPTY_H_DP))
                .px(px(16.))
                .flex()
                .items_center()
                .text_color(paint(search::empty_content(theme)))
                .child(search::EMPTY_SUGGESTIONS)
                .into_any_element(),
        );
    } else {
        let flatten = !show_groups;
        let flat_count: usize = grouped.iter().map(|(_, items)| items.len()).sum();
        let query = this.search.value().to_string();
        for (gi, (title, items)) in grouped.into_iter().enumerate() {
            let group_len = items.len();
            list_children.push(
                div()
                    .id(SharedString::from(format!("search-group-{gi}")))
                    .flex()
                    .flex_col()
                    .gap(px(search::ROW_GAP_DP))
                    .px(px(8.))
                    .when(gi > 0, |el| {
                        el.mt(px(if show_groups {
                            search::SUGGESTION_GROUP_GAP_DP
                        } else {
                            search::ROW_GAP_DP
                        }))
                    })
                    .when(show_groups, |el| {
                        el.child(
                            div()
                                .h(px(search::SUGGESTION_GROUP_TITLE_H_DP))
                                .px(px(8.))
                                .flex()
                                .items_center()
                                .text_size(px(12.))
                                .text_color(paint(view.suggestion_icon))
                                .child(title),
                        )
                    })
                    .children(items.into_iter().enumerate().map(|(gi_row, label)| {
                        let i = flat;
                        flat += 1;
                        let (index, count) = if flatten {
                            (i, flat_count)
                        } else {
                            (gi_row, group_len)
                        };
                        let selected = !query.is_empty() && label.eq_ignore_ascii_case(&query);
                        let corners = search::row_corners(index, count, selected);
                        let q = query.clone();
                        let two = status.uses_two_line_rows();
                        let open = status.shows_open_affordance();
                        let supporting = search::supporting_for(label);
                        div()
                            .id(SharedString::from(format!("search-sug-{i}")))
                            .h(px(search::row_height_dp(status)))
                            .px(px(16.))
                            .flex()
                            .items_center()
                            .gap(px(search::row_leading_gap_dp(status)))
                            .bg(paint(search::row_container(theme, selected)))
                            .text_color(paint(search::row_content(theme, selected)))
                            .rounded_tl(px(corners.top_left))
                            .rounded_tr(px(corners.top_right))
                            .rounded_br(px(corners.bottom_right))
                            .rounded_bl(px(corners.bottom_left))
                            .child({
                                let kind = search::row_leading_kind(status, label);
                                let sz = search::row_leading_size_dp(status, kind);
                                let mut lead = div()
                                    .w(px(sz))
                                    .h(px(sz))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(paint(search::row_leading_content(theme, kind)))
                                    .child(search::row_leading_glyph(kind, label, i));
                                if let Some(bg) = search::row_leading_container(theme, kind) {
                                    lead = lead.bg(paint(bg)).rounded(px(sz / 2.0));
                                }
                                lead
                            })
                            .child(if two {
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .child(
                                        div()
                                            .text_size(px(view.suggestion_style.size_sp))
                                            .child(label),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(view.result_supporting_style.size_sp))
                                            .text_color(paint(search::row_supporting(
                                                theme, selected,
                                            )))
                                            .child(supporting),
                                    )
                                    .into_any_element()
                            } else {
                                div().child(label).into_any_element()
                            })
                            .when(open, |el| {
                                el.child(
                                    div()
                                        .w(px(24.))
                                        .h(px(24.))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(paint(view.suggestion_icon))
                                        .child(search::RESULT_OPEN),
                                )
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(picked) = search::pick_suggestion(&q, i) {
                                    this.search.set_value(picked);
                                    this.search.set_focus(false);
                                    cx.notify();
                                }
                            }))
                    }))
                    .into_any_element(),
            );
        }
    }
    let list = div()
        .flex()
        .flex_col()
        .when(open, |el| {
            el.max_h(px(search::docked_list_max_h_dp(search::DEMO_SCREEN_H_DP)))
        })
        .children(list_children)
        .into_any_element();
    let morph = div()
        .id("search-morph")
        .relative()
        .w_full()
        .flex()
        .flex_col()
        .overflow_hidden()
        .bg(paint(contained_bg))
        .tab_index(0)
        .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
            if this.search_open {
                search::apply_key_to_editor(&mut this.search, &ev.keystroke.key);
                cx.notify();
            }
        }))
        .with_animation(
            if open { "search-grow" } else { "search-shrink" },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let linear = if open { delta } else { 1.0 - delta };
                let frame =
                    search::contained_frame_eased_layout(search_layout, linear, suggestion_count);
                this.min_h(px(frame.height_dp))
                    .max_h(px(search::docked_max_h_dp(search::DEMO_SCREEN_H_DP)))
                    .rounded(px(frame.corner_dp))
                    .ml(px(frame.margin_dp))
                    .mr(px(frame.margin_dp))
            },
        )
        .child(header)
        .child(
            div()
                .with_animation(
                    if open {
                        "search-list-in"
                    } else {
                        "search-list-out"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if open { delta } else { 1.0 - delta };
                        this.opacity(search::morph_list_opacity(t))
                    },
                )
                .child(list),
        );
    div()
        .id("search-docked-stage")
        .relative()
        .w_full()
        .min_h(px(if open {
            search::docked_max_h_dp(search::DEMO_SCREEN_H_DP)
        } else {
            search::HEIGHT_DP
        }))
        .when(
            open && search::uses_docked_scrim(search_layout, true),
            |el| {
                el.child(
                    div()
                        .id("search-docked-scrim")
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .size_full()
                        .bg(paint(search::docked_scrim(theme)))
                        .on_click(cx.listener(|this, _, _, cx| {
                            if search::dismiss_on_scrim() {
                                this.search_open = false;
                                this.search.set_focus(false);
                                cx.notify();
                            }
                        })),
                )
            },
        )
        .child(morph)
}

fn time_scroll_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = time_picker::resolve_scroll(theme);
    let input = time_picker::resolve_input(theme);
    let mode = this.time_display;
    let input_mode = mode == time_picker::TimePickerDisplayMode::Input;
    div()
        .w_full()
        .p(px(time_picker::CONTAINER_PAD_DP))
        .rounded(px(a.corners.top_left))
        .bg(paint(a.container))
        .shadow_md()
        .flex()
        .flex_col()
        .gap(px(16.))
        .tab_index(0)
        .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
            if this.time_display == time_picker::TimePickerDisplayMode::Input {
                this.time_input.apply_key(&ev.keystroke.key);
                let (h, m) = this.time_input.commit_or(this.time_hour, this.time_minute);
                this.time_hour = h;
                this.time_minute = m;
                this.sync_period_from_hour();
                cx.notify();
            }
        }))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(spaced_line(
                    time_picker::TITLE,
                    a.title_style.size_sp,
                    paint(a.header),
                ))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.))
                        .child(
                            div()
                                .id("time-format-toggle")
                                .w(px(time_picker::TOGGLE_SIZE_DP))
                                .h(px(time_picker::TOGGLE_SIZE_DP))
                                .rounded(px(time_picker::TOGGLE_SIZE_DP / 2.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(16.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(paint(input.toggle))
                                .child(this.time_format.toggle_text())
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.toggle_time_format();
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .id("scroll-display-mode-toggle")
                                .w(px(time_picker::TOGGLE_SIZE_DP))
                                .h(px(time_picker::TOGGLE_SIZE_DP))
                                .rounded(px(time_picker::TOGGLE_SIZE_DP / 2.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(time_picker::TOGGLE_ICON_DP))
                                .text_color(paint(input.toggle))
                                .child(mode.toggle_icon())
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.toggle_time_display();
                                    cx.notify();
                                })),
                        ),
                ),
        )
        .when(!input_mode, |el| {
            el.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(time_picker::SCROLL_GAP_DP))
                    .child(desktop_scroll_field(
                        this,
                        theme,
                        cx,
                        time_picker::ScrollKind::Hour,
                        &a,
                    ))
                    .child(
                        div()
                            .mt(px(time_picker::SCROLL_COLON_OFFSET_Y_DP))
                            .text_size(px(a.colon_style.size_sp))
                            .font_weight(type_weight(a.colon_style))
                            .text_color(paint(a.colon))
                            .child(":"),
                    )
                    .child(desktop_scroll_field(
                        this,
                        theme,
                        cx,
                        time_picker::ScrollKind::Minute,
                        &a,
                    ))
                    .when(this.time_format.shows_period(), |row| {
                        row.child(desktop_period_column(this, cx, &a))
                    }),
            )
        })
        .when(input_mode, |el| {
            el.child(desktop_time_input(this, cx, &input))
        })
}

fn desktop_period_column(
    this: &CatalogView,
    cx: &mut Context<CatalogView>,
    a: &time_picker::TimeScrollAppearance,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(time_picker::PERIOD_GAP_DP))
        .children([DayPeriod::Am, DayPeriod::Pm].into_iter().map(|period| {
            let selected = this.time_period == period;
            div()
                .id(SharedString::from(format!("scroll-{}", period.label())))
                .w(px(time_picker::PERIOD_W_DP))
                .h(px(time_picker::PERIOD_H_DP))
                .rounded(px(8.))
                .bg(paint(if selected {
                    a.period_selected_container
                } else {
                    a.period_idle_container
                }))
                .text_color(paint(if selected {
                    a.period_selected
                } else {
                    a.period_idle
                }))
                .font_weight(type_weight(a.period_style))
                .flex()
                .items_center()
                .justify_center()
                .child(period.label())
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.time_period = period;
                    cx.notify();
                }))
        }))
}

fn desktop_time_input(
    this: &CatalogView,
    cx: &mut Context<CatalogView>,
    a: &time_picker::TimeInputAppearance,
) -> impl IntoElement {
    let hour_on = this.time_input.focus == time_picker::ScrollKind::Hour;
    div()
        .flex()
        .items_center()
        .gap(px(time_picker::INPUT_COLON_GAP_DP))
        .child(desktop_time_input_field(
            this,
            cx,
            time_picker::ScrollKind::Hour,
            hour_on,
            a,
        ))
        .child(
            div()
                .text_size(px(a.colon_style.size_sp))
                .font_weight(type_weight(a.colon_style))
                .text_color(paint(a.colon))
                .child(":"),
        )
        .child(desktop_time_input_field(
            this,
            cx,
            time_picker::ScrollKind::Minute,
            !hour_on,
            a,
        ))
        .when(this.time_format.shows_period(), |row| {
            row.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(time_picker::PERIOD_GAP_DP))
                    .children([DayPeriod::Am, DayPeriod::Pm].into_iter().map(|period| {
                        let selected = this.time_period == period;
                        div()
                            .id(SharedString::from(format!("input-{}", period.label())))
                            .w(px(a.period_w_dp))
                            .h(px(a.period_h_dp / 2.0 - 4.0))
                            .rounded(px(8.))
                            .bg(paint(if selected {
                                a.period_selected_container
                            } else {
                                a.period_idle_container
                            }))
                            .text_color(paint(if selected {
                                a.period_selected
                            } else {
                                a.period_idle
                            }))
                            .font_weight(type_weight(a.period_style))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(period.label())
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.time_period = period;
                                cx.notify();
                            }))
                    })),
            )
        })
}

fn desktop_time_input_field(
    this: &CatalogView,
    cx: &mut Context<CatalogView>,
    kind: time_picker::ScrollKind,
    focused: bool,
    a: &time_picker::TimeInputAppearance,
) -> impl IntoElement {
    let label = match kind {
        time_picker::ScrollKind::Hour => this.time_input.hour.display(),
        time_picker::ScrollKind::Minute => this.time_input.minute.display(),
    };
    div()
        .id(SharedString::from(format!("time-input-{}", kind.label())))
        .w(px(a.field_w_dp))
        .h(px(a.field_h_dp))
        .rounded(px(a.field_corners.top_left))
        .bg(paint(if focused {
            a.field_focused
        } else {
            a.field_container
        }))
        .text_color(paint(if focused {
            a.field_focused_content
        } else {
            a.field_content
        }))
        .text_size(px(a.field_style.size_sp))
        .font_weight(type_weight(a.field_style))
        .flex()
        .items_center()
        .justify_center()
        .child(label)
        .on_click(cx.listener(move |this, _, _, cx| {
            this.time_input.focus = kind;
            cx.notify();
        }))
}

fn desktop_scroll_field(
    this: &CatalogView,
    _theme: &Theme,
    cx: &mut Context<CatalogView>,
    kind: time_picker::ScrollKind,
    a: &time_picker::TimeScrollAppearance,
) -> impl IntoElement {
    let field = match kind {
        time_picker::ScrollKind::Hour => this.time_scroll.hour,
        time_picker::ScrollKind::Minute => this.time_scroll.minute,
    };
    let slots = field.slots();
    let item_h = a.item_h_dp;
    div()
        .id(SharedString::from(format!("scroll-field-{}", kind.label())))
        .relative()
        .w(px(a.field_w_dp))
        .h(px(a.field_h_dp))
        .rounded(px(a.field_corners.top_left))
        .bg(paint(a.field_container))
        .overflow_hidden()
        .on_scroll_wheel(cx.listener(move |this, ev: &ScrollWheelEvent, _, cx| {
            let dy = match ev.delta {
                ScrollDelta::Pixels(p) => f32::from(p.y),
                ScrollDelta::Lines(p) => p.y * item_h,
            };
            time_picker::apply_wheel(this.time_scroll.field_mut(kind), dy);
            this.time_hour = this.time_scroll.hour_value();
            this.time_minute = this.time_scroll.minute_value();
            this.sync_period_from_hour();
            cx.notify();
        }))
        .children(slots.into_iter().map(|slot| {
            let index = slot.index;
            let selected = slot.selected;
            let style = if selected {
                a.selected_style
            } else {
                a.unselected_style
            };
            let color = if selected { a.selected } else { a.unselected };
            div()
                .id(SharedString::from(format!(
                    "scroll-{}-{}",
                    kind.label(),
                    slot.label
                )))
                .absolute()
                .left(px(0.))
                .top(px(slot.y_dp))
                .w(px(a.field_w_dp))
                .h(px(a.item_h_dp))
                .flex()
                .items_center()
                .justify_center()
                .opacity(slot.opacity)
                .text_size(px(style.size_sp))
                .font_weight(type_weight(style))
                .text_color(paint(color))
                .child(slot.label)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.time_scroll.field_mut(kind).snap_to_index(index);
                    this.time_hour = this.time_scroll.hour_value();
                    this.time_minute = this.time_scroll.minute_value();
                    this.sync_period_from_hour();
                    cx.notify();
                }))
        }))
}

fn time_picker_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = time_picker::resolve(theme);
    let clock = a.clock_dp * 0.75;
    let number = 32.0;
    let hour_on = this.time_dial == DialFace::Hour;
    let labels: Vec<(u8, String, f32, f32, bool)> = match this.time_dial {
        DialFace::Hour => time_picker::hour_cells(this.time_format, clock, number)
            .into_iter()
            .map(|cell| {
                (
                    cell.hour,
                    cell.label,
                    cell.x,
                    cell.y,
                    cell.hour == this.time_dial_hour(),
                )
            })
            .collect(),
        DialFace::Minute => time_picker::minute_labels()
            .map(|m| {
                let (x, y) = time_picker::minute_offset(m, clock, number);
                (m, format!("{m:02}"), x, y, m == this.time_minute)
            })
            .collect(),
    };
    let from_angle = this.time_hand_from;
    let to_angle =
        time_picker::hand_angle_deg(this.time_dial, this.time_dial_hour(), this.time_minute);
    let hand_gen = this.time_hand_gen;
    let hand_ms = time_picker::hand_motion_ms(theme) as u64;
    let hour_live = hour_on;
    let live_hour = this.time_dial_hour();
    let live_minute = this.time_minute;
    let live_format = this.time_format;
    let live_radius =
        time_picker::selector_radius_dp(this.time_dial, live_hour, live_format, clock);
    let hub = (clock / 2.0, clock / 2.0);
    let hand_color = paint(a.hand);
    div()
        .w_full()
        .p(px(time_picker::CONTAINER_PAD_DP))
        .rounded(px(a.corners.top_left))
        .bg(paint(a.container))
        .shadow_md()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(time_picker::HORIZONTAL_GAP_DP))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(16.))
                .child(spaced_line(
                    time_picker::TITLE,
                    a.title_style.size_sp,
                    paint(a.header),
                ))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.))
                        .child(
                            div()
                                .id("time-hour-field")
                                .px(px(8.))
                                .p(px(4.))
                                .rounded(px(8.))
                                .bg(paint(if hour_on {
                                    a.number_selected_container
                                } else {
                                    a.clock
                                }))
                                .text_color(paint(if hour_on {
                                    a.number_selected
                                } else {
                                    a.header
                                }))
                                .font_weight(type_weight(a.time_style))
                                .w(px(time_picker::time_selector_w_dp(this.time_format)))
                                .h(px(time_picker::TIME_SELECTOR_H_DP))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(time_picker::format_hour_field_for(
                                    this.time_dial_hour(),
                                    this.time_format,
                                ))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.bump_time_hand();
                                    this.time_dial = DialFace::Hour;
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .font_weight(type_weight(a.time_style))
                                .text_color(paint(a.header))
                                .child(":"),
                        )
                        .child(
                            div()
                                .id("time-minute-field")
                                .px(px(8.))
                                .p(px(4.))
                                .rounded(px(8.))
                                .bg(paint(if !hour_on {
                                    a.number_selected_container
                                } else {
                                    a.clock
                                }))
                                .text_color(paint(if !hour_on {
                                    a.number_selected
                                } else {
                                    a.header
                                }))
                                .font_weight(type_weight(a.time_style))
                                .w(px(time_picker::time_selector_w_dp(this.time_format)))
                                .h(px(time_picker::TIME_SELECTOR_H_DP))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(time_picker::format_minute_field(this.time_minute))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.bump_time_hand();
                                    this.time_dial = DialFace::Minute;
                                    cx.notify();
                                })),
                        ),
                )
                .when(this.time_format.shows_period(), |col| {
                    col.child(
                        div()
                            .flex()
                            .flex_row()
                            .w(px(time_picker::period_w_dp(
                                time_picker::TimePickerLayoutType::Horizontal,
                            )))
                            .h(px(time_picker::period_h_dp(
                                time_picker::TimePickerLayoutType::Horizontal,
                            )))
                            .children([DayPeriod::Am, DayPeriod::Pm].into_iter().map(|period| {
                                let selected = this.time_period == period;
                                div()
                                    .id(SharedString::from(period.label()))
                                    .flex_1()
                                    .h_full()
                                    .rounded(px(8.))
                                    .bg(paint(if selected {
                                        a.period_selected_container
                                    } else {
                                        a.period_idle_container
                                    }))
                                    .text_color(paint(if selected {
                                        a.period_selected
                                    } else {
                                        a.period_idle
                                    }))
                                    .font_weight(type_weight(a.period_style))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(period.label())
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.time_period = period;
                                        cx.notify();
                                    }))
                            })),
                    )
                }),
        )
        .child(
            div()
                .relative()
                .w(px(clock))
                .h(px(clock))
                .rounded(px(clock / 2.0))
                .bg(paint(a.clock))
                .child(
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(clock))
                        .h(px(clock))
                        .with_animation(
                            SharedString::from(format!(
                                "time-hand-{hand_gen}-{}",
                                if hour_live { "live" } else { "once" }
                            )),
                            if hour_live {
                                Animation::new(Duration::from_millis(hand_ms.saturating_mul(12)))
                                    .repeat()
                            } else {
                                Animation::new(Duration::from_millis(hand_ms))
                            },
                            move |this, delta| {
                                let angle = if hour_live {
                                    time_picker::hour_face_live_angle_deg(
                                        live_hour,
                                        live_minute,
                                        delta,
                                    )
                                } else {
                                    time_picker::lerp_angle_deg(from_angle, to_angle, delta)
                                };
                                let quad = time_picker::hand_quad_at_radius(
                                    clock,
                                    angle,
                                    number,
                                    live_radius,
                                );
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let mut builder = PathBuilder::fill();
                                            for (i, (x, y)) in quad.iter().enumerate() {
                                                let p = point(
                                                    bounds.origin.x + px(*x),
                                                    bounds.origin.y + px(*y),
                                                );
                                                if i == 0 {
                                                    builder.move_to(p);
                                                } else {
                                                    builder.line_to(p);
                                                }
                                            }
                                            builder.close();
                                            if let Ok(path) = builder.build() {
                                                window.paint_path(path, hand_color);
                                            }
                                            let mut hub_b = PathBuilder::fill();
                                            let r = time_picker::HAND_HUB_DP / 2.0;
                                            let n = 12u32;
                                            for i in 0..n {
                                                let ang =
                                                    i as f32 / n as f32 * std::f32::consts::TAU;
                                                let p = point(
                                                    bounds.origin.x + px(hub.0 + r * ang.cos()),
                                                    bounds.origin.y + px(hub.1 + r * ang.sin()),
                                                );
                                                if i == 0 {
                                                    hub_b.move_to(p);
                                                } else {
                                                    hub_b.line_to(p);
                                                }
                                            }
                                            hub_b.close();
                                            if let Ok(path) = hub_b.build() {
                                                window.paint_path(path, hand_color);
                                            }
                                        },
                                    )
                                    .w(px(clock))
                                    .h(px(clock)),
                                )
                            },
                        ),
                )
                .child({
                    let second_color = hand_color;
                    let period = time_picker::SECOND_HAND_FRAME_MS as u64;
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(clock))
                        .h(px(clock))
                        .with_animation(
                            "time-second-hand",
                            Animation::new(Duration::from_millis(period)).repeat(),
                            move |this, _delta| {
                                let angle = time_picker::second_hand_angle_wall_clock();
                                let quad = time_picker::second_hand_quad(clock, angle, number);
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let mut builder = PathBuilder::fill();
                                            for (i, (x, y)) in quad.iter().enumerate() {
                                                let p = point(
                                                    bounds.origin.x + px(*x),
                                                    bounds.origin.y + px(*y),
                                                );
                                                if i == 0 {
                                                    builder.move_to(p);
                                                } else {
                                                    builder.line_to(p);
                                                }
                                            }
                                            builder.close();
                                            if let Ok(path) = builder.build() {
                                                window.paint_path(path, second_color);
                                            }
                                        },
                                    )
                                    .w(px(clock))
                                    .h(px(clock)),
                                )
                            },
                        )
                })
                .children(labels.into_iter().map(|(value, label, x, y, selected)| {
                    let face = this.time_dial;
                    div()
                        .id(SharedString::from(format!("dial-{value}")))
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(number))
                        .h(px(number))
                        .rounded(px(number / 2.0))
                        .bg(paint(if selected {
                            a.number_selected_container
                        } else {
                            a.clock
                        }))
                        .text_color(paint(if selected {
                            a.number_selected
                        } else {
                            a.number
                        }))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(label)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.bump_time_hand();
                            match face {
                                DialFace::Hour => {
                                    this.time_hour = time_picker::hour_from_dial(
                                        time_picker::select_hour_for(
                                            this.time_dial_hour(),
                                            value,
                                            this.time_format,
                                        ),
                                        this.time_period,
                                        this.time_format,
                                    );
                                    this.time_dial = DialFace::Minute;
                                }
                                DialFace::Minute => {
                                    this.time_minute =
                                        time_picker::select_minute(this.time_minute, value);
                                }
                            }
                            cx.notify();
                        }))
                })),
        )
}

fn wide_rail_icon_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let collapsed = navigation_rail::resolve_wide_collapsed(theme);
    let expanded = navigation_rail::resolve_mode(theme, navigation_rail::RailMode::Expanded);
    div()
        .id("wide-rail-pair")
        .flex()
        .flex_row()
        .gap(px(24.))
        .child(nav_rail_static_column(
            this,
            theme,
            collapsed.width_dp,
            navigation_rail::IconPosition::Top,
            cx,
        ))
        .child(nav_rail_static_column(
            this,
            theme,
            expanded.width_dp,
            navigation_rail::IconPosition::Start,
            cx,
        ))
}

fn nav_rail_static_column(
    this: &CatalogView,
    theme: &Theme,
    width_dp: f32,
    position: navigation_rail::IconPosition,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let rail = navigation_rail::resolve(theme);
    let selected = this.rail_selected;
    div()
        .id(SharedString::from(format!(
            "wide-rail-{}",
            position.label()
        )))
        .w(px(width_dp))
        .min_h(px(280.))
        .pt(px(navigation_rail::content_padding().top_dp))
        .pb(px(navigation_rail::content_padding().bottom_dp))
        .bg(paint(rail.container))
        .flex()
        .flex_col()
        .when(position.is_start(), |el| el.items_stretch())
        .when(!position.is_start(), |el| el.items_center())
        .gap(px(navigation_rail::DEST_GAP_DP))
        .children(nav_rail_dest_views(
            theme,
            &rail,
            width_dp,
            position,
            selected,
            false,
            false,
            "wide-rail",
            navigation_rail::RailCollapsedKind::Wide,
            cx,
        ))
}

fn wide_rail_in_flow_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let body_fg = paint(theme.color.on_surface);
    div()
        .id("wide-rail-inflow")
        .flex()
        .flex_row()
        .w_full()
        .min_h(px(280.))
        .overflow_hidden()
        .child(nav_rail_column(
            this,
            theme,
            this.wide_rail_mode,
            navigation_rail::RailExpandedLayout::Standard,
            navigation_rail::RailCollapsedKind::Wide,
            cx,
        ))
        .child(
            div()
                .id("wide-rail-inflow-body")
                .flex_1()
                .p(px(16.))
                .text_color(body_fg)
                .child(navigation_rail::IN_FLOW_BODY),
        )
}

fn nav_rail_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let expanded = this.rail_mode == navigation_rail::RailMode::Expanded;
    let scrim_c = paint(navigation_rail::scrim(theme));
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    div()
        .id("nav-rail-stage")
        .relative()
        .w_full()
        .min_h(px(280.))
        .overflow_hidden()
        .child(
            div()
                .id("rail-scrim")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .bg(scrim_c)
                .when(expanded, |el| {
                    el.on_click(cx.listener(|this, _, _, cx| {
                        this.rail_mode = navigation_rail::RailMode::Collapsed;
                        cx.notify();
                    }))
                })
                .with_animation(
                    if expanded {
                        "rail-scrim-in"
                    } else {
                        "rail-scrim-out"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if expanded { delta } else { 1.0 - delta };
                        this.opacity(
                            navigation_rail::scrim_opacity_for(
                                navigation_rail::RailExpandedLayout::Modal,
                                t,
                            ) / navigation_rail::SCRIM_OPACITY,
                        )
                    },
                ),
        )
        .child(
            div()
                .id("nav-rail-window")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .h_full()
                .when(expanded, |el| el.shadow_lg())
                .child(nav_rail_column(
                    this,
                    theme,
                    this.rail_mode,
                    navigation_rail::RailExpandedLayout::Modal,
                    navigation_rail::RailCollapsedKind::Wide,
                    cx,
                )),
        )
}

fn narrow_rail_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let expanded = this.narrow_rail_mode == navigation_rail::RailMode::Expanded;
    let scrim_c = paint(navigation_rail::scrim(theme));
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    div()
        .id("narrow-rail-stage")
        .relative()
        .w_full()
        .min_h(px(280.))
        .overflow_hidden()
        .child(
            div()
                .id("narrow-rail-scrim")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .bg(scrim_c)
                .when(expanded, |el| {
                    el.on_click(cx.listener(|this, _, _, cx| {
                        this.narrow_rail_mode = navigation_rail::RailMode::Collapsed;
                        cx.notify();
                    }))
                })
                .with_animation(
                    if expanded {
                        "narrow-rail-scrim-in"
                    } else {
                        "narrow-rail-scrim-out"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if expanded { delta } else { 1.0 - delta };
                        this.opacity(
                            navigation_rail::scrim_opacity_for(
                                navigation_rail::RailExpandedLayout::Modal,
                                t,
                            ) / navigation_rail::SCRIM_OPACITY,
                        )
                    },
                ),
        )
        .child(
            div()
                .id("narrow-rail-window")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .h_full()
                .when(expanded, |el| el.shadow_lg())
                .child(nav_rail_column(
                    this,
                    theme,
                    this.narrow_rail_mode,
                    navigation_rail::RailExpandedLayout::Modal,
                    navigation_rail::RailCollapsedKind::Narrow,
                    cx,
                )),
        )
}

fn hide_rail_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let expanded = this.hide_rail_mode == navigation_rail::RailMode::Expanded;
    let scrim_c = paint(navigation_rail::scrim(theme));
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    let rail = navigation_rail::resolve_mode(theme, navigation_rail::RailMode::Expanded);
    let body_fg = paint(theme.color.on_surface);
    let theme_anim = *theme;
    let width_dp = navigation_rail::EXPANDED_WIDTH_DP;
    div()
        .id("wide-rail-hide")
        .relative()
        .w_full()
        .min_h(px(280.))
        .overflow_hidden()
        .child(
            div()
                .id("hide-rail-scrim")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .bg(scrim_c)
                .when(expanded, |el| {
                    el.on_click(cx.listener(|this, _, _, cx| {
                        this.hide_rail_mode = navigation_rail::RailMode::Collapsed;
                        cx.notify();
                    }))
                })
                .with_animation(
                    if expanded {
                        "hide-rail-scrim-in"
                    } else {
                        "hide-rail-scrim-out"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if expanded { delta } else { 1.0 - delta };
                        this.opacity(
                            navigation_rail::scrim_opacity_for(
                                navigation_rail::HIDE_DEMO_LAYOUT,
                                t,
                            ) / navigation_rail::SCRIM_OPACITY,
                        )
                    },
                ),
        )
        .child(
            div()
                .id("wide-rail-hide-body")
                .flex()
                .flex_row()
                .items_start()
                .child(
                    div()
                        .id("hide-rail-menu")
                        .w(px(navigation_rail::FAB_SLOT_DP))
                        .h(px(navigation_rail::FAB_SLOT_DP))
                        .ml(px(16.))
                        .rounded(px(16.))
                        .bg(paint(rail.fab))
                        .text_color(paint(rail.fab_icon))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(navigation_rail::HIDE_MENU_GLYPH)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.hide_rail_mode = navigation_rail::toggle_mode(this.hide_rail_mode);
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .id("wide-rail-hide-inbox")
                        .flex_1()
                        .p(px(16.))
                        .text_color(body_fg)
                        .child(navigation_rail::IN_FLOW_BODY),
                ),
        )
        .child(
            div()
                .id("hide-rail-window")
                .absolute()
                .top(px(0.))
                .h_full()
                .w(px(width_dp))
                .when(expanded, |el| el.shadow_lg())
                .with_animation(
                    if expanded {
                        "hide-rail-slide-in"
                    } else {
                        "hide-rail-slide-out"
                    },
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if expanded { delta } else { 1.0 - delta };
                        this.left(px(navigation_rail::hide_slide_offset_eased(&theme_anim, t)))
                    },
                )
                .child(hide_rail_column(this, theme, expanded, cx)),
        )
}

fn rail_extended_fab(
    theme: &Theme,
    rail: &navigation_rail::NavRailAppearance,
    expanded: bool,
    collapsed: navigation_rail::RailCollapsedKind,
    id: &'static str,
    label_id: &'static str,
    anim_expand: &'static str,
    anim_collapse: &'static str,
    on_toggle: impl Fn(&mut CatalogView) + 'static,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    let theme_a = *theme;
    let label = theme.typography.label_large;
    div()
        .id(id)
        .h(px(navigation_rail::FAB_SLOT_DP))
        .rounded(px(navigation_rail::FAB_CORNER_DP))
        .bg(paint(rail.fab))
        .text_color(paint(rail.fab_icon))
        .flex()
        .flex_row()
        .items_center()
        .overflow_hidden()
        .px(px(navigation_rail::FAB_PAD_EXPANDED_DP))
        .gap(px(if expanded {
            navigation_rail::FAB_ICON_LABEL_GAP_DP
        } else {
            0.
        }))
        .on_click(cx.listener(move |this, _, _, cx| {
            on_toggle(this);
            cx.notify();
        }))
        .with_animation(
            if expanded { anim_expand } else { anim_collapse },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let t = if expanded { delta } else { 1.0 - delta };
                let rail_w = navigation_rail::morph_width_eased_kind(&theme_a, collapsed, t);
                let morph = navigation_rail::fab_morph_kind(&theme_a, t, rail_w, collapsed);
                this.w(px(morph.width_dp)).ml(px(morph.margin_start_dp))
            },
        )
        .child(navigation_rail::FAB_GLYPH)
        .child(
            div()
                .id(label_id)
                .text_size(px(label.size_sp))
                .child(navigation_rail::FAB_LABEL),
        )
}

fn header_extended_fab(
    theme: &Theme,
    rail: &navigation_rail::NavRailAppearance,
    expanded: bool,
) -> impl IntoElement {
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    let theme_a = *theme;
    let label = theme.typography.label_large;
    div()
        .id("header-rail-fab")
        .h(px(navigation_rail::FAB_SLOT_DP))
        .rounded(px(navigation_rail::FAB_CORNER_DP))
        .bg(paint(rail.fab))
        .text_color(paint(rail.fab_icon))
        .flex()
        .flex_row()
        .items_center()
        .overflow_hidden()
        .px(px(navigation_rail::FAB_PAD_EXPANDED_DP))
        .gap(px(if expanded {
            navigation_rail::FAB_ICON_LABEL_GAP_DP
        } else {
            0.
        }))
        .with_animation(
            if expanded {
                "header-fab-expand"
            } else {
                "header-fab-collapse"
            },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let t = if expanded { delta } else { 1.0 - delta };
                let rail_w = navigation_rail::morph_width_eased_kind(
                    &theme_a,
                    navigation_rail::RailCollapsedKind::Wide,
                    t,
                );
                let morph = navigation_rail::fab_morph(&theme_a, t, rail_w);
                this.w(px(morph.width_dp)).ml(px(morph.margin_start_dp))
            },
        )
        .child(navigation_rail::FAB_GLYPH)
        .child(
            div()
                .id("header-rail-fab-label")
                .text_size(px(label.size_sp))
                .child(navigation_rail::FAB_LABEL),
        )
}

fn header_rail_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let body_fg = paint(theme.color.on_surface);
    div()
        .id("wide-rail-header")
        .flex()
        .flex_row()
        .w_full()
        .min_h(px(280.))
        .overflow_hidden()
        .child(header_rail_column(this, theme, cx))
        .child(
            div()
                .id("wide-rail-header-body")
                .flex_1()
                .p(px(16.))
                .text_color(body_fg)
                .child(navigation_rail::IN_FLOW_BODY),
        )
}

fn header_rail_column(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let mode = this.header_rail_mode;
    let expanded = mode == navigation_rail::RailMode::Expanded;
    let rail =
        navigation_rail::resolve_mode_kind(theme, mode, navigation_rail::RailCollapsedKind::Wide);
    let selected = this.rail_selected;
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    let position = navigation_rail::icon_position_for_mode(mode);
    let arrangement = navigation_rail::HEADER_DEMO_ARRANGEMENT;
    let header_btn = icon_button::resolve(
        theme,
        icon_button::IconButtonVariant::Standard,
        InteractionState::Enabled,
    );
    let tip = tooltip::resolve_plain(theme);
    let theme_w = *theme;
    let tip_open = this.header_tooltip_open;
    div()
        .id("header-rail")
        .relative()
        .h_full()
        .min_h(px(280.))
        .overflow_hidden()
        .pt(px(navigation_rail::content_padding().top_dp))
        .pb(px(navigation_rail::content_padding().bottom_dp))
        .bg(paint(rail.container))
        .flex()
        .flex_col()
        .items_stretch()
        .with_animation(
            if expanded {
                "header-rail-expand"
            } else {
                "header-rail-collapse"
            },
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let t = if expanded { delta } else { 1.0 - delta };
                this.w(px(navigation_rail::morph_width_eased_kind(
                    &theme_w,
                    navigation_rail::RailCollapsedKind::Wide,
                    t,
                )))
            },
        )
        .child(
            div()
                .id("header-rail-slot")
                .relative()
                .flex()
                .flex_col()
                .items_stretch()
                .gap(px(navigation_rail::HEADER_FAB_GAP_DP))
                .child(
                    div()
                        .id("header-rail-menu-row")
                        .relative()
                        .flex()
                        .flex_col()
                        .items_start()
                        .pl(px(navigation_rail::HEADER_PAD_START_DP))
                        .when(tip_open, |el| {
                            el.child(
                                div()
                                    .id("header-rail-tooltip")
                                    .absolute()
                                    .bottom(px(
                                        navigation_rail::HEADER_BUTTON_DP + tooltip::ANCHOR_GAP_DP
                                    ))
                                    .left(px(navigation_rail::HEADER_PAD_START_DP))
                                    .h(px(tip.min_height_dp))
                                    .px(px(tip.pad_start_dp))
                                    .py(px(tip.pad_top_dp))
                                    .rounded(px(tip.corners.top_left))
                                    .bg(paint(tip.container))
                                    .text_color(paint(tip.supporting))
                                    .text_size(px(tip.supporting_style.size_sp))
                                    .flex()
                                    .items_center()
                                    .child(navigation_rail::header_menu_label(expanded)),
                            )
                        })
                        .child(
                            div()
                                .id("header-rail-menu")
                                .w(px(navigation_rail::HEADER_BUTTON_DP))
                                .h(px(navigation_rail::HEADER_BUTTON_DP))
                                .rounded(px(header_btn.corners.top_left))
                                .text_color(paint(header_btn.content))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(navigation_rail::header_menu_glyph(expanded))
                                .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                                    this.header_tooltip_open = *hovered;
                                    cx.notify();
                                }))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.header_rail_mode =
                                        navigation_rail::toggle_mode(this.header_rail_mode);
                                    cx.notify();
                                })),
                        ),
                )
                .child(header_extended_fab(theme, &rail, expanded)),
        )
        .child(
            div()
                .id("header-rail-dests")
                .flex()
                .flex_col()
                .flex_1()
                .items_stretch()
                .gap(px(navigation_rail::DEST_GAP_DP))
                .when(arrangement.is_bottom(), |el| el.justify_end())
                .children(nav_rail_dest_views(
                    theme,
                    &rail,
                    rail.width_dp,
                    position,
                    selected,
                    true,
                    expanded,
                    "header-rail",
                    navigation_rail::RailCollapsedKind::Wide,
                    cx,
                )),
        )
}

fn hide_extended_fab(
    theme: &Theme,
    rail: &navigation_rail::NavRailAppearance,
    rail_width_dp: f32,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let morph = navigation_rail::fab_morph_hide(theme, rail_width_dp);
    let label = theme.typography.label_large;
    div()
        .id("hide-rail-fab")
        .w(px(morph.width_dp))
        .h(px(morph.height_dp))
        .ml(px(morph.margin_start_dp))
        .rounded(px(morph.radius_dp))
        .bg(paint(rail.fab))
        .text_color(paint(rail.fab_icon))
        .flex()
        .flex_row()
        .items_center()
        .overflow_hidden()
        .px(px(morph.pad_h_dp))
        .gap(px(morph.gap_dp))
        .on_click(cx.listener(|this, _, _, cx| {
            this.hide_rail_mode = navigation_rail::toggle_mode(this.hide_rail_mode);
            cx.notify();
        }))
        .child(morph.glyph)
        .child(
            div()
                .id("hide-rail-fab-label")
                .text_size(px(label.size_sp))
                .child(morph.label),
        )
}

fn hide_rail_column(
    this: &CatalogView,
    theme: &Theme,
    expanded: bool,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let mut rail = navigation_rail::resolve_mode(theme, navigation_rail::RailMode::Expanded);
    rail.container = navigation_rail::hide_container_color(theme);
    let selected = this.rail_selected;
    let width_dp = rail.width_dp;
    let arrangement = navigation_rail::HIDE_DEMO_ARRANGEMENT;
    div()
        .id("hide-rail")
        .relative()
        .w(px(width_dp))
        .h_full()
        .overflow_hidden()
        .pt(px(navigation_rail::content_padding().top_dp))
        .pb(px(navigation_rail::content_padding().bottom_dp))
        .bg(paint(rail.container))
        .rounded(px(navigation_rail::hide_shape_dp()))
        .flex()
        .flex_col()
        .items_stretch()
        .child(hide_extended_fab(theme, &rail, width_dp, cx))
        .child(
            div()
                .id("hide-rail-dests")
                .when(arrangement.is_center(), |el| {
                    el.absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .size_full()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .items_stretch()
                        .gap(px(navigation_rail::DEST_GAP_DP))
                })
                .when(!arrangement.is_center(), |el| {
                    el.flex()
                        .flex_col()
                        .items_stretch()
                        .gap(px(navigation_rail::DEST_GAP_DP))
                })
                .children(nav_rail_dest_views(
                    theme,
                    &rail,
                    width_dp,
                    navigation_rail::IconPosition::Start,
                    selected,
                    false,
                    expanded,
                    "hide-rail",
                    navigation_rail::RailCollapsedKind::Wide,
                    cx,
                )),
        )
}

fn nav_rail_column(
    this: &CatalogView,
    theme: &Theme,
    mode: navigation_rail::RailMode,
    layout: navigation_rail::RailExpandedLayout,
    collapsed: navigation_rail::RailCollapsedKind,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let rail = navigation_rail::resolve_layout(theme, mode, collapsed, layout);
    let expanded = mode == navigation_rail::RailMode::Expanded;
    let selected = this.rail_selected;
    let morph_ms = navigation_rail::morph_ms(theme) as u64;
    let position = navigation_rail::icon_position_for_mode(mode);
    let theme = *theme;
    let in_flow = layout.in_flow();
    let narrow = collapsed.is_narrow();
    let id = if in_flow {
        "wide-inflow-rail"
    } else if narrow {
        "narrow-rail"
    } else {
        "nav-rail"
    };
    let fab_id = if in_flow {
        "wide-inflow-fab"
    } else if narrow {
        "narrow-rail-fab"
    } else {
        "rail-fab"
    };
    let fab_label_id = if in_flow {
        "wide-inflow-fab-label"
    } else if narrow {
        "narrow-rail-fab-label"
    } else {
        "rail-fab-label"
    };
    let fab_anim_expand = if in_flow {
        "wide-inflow-fab-expand"
    } else if narrow {
        "narrow-rail-fab-expand"
    } else {
        "rail-fab-expand"
    };
    let fab_anim_collapse = if in_flow {
        "wide-inflow-fab-collapse"
    } else if narrow {
        "narrow-rail-fab-collapse"
    } else {
        "rail-fab-collapse"
    };
    let anim = if in_flow {
        if expanded {
            "wide-inflow-expand"
        } else {
            "wide-inflow-collapse"
        }
    } else if narrow {
        if expanded {
            "narrow-rail-expand"
        } else {
            "narrow-rail-collapse"
        }
    } else if expanded {
        "rail-expand"
    } else {
        "rail-collapse"
    };
    let dest_prefix = if in_flow {
        "wide-inflow"
    } else if narrow {
        "narrow-rail"
    } else {
        "rail"
    };
    div()
        .id(SharedString::from(id))
        .overflow_hidden()
        .pt(px(navigation_rail::content_padding().top_dp))
        .pb(px(navigation_rail::content_padding().bottom_dp))
        .bg(paint(rail.container))
        .rounded(px(navigation_rail::shape_dp_for(layout, expanded)))
        .flex()
        .flex_col()
        .items_stretch()
        .gap(px(navigation_rail::DEST_GAP_DP))
        .with_animation(
            anim,
            Animation::new(Duration::from_millis(morph_ms)),
            move |this, delta| {
                let t = if expanded { delta } else { 1.0 - delta };
                let morph = navigation_rail::container_morph(&theme, layout, t);
                this.w(px(navigation_rail::morph_width_eased_kind(
                    &theme, collapsed, t,
                )))
                .rounded(px(morph.corner_dp))
                .bg(paint(morph.color))
            },
        )
        .child(rail_extended_fab(
            &theme,
            &rail,
            expanded,
            collapsed,
            fab_id,
            fab_label_id,
            fab_anim_expand,
            fab_anim_collapse,
            move |this| {
                if in_flow {
                    this.wide_rail_mode = navigation_rail::toggle_mode(this.wide_rail_mode);
                } else if narrow {
                    this.narrow_rail_mode = navigation_rail::toggle_mode(this.narrow_rail_mode);
                } else {
                    this.rail_mode = navigation_rail::toggle_mode(this.rail_mode);
                }
            },
            cx,
        ))
        .children(nav_rail_dest_views(
            &theme,
            &rail,
            rail.width_dp,
            position,
            selected,
            true,
            expanded,
            dest_prefix,
            collapsed,
            cx,
        ))
}

fn rail_dest_indicator_bg(
    rail: &navigation_rail::NavRailAppearance,
    active: bool,
    alpha: f32,
) -> gpui_material::Argb {
    if active {
        rail.active_indicator
            .with_alpha(alpha)
            .composite_over(rail.container)
    } else {
        rail.container
    }
}

fn rail_dest_layers(
    m: navigation_rail::RailItemMorph,
    rail: &navigation_rail::NavRailAppearance,
    active: bool,
    icon: &'static str,
    label: &'static str,
    badge: Option<u32>,
) -> Vec<gpui::AnyElement> {
    let mut kids = vec![
        div()
            .absolute()
            .left(px(m.icon_left_dp))
            .top(px(m.icon_top_dp))
            .w(px(m.icon_box_w_dp))
            .h(px(m.icon_box_h_dp))
            .rounded(px(m.icon_box_h_dp / 2.0))
            .bg(paint(rail_dest_indicator_bg(
                rail,
                active,
                m.icon_indicator_alpha,
            )))
            .text_color(paint(if active {
                rail.active_icon
            } else {
                rail.inactive_icon
            }))
            .flex()
            .items_center()
            .justify_center()
            .child(icon)
            .into_any_element(),
        div()
            .absolute()
            .left(px(m.label_left_dp))
            .top(px(m.label_top_dp))
            .w(px(m.label_width_dp))
            .h(px(m.label_line_sp))
            .text_size(px(m.label_size_sp))
            .text_color(paint(if active {
                rail.active_label
            } else {
                rail.inactive_label
            }))
            .when(m.label_center, |el| el.flex().justify_center())
            .child(label)
            .into_any_element(),
    ];
    match badge {
        Some(0) => kids.push(
            div()
                .absolute()
                .top(px(2.))
                .right(px(m.badge_right_dp))
                .w(px(6.))
                .h(px(6.))
                .rounded(px(3.))
                .bg(paint(rail.badge))
                .into_any_element(),
        ),
        Some(n) => kids.push(
            div()
                .absolute()
                .top(px(2.))
                .right(px(m.badge_right_dp))
                .min_w(px(16.))
                .h(px(16.))
                .px(px(4.))
                .rounded(px(8.))
                .bg(paint(rail.badge))
                .text_color(paint(rail.badge_label))
                .text_size(px(10.))
                .flex()
                .items_center()
                .justify_center()
                .child(badge::label_for_count(n))
                .into_any_element(),
        ),
        None => {}
    }
    kids
}

fn nav_rail_dest_views(
    theme: &Theme,
    rail: &navigation_rail::NavRailAppearance,
    width_dp: f32,
    position: navigation_rail::IconPosition,
    selected: usize,
    animate: bool,
    expanded: bool,
    id_prefix: &'static str,
    collapsed: navigation_rail::RailCollapsedKind,
    cx: &mut Context<CatalogView>,
) -> Vec<gpui::AnyElement> {
    let theme = *theme;
    let rail = *rail;
    let morph_ms = navigation_rail::morph_ms(&theme) as u64;
    let settled_t = navigation_rail::icon_position_t(position.is_start());
    navigation_rail::DESTINATIONS
        .iter()
        .zip(navigation_rail::DESTINATION_ICONS.iter())
        .zip(navigation_rail::DESTINATION_BADGES.iter())
        .enumerate()
        .map(|(i, ((label, icon), badge))| {
            let active = navigation_rail::is_active(selected, i);
            let dest = div()
                .id(SharedString::from(format!("{id_prefix}-dest-{i}")))
                .relative()
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.rail_selected = navigation_rail::select_destination(this.rail_selected, i);
                    cx.notify();
                }));
            if animate {
                dest.with_animation(
                    SharedString::from(format!(
                        "{id_prefix}-icon-{}-{i}",
                        if expanded { "in" } else { "out" }
                    )),
                    Animation::new(Duration::from_millis(morph_ms)),
                    move |this, delta| {
                        let t = if expanded { delta } else { 1.0 - delta };
                        let rail_w = navigation_rail::morph_width_eased_kind(&theme, collapsed, t);
                        let m = navigation_rail::item_morph(&theme, t, rail_w);
                        this.w(px(m.dest_width_dp))
                            .h(px(m.dest_height_dp))
                            .ml(px(m.dest_ml_dp))
                            .rounded(px(m.dest_radius_dp))
                            .bg(paint(rail_dest_indicator_bg(
                                &rail,
                                active,
                                m.dest_indicator_alpha,
                            )))
                            .children(rail_dest_layers(m, &rail, active, *icon, *label, *badge))
                    },
                )
                .into_any_element()
            } else {
                let m = navigation_rail::item_morph(&theme, settled_t, width_dp);
                dest.w(px(m.dest_width_dp))
                    .h(px(m.dest_height_dp))
                    .ml(px(m.dest_ml_dp))
                    .rounded(px(m.dest_radius_dp))
                    .bg(paint(rail_dest_indicator_bg(
                        &rail,
                        active,
                        m.dest_indicator_alpha,
                    )))
                    .children(rail_dest_layers(m, &rail, active, *icon, *label, *badge))
                    .into_any_element()
            }
        })
        .collect()
}

fn progress_heroes(theme: &Theme) -> impl IntoElement {
    let lin = progress::linear(theme, 0.6);
    let indet = progress::linear_indeterminate(theme);
    let wave = progress::wavy(theme, progress::WAVE_DEMO_PROGRESS);
    let span = indet.head_span;
    let ind_color = paint(indet.indicator);
    let wave_color = paint(wave.indicator);
    let wave_w = wave.width_dp;
    let wave_h = wave.height_dp;
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            div()
                .w(px(240.))
                .h(px(lin.height_dp))
                .rounded(px(2.))
                .bg(paint(lin.track))
                .child(
                    div()
                        .h_full()
                        .w(px(240. * lin.progress))
                        .bg(paint(lin.indicator)),
                ),
        )
        .child(
            div()
                .w(px(wave_w))
                .h(px(wave_h))
                .bg(paint(wave.track))
                .with_animation(
                    "wavy-progress",
                    Animation::new(Duration::from_millis(wave.duration_ms as u64)).repeat(),
                    move |this, delta| {
                        this.child(
                            canvas(
                                move |_, _, _| {},
                                move |bounds, _, window, _| {
                                    let pts = progress::wave_polyline(
                                        wave_w,
                                        wave_h,
                                        progress::WAVE_DEMO_PROGRESS,
                                        delta,
                                    );
                                    let mut builder = stroke_round(progress::WAVE_STROKE_DP);
                                    for (i, (x, y)) in pts.iter().enumerate() {
                                        let p = point(
                                            bounds.origin.x + px(*x),
                                            bounds.origin.y + px(*y),
                                        );
                                        if i == 0 {
                                            builder.move_to(p);
                                        } else {
                                            builder.line_to(p);
                                        }
                                    }
                                    if let Ok(path) = builder.build() {
                                        window.paint_path(path, wave_color);
                                    }
                                },
                            )
                            .w(px(wave_w))
                            .h(px(wave_h)),
                        )
                    },
                ),
        )
        .child(
            div()
                .relative()
                .w(px(240.))
                .h(px(indet.height_dp))
                .rounded(px(2.))
                .bg(paint(indet.track))
                .with_animation(
                    "indet-head",
                    Animation::new(Duration::from_millis(indet.duration_ms as u64)).repeat(),
                    move |this, delta| {
                        this.child(
                            div()
                                .absolute()
                                .left(px(-80.0 + delta * 320.0))
                                .h_full()
                                .w(px(240.0 * span))
                                .bg(ind_color),
                        )
                    },
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(8.))
                .child({
                    let load = progress::contained_loading_indicator(theme);
                    let box_s = load.contained_dp;
                    let shape_s = load.size_dp;
                    let ptr_color = paint(load.indicator);
                    let box_bg = paint(load.container);
                    let dur = load.duration_ms as u64;
                    div()
                        .w(px(box_s))
                        .h(px(box_s))
                        .rounded(px(box_s / 2.0))
                        .bg(box_bg)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(div().w(px(shape_s)).h(px(shape_s)).with_animation(
                            "ptr-morph",
                            Animation::new(Duration::from_millis(dur)).repeat(),
                            move |this, delta| {
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let pts = progress::loading_polygon(shape_s, delta);
                                            paint_filled_polygon(
                                                window,
                                                bounds.origin,
                                                &pts,
                                                ptr_color,
                                            );
                                        },
                                    )
                                    .w(px(shape_s))
                                    .h(px(shape_s)),
                                )
                            },
                        ))
                })
                .child(spaced_line(
                    progress::PTR_LABEL,
                    12.0,
                    paint(theme.color.on_surface_variant),
                )),
        )
        .child({
            let load = progress::loading_indicator(theme);
            let size = load.size_dp;
            let load_color = paint(load.indicator);
            let dur = load.duration_ms as u64;
            let circ = progress::circular_indeterminate(theme);
            let cap_size = circ.size_dp;
            let cap_stroke = circ.stroke_dp;
            let cap_arc = circ.arc_deg;
            let cap_color = paint(circ.indicator);
            let cap_dur = circ.duration_ms as u64;
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(8.))
                        .child(div().w(px(size)).h(px(size)).with_animation(
                            "loading-morph",
                            Animation::new(Duration::from_millis(dur)).repeat(),
                            move |this, delta| {
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let pts = progress::loading_polygon(size, delta);
                                            paint_filled_polygon(
                                                window,
                                                bounds.origin,
                                                &pts,
                                                load_color,
                                            );
                                        },
                                    )
                                    .w(px(size))
                                    .h(px(size)),
                                )
                            },
                        ))
                        .child(spaced_line(
                            progress::LOADING_LABEL,
                            12.0,
                            paint(theme.color.on_surface_variant),
                        )),
                )
                .child({
                    let det_size = load.size_dp;
                    let det_color = paint(load.indicator);
                    let wait_ms = progress::determinate_wait_ms(theme) as u64;
                    let wait_label = paint(theme.color.on_surface_variant);
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(8.))
                        .child(div().w(px(det_size)).h(px(det_size)).with_animation(
                            "loading-det-wait",
                            Animation::new(Duration::from_millis(wait_ms)).repeat(),
                            move |this, delta| {
                                let wait = progress::WaitProgress::from_fraction(delta);
                                let pts = progress::loading_polygon_for_wait(det_size, wait);
                                this.child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            paint_filled_polygon(
                                                window,
                                                bounds.origin,
                                                &pts,
                                                det_color,
                                            );
                                        },
                                    )
                                    .w(px(det_size))
                                    .h(px(det_size)),
                                )
                            },
                        ))
                        .child(div().w(px(det_size)).with_animation(
                            "loading-det-label",
                            Animation::new(Duration::from_millis(wait_ms)).repeat(),
                            move |this, delta| {
                                this.child(spaced_line(
                                    format!("{:.0}%", delta * 100.0),
                                    12.0,
                                    wait_label,
                                ))
                            },
                        ))
                })
                .child(div().w(px(cap_size)).h(px(cap_size)).with_animation(
                    "circ-indet-cap",
                    Animation::new(Duration::from_millis(cap_dur)).repeat(),
                    move |this, delta| {
                        this.child(
                            canvas(
                                move |_, _, _| {},
                                move |bounds, _, window, _| {
                                    let pts = progress::ptr_arc_polyline(
                                        cap_size, cap_stroke, cap_arc, delta,
                                    );
                                    paint_round_polyline(
                                        window,
                                        bounds.origin,
                                        &pts,
                                        cap_stroke,
                                        cap_color,
                                    );
                                },
                            )
                            .w(px(cap_size))
                            .h(px(cap_size)),
                        )
                    },
                ))
        })
}

fn carousel_hero(
    this: &CatalogView,
    theme: &Theme,
    cx: &mut Context<CatalogView>,
) -> impl IntoElement {
    let a = carousel::resolve(theme);
    let selected = this.carousel_index;
    let layout = this.carousel_layout;
    let offset_t = this.carousel_fling.snap_offset_t();
    let shift = carousel::parallax_offset_dp(offset_t);
    div()
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            div()
                .id("carousel-layout")
                .text_size(px(12.))
                .text_color(paint(theme.color.on_surface_variant))
                .child(format!("layout · {}", layout.label()))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.carousel_layout = this.carousel_layout.next();
                    cx.notify();
                })),
        )
        .child(
            div()
                .id("carousel")
                .relative()
                .w_full()
                .when(
                    layout.uses_lists_scene() || layout.uses_phone_frame(),
                    |el| {
                        el.border_color(paint(theme.color.on_surface))
                            .p(px(12.))
                            .rounded(px(carousel::PHONE_CORNER_DP))
                            .overflow_hidden()
                    },
                )
                .when(layout.center_aligned(), |el| el.justify_center())
                .flex()
                .when(layout.axis() == carousel::CarouselAxis::Vertical, |el| {
                    el.flex_col()
                })
                .gap(px(a.gap_dp))
                .child(
                    div().absolute().w(px(1.)).h(px(1.)).with_animation(
                        "carousel-live-clock",
                        Animation::new(Duration::from_millis(
                            gpui_material::motion::FRAME_MS as u64,
                        ))
                        .repeat(),
                        |el, _| el,
                    ),
                )
                .on_scroll_wheel(cx.listener(|this, ev: &ScrollWheelEvent, _, cx| {
                    let (dx, dy) = match ev.delta {
                        ScrollDelta::Pixels(p) => (f32::from(p.x), f32::from(p.y)),
                        ScrollDelta::Lines(p) => (p.x, p.y),
                    };
                    this.carousel_fling.selected = this.carousel_index;
                    this.carousel_fling.impulse(dx, dy);
                    this.carousel_fling_at = None;
                    cx.notify();
                }))
                .children(
                    carousel::MEDIA_CAPTIONS
                        .iter()
                        .enumerate()
                        .map(|(i, _label)| {
                            let w = carousel::item_width_during_fling_for(
                                layout, i, selected, offset_t,
                            );
                            let h = carousel::item_height_for_index(layout, i)
                                * if layout.axis() == carousel::CarouselAxis::Vertical {
                                    0.45
                                } else {
                                    1.0
                                };
                            div()
                                .id(SharedString::from(format!("carousel-{i}")))
                                .w(px(w))
                                .h(px(h))
                                .ml(px(shift))
                                .rounded(px(a.corners.top_left))
                                .overflow_hidden()
                                .child(photo_fill(carousel::media_kind(i)))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.carousel_index = carousel::snap_to(i);
                                    this.carousel_fling = carousel::FlingState::new(i);
                                    this.carousel_fling_at = None;
                                    cx.notify();
                                }))
                        }),
                ),
        )
        .when(layout.uses_lists_scene(), |el| {
            el.child(
                div()
                    .px(px(8.))
                    .pt(px(8.))
                    .flex()
                    .flex_col()
                    .child(spaced_line(
                        carousel::LISTS_TITLE,
                        16.0,
                        paint(theme.color.on_surface),
                    ))
                    .children(carousel::LISTS_ROWS.iter().map(|(icon, title, sub)| {
                        div()
                            .h(px(56.))
                            .flex()
                            .items_center()
                            .gap(px(12.))
                            .child(*icon)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(spaced_line(*title, 14.0, paint(theme.color.on_surface)))
                                    .child(spaced_line(
                                        *sub,
                                        12.0,
                                        paint(theme.color.on_surface_variant),
                                    )),
                            )
                    })),
            )
        })
}

fn section_title(theme: &Theme, title: &'static str) -> impl IntoElement {
    let style = theme.typography.title_medium.emphasized();
    div()
        .mt(px(8.))
        .text_size(px(style.size_sp))
        .font_weight(type_weight(style))
        .text_color(paint(theme.color.on_surface))
        .whitespace_nowrap()
        .child(spaced_line(
            title,
            style.size_sp,
            paint(theme.color.on_surface),
        ))
}

fn spaced_line(text: impl AsRef<str>, size: f32, color: gpui::Rgba) -> gpui::AnyElement {
    let text = text.as_ref();
    let words = typography::words(text);
    if words.len() <= 1 {
        return div()
            .text_size(px(size))
            .text_color(color)
            .line_height(px(size + 6.0))
            .child(text.to_string())
            .into_any_element();
    }
    let mut row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_end()
        .gap(px(typography::WORD_GAP_DP));
    for w in words {
        row = row.child(
            div()
                .text_size(px(size))
                .text_color(color)
                .line_height(px(size + 6.0))
                .child(w.to_string()),
        );
    }
    row.into_any_element()
}

fn outlined_notched_field(
    id: &'static str,
    field: &text_field::TextFieldAppearance,
    label: SharedString,
    value: SharedString,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let outline = field.field.outline.unwrap_or((field.label, 1.0));
    let frame = text_field::notch_frame(label.as_ref(), field);
    let fill = field.field.container;
    let (lx, ly) = frame.label_origin_dp();
    let stroke_color = paint(outline.0);
    let h = field.field.height_dp;
    let radius = frame.radius_dp;
    let field_m = field.clone();
    let label_m = label.clone();
    div()
        .id(id)
        .relative()
        .w_full()
        .h(px(h))
        .on_click(on_click)
        .child(
            div()
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .rounded(px(radius))
                .bg(paint(fill)),
        )
        .child(
            canvas(
                move |_, window, _| {
                    let measured = measure_label_width_dp(
                        window,
                        label_m.as_ref(),
                        field_m.label_style.size_sp,
                    );
                    text_field::notch_frame_from_layout(label_m.as_ref(), &field_m, measured)
                },
                move |bounds, frame, window, _| {
                    let w = f32::from(bounds.size.width);
                    let verbs_w = frame.evenodd_verbs(w);
                    let mut fill_b = PathBuilder::fill().with_style(PathStyle::Fill(
                        FillOptions::default().with_fill_rule(FillRule::EvenOdd),
                    ));
                    feed_outline_verbs(&mut fill_b, bounds.origin, verbs_w);
                    if let Ok(path) = fill_b.build() {
                        window.paint_path(path, stroke_color);
                    }
                    let center = frame.centerline_polyline(w);
                    paint_round_polyline(
                        window,
                        bounds.origin,
                        &center,
                        frame.stroke_dp,
                        stroke_color,
                    );
                },
            )
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .size_full(),
        )
        .child(
            div()
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .size_full()
                .px(px(16.))
                .flex()
                .items_center()
                .child(
                    div()
                        .text_size(px(field.input_style.size_sp))
                        .text_color(paint(field.input))
                        .child(value),
                ),
        )
        .child(
            div()
                .absolute()
                .left(px(lx))
                .top(px(ly))
                .w(px(frame.width_dp))
                .h(px(frame.label_h_dp))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_size(px(field.label_style.size_sp))
                        .text_color(paint(field.label))
                        .line_height(px(field.label_style.line_height_sp))
                        .child(label),
                ),
        )
}

fn field_block(
    id: &'static str,
    field: &text_field::TextFieldAppearance,
    label: impl Into<SharedString>,
    value: impl Into<SharedString>,
    supporting: impl Into<SharedString>,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let outline = field.field.outline.unwrap_or((field.label, 1.0));
    let outlined = field.is_outlined();
    let label = label.into();
    let value = value.into();
    let box_el = if outlined && field.notched {
        outlined_notched_field(id, field, label, value, on_click).into_any_element()
    } else if outlined && field.floating {
        div()
            .id(id)
            .h(px(field.field.height_dp))
            .px(px(16.))
            .rounded(px(field.field.corners.top_left))
            .bg(paint(field.field.container))
            .border_1()
            .border_color(paint(outline.0))
            .flex()
            .flex_col()
            .justify_end()
            .pb(px(8.))
            .on_click(on_click)
            .child(
                div()
                    .text_size(px(field.label_style.size_sp))
                    .text_color(paint(field.label))
                    .child(label),
            )
            .child(
                div()
                    .text_size(px(field.input_style.size_sp))
                    .text_color(paint(field.input))
                    .child(value),
            )
            .into_any_element()
    } else if outlined {
        div()
            .id(id)
            .h(px(field.field.height_dp))
            .px(px(16.))
            .rounded(px(field.field.corners.top_left))
            .bg(paint(field.field.container))
            .border_1()
            .border_color(paint(outline.0))
            .flex()
            .items_center()
            .on_click(on_click)
            .child(
                div()
                    .text_size(px(field.label_style.size_sp))
                    .text_color(paint(field.label))
                    .child(label),
            )
            .into_any_element()
    } else if field.floating {
        div()
            .id(id)
            .h(px(field.field.height_dp))
            .px(px(16.))
            .rounded(px(field.field.corners.top_left))
            .bg(paint(field.field.container))
            .flex()
            .flex_col()
            .justify_end()
            .pb(px(8.))
            .on_click(on_click)
            .child(
                div()
                    .text_size(px(field.label_style.size_sp))
                    .text_color(paint(field.label))
                    .child(label),
            )
            .child(
                div()
                    .text_size(px(field.input_style.size_sp))
                    .text_color(paint(field.input))
                    .child(value),
            )
            .when(field.shows_indicator(), |el| {
                el.child(
                    div()
                        .h(px(outline.1.max(1.0)))
                        .w_full()
                        .bg(paint(outline.0)),
                )
            })
            .into_any_element()
    } else {
        div()
            .id(id)
            .h(px(field.field.height_dp))
            .px(px(16.))
            .rounded(px(field.field.corners.top_left))
            .bg(paint(field.field.container))
            .flex()
            .flex_col()
            .on_click(on_click)
            .child(
                div().flex_1().flex().items_center().child(
                    div()
                        .text_size(px(field.label_style.size_sp))
                        .text_color(paint(field.label))
                        .child(label),
                ),
            )
            .when(field.shows_indicator(), |el| {
                el.child(
                    div()
                        .h(px(outline.1.max(1.0)))
                        .w_full()
                        .bg(paint(outline.0)),
                )
            })
            .into_any_element()
    };
    div().flex().flex_col().gap(px(4.)).child(box_el).child(
        div()
            .px(px(16.))
            .text_size(px(field.supporting_style.size_sp))
            .text_color(paint(field.supporting))
            .child(supporting.into()),
    )
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(720.), px(880.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Material 3 desktop catalog".into()),
                    ..Default::default()
                }),
                app_id: Some("dev.gpui.material_desktop_demo".into()),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| CatalogView {
                    dark: false,
                    taps: 0,
                    checked: true,
                    switched: true,
                    overlay: Overlay::None,
                    overlay_menu: menu::OverlayMenuSession::overlay(),
                    cascade_menu: menu::OverlayMenuSession::cascade(),
                    standard_overflow_menu: menu::OverlayMenuSession::standard_overflow(),
                    connected_overflow_menu: menu::OverlayMenuSession::connected_overflow(),
                    split_menu: menu::OverlayMenuSession::split(),
                    slider: slider::OVERVIEW_ROWS[3].value,
                    ringtone: 2,
                    tab: tabs::SCENE_SELECTED,
                    picker_year: 2026,
                    picker_month: 9,
                    selected: CivilDate {
                        year: 2026,
                        month: 9,
                        day: 15,
                    },
                    today: CivilDate {
                        year: 2026,
                        month: 9,
                        day: 11,
                    },
                    filled: TextFieldEditor::new_expressive(
                        text_field::TextFieldVariant::Filled,
                        "Input text",
                    ),
                    outlined: TextFieldEditor::new_expressive(
                        text_field::TextFieldVariant::Outlined,
                        "you@domain.com",
                    ),
                    group_selected: button_group::DEMO_SELECTED,
                    standard_selected: button_group::STANDARD_SELECTED,
                    list_selected: list::SCENE_SELECTED,
                    list_swipe: list::ListSwipeState::revealed(),
                    list_swipe_at: None,
                    list_order: list::REORDER_DEMO,
                    tooltip_plain_open: false,
                    tooltip_rich_open: false,
                    icon_selected: button_group::ICON_SELECTED,
                    overflow_open: button_group::OVERFLOW_OPEN,
                    standard_overflow_open: button_group::STANDARD_OVERFLOW_OPEN,
                    range_start: slider::RANGE_DEMO_START,
                    range_end: slider::RANGE_DEMO_END,
                    range_drag: None,
                    range_focus: slider::RangeThumb::Start,
                    range_moved: false,
                    range_hit: Rc::new(Cell::new((0.0, slider::RANGE_TRACK_W_DP))),
                    docked_open: date_picker::DOCKED_OPEN_BY_DEFAULT,
                    date_display: date_picker::LIVE_DISPLAY_MODE,
                    date_pane: date_picker::LIVE_PANE,
                    date_range: date_picker::DateRangeSelection::demo(),
                    date_range_committed: date_picker::DateRangeSelection::demo(),
                    range_year: date_picker::RANGE_DEMO_START.year,
                    range_month: date_picker::RANGE_DEMO_START.month,
                    range_pane: date_picker::LIVE_PANE,
                    range_display: date_picker::LIVE_DISPLAY_MODE,
                    search_open: search::VIEW_OPEN_BY_DEFAULT,
                    search_filter: search::SearchFilter::All,
                    search: {
                        let mut ed = TextFieldEditor::new(text_field::TextFieldVariant::Filled, "");
                        ed.set_focus(search::VIEW_OPEN_BY_DEFAULT);
                        ed
                    },
                    rail_selected: navigation_rail::DEMO_SELECTED,
                    rail_mode: navigation_rail::DEMO_MODE,
                    wide_rail_mode: navigation_rail::WIDE_DEMO_MODE,
                    narrow_rail_mode: navigation_rail::NARROW_DEMO_MODE,
                    hide_rail_mode: navigation_rail::HIDE_DEMO_MODE,
                    header_rail_mode: navigation_rail::HEADER_DEMO_MODE,
                    header_tooltip_open: false,
                    carousel_index: carousel::DEMO_INDEX,
                    carousel_layout: carousel::CarouselLayout::Hero,
                    carousel_fling: carousel::FlingState::new(carousel::DEMO_INDEX),
                    carousel_fling_at: None,
                    typeahead_focus: cx.focus_handle(),
                    typeahead_host: None,
                    chip_pressed: None,
                    chip_press_seq: 0,
                    chip_press_anim: None,
                    snack_state: snackbar::SnackbarState::short(),
                    snack_at: Instant::now(),
                    fab_menu_open: fab_menu::DEMO_EXPANDED,
                    split_open: split_button::DEMO_OPEN,
                    app_bar_collapse: 0.0,
                    last_catalog_ime: None,
                    time_hour: time_picker::demo_hour(time_picker::DEMO_FORMAT),
                    time_minute: time_picker::DEMO_MINUTE,
                    time_period: time_picker::DEMO_PERIOD,
                    time_dial: time_picker::DEMO_DIAL,
                    time_hand_from: time_picker::hand_angle_deg(
                        time_picker::DEMO_DIAL,
                        time_picker::demo_hour(time_picker::DEMO_FORMAT),
                        time_picker::DEMO_MINUTE,
                    ),
                    time_hand_gen: 0,
                    time_scroll: time_picker::TimeScrollState::demo(),
                    time_scroll_at: None,
                    time_display: time_picker::DEMO_DISPLAY_MODE,
                    time_input: time_picker::TimeInputState::demo(),
                    time_format: time_picker::DEMO_FORMAT,
                })
            },
        )
        .expect("failed to open Material desktop catalog window");
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use super::{nav_rail_os_popup_options, WindowKind};
    use gpui_material::components::{
        button, button_group, carousel, chip, date_picker, dialog, fab_menu, icon_button, list,
        menu, navigation_bar, navigation_rail, progress, search, side_sheet, slider, split_button,
        text_field, time_picker, toolbar, tooltip, top_app_bar,
    };
    use gpui_material::theme::Theme;
    use gpui_material::InteractionState;

    #[test]
    fn desktop_heroes_use_shared_resolve_not_local_constants() {
        let theme = Theme::light();
        let btn = button::resolve(
            &theme,
            button::ButtonVariant::Filled,
            InteractionState::Enabled,
        );
        assert_eq!(btn.height_dp, 40.0);
        assert_eq!(btn.pad_start_dp, 16.0);
        assert_eq!(btn.container, theme.color.primary);

        let field = text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Outlined,
            InteractionState::Focused,
            true,
        );
        assert!(field.notched);
        assert_eq!(field.field.outline, Some((theme.color.primary, 2.0)));

        let s = slider::resolve(&theme, 0.55, InteractionState::Enabled);
        assert_eq!(s.track_h, 16.0);
        assert_eq!(s.handle_w, 4.0);
        assert_eq!(s.handle_h, 44.0);
        assert_eq!(s.handle_h_visual, 28.0);
        assert_eq!(s.inactive, theme.color.surface_container_highest);
        assert_eq!(s.stop_count, 2);
        let alarm = slider::resolve_with_stops(&theme, 0.52, InteractionState::Enabled, 13);
        assert_eq!(alarm.stop_count, 13);
        assert_eq!(slider::OVERVIEW_ROWS[1].stop_count, 13);
        assert_eq!(slider::OVERVIEW_ROWS[0].label, "Call volume");

        let empty_outlined = text_field::resolve(
            &theme,
            text_field::TextFieldVariant::Outlined,
            InteractionState::Enabled,
            false,
        );
        assert!(!empty_outlined.notched);
        assert!(!empty_outlined.floating);
        assert_eq!(empty_outlined.label_style.name, "bodyLarge");

        let tonal = text_field::resolve_expressive(
            &theme,
            text_field::TextFieldVariant::Filled,
            InteractionState::Enabled,
            false,
        );
        assert_eq!(tonal.field.corners.top_left, text_field::ROUNDED_SHAPE_DP);
        assert_eq!(tonal.field.container, theme.color.surface_container);
        assert!(!tonal.shows_indicator());
        assert_eq!(tonal.label_position, text_field::LabelPosition::Inside);

        let group = button_group::resolve_segment(&theme, 0, 3, false, false);
        assert_eq!(group.corners.top_left, 20.0);
        assert_eq!(button_group::CONNECTED_GAP_DP, 2.0);
        let range = slider::resolve_range(&theme, 0.2, 0.75, InteractionState::Enabled);
        assert_eq!(range.start, 0.2);
        let (s, e) = slider::nudge_start(0.20, 0.75, slider::RANGE_STEP);
        assert!((s - 0.25).abs() < 1e-5);
        assert_eq!(e, 0.75);
        assert_eq!(
            dialog::resolve(&theme).headline_style.name,
            "headlineSmallEmphasized"
        );
        assert_eq!(dialog::resolve_fullscreen(&theme).header_h_dp, 64.0);
        assert_eq!(button_group::icon_group_count(), 4);
        assert_eq!(
            carousel::item_width_for(carousel::CarouselLayout::CenteredHero, 0, 0),
            carousel::CENTERED_LARGE_W_DP
        );
        assert_eq!(
            carousel::item_width_for(carousel::CarouselLayout::FullScreen, 0, 0),
            carousel::FULLSCREEN_W_DP
        );
        assert!(carousel::CarouselLayout::CenteredHero.uses_phone_frame());
        assert_eq!(search::resolve(&theme).bar.height_dp, 56.0);
        assert_eq!(search::resolve_view(&theme).header_h_dp, 72.0);
        assert_eq!(search::SUGGESTIONS.len(), 4);
        assert_eq!(
            time_picker::resolve(&theme).time_style.name,
            "displaySmallEmphasized"
        );
        assert_eq!(
            time_picker::DEMO_STYLE,
            time_picker::TimePickerStyle::Scroll
        );
        let scroll = time_picker::resolve_scroll(&theme);
        assert_eq!(scroll.container, theme.color.primary_container);
        assert_eq!(scroll.field_h_dp, time_picker::SCROLL_FIELD_H_DP);
        assert_eq!(scroll.selected_style.name, "displayLargeEmphasized");
        assert_eq!(
            time_picker::TimeScrollState::demo().hour_value(),
            time_picker::DEMO_HOUR_24
        );
        assert_eq!(time_picker::DEMO_FORMAT, time_picker::TimeFormat::Hour24);
        assert!(!time_picker::DEMO_FORMAT.shows_period());
        assert_eq!(search::DEMO_STYLE, search::SearchStyle::Contained);
        assert_eq!(search::contained_margin_dp(true), 12.0);
        assert_eq!(
            search::WindowWidthClass::from_width_dp(search::DEMO_COMPACT_WIDTH_DP)
                .expanded_search(),
            search::SearchExpandedLayout::FullScreen
        );
        assert_eq!(
            search::contained_corner_dp_layout(search::SearchExpandedLayout::FullScreen, true),
            0.0
        );
        assert_eq!(search::SUGGESTION_GROUPS.len(), 2);
        assert_eq!(search::SUGGESTION_GROUP_GAP_DP, 8.0);
        assert_eq!(
            search::list_status(search::DEMO_QUERY, true),
            search::SearchListStatus::QuickResults
        );
        assert_eq!(
            search::list_status("App", false),
            search::SearchListStatus::Results
        );
        assert_eq!(search::DOCKED_MIN_H_DP, 240.0);
        assert!(search::uses_docked_scrim(
            search::SearchExpandedLayout::Docked,
            true
        ));
        assert_eq!(search::SCRIM_OPACITY, 0.32);
        assert_eq!(search::ROW_GAP_DP, 2.0);
        assert_eq!(search::row_corners(0, 3, false).top_left, 16.0);
        assert_eq!(search::RESULT_H_DP, 72.0);
        assert_eq!(search::supporting_for("App"), "Installed application");
        assert_eq!(search::ROW_LEADING_AVATAR_DP, 40.0);
        assert!(search::shows_empty(search::DEMO_EMPTY_QUERY));
        assert_eq!(search::FILTER_ROW_H_DP, 48.0);
        assert!(search::shows_empty_in(
            search::DEMO_QUERY,
            search::SearchFilter::Settings
        ));
        assert_eq!(search::EMPTY_H_DP, 56.0);
        assert_eq!(
            date_picker::input_field_value(date_picker::CivilDate {
                year: 2026,
                month: 9,
                day: 15,
            }),
            "09/15/2026"
        );
        assert_eq!(
            date_picker::DEMO_DISPLAY_MODE,
            date_picker::DatePickerDisplayMode::Input
        );
        assert_eq!(
            date_picker::LIVE_DISPLAY_MODE,
            date_picker::DatePickerDisplayMode::Picker
        );
        assert!(date_picker::SHOW_MODE_TOGGLE);
        assert_eq!(
            date_picker::apply_display_toggle(date_picker::LIVE_DISPLAY_MODE),
            date_picker::DatePickerDisplayMode::Input
        );
        assert_eq!(
            date_picker::supporting_for(date_picker::DatePickerDisplayMode::Input),
            Some(date_picker::INPUT_SUPPORTING)
        );
        assert_eq!(
            date_picker::LIVE_PANE,
            date_picker::DatePickerPane::Calendar
        );
        assert_eq!(date_picker::YEARS_IN_ROW, 3);
        assert_eq!(date_picker::YEAR_CONTAINER_W_DP, 72.0);
        assert_eq!(date_picker::year_window(2026)[4], 2026);
        assert_eq!(
            date_picker::classify_year(2026, 2026, 2026),
            date_picker::YearKind::Selected
        );
        assert_eq!(date_picker::RANGE_INPUT_HEADLINE, "Enter dates");
        assert!(date_picker::is_range_input_valid(
            "09/15/2026",
            "09/21/2026"
        ));
        assert!(date_picker::RANGE_LIVE);
        let restarted = date_picker::apply_range_tap(
            date_picker::DateRangeSelection::demo(),
            date_picker::RANGE_DEMO_START,
        );
        assert_eq!(restarted.end, None);
        assert_eq!(
            date_picker::header_range_selection(restarted),
            "Sep 15 – End date"
        );
        assert!(date_picker::RANGE_MONTH_NAV);
        assert!(date_picker::RANGE_YEAR_PANE);
        assert!(date_picker::RANGE_SHOW_MODE_TOGGLE);
        assert!(date_picker::RANGE_ACTIONS);
        assert_eq!(
            date_picker::apply_range_confirm(date_picker::DateRangeSelection::demo()),
            date_picker::DateRangeSelection::demo()
        );
        assert_eq!(
            date_picker::range_title_for(date_picker::DatePickerDisplayMode::Input),
            date_picker::RANGE_INPUT_HEADLINE
        );
        assert_eq!(
            date_picker::range_field_value(date_picker::DateRangeSelection::demo(), false),
            "09/15/2026"
        );
        assert_eq!(date_picker::apply_range_month(2026, 9, 1), (2026, 10));
        assert_eq!(date_picker::apply_range_year(2026, 9, 2027), (2027, 9));
        assert_eq!(
            date_picker::apply_range_month(date_picker::YEAR_RANGE_START, 1, -1),
            (date_picker::YEAR_RANGE_START, 1)
        );
        assert_eq!(
            search::row_leading_kind(search::SearchListStatus::Results, "App"),
            search::RowLeadingKind::Avatar
        );
        assert!(search::shows_clear(search::DEMO_QUERY));
        assert_eq!(
            search::trailing_action(search::DEMO_QUERY),
            search::TRAILING_CLEAR
        );
        let input = time_picker::resolve_input(&theme);
        assert_eq!(input.field_w_dp, 96.0);
        assert_eq!(input.field_h_dp, 72.0);
        assert_eq!(
            time_picker::DEMO_DISPLAY_MODE.toggle(),
            time_picker::TimePickerDisplayMode::Scroll
        );
        assert!(time_picker::TimeInputState::demo().is_input_valid());
        assert_eq!(
            time_picker::TimeInputState::demo().hour_value(),
            Some(time_picker::DEMO_HOUR_24)
        );
        assert_eq!(time_picker::DEMO_DIAL, time_picker::DialFace::Hour);
        assert_eq!(
            time_picker::DEMO_DESKTOP_LAYOUT,
            time_picker::TimePickerLayoutType::Horizontal
        );
        assert_eq!(time_picker::PERIOD_HORIZONTAL_W_DP, 216.0);
        assert_eq!(
            time_picker::hour_ring(18, time_picker::TimeFormat::Hour24),
            time_picker::DialRing::Inner
        );
        assert!((time_picker::INNER_CIRCLE_RADIUS_DP - 69.0).abs() < 0.01);
        assert!(
            (time_picker::time_selector_w_dp(time_picker::TimeFormat::Hour24) - 114.0).abs() < 0.01
        );
        let (s, e, thumb) =
            slider::apply_arrow(0.2, 0.75, slider::RangeThumb::Start, "right").expect("arrow");
        assert!((s - 0.25).abs() < 1e-5);
        assert_eq!(e, 0.75);
        assert_eq!(thumb, slider::RangeThumb::Start);
        let frame = text_field::notch_frame("Email", &field);
        assert_eq!(frame.radius_dp, 4.0);
        assert_eq!(frame.stroke_dp, 2.0);
        assert_eq!(frame.top_lead_dp(), 4.0);
        assert_eq!(frame.notch_gap_h_dp(), 2.0);
        assert!(frame.outline_svg_d(280.0).starts_with('M'));
        assert!(frame.evenodd_svg_d(280.0).contains('Z'));
        assert_eq!(frame.evenodd_subpath_count(280.0), 1);
        assert_eq!(carousel::LARGE_W_DP, 256.0);
        let popup_opts = nav_rail_os_popup_options(navigation_rail::os_popup_spec(
            navigation_rail::RailMode::Expanded,
        ));
        assert!(matches!(popup_opts.kind, WindowKind::PopUp));
        assert!(!navigation_rail::OS_POPUP_OPENED);
        assert!(popup_opts
            .titlebar
            .as_ref()
            .and_then(|t| t.title.as_ref())
            .map(|s| s.as_ref() == navigation_rail::OS_POPUP_TITLE)
            .unwrap_or(false));
        assert!((search::morph_path_scale_eased(0.0) - search::SHARED_SCALE_DOCKED).abs() < 0.01);
        assert_eq!(gpui_material::motion::FRAME_MS, 16);
        assert_eq!(progress::STROKE_CAP, progress::StrokeCap::Round);
        assert_eq!(search::resolve_activity(&theme).corners.top_left, 0.0);
        assert!((slider::fraction_from_local_x(140.0, 280.0) - 0.5).abs() < 1e-5);
        assert_eq!(
            fab_menu::resolve_item(&theme, fab_menu::FabMenuColor::Primary).height_dp,
            56.0
        );
        assert_eq!(split_button::GAP_DP, 2.0);
        assert_eq!(
            toolbar::resolve(
                &theme,
                toolbar::ToolbarKind::Floating,
                toolbar::ToolbarColor::Vibrant,
                toolbar::ToolbarAxis::Horizontal,
            )
            .height_dp,
            64.0
        );
        assert_eq!(
            top_app_bar::resolve_scene(&theme, 0.0).expanded_height_dp,
            152.0
        );
        assert_eq!(top_app_bar::resolve_scene(&theme, 1.0).height_dp, 64.0);
        assert_eq!(side_sheet::resolve_scene(&theme).width_dp, 256.0);
        assert_eq!(side_sheet::HEADLINE, "Filters");
        assert_eq!(navigation_bar::resolve(&theme).height_dp, 64.0);
        assert_eq!(
            navigation_bar::resolve_horizontal(&theme).indicator_h_dp,
            40.0
        );
        assert_eq!(tooltip::resolve_plain(&theme).min_height_dp, 24.0);
        assert_eq!(tooltip::resolve_rich(&theme).max_width_dp, 320.0);
        assert_eq!(tooltip::PLAIN_TEXT, "Add to library");
        assert_eq!(list::SEGMENTED_GAP_DP, 2.0);
        assert_eq!(list::SCENE_HEADLINES[0], "Wi-Fi");
        assert_eq!(
            list::resolve_scene(&theme, 0, 0).container,
            theme.color.secondary_container
        );
        assert_eq!(tooltip::CARET_W_DP, 16.0);
        assert_eq!(tooltip::LONG_PRESS_MS, 500);
        assert_eq!(button_group::STANDARD_GAP_DP, 12.0);
        assert_eq!(button_group::EXPANDED_RATIO, 0.15);
        assert_eq!(button_group::STANDARD_OVERFLOW_ITEMS[0], "Left");
        let std_ov = button_group::resolve_standard_overflow(&theme, false);
        assert_eq!(std_ov.container, theme.color.primary);
        assert_eq!(std_ov.corners.top_left, 20.0);
        assert_eq!(list::SWIPE_REVEAL_DP, 80.0);
        assert_eq!(list::SWIPE_OVERSHOOT_DP, 16.0);
        assert_eq!(list::SWIPE_PRIMARY_ACTION_DP, 360.0);
        assert_eq!(list::DRAG_HANDLE_DP, 24.0);
        assert_eq!(list::SWIPE_LEADING_LABEL, "Archive");
        assert_eq!(
            list::ListSwipeState::revealed().phase,
            list::SwipePhase::Open
        );
        assert_eq!(
            icon_button::container_width_dp(
                icon_button::WIDTH_HERO_SIZE,
                icon_button::IconButtonWidth::Wide
            ),
            52.0
        );
        assert_eq!(
            icon_button::container_width_dp(
                icon_button::WIDTH_HERO_SIZE,
                icon_button::IconButtonWidth::Narrow
            ),
            32.0
        );
        assert_eq!(
            icon_button::pad_h_dp(
                button::ButtonSize::Medium,
                icon_button::IconButtonWidth::Wide
            ),
            24.0
        );
        let toggle_on = icon_button::resolve_toggle(
            &theme,
            icon_button::IconButtonVariant::Filled,
            icon_button::TOGGLE_HERO_SIZE,
            button::ButtonShape::Round,
            true,
            InteractionState::Enabled,
        );
        assert_eq!(toggle_on.container, theme.color.primary);
        assert_eq!(toggle_on.corners.top_left, 12.0);
        let toggle_off = icon_button::resolve_toggle(
            &theme,
            icon_button::IconButtonVariant::Filled,
            icon_button::TOGGLE_HERO_SIZE,
            button::ButtonShape::Round,
            false,
            InteractionState::Enabled,
        );
        assert_eq!(toggle_off.container, theme.color.surface_container);
        assert_eq!(toggle_off.corners.top_left, 20.0);
        assert_eq!(icon_button::IconButtonSelection::Unselected.glyph(), "☆");
        assert_eq!(menu::ITEM_HEIGHT_DP, 44.0);
        assert_eq!(menu::GROUP_GAP_DP, 2.0);
        let menu_shell = menu::resolve_menu(&theme);
        assert_eq!(menu_shell.container, theme.color.surface_container_low);
        assert_eq!(menu_shell.corners.top_left, 16.0);
        let menu_sel = menu::resolve_item(&theme, true, InteractionState::Enabled);
        assert_eq!(menu_sel.container, theme.color.tertiary_container);
        assert_eq!(menu_sel.height_dp, 44.0);
        let week = menu::resolve_horizontal(
            &theme,
            menu::MenuScheme::Standard,
            menu::HORIZONTAL_SELECTED,
            menu::HORIZONTAL_LABELS.len(),
            true,
            InteractionState::Enabled,
        );
        assert_eq!(week.corners.top_left, 999.0);
        let submenu = menu::resolve_submenu(&theme, menu::MenuScheme::Standard);
        assert_eq!(submenu.corners.top_left, 24.0);
        assert_eq!(
            menu::group_corners_focus(0, 3, menu::MenuFocus::Inactive).top_left,
            8.0
        );
        assert_eq!(menu::SUBMENU_ITEMS[0].label, "Share");
        assert_eq!(
            menu::typeahead_index(&menu::submenu_labels(), 0, 's'),
            Some(1)
        );
        let mut overlay = menu::OverlayMenuSession::overlay();
        assert!(!overlay.submenu_open);
        assert_eq!(
            overlay.click_parent(menu::MORE_INDEX),
            menu::OverlayMenuAction::Stay
        );
        assert!(overlay.submenu_open);
        assert_eq!(overlay.parent_focus(), menu::MenuFocus::Inactive);
        let mut overflow = menu::OverlayMenuSession::standard_overflow();
        assert_eq!(overflow.kind, menu::GroupedPopupKind::StandardOverflow);
        assert_eq!(overflow.kind.groups().len(), 2);
        assert_eq!(overflow.kind.item_at(0).unwrap().2.label, "Left");
        assert_eq!(
            overflow.click_parent(overflow.kind.more_index()),
            menu::OverlayMenuAction::Stay
        );
        assert!(overflow.submenu_open);
        let split = menu::OverlayMenuSession::split();
        assert_eq!(split.kind.item_at(0).unwrap().2.label, "Add to cart");
        assert_eq!(menu::HOVER_OPEN_DELAY_MS, 200);
        let mut delayed = menu::OverlayMenuSession::standard_overflow();
        let intent = delayed.hover_parent(delayed.kind.more_index());
        assert!(!delayed.submenu_open);
        let menu::HoverOpenIntent::Delay { index, seq } = intent else {
            panic!("overflow More hover should delay");
        };
        assert!(delayed.confirm_hover_open(index, seq));
        assert!(delayed.submenu_open);
        assert_eq!(chip::HEIGHT_DP, 32.0);
        assert_eq!(chip::UNSELECTED_CORNER_DP, 12.0);
        assert_eq!(chip::SELECTED_CORNER_DP, 16.0);
        assert_eq!(chip::PRESSED_CORNER_DP, 8.0);
        assert_eq!(chip::FILTER_HERO[1].label, "Washer");
        assert_eq!(chip::AVATAR_DP, 24.0);
        assert!(chip::INPUT_AVATAR_HERO[0].has_avatar());
        assert_eq!(
            chip::animated_corner_dp(&theme, chip::ChipVariant::Filter, false, 0.0),
            12.0
        );
        assert_eq!(
            chip::animated_corner_dp(&theme, chip::ChipVariant::Filter, false, 1.0),
            8.0
        );
        assert_eq!(chip::press_t_anim(true, 0.5), 0.5);
        assert_eq!(chip::press_ms(&theme), 350);
        assert!(menu::TYPEAHEAD_AUTOFOCUS);
        assert!(menu::typeahead_autofocus_in_page(true));
        assert!(!menu::typeahead_autofocus_in_page(false));
        assert_eq!(
            menu::typeahead_autofocus_kind(false, true, false, false),
            Some(menu::GroupedPopupKind::StandardOverflow)
        );
        let washer = chip::resolve(
            &theme,
            chip::ChipVariant::Filter,
            true,
            InteractionState::Enabled,
        );
        assert_eq!(washer.container, theme.color.secondary_container);
        assert_eq!(washer.corners.top_left, 16.0);
        assert_eq!(washer.pad_start_dp, 8.0);
        let elevator = chip::resolve(
            &theme,
            chip::ChipVariant::Filter,
            false,
            InteractionState::Enabled,
        );
        assert_eq!(elevator.corners.top_left, 12.0);
        assert_eq!(elevator.outline, Some((theme.color.outline_variant, 1.0)));
        assert_eq!(elevator.content, theme.color.on_surface_variant);
        let pets = chip::resolve(
            &theme,
            chip::ChipVariant::Filter,
            false,
            InteractionState::Pressed,
        );
        assert_eq!(pets.corners.top_left, 8.0);
        let elev = chip::resolve_demo(&theme, chip::ELEVATED_FILTER_HERO[0]);
        assert_eq!(elev.container, theme.color.surface_container_low);
        assert_eq!(elev.elevation_dp, 1.0);
        assert_eq!(elev.outline, None);
        assert_eq!(elev.secondary_content, Some(theme.color.on_surface_variant));
        assert!(!menu::OVERLAY_USES_SCRIM);
        assert_eq!(menu::OVERLAY_ANCHOR_LABEL, "Menu");
        assert_eq!(
            navigation_rail::icon_position_for(false),
            navigation_rail::IconPosition::Top
        );
        assert_eq!(
            navigation_rail::icon_position_for(true),
            navigation_rail::IconPosition::Start
        );
        let start = navigation_rail::item_metrics(&theme, navigation_rail::IconPosition::Start);
        assert_eq!(start.indicator_h_dp, 56.0);
        assert_eq!(start.label_style.name, "labelLarge");
        assert_eq!(navigation_rail::WIDE_COLLAPSED_WIDTH_DP, 96.0);
        assert_eq!(
            navigation_rail::RailMode::Collapsed.width_dp(),
            navigation_rail::WIDE_COLLAPSED_WIDTH_DP
        );
        assert_eq!(
            navigation_rail::RailExpandedLayout::Standard.label(),
            "standard"
        );
        assert!(navigation_rail::RailExpandedLayout::Standard.in_flow());
        assert!(!navigation_rail::is_modal_for(
            navigation_rail::RailMode::Expanded,
            navigation_rail::RailExpandedLayout::Standard
        ));
        assert!((navigation_rail::morph_width_dp(0.0) - 96.0).abs() < 0.01);
        assert_eq!(
            navigation_rail::RailCollapsedKind::Narrow.width_dp(),
            navigation_rail::WIDTH_DP
        );
        assert!(
            (navigation_rail::morph_width_eased_kind(
                &theme,
                navigation_rail::RailCollapsedKind::Narrow,
                0.0
            ) - 80.0)
                .abs()
                < 0.01
        );
        assert_eq!(
            navigation_rail::resolve_mode_kind(
                &theme,
                navigation_rail::RailMode::Collapsed,
                navigation_rail::RailCollapsedKind::Narrow
            )
            .width_dp,
            80.0
        );
        assert_eq!(navigation_rail::IN_FLOW_BODY, "Inbox");
        assert!(navigation_rail::WIDE_DEMO_HAS_EXTENDED_FAB);
        assert!(navigation_rail::MODAL_DEMO_HAS_EXTENDED_FAB);
        assert!(navigation_rail::NARROW_DEMO_HAS_EXTENDED_FAB);
        assert!(navigation_rail::HIDE_DEMO_HAS_EXTENDED_FAB);
        let fabh = navigation_rail::fab_morph_hide(&theme, 220.0);
        assert!((fabh.width_dp - 188.0).abs() < 0.01);
        assert_eq!(fabh.label, "Create");
        assert!(navigation_rail::WIDE_DEMO_LAYOUT.in_flow());
        assert!(navigation_rail::HIDE_DEMO_HIDE_ON_COLLAPSE);
        assert!(navigation_rail::HIDE_DEMO_ARRANGEMENT.is_center());
        assert!(navigation_rail::HEADER_DEMO_HAS_HEADER);
        assert!(navigation_rail::HEADER_DEMO_HAS_FAB);
        assert_eq!(navigation_rail::FAB_LABEL, "Create");
        let fab0 = navigation_rail::fab_morph(&theme, 0.0, 96.0);
        assert!((fab0.width_dp - 56.0).abs() < 0.01);
        let fab1 = navigation_rail::fab_morph(&theme, 1.0, 220.0);
        assert!((fab1.width_dp - 188.0).abs() < 0.01);
        let fabn0 = navigation_rail::fab_morph_kind(
            &theme,
            0.0,
            80.0,
            navigation_rail::RailCollapsedKind::Narrow,
        );
        assert!((fabn0.width_dp - 56.0).abs() < 0.01);
        assert!((fabn0.margin_start_dp - 12.0).abs() < 0.01);
        let fabn1 = navigation_rail::fab_morph_kind(
            &theme,
            1.0,
            220.0,
            navigation_rail::RailCollapsedKind::Narrow,
        );
        assert!((fabn1.width_dp - 188.0).abs() < 0.01);
        assert_eq!(navigation_rail::MODAL_EXPANDED_SHAPE_DP, 16.0);
        assert_eq!(navigation_rail::CONTENT_PAD_VERTICAL_DP, 44.0);
        assert_eq!(
            navigation_rail::content_padding().top_dp,
            navigation_rail::WIDE_TOP_SPACE_DP
        );
        assert!((navigation_rail::content_padding().start_dp).abs() < 0.01);
        assert_eq!(
            navigation_rail::container_morph_for_mode(
                &theme,
                navigation_rail::RailExpandedLayout::Modal,
                navigation_rail::RailMode::Expanded
            )
            .color,
            theme.color.surface_container
        );
        assert!(navigation_rail::HEADER_DEMO_ARRANGEMENT.is_bottom());
        assert_eq!(
            navigation_rail::header_menu_glyph(false),
            navigation_rail::HEADER_MENU_GLYPH
        );
        assert_eq!(
            navigation_rail::header_menu_label(true),
            navigation_rail::HEADER_COLLAPSE_LABEL
        );
        assert!((navigation_rail::hide_slide_offset_dp(0.0) + 220.0).abs() < 0.01);
        assert_eq!(
            navigation_rail::icon_position_for_hide_mode(
                navigation_rail::RailMode::Collapsed,
                true
            ),
            navigation_rail::IconPosition::Start
        );
        let morph = navigation_rail::item_morph(&theme, 0.2, 150.0);
        assert!(morph.icon_box_w_dp > 24.0 && morph.icon_box_w_dp < 56.0);
        assert!(morph.dest_indicator_alpha > 0.0 && morph.dest_indicator_alpha < 1.0);
        assert_eq!(
            navigation_rail::resolve(&theme).active_label,
            theme.color.secondary
        );
    }
}
