#!/bin/bash
# Desktop Material catalog (Linux / macOS / Windows via upstream Zed GPUI).
set -euo pipefail
cd "$(dirname "$0")/.."

# Cloud / CI hosts often have X11 but no GPU. wgpu then needs a software backend.
if [ ! -e /dev/dri ]; then
  export WGPU_BACKEND="${WGPU_BACKEND:-gl}"
fi

exec cargo run -p material_desktop_demo "$@"
