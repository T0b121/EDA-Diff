/*
Web Worker host for the Rust/WebAssembly EDA core.

CPU-heavy parsing, diffing, merging, and validation execute through this worker
so the browser UI remains responsive for large EDA projects.
*/

import { getCoreStatus, parseKiCadPcb } from "../core/core";
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

    if (request.type === "parse-kicad-pcb") {
      const source = new TextDecoder("utf-8", { fatal: true }).decode(request.bytes);
      respond({
        id: request.id,
        type: "pcb",
        json: await parseKiCadPcb(source, request.path)
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
