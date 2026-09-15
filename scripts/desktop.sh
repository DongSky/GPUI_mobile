#!/bin/bash
# Desktop Material catalog (Linux / macOS / Windows via upstream Zed GPUI).
set -euo pipefail
cd "$(dirname "$0")/.."

# Prefer Roboto when we can drop TTFs into the user font dir.
FONT_DIR="${HOME:-/tmp}/.local/share/fonts"
mkdir -p "$FONT_DIR"
if [ ! -f "$FONT_DIR/Roboto-Regular.ttf" ]; then
  # google/fonts Apache Roboto static TTFs (no apt required).
  for weight in Regular Medium Bold; do
    curl -fsSL "https://github.com/googlefonts/roboto/raw/main/src/hinted/Roboto-${weight}.ttf" \
      -o "$FONT_DIR/Roboto-${weight}.ttf" || true
  done
  fc-cache -f "$FONT_DIR" >/dev/null 2>&1 || true
fi

# Cloud / CI hosts often have X11 but no GPU. wgpu then needs a software backend.
if [ ! -e /dev/dri ]; then
  export WGPU_BACKEND="${WGPU_BACKEND:-gl}"
  export LIBGL_ALWAYS_SOFTWARE="${LIBGL_ALWAYS_SOFTWARE:-1}"
  export MESA_GL_VERSION_OVERRIDE="${MESA_GL_VERSION_OVERRIDE:-3.3}"
  export MESA_GLSL_VERSION_OVERRIDE="${MESA_GLSL_VERSION_OVERRIDE:-330}"
  export GALLIUM_DRIVER="${GALLIUM_DRIVER:-llvmpipe}"
fi

exec cargo run -p material_desktop_demo "$@"
