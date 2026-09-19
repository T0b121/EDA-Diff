/*
SVG renderer for canonical schematic comparisons.

This module renders sheet-level geometry and delegates placed symbol bodies to
the canonical symbol renderer. Native KiCad syntax must never reach this layer.
*/

import type {
  ChangeKind,
  ObjectChange,
  Point,
  Revision,
  Schematic,
  SchematicComparison
} from "./schematic-model";
import { drawSchematicSymbol } from "./schematic-symbol";

export type { SchematicComparison } from "./schematic-model";

const SVG_NS = "http://www.w3.org/2000/svg";

export function renderSchematicComparison(
  comparison: SchematicComparison,
  container: HTMLElement
): void {
  const status = buildStatusMap(comparison.diff.changes);
  const bounds = calculateBounds(comparison.before, comparison.after);
  const svg = element("svg");
  svg.classList.add("pcb-canvas", "schematic-canvas");

  const fitViewBox =
    bounds.minX + " " + bounds.minY + " " + bounds.width + " " + bounds.height;
  svg.setAttribute("viewBox", fitViewBox);
  svg.dataset.fitViewBox = fitViewBox;
  svg.setAttribute("preserveAspectRatio", "xMidYMid meet");
  svg.setAttribute("role", "img");
  svg.setAttribute("aria-label", "Visual schematic comparison");

  drawRevision(svg, comparison.before, "before", status);
  drawRevision(svg, comparison.after, "after", status);
  container.replaceChildren(svg);
}

function drawRevision(
  svg: SVGSVGElement,
  schematic: Schematic,
  revision: Revision,
  status: Map<string, ChangeKind>
): void {
  for (const wire of schematic.wires) {
    const polyline = element("polyline");
    polyline.setAttribute(
      "points",
      wire.points.map((point) => point.x_mm + "," + point.y_mm).join(" ")
    );
    polyline.classList.add("schematic-wire");
    decorate(polyline, status.get(wire.id) ?? "unchanged", revision);
    svg.append(polyline);
  }

  for (const junction of schematic.junctions) {
    const circle = element("circle");
    circle.setAttribute("cx", String(junction.position.x_mm));
    circle.setAttribute("cy", String(junction.position.y_mm));
    circle.setAttribute("r", "0.65");
    decorate(circle, status.get(junction.id) ?? "unchanged", revision);
    svg.append(circle);
  }

  for (const symbol of schematic.symbols) {
    drawSchematicSymbol(
      svg,
      symbol,
      schematic.symbol_definitions,
      revision,
      status.get(symbol.id) ?? "unchanged"
    );
  }

  for (const label of schematic.labels) {
    const text = element("text");
    text.setAttribute("x", String(label.position.x_mm));
    text.setAttribute("y", String(label.position.y_mm));
    text.setAttribute(
      "transform",
      "rotate(" + label.rotation.degrees + " " +
        label.position.x_mm + " " + label.position.y_mm + ")"
    );
    text.classList.add("schematic-label");
    text.textContent = label.name;
    decorate(text, status.get(label.id) ?? "unchanged", revision);
    svg.append(text);
  }
}

function decorate(
  node: SVGElement,
  kind: ChangeKind,
  revision: Revision
): void {
  node.classList.add(
    "pcb-object",
    "schematic-object",
    "change-" + kind,
    "revision-" + revision
  );
}

function buildStatusMap(changes: ObjectChange[]): Map<string, ChangeKind> {
  const status = new Map<string, ChangeKind>();
  for (const change of changes) {
    if (change.before_id) status.set(change.before_id, change.kind);
    if (change.after_id) status.set(change.after_id, change.kind);
  }
  return status;
}

function calculateBounds(before: Schematic, after: Schematic) {
  const points = [...collectPoints(before), ...collectPoints(after)];
  if (points.length === 0) {
    return { minX: 0, minY: 0, width: 100, height: 70 };
  }

  const xs = points.map((point) => point.x_mm);
  const ys = points.map((point) => point.y_mm);
  const margin = 8;
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

function collectPoints(schematic: Schematic): Point[] {
  const points: Point[] = [];
  for (const symbol of schematic.symbols) points.push(symbol.position);
  for (const wire of schematic.wires) points.push(...wire.points);
  for (const junction of schematic.junctions) points.push(junction.position);
  for (const label of schematic.labels) points.push(label.position);
  return points;
}

function element<K extends keyof SVGElementTagNameMap>(
  name: K
): SVGElementTagNameMap[K] {
  return document.createElementNS(SVG_NS, name);
}
