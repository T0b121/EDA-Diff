/*
Browser entry point for the EDA-Diff single-page application.

This module bootstraps UI-to-worker communication. Parsing, repository access,
storage, routing, and EDA domain behavior stay in dedicated modules.
*/

import type { CoreRequest, CoreResponse } from "../worker/messages";

const status = document.querySelector<HTMLElement>("#build-status");
const coreWorker = new Worker(new URL("../worker/core-worker.ts", import.meta.url), {
  type: "module"
});

coreWorker.addEventListener("message", (event: MessageEvent<CoreResponse>) => {
  if (!status || event.data.id !== 1) {
    return;
  }

  status.textContent =
    event.data.type === "status"
      ? event.data.status
      : `Core error: ${event.data.message}`;
});

const request: CoreRequest = { id: 1, type: "status" };
coreWorker.postMessage(request);
