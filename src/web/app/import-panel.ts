/*
Local semantic comparison controller for KiCad files.

This module reuses the existing compare-panel entry point. It validates two
compatible files, transfers them to the core worker, and renders diff results.
*/

import { renderPcbComparison, type PcbComparison } from "../render/pcb-svg";
import type { CoreRequest, CoreResponse } from "../worker/messages";

interface DiffSummary {
  added: number;
  removed: number;
  modified: number;
  unchanged: number;
}

interface FieldChange {
  field: string;
  before: unknown;
  after: unknown;
}

interface ObjectChange {
  object_type: string;
  display_name?: string;
  kind: "added" | "removed" | "modified" | "unchanged";
  fields: FieldChange[];
}

interface DiffReport {
  summary: DiffSummary;
  changes: ObjectChange[];
}

type DiffRequestType = "diff-kicad-pcb" | "diff-kicad-schematic";

export function connectEdaComparePanel(worker: Worker): void {
  const beforeInput = document.querySelector<HTMLInputElement>("#before-file");
  const afterInput = document.querySelector<HTMLInputElement>("#after-file");
  const button = document.querySelector<HTMLButtonElement>("#compare-files");
  const status = document.querySelector<HTMLOutputElement>("#diff-status");
  const report = document.querySelector<HTMLElement>("#diff-report");
  const visual = document.querySelector<HTMLElement>("#pcb-visual");

  if (!beforeInput || !afterInput || !button || !status || !report || !visual) {
    return;
  }

  let requestId = 100;

  button.addEventListener("click", async () => {
    const beforeFile = beforeInput.files?.[0];
    const afterFile = afterInput.files?.[0];

    if (!beforeFile || !afterFile) {
      status.textContent = "Select both files first.";
      return;
    }

    const type = diffType(beforeFile.name, afterFile.name);
    if (!type) {
      status.textContent = "Both files must be the same supported KiCad type.";
      report.hidden = true;
      return;
    }

    status.textContent = "Comparing files locally…";
    report.hidden = true;
    visual.hidden = true;
    visual.replaceChildren();

    const id = requestId++;
    const [beforeBytes, afterBytes] = await Promise.all([
      beforeFile.arrayBuffer(),
      afterFile.arrayBuffer()
    ]);

    const request: CoreRequest = {
      id,
      type,
      beforePath: beforeFile.name,
      afterPath: afterFile.name,
      beforeBytes,
      afterBytes
    };

    const listener = (event: MessageEvent<CoreResponse>): void => {
      if (event.data.id !== id) {
        return;
      }

      worker.removeEventListener("message", listener);

      if (event.data.type === "error") {
        status.textContent = `Comparison failed: ${event.data.message}`;
        return;
      }

      if (event.data.type === "pcb-comparison") {
        const comparison = JSON.parse(event.data.json) as PcbComparison;
        renderDiff(comparison.diff as DiffReport, status, report);
        renderPcbComparison(comparison, visual);
        visual.hidden = false;
        return;
      }

      if (event.data.type === "diff") {
        const result = JSON.parse(event.data.json) as DiffReport;
        renderDiff(result, status, report);
        return;
      }

      status.textContent = "Unexpected response from the EDA core.";
    };

    worker.addEventListener("message", listener);
    worker.postMessage(request, [beforeBytes, afterBytes]);
  });
}

function diffType(beforeName: string, afterName: string): DiffRequestType | undefined {
  if (beforeName.endsWith(".kicad_pcb") && afterName.endsWith(".kicad_pcb")) {
    return "diff-kicad-pcb";
  }

  if (beforeName.endsWith(".kicad_sch") && afterName.endsWith(".kicad_sch")) {
    return "diff-kicad-schematic";
  }

  return undefined;
}

function renderDiff(
  diff: DiffReport,
  status: HTMLOutputElement,
  report: HTMLElement
): void {
  const { added, removed, modified, unchanged } = diff.summary;
  status.textContent =
    `${added} added, ${removed} removed, ${modified} modified, ` +
    `${unchanged} unchanged`;

  const visibleChanges = diff.changes.filter((change) => change.kind !== "unchanged");
  report.replaceChildren();

  if (visibleChanges.length === 0) {
    report.textContent = "No semantic changes detected.";
    report.hidden = false;
    return;
  }

  const list = document.createElement("ul");
  list.className = "change-list";

  for (const change of visibleChanges) {
    list.append(renderChange(change));
  }

  report.append(list);
  report.hidden = false;
}

function renderChange(change: ObjectChange): HTMLLIElement {
  const item = document.createElement("li");
  const title = document.createElement("strong");
  title.textContent =
    `${change.kind}: ${change.display_name ?? change.object_type}`;
  item.append(title);

  if (change.fields.length > 0) {
    const fields = document.createElement("ul");
    for (const field of change.fields) {
      const detail = document.createElement("li");
      detail.textContent =
        `${field.field}: ${formatValue(field.before)} → ${formatValue(field.after)}`;
      fields.append(detail);
    }
    item.append(fields);
  }

  return item;
}

function formatValue(value: unknown): string {
  return typeof value === "string" ? value : JSON.stringify(value);
}
