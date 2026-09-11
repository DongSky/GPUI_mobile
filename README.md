# GPUI_mobile

GPUI (Zed) on Android, plus a **Material 3 / Material You** component layer.

The original PoC rendered desktop `gpui-component` widgets (shadcn-like). Those do
not match Material Design. This repo now owns Material 3 tokens and resolved
appearances in `crates/gpui_material`, used by:

- host unit tests (no Android SDK)
- an HTML catalog for visual QA
- the Android `component_demo` catalog APK

Token values are taken from androidx Material 3 **v0_210** (`PaletteTokens`,
`ColorLightTokens`, `ColorDarkTokens`, `TypeScaleTokens`).

## Component inventory

| Component | Material Design | Parity | Notes | Docs |
|---|---|---|---|---|
| Color scheme | Color roles (baseline light/dark) | done | androidx PaletteTokens / ColorLightTokens / ColorDarkTokens v0_210 | [spec](https://m3.material.io/styles/color/roles) |
| Typography | Type scale (15 baseline styles) | done | Roboto; emphasized styles not implemented | [spec](https://m3.material.io/styles/typography/type-scale-tokens) |
| Shape | Shape scale | done | none through extra-large + full | [spec](https://m3.material.io/styles/shape/shape-scale-tokens) |
| Elevation | Elevation levels 0–5 | done | dp levels + catalog shadow; no tonal-overlay GPU lighting | [spec](https://m3.material.io/styles/elevation) |
| State layers | Interaction states | done | hover 8% / focus 10% / pressed 10% / dragged 16% / disabled 12%+38% | [spec](https://m3.material.io/foundations/interaction/states/state-layers) |
| Motion tokens | Easing and duration | partial | Tokens only; no shared animation runtime on GPUI yet | [spec](https://m3.material.io/styles/motion/easing-and-duration/tokens-specs) |
| Button | Common buttons (filled, tonal, elevated, outlined, text) | done | Default 40dp size; no expressive XS–XL or morph-to-square press | [spec](https://m3.material.io/components/buttons/specs) |
| Icon button | Icon buttons | done | Standard / filled / tonal / outlined; 40dp container, 48dp target | [spec](https://m3.material.io/components/icon-buttons/specs) |
| FAB | Floating action button | done | Regular 56dp; no extended / small / large FAB | [spec](https://m3.material.io/components/floating-action-button/specs) |
| Text field | Filled and outlined text fields | partial | Visual states including error; Android IME / caret editing not wired | [spec](https://m3.material.io/components/text-fields/specs) |
| List | Lists | done | One / two / three line; 56 / 72 / 88dp | [spec](https://m3.material.io/components/lists/specs) |
| Checkbox | Checkbox | done | 18dp / 2dp corners / 48dp target; checked, unchecked, indeterminate | [spec](https://m3.material.io/components/checkbox/specs) |
| Radio | Radio button | done | 20dp / 48dp target | [spec](https://m3.material.io/components/radio-button/specs) |
| Switch | Switch | done | 52×32 track; 16/24dp thumb | [spec](https://m3.material.io/components/switch/specs) |
| Chip | Assist / filter / input / suggestion chips | done | 32dp height; selected filter/input use secondary container | [spec](https://m3.material.io/components/chips/specs) |
| Card | Elevated / filled / outlined cards | done | 12dp corners, 16dp padding | [spec](https://m3.material.io/components/cards/specs) |
| Divider | Divider | done | 1dp outline-variant; full-bleed and inset | [spec](https://m3.material.io/components/divider/specs) |
| Progress | Linear and circular progress indicators | done | Determinate only in catalog; no wavy/indeterminate motion | [spec](https://m3.material.io/components/progress-indicators/specs) |
| Top app bar | Small top app bar | done | 64dp surface bar; no medium/large collapsing | [spec](https://m3.material.io/components/top-app-bar/specs) |
| Snackbar | Snackbar | done | Visual only; no timeout / swipe-to-dismiss runtime | [spec](https://m3.material.io/components/snackbar/specs) |
| Navigation bar | Navigation bar | done | 80dp; active indicator 64×32; 3 destinations in catalog | [spec](https://m3.material.io/components/navigation-bar/specs) |
| Dialog | Basic dialogs | not started | Follow-up | [spec](https://m3.material.io/components/dialogs/specs) |
| Bottom sheet | Bottom sheets | not started | Follow-up | [spec](https://m3.material.io/components/bottom-sheets/specs) |
| Menu | Menus | not started | Follow-up | [spec](https://m3.material.io/components/menus/specs) |
| Slider | Sliders | not started | Follow-up | [spec](https://m3.material.io/components/sliders/specs) |
| Tabs | Tabs | not started | Follow-up | [spec](https://m3.material.io/components/tabs/specs) |
| Badge | Badges | not started | Follow-up | [spec](https://m3.material.io/components/badges/specs) |
| Date picker | Date pickers | not started | Follow-up | [spec](https://m3.material.io/components/date-pickers/specs) |

`done` means tokens, metrics, and catalog states match the cited Material 3 spec
for the baseline (non-expressive) size. Hover is shown as a forced catalog state
(touch devices have no hover).

## Build (host checks — no Android SDK)

Requires Rust **1.85+** (edition 2024). This repo pins `stable` via
`rust-toolchain.toml`.

```bash
rustup show          # should select stable from rust-toolchain.toml
scripts/test.sh      # cargo test -p gpui_material && cargo test -p gpui_android
scripts/catalog.sh   # writes target/material-catalog-light.html and -dark.html
```

Open the HTML catalog in a browser for visual QA of every supported component
in canonical states.

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
| Visual catalog | `scripts/catalog.sh` | Light + dark HTML rendered from the same `resolve()` functions |

## Layout

```
crates/gpui_material/     Material 3 tokens + resolve() + HTML catalog
crates/gpui_android/      GPUI Platform for Android (wgpu + NativeActivity)
crates/component_demo/    Material catalog APK
crates/hello_gpui/        PoC-1 (rect + CJK text + tap counter)
android/                  Gradle NativeActivity packager
scripts/                  env, build, run, test, catalog, SDK setup
```
