# Build script for the GitHub Pages artifact.
#
# This script assembles the browser-ready site in dist/. Keep deployment logic
# out of here; future language-specific compilers may feed their generated web
# output into dist/ while GitHub Actions remains responsible for publishing it.

set -eu

rm -rf dist
mkdir -p dist

cp -R src/. dist/

if [ -d assets ]; then
  cp -R assets dist/assets
fi

find dist -name '.gitkeep' -delete

printf 'Built GitHub Pages site in dist/\n'
