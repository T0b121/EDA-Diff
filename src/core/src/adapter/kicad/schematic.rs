/*
KiCad schematic to canonical schematic converter.

This parser imports placed symbols, wires, junctions, and labels into the shared
EDA model. Electrical net inference is intentionally left to a later core layer.
*/

use lexpr::Value;

use crate::adapter::error::AdapterError;
use crate::model::common::{ObjectId, Rotation};
use crate::model::schematic::{
    Junction, Label, LabelKind, Net, Schematic, Symbol, Wire,
};

use super::native::{object_native_id, property, source_ref};
use super::sexpr;

pub fn parse(source: &str, path: &str) -> Result<Schematic, AdapterError> {
    let root = sexpr::parse(source)?;
    if sexpr::head(&root) != Some("kicad_sch") {
        return Err(AdapterError::UnsupportedFormat(
            "Expected a KiCad .kicad_sch document".to_owned(),
        ));
    }

    Ok(Schematic {
        symbol_definitions: super::symbol::parse_definitions(&root),
        symbols: sexpr::children(&root, "symbol")
            .map(|value| parse_symbol(value, path))
            .collect::<Result<Vec<_>, _>>()?,
        wires: sexpr::children(&root, "wire")
            .map(|value| parse_wire(value, path))
            .collect::<Result<Vec<_>, _>>()?,
        junctions: sexpr::children(&root, "junction")
            .map(|value| parse_junction(value, path))
            .collect::<Result<Vec<_>, _>>()?,
        labels: parse_labels(&root, path)?,
        nets: Vec::<Net>::new(),
    })
}

fn parse_symbol(value: &Value, path: &str) -> Result<Symbol, AdapterError> {
    let native_id = required_native_id(value, "symbol")?;
    let library_id = sexpr::child_text(value, "lib_id")
        .map(str::to_owned)
        .or_else(|| sexpr::argument(value, 0).and_then(sexpr::text).map(str::to_owned));

    Ok(Symbol {
        id: ObjectId::new(format!("kicad:{native_id}")),
        reference: property(value, "Reference").unwrap_or_default(),
        value: property(value, "Value").unwrap_or_default(),
        library_id,
        footprint: property(value, "Footprint").filter(|value| !value.is_empty()),
        unit: sexpr::child_number(value, "unit").unwrap_or(1.0) as u32,
        position: sexpr::required_point(value, "at")?,
        rotation: Rotation {
            degrees: sexpr::child_rotation(value, "at"),
        },
        mirror_x: sexpr::child_text(value, "mirror") == Some("x"),
        mirror_y: sexpr::child_text(value, "mirror") == Some("y"),
        source: source_ref(path, native_id),
    })
}

fn parse_wire(value: &Value, path: &str) -> Result<Wire, AdapterError> {
    let native_id = required_native_id(value, "wire")?;
    let points = sexpr::child_xy_points(value, "pts");

    if points.len() < 2 {
        return Err(AdapterError::MissingField(
            "KiCad wire must contain at least two points".to_owned(),
        ));
    }

    Ok(Wire {
        id: ObjectId::new(format!("kicad:{native_id}")),
        points,
        source: source_ref(path, native_id),
    })
}

fn parse_junction(value: &Value, path: &str) -> Result<Junction, AdapterError> {
    let native_id = required_native_id(value, "junction")?;
    Ok(Junction {
        id: ObjectId::new(format!("kicad:{native_id}")),
        position: sexpr::required_point(value, "at")?,
        source: source_ref(path, native_id),
    })
}

fn parse_labels(root: &Value, path: &str) -> Result<Vec<Label>, AdapterError> {
    let mut labels = Vec::new();

    for (node_name, kind) in [
        ("label", LabelKind::Local),
        ("global_label", LabelKind::Global),
        ("hierarchical_label", LabelKind::Hierarchical),
    ] {
        for value in sexpr::children(root, node_name) {
            labels.push(parse_label(value, path, kind.clone())?);
        }
    }

    Ok(labels)
}

fn parse_label(
    value: &Value,
    path: &str,
    kind: LabelKind,
) -> Result<Label, AdapterError> {
    let native_id = required_native_id(value, "label")?;
    Ok(Label {
        id: ObjectId::new(format!("kicad:{native_id}")),
        name: sexpr::argument(value, 0)
            .and_then(sexpr::text)
            .unwrap_or("")
            .to_owned(),
        kind,
        position: sexpr::required_point(value, "at")?,
        rotation: Rotation {
            degrees: sexpr::child_rotation(value, "at"),
        },
        electrical_shape: sexpr::child_text(value, "shape").map(str::to_owned),
        source: source_ref(path, native_id),
    })
}

fn required_native_id(value: &Value, kind: &str) -> Result<String, AdapterError> {
    object_native_id(value).ok_or_else(|| {
        AdapterError::MissingField(format!("KiCad {kind} is missing a native identifier"))
    })
}
