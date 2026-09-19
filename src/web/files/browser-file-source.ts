/*
Browser File adapter for local EDA-Diff projects.

This adapter wraps File objects supplied by drag-and-drop or file inputs. Direct
directory access can later implement the same ProjectFileSource contract.
*/

import type { ProjectFile, ProjectFileSource } from "./project-file-source";

export class BrowserFileSource implements ProjectFileSource {
  readonly label: string;

  constructor(private readonly files: readonly File[], label = "Local files") {
    this.label = label;
  }

  async listFiles(): Promise<ProjectFile[]> {
    return Promise.all(
      this.files.map(async (file) => ({
        path: file.webkitRelativePath || file.name,
        bytes: await file.arrayBuffer()
      }))
    );
  }
}
