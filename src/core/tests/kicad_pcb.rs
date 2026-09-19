/*
Integration tests for the initial KiCad PCB adapter.

These tests use a minimal representative board document to verify conversion
into the canonical model without depending on browser or WebAssembly code.
*/

use eda_diff_core::adapter::format_adapter::FormatAdapter;
use eda_diff_core::adapter::kicad::KiCadAdapter;

const BOARD: &str = r#"
(kicad_pcb
  (version 20240108)
  (generator pcbnew)
  (net 0 "")
  (net 1 "GND")
  (footprint "Resistor_SMD:R_0603"
    (layer "F.Cu")
    (at 10 20 90)
    (uuid 11111111-1111-1111-1111-111111111111)
    (property "Reference" "R1")
    (property "Value" "10k")
    (pad "1" smd roundrect
      (at -0.825 0 90)
      (size 0.8 0.95)
      (layers "F.Cu" "F.Mask" "F.Paste")
      (net 1 "GND")
      (uuid 44444444-4444-4444-4444-444444444444)
    )
  )
  (segment
    (start 10 20)
    (end 15 20)
    (width 0.25)
    (layer "F.Cu")
    (net 1)
    (tstamp 22222222-2222-2222-2222-222222222222)
  )
  (via
    (at 15 20)
    (size 0.8)
    (drill 0.4)
    (layers "F.Cu" "B.Cu")
    (net 1)
    (uuid 33333333-3333-3333-3333-333333333333)
  )
  (gr_line
    (start 0 0)
    (end 20 0)
    (layer "Edge.Cuts")
    (uuid 55555555-5555-5555-5555-555555555555)
  )
  (gr_arc
    (start 20 0)
    (mid 21 1)
    (end 20 2)
    (layer "Edge.Cuts")
    (uuid 66666666-6666-6666-6666-666666666666)
  )
)
"#;

#[test]
fn imports_core_board_objects() {
    let pcb = KiCadAdapter
        .parse_pcb(BOARD, "board.kicad_pcb")
        .expect("sample PCB should parse");

    assert_eq!(pcb.nets.len(), 2);
    assert_eq!(pcb.footprints.len(), 1);
    assert_eq!(pcb.tracks.len(), 1);
    assert_eq!(pcb.vias.len(), 1);
    assert_eq!(pcb.board_outline.len(), 2);

    let footprint = &pcb.footprints[0];
    assert_eq!(
        footprint.source.native_id.as_deref(),
        Some("11111111-1111-1111-1111-111111111111")
    );
    assert_eq!(footprint.reference, "R1");
    assert_eq!(footprint.value, "10k");
    assert_eq!(footprint.position.x_mm, 10.0);
    assert_eq!(footprint.rotation.degrees, 90.0);
    assert_eq!(footprint.pads.len(), 1);

    let pad = &footprint.pads[0];
    assert_eq!(pad.number, "1");
    assert_eq!(pad.shape, "roundrect");
    assert_eq!(pad.position.x_mm, -0.825);
    assert_eq!(pad.rotation.degrees, 90.0);
    assert_eq!(pad.size.width_mm, 0.8);
    assert_eq!(pad.layers, vec!["F.Cu", "F.Mask", "F.Paste"]);
    assert_eq!(pad.net_id.as_ref().map(|id| id.0.as_str()), Some("pcb-net:1"));

    let track = &pcb.tracks[0];
    assert_eq!(track.width_mm, 0.25);
    assert_eq!(track.layer, "F.Cu");
    assert_eq!(track.net_id.as_ref().map(|id| id.0.as_str()), Some("pcb-net:1"));

    let via = &pcb.vias[0];
    assert_eq!(via.diameter_mm, 0.8);
    assert_eq!(via.drill_mm, 0.4);
}

#[test]
fn rejects_non_board_documents() {
    let error = KiCadAdapter
        .parse_pcb("(kicad_sch (version 20231120))", "wrong.kicad_pcb")
        .expect_err("schematic must not parse as PCB");

    assert!(error.to_string().contains("Expected a KiCad"));
}
