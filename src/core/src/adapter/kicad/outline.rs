/*
KiCad board-outline conversion.

Only graphics on Edge.Cuts are treated as board geometry. Lines and arcs are
preserved as canonical primitives; additional KiCad edge types can be added here.
*/

use lexpr::Value;

use crate::adapter::error::AdapterError;
use crate::model::common::ObjectId;
use crate::model::pcb::BoardEdge;

use super::sexpr;
use super::pcb::{object_native_id, source_ref};

pub fn parse(root: &Value, path: &str) -> Result<Vec<BoardEdge>, AdapterError> {
    let mut edges = Vec::new();

    for value in sexpr::children(root, "gr_line") {
        if is_edge_cut(value) {
            edges.push(parse_line(value, path)?);
        }
    }

    for value in sexpr::children(root, "gr_arc") {
        if is_edge_cut(value) {
            edges.push(parse_arc(value, path)?);
        }
    }

    Ok(edges)
}

fn parse_line(value: &Value, path: &str) -> Result<BoardEdge, AdapterError> {
    let native_id = object_native_id(value).unwrap_or_else(|| "unknown".to_owned());
    Ok(BoardEdge::Line {
        id: ObjectId::new(format!("kicad:{native_id}")),
        start: sexpr::required_point(value, "start")?,
        end: sexpr::required_point(value, "end")?,
        source: source_ref(path, native_id),
    })
}

fn parse_arc(value: &Value, path: &str) -> Result<BoardEdge, AdapterError> {
    let native_id = object_native_id(value).unwrap_or_else(|| "unknown".to_owned());
    Ok(BoardEdge::Arc {
        id: ObjectId::new(format!("kicad:{native_id}")),
        start: sexpr::required_point(value, "start")?,
        mid: sexpr::required_point(value, "mid")?,
        end: sexpr::required_point(value, "end")?,
        source: source_ref(path, native_id),
    })
}

fn is_edge_cut(value: &Value) -> bool {
    sexpr::child_text(value, "layer") == Some("Edge.Cuts")
}
