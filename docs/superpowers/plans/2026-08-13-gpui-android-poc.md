# GPUI Android PoC 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现 `gpui_android` 平台层 + APK 脚手架，让纯 GPUI demo（PoC-1）与 gpui-component 组件页（PoC-2）在 Android arm64 模拟器上运行并可自动化调试。

**Architecture:** 仿照上游 `gpui_web`/`gpui_linux` 的结构自研 `gpui_android` crate（实现 `gpui::Platform` 系列 trait），渲染与文字完全复用上游 Apache-2.0 的 `gpui_wgpu`（wgpu→Vulkan/GL + cosmic-text）。窗口与生命周期用 `android-activity`（NativeActivity 模式，零 Java 代码），Gradle 仅负责把 `.so` 打进 APK。

**Tech Stack:** Rust (edition 2024) / gpui + gpui_wgpu（zed git 依赖）/ android-activity 0.6 + ndk 0.9 / cargo-ndk / Gradle 8.14.3 + AGP 8.7.3 / arm64 AVD "Medium_Phone_API_35"。

## Global Constraints

- **上游版本锁定**：gpui/gpui_wgpu 以 **不带 rev 的 git URL** 声明（`git = "https://github.com/zed-industries/zed"`），用 **提交 Cargo.lock** 锁定到 commit `a8fafdd7ee36fb3fb98ebbfe5d3be983301d9e74`（本计划所有签名均对照此 rev 核实）。⚠️ 此处与 spec 的"Cargo.toml 中 pin rev"字面不同但达成同一目的（可复现构建）；原因：gpui-component 声明的 gpui 依赖是无 rev 的同 URL，只有源声明完全一致 cargo 才会统一为同一份 gpui，否则出现两份 gpui 类型冲突。
- **gpui 必须 `default-features = false`**（默认 features 含 font-kit/wayland/x11/windows-manifest，Android 上编不过或无意义）；`gpui_wgpu` 不开 `font-kit` feature（走 `cfg(not(feature = "font-kit"))` 回退路径）。
- **禁止复制 AGPL（itsbalamurali/gpui-mobile）与无 license（Zdroid）代码**，只可参考思路。
- **不使用 `gpui::queue`**（`PriorityQueueSender/Receiver` 在 gpui.rs:42/142 被门控为 windows|linux|wasm|test，Android 下不编译）；调度器用 `flume` 通道，PoC 忽略优先级（FIFO），代码中注释说明。
- **不 fork zed**：所有适配都在本仓库内完成。
- 环境（已核实）：NDK `25.1.8937393`，build-tools 35.0.0，AVD `Medium_Phone_API_35`（arm64-v8a, API 35, Play Store 镜像），JAVA_HOME 用 Android Studio JBR 17，Gradle 用 `~/.gradle/wrapper/dists/gradle-8.14.3-bin/*/gradle-8.14.3/bin/gradle`，Rust android 四 target 已装，`cargo-ndk` 需安装。
- minSdk 30，PoC 仅 arm64-v8a，单窗口。
- 遵守 `.refs/zed/CLAUDE.md` 的 Rust 规范：不用 `unwrap()`（测试除外）、不写 `mod.rs`、lib 根用具名文件（`[lib] path = "src/gpui_android.rs"`）、错误不得 `let _ =` 静默丢弃。
- 每个 task 结束必须 `git commit`。构建/部署一律先 `source scripts/env.sh`。

**关键上游签名备忘**（rev a8fafdd 已核实，实现时以编译器为准）：
- `Application::with_platform(platform: Rc<dyn Platform>) -> Self`；`run(self, on_finish_launching: impl 'static + FnOnce(&mut App))`
- `WgpuRenderer::new<W>(gpu_context: GpuContext, window: &W, config: WgpuSurfaceConfig, compositor_gpu: Option<CompositorGpuHint>) -> anyhow::Result<Self>`，`W: HasWindowHandle + HasDisplayHandle + Debug + Send + Sync + Clone + 'static`
- `pub type GpuContext = Rc<RefCell<Option<WgpuContext>>>`；`WgpuSurfaceConfig { size: Size<DevicePixels>, transparent: bool, preferred_present_mode: Option<wgpu::PresentMode> }`
- 面向 Android 的现成手段：`WgpuRenderer::unconfigure_surface()`（TerminateWindow 用）、`replace_surface(&W, WgpuSurfaceConfig, &wgpu::Instance)`（InitWindow 重建用，instance 必须与创建 device 的同一个）、`CosmicTextSystem::new_without_system_fonts(fallback: &str)` + `add_fonts(Vec<Cow<'static, [u8]>>)`
- `PlatformDispatcher` 必须实现：`is_main_thread`、`dispatch`、`dispatch_on_main_thread`、`dispatch_after`、`spawn_realtime`（其余有默认实现）
- android-activity 0.6：`android_main(app: AndroidApp)` 入口；`app.poll_events(Some(timeout), |event| ...)`；`PollEvent::{Wake, Timeout, Main(MainEvent)}`；`MainEvent::{InitWindow, TerminateWindow, WindowResized, ContentRectChanged, ConfigChanged, GainedFocus, LostFocus, Start, Resume{..}, Pause, Stop, SaveState{..}, Destroy, LowMemory, InputAvailable, InsetsChanged, RedrawNeeded, ..}`（non_exhaustive，必须留通配分支）；`app.native_window() -> Option<NativeWindow>`（仅 InitWindow~TerminateWindow 之间 Some）；`app.create_waker() -> AndroidAppWaker`（`.wake()`）；`app.input_events_iter()`（lending iterator，`iter.next(|event| InputStatus) -> bool`）；`app.config().density() -> Option<u32>`（dpi/160=scale）；`MotionEvent::{action() -> MotionAction, pointers(), pointer_index()}`，`Pointer::{x(), y()}`

