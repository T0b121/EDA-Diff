/*
Renderer for canonical schematic symbol definitions.

Placed symbols are resolved against canonical embedded definitions, then their
selected unit/common graphics and pins are transformed into SVG. A small fallback
is retained only for files whose adapter could not provide symbol geometry.
*/

import type {
  ChangeKind,
  Revision,
  SymbolDefinition,
  SymbolFill,
  SymbolGraphic,
  SymbolObject,
  SymbolPin
} from "./schematic-model";

const SVG_NS = "http://www.w3.org/2000/svg";

export function drawSchematicSymbol(
  svg: SVGSVGElement,
  symbol: SymbolObject,
  definitions: SymbolDefinition[],
  revision: Revision,
  kind: ChangeKind
): void {
  const group = element("g");
  const sx = symbol.mirror_x ? -1 : 1;
  const sy = symbol.mirror_y ? -1 : 1;
  group.setAttribute(
    "transform",
    "translate(" + symbol.position.x_mm + " " + symbol.position.y_mm + ") " +
      "rotate(" + symbol.rotation.degrees + ") scale(" + sx + " " + sy + ")"
  );
  group.classList.add("schematic-symbol");

  const definition = definitions.find(
    (item) => item.library_id === symbol.library_id
  );

  if (!definition) {
    drawFallback(group, symbol, revision, kind);
    svg.append(group);
    return;
  }

  const units = definition.units.filter(
    (unit) =>
      (unit.unit === 0 || unit.unit === symbol.unit) &&
      (unit.style === 0 || unit.style === 1)
  );

  for (const unit of units) {
    for (const graphic of unit.graphics) {
      group.append(drawGraphic(graphic, revision, kind));
    }
    for (const pin of unit.pins) {
      drawPin(group, pin, revision, kind);
    }
  }

  drawIdentity(group, symbol, revision, kind);
  svg.append(group);
}

function drawGraphic(
  graphic: SymbolGraphic,
  revision: Revision,
  kind: ChangeKind
): SVGElement {
  let node: SVGElement;

  switch (graphic.kind) {
    case "rectangle": {
      const rect = element("rect");
      rect.setAttribute("x", String(Math.min(graphic.start.x_mm, graphic.end.x_mm)));
      rect.setAttribute("y", String(Math.min(graphic.start.y_mm, graphic.end.y_mm)));
      rect.setAttribute("width", String(Math.abs(graphic.end.x_mm - graphic.start.x_mm)));
      rect.setAttribute("height", String(Math.abs(graphic.end.y_mm - graphic.start.y_mm)));
      node = rect;
      break;
    }
    case "polyline": {
      const polyline = element("polyline");
      polyline.setAttribute(
        "points",
        graphic.points.map((point) => point.x_mm + "," + point.y_mm).join(" ")
      );
      node = polyline;
      break;
    }
    case "circle": {
      const circle = element("circle");
      circle.setAttribute("cx", String(graphic.center.x_mm));
      circle.setAttribute("cy", String(graphic.center.y_mm));
      circle.setAttribute("r", String(graphic.radius_mm));
      node = circle;
      break;
    }
    case "arc": {
      const path = element("path");
      path.setAttribute("d", arcPath(graphic.start, graphic.mid, graphic.end));
      node = path;
      break;
    }
  }

  node.classList.add("symbol-graphic", "symbol-fill-" + graphic.fill);
  node.setAttribute("stroke-width", String(graphic.stroke_width_mm || 0.25));
  if (graphic.fill === "none") node.setAttribute("fill", "none");
  decorate(node, kind, revision);
  return node;
}

