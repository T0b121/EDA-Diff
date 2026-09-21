/*
GitHub repository browser and compare selector.

This module uses the existing repository-provider abstraction to load public
commit history, select two revisions, choose a common KiCad file, and hand the
downloaded files to the existing local compare panel.
*/

import { GitHubRepositoryProvider } from "../git/github-repository-provider";
import type { RepositoryCommit } from "../git/repository-provider";

const SUPPORTED = [".kicad_pcb", ".kicad_sch"];

export function connectRepositoryView(container: HTMLElement): void {
  const status = document.createElement("p");
  status.setAttribute("aria-live", "polite");
  const load = document.createElement("button");
  load.type = "button";
  load.textContent = "Load GitHub repository";
  const list = document.createElement("ol");
  list.className = "repository-commits";
  container.append(load, status, list);

  load.addEventListener("click", async () => {
    const repository = parseRepository(location.hash);
    if (!repository) {
      status.textContent = "Use #/repository/OWNER/REPOSITORY.";
      return;
    }

    status.textContent = "Loading repository history…";
    list.replaceChildren();

    try {
      const provider = new GitHubRepositoryProvider(repository);
      const commits = await provider.listCommits(50);
      renderCommits(list, commits, status, provider);
    } catch (error) {
      status.textContent = message(error);
    }
  });
}

function parseRepository(hash: string): { owner: string; repository: string } | undefined {
  const parts = hash.slice(1).split("/").filter(Boolean);
  return parts.length === 3 && parts[0] === "repository"
    ? { owner: parts[1], repository: parts[2] }
    : undefined;
}

function renderCommits(
  list: HTMLOListElement,
  commits: RepositoryCommit[],
  status: HTMLElement,
  provider: GitHubRepositoryProvider
): void {
  let first: RepositoryCommit | undefined;
  status.textContent = commits.length
    ? "Select the older revision, then the newer revision."
    : "No commits found.";

  for (const commit of commits) {
    const item = document.createElement("li");
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = commit.shortId + " — " + commit.message;
    button.addEventListener("click", async () => {
      if (!first) {
        first = commit;
        status.textContent = "Older revision selected. Select the newer revision.";
        return;
      }

      const older = first;
      first = undefined;
      await chooseFile(provider, older, commit, status);
    });
    item.append(button);
    list.append(item);
  }
}

async function chooseFile(
  provider: GitHubRepositoryProvider,
  before: RepositoryCommit,
  after: RepositoryCommit,
  status: HTMLElement
): Promise<void> {
  status.textContent = "Finding common KiCad files…";
  try {
    const [beforeFiles, afterFiles] = await Promise.all([
      provider.listFiles(before.id),
      provider.listFiles(after.id)
    ]);
    const common = beforeFiles.filter((path) =>
      afterFiles.includes(path) &&
      SUPPORTED.some((extension) => path.endsWith(extension))
    );

    if (common.length === 0) {
      throw new Error("No common supported KiCad file exists in both revisions.");
    }
    if (common.length === 1) {
      await loadFile(provider, before, after, common[0], status);
      return;
    }

    status.textContent = "Select the KiCad file to compare:";
    showFileChoices(provider, before, after, common, status);
  } catch (error) {
    status.textContent = message(error);
  }
}

function showFileChoices(
  provider: GitHubRepositoryProvider,
  before: RepositoryCommit,
  after: RepositoryCommit,
  paths: string[],
  status: HTMLElement
): void {
  const panel = document.querySelector<HTMLElement>("#repository-files");
  if (!panel) return;
  panel.replaceChildren();

  for (const path of paths) {
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = path;
    button.addEventListener("click", () => {
      void loadFile(provider, before, after, path, status);
    });
    panel.append(button);
  }
}

async function loadFile(
  provider: GitHubRepositoryProvider,
  before: RepositoryCommit,
  after: RepositoryCommit,
  path: string,
  status: HTMLElement
): Promise<void> {
  status.textContent = "Downloading selected revisions…";
  const [beforeBytes, afterBytes] = await Promise.all([
    provider.readFile(before.id, path),
    provider.readFile(after.id, path)
  ]);
  if (!beforeBytes || !afterBytes) {
    status.textContent = "The selected file could not be read.";
    return;
  }

  const beforeInput = document.querySelector<HTMLInputElement>("#before-file");
  const afterInput = document.querySelector<HTMLInputElement>("#after-file");
  const compare = document.querySelector<HTMLButtonElement>("#compare-files");
  if (!beforeInput || !afterInput || !compare) {
    status.textContent = "Compare controls are not available.";
    return;
  }

  const transfer = new DataTransfer();
  transfer.items.add(new File([beforeBytes], path));
  beforeInput.files = transfer.files;
  const afterTransfer = new DataTransfer();
  afterTransfer.items.add(new File([afterBytes], path));
  afterInput.files = afterTransfer.files;

  window.location.hash = "#/compare";
  compare.click();
  status.textContent = "Files loaded into the compare view.";
}

function message(error: unknown): string {
  return "Repository could not be loaded: " +
    (error instanceof Error ? error.message : String(error));
}
