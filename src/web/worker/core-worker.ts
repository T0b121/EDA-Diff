/*
Web Worker host for the Rust/WebAssembly EDA core.

CPU-heavy parsing, diffing, merging, and validation will execute through this
worker so the browser UI thread remains responsive for large EDA projects.
*/

import { getCoreStatus } from "../core/core";
import type { CoreRequest, CoreResponse } from "./messages";

self.addEventListener("message", async (event: MessageEvent<CoreRequest>) => {
  const request = event.data;

  try {
    if (request.type === "status") {
      const response: CoreResponse = {
        id: request.id,
        type: "status",
        status: await getCoreStatus()
      };
      self.postMessage(response);
    }
  } catch (error) {
    const response: CoreResponse = {
      id: request.id,
      type: "error",
      message: error instanceof Error ? error.message : String(error)
    };
    self.postMessage(response);
  }
});
