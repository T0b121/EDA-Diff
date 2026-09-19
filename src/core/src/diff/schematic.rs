use serde::Serialize;

use crate::model::schematic::{Junction, Label, Net, Schematic, Symbol, Wire};

use super::matcher::match_items;
use super::types::{DiffReport, FieldChange, MatchMethod, ObjectChange};

pub fn diff_schematic(before: &Schematic, after: &Schematic) -> DiffReport {
    let mut changes = Vec::new();

    diff_symbols(&before.symbols, &after.symbols, &mut changes);
    diff_wires(&before.wires, &after.wires, &mut changes);
    diff_junctions(&before.junctions, &after.junctions, &mut changes);
    diff_labels(&before.labels, &after.labels, &mut changes);
    diff_nets(&before.nets, &after.nets, &mut changes);

    DiffReport::new(changes)
}

fn diff_symbols(before: &[Symbol], after: &[Symbol], changes: &mut Vec<ObjectChange>) {
    for matched in match_items(
        before,
        after,
        |item| item.id.0.clone(),
        |item| (!item.reference.is_empty()).then(|| item.reference.clone()),
        MatchMethod::Reference,
    ) {
        match (matched.before, matched.after) {
            (Some(before_index), Some(after_index)) => {
                let old = &before[before_index];
                let new = &after[after_index];
                let mut fields = Vec::new();
                field("reference", &old.reference, &new.reference, &mut fields);
                field("value", &old.value, &new.value, &mut fields);
                field("library-id", &old.library_id, &new.library_id, &mut fields);
                field("footprint", &old.footprint, &new.footprint, &mut fields);
                field("unit", &old.unit, &new.unit, &mut fields);
                field("position", &old.position, &new.position, &mut fields);
                field("rotation", &old.rotation, &new.rotation, &mut fields);
                field("mirror-x", &old.mirror_x, &new.mirror_x, &mut fields);
                field("mirror-y", &old.mirror_y, &new.mirror_y, &mut fields);

                changes.push(ObjectChange::compared(
                    "schematic-symbol",
                    non_empty(&new.reference).or_else(|| non_empty(&old.reference)),
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "schematic-symbol",
                non_empty(&before[index].reference),
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "schematic-symbol",
                non_empty(&after[index].reference),
                after[index].id.clone(),
            )),
            (None, None) => unreachable!(),
        }
    }
}

fn diff_wires(before: &[Wire], after: &[Wire], changes: &mut Vec<ObjectChange>) {
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
                field("points", &old.points, &new.points, &mut fields);
                changes.push(ObjectChange::compared(
                    "schematic-wire",
                    None,
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "schematic-wire",
                None,
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "schematic-wire",
                None,
                after[index].id.clone(),
            )),
            (None, None) => unreachable!(),
        }
    }
}

fn diff_junctions(
    before: &[Junction],
    after: &[Junction],
    changes: &mut Vec<ObjectChange>,
) {
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
                changes.push(ObjectChange::compared(
                    "schematic-junction",
                    None,
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "schematic-junction",
                None,
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "schematic-junction",
                None,
                after[index].id.clone(),
            )),
            (None, None) => unreachable!(),
        }
    }
}

fn diff_labels(before: &[Label], after: &[Label], changes: &mut Vec<ObjectChange>) {
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
                field("kind", &old.kind, &new.kind, &mut fields);
                field("position", &old.position, &new.position, &mut fields);
                field("rotation", &old.rotation, &new.rotation, &mut fields);
                field(
                    "electrical-shape",
                    &old.electrical_shape,
                    &new.electrical_shape,
                    &mut fields,
                );
                changes.push(ObjectChange::compared(
                    "schematic-label",
                    non_empty(&new.name).or_else(|| non_empty(&old.name)),
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "schematic-label",
                non_empty(&before[index].name),
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "schematic-label",
                non_empty(&after[index].name),
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
                    "schematic-net",
                    non_empty(&new.name).or_else(|| non_empty(&old.name)),
                    old.id.clone(),
                    new.id.clone(),
                    matched.method.unwrap(),
                    fields,
                ));
            }
            (Some(index), None) => changes.push(ObjectChange::removed(
                "schematic-net",
                non_empty(&before[index].name),
                before[index].id.clone(),
            )),
            (None, Some(index)) => changes.push(ObjectChange::added(
                "schematic-net",
                non_empty(&after[index].name),
                after[index].id.clone(),
            )),
            (None, None) => unreachable!(),
        }
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
