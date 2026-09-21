/*
Browser entry point for the EDA-Diff single-page application.

This module bootstraps routing and UI-to-worker communication. Parsing,
repository access, storage, and EDA domain behavior stay in dedicated modules.
*/

import { connectEdaComparePanel } from "./import-panel";
import { connectRepositoryView } from "./repository-view";
import { watchRoute, type AppRoute } from "./router";
import type { CoreRequest, CoreResponse } from "../worker/messages";

const status = document.querySelector<HTMLElement>("#build-status");
const description = document.querySelector<HTMLElement>("#view-description");
const comparePanel = document.querySelector<HTMLElement>("#compare-panel");
const repositoryPanel = document.querySelector<HTMLElement>("#repository-panel");
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

  if (comparePanel) {
    comparePanel.hidden = route !== "compare";
  }
  if (repositoryPanel) {
    repositoryPanel.hidden = route !== "repository";
  }
});

connectEdaComparePanel(coreWorker);
if (repositoryPanel) {
  connectRepositoryView(repositoryPanel);
}

coreWorker.addEventListener("message", (event: MessageEvent<CoreResponse>) => {
  if (!status || event.data.id !== 1) {
    return;
  }

  if (event.data.type === "status") {
    status.textContent = event.data.status;
  } else if (event.data.type === "error") {
    status.textContent = `Core error: ${event.data.message}`;
  }
});

const request: CoreRequest = { id: 1, type: "status" };
coreWorker.postMessage(request);
