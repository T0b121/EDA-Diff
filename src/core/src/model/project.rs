/*
Top-level canonical EDA project model.

A project can contain a schematic, a PCB, or both. Adapters populate this model
so rendering, diffing, and merging operate on one shared representation.
*/

use serde::{Deserialize, Serialize};

use super::{pcb::Pcb, schematic::Schematic};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdaProject {
    pub name: String,
    pub schematic: Option<Schematic>,
    pub pcb: Option<Pcb>,
}

impl EdaProject {
    pub fn empty(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            schematic: None,
            pcb: None,
        }
    }
}
