use serde::Serialize;

use crate::model::pcb::{BoardEdge, Footprint, Net, Pad, Pcb, Track, Via};

use super::matcher::match_items;
use super::types::{DiffReport, FieldChange, MatchMethod, ObjectChange};

pub fn diff_pcb(before: &Pcb, after: &Pcb) -> DiffReport {
    let mut changes = Vec::new();

    diff_footprints(before, after, &mut changes);
    diff_tracks(&before.tracks, &after.tracks, &mut changes);
    diff_vias(&before.vias, &after.vias, &mut changes);
    diff_nets(&before.nets, &after.nets, &mut changes);
    diff_board_edges(&before.board_outline, &after.board_outline, &mut changes);

    DiffReport::new(changes)
}

fn diff_footprints(before: &Pcb, after: &Pcb, changes: &mut Vec<ObjectChange>) {
    for matched in match_items(
        &before.footprints,
        &after.footprints,
        |item| item.id.0.clone(),
        |item| (!item.reference.is_empty()).then(|| item.reference.clone()),
        MatchMethod::Reference,
    ) {
        match (matched.before, matched.after) {
            (Some(before_index), Some(after_index)) => {
                let old = &before.footprints[before_index];
                let new = &after.footprints[after_index];
                let mut fields = Vec::new();
                field("reference", &old.reference, &new.reference, &mut fields);
                field("value", &old.value, &new.value, &mut fields);
                field(
                    "library-link",
                    &old.library_link,
                    &new.library_link,
                    &mut fields,
                );
                field("position", &old.position, &new.position, &mut fields);
                field("rotation", &old.rotation, &new.rotation, &mut fields);
                field("layer", &old.layer, &new.layer, &mut fields);

                changes.push(ObjectChange::compared(
                    "pcb-footprint",
                    display_reference(old, new),
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.expect("paired items have a match method"),
                    fields,
                ));

                diff_pads(old, new, changes);
            }
            (Some(index), None) => {
                let item = &before.footprints[index];
                changes.push(ObjectChange::removed(
                    "pcb-footprint",
                    non_empty(&item.reference),
                    item.id.clone(),
                ));
                for pad in &item.pads {
                    changes.push(ObjectChange::removed(
                        "pcb-pad",
                        pad_display_name(item, pad),
                        pad.id.clone(),
                    ));
                }
            }
            (None, Some(index)) => {
                let item = &after.footprints[index];
                changes.push(ObjectChange::added(
                    "pcb-footprint",
                    non_empty(&item.reference),
                    item.id.clone(),
                ));
                for pad in &item.pads {
                    changes.push(ObjectChange::added(
                        "pcb-pad",
                        pad_display_name(item, pad),
                        pad.id.clone(),
                    ));
                }
            }
            (None, None) => unreachable!(),
        }
    }
}

fn diff_pads(before: &Footprint, after: &Footprint, changes: &mut Vec<ObjectChange>) {
    for matched in match_items(
        &before.pads,
        &after.pads,
        |item| item.id.0.clone(),
        |_| None,
        MatchMethod::Reference,
    ) {
        match (matched.before, matched.after) {
            (Some(before_index), Some(after_index)) => {
                let old = &before.pads[before_index];
                let new = &after.pads[after_index];
                let mut fields = Vec::new();
                field("number", &old.number, &new.number, &mut fields);
                field("kind", &old.kind, &new.kind, &mut fields);
                field("shape", &old.shape, &new.shape, &mut fields);
                field("position", &old.position, &new.position, &mut fields);
                field("rotation", &old.rotation, &new.rotation, &mut fields);
                field("size", &old.size, &new.size, &mut fields);
                field("drill-mm", &old.drill_mm, &new.drill_mm, &mut fields);
                field("layers", &old.layers, &new.layers, &mut fields);
                field("net-id", &old.net_id, &new.net_id, &mut fields);

                changes.push(ObjectChange::compared(
                    "pcb-pad",
                    pad_display_name(after, new),
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.expect("paired items have a match method"),
                    fields,
                ));
            }
            (Some(index), None) => {
                let item = &before.pads[index];
                changes.push(ObjectChange::removed(
                    "pcb-pad",
                    pad_display_name(before, item),
                    item.id.clone(),
                ));
            }
            (None, Some(index)) => {
                let item = &after.pads[index];
                changes.push(ObjectChange::added(
                    "pcb-pad",
                    pad_display_name(after, item),
                    item.id.clone(),
                ));
            }
            (None, None) => unreachable!(),
        }
    }
}

