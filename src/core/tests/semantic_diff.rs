/*
Integration tests for the format-independent semantic diff engine.
*/

use eda_diff_core::diff::{diff_pcb, diff_schematic, ChangeKind, MatchMethod};
use eda_diff_core::model::common::{ObjectId, Point, Rotation};
use eda_diff_core::model::pcb::{Footprint, Net as PcbNet, Pcb};
use eda_diff_core::model::schematic::{Net as SchematicNet, Schematic, Symbol};
use eda_diff_core::model::source::{SourceFormat, SourceRef};

fn source(id: &str) -> SourceRef {
    SourceRef {
        format: SourceFormat::KiCad,
        file_path: "test.kicad".to_owned(),
        native_id: Some(id.to_owned()),
    }
}

fn symbol(id: &str, reference: &str, value: &str, x: f64) -> Symbol {
    Symbol {
        id: ObjectId::new(id),
        reference: reference.to_owned(),
        value: value.to_owned(),
        library_id: Some("Device:R".to_owned()),
        footprint: None,
        unit: 1,
        position: Point::new(x, 10.0),
        rotation: Rotation { degrees: 0.0 },
        source: source(id),
    }
}

fn footprint(id: &str, reference: &str, value: &str, x: f64) -> Footprint {
    Footprint {
        id: ObjectId::new(id),
        reference: reference.to_owned(),
        value: value.to_owned(),
        library_link: Some("Resistor_SMD:R_0603".to_owned()),
        position: Point::new(x, 20.0),
        rotation: Rotation { degrees: 0.0 },
        layer: "F.Cu".to_owned(),
        pads: Vec::new(),
        source: source(id),
    }
}

#[test]
fn schematic_diff_reports_modified_added_removed_and_unchanged() {
    let mut before = Schematic::empty();
    before.symbols = vec![
        symbol("same-r1", "R1", "10k", 10.0),
        symbol("removed-r2", "R2", "1k", 20.0),
        symbol("same-r3", "R3", "4.7k", 30.0),
    ];

    let mut after = Schematic::empty();
    after.symbols = vec![
        symbol("same-r1", "R1", "22k", 12.0),
        symbol("added-r4", "R4", "100k", 40.0),
        symbol("same-r3", "R3", "4.7k", 30.0),
    ];

    let report = diff_schematic(&before, &after);

    assert_eq!(report.summary.modified, 1);
    assert_eq!(report.summary.added, 1);
    assert_eq!(report.summary.removed, 1);
    assert_eq!(report.summary.unchanged, 1);

    let r1 = report
        .changes
        .iter()
        .find(|change| change.display_name.as_deref() == Some("R1"))
        .expect("R1 should be present");

    assert_eq!(r1.kind, ChangeKind::Modified);
    assert_eq!(r1.match_method, Some(MatchMethod::ObjectId));
    assert!(r1.fields.iter().any(|field| field.field == "value"));
    assert!(r1.fields.iter().any(|field| field.field == "position"));
}

#[test]
fn schematic_symbol_falls_back_to_reference_when_id_changes() {
    let mut before = Schematic::empty();
    before.symbols.push(symbol("old-id", "U1", "MCU", 10.0));

    let mut after = Schematic::empty();
    after.symbols.push(symbol("new-id", "U1", "MCU", 11.0));

    let report = diff_schematic(&before, &after);

    assert_eq!(report.summary.modified, 1);
    assert_eq!(report.summary.added, 0);
    assert_eq!(report.summary.removed, 0);

    let change = &report.changes[0];
    assert_eq!(change.match_method, Some(MatchMethod::Reference));
    assert_eq!(change.before_id.as_ref().map(|id| id.0.as_str()), Some("old-id"));
    assert_eq!(change.after_id.as_ref().map(|id| id.0.as_str()), Some("new-id"));
}

#[test]
fn pcb_diff_matches_footprints_by_reference_and_nets_by_name() {
    let mut before = Pcb::empty();
    before.footprints.push(footprint("old-r1", "R1", "10k", 10.0));
    before.nets.push(PcbNet {
        id: ObjectId::new("old-gnd"),
        name: "GND".to_owned(),
    });

    let mut after = Pcb::empty();
    after.footprints.push(footprint("new-r1", "R1", "22k", 10.0));
    after.nets.push(PcbNet {
        id: ObjectId::new("new-gnd"),
        name: "GND".to_owned(),
    });

    let report = diff_pcb(&before, &after);

    let footprint = report
        .changes
        .iter()
        .find(|change| change.object_type == "pcb-footprint")
        .expect("footprint diff");
    assert_eq!(footprint.kind, ChangeKind::Modified);
    assert_eq!(footprint.match_method, Some(MatchMethod::Reference));
    assert!(footprint.fields.iter().any(|field| field.field == "value"));

    let net = report
        .changes
        .iter()
        .find(|change| change.object_type == "pcb-net")
        .expect("net diff");
    assert_eq!(net.kind, ChangeKind::Unchanged);
    assert_eq!(net.match_method, Some(MatchMethod::Name));
}

#[test]
fn duplicate_fallback_keys_do_not_create_ambiguous_matches() {
    let mut before = Schematic::empty();
    before.nets = vec![
        SchematicNet {
            id: ObjectId::new("old-1"),
            name: "DUP".to_owned(),
        },
        SchematicNet {
            id: ObjectId::new("old-2"),
            name: "DUP".to_owned(),
        },
    ];

    let mut after = Schematic::empty();
    after.nets = vec![
        SchematicNet {
            id: ObjectId::new("new-1"),
            name: "DUP".to_owned(),
        },
        SchematicNet {
            id: ObjectId::new("new-2"),
            name: "DUP".to_owned(),
        },
    ];

    let report = diff_schematic(&before, &after);

    assert_eq!(report.summary.added, 2);
    assert_eq!(report.summary.removed, 2);
    assert_eq!(report.summary.modified, 0);
    assert_eq!(report.summary.unchanged, 0);
}
