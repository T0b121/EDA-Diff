/*
Web Worker host for the Rust/WebAssembly EDA core.

CPU-heavy parsing, diffing, merging, and validation execute through this worker
so the browser UI remains responsive for large EDA projects.
*/

import {
  getCoreStatus,
  parseKiCadPcb,
  parseKiCadSchematic
} from "../core/core";
import type { CoreRequest, CoreResponse } from "./messages";

self.addEventListener("message", async (event: MessageEvent<CoreRequest>) => {
  const request = event.data;

  try {
    if (request.type === "status") {
      respond({
        id: request.id,
        type: "status",
        status: await getCoreStatus()
      });
      return;
    }

    const source = new TextDecoder("utf-8", { fatal: true }).decode(request.bytes);

    if (request.type === "parse-kicad-pcb") {
      respond({
        id: request.id,
        type: "pcb",
        json: await parseKiCadPcb(source, request.path)
      });
      return;
    }

    if (request.type === "parse-kicad-schematic") {
      respond({
        id: request.id,
        type: "schematic",
        json: await parseKiCadSchematic(source, request.path)
      });
    }
  } catch (error) {
    respond({
      id: request.id,
      type: "error",
      message: error instanceof Error ? error.message : String(error)
    });
  }
});

function respond(response: CoreResponse): void {
  self.postMessage(response);
}
