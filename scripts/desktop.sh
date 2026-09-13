#!/bin/bash
# Desktop Material catalog (Linux / macOS / Windows via upstream Zed GPUI).
set -euo pipefail
cd "$(dirname "$0")/.."
exec cargo run -p material_desktop_demo "$@"
