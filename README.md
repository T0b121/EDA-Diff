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
- Hash-based SPA routing compatible with static GitHub Pages.
- Browser storage, file-source, and repository-provider boundaries.

KiCad import and semantic comparison are intentionally incomplete at this stage.
PCB zones, additional board graphics/rules, schematic pin connectivity, inferred
nets, and visual rendering remain planned.

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
