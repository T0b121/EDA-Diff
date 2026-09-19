/*
KiCad native-object metadata helpers.

These helpers centralize UUID/timestamp extraction and source provenance so PCB
and schematic adapters do not duplicate native identity handling.
*/

use lexpr::Value;

use crate::model::source::{SourceFormat, SourceRef};

use super::sexpr;

pub fn object_native_id(value: &Value) -> Option<String> {
    sexpr::child_text(value, "uuid")
        .or_else(|| sexpr::child_text(value, "tstamp"))
        .map(str::to_owned)
}

pub fn source_ref(path: &str, native_id: String) -> SourceRef {
    let mut source = SourceRef::new(SourceFormat::KiCad, path);
    source.native_id = Some(native_id);
    source
}
