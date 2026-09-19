/*
Local project-file source contracts for EDA-Diff.

The application consumes project files through this interface so drag-and-drop,
file pickers, directory handles, and future providers do not leak into EDA logic.
*/

export interface ProjectFile {
  path: string;
  bytes: ArrayBuffer;
}

export interface ProjectFileSource {
  readonly label: string;
  listFiles(): Promise<ProjectFile[]>;
}
