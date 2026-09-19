/*
Canonical EDA model module.

All format adapters convert native files into these shared structures. Diff,
merge, rendering, and validation must depend on this module rather than formats.
*/

pub mod common;
pub mod pcb;
pub mod project;
pub mod schematic;
pub mod source;

pub use project::EdaProject;
