#!/bin/bash
# Capture a live Desktop GPUI catalog window (ffmpeg x11grab).
#
# Cloud / CI hosts often have X11 (DISPLAY=:1) but no /dev/dri. This script
# sets software GL + lavapipe Vulkan when present, launches
# material_desktop_demo, grabs the window, and writes a PNG.
#
# If wgpu cannot create a surface, the demo prints
# "Failed to create surface for any enabled backend" and this script exits
# non-zero. Visual QA then uses the HTML catalog generated from the same
# gpui_material::resolve() recipes — label those PNGs as HTML, not GPUI pixels.
set -euo pipefail
cd "$(dirname "$0")/.."

OUT="${1:-docs/qa/desktop_gpui_live.png}"
mkdir -p "$(dirname "$OUT")"

export DISPLAY="${DISPLAY:-:1}"

# Prefer Roboto when we can drop TTFs into the user font dir.
FONT_DIR="${HOME:-/tmp}/.local/share/fonts"
mkdir -p "$FONT_DIR"
if [ ! -f "$FONT_DIR/Roboto-Regular.ttf" ]; then
  for weight in Regular Medium Bold; do
    curl -fsSL "https://github.com/googlefonts/roboto/raw/main/src/hinted/Roboto-${weight}.ttf" \
      -o "$FONT_DIR/Roboto-${weight}.ttf" || true
  done
  fc-cache -f "$FONT_DIR" >/dev/null 2>&1 || true
fi

if [ ! -e /dev/dri ]; then
  export LIBGL_ALWAYS_SOFTWARE="${LIBGL_ALWAYS_SOFTWARE:-1}"
  export MESA_GL_VERSION_OVERRIDE="${MESA_GL_VERSION_OVERRIDE:-3.3}"
  export MESA_GLSL_VERSION_OVERRIDE="${MESA_GLSL_VERSION_OVERRIDE:-330}"
  export GALLIUM_DRIVER="${GALLIUM_DRIVER:-llvmpipe}"
fi

# Optional extracted Mesa lavapipe (Ubuntu noble mesa-vulkan-drivers).
LVP_LIB=""
for candidate in \
  /tmp/mesa-noble/usr/lib/x86_64-linux-gnu \
  /tmp/mesa-vk/usr/lib/x86_64-linux-gnu
do
  if [ -f "$candidate/libvulkan_lvp.so" ]; then
    LVP_LIB="$candidate"
    break
  fi
done
if [ -n "$LVP_LIB" ]; then
  ICD_SRC="$(dirname "$LVP_LIB")/../share/vulkan/icd.d/lvp_icd.json"
  if [ ! -f "$ICD_SRC" ]; then
    ICD_SRC="/tmp/mesa-noble/usr/share/vulkan/icd.d/lvp_icd.json"
  fi
  if [ -f "$ICD_SRC" ]; then
    ICD_DIR="${TMPDIR:-/tmp}/gpui-lvp-icd"
    mkdir -p "$ICD_DIR"
    python3 - "$ICD_SRC" "$LVP_LIB/libvulkan_lvp.so" "$ICD_DIR/lvp_icd.json" <<'PY'
import json, sys
src, lib, dest = sys.argv[1:]
with open(src) as f:
    data = json.load(f)
data.setdefault("ICD", {})["library_path"] = lib
with open(dest, "w") as f:
    json.dump(data, f)
print(dest)
PY
    export VK_ICD_FILENAMES="$ICD_DIR/lvp_icd.json"
    # Prepend — do not hide system libs such as libxkbcommon-x11.
    export LD_LIBRARY_PATH="$LVP_LIB:/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
    export WGPU_BACKEND="${WGPU_BACKEND:-vulkan}"
  fi
fi

if [ -z "${WGPU_BACKEND:-}" ] && [ ! -e /dev/dri ]; then
  export WGPU_BACKEND=gl
fi

# Extra extracted EGL / xcb bits from earlier v5/v6 attempts.
for extra in /tmp/xkbx11/usr/lib/x86_64-linux-gnu /tmp/xcbxkb/usr/lib/x86_64-linux-gnu \
            /tmp/egl/usr/lib/x86_64-linux-gnu /tmp/linklibs; do
  if [ -d "$extra" ]; then
    export LD_LIBRARY_PATH="$extra${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  fi
done

echo "DISPLAY=$DISPLAY WGPU_BACKEND=${WGPU_BACKEND:-} VK_ICD_FILENAMES=${VK_ICD_FILENAMES:-}"
echo "LD_LIBRARY_PATH=${LD_LIBRARY_PATH:-}"

cargo build -p material_desktop_demo

BIN=./target/debug/material_desktop_demo
LOG="${TMPDIR:-/tmp}/gpui-desktop-screenshot.log"
: > "$LOG"

# Do not pkill -f material_desktop_demo (matches this wrapper). Kill by exe path.
if pids=$(ps -eo pid,args | awk '/\.\/target\/debug\/material_desktop_demo/ && !/awk/ {print $1}'); then
  for pid in $pids; do
    exe=$(readlink "/proc/$pid/exe" 2>/dev/null || true)
    if [ "$exe" = "$(readlink -f "$BIN")" ]; then
      kill "$pid" 2>/dev/null || true
    fi
  done
fi

"$BIN" >>"$LOG" 2>&1 &
APP_PID=$!

cleanup() {
  if kill -0 "$APP_PID" 2>/dev/null; then
    kill "$APP_PID" 2>/dev/null || true
    wait "$APP_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

# Wait for a window or a wgpu failure.
for i in $(seq 1 40); do
  if grep -q "Failed to create surface" "$LOG" 2>/dev/null; then
    echo "GPUI window did not open (wgpu surface). Log:" >&2
    cat "$LOG" >&2
    exit 2
  fi
  if ! kill -0 "$APP_PID" 2>/dev/null; then
    echo "material_desktop_demo exited before a window opened. Log:" >&2
    cat "$LOG" >&2
    exit 2
  fi
  if timeout 2 xdotool search --name "Material 3 desktop" >/dev/null 2>&1; then
    break
  fi
  sleep 0.5
done

if grep -q "Failed to create surface" "$LOG" 2>/dev/null; then
  echo "GPUI window did not open (wgpu surface). Log:" >&2
  cat "$LOG" >&2
  exit 2
fi

sleep 1
# Catalog window is ~720×880. ffmpeg x11grab wants DISPLAY+X,Y (e.g. :1+1200,85).
GEOM="720x880"
GRAB="${DISPLAY}+0,0"
if command -v xdotool >/dev/null 2>&1; then
  WID=$(timeout 2 xdotool search --name "Material 3 desktop" | head -n1 || true)
  if [ -n "${WID:-}" ]; then
    eval "$(timeout 2 xdotool getwindowgeometry --shell "$WID" 2>/dev/null || true)"
    if [ -n "${WIDTH:-}" ] && [ -n "${HEIGHT:-}" ]; then
      GEOM="${WIDTH}x${HEIGHT}"
      GRAB="${DISPLAY}+${X:-0},${Y:-0}"
    fi
  fi
fi

timeout 8 ffmpeg -y -f x11grab -video_size "$GEOM" -i "$GRAB" \
  -frames:v 1 -update 1 "$OUT" >/dev/null 2>&1 || \
timeout 8 ffmpeg -y -f x11grab -i "$DISPLAY" -frames:v 1 -update 1 "$OUT" >/dev/null 2>&1

if [ ! -s "$OUT" ]; then
  echo "ffmpeg capture failed. Log:" >&2
  cat "$LOG" >&2
  exit 3
fi

echo "wrote $OUT"
