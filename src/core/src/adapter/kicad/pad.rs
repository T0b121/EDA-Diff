/*
KiCad footprint pad conversion.

Pads stay in footprint-local coordinates so their native placement and rotation
remain available for later rendering, diffing, and format round-tripping.
*/

use std::collections::HashMap;

use lexpr::Value;

use crate::adapter::error::AdapterError;
use crate::model::common::{ObjectId, Rotation, Size};
use crate::model::pcb::Pad;

use super::sexpr;
use super::{net_reference, object_native_id, source_ref};

pub fn parse(
    value: &Value,
    path: &str,
    net_ids: &HashMap<i64, ObjectId>,
) -> Result<Pad, AdapterError> {
    let native_id = object_native_id(value).unwrap_or_else(|| "unknown".to_owned());
    let at = sexpr::child(value, "at");
    let size = sexpr::child(value, "size");

    let width_mm = size
        .and_then(|node| sexpr::argument(node, 0))
        .and_then(sexpr::number)
        .unwrap_or(0.0);
    let height_mm = size
        .and_then(|node| sexpr::argument(node, 1))
        .and_then(sexpr::number)
        .unwrap_or(width_mm);

    Ok(Pad {
        id: ObjectId::new(format!("kicad:{native_id}")),
        number: sexpr::argument(value, 0)
            .and_then(sexpr::text)
            .unwrap_or("")
            .to_owned(),
        kind: sexpr::argument(value, 1)
            .and_then(sexpr::text)
            .unwrap_or("")
            .to_owned(),
        shape: sexpr::argument(value, 2)
            .and_then(sexpr::text)
            .unwrap_or("")
            .to_owned(),
        position: sexpr::required_point(value, "at")?,
        rotation: Rotation {
            degrees: at
                .and_then(|node| sexpr::argument(node, 2))
                .and_then(sexpr::number)
                .unwrap_or(0.0),
        },
        size: Size::new(width_mm, height_mm),
        drill_mm: parse_drill(value),
        layers: sexpr::child(value, "layers")
            .map(sexpr::arguments_text)
            .unwrap_or_default(),
        net_id: net_reference(value, net_ids),
        source: source_ref(path, native_id),
    })
}

fn parse_drill(value: &Value) -> Option<f64> {
    let drill = sexpr::child(value, "drill")?;
    sexpr::argument(drill, 0)
        .and_then(sexpr::number)
        .or_else(|| sexpr::argument(drill, 1).and_then(sexpr::number))
}
