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
    let node = sexpr::child(value, name)?;
    let mut parts = node.list_iter()?.skip(1).peekable();

    let first = parts.next()?;
    if parts.peek().is_none() {
        return Some(
            sexpr::text(first)
                .map(str::to_owned)
                .unwrap_or_else(|| first.to_string()),
        );
    }

    let mut identifier = scalar_text(first);
    for part in parts {
        identifier.push_str(&scalar_text(part));
    }
    Some(identifier)
}

fn scalar_text(value: &Value) -> String {
    sexpr::text(value)
        .map(str::to_owned)
        .unwrap_or_else(|| value.to_string())
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
