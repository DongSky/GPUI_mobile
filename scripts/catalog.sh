#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target
cargo run -p gpui_material --bin material_catalog -- target/material-catalog-light.html
cargo run -p gpui_material --bin material_catalog -- target/material-catalog-dark.html --dark
echo "HTML catalog:"
echo "  target/material-catalog-light.html"
echo "  target/material-catalog-dark.html"
