/*
SVG renderer for canonical PCB comparisons.

Rendering consumes only the format-independent PCB model. Native KiCad syntax
must never leak into this module; adapters are responsible for normalization.
*/

interface Point { x_mm: number; y_mm: number }
interface Size { width_mm: number; height_mm: number }
interface Rotation { degrees: number }

interface Pad {
  id: string;
  number: string;
  shape: string;
  position: Point;
  rotation: Rotation;
  size: Size;
  layers: string[];
}

interface Footprint {
  id: string;
  reference: string;
  position: Point;
  rotation: Rotation;
  layer: string;
  pads: Pad[];
}

interface Track {
  id: string;
  start: Point;
  end: Point;
  width_mm: number;
  layer: string;
}

interface Via {
  id: string;
  position: Point;
  diameter_mm: number;
  layers: string[];
}

type BoardEdge =
  | { kind: "line"; id: string; start: Point; end: Point }
  | { kind: "arc"; id: string; start: Point; mid: Point; end: Point };

interface PcbLayer {
  name: string;
  kind: "copper" | "technical" | "user" | "other";
}

interface Pcb {
  layers: PcbLayer[];
  footprints: Footprint[];
  tracks: Track[];
  vias: Via[];
  board_outline: BoardEdge[];
}

interface ObjectChange {
  before_id?: string;
  after_id?: string;
  kind: "added" | "removed" | "modified" | "unchanged";
}

export interface PcbComparison {
  before: Pcb;
  after: Pcb;
  diff: { changes: ObjectChange[] };
}

type ChangeKind = ObjectChange["kind"];
type Revision = "before" | "after";

const SVG_NS = "http://www.w3.org/2000/svg";

export function renderPcbComparison(
  comparison: PcbComparison,
  container: HTMLElement
): void {
  const status = buildStatusMap(comparison.diff.changes);
  const bounds = calculateBounds(comparison.before, comparison.after);
  const svg = element("svg");
  svg.classList.add("pcb-canvas");
  const fitViewBox = `${bounds.minX} ${bounds.minY} ${bounds.width} ${bounds.height}`;
  svg.setAttribute("viewBox", fitViewBox);
  svg.dataset.fitViewBox = fitViewBox;
  svg.setAttribute("preserveAspectRatio", "xMidYMid meet");
  svg.setAttribute("role", "img");
  svg.setAttribute("aria-label", "Visual PCB comparison");

  drawRevision(svg, comparison.before, "before", status, false);
  drawRevision(svg, comparison.after, "after", status, false);

  container.replaceChildren(svg);
}

function drawRevision(
  svg: SVGSVGElement,
  pcb: Pcb,
  revision: Revision,
  status: Map<string, ChangeKind>,
  changesOnly: boolean
): void {
  for (const edge of pcb.board_outline) {
    drawEdge(svg, edge, revision, status, changesOnly);
  }
  for (const track of pcb.tracks) {
    drawTrack(svg, track, revision, status, changesOnly);
  }
  for (const footprint of pcb.footprints) {
    for (const pad of footprint.pads) {
      drawPad(svg, footprint, pad, revision, status, changesOnly);
    }
  }
  for (const via of pcb.vias) {
    drawVia(svg, via, revision, status, changesOnly);
  }
}

function drawTrack(
  svg: SVGSVGElement,
  track: Track,
  revision: Revision,
  status: Map<string, ChangeKind>,
  changesOnly: boolean
): void {
  const kind = status.get(track.id) ?? "unchanged";
  if (changesOnly && kind === "unchanged") return;

  const line = element("line");
  setLine(line, track.start, track.end);
  line.setAttribute("stroke-width", String(Math.max(track.width_mm, 0.15)));
  decorate(line, kind, revision, [track.layer]);
  svg.append(line);
}

