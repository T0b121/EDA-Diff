/*
Vite configuration for the EDA-Diff browser build.

The app is hosted below /EDA-Diff/ on GitHub Pages, so asset URLs must use that
base path. Build output always goes to the repository-level dist/ directory.
*/

import { defineConfig } from "vite";
import { fileURLToPath, URL } from "node:url";

export default defineConfig({
  root: fileURLToPath(new URL("./src/web", import.meta.url)),
  base: "/EDA-Diff/",
  build: {
    outDir: fileURLToPath(new URL("./dist", import.meta.url)),
    emptyOutDir: true
  }
});
