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
    pub nets: Vec<Net>,
}

impl Schematic {
    pub fn empty() -> Self {
        Self {
            symbols: Vec::new(),
            wires: Vec::new(),
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
    pub position: Point,
    pub rotation: Rotation,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Wire {
    pub id: ObjectId,
    pub start: Point,
    pub end: Point,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Net {
    pub id: ObjectId,
    pub name: String,
}
