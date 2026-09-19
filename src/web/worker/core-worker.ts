/*
Web Worker host for the Rust/WebAssembly EDA core.

CPU-heavy parsing, diffing, merging, and validation execute through this worker
so the browser UI remains responsive for large EDA projects.
*/

import {
  compareKiCadPcb,
  compareKiCadSchematic,
  diffKiCadSchematic,
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

    if (request.type === "parse-kicad-pcb" || request.type === "parse-kicad-schematic") {
      const source = decode(request.bytes);
      const json =
        request.type === "parse-kicad-pcb"
          ? await parseKiCadPcb(source, request.path)
          : await parseKiCadSchematic(source, request.path);

      respond({
        id: request.id,
        type: request.type === "parse-kicad-pcb" ? "pcb" : "schematic",
        json
      });
      return;
    }

    const beforeSource = decode(request.beforeBytes);
    const afterSource = decode(request.afterBytes);
    if (request.type === "diff-kicad-pcb") {
      respond({
        id: request.id,
        type: "pcb-comparison",
        json: await compareKiCadPcb(
          beforeSource,
          afterSource,
          request.beforePath,
          request.afterPath
        )
      });
      return;
    }

    respond({
      id: request.id,
      type: "schematic-comparison",
      json: await compareKiCadSchematic(
        beforeSource,
        afterSource,
        request.beforePath,
        request.afterPath
      )
    });
  } catch (error) {
    respond({
      id: request.id,
      type: "error",
      message: error instanceof Error ? error.message : String(error)
    });
  }
});

function decode(bytes: ArrayBuffer): string {
  return new TextDecoder("utf-8", { fatal: true }).decode(bytes);
}

function respond(response: CoreResponse): void {
  self.postMessage(response);
}
