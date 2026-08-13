# GPUI Android 开发框架 — PoC 设计（第一期）

日期：2026-08-13
状态：已批准
范围：第一期 PoC + 脚手架；调通后进入第二期（可复用的完整框架）

## 1. 背景与调研结论

- GPUI（Zed 的 UI 框架）官方平台层仅有 `gpui_macos` / `gpui_linux` / `gpui_windows` / `gpui_web`（WASM），**无 Android 支持**。
- 官方 issue [zed-industries/zed#43207](https://github.com/zed-industries/zed/issues/43207)（"gpui: On Android"）已被 **Closed as not planned** —— 官方明确不做。
- [longbridge/gpui-component](https://github.com/longbridge/gpui-component) 定位为跨平台桌面组件库（macOS/Windows/Linux + Web WASM），不支持移动端。
- 社区先例证明可行性，但代码不可复用：
  - [itsbalamurali/gpui-mobile](https://github.com/itsbalamurali/gpui-mobile)：AGPL-3.0（传染性 license），2026-06 后停更，依赖未发布的 zed git crate。
  - [Dylanmurzello/zed-android-port](https://github.com/Dylanmurzello/zed-android-port)（Zdroid）：无明确 license（NOASSERTION），仅可参考思路。
- 上游 `gpui_wgpu`（Apache-2.0）已提供跨平台 wgpu 渲染器 + cosmic-text/swash 文字系统，`gpui_web` 即基于它实现（仅 8 个源文件）—— 渲染栈无需自研。
- gpui core 的平台专属依赖均以 `target_os` 门控（macos/linux/windows/wasm），`target_os = "android"` 不会命中桌面依赖。

**结论：不支持 Android → 采用方案 A 自研 `gpui_android` 平台层。**

## 2. 目标与非目标

**目标（PoC）**
1. `gpui_android` crate：实现 `gpui::Platform` 系列 trait 的最小可用子集。
2. APK 工程脚手架：Rust → `.so` → APK → 模拟器 的一键构建部署链。
3. 在 Android 模拟器（arm64 AVD）上跑通两级 demo：
   - PoC-1：纯 GPUI（矩形 + 中英文文字 + 触摸计数器）。
   - PoC-2：叠加 gpui-component 组件页（Button / Input 显示 / List）。
4. 全自动调试闭环（logcat + screencap + input 注入），无需人工干预。

**非目标（留给第二期）**
- 输入法（IME）/ 软键盘、剪贴板、多窗口、多指手势与惯性滚动、文件对话框、生产级发布链（签名/ABI 矩阵/上架）。

## 3. 技术路线（方案 A，已批准）

自研 `gpui_android` 平台层 + 复用上游 Apache-2.0 的 `gpui_wgpu` 渲染器。社区 AGPL/无 license 项目仅作架构参考，不拷贝代码。License 干净，与上游架构同构，可演进为第二期完整框架。

被否方案：B（集成 AGPL 的 gpui-mobile —— license 传染且停更）、C（fork Glass-HQ/gpui —— 实际无 Android 支持且偏离上游）。

## 4. 仓库布局

```
GPUI_mobile/
├── Cargo.toml                 # workspace；gpui/gpui_wgpu 指向 zed git（pin rev）
├── crates/
│   ├── gpui_android/          # ★ 平台层 crate（本项目核心产出）
│   └── hello_gpui/            # PoC demo（cdylib，android-activity 入口）
│       └── (二期加 component_demo/)
├── android/                   # 最小 Gradle 工程，仅打包 .so；无 Java 源码
│   └── app/src/main/AndroidManifest.xml  # 声明 android.app.NativeActivity
├── scripts/                   # build.sh / run.sh / logs.sh
└── docs/
```

关键决策：
1. 渲染/文字完全复用 `gpui_wgpu`，照 `gpui_web` 的集成模式。
2. 窗口与生命周期用 `android-activity`（NativeActivity 模式）：PoC 零 Java；二期需 IME 时升级 GameActivity。
3. zed git 依赖 pin 固定 rev（实施第一步锁定为当日 zed main HEAD 的 commit hash，写入 workspace Cargo.toml），保证可复现构建。
4. `gpui_android` 与 `gpui_linux`/`gpui_web` 同构，便于后续独立发布或上游化。

## 5. 平台层组件设计

| 组件 | 实现 trait | PoC 职责 |
|---|---|---|
| `AndroidPlatform` | `gpui::Platform` | 持有 android-activity 主循环；`run()` 事件泵；`open_window()`；文字系统实例化 gpui_wgpu 的 cosmic-text 系统并加载 `/system/fonts` Roboto；桌面专属方法（剪贴板/菜单/对话框等）stub：返回默认值 + log warn |
| `AndroidDispatcher` | `gpui::PlatformDispatcher` | 主线程：管道 + ALooper 唤醒；后台：std::thread 线程池；定时：延时队列 |
| `AndroidWindow` | `gpui::PlatformWindow` | 包装 ANativeWindow（实现 raw-window-handle 供 wgpu 建 surface）；DPI 取 AConfiguration density；request_frame 挂帧回调 |
| `AndroidDisplay` | `gpui::PlatformDisplay` | 单显示器，bounds = 屏幕尺寸 |

**数据流（一帧）：**
```
android-activity 事件泵
  ├─ MainEvent::InitWindow      → 创建 wgpu surface（Vulkan，失败回退 GLES）
  ├─ MainEvent::TerminateWindow → 销毁 surface（切后台必须处理）
  ├─ InputEvent::Motion(touch)  → 坐标÷scale → gpui MouseDown/Move/Up
  └─ 帧回调 → 布局绘制 → Scene → gpui_wgpu → surface.present()
```

**触摸映射（PoC）**：单指触摸映射为鼠标左键按下/移动/抬起，使 gpui/gpui-component 现有 click/hover/scroll 逻辑零改动可用。多指手势、惯性滚动留二期。

## 6. 构建、调试与错误处理

**构建链（scripts/ 一键化）：**
```
cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build
cd android && ./gradlew assembleDebug
adb install -r app-debug.apk && adb shell am start …
```
工具补齐：安装 cargo-ndk；脚本内导出 ANDROID_HOME 与 platform-tools PATH（本机 SDK/NDK/AVD 镜像已就绪；Rust android target 已装）。

**调试手段：**
- `android_logger` 接管 `log` → logcat。
- 自定义 panic hook：panic 消息 + backtrace 打入 logcat。
- 验证循环：`adb exec-out screencap` 截图核对渲染；`adb shell input tap/swipe` 注入触摸做交互验证。

**Android 特有错误处理：**
- Surface 生命周期：后台销毁 / 前台重建（TerminateWindow/InitWindow 成对处理），否则 resume 即崩。
- 旋转 → 尺寸变化 → 通知 gpui resize。
- Vulkan 初始化失败自动回退 GLES 后端（wgpu Backends 配置）。

## 7. 测试与验收标准

| 阶段 | 验收标准 |
|---|---|
| PoC-1 纯 GPUI | 模拟器上 APK 启动；渲染彩色矩形 + 中英文文字；点击计数器按钮数字增加；pause/resume 与旋转不崩溃；logcat 无 panic |
| PoC-2 组件叠加 | gpui-component 的 Button / Input（显示）/ List 页面渲染正确、可点击滚动 |

单元测试：触摸映射、DPI 换算等纯逻辑抽为独立模块，host 上 `cargo test`；平台集成靠模拟器 e2e（截图 + input 注入）。

## 8. 已知风险与对策

| 风险 | 对策 |
|---|---|
| `Platform` trait 方法面大（约 40 个） | 桌面专属方法以合理默认值 stub，编译期发现遗漏 |
| gpui core 零星 `cfg(unix)`/linux 假设不适配 Android | 优先在 gpui_android 内规避；必要时 fork zed 打最小 patch（pin rev 保证 patch 稳定） |
| zed git 依赖 API 漂移 | pin rev；升级视为独立任务 |
| 模拟器 Vulkan 兼容性 | wgpu 配置 Vulkan+GLES 双后端自动选择 |
| gpui-component 可能引用桌面专属 API | PoC-2 阶段单独排查，demo 只选用平台无关组件 |
