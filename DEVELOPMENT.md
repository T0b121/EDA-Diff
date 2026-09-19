<!--
Development and repository workflow for EDA-Diff.

This document defines how changes are structured, reviewed, committed, merged,
built, and released, plus the architectural boundaries contributors must keep.
-->

# Development workflow

## Branches

- Never implement normal tasks directly on `main`.
- Create a dedicated branch for each task.
- Keep completed task branches instead of deleting them.
- Merge completed work back into `main` with a merge commit; do not fast-forward.
- A merge into `main` does not automatically create a release.

The repository's first bootstrap commit is the only exception because Git cannot
create a branch before an initial commit exists.

## Commits and subtasks

Split a task into small, understandable subtasks and create one commit for each
subtask. A subtask should usually touch about one to three files, contain roughly
50 to 100 changed lines, and stay below about 200 changed lines where practical.

These are guidelines rather than artificial limits. A one-line change can be a
complete subtask, while tightly coupled work may reasonably require more.

## Reuse before adding

Before adding a new file, helper, abstraction, parser, or model:

1. inspect the existing implementation;
2. reuse or extend an existing component when it owns the responsibility;
3. add an abstraction only when the current design cannot represent the need.

Format-specific code converts to and from the shared EDA model. It must not
introduce a second internal EDA representation or a parallel diff engine.

## File headers

Source, configuration, workflow, and documentation files start with a short
multi-line comment explaining their responsibility and important boundaries.
The header should identify the file quickly without duplicating full docs.

## Application architecture

The deployed product is a static, local-first single-page application:

- `src/web/`: HTML, CSS, TypeScript, UI, routing, browser APIs, providers.
- `src/core/`: Rust EDA domain logic reusable as native code and WebAssembly.
- `src/web/worker/`: bridge that keeps expensive Rust/WASM work off the UI thread.
- `src/web/storage/`: persistence boundaries; small preferences may use
  localStorage while large data is reserved for IndexedDB/OPFS implementations.
- `src/web/files/`: local project source adapters.
- `src/web/git/`: repository-provider contracts and Git-host integrations.
- `assets/`: static assets grouped by type.
- `scripts/`: build/development automation.
- `.github/workflows/`: GitHub Actions workflows.

GitHub Pages provides no EDA-Diff application server. Do not design features that
require server-side sessions, private secrets embedded in the client, or server
filesystem state. Hash routing is used so direct SPA navigation remains valid on
static Pages hosting.

Generated build output, Rust target files, Node dependencies, and generated WASM
bindings are not committed.

## Data boundaries

Original project files or repositories remain the source of truth. Parsed EDA
models and derived comparison data are caches and may be regenerated.

Browser persistence must not be treated as the only durable copy of user data.
Sensitive credentials must not be persisted in URLs, source files, or build
artifacts.

## Releases and GitHub Pages

Publishing is tag-driven. A tag starts the Pages workflow, but deployment
continues only if the tagged commit is contained in `main`. Ordinary merges do
not create releases.

The build compiles the Rust core to WebAssembly, bundles the TypeScript SPA with
Vite, and produces `dist/`. GitHub Pages deploys only that static artifact.
