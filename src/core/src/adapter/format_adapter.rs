/*
Format-adapter interface for EDA-Diff.

Every supported EDA format converts native files into the canonical model via
this boundary. Shared diff and merge code must never parse vendor formats itself.
*/

use crate::model::{pcb::Pcb, schematic::Schematic};

use super::error::AdapterError;

pub trait FormatAdapter {
    fn id(&self) -> &'static str;
    fn supports_path(&self, path: &str) -> bool;
    fn parse_pcb(&self, source: &str, path: &str) -> Result<Pcb, AdapterError>;
    fn parse_schematic(&self, source: &str, path: &str) -> Result<Schematic, AdapterError>;
}
