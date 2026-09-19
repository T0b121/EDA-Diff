/*
Format-independent semantic diff engine.

Adapters convert native EDA files into the canonical model first. Diffing then
operates only on canonical PCB and schematic concepts.
*/

mod matcher;
mod pcb;
mod schematic;
mod types;

pub use pcb::diff_pcb;
pub use schematic::diff_schematic;
pub use types::{
    ChangeKind, DiffReport, DiffSummary, FieldChange, MatchMethod, ObjectChange,
};
