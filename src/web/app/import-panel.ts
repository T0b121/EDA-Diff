/*
Local KiCad PCB import panel controller.

This module transfers selected board bytes to the core worker and presents a
small canonical-model summary. It never uploads user project data.
*/

import type { CoreRequest, CoreResponse } from "../worker/messages";

interface ParsedPcb {
  footprints: unknown[];
  tracks: unknown[];
  vias: unknown[];
  nets: unknown[];
}

export function connectPcbImportPanel(worker: Worker): void {
  const input = document.querySelector<HTMLInputElement>("#pcb-file");
  const result = document.querySelector<HTMLOutputElement>("#pcb-result");

  if (!input || !result) {
    return;
  }

  let requestId = 100;

  input.addEventListener("change", async () => {
    const file = input.files?.[0];
    if (!file) {
      return;
    }

    const id = requestId++;
    result.textContent = `Parsing ${file.name} locally…`;

    const request: CoreRequest = {
      id,
      type: "parse-kicad-pcb",
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

function describeResult(response: CoreResponse, fileName: string): string {
  if (response.type === "error") {
    return `Import failed: ${response.message}`;
  }

  if (response.type !== "pcb") {
    return `Unexpected response while importing ${fileName}.`;
  }

  const pcb = JSON.parse(response.json) as ParsedPcb;
  return [
    `${fileName}:`,
    `${pcb.footprints.length} footprints,`,
    `${pcb.tracks.length} tracks,`,
    `${pcb.vias.length} vias,`,
    `${pcb.nets.length} nets`
  ].join(" ");
}
