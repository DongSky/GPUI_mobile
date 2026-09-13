# material_desktop_demo

Desktop (macOS / Linux / Windows) Material 3 / Expressive catalog.

It boots an official Zed GPUI `Application` via `gpui_platform::application()`,
then paints `gpui_material::resolve()` appearances with the same `div` mapping
as Android `component_demo`. Colors and sizes come from the shared crate — this
binary does not define Material constants.

`gpui_material` stays dependency-free. This crate is the desktop consumer; it
does **not** pull in `gpui-component` / shadcn.

## Run

From the repo root (Linux / macOS; Windows uses the same cargo command):

```bash
cargo run -p material_desktop_demo
# or
scripts/desktop.sh
```

Host check without opening a window:

```bash
cargo test -p material_desktop_demo
```

Requires a display (X11 or Wayland). In this repo's cloud agent environment,
`DISPLAY=:1` is typically already set.

Linux first compile needs Zed/GPUI native headers:

```bash
sudo apt-get install -y libfontconfig1-dev libfreetype6-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev
```

## Skeleton scope

Heroes: Expressive buttons, outlined text-field notch, Expressive slider,
dialog open/close, date picker. Also icon buttons, checkbox/switch, tabs.

Out of scope: iOS, full Expressive motion clock, system IME, installers,
pixel-perfect state matrix.
