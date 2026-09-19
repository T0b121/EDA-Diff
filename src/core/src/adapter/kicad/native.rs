/*
KiCad native-object metadata helpers.

These helpers centralize UUID/timestamp extraction and source provenance so PCB
and schematic adapters do not duplicate native identity handling.
*/

use lexpr::Value;

use crate::model::source::{SourceFormat, SourceRef};

use super::sexpr;

pub fn object_native_id(value: &Value) -> Option<String> {
    child_identifier(value, "uuid").or_else(|| child_identifier(value, "tstamp"))
}

fn child_identifier(value: &Value, name: &str) -> Option<String> {
    let argument = sexpr::argument(sexpr::child(value, name)?, 0)?;

    sexpr::text(argument)
        .map(str::to_owned)
        .or_else(|| Some(argument.to_string()))
}

pub fn source_ref(path: &str, native_id: String) -> SourceRef {
    let mut source = SourceRef::new(SourceFormat::KiCad, path);
    source.native_id = Some(native_id);
    source
}

pub fn property(value: &Value, name: &str) -> Option<String> {
    sexpr::children(value, "property").find_map(|node| {
        (sexpr::argument(node, 0).and_then(sexpr::text) == Some(name))
            .then(|| sexpr::argument(node, 1).and_then(sexpr::text).map(str::to_owned))
            .flatten()
    })
}