function drawPin(
  group: SVGGElement,
  pin: SymbolPin,
  revision: Revision,
  kind: ChangeKind
): void {
  const angle = pin.rotation.degrees * Math.PI / 180;
  const endX = pin.position.x_mm + Math.cos(angle) * pin.length_mm;
  const endY = pin.position.y_mm + Math.sin(angle) * pin.length_mm;

  const line = element("line");
  line.setAttribute("x1", String(pin.position.x_mm));
  line.setAttribute("y1", String(pin.position.y_mm));
  line.setAttribute("x2", String(endX));
  line.setAttribute("y2", String(endY));
  line.classList.add("schematic-pin");
  decorate(line, kind, revision);
  group.append(line);

  if (pin.graphic_style.includes("inverted")) {
    const bubble = element("circle");
    bubble.setAttribute("cx", String(endX));
    bubble.setAttribute("cy", String(endY));
    bubble.setAttribute("r", "0.55");
    bubble.classList.add("schematic-pin-decoration");
    bubble.setAttribute("fill", "none");
    decorate(bubble, kind, revision);
    group.append(bubble);
  }

  const number = element("text");
  number.setAttribute("x", String(pin.position.x_mm));
  number.setAttribute("y", String(pin.position.y_mm - 0.45));
  number.classList.add("schematic-pin-number");
  number.textContent = pin.number;
  decorate(number, kind, revision);
  group.append(number);
}

function drawIdentity(
  group: SVGGElement,
  symbol: SymbolObject,
  revision: Revision,
  kind: ChangeKind
): void {
  const reference = element("text");
  reference.setAttribute("x", "0");
  reference.setAttribute("y", "-4");
  reference.classList.add("schematic-reference");
  reference.textContent = symbol.reference || "?";
  decorate(reference, kind, revision);

  const value = element("text");
  value.setAttribute("x", "0");
  value.setAttribute("y", "4");
  value.classList.add("schematic-value");
  value.textContent = symbol.value;
  decorate(value, kind, revision);
  group.append(reference, value);
}

function drawFallback(
  group: SVGGElement,
  symbol: SymbolObject,
  revision: Revision,
  kind: ChangeKind
): void {
  const body = element("rect");
  body.setAttribute("x", "-5");
  body.setAttribute("y", "-3");
  body.setAttribute("width", "10");
  body.setAttribute("height", "6");
  body.classList.add("schematic-symbol-fallback");
  decorate(body, kind, revision);
  group.append(body);
  drawIdentity(group, symbol, revision, kind);
}

function decorate(node: SVGElement, kind: ChangeKind, revision: Revision): void {
  node.classList.add(
    "pcb-object",
    "schematic-object",
    "change-" + kind,
    "revision-" + revision
  );
}

function arcPath(
  start: { x_mm: number; y_mm: number },
  mid: { x_mm: number; y_mm: number },
  end: { x_mm: number; y_mm: number }
): string {
  const d = 2 * (
    start.x_mm * (mid.y_mm - end.y_mm) +
    mid.x_mm * (end.y_mm - start.y_mm) +
    end.x_mm * (start.y_mm - mid.y_mm)
  );
  if (Math.abs(d) < 1e-9) {
    return "M " + start.x_mm + " " + start.y_mm +
      " L " + end.x_mm + " " + end.y_mm;
  }

  const s2 = start.x_mm ** 2 + start.y_mm ** 2;
  const m2 = mid.x_mm ** 2 + mid.y_mm ** 2;
  const e2 = end.x_mm ** 2 + end.y_mm ** 2;
  const ux = (s2 * (mid.y_mm - end.y_mm) + m2 * (end.y_mm - start.y_mm) + e2 * (start.y_mm - mid.y_mm)) / d;
  const uy = (s2 * (end.x_mm - mid.x_mm) + m2 * (start.x_mm - end.x_mm) + e2 * (mid.x_mm - start.x_mm)) / d;
  const radius = Math.hypot(start.x_mm - ux, start.y_mm - uy);
  const cross =
    (mid.x_mm - start.x_mm) * (end.y_mm - mid.y_mm) -
    (mid.y_mm - start.y_mm) * (end.x_mm - mid.x_mm);
  const sweep = cross > 0 ? 1 : 0;

  return "M " + start.x_mm + " " + start.y_mm +
    " A " + radius + " " + radius + " 0 0 " + sweep +
    " " + end.x_mm + " " + end.y_mm;
}

function element<K extends keyof SVGElementTagNameMap>(
  name: K
): SVGElementTagNameMap[K] {
  return document.createElementNS(SVG_NS, name);
}