---

### Task 1: Workspace 脚手架与工具链

**Files:**
- Create: `Cargo.toml`（workspace 根）
- Create: `.gitignore`
- Create: `scripts/env.sh`
- Create: `crates/gpui_android/Cargo.toml`、`crates/gpui_android/src/gpui_android.rs`（最小骨架）

**Interfaces:**
- Produces: workspace 依赖别名 `gpui`、`gpui_wgpu`、`android-activity`、`ndk`、`flume` 等，后续所有 task 经 `xxx.workspace = true` 使用；`scripts/env.sh` 导出 `ANDROID_HOME`/`ANDROID_NDK_HOME`/`JAVA_HOME`/`GRADLE_BIN` 与 PATH。

- [ ] **Step 1: 环境预检与 cargo-ndk 安装**

```bash
rustc --version   # 需 ≥ 1.85（edition 2024 + android-activity MSRV）
cargo install cargo-ndk
cargo ndk --version
```
Expected: cargo-ndk 安装成功。若 rustc < 1.85：`rustup update stable`。

- [ ] **Step 2: 写 workspace 根 `Cargo.toml`**

```toml
[workspace]
resolver = "2"
members = ["crates/gpui_android", "crates/hello_gpui"]

[workspace.dependencies]
# 无 rev 的 git 源声明（与 gpui-component 的声明一致以便 cargo 统一源）；
# 具体 commit 由提交进仓库的 Cargo.lock 锁定。
gpui = { git = "https://github.com/zed-industries/zed", default-features = false }
gpui_wgpu = { git = "https://github.com/zed-industries/zed" }

anyhow = "1"
log = "0.4"
flume = "0.12"
futures = "0.3"
uuid = { version = "1", features = ["v4"] }
raw-window-handle = "0.6"
# ndk 版本必须与 android-activity 0.6 内部使用的一致（0.9），否则出现两个 NativeWindow 类型
android-activity = { version = "0.6", features = ["native-activity"] }
ndk = "0.9"
android_logger = "0.14"

gpui_android = { path = "crates/gpui_android" }
```

- [ ] **Step 3: 写 `.gitignore`**

```gitignore
/target
/.refs/
android/.gradle/
android/build/
android/app/build/
android/app/src/main/jniLibs/
android/local.properties
*.log
```

- [ ] **Step 4: 写 `scripts/env.sh`**

```bash
#!/bin/bash
# 构建/部署脚本共用环境。用法: source scripts/env.sh
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/25.1.8937393"
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
_gradle_glob=("$HOME/.gradle/wrapper/dists/gradle-8.14.3-bin"/*/gradle-8.14.3/bin/gradle)
export GRADLE_BIN="${_gradle_glob[0]}"
export AVD_NAME="Medium_Phone_API_35"
export APP_ID="dev.youkai.hellogpui"
```

- [ ] **Step 5: 建 `gpui_android` 最小骨架（先只依赖 gpui，验证解析）**

`crates/gpui_android/Cargo.toml`：
```toml
[package]
name = "gpui_android"
version = "0.1.0"
edition = "2024"
license = "Apache-2.0"

[lib]
path = "src/gpui_android.rs"

[dependencies]
gpui.workspace = true
anyhow.workspace = true
log.workspace = true
flume.workspace = true
futures.workspace = true
uuid.workspace = true

[target.'cfg(target_os = "android")'.dependencies]
gpui_wgpu.workspace = true
android-activity.workspace = true
ndk.workspace = true
raw-window-handle.workspace = true
```

`crates/gpui_android/src/gpui_android.rs`：
```rust
//! GPUI 的 Android 平台层：以 android-activity(NativeActivity) 驱动主循环，
//! 渲染与文字复用上游 gpui_wgpu。桌面窗口专属能力（多窗口、剪贴板、菜单、
//! 文件对话框等）在 PoC 阶段以合理默认值 stub。

pub mod events;
```
以及占位 `crates/gpui_android/src/events.rs`（Task 2 填充）：
```rust
//! 触摸事件 → GPUI 鼠标事件的纯逻辑映射，可在 host 上单元测试。
```
注意：`members` 里的 `crates/hello_gpui` 目录尚不存在，先建空壳避免 workspace 解析失败：
`crates/hello_gpui/Cargo.toml`：
```toml
[package]
name = "hello_gpui"
version = "0.1.0"
edition = "2024"
license = "Apache-2.0"

[lib]
crate-type = ["cdylib"]
path = "src/hello_gpui.rs"

[dependencies]
log.workspace = true

[target.'cfg(target_os = "android")'.dependencies]
gpui.workspace = true
gpui_android.workspace = true
android-activity.workspace = true
android_logger.workspace = true
```
`crates/hello_gpui/src/hello_gpui.rs`：
```rust
//! PoC demo 入口（Task 6 实现 android_main）。
```

