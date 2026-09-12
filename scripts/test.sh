#!/bin/bash
# Host-side checks (no Android SDK required).
set -euo pipefail
cd "$(dirname "$0")/.."
cargo test -p gpui_material
cargo test -p gpui_android
cargo run -p gpui_material --bin material_catalog -- target/material-catalog-light.html
echo "OK: gpui_material + gpui_android host tests; catalog at target/material-catalog-light.html"
