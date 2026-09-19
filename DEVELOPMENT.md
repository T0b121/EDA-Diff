<!--
Development and repository workflow for EDA-Diff.

This document defines how changes are structured, reviewed, committed, merged,
and released. It is intentionally concise so contributors can understand the
project rules without searching through unrelated documentation.
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
subtask.

As a guideline, a subtask should usually:

- touch about one to three files;
- contain roughly 50 to 100 changed lines;
- stay below about 200 changed lines where practical;
- represent one logical change that can be understood on its own.

These are guidelines, not artificial limits. A one-line change can be a complete
subtask, while tightly coupled work may reasonably require more files or lines.

## Reuse before adding

Before adding a new file, helper, abstraction, parser, or model:

1. inspect the existing implementation;
2. reuse or extend an existing component when it already owns the responsibility;
3. add a new abstraction only when the existing design cannot represent the need.

Format-specific code must adapt into the shared EDA model instead of creating a
second internal EDA representation. For example, an Eagle adapter should convert
between Eagle data and the common model used by the rest of EDA-Diff.

## File headers

Source, configuration, workflow, and documentation files should start with a
short multi-line comment describing:

- what the file is responsible for;
- what belongs in the file;
- important boundaries when they are not obvious from the filename.

The header should help identify the file immediately without becoming a second
full documentation page.

## Repository layout

- `src/` contains application and program source code.
- `assets/` contains static project assets, grouped by asset type.
- `scripts/` contains project automation used by builds or development.
- `.github/workflows/` contains GitHub Actions workflows.

Generated build output is not source code and should not be committed unless a
future task explicitly requires it.

## Releases and GitHub Pages

Publishing is tag-driven.

A tag triggers the Pages workflow, but deployment proceeds only if the tagged
commit is contained in `main`. This keeps development branches publish-safe and
allows multiple merges to happen without creating a version tag.

The web build produces a `dist/` directory. Future compilers such as
Emscripten may place WebAssembly and generated JavaScript into that directory
without changing the Pages deployment model.
