/*
Canonical PCB model for EDA-Diff.

The model represents board-level geometry and connectivity independent of any
vendor file format. Format-specific details remain inside adapters.
*/

use serde::{Deserialize, Serialize};

use super::common::{ObjectId, Point, Rotation, Size};
use super::source::SourceRef;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pcb {
    pub footprints: Vec<Footprint>,
    pub tracks: Vec<Track>,
    pub vias: Vec<Via>,
    pub nets: Vec<Net>,
    pub board_outline: Vec<BoardEdge>,
}

impl Pcb {
    pub fn empty() -> Self {
        Self {
            footprints: Vec::new(),
            tracks: Vec::new(),
            vias: Vec::new(),
            nets: Vec::new(),
            board_outline: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Footprint {
    pub id: ObjectId,
    pub reference: String,
    pub value: String,
    pub library_link: Option<String>,
    pub position: Point,
    pub rotation: Rotation,
    pub layer: String,
    pub pads: Vec<Pad>,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pad {
    pub id: ObjectId,
    pub number: String,
    pub kind: String,
    pub shape: String,
    pub position: Point,
    pub rotation: Rotation,
    pub size: Size,
    pub drill_mm: Option<f64>,
    pub layers: Vec<String>,
    pub net_id: Option<ObjectId>,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: ObjectId,
    pub start: Point,
    pub end: Point,
    pub width_mm: f64,
    pub layer: String,
    pub net_id: Option<ObjectId>,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Via {
    pub id: ObjectId,
    pub position: Point,
    pub diameter_mm: f64,
    pub drill_mm: f64,
    pub net_id: Option<ObjectId>,
    pub source: SourceRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum BoardEdge {
    Line {
        id: ObjectId,
        start: Point,
        end: Point,
        source: SourceRef,
    },
    Arc {
        id: ObjectId,
        start: Point,
        mid: Point,
        end: Point,
        source: SourceRef,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Net {
    pub id: ObjectId,
    pub name: String,
}
