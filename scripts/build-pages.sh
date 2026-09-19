# Build script for the complete GitHub Pages artifact.
#
# This script tests the native Rust core, builds it to WebAssembly, bundles the
# TypeScript SPA with Vite, and adds static assets to the generated dist/ output.

set -eu

cargo test --manifest-path src/core/Cargo.toml
npm run build

if [ -d assets ]; then
  cp -R assets dist/assets
fi

find dist -name '.gitkeep' -delete

printf 'Tested core and built Rust/WASM + TypeScript site in dist/\n'