- [ ] **Step 6: 生成并锁定 Cargo.lock 到已核实的 zed commit**

```bash
cargo generate-lockfile
cargo update -p gpui --precise a8fafdd7ee36fb3fb98ebbfe5d3be983301d9e74
grep -A2 'name = "gpui"' Cargo.lock | head -5
```
Expected: lock 中 gpui source 为 `git+https://github.com/zed-industries/zed#a8fafdd7...`。首次拉取 zed git 仓库较大（数 GB），耐心等待。

- [ ] **Step 7: 验证 host 编译 + android target 解析**

```bash
cargo check -p gpui_android
cargo ndk -t arm64-v8a check -p gpui_android
```
Expected: 两者 PASS（骨架无实际代码，主要验证 gpui/gpui_wgpu 在 android target 下能编译——这是本计划最大的前置风险，尽早暴露）。若 gpui 传递依赖在 android 下编译失败：记录错误，优先通过调整 feature 解决，不得擅自 fork zed，解决不了则停下向用户汇报。

- [ ] **Step 8: Commit**

```bash
git add -A && git commit -m "chore: workspace 脚手架，锁定 zed 依赖至 a8fafdd"
```

---

### Task 2: 触摸映射纯逻辑模块 events.rs（TDD）

**Files:**
- Modify: `crates/gpui_android/src/events.rs`
- Test: 同文件 `#[cfg(test)] mod tests`

**Interfaces:**
- Produces: `pub enum TouchAction { Down, Move, Up, Cancel }`；`pub struct TouchState`（`Default`）；`pub fn TouchState::map(&mut self, action: TouchAction, physical_x: f32, physical_y: f32, scale: f32) -> Vec<gpui::PlatformInput>`；`pub fn TouchState::last_position(&self) -> gpui::Point<gpui::Pixels>`。Task 4 的 `AndroidWindowInner::handle_motion` 与 Task 5 的输入泵消费。
- 设计约束：单指→左键映射（spec §5）；不依赖 android-activity 类型（host 可测）。

- [ ] **Step 1: 写失败测试**

```rust
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
```

- [ ] **Step 2: 运行确认失败**

Run: `cargo test -p gpui_android`
Expected: FAIL（`TouchAction`/`TouchState` 未定义）。

- [ ] **Step 3: 最小实现**

```rust
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
```

- [ ] **Step 4: 运行确认通过**

Run: `cargo test -p gpui_android`
Expected: 5 tests PASS。

- [ ] **Step 5: Commit**

```bash
git add -A && git commit -m "feat(gpui_android): 触摸→鼠标事件纯逻辑映射（host 单测）"
```

---

### Task 3: 定时器队列（TDD）+ AndroidDispatcher

**Files:**
- Create: `crates/gpui_android/src/timers.rs`（纯逻辑，host 可测）
- Create: `crates/gpui_android/src/dispatcher.rs`（android-gated）
- Modify: `crates/gpui_android/src/gpui_android.rs`

**Interfaces:**
- Consumes: `gpui::{PlatformDispatcher, Priority, RunnableVariant}`、`android_activity::AndroidAppWaker`
- Produces: `pub struct TimerQueue<T>`：`new()`、`insert(deadline: std::time::Instant, payload: T)`、`next_deadline() -> Option<std::time::Instant>`、`pop_due(now: std::time::Instant) -> Vec<T>`；`pub(crate) struct AndroidDispatcher`：`pub fn new(waker: AndroidAppWaker) -> (Arc<Self>, flume::Receiver<RunnableVariant>)`（receiver 由 Task 5 主循环 drain）。

- [ ] **Step 1: 写 TimerQueue 失败测试（timers.rs）**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn pop_due_returns_only_expired_in_deadline_order() {
        let mut queue = TimerQueue::new();
        let now = Instant::now();
        queue.insert(now + Duration::from_millis(50), "late");
        queue.insert(now + Duration::from_millis(10), "early");
        queue.insert(now + Duration::from_millis(500), "future");
        let due = queue.pop_due(now + Duration::from_millis(100));
        assert_eq!(due, vec!["early", "late"]);
        assert_eq!(queue.next_deadline(), Some(now + Duration::from_millis(500)));
    }

    #[test]
    fn empty_queue_has_no_deadline() {
        let queue: TimerQueue<()> = TimerQueue::new();
        assert_eq!(queue.next_deadline(), None);
    }
}
```

- [ ] **Step 2: 运行确认失败**

Run: `cargo test -p gpui_android timers`
Expected: FAIL（TimerQueue 未定义）。

- [ ] **Step 3: 实现 TimerQueue**

```rust
//! dispatch_after 用的最小堆定时队列；纯逻辑，host 可测。
use std::collections::BinaryHeap;
use std::time::Instant;

struct Entry<T> {
    deadline: Instant,
    sequence: u64,
    payload: T,
}

impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline && self.sequence == other.sequence
    }
}
impl<T> Eq for Entry<T> {}
impl<T> PartialOrd for Entry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Entry<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap 是最大堆，反转得到最早 deadline 优先。
        other
            .deadline
            .cmp(&self.deadline)
            .then(other.sequence.cmp(&self.sequence))
    }
}

pub struct TimerQueue<T> {
    heap: BinaryHeap<Entry<T>>,
    next_sequence: u64,
}

impl<T> TimerQueue<T> {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new(), next_sequence: 0 }
    }

    pub fn insert(&mut self, deadline: Instant, payload: T) {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.heap.push(Entry { deadline, sequence, payload });
    }

    pub fn next_deadline(&self) -> Option<Instant> {
        self.heap.peek().map(|entry| entry.deadline)
    }

    pub fn pop_due(&mut self, now: Instant) -> Vec<T> {
        let mut due = Vec::new();
        while self.heap.peek().is_some_and(|entry| entry.deadline <= now) {
            if let Some(entry) = self.heap.pop() {
                due.push(entry.payload);
            }
        }
        due
    }
}

impl<T> Default for TimerQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: 运行确认通过**

Run: `cargo test -p gpui_android timers`
Expected: 2 tests PASS。

- [ ] **Step 5: 实现 AndroidDispatcher（dispatcher.rs，模式对照 LinuxDispatcher/WebDispatcher）**

```rust
use crate::timers::TimerQueue;
use android_activity::AndroidAppWaker;
use gpui::{PlatformDispatcher, Priority, RunnableVariant};
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

const MIN_BACKGROUND_THREADS: usize = 2;

struct TimerTask {
    deadline: Instant,
    runnable: RunnableVariant,
}

pub(crate) struct AndroidDispatcher {
    main_thread_id: thread::ThreadId,
    main_sender: flume::Sender<RunnableVariant>,
    background_sender: flume::Sender<RunnableVariant>,
    timer_sender: flume::Sender<TimerTask>,
    waker: AndroidAppWaker,
    _background_threads: Vec<thread::JoinHandle<()>>,
}

// AndroidAppWaker 按文档可跨线程使用；若编译器认定 !Sync，则改为 Mutex<AndroidAppWaker> 包裹。
unsafe impl Send for AndroidDispatcher {}
unsafe impl Sync for AndroidDispatcher {}

impl AndroidDispatcher {
    /// 必须在 android_main 线程上创建（该线程被视为 GPUI 主线程）。
    /// 返回的 receiver 由平台主循环在每次 poll 后 drain。
    pub(crate) fn new(waker: AndroidAppWaker) -> (Arc<Self>, flume::Receiver<RunnableVariant>) {
        let (main_sender, main_receiver) = flume::unbounded::<RunnableVariant>();
        let (background_sender, background_receiver) = flume::unbounded::<RunnableVariant>();
        let thread_count = thread::available_parallelism()
            .map_or(MIN_BACKGROUND_THREADS, |n| n.get().max(MIN_BACKGROUND_THREADS));

        let mut background_threads = (0..thread_count)
            .map(|i| {
                let receiver = background_receiver.clone();
                thread::Builder::new()
                    .name(format!("gpui-worker-{i}"))
                    .spawn(move || {
                        while let Ok(runnable) = receiver.recv() {
                            runnable.run();
                        }
                    })
                    .expect("failed to spawn background worker thread")
            })
            .collect::<Vec<_>>();

        let (timer_sender, timer_receiver) = flume::unbounded::<TimerTask>();
        let timer_thread = thread::Builder::new()
            .name("gpui-timer".to_owned())
            .spawn(move || run_timer_loop(timer_receiver))
            .expect("failed to spawn timer thread");
        background_threads.push(timer_thread);

        let dispatcher = Arc::new(Self {
            main_thread_id: thread::current().id(),
            main_sender,
            background_sender,
            timer_sender,
            waker,
            _background_threads: background_threads,
        });
        (dispatcher, main_receiver)
    }
}

fn run_timer_loop(receiver: flume::Receiver<TimerTask>) {
    let mut queue = TimerQueue::new();
    loop {
        let recv_result = match queue.next_deadline() {
            Some(deadline) => {
                let timeout = deadline.saturating_duration_since(Instant::now());
                if timeout.is_zero() {
                    Err(flume::RecvTimeoutError::Timeout)
                } else {
                    receiver.recv_timeout(timeout)
                }
            }
            None => receiver.recv().map_err(|_| flume::RecvTimeoutError::Disconnected),
        };
        match recv_result {
            Ok(task) => queue.insert(task.deadline, task.runnable),
            Err(flume::RecvTimeoutError::Timeout) => {}
            Err(flume::RecvTimeoutError::Disconnected) => break,
        }
        // 与 LinuxDispatcher 一致：到期的 runnable 直接在定时器线程上运行。
        for runnable in queue.pop_due(Instant::now()) {
            runnable.run();
        }
    }
}

impl PlatformDispatcher for AndroidDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    // PoC 忽略 Priority（FIFO）：gpui::queue 的优先级通道在 Android 上未编译，
    // 且 PoC 负载单一；二期如需优先级可自建多级队列。
    fn dispatch(&self, runnable: RunnableVariant, _priority: Priority) {
        if let Err(error) = self.background_sender.send(runnable) {
            // 后台线程池仅在进程退出时消失；参照 LinuxDispatcher，
            // runnable 可能持有 !Send 状态，禁止在错误路径 drop。
            std::mem::forget(error.into_inner());
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        match self.main_sender.send(runnable) {
            Ok(()) => self.waker.wake(),
            // main receiver 已被 drop 意味着应用正在退出；同上禁止 drop。
            Err(error) => std::mem::forget(error.into_inner()),
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let task = TimerTask { deadline: Instant::now() + duration, runnable };
        if let Err(error) = self.timer_sender.send(task) {
            std::mem::forget(error.into_inner());
        }
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        // Android 上不设实时调度优先级（需要额外权限），普通线程即可。
        if let Err(error) = thread::Builder::new().name("gpui-realtime".to_owned()).spawn(f) {
            log::error!("failed to spawn realtime thread: {error}");
        }
    }
}
```

