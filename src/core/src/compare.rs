/*
Format-independent comparison bundles for browser and API consumers.

Bundles keep canonical models together with their semantic diff so renderers can
visualize both revisions without reparsing native EDA files.
*/

use serde::Serialize;

use crate::diff::{diff_pcb, diff_schematic, DiffReport};
use crate::model::pcb::Pcb;
use crate::model::schematic::Schematic;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PcbComparison {
    pub before: Pcb,
    pub after: Pcb,
    pub diff: DiffReport,
}

impl PcbComparison {
    pub fn new(before: Pcb, after: Pcb) -> Self {
        let diff = diff_pcb(&before, &after);
        Self {
            before,
            after,
            diff,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SchematicComparison {
    pub before: Schematic,
    pub after: Schematic,
    pub diff: DiffReport,
}

impl SchematicComparison {
    pub fn new(before: Schematic, after: Schematic) -> Self {
        let diff = diff_schematic(&before, &after);
        Self {
            before,
            after,
            diff,
        }
    }
}
