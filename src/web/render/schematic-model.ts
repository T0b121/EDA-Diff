/*
Browser-side type view of the canonical schematic model.

These interfaces mirror serialized Rust core data for rendering only. They must
not contain KiCad-specific syntax or parsing rules; native formats are normalized
by adapters before reaching the browser renderer.
*/

export interface Point {
  x_mm: number;
  y_mm: number;
}

export interface Rotation {
  degrees: number;
}

export interface SymbolPin {
  number: string;
  name: string;
  electrical_type: string;
  graphic_style: string;
  position: Point;
  rotation: Rotation;
  length_mm: number;
}

export type SymbolFill = "none" | "outline" | "background";

export type SymbolGraphic =
  | {
      kind: "rectangle";
      start: Point;
      end: Point;
      stroke_width_mm: number;
      fill: SymbolFill;
    }
  | {
      kind: "polyline";
      points: Point[];
      stroke_width_mm: number;
      fill: SymbolFill;
    }
  | {
      kind: "circle";
      center: Point;
      radius_mm: number;
      stroke_width_mm: number;
      fill: SymbolFill;
    }
  | {
      kind: "arc";
      start: Point;
      mid: Point;
      end: Point;
      stroke_width_mm: number;
      fill: SymbolFill;
    };

export interface SymbolUnit {
  unit: number;
  style: number;
  graphics: SymbolGraphic[];
  pins: SymbolPin[];
}

export interface SymbolDefinition {
  library_id: string;
  units: SymbolUnit[];
}

export interface SymbolObject {
  id: string;
  reference: string;
  value: string;
  library_id?: string;
  unit: number;
  position: Point;
  rotation: Rotation;
  mirror_x: boolean;
  mirror_y: boolean;
}

export interface Wire {
  id: string;
  points: Point[];
}

export interface Junction {
  id: string;
  position: Point;
}

export interface Label {
  id: string;
  name: string;
  position: Point;
  rotation: Rotation;
}

export interface Schematic {
  symbol_definitions: SymbolDefinition[];
  symbols: SymbolObject[];
  wires: Wire[];
  junctions: Junction[];
  labels: Label[];
}

export interface ObjectChange {
  before_id?: string;
  after_id?: string;
  kind: ChangeKind;
}

export type ChangeKind = "added" | "removed" | "modified" | "unchanged";
export type Revision = "before" | "after";

export interface SchematicComparison {
  before: Schematic;
  after: Schematic;
  diff: { changes: ObjectChange[] };
}
