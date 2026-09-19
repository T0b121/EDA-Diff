/*
Browser entry point for the EDA-Diff single-page application.

This module bootstraps routing and UI-to-worker communication. Parsing,
repository access, storage, and EDA domain behavior stay in dedicated modules.
*/

import { watchRoute, type AppRoute } from "./router";
import type { CoreRequest, CoreResponse } from "../worker/messages";

const status = document.querySelector<HTMLElement>("#build-status");
const description = document.querySelector<HTMLElement>("#view-description");
const coreWorker = new Worker(new URL("../worker/core-worker.ts", import.meta.url), {
  type: "module"
});

const descriptions: Record<AppRoute, string> = {
  home: "Browser-based schematic and PCB comparison with Git-aware workflows.",
  compare: "Compare two EDA revisions without uploading project files.",
  repository: "Browse repository history and select revisions for comparison."
};

watchRoute((route) => {
  if (description) {
    description.textContent = descriptions[route];
  }
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
