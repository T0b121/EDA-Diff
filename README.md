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
- KiCad PCB adapter for nets, footprints, pads, tracks, vias, and Edge.Cuts geometry.
- Initial KiCad schematic adapter for symbols, wires, junctions, and labels.
- Local `.kicad_pcb` and `.kicad_sch` semantic comparison in the Compare view.
- Structured added/removed/modified/unchanged results with field-level changes.
- Initial SVG PCB diff overlay for tracks, vias, pads, footprints, and board edges.
- SVG schematic diff for embedded symbol graphics, pins, wires, junctions, and labels.
- Interactive PCB viewport with pan, cursor-centered zoom, fit-to-board, and Before/After visibility controls.
- Canonical PCB layer metadata with per-layer visibility controls in the visual diff.
- Hash-based SPA routing compatible with static GitHub Pages.
- Public GitHub repository history browsing with revision and KiCad file selection.
- Browser storage, file-source, and repository-provider boundaries.

KiCad import and semantic comparison remain intentionally scoped. PCB zones,
additional board graphics/rules, full schematic connectivity inference, and
write/merge workflows remain planned after the first stable release.

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
