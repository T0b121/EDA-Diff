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

## Current capabilities

- Rust/WASM core running through a Web Worker.
- Canonical, format-independent project, schematic, and PCB data structures.
- Initial KiCad PCB adapter for nets, footprints, straight tracks, and vias.
- Local `.kicad_pcb` import test in the Compare view.
- Hash-based SPA routing compatible with static GitHub Pages.
- Browser storage, file-source, and repository-provider boundaries.

KiCad PCB import is intentionally incomplete at this stage. Pads, zones, graphic
items, board outlines, rules, and schematic parsing are planned additions.

## Architecture

- **Web:** TypeScript + HTML + CSS, bundled with Vite.
- **Core:** Rust, reusable natively and through WebAssembly.
- **Execution:** CPU-heavy core work runs through a Web Worker.
- **Routing:** hash-based SPA routes remain compatible with static GitHub Pages.
- **Storage:** browser persistence is abstracted for localStorage, IndexedDB, and
  OPFS use according to data size and lifetime.
- **Files/Git:** browser file sources and repository providers are isolated behind
  interfaces so local files, GitHub, and later providers can share the same UI.

EDA project content is processed locally in the browser by default rather than
uploaded to an EDA-Diff server.
