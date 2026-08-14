#!/bin/bash
cd "$(dirname "$0")/.."
source scripts/env.sh
out="${1:-/tmp/gpui_screenshot.png}"
adb exec-out screencap -p > "$out"
echo "$out"
