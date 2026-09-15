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
| Motion tokens | Easing and duration | done | Expressive spatial/effects springs + legacy emphasized eval; catalog CSS morph; no GPUI animation clock | [spec](https://m3.material.io/styles/motion/easing-and-duration/tokens-specs) |
| Button | Common buttons (filled, tonal, elevated, outlined, text) | done | Expressive XS–XL, round/square, press morph; default S 40×16; outlined = outline-variant | [spec](https://m3.material.io/components/buttons/specs) |
| Icon button | Icon buttons | done | Expressive XS–XL (32/40/56/96/136), round/square, press morph; default S 40×24 | [spec](https://m3.material.io/components/icon-buttons/specs) |
| FAB | Floating action button | done | Expressive regular 56 / medium 80 / large 96 / small-extended; 40dp small FAB deprecated | [spec](https://m3.material.io/components/floating-action-button/specs) |
| Text field | Filled and outlined text fields | done | Floating label; outlined C-path even-odd notch (RoundedPolygon cubics live); IME caret + InputConnection session + JNI IMM queue (no live JNIEnv yet) | [spec](https://m3.material.io/components/text-fields/specs) |
| List | Lists | done | One / two / three line; 56 / 72 / 88dp | [spec](https://m3.material.io/components/lists/specs) |
| Checkbox | Checkbox | done | 18dp / 2dp corners / 48dp target; checked, unchecked, indeterminate | [spec](https://m3.material.io/components/checkbox/specs) |
| Radio | Radio button | done | 20dp / 48dp target | [spec](https://m3.material.io/components/radio-button/specs) |
| Switch | Switch | done | 52×32 track; 16/24dp thumb | [spec](https://m3.material.io/components/switch/specs) |
| Chip | Assist / filter / input / suggestion chips | done | 32dp height; selected filter/input use secondary container | [spec](https://m3.material.io/components/chips/specs) |
| Card | Elevated / filled / outlined cards | done | 12dp corners, 16dp padding | [spec](https://m3.material.io/components/cards/specs) |
| Divider | Divider | done | 1dp outline-variant; full-bleed and inset | [spec](https://m3.material.io/components/divider/specs) |
| Progress | Linear and circular progress indicators | done | Determinate + wavy + Expressive 7-shape morph loading + contained PTR + WaitProgress determinate morph + round-capped circular sausage + wavy LineCap::Round stroke | [spec](https://m3.material.io/components/progress-indicators/specs) |
| Top app bar | Small top app bar | done | 64dp surface bar; no medium/large collapsing | [spec](https://m3.material.io/components/top-app-bar/specs) |
| Snackbar | Snackbar | done | Visual only; no timeout / swipe-to-dismiss runtime | [spec](https://m3.material.io/components/snackbar/specs) |
| Navigation bar | Navigation bar | done | 80dp; active indicator 64×32; 3 destinations in catalog | [spec](https://m3.material.io/components/navigation-bar/specs) |
| Navigation rail | Navigation rail | done | Collapsed 80dp; expanded 220dp overlay-window / popup-kind + 32% scrim, level-2 elevation, focus trap | [spec](https://m3.material.io/components/navigation-rail/specs) |
| Dialog | Basic dialogs | done | surface-container-high, 28dp, elev 3, headlineSmall/bodyMedium, 32% scrim | [spec](https://m3.material.io/components/dialogs/specs) |
| Bottom sheet | Bottom sheets | done | surface-container-low, extra-large top 28dp, 32×4 handle, elev 1 | [spec](https://m3.material.io/components/bottom-sheets/specs) |
| Menu | Menus | done | surface-container, 4dp, elev 2, 48dp items; selected secondary-container | [spec](https://m3.material.io/components/menus/specs) |
| Slider | Sliders | done | Expressive XS default: 16dp track, 4×44 handle, 6dp gap, 4dp stops; dual-thumb range with 5% tick-snap + painted ticks | [spec](https://m3.material.io/components/sliders/specs) |
| Tabs | Tabs | done | Primary 48dp + 3dp primary indicator; secondary 2dp full-width | [spec](https://m3.material.io/components/tabs/specs) |
| Badge | Badges | done | Small 6dp / large 16dp; error/on-error; 999+ | [spec](https://m3.material.io/components/badges/specs) |
| Date picker | Date pickers | done | Modal calendar; docked popup with month nav + outside-click dismiss | [spec](https://m3.material.io/components/date-pickers/specs) |
| Search | Search bar + view | done | 56dp docked bar shared-element growing-bar into full-screen search activity (leading/back + avatar crossfade; GPUI scaled-margin stand-in for CSS scale) | [spec](https://m3.material.io/components/search/specs) |
| Time picker | Time pickers (dial) | done | Hour + minute dial, analog selector hand + wall-clock second hand | [spec](https://m3.material.io/components/time-pickers/specs) |

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
HTML, not GPUI pixels. Latest live frames: `docs/qa/desktop_capture_v12.md`.

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
