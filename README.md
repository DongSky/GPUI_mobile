# GPUI_mobile

GPUI (Zed) on Android, plus a **Material 3 / Material You** component layer.

The original PoC rendered desktop `gpui-component` widgets (shadcn-like). Those do
not match Material Design. This repo now owns Material 3 tokens and resolved
appearances in `crates/gpui_material`, used by:

- host unit tests (no Android SDK)
- an HTML catalog for visual QA
- the Android `component_demo` catalog APK
- the desktop `material_desktop_demo` catalog (upstream Zed GPUI)

Color roles still match androidx **v0_210** light/dark schemes. Component metrics
follow the **current** [Material 3 / Expressive](https://m3.material.io) site
(May 2025+), not the older baseline-only “skip Expressive” stance.

## Component inventory

| Component | Material Design | Parity | Notes | Docs |
|---|---|---|---|---|
| Color scheme | Color roles (baseline light/dark) | done | androidx PaletteTokens / ColorLightTokens / ColorDarkTokens v0_210 | [spec](https://m3.material.io/styles/color/roles) |
| Typography | Type scale (15 baseline styles) | done | Roboto; emphasized styles not implemented | [spec](https://m3.material.io/styles/typography/type-scale-tokens) |
| Shape | Shape scale | done | Expressive scale: none … extra-extra-large (48) + full; large-increased 20 | [spec](https://m3.material.io/styles/shape/shape-scale-tokens) |
| Elevation | Elevation levels 0–5 | done | dp levels + catalog shadow; no tonal-overlay GPU lighting | [spec](https://m3.material.io/styles/elevation) |
| State layers | Interaction states | done | hover 8% / focus 10% / pressed 10% / dragged 16% / disabled 12%+38% | [spec](https://m3.material.io/foundations/interaction/states/state-layers) |
| Motion tokens | Easing and duration | done | Expressive spatial/effects springs + legacy emphasized eval; catalog CSS morph; shared FRAME_MS/FRAME_DT vsync clock | [spec](https://m3.material.io/styles/motion/easing-and-duration/tokens-specs) |
| Button | Common buttons (filled, tonal, elevated, outlined, text) | done | Expressive XS–XL, round/square, press morph; default S 40×16; outlined = outline-variant | [spec](https://m3.material.io/components/buttons/specs) |
| Button group | Standard + connected button groups (Expressive) | done | Standard: 12dp gap, ExpandedRatio 0.15 neighbor morph, Start/Center/End tonal→filled square. Connected: 2dp gap, 8dp inner, full-round outer; Day/Week/Month + icon row with overflow | [spec](https://m3.material.io/components/button-groups/specs) |
| Icon button | Icon buttons | done | Expressive XS–XL (32/40/56/96/136), round/square, press morph; default S 40×24 | [spec](https://m3.material.io/components/icon-buttons/specs) |
| FAB | Floating action button | done | Expressive regular 56 / medium 80 / large 96 / small-extended; 40dp small FAB deprecated | [spec](https://m3.material.io/components/floating-action-button/specs) |
| FAB menu | FAB menu (Expressive) | done | 2–6 item pills 56dp full-round + 56dp close FAB; 8dp close gap / 4dp item gap; primary/secondary/tertiary sets; Document / Message / Folder hero over decoded woven-basket JPEG | [spec](https://m3.material.io/components/fab-menu/specs) |
| Split button | Split buttons (Expressive) | done | Leading action + trailing menu, 2dp gap; outer full-round, inner 4dp rest / 12dp press (S); open trailing goes full-round + pressed layer; enamel-mugs JPEG product card ($7.49) | [spec](https://m3.material.io/components/split-button/specs) |
| Toolbar | Floating / docked toolbars (Expressive) | done | Floating 64dp full-round surface-container / vibrant primary-container + tertiary FAB; docked full-width; horizontal + vertical; 8dp pad / 4dp item gap; chat-thread scene (Renee Claess + decoded dog JPEG) | [spec](https://m3.material.io/components/toolbars/specs) |
| Text field | Filled and outlined text fields | done | Floating label; outlined C-path even-odd + lyon centerline; IME JNI-mangled Java_dev_gpui_… exports + RegisterNatives fnPtr + session handle + CursorAnchorInfo | [spec](https://m3.material.io/components/text-fields/specs) |
| List | Expressive segmented + baseline lists | done | Expressive segmented (recommended): 2dp gap, 4dp inner / 16dp outer, selected 16dp + secondary-container; Wi-Fi switches; swipe Archive/Delete 80dp rails; drag-handle reorder. Baseline 56/72/88 0-corner still available | [spec](https://m3.material.io/components/lists/specs) |
| Checkbox | Checkbox | done | 18dp / 2dp corners / 48dp target; checked, unchecked, indeterminate | [spec](https://m3.material.io/components/checkbox/specs) |
| Radio | Radio button | done | 20dp / 48dp target | [spec](https://m3.material.io/components/radio-button/specs) |
| Switch | Switch | done | 52×32 track; 16/24dp thumb | [spec](https://m3.material.io/components/switch/specs) |
| Chip | Assist / filter / input / suggestion chips | done | 32dp height; selected filter/input use secondary container | [spec](https://m3.material.io/components/chips/specs) |
| Card | Elevated / filled / outlined cards | done | 12dp corners, 16dp padding | [spec](https://m3.material.io/components/cards/specs) |
| Divider | Divider | done | 1dp outline-variant; full-bleed and inset | [spec](https://m3.material.io/components/divider/specs) |
| Progress | Linear and circular progress indicators | done | Determinate + wavy + Expressive 7-shape morph loading + contained PTR + WaitProgress determinate morph + circular/PTR StrokeCap::Round | [spec](https://m3.material.io/components/progress-indicators/specs) |
| Top app bar | Small + medium/large flexible + search | done | Expressive flexible: small 64; medium 112/136; large 120/152 compress to 64; scrolled surface-container elev 2; Bloom album hero + search bar | [spec](https://m3.material.io/components/app-bars/specs) |
| Snackbar | Snackbar | done | Inverse surface 48dp; Gmail scene with decoded JPEG avatars + Mail/Chat/Rooms/Meet filled dest icons + peeking thread + Email archived / Action / close; timeout 4s/10s + swipe 72dp | [spec](https://m3.material.io/components/snackbar/specs) |
| Navigation bar | Flexible / short navigation bar (Expressive) | done | Flexible 64dp (baseline 80dp not recommended); compact vertical 56×32 + secondary label; medium horizontal 40dp pill; surface-container elev 2 | [spec](https://m3.material.io/components/navigation-bar/specs) |
| Navigation rail | Navigation rail | done | Collapsed 80dp; expanded 220dp overlay-window / popup-kind + 32% scrim; OsPopupSpec titled PopUp (not opened on Linux/NativeActivity) | [spec](https://m3.material.io/components/navigation-rail/specs) |
| Dialog | Basic + full-screen dialogs | done | Reset-settings + ringtone list + full-screen Event editor (0dp, 64dp header, close/Save) | [spec](https://m3.material.io/components/dialogs/specs) |
| Bottom sheet | Bottom sheets | done | surface-container-low, extra-large top 28dp, 32×4 handle, elev 1; share sheet over decoded album JPEG with Send people row + horizontal Share/Add to/Trash actions | [spec](https://m3.material.io/components/bottom-sheets/specs) |
| Side sheet | Standard / modal / detached side sheets | done | 256dp; standard surface elev 0; modal surface-container-low elev 1 + 16dp start corners + 32% scrim; detached 16dp margin; Filters hero. Nav drawer is Expressive-deprecated | [spec](https://m3.material.io/components/side-sheets/specs) |
| Tooltip | Plain + rich tooltips | done | Plain inverse-surface 24dp extra-small + 16×8 caret; rich surface-container medium elev 2 + two actions + caret; hover / 500ms long-press. Banner is not on the current Expressive list | [spec](https://m3.material.io/components/tooltips/specs) |
| Menu | Menus | done | surface-container, 4dp, elev 2, 48dp items; selected secondary-container | [spec](https://m3.material.io/components/menus/specs) |
| Slider | Sliders | done | Expressive XS default: 16dp track, 4×44 handle, 6dp gap, 4dp stops; dual-thumb range with 5% tick-snap + painted ticks | [spec](https://m3.material.io/components/sliders/specs) |
| Tabs | Tabs | done | Primary 48dp + 3dp indicator; secondary 2dp full-width; icon+label 64dp + My saved media phone (Audio selected, Bloom/Egret decoded JPEG tiles, app-bar chrome) | [spec](https://m3.material.io/components/tabs/specs) |
| Badge | Badges | done | Small 6dp / large 16dp; error/on-error; 999+ | [spec](https://m3.material.io/components/badges/specs) |
| Date picker | Date pickers | done | Modal calendar; docked popup with month nav + outside-click dismiss | [spec](https://m3.material.io/components/date-pickers/specs) |
| Search | Search bar + view | done | 56dp docked bar shared-element growing-bar into full-screen search activity (leading/back + avatar crossfade; MorphLayerTransform top-center; with_animation PathBuilder::scale fill; morph_layer_box layout) | [spec](https://m3.material.io/components/search/specs) |
| Time picker | Time pickers (dial) | done | Hour + minute dial, analog selector hand + wall-clock second hand (16ms GPUI frames) | [spec](https://m3.material.io/components/time-pickers/specs) |
| Carousel | Carousel (hero / multi-browse / uncontained-multi) | done | Hero / multi-browse / uncontained / uncontained-multi / centered-hero / full-screen; phone-frame mask; Your lists phone; decoded JPEG media + parallax | [spec](https://m3.material.io/components/carousel/specs) |

`done` means tokens, metrics, and catalog **heroes** match the current
[m3.material.io](https://m3.material.io) Expressive language (not the older
baseline-only set). Hover is shown as a forced catalog state (touch has no hover).

## Build (host checks — no Android SDK)

Requires Rust **1.85+** (edition 2024). This repo pins `stable` via
`rust-toolchain.toml`.

```bash
rustup show          # should select stable from rust-toolchain.toml
scripts/test.sh      # gpui_material + gpui_android + desktop unit test + HTML catalog
scripts/catalog.sh   # writes target/material-catalog-light.html and -dark.html
```

Open the HTML catalog in a browser for visual QA of every supported component
in canonical states.

## Run (desktop catalog — Linux / macOS / Windows)

Same `gpui_material::resolve()` appearances as the Android APK and HTML catalog,
painted with upstream Zed GPUI (`gpui_platform::application()` + `open_window`).

```bash
cargo run -p material_desktop_demo
# or
scripts/desktop.sh
```

Needs a display (X11 or Wayland). `cargo test -p material_desktop_demo` checks
that heroes read metrics from `resolve()` and does not open a window.

Live window screenshots (ffmpeg x11grab of the GPUI catalog):

```bash
scripts/desktop-screenshot.sh docs/qa/desktop_gpui_live.png
```

Hosts without `/dev/dri` set `WGPU_BACKEND=gl` (and lavapipe Vulkan when an
extracted Mesa ICD is present). If wgpu cannot create a surface, the script
exits non-zero — use the HTML catalog for Visual QA and label those PNGs as
HTML, not GPUI pixels. Latest live frames: `docs/qa/desktop_capture_v16.md`.

Linux first compile of the desktop crate needs Zed/GPUI native headers
(`libfontconfig1-dev`, `libfreetype6-dev`, `libxkbcommon-dev`,
`libxkbcommon-x11-dev`, `libwayland-dev`). Android builds do not.

## Build (Android APK)

One-time toolchain (Linux example; macOS uses the same script):

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
scripts/setup-android-sdk.sh   # cmdline-tools + platform 35 + NDK 25.1.8937393
# Optional: install Gradle 8.14.3, or add a Gradle wrapper under android/
```

Then:

```bash
source scripts/env.sh
DEMO_CRATE=component_demo scripts/build.sh   # Material catalog APK
# or:
DEMO_CRATE=hello_gpui scripts/build.sh       # PoC-1 colored-rect demo
scripts/run.sh                               # emulator + adb install + launch
scripts/screenshot.sh /tmp/catalog.png
```

`scripts/env.sh` is portable (Linux + macOS). It looks for:

- `ANDROID_HOME` (`$HOME/Android/Sdk` or `$HOME/Library/Android/sdk`)
- NDK `25.1.8937393` or the newest installed NDK
- `JAVA_HOME` (Android Studio JBR on macOS, or the `java` on `PATH`)
- `android/gradlew`, then a local Gradle 8.14.3 dist, then `gradle` on `PATH`

The first Android compile downloads the pinned Zed `gpui` git dependency
(lockfile commit `a8fafdd7…`) and is large.

## Tests

| Suite | Command | What it covers |
|---|---|---|
| Material tokens | `cargo test -p gpui_material` | Palette RGB, light/dark roles, type scale, button/field/list/selection metrics, disabled compositing, catalog HTML evidence |
| Touch / timers | `cargo test -p gpui_android` | Touch→mouse/scroll mapping, timer queue |
| Desktop catalog | `cargo test -p material_desktop_demo` | Shared `resolve()` heroes (no window). Run the app with `scripts/desktop.sh` |
| Visual catalog | `scripts/catalog.sh` | Light + dark HTML rendered from the same `resolve()` functions |

## Layout

```
crates/gpui_material/          Material 3 tokens + resolve() + HTML catalog
crates/gpui_android/           GPUI Platform for Android (wgpu + NativeActivity)
crates/component_demo/         Material catalog APK
crates/material_desktop_demo/  Desktop GPUI catalog (same resolve() → div)
crates/hello_gpui/             PoC-1 (rect + CJK text + tap counter)
android/                       Gradle NativeActivity packager
scripts/                       env, build, run, test, catalog, desktop, desktop-screenshot, SDK setup
```
