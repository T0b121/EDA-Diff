/*
Vite configuration for the EDA-Diff browser build.

The app is hosted below /EDA-Diff/ on GitHub Pages, so asset URLs use that base
path. Build output is placed in the repository-level dist/ directory.
*/

import { defineConfig } from "vite";

export default defineConfig({
  root: "src/web",
  base: "/EDA-Diff/",
  build: {
    outDir: "../../dist",
    emptyOutDir: true
  }
});