- [ ] **Step 6: 挂进 lib 根并编译检查**

`gpui_android.rs` 改为：
```rust
pub mod events;
pub mod timers;

#[cfg(target_os = "android")]
mod dispatcher;
```
Run: `cargo test -p gpui_android && cargo ndk -t arm64-v8a check -p gpui_android`
Expected: host 测试 PASS；android check PASS（dispatcher 编译成功；若 `unsafe impl Send/Sync` 与实际情况冲突按注释调整）。

- [ ] **Step 7: Commit**

```bash
git add -A && git commit -m "feat(gpui_android): AndroidDispatcher（flume 通道 + ALooper 唤醒 + 定时器线程）"
```

---

### Task 4: AndroidDisplay + AndroidWindow

**Files:**
- Create: `crates/gpui_android/src/display.rs`
- Create: `crates/gpui_android/src/window.rs`
- Modify: `crates/gpui_android/src/gpui_android.rs`

**Interfaces:**
- Consumes: Task 2 的 `TouchState`/`TouchAction`；`gpui_wgpu::{GpuContext, WgpuRenderer, WgpuSurfaceConfig}`；`ndk::native_window::NativeWindow`
- Produces（Task 5 消费）:
  - `pub(crate) struct AndroidDisplay`：`new(size_physical: (i32, i32), scale: f32) -> Self`，实现 `PlatformDisplay`
  - `pub(crate) struct AndroidWindowInner`（`Rc` 共享）：`new(native_window: NativeWindow, scale: f32, gpu_context: &GpuContext, display: Rc<AndroidDisplay>) -> anyhow::Result<Rc<Self>>`；`handle_surface_destroyed(&self)`；`handle_surface_recreated(&self, native_window: NativeWindow) -> anyhow::Result<()>`；`handle_resize(&self, physical: (i32, i32), scale: f32)`；`handle_touch(&self, action: crate::events::TouchAction, physical_x: f32, physical_y: f32)`；`fire_request_frame(&self)`；`set_active(&self, active: bool)`；`surface_alive(&self) -> bool`
  - `pub(crate) struct AndroidWindow`（`inner: Rc<AndroidWindowInner>`），实现 `PlatformWindow + HasWindowHandle + HasDisplayHandle`

- [ ] **Step 1: 写 display.rs**

```rust
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
```

- [ ] **Step 2: 写 window.rs（骨干代码如下；PlatformWindow 全部必需方法对照 WebWindow 的 stub 策略）**

```rust
use crate::display::{AndroidDisplay, logical_size};
use crate::events::{TouchAction, TouchState};
use gpui::{
    AnyWindowHandle, Bounds, Capslock, DevicePixels, DispatchEventResult, GpuSpecs, Modifiers,
    Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow,
    Point, PromptButton, PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea,
};
use gpui_wgpu::{GpuContext, WgpuRenderer, WgpuSurfaceConfig, wgpu};
use ndk::native_window::NativeWindow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// 供 wgpu 建 surface 的 raw-window-handle 包装。
/// WgpuRenderer::new 要求 Clone + Debug + Send + Sync + 'static。
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
                |callback| callback(input.clone()),
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
    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}
}
```

- [ ] **Step 3: 挂进 lib 根并编译检查**

