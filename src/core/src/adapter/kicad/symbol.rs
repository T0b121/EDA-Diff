/*
KiCad embedded symbol-library parser.

This module converts the symbol definitions embedded in .kicad_sch files into
the format-independent schematic symbol model. KiCad unit/style identifiers,
drawing primitives, pin metadata, stroke width, and fill mode stay isolated here.
*/

use lexpr::Value;

use crate::model::common::{Point, Rotation};
use crate::model::schematic::{
    SymbolDefinition, SymbolFill, SymbolGraphic, SymbolPin, SymbolUnit,
};

use super::sexpr;

pub fn parse_definitions(root: &Value) -> Vec<SymbolDefinition> {
    let Some(library) = sexpr::child(root, "lib_symbols") else {
        return Vec::new();
    };

    sexpr::children(library, "symbol")
        .filter_map(parse_definition)
        .collect()
}

fn parse_definition(value: &Value) -> Option<SymbolDefinition> {
    let library_id = sexpr::argument(value, 0).and_then(sexpr::text)?.to_owned();
    let mut units = Vec::new();

    if has_drawables(value) {
        units.push(parse_unit(value, 0, 0));
    }

    for child in sexpr::children(value, "symbol") {
        let id = sexpr::argument(child, 0).and_then(sexpr::text)?;
        let (unit, style) = unit_style(id)?;
        units.push(parse_unit(child, unit, style));
    }

    Some(SymbolDefinition { library_id, units })
}

fn parse_unit(value: &Value, unit: u32, style: u32) -> SymbolUnit {
    let mut graphics = Vec::new();

    for node in value.list_iter().into_iter().flatten().skip(1) {
        match sexpr::head(node) {
            Some("rectangle") => parse_rectangle(node).map(|item| graphics.push(item)),
            Some("polyline") => parse_polyline(node).map(|item| graphics.push(item)),
            Some("circle") => parse_circle(node).map(|item| graphics.push(item)),
            Some("arc") => parse_arc(node).map(|item| graphics.push(item)),
            _ => None,
        };
    }

    let pins = sexpr::children(value, "pin").filter_map(parse_pin).collect();
    SymbolUnit { unit, style, graphics, pins }
}

fn parse_rectangle(value: &Value) -> Option<SymbolGraphic> {
    Some(SymbolGraphic::Rectangle {
        start: sexpr::point(value, "start")?,
        end: sexpr::point(value, "end")?,
        stroke_width_mm: stroke_width(value),
        fill: fill(value),
    })
}

fn parse_polyline(value: &Value) -> Option<SymbolGraphic> {
    Some(SymbolGraphic::Polyline {
        points: sexpr::child_xy_points(value, "pts"),
        stroke_width_mm: stroke_width(value),
        fill: fill(value),
    })
}

fn parse_circle(value: &Value) -> Option<SymbolGraphic> {
    Some(SymbolGraphic::Circle {
        center: sexpr::point(value, "center")?,
        radius_mm: sexpr::child_number(value, "radius")?,
        stroke_width_mm: stroke_width(value),
        fill: fill(value),
    })
}

fn parse_arc(value: &Value) -> Option<SymbolGraphic> {
    Some(SymbolGraphic::Arc {
        start: sexpr::point(value, "start")?,
        mid: sexpr::point(value, "mid")?,
        end: sexpr::point(value, "end")?,
        stroke_width_mm: stroke_width(value),
        fill: fill(value),
    })
}

fn parse_pin(value: &Value) -> Option<SymbolPin> {
    let electrical_type = sexpr::argument(value, 0).and_then(sexpr::text)?.to_owned();
    let graphic_style = sexpr::argument(value, 1).and_then(sexpr::text)?.to_owned();

    Some(SymbolPin {
        number: sexpr::child_text(value, "number").unwrap_or("").to_owned(),
        name: sexpr::child_text(value, "name").unwrap_or("").to_owned(),
        electrical_type,
        graphic_style,
        position: sexpr::point(value, "at")?,
        rotation: Rotation { degrees: sexpr::child_rotation(value, "at") },
        length_mm: sexpr::child_number(value, "length").unwrap_or(0.0),
    })
}

fn stroke_width(value: &Value) -> f64 {
    sexpr::child(value, "stroke")
        .and_then(|stroke| sexpr::child_number(stroke, "width"))
        .unwrap_or(0.0)
}

fn fill(value: &Value) -> SymbolFill {
    let mode = sexpr::child(value, "fill")
        .and_then(|fill| sexpr::child_text(fill, "type"));

    match mode {
        Some("outline") => SymbolFill::Outline,
        Some("background") => SymbolFill::Background,
        _ => SymbolFill::None,
    }
}

fn unit_style(id: &str) -> Option<(u32, u32)> {
    let mut parts = id.rsplitn(3, '_');
    let style = parts.next()?.parse().ok()?;
    let unit = parts.next()?.parse().ok()?;
    Some((unit, style))
}

fn has_drawables(value: &Value) -> bool {
    value.list_iter().into_iter().flatten().skip(1).any(|node| {
        matches!(
            sexpr::head(node),
            Some("rectangle" | "polyline" | "circle" | "arc" | "pin")
        )
    })
}
