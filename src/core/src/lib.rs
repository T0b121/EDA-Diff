/*
Shared Rust core for EDA-Diff.

This crate will hold the canonical EDA model, format adapters, semantic diffing,
merge logic, and validation. Browser-specific behavior must stay outside it.
*/

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn core_status() -> String {
    "EDA-Diff Rust core ready".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_ready_status() {
        assert_eq!(core_status(), "EDA-Diff Rust core ready");
    }
}
