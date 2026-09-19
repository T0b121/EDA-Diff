<!--
EDA-Diff project overview.

This file gives a short orientation for the repository. Detailed implementation
information stays close to source code and in DEVELOPMENT.md.
-->

# EDA-Diff

EDA-Diff is an open-source, Git-oriented tool for visual and semantic comparison
of electronic design files.

The application is designed as a local-first browser SPA for GitHub Pages.
TypeScript, HTML, and CSS provide the browser interface while a shared Rust core
is compiled to WebAssembly for EDA parsing, semantic comparison, merging, and
validation.

KiCad is the first planned format. Additional formats use adapters into the same
canonical EDA model instead of creating format-specific comparison engines.

## Architecture

- **Web:** TypeScript + HTML + CSS, bundled with Vite.
- **Core:** Rust, reusable natively and through WebAssembly.
- **Execution:** CPU-heavy core work runs through a Web Worker.
- **Routing:** hash-based SPA routes remain compatible with static GitHub Pages.
- **Storage:** browser persistence is abstracted for localStorage, IndexedDB, and
  OPFS use according to data size and lifetime.
- **Files/Git:** browser file sources and repository providers are isolated behind
  interfaces so local files, GitHub, and later providers can share the same UI.

EDA project content is intended to be processed locally in the browser by
default rather than uploaded to an EDA-Diff server.