`gpui_android.rs` 增加：
```rust
#[cfg(target_os = "android")]
mod display;
#[cfg(target_os = "android")]
mod window;
```
Run: `cargo ndk -t arm64-v8a check -p gpui_android`
Expected: PASS。常见修正点（编译器为准）：`NativeWindow::ptr()` 的返回类型转换、`PlatformInput` 是否 `Clone`（若不是，`handle_touch` 里改为逐个构造）、`raw_window_handle` 各类型路径。

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "feat(gpui_android): AndroidDisplay + AndroidWindow（wgpu surface 生命周期 + 触摸分发）"
```

---

### Task 5: AndroidPlatform 主循环 + 文字系统

**Files:**
- Create: `crates/gpui_android/src/platform.rs`
- Modify: `crates/gpui_android/src/gpui_android.rs`（导出 `AndroidPlatform`）

**Interfaces:**
- Consumes: Task 3 `AndroidDispatcher::new`、Task 4 `AndroidWindowInner`/`AndroidWindow`/`AndroidDisplay`
- Produces: `pub struct AndroidPlatform`：`pub fn new(app: AndroidApp) -> Rc<Self>`，实现 `gpui::Platform` 全部必需方法。Task 6 demo 以 `Application::with_platform(AndroidPlatform::new(app))` 使用。

- [ ] **Step 1: 写 platform.rs**

结构体与构造：
```rust
use crate::dispatcher::AndroidDispatcher;
use crate::display::AndroidDisplay;
use crate::events::TouchAction;
use crate::window::{AndroidWindow, AndroidWindowInner};
use android_activity::input::{InputEvent, InputStatus, MotionAction};
use android_activity::{AndroidApp, MainEvent, PollEvent};
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
    dispatcher: Arc<AndroidDispatcher>,
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
            dispatcher,
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
```

主循环（`Platform::run` 与事件处理）：
```rust
impl AndroidPlatform {
    fn handle_main_event(
        &self,
        event: MainEvent,
        launched: &mut bool,
        on_finish_launching: &mut Option<Box<dyn FnOnce()>>,
    ) {
        match event {
            MainEvent::InitWindow { .. } => {
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
```
注：若编译报 `Platform` 还有本计划未列出的必需方法（上游可能已前进），按 WebPlatform 同名实现的 stub 策略补齐。

- [ ] **Step 2: 挂进 lib 根**

`gpui_android.rs` 最终形态：
```rust
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
```

- [ ] **Step 3: 编译检查**

Run: `cargo ndk -t arm64-v8a check -p gpui_android && cargo test -p gpui_android`
Expected: 双双 PASS。

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "feat(gpui_android): AndroidPlatform 主循环、系统字体加载与 Platform trait 实现"
```

---

### Task 6: hello_gpui demo（PoC-1 应用层）

**Files:**
- Modify: `crates/hello_gpui/src/hello_gpui.rs`

**Interfaces:**
- Consumes: `gpui_android::AndroidPlatform`、`gpui::Application::with_platform`
- Produces: `libhello_gpui.so`（cdylib，导出 `android_main`）。点击计数按钮时输出日志行 `count incremented to N`（Task 8 交互验证依赖此行）。

- [ ] **Step 1: 写 demo**

```rust
//! PoC-1：彩色矩形 + 中英文文字 + 触摸计数器。

#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use gpui::{
    App, Application, Context, IntoElement, ParentElement, Render, Styled, Window, WindowOptions,
    div, px, rgb,
};
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
```
注：`on_click`/`id` 需要 `InteractiveElement`/`StatefulInteractiveElement` trait，按编译器提示从 `gpui::prelude::*` 或具名导入补齐。

- [ ] **Step 2: 构建 .so**

```bash
source scripts/env.sh
cargo ndk -t arm64-v8a --platform 30 -o android/app/src/main/jniLibs build -p hello_gpui --release
ls -la android/app/src/main/jniLibs/arm64-v8a/
```
Expected: 出现 `libhello_gpui.so`（首次全量编译 gpui 较久）。

- [ ] **Step 3: Commit**

```bash
git add -A && git commit -m "feat(hello_gpui): PoC-1 demo（矩形+中英文+计数器）与 android_main 入口"
```

---

### Task 7: Android Gradle 打包工程

**Files:**
- Create: `android/settings.gradle`、`android/build.gradle`、`android/gradle.properties`
- Create: `android/app/build.gradle`、`android/app/src/main/AndroidManifest.xml`

**Interfaces:**
- Consumes: Task 6 产出的 `android/app/src/main/jniLibs/arm64-v8a/libhello_gpui.so`
- Produces: `android/app/build/outputs/apk/debug/app-debug.apk`；applicationId `dev.youkai.hellogpui`，启动组件 `android.app.NativeActivity`。

- [ ] **Step 1: 写 Gradle 工程（无 Java 源码）**

`android/settings.gradle`：
```groovy
pluginManagement {
    repositories { google(); mavenCentral(); gradlePluginPortal() }
}
dependencyResolutionManagement {
    repositories { google(); mavenCentral() }
}
rootProject.name = "hello_gpui"
include ':app'
```
`android/build.gradle`：
```groovy
plugins {
    id 'com.android.application' version '8.7.3' apply false
}
```
`android/gradle.properties`：
```properties
android.useAndroidX=false
org.gradle.jvmargs=-Xmx2g
```
`android/app/build.gradle`：
```groovy
plugins { id 'com.android.application' }

android {
    namespace 'dev.youkai.hellogpui'
    compileSdk 35
    defaultConfig {
        applicationId "dev.youkai.hellogpui"
        minSdk 30
        targetSdk 35
        versionCode 1
        versionName "0.1"
    }
}
```
`android/app/src/main/AndroidManifest.xml`：
```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <application android:label="Hello GPUI" android:hasCode="false">
        <activity
            android:name="android.app.NativeActivity"
            android:exported="true"
            android:configChanges="orientation|screenSize|screenLayout|keyboardHidden|density|uiMode">
            <meta-data android:name="android.app.lib_name" android:value="hello_gpui" />
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
```
关键点：`android:hasCode="false"`（纯 native）；`lib_name` 必须与 cdylib 名一致（`hello_gpui` → `libhello_gpui.so`）；`configChanges` 声明旋转等自行处理，避免 Activity 重建。

- [ ] **Step 2: 打包**

```bash
source scripts/env.sh
cd android && "$GRADLE_BIN" assembleDebug && cd ..
ls -la android/app/build/outputs/apk/debug/app-debug.apk
```
Expected: APK 生成（首次需下载 AGP 依赖）。若 AGP 8.7.3 与 Gradle 8.14.3 不兼容报错，将 AGP 版本降至错误信息提示的兼容版本（如 8.5.x）。

- [ ] **Step 3: Commit**

```bash
git add -A && git commit -m "build(android): NativeActivity Gradle 打包工程"
```

---

### Task 8: 部署脚本 + PoC-1 模拟器验收

**Files:**
- Create: `scripts/build.sh`、`scripts/run.sh`、`scripts/logs.sh`、`scripts/screenshot.sh`

**Interfaces:**
- Consumes: Task 1 `env.sh`、Task 7 APK
- Produces: 一键构建-部署-截图闭环；PoC-1 验收证据（截图 + logcat）。

- [ ] **Step 1: 写脚本**

`scripts/build.sh`：
```bash
#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
cargo ndk -t arm64-v8a --platform 30 -o android/app/src/main/jniLibs build -p hello_gpui --release
(cd android && "$GRADLE_BIN" assembleDebug)
echo "APK: android/app/build/outputs/apk/debug/app-debug.apk"
```
`scripts/run.sh`：
```bash
#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
if ! adb get-state >/dev/null 2>&1; then
    echo "starting emulator $AVD_NAME ..."
    nohup emulator -avd "$AVD_NAME" -netdelay none -netspeed full >/tmp/emulator.log 2>&1 &
    adb wait-for-device
fi
until [ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; do
    sleep 2
done
adb install -r android/app/build/outputs/apk/debug/app-debug.apk
adb logcat -c
adb shell am start -n "$APP_ID/android.app.NativeActivity"
```
`scripts/logs.sh`：
```bash
#!/bin/bash
cd "$(dirname "$0")/.."
source scripts/env.sh
adb logcat -d -v time | grep -E "hello_gpui|RustStdoutStderr|wgpu|gpui|AndroidRuntime|libc.*Fatal" || true
```
`scripts/screenshot.sh`：
```bash
#!/bin/bash
cd "$(dirname "$0")/.."
source scripts/env.sh
out="${1:-/tmp/gpui_screenshot.png}"
adb exec-out screencap -p > "$out"
echo "$out"
```
`chmod +x scripts/*.sh`

- [ ] **Step 2: 端到端首跑**

```bash
scripts/build.sh && scripts/run.sh
sleep 5 && scripts/logs.sh
scripts/screenshot.sh /tmp/poc1_boot.png
```
Expected: logcat 出现 `android_main starting`、wgpu 的 `Selected GPU adapter: ...`（Vulkan 或 GL 均可），无 `PANIC`/`Fatal signal`；用 Read 工具查看截图，画面为深蓝背景 + 橙色圆角矩形 + 白色中英文文字 + 蓝色按钮。

**排障指引（按症状）：**
- 启动即闪退、logcat 有 `library "libhello_gpui.so" not found` → manifest `lib_name` 与 so 名不一致或 jniLibs 目录层级错误。
- `PANIC` 含 surface/adapter → 查 wgpu 初始化日志；若 Vulkan 失败未回退，确认 gpui_wgpu instance 的 Backends 含 GL；模拟器可加 `-gpu host` 或 `-gpu swiftshader_indirect` 重试。
- 画面全黑但无 panic → 检查 `fire_request_frame` 是否被调用（加临时日志）、`draw` 是否执行、`update_drawable_size` 尺寸是否为 0。
- 文字缺失但矩形正常 → 字体加载日志（`skipping font ...`），确认模拟器 `/system/fonts` 下实际文件名并补进 `SYSTEM_FONT_PATHS`。

- [ ] **Step 3: 交互验证（触摸计数器）**

```bash
# 从截图估算按钮中心物理坐标（Medium Phone 1080x2400，按钮约在屏幕中下部）
adb shell input tap 540 1400   # 坐标以截图实测为准
sleep 1 && scripts/logs.sh | grep "count incremented"
scripts/screenshot.sh /tmp/poc1_after_tap.png
```
Expected: 日志出现 `count incremented to 1`；截图中计数文字变为 `Count: 1`。若未命中，按截图逐步校正 tap 坐标。

- [ ] **Step 4: 生命周期与旋转验证**

```bash
adb shell input keyevent KEYCODE_HOME && sleep 2
adb shell am start -n "$APP_ID/android.app.NativeActivity" && sleep 3
scripts/logs.sh | grep -cE "PANIC|Fatal" || echo "lifecycle OK"
scripts/screenshot.sh /tmp/poc1_resumed.png
adb shell settings put system accelerometer_rotation 0
adb shell settings put system user_rotation 1 && sleep 3
scripts/screenshot.sh /tmp/poc1_rotated.png
adb shell settings put system user_rotation 0
```
Expected: 切后台/回前台后无 PANIC/Fatal，画面恢复（验证 TerminateWindow→InitWindow 的 surface 重建链路）；旋转后截图为横屏且内容按新尺寸铺满。

- [ ] **Step 5: Commit（PoC-1 达成）**

```bash
git add -A && git commit -m "feat: PoC-1 通过——GPUI demo 在 Android 模拟器渲染、交互、生命周期均正常"
```

---

### Task 9: PoC-2 gpui-component 组件页

**Files:**
- Create: `crates/component_demo/Cargo.toml`、`crates/component_demo/src/component_demo.rs`
- Modify: `Cargo.toml`（workspace members + gpui-component 依赖）
- Modify: `android/app/src/main/AndroidManifest.xml`、`android/app/build.gradle`（如需改 lib_name 指向新 demo，改为 `component_demo`）

**Interfaces:**
- Consumes: Task 5 `AndroidPlatform`、gpui-component（git 依赖）
- Produces: 含 Button / Input（仅显示）/ List 的组件页 APK；验收证据同 Task 8 模式。

- [ ] **Step 1: 接入 gpui-component 依赖**

workspace `Cargo.toml` 增加：
```toml
gpui-component = { git = "https://github.com/longbridge/gpui-component" }
```
```bash
cargo update
cargo tree -p gpui --target aarch64-linux-android -i 2>&1 | head -20
```
Expected: 只有**一份** gpui（gpui-component 与本 workspace 统一到同一 commit）。注意：`cargo update` 会把 gpui 前移到 zed main 最新 commit——这会偏离 a8fafdd。**优先尝试**：`cargo update -p gpui --precise a8fafdd7ee36fb3fb98ebbfe5d3be983301d9e74`；若 gpui-component 需要更新的 gpui API 导致编译失败，则允许整体前移到 gpui-component 兼容的 commit，并重跑 Task 8 的全部验证（记录新 commit 于提交信息）。

- [ ] **Step 2: 建 component_demo crate**

`crates/component_demo/Cargo.toml` 仿照 hello_gpui，增加 `gpui-component.workspace = true`。
`component_demo.rs` 结构同 hello_gpui 的 android_main，视图替换为组件页（以下代码基于 gpui-component README，**实现时以 `gh api` 查看其 `examples/` 目录校正 API**——初始化通常需要 `gpui_component::init(cx)` 与主题设置）：
```rust
use gpui_component::button::Button;
use gpui_component::input::TextInput;
use gpui_component::list::List;
// 视图 render 中：
// v_flex().gap_4().p_4()
//     .child(Button::new("btn").label("Click me").on_click(...日志计数...))
//     .child(TextInput 只读展示)
//     .child(List 渲染 20 行文本，可滚动)
```
lib_name 切换：`AndroidManifest.xml` 的 `android.app.lib_name` 改为 `component_demo`，applicationId 保持不变。

- [ ] **Step 3: 排查桌面专属 API**

```bash
cargo ndk -t arm64-v8a check -p component_demo 2>&1 | tee /tmp/poc2_check.log
```
Expected: PASS。若 gpui-component 引用了 Android 上不存在的平台能力（如剪贴板增强、窗口装饰），demo 中只选用平台无关组件绕开；仍编不过则记录并向用户汇报取舍。

- [ ] **Step 4: 构建部署验证（复用 Task 8 脚本，build.sh 中 `-p hello_gpui` 参数化为 `-p ${DEMO_CRATE:-hello_gpui}`）**

```bash
DEMO_CRATE=component_demo scripts/build.sh && scripts/run.sh
sleep 5 && scripts/logs.sh
scripts/screenshot.sh /tmp/poc2_components.png
# 点击 Button（坐标按截图实测）、swipe 滚动 List：
adb shell input tap 540 600
adb shell input swipe 540 1800 540 800 300
scripts/screenshot.sh /tmp/poc2_after_scroll.png
```
Expected: 截图中 Button/Input/List 渲染正确；tap 后日志有点击记录；swipe 后 List 滚动位置变化。

- [ ] **Step 5: Commit（PoC-2 达成）**

```bash
git add -A && git commit -m "feat: PoC-2 通过——gpui-component 组件页在 Android 可渲染可交互"
```

---

## Self-Review 记录

1. **Spec 覆盖**：§4 仓库布局→Task 1/7/8；§5 四组件与数据流→Task 3/4/5，触摸映射→Task 2；§6 构建调试→Task 8（panic hook 在 Task 6）；§7 PoC-1/PoC-2 验收→Task 8/9；§8 风险对策→内嵌于各 task 排障指引。无缺口。
2. **Spec 偏差（已声明）**：rev 锁定由"Cargo.toml 写 rev"改为"无 rev git 源 + 提交 Cargo.lock 锁定"，原因见 Global Constraints（gpui-component 源统一），达成同一可复现目标。
3. **类型一致性**：`TouchState::map` 签名在 Task 2 定义、Task 4 `handle_touch` 消费一致；`AndroidDispatcher::new` 返回 `(Arc<Self>, Receiver)` 在 Task 5 构造函数消费一致；`AndroidWindowInner` 方法名在 Task 4 定义与 Task 5 主循环调用一致；`GpuContext` 均为 `gpui_wgpu` 导出类型。
4. **占位符扫描**：Task 9 Step 2 的 gpui-component 视图代码依赖其未核实的 API，已明确给出"以 examples/ 校正"的具体动作而非 TBD；其余步骤均含完整代码/命令。
