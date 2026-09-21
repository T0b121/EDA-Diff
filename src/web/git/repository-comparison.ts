/*
Repository commit comparison controller.

This module connects GitHub commit selection to the existing EDA comparison
pipeline. It discovers supported KiCad files at both revisions and downloads
the selected pair through the existing repository-provider abstraction.
*/

import type { RepositoryCommit, RepositoryProvider } from "./repository-provider";

export interface RepositoryComparisonFiles {
  beforePath: string;
  afterPath: string;
  beforeBytes: ArrayBuffer;
  afterBytes: ArrayBuffer;
}

const SUPPORTED_EXTENSIONS = [".kicad_pcb", ".kicad_sch"];

export async function findComparisonFiles(
  provider: RepositoryProvider,
  selection: { before: RepositoryCommit; after: RepositoryCommit }
): Promise<RepositoryComparisonFiles> {
  const [beforeFiles, afterFiles] = await Promise.all([
    provider.listFiles(selection.before.id),
    provider.listFiles(selection.after.id)
  ]);

  const common = beforeFiles
    .filter((path) => afterFiles.includes(path))
    .filter((path) =>
      SUPPORTED_EXTENSIONS.some((extension) => path.endsWith(extension))
    );

  if (common.length === 0) {
    throw new Error("No common supported KiCad file exists in both revisions.");
  }

  if (common.length > 1) {
    throw new Error(
      "Multiple supported KiCad files exist. Select a project file explicitly."
    );
  }

  const path = common[0];
  const [beforeBytes, afterBytes] = await Promise.all([
    provider.readFile(selection.before.id, path),
    provider.readFile(selection.after.id, path)
  ]);

  if (!beforeBytes || !afterBytes) {
    throw new Error("The selected KiCad file could not be read.");
  }

  return {
    beforePath: path,
    afterPath: path,
    beforeBytes,
    afterBytes
  };
}
