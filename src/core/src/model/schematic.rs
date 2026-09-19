/*
Canonical schematic model for EDA-Diff.

This file contains only format-independent electrical concepts. KiCad, Eagle,
Altium, and future adapters translate native schematic data into these types.
*/

use serde::{Deserialize, Serialize};

use super::common::{ObjectId, Point, Rotation};
use super::source::SourceRef;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schematic {
    pub symbol_definitions: Vec<SymbolDefinition>,
    pub symbols: Vec<Symbol>,
    pub wires: Vec<Wire>,
    pub junctions: Vec<Junction>,
    pub labels: Vec<Label>,
    pub nets: Vec<Net>,
}

impl Schematic {
    pub fn empty() -> Self {
        Self {
            symbol_definitions: Vec::new(),
            symbols: Vec::new(),
            wires: Vec::new(),
            junctions: Vec::new(),
            labels: Vec::new(),
            nets: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub id: ObjectId,
    pub reference: String,
    pub value: String,
    pub library_id: Option<String>,
    pub footprint: Option<String>,
    pub unit: u32,
    pub position: Point,
    pub rotation: Rotation,
    pub mirror_x: bool,
    pub mirror_y: bool,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymbolDefinition {
    pub library_id: String,
    pub units: Vec<SymbolUnit>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymbolUnit {
    pub unit: u32,
    pub style: u32,
    pub graphics: Vec<SymbolGraphic>,
    pub pins: Vec<SymbolPin>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SymbolGraphic {
    Rectangle {
        start: Point,
        end: Point,
        stroke_width_mm: f64,
        fill: SymbolFill,
    },
    Polyline {
        points: Vec<Point>,
        stroke_width_mm: f64,
        fill: SymbolFill,
    },
    Circle {
        center: Point,
        radius_mm: f64,
        stroke_width_mm: f64,
        fill: SymbolFill,
    },
    Arc {
        start: Point,
        mid: Point,
        end: Point,
        stroke_width_mm: f64,
        fill: SymbolFill,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolFill {
    None,
    Outline,
    Background,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymbolPin {
    pub number: String,
    pub name: String,
    pub electrical_type: String,
    pub graphic_style: String,
    pub position: Point,
    pub rotation: Rotation,
    pub length_mm: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Wire {
    pub id: ObjectId,
    pub points: Vec<Point>,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Junction {
    pub id: ObjectId,
    pub position: Point,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LabelKind {
    Local,
    Global,
    Hierarchical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub id: ObjectId,
    pub name: String,
    pub kind: LabelKind,
    pub position: Point,
    pub rotation: Rotation,
    pub electrical_shape: Option<String>,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Net {
    pub id: ObjectId,
    pub name: String,
}
