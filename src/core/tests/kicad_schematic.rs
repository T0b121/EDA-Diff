/*
Integration tests for the initial KiCad schematic adapter.

The sample covers placed symbols plus the connectivity geometry required for
later rendering and semantic net inference without relying on browser code.
*/

use eda_diff_core::adapter::format_adapter::FormatAdapter;
use eda_diff_core::adapter::kicad::KiCadAdapter;
use eda_diff_core::model::schematic::LabelKind;

const SCHEMATIC: &str = r#"
(kicad_sch
  (version 20231120)
  (generator eeschema)
  (symbol
    (lib_id "Device:R")
    (at 40 50 90)
    (unit 1)
    (uuid 11111111-1111-1111-1111-111111111111)
    (property "Reference" "R1")
    (property "Value" "10k")
    (property "Footprint" "Resistor_SMD:R_0603")
  )
  (wire
    (pts (xy 40 50) (xy 50 50))
    (uuid 22222222-2222-2222-2222-222222222222)
  )
  (junction
    (at 50 50)
    (uuid 33333333-3333-3333-3333-333333333333)
  )
  (label "LOCAL_NET"
    (at 50 50 0)
    (uuid 44444444-4444-4444-4444-444444444444)
  )
  (global_label "GND"
    (shape input)
    (at 40 50 180)
    (uuid 55555555-5555-5555-5555-555555555555)
  )
)
"#;

#[test]
fn imports_core_schematic_objects() {
    let schematic = KiCadAdapter
        .parse_schematic(SCHEMATIC, "sample.kicad_sch")
        .expect("sample schematic should parse");

    assert_eq!(schematic.symbols.len(), 1);
    assert_eq!(schematic.wires.len(), 1);
    assert_eq!(schematic.junctions.len(), 1);
    assert_eq!(schematic.labels.len(), 2);
    assert!(schematic.nets.is_empty());

    let symbol = &schematic.symbols[0];
    assert_eq!(symbol.reference, "R1");
    assert_eq!(symbol.value, "10k");
    assert_eq!(symbol.library_id.as_deref(), Some("Device:R"));
    assert_eq!(symbol.footprint.as_deref(), Some("Resistor_SMD:R_0603"));
    assert_eq!(symbol.position.x_mm, 40.0);
    assert_eq!(symbol.rotation.degrees, 90.0);

    assert_eq!(schematic.wires[0].points.len(), 2);
    assert_eq!(schematic.junctions[0].position.x_mm, 50.0);

    assert_eq!(schematic.labels[0].name, "LOCAL_NET");
    assert_eq!(schematic.labels[0].kind, LabelKind::Local);
    assert_eq!(schematic.labels[1].name, "GND");
    assert_eq!(schematic.labels[1].kind, LabelKind::Global);
    assert_eq!(schematic.labels[1].electrical_shape.as_deref(), Some("input"));
}

#[test]
fn rejects_non_schematic_documents() {
    let error = KiCadAdapter
        .parse_schematic("(kicad_pcb (version 20240108))", "wrong.kicad_sch")
        .expect_err("board must not parse as schematic");

    assert!(error.to_string().contains("Expected a KiCad"));
}
