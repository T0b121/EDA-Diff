/*
Shared Rust core for EDA-Diff.

This crate owns the canonical EDA model, format adapters, semantic diffing,
merge logic, and validation. Browser-specific behavior must stay outside it.
*/

pub mod adapter;
pub mod model;

use adapter::format_adapter::FormatAdapter;
use adapter::kicad::KiCadAdapter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn core_status() -> String {
    "EDA-Diff Rust core ready".to_owned()
}

#[wasm_bindgen]
pub fn parse_kicad_pcb_json(source: &str, path: &str) -> Result<String, JsValue> {
    let pcb = KiCadAdapter
        .parse_pcb(source, path)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    serde_json::to_string(&pcb)
        .map_err(|error| JsValue::from_str(&format!("Failed to serialize PCB: {error}")))
}

#[wasm_bindgen]
pub fn parse_kicad_schematic_json(source: &str, path: &str) -> Result<String, JsValue> {
    let schematic = KiCadAdapter
        .parse_schematic(source, path)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    serde_json::to_string(&schematic)
        .map_err(|error| JsValue::from_str(&format!("Failed to serialize schematic: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_ready_status() {
        assert_eq!(core_status(), "EDA-Diff Rust core ready");
    }
}
