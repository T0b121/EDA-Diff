/*
Canonical PCB model for EDA-Diff.

The model represents board-level geometry and connectivity independent of any
vendor file format. Format-specific details remain inside adapters.
*/

use serde::{Deserialize, Serialize};

use super::common::{ObjectId, Point, Rotation};
use super::source::SourceRef;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pcb {
    pub footprints: Vec<Footprint>,
    pub tracks: Vec<Track>,
    pub vias: Vec<Via>,
    pub nets: Vec<Net>,
}

impl Pcb {
    pub fn empty() -> Self {
        Self {
            footprints: Vec::new(),
            tracks: Vec::new(),
            vias: Vec::new(),
            nets: Vec::new(),
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
pub struct Net {
    pub id: ObjectId,
    pub name: String,
}