function drawVia(
  svg: SVGSVGElement,
  via: Via,
  revision: Revision,
  status: Map<string, ChangeKind>,
  changesOnly: boolean
): void {
  const kind = status.get(via.id) ?? "unchanged";
  if (changesOnly && kind === "unchanged") return;

  const circle = element("circle");
  circle.setAttribute("cx", String(via.position.x_mm));
  circle.setAttribute("cy", String(via.position.y_mm));
  circle.setAttribute("r", String(Math.max(via.diameter_mm / 2, 0.2)));
  decorate(circle, kind, revision, via.layers);
  svg.append(circle);
}

function drawPad(
  svg: SVGSVGElement,
  footprint: Footprint,
  pad: Pad,
  revision: Revision,
  status: Map<string, ChangeKind>,
  changesOnly: boolean
): void {
  const ownKind = status.get(pad.id);
  const footprintKind = status.get(footprint.id);
  const kind =
    ownKind === "unchanged" && footprintKind === "modified"
      ? "modified"
      : ownKind ?? footprintKind ?? "unchanged";
  if (changesOnly && kind === "unchanged") return;

  const position = transformPadPosition(footprint, pad);
  const shape = pad.shape === "circle" || pad.shape === "oval"
    ? element("ellipse")
    : element("rect");

  if (shape instanceof SVGEllipseElement) {
    shape.setAttribute("cx", String(position.x_mm));
    shape.setAttribute("cy", String(position.y_mm));
    shape.setAttribute("rx", String(Math.max(pad.size.width_mm / 2, 0.15)));
    shape.setAttribute("ry", String(Math.max(pad.size.height_mm / 2, 0.15)));
  } else {
    shape.setAttribute("x", String(position.x_mm - pad.size.width_mm / 2));
    shape.setAttribute("y", String(position.y_mm - pad.size.height_mm / 2));
    shape.setAttribute("width", String(Math.max(pad.size.width_mm, 0.3)));
    shape.setAttribute("height", String(Math.max(pad.size.height_mm, 0.3)));
  }

  const rotation = footprint.rotation.degrees + pad.rotation.degrees;
  shape.setAttribute(
    "transform",
    `rotate(${rotation} ${position.x_mm} ${position.y_mm})`
  );
  decorate(shape, kind, revision, pad.layers.length > 0 ? pad.layers : [footprint.layer]);
  svg.append(shape);
}

function drawEdge(
  svg: SVGSVGElement,
  edge: BoardEdge,
  revision: Revision,
  status: Map<string, ChangeKind>,
  changesOnly: boolean
): void {
  const kind = status.get(edge.id) ?? "unchanged";
  if (changesOnly && kind === "unchanged") return;

  const path = element("path");
  path.setAttribute(
    "d",
    edge.kind === "line"
      ? `M ${edge.start.x_mm} ${edge.start.y_mm} L ${edge.end.x_mm} ${edge.end.y_mm}`
      : arcPath(edge.start, edge.mid, edge.end)
  );
  path.classList.add("pcb-outline");
  path.setAttribute("stroke-width", "0.2");
  decorate(path, kind, revision, ["Edge.Cuts"]);
  svg.append(path);
}

function transformPadPosition(footprint: Footprint, pad: Pad): Point {
  const angle = footprint.rotation.degrees * Math.PI / 180;
  const x = pad.position.x_mm;
  const y = pad.position.y_mm;

  return {
    x_mm: footprint.position.x_mm + x * Math.cos(angle) - y * Math.sin(angle),
    y_mm: footprint.position.y_mm + x * Math.sin(angle) + y * Math.cos(angle)
  };
}

function buildStatusMap(changes: ObjectChange[]): Map<string, ChangeKind> {
  const status = new Map<string, ChangeKind>();
  for (const change of changes) {
    if (change.before_id) status.set(change.before_id, change.kind);
    if (change.after_id) status.set(change.after_id, change.kind);
  }
  return status;
}

