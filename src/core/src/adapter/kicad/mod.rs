/*
KiCad format adapter.

This module owns all KiCad-specific parsing and serialization. The rest of the
application communicates through the generic FormatAdapter boundary.
*/

mod native;
mod outline;
mod pad;
mod pcb;
mod schematic;
mod symbol;
mod sexpr;

use crate::adapter::error::AdapterError;
use crate::adapter::format_adapter::FormatAdapter;
use crate::model::{pcb::Pcb, schematic::Schematic};

pub struct KiCadAdapter;

impl FormatAdapter for KiCadAdapter {
    fn id(&self) -> &'static str {
        "kicad"
    }

    fn supports_path(&self, path: &str) -> bool {
        path.ends_with(".kicad_pcb") || path.ends_with(".kicad_sch")
    }

    fn parse_pcb(&self, source: &str, path: &str) -> Result<Pcb, AdapterError> {
        pcb::parse(source, path)
    }

    fn parse_schematic(&self, source: &str, path: &str) -> Result<Schematic, AdapterError> {
        schematic::parse(source, path)
    }
}
