# Build script for the complete GitHub Pages artifact.
#
# This script builds the shared Rust core to WebAssembly, bundles the TypeScript
# SPA with Vite, and then adds static assets to the generated dist/ directory.

set -eu

npm run build

if [ -d assets ]; then
  cp -R assets dist/assets
fi

find dist -name '.gitkeep' -delete

printf 'Built Rust/WASM and TypeScript GitHub Pages site in dist/\n'