function decorate(
  node: SVGElement,
  kind: ChangeKind,
  revision: Revision,
  layers: string[]
): void {
  node.classList.add("pcb-object", `change-${kind}`, `revision-${revision}`);
  node.dataset.layers = layers.join("|");
}

function setLine(line: SVGLineElement, start: Point, end: Point): void {
  line.setAttribute("x1", String(start.x_mm));
  line.setAttribute("y1", String(start.y_mm));
  line.setAttribute("x2", String(end.x_mm));
  line.setAttribute("y2", String(end.y_mm));
}

function calculateBounds(before: Pcb, after: Pcb) {
  const points = [...collectPoints(before), ...collectPoints(after)];
  if (points.length === 0) {
    return { minX: 0, minY: 0, width: 100, height: 100 };
  }

  const xs = points.map((point) => point.x_mm);
  const ys = points.map((point) => point.y_mm);
  const margin = 5;
  const minX = Math.min(...xs) - margin;
  const minY = Math.min(...ys) - margin;
  const maxX = Math.max(...xs) + margin;
  const maxY = Math.max(...ys) + margin;

  return {
    minX,
    minY,
    width: Math.max(maxX - minX, 1),
    height: Math.max(maxY - minY, 1)
  };
}

function collectPoints(pcb: Pcb): Point[] {
  const points: Point[] = [];
  for (const track of pcb.tracks) points.push(track.start, track.end);
  for (const via of pcb.vias) points.push(via.position);
  for (const edge of pcb.board_outline) {
    points.push(edge.start, edge.end);
    if (edge.kind === "arc") points.push(edge.mid);
  }
  for (const footprint of pcb.footprints) {
    points.push(footprint.position);
    for (const pad of footprint.pads) {
      points.push(transformPadPosition(footprint, pad));
    }
  }
  return points;
}

function arcPath(start: Point, mid: Point, end: Point): string {
  const d = 2 * (
    start.x_mm * (mid.y_mm - end.y_mm) +
    mid.x_mm * (end.y_mm - start.y_mm) +
    end.x_mm * (start.y_mm - mid.y_mm)
  );

  if (Math.abs(d) < 1e-9) {
    return `M ${start.x_mm} ${start.y_mm} L ${end.x_mm} ${end.y_mm}`;
  }

  const s2 = start.x_mm ** 2 + start.y_mm ** 2;
  const m2 = mid.x_mm ** 2 + mid.y_mm ** 2;
  const e2 = end.x_mm ** 2 + end.y_mm ** 2;
  const cx = (s2 * (mid.y_mm - end.y_mm) +
    m2 * (end.y_mm - start.y_mm) +
    e2 * (start.y_mm - mid.y_mm)) / d;
  const cy = (s2 * (end.x_mm - mid.x_mm) +
    m2 * (start.x_mm - end.x_mm) +
    e2 * (mid.x_mm - start.x_mm)) / d;
  const radius = Math.hypot(start.x_mm - cx, start.y_mm - cy);

  const startAngle = Math.atan2(start.y_mm - cy, start.x_mm - cx);
  const midAngle = Math.atan2(mid.y_mm - cy, mid.x_mm - cx);
  const endAngle = Math.atan2(end.y_mm - cy, end.x_mm - cx);
  const forward = normalizeAngle(endAngle - startAngle);
  const midForward = normalizeAngle(midAngle - startAngle);
  const sweep = midForward <= forward ? 1 : 0;
  const delta = sweep ? forward : normalizeAngle(startAngle - endAngle);
  const largeArc = delta > Math.PI ? 1 : 0;

  return `M ${start.x_mm} ${start.y_mm} A ${radius} ${radius} 0 ${largeArc} ${sweep} ${end.x_mm} ${end.y_mm}`;
}

function normalizeAngle(angle: number): number {
  const full = Math.PI * 2;
  return ((angle % full) + full) % full;
}

function element<K extends keyof SVGElementTagNameMap>(
  name: K
): SVGElementTagNameMap[K] {
  return document.createElementNS(SVG_NS, name);
}
