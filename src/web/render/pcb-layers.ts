/*
PCB layer visibility controls for canonical comparison rendering.

This module builds its controls from the canonical layer model and filters SVG
objects through their normalized layer annotations without knowing KiCad syntax.
*/

import type { PcbComparison } from "./pcb-svg";

interface PcbLayer {
  name: string;
  kind: "copper" | "technical" | "user" | "other";
}

export function renderPcbLayerControls(
  comparison: PcbComparison,
  container: HTMLElement,
  host: HTMLElement
): void {
  const layers = collectLayers(comparison);
  host.replaceChildren();

  if (layers.length === 0) {
    host.hidden = true;
    return;
  }

  const actions = document.createElement("div");
  actions.className = "pcb-layer-actions";

  const all = button("All", () => setAll(true));
  const none = button("None", () => setAll(false));
  actions.append(all, none);

  const list = document.createElement("div");
  list.className = "pcb-layer-list";

  const checkboxes = new Map<string, HTMLInputElement>();

  for (const layer of layers) {
    const label = document.createElement("label");
    label.className = `pcb-layer pcb-layer-${layer.kind}`;

    const input = document.createElement("input");
    input.type = "checkbox";
    input.checked = true;
    input.addEventListener("change", apply);

    const text = document.createElement("span");
    text.textContent = layer.name;

    checkboxes.set(layer.name, input);
    label.append(input, text);
    list.append(label);
  }

  host.append(actions, list);
  host.hidden = false;

  function setAll(checked: boolean): void {
    for (const input of checkboxes.values()) {
      input.checked = checked;
    }
    apply();
  }

  function apply(): void {
    const visible = new Set(
      [...checkboxes]
        .filter(([, input]) => input.checked)
        .map(([name]) => name)
    );

    for (const node of container.querySelectorAll<SVGElement>(".pcb-object")) {
      const layers = (node.dataset.layers ?? "")
        .split("|")
        .filter(Boolean);

      node.classList.toggle(
        "layer-hidden",
        layers.length > 0 && !layers.some((layer) => layerVisible(layer, visible))
      );
    }
  }
}

function collectLayers(comparison: PcbComparison): PcbLayer[] {
  const ordered = new Map<string, PcbLayer>();

  for (const layer of [...comparison.before.layers, ...comparison.after.layers]) {
    if (!ordered.has(layer.name)) {
      ordered.set(layer.name, layer);
    }
  }

  return [...ordered.values()];
}

function layerVisible(layer: string, visible: Set<string>): boolean {
  if (visible.has(layer)) {
    return true;
  }

  if (layer.startsWith("*.")) {
    const suffix = layer.slice(1);
    return [...visible].some((name) => name.endsWith(suffix));
  }

  return false;
}

function button(label: string, action: () => void): HTMLButtonElement {
  const result = document.createElement("button");
  result.type = "button";
  result.textContent = label;
  result.addEventListener("click", action);
  return result;
}
