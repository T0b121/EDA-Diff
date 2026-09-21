/*
GitHub repository browser view.

This module provides a small repository-history UI for the browser SPA. It uses
the repository-provider abstraction and emits selected commit/file pairs back to
the compare workflow instead of parsing GitHub data inside the view.
*/

import { GitHubRepositoryProvider } from "../git/github-repository-provider";
import type { RepositoryCommit } from "../git/repository-provider";

export interface RepositorySelection {
  before: RepositoryCommit;
  after: RepositoryCommit;
}

export function connectRepositoryView(
  container: HTMLElement,
  onSelection: (selection: RepositorySelection) => void
): void {
  const button = document.createElement("button");
  button.type = "button";
  button.textContent = "Load GitHub repository";
  container.append(button);

  const status = document.createElement("p");
  status.setAttribute("aria-live", "polite");
  container.append(status);

  const list = document.createElement("ol");
  list.className = "repository-commits";
  container.append(list);

  button.addEventListener("click", async () => {
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
      renderCommits(list, commits, status, onSelection);
      status.textContent = commits.length === 0
        ? "No commits found."
        : "Select two commits to compare.";
    } catch (error) {
      status.textContent =
        "Repository could not be loaded: " +
        (error instanceof Error ? error.message : String(error));
    }
  });
}

function parseRepository(hash: string): {
  owner: string;
  repository: string;
} | undefined {
  const parts = hash.slice(1).split("/").filter(Boolean);
  if (parts.length !== 3 || parts[0] !== "repository") {
    return undefined;
  }

  return { owner: parts[1], repository: parts[2] };
}

function renderCommits(
  list: HTMLOListElement,
  commits: RepositoryCommit[],
  status: HTMLElement,
  onSelection: (selection: RepositorySelection) => void
): void {
  let first: RepositoryCommit | undefined;

  for (const commit of commits) {
    const item = document.createElement("li");
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = commit.shortId + " — " + commit.message;

    button.addEventListener("click", () => {
      if (!first) {
        first = commit;
        status.textContent = "First revision selected. Select another commit.";
        return;
      }

      const second = commit;
      onSelection({
        before: second,
        after: first
      });
      status.textContent =
        "Selected " + second.shortId + " → " + first.shortId + ".";
      first = undefined;
    });

    item.append(button);
    list.append(item);
  }
}
