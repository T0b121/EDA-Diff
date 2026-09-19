/*
Local KiCad file import panel controller.

This module transfers selected PCB or schematic bytes to the same core worker and
presents a compact canonical-model summary. User files are never uploaded.
*/

import type { CoreRequest, CoreResponse } from "../worker/messages";

interface ParsedFootprint {
  pads: unknown[];
}

interface ParsedPcb {
  footprints: ParsedFootprint[];
  tracks: unknown[];
  vias: unknown[];
  nets: unknown[];
  board_outline: unknown[];
}

interface ParsedSchematic {
  symbols: unknown[];
  wires: unknown[];
  junctions: unknown[];
  labels: unknown[];
  nets: unknown[];
}

export function connectEdaImportPanel(worker: Worker): void {
  const input = document.querySelector<HTMLInputElement>("#eda-file");
  const result = document.querySelector<HTMLOutputElement>("#eda-result");

  if (!input || !result) {
    return;
  }

  let requestId = 100;

  input.addEventListener("change", async () => {
    const file = input.files?.[0];
    if (!file) {
      return;
    }

    const type = requestType(file.name);
    if (!type) {
      result.textContent = "Unsupported file type.";
      return;
    }

    const id = requestId++;
    result.textContent = `Parsing ${file.name} locally…`;
    const request: CoreRequest = {
      id,
      type,
      path: file.name,
      bytes: await file.arrayBuffer()
    };

    const listener = (event: MessageEvent<CoreResponse>): void => {
      if (event.data.id !== id) {
        return;
      }

      worker.removeEventListener("message", listener);
      result.textContent = describeResult(event.data, file.name);
    };

    worker.addEventListener("message", listener);
    worker.postMessage(request, [request.bytes]);
  });
}

function requestType(
  fileName: string
): "parse-kicad-pcb" | "parse-kicad-schematic" | undefined {
  if (fileName.endsWith(".kicad_pcb")) {
    return "parse-kicad-pcb";
  }

  if (fileName.endsWith(".kicad_sch")) {
    return "parse-kicad-schematic";
  }

  return undefined;
}

function describeResult(response: CoreResponse, fileName: string): string {
  if (response.type === "error") {
    return `Import failed: ${response.message}`;
  }

  if (response.type === "pcb") {
    return describePcb(JSON.parse(response.json) as ParsedPcb, fileName);
  }

  if (response.type === "schematic") {
    return describeSchematic(
      JSON.parse(response.json) as ParsedSchematic,
      fileName
    );
  }

  return `Unexpected response while importing ${fileName}.`;
}

function describePcb(pcb: ParsedPcb, fileName: string): string {
  const padCount = pcb.footprints.reduce(
    (total, footprint) => total + footprint.pads.length,
    0
  );

  return [
    `${fileName}:`,
    `${pcb.footprints.length} footprints,`,
    `${padCount} pads,`,
    `${pcb.tracks.length} tracks,`,
    `${pcb.vias.length} vias,`,
    `${pcb.nets.length} nets,`,
    `${pcb.board_outline.length} board edges`
  ].join(" ");
}

function describeSchematic(
  schematic: ParsedSchematic,
  fileName: string
): string {
  return [
    `${fileName}:`,
    `${schematic.symbols.length} symbols,`,
    `${schematic.wires.length} wires,`,
    `${schematic.junctions.length} junctions,`,
    `${schematic.labels.length} labels`
  ].join(" ");
}
