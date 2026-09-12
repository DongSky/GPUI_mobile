#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target docs/catalog
cargo run -p gpui_material --bin material_catalog -- target/material-catalog-light.html
cargo run -p gpui_material --bin material_catalog -- target/material-catalog-dark.html --dark
cp target/material-catalog-light.html docs/catalog/material-catalog-light.html
cp target/material-catalog-dark.html docs/catalog/material-catalog-dark.html
echo "HTML catalog:"
echo "  target/material-catalog-light.html"
echo "  target/material-catalog-dark.html"
echo "  docs/catalog/material-catalog-light.html"
echo "  docs/catalog/material-catalog-dark.html"
