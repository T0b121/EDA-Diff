/*
KiCad PCB to canonical PCB converter.

This parser coordinates conversion of stable board concepts into the canonical
model. Detailed pad and board-outline conversion stays in focused submodules.
*/

use std::collections::HashMap;

use lexpr::Value;

use crate::adapter::error::AdapterError;
use crate::model::common::{ObjectId, Rotation};
use crate::model::pcb::{Footprint, Net, Pcb, PcbLayer, PcbLayerKind, Track, Via};
use super::native::{object_native_id, property, source_ref};
use super::sexpr;

pub fn parse(source: &str, path: &str) -> Result<Pcb, AdapterError> {
    let root = sexpr::parse(source)?;
    if sexpr::head(&root) != Some("kicad_pcb") {
        return Err(AdapterError::UnsupportedFormat(
            "Expected a KiCad .kicad_pcb document".to_owned(),
        ));
    }

    let layers = parse_layers(&root);
    let (nets, net_ids) = parse_nets(&root);
    let footprints = sexpr::children(&root, "footprint")
        .map(|value| parse_footprint(value, path, &net_ids))
        .collect::<Result<Vec<_>, _>>()?;
    let tracks = sexpr::children(&root, "segment")
        .map(|value| parse_track(value, path, &net_ids))
        .collect::<Result<Vec<_>, _>>()?;
    let vias = sexpr::children(&root, "via")
        .map(|value| parse_via(value, path, &net_ids))
        .collect::<Result<Vec<_>, _>>()?;
    let board_outline = super::outline::parse(&root, path)?;

    Ok(Pcb {
        layers,
        footprints,
        tracks,
        vias,
        nets,
        board_outline,
    })
}

fn parse_layers(root: &Value) -> Vec<PcbLayer> {
    let Some(layers) = sexpr::child(root, "layers") else {
        return Vec::new();
    };

    layers
        .list_iter()
        .into_iter()
        .flatten()
        .skip(1)
        .filter_map(|node| {
            let mut values = node.list_iter()?;
            values.next()?;
            let name = values.next()
                .and_then(sexpr::text)?
                .to_owned();
            let native_kind = values.next()
                .and_then(sexpr::text)
                .unwrap_or("");

            Some(PcbLayer {
                kind: classify_layer(&name, native_kind),
                name,
            })
        })
        .collect()
}

fn classify_layer(name: &str, native_kind: &str) -> PcbLayerKind {
    if name.ends_with(".Cu") {
        PcbLayerKind::Copper
    } else if matches!(name, "Edge.Cuts" | "Margin" | "F.CrtYd" | "B.CrtYd") {
        PcbLayerKind::Technical
    } else if native_kind == "user" {
        PcbLayerKind::User
    } else {
        PcbLayerKind::Other
    }
}

fn parse_nets(root: &Value) -> (Vec<Net>, HashMap<i64, ObjectId>) {
    let mut net_ids = HashMap::new();
    let nets = sexpr::children(root, "net")
        .filter_map(|node| {
            let ordinal = sexpr::argument(node, 0)?.as_i64()?;
            let name = sexpr::text(sexpr::argument(node, 1)?)?.to_owned();
            let id = ObjectId::new(format!("pcb-net:{ordinal}"));
            net_ids.insert(ordinal, id.clone());
            Some(Net { id, name })
        })
        .collect();
    (nets, net_ids)
}

fn parse_footprint(
    value: &Value,
    path: &str,
    net_ids: &HashMap<i64, ObjectId>,
) -> Result<Footprint, AdapterError> {
    let native_id = object_native_id(value).unwrap_or_else(|| "unknown".to_owned());
    let source = source_ref(path, native_id.clone());

    let at = sexpr::child(value, "at");
    let position = sexpr::required_point(value, "at")?;
    let rotation = Rotation {
        degrees: at
            .and_then(|node| sexpr::argument(node, 2))
            .and_then(sexpr::number)
            .unwrap_or(0.0),
    };

    Ok(Footprint {
        id: ObjectId::new(format!("kicad:{native_id}")),
        reference: property(value, "Reference").unwrap_or_default(),
        value: property(value, "Value").unwrap_or_default(),
        library_link: sexpr::argument(value, 0).and_then(sexpr::text).map(str::to_owned),
        position,
        rotation,
        layer: sexpr::child_text(value, "layer").unwrap_or("").to_owned(),
        pads: sexpr::children(value, "pad")
            .map(|pad| super::pad::parse(pad, path, net_ids))
            .collect::<Result<Vec<_>, _>>()?,
        source,
    })
}

fn parse_track(
    value: &Value,
    path: &str,
    net_ids: &HashMap<i64, ObjectId>,
) -> Result<Track, AdapterError> {
    let native_id = object_native_id(value).unwrap_or_else(|| "unknown".to_owned());
    Ok(Track {
        id: ObjectId::new(format!("kicad:{native_id}")),
        start: sexpr::required_point(value, "start")?,
        end: sexpr::required_point(value, "end")?,
        width_mm: sexpr::child_number(value, "width").unwrap_or(0.0),
        layer: sexpr::child_text(value, "layer").unwrap_or("").to_owned(),
        net_id: net_reference(value, net_ids),
        source: source_ref(path, native_id),
    })
}

fn parse_via(
    value: &Value,
    path: &str,
    net_ids: &HashMap<i64, ObjectId>,
) -> Result<Via, AdapterError> {
    let native_id = object_native_id(value).unwrap_or_else(|| "unknown".to_owned());
    Ok(Via {
        id: ObjectId::new(format!("kicad:{native_id}")),
        position: sexpr::required_point(value, "at")?,
        diameter_mm: sexpr::child_number(value, "size").unwrap_or(0.0),
        drill_mm: sexpr::child_number(value, "drill").unwrap_or(0.0),
        layers: sexpr::child(value, "layers")
            .map(sexpr::arguments_text)
            .unwrap_or_default(),
        net_id: net_reference(value, net_ids),
        source: source_ref(path, native_id),
    })
}

pub(super) fn net_reference(value: &Value, net_ids: &HashMap<i64, ObjectId>) -> Option<ObjectId> {
    let ordinal = sexpr::child(value, "net")
        .and_then(|node| sexpr::argument(node, 0))
        .and_then(Value::as_i64)?;
    net_ids.get(&ordinal).cloned()
}
