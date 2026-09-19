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
    pub symbols: Vec<Symbol>,
    pub wires: Vec<Wire>,
    pub junctions: Vec<Junction>,
    pub labels: Vec<Label>,
    pub nets: Vec<Net>,
}

impl Schematic {
    pub fn empty() -> Self {
        Self {
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
    pub source: SourceRef,
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