fn diff_tracks(before: &[Track], after: &[Track], changes: &mut Vec<ObjectChange>) {
    for matched in match_items(
        before,
        after,
        |item| item.id.0.clone(),
        |_| None,
        MatchMethod::Reference,
    ) {
        match (matched.before, matched.after) {
            (Some(before_index), Some(after_index)) => {
                let old = &before[before_index];
                let new = &after[after_index];
                let mut fields = Vec::new();
                field("start", &old.start, &new.start, &mut fields);
                field("end", &old.end, &new.end, &mut fields);
                field("width-mm", &old.width_mm, &new.width_mm, &mut fields);
                field("layer", &old.layer, &new.layer, &mut fields);
                field("net-id", &old.net_id, &new.net_id, &mut fields);
                changes.push(ObjectChange::compared(
                    "pcb-track",
                    None,
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "pcb-track",
                None,
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "pcb-track",
                None,
                after[index].id.clone(),
            )),
            (None, None) => unreachable!(),
        }
    }
}

fn diff_vias(before: &[Via], after: &[Via], changes: &mut Vec<ObjectChange>) {
    for matched in match_items(
        before,
        after,
        |item| item.id.0.clone(),
        |_| None,
        MatchMethod::Reference,
    ) {
        match (matched.before, matched.after) {
            (Some(before_index), Some(after_index)) => {
                let old = &before[before_index];
                let new = &after[after_index];
                let mut fields = Vec::new();
                field("position", &old.position, &new.position, &mut fields);
                field(
                    "diameter-mm",
                    &old.diameter_mm,
                    &new.diameter_mm,
                    &mut fields,
                );
                field("drill-mm", &old.drill_mm, &new.drill_mm, &mut fields);
                field("net-id", &old.net_id, &new.net_id, &mut fields);
                changes.push(ObjectChange::compared(
                    "pcb-via",
                    None,
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "pcb-via",
                None,
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "pcb-via",
                None,
                after[index].id.clone(),
            )),
            (None, None) => unreachable!(),
        }
    }
}

fn diff_nets(before: &[Net], after: &[Net], changes: &mut Vec<ObjectChange>) {
    for matched in match_items(
        before,
        after,
        |item| item.id.0.clone(),
        |item| (!item.name.is_empty()).then(|| item.name.clone()),
        MatchMethod::Name,
    ) {
        match (matched.before, matched.after) {
            (Some(before_index), Some(after_index)) => {
                let old = &before[before_index];
                let new = &after[after_index];
                let mut fields = Vec::new();
                field("name", &old.name, &new.name, &mut fields);
                changes.push(ObjectChange::compared(
                    "pcb-net",
                    non_empty(&new.name).or_else(|| non_empty(&old.name)),
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "pcb-net",
                non_empty(&before[index].name),
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "pcb-net",
                non_empty(&after[index].name),
                after[index].id.clone(),
            )),
            (None, None) => unreachable!(),
        }
    }
}

fn diff_board_edges(
    before: &[BoardEdge],
    after: &[BoardEdge],
    changes: &mut Vec<ObjectChange>,
) {
    for matched in match_items(
        before,
        after,
        edge_id,
        |_| None,
        MatchMethod::Reference,
    ) {
        match (matched.before, matched.after) {
            (Some(before_index), Some(after_index)) => {
                let old = &before[before_index];
                let new = &after[after_index];
                let mut fields = Vec::new();
                field("geometry", old, new, &mut fields);
                changes.push(ObjectChange::compared(
                    "pcb-board-edge",
                    None,
                    edge_object_id(old),
                    edge_object_id(new),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "pcb-board-edge",
                None,
                edge_object_id(&before[index]),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "pcb-board-edge",
                None,
                edge_object_id(&after[index]),
            )),
            (None, None) => unreachable!(),
        }
    }
}

fn edge_id(edge: &BoardEdge) -> String {
    edge_object_id(edge).0
}

fn edge_object_id(edge: &BoardEdge) -> crate::model::common::ObjectId {
    match edge {
        BoardEdge::Line { id, .. } | BoardEdge::Arc { id, .. } => id.clone(),
    }
}

fn display_reference(before: &Footprint, after: &Footprint) -> Option<String> {
    non_empty(&after.reference).or_else(|| non_empty(&before.reference))
}

fn pad_display_name(footprint: &Footprint, pad: &Pad) -> Option<String> {
    match (non_empty(&footprint.reference), non_empty(&pad.number)) {
        (Some(reference), Some(number)) => Some(format!("{reference}.{number}")),
        (Some(reference), None) => Some(reference),
        (None, Some(number)) => Some(number),
        (None, None) => None,
    }
}

fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

fn field<T: PartialEq + Serialize>(
    name: &str,
    before: &T,
    after: &T,
    fields: &mut Vec<FieldChange>,
) {
    if before == after {
        return;
    }

    fields.push(FieldChange {
        field: name.to_owned(),
        before: serde_json::to_value(before).expect("canonical values serialize"),
        after: serde_json::to_value(after).expect("canonical values serialize"),
    });
}
